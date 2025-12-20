use lalr::{LR1ParseTable, LRAction};
use relex::{Token, TokenKind};

use crate::common::action::Action;
use crate::common::symbol_table::{NonTerminal, Terminal};

pub struct Parser<'a, Action> {
    parse_table: LR1ParseTable<'a, Terminal, NonTerminal, ()>,
    semantic_action: Action,
}

impl<'a, A> Parser<'a, A>
where
    A: Action,
{
    pub const fn new(
        parse_table: LR1ParseTable<'a, Terminal, NonTerminal, ()>,
        semantic_action: A,
    ) -> Self {
        Self {
            parse_table,
            semantic_action,
        }
    }

    pub fn parse<I>(&mut self, mut iterator: I) -> Result<A::ParseResult, A::ParseError>
    where
        I: Iterator<Item = Token<'a, Terminal>>,
    {
        let parse_table = &self.parse_table;
        let mut state_stack = vec![0];
        let mut token = iterator.next().expect("Input token stream is empty");

        loop {
            let state = *state_stack
                .last()
                .expect("State stack is empty during parsing");
            let action = if token.kind.is_eof() {
                parse_table.states[state].eof.as_ref()
            } else {
                parse_table.states[state].lookahead.get(&token.kind)
            };

            match action {
                Some(LRAction::Reduce(non_terminal, rhs)) => {
                    for _ in 0..rhs.syms.len() {
                        state_stack.pop();
                    }
                    let state = *state_stack
                        .last()
                        .expect("State stack is empty after reduction");
                    let next_state = parse_table.states[state]
                        .goto
                        .get(non_terminal)
                        .expect("Failed to get next state from parse table");
                    state_stack.push(*next_state);
                    self.semantic_action.on_reduce(non_terminal, rhs);
                }
                Some(LRAction::Shift(next_state)) => {
                    state_stack.push(*next_state);
                    self.semantic_action.on_shift(token);
                    token = iterator
                        .next()
                        .expect("Unexpected end of input token stream");
                }
                Some(LRAction::Accept) => {
                    return Ok(self.semantic_action.on_accept());
                }
                _ => panic!(
                    "Failed to get action for state {} and token {:?}",
                    state, token.kind
                ),
            }
        }
    }
}
