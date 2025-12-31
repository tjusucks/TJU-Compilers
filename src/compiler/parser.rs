use relex::TokenKind;

use crate::common::action::Action;
use crate::common::grammar::{LR1ParseTable, LRAction};
use crate::common::symbol_table::{NonTerminal, Terminal};
use crate::compiler::lexer::LocatedToken;

pub struct Parser<'a, Action> {
    parse_table: &'a LR1ParseTable<'a, Terminal, NonTerminal, ()>,
    semantic_action: Action,
}

impl<'a, A> Parser<'a, A>
where
    A: Action,
{
    pub const fn new(
        parse_table: &'a LR1ParseTable<'a, Terminal, NonTerminal, ()>,
        semantic_action: A,
    ) -> Self {
        Self {
            parse_table,
            semantic_action,
        }
    }

    /// Parses the input token stream using the LALR(1) parser.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The input token stream is empty
    /// - The state stack becomes empty during parsing
    /// - Next state cannot be found from the parse table
    /// - Unexpected end of input token stream occurs during parsing
    /// - Failed to get action for a particular state and token combination
    ///
    /// # Errors
    ///
    /// Returns an error if the parsing process encounters an unrecoverable parsing error.
    pub fn parse<I>(&mut self, mut iterator: I) -> Result<A::ParseResult, A::ParseError>
    where
        I: Iterator<Item = LocatedToken<'a>>,
    {
        let parse_table = &self.parse_table;
        let mut state_stack = vec![0];
        let mut located_token = iterator.next().expect("Input token stream is empty");

        loop {
            let state = *state_stack
                .last()
                .expect("State stack is empty during parsing");
            let action = if located_token.token.kind.is_eof() {
                parse_table.states[state].eof.as_ref()
            } else {
                parse_table.states[state]
                    .lookahead
                    .get(&located_token.token.kind)
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
                    self.semantic_action.on_shift(located_token.token);
                    located_token = iterator
                        .next()
                        .expect("Unexpected end of input token stream");
                }
                Some(LRAction::Accept) => {
                    return Ok(self.semantic_action.on_accept());
                }
                _ => {
                    return Err(self
                        .semantic_action
                        .on_error(located_token.token, located_token.span));
                }
            }
        }
    }
}
