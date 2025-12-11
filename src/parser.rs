use lalr::{Grammar, LR1ParseTable, LRAction, Rhs};
use relex::Token;

use crate::parse_tree::{ParseError, ParseTree, Span};
use crate::semantic_action::SemanticAction;
use crate::symbol::{NonTerminal, Terminal};

pub struct Parser<'a, SemanticAction> {
    parse_table: LR1ParseTable<'a, Terminal, NonTerminal, ()>,
    semantic_action: SemanticAction,
}

impl<'a, Action> Parser<'a, Action>
where
    Action: SemanticAction,
{
    pub fn new<ReduceFn, PriorityFn>(
        grammar: &'a Grammar<Terminal, NonTerminal, ()>,
        reduce_on: ReduceFn,
        priority_of: PriorityFn,
        semantic_action: Action,
    ) -> Self
    where
        ReduceFn: FnMut(&Rhs<Terminal, NonTerminal, ()>, Option<&Terminal>) -> bool,
        PriorityFn: FnMut(&Rhs<Terminal, NonTerminal, ()>, Option<&Terminal>) -> i32,
    {
        match grammar.lalr1(reduce_on, priority_of) {
            Ok(parse_table) => Self {
                parse_table,
                semantic_action,
            },
            Err(conflict) => panic!("Grammar is not LALR(1), conflict detected: {conflict:?}"),
        }
    }

    pub fn parse<I>(&mut self, mut iterator: I) -> Result<ParseTree, ParseError>
    where
        I: Iterator<Item = Token<'a, Terminal>>,
    {
        let parse_table = &self.parse_table;
        let mut state_stack = vec![0];
        let mut node_stack: Vec<ParseTree> = vec![];
        let mut token = iterator.next().unwrap();

        loop {
            let state = *state_stack.last().unwrap();
            let action = if token.kind == Terminal::Eof {
                parse_table.states[state].eof.as_ref()
            } else {
                parse_table.states[state].lookahead.get(&token.kind)
            };

            match action {
                Some(LRAction::Reduce(non_terminal, rhs)) => {
                    for _ in 0..rhs.syms.len() {
                        state_stack.pop();
                    }
                    let state = *state_stack.last().unwrap();
                    let next_state = parse_table.states[state].goto.get(non_terminal).unwrap();
                    state_stack.push(*next_state);
                    self.semantic_action
                        .on_reduce(**non_terminal, rhs, &mut node_stack);
                }
                Some(LRAction::Shift(next_state)) => {
                    let new_node = ParseTree::terminal(
                        token.kind,
                        token.text.to_string(),
                        Span::new(0, 0, 1, 1),
                    );
                    state_stack.push(*next_state);
                    node_stack.push(new_node);
                    token = iterator.next().unwrap();
                }
                Some(LRAction::Accept) => {
                    break;
                }
                _ => panic!(
                    "Failed to get action for state {} and token {:?}",
                    state, token.kind
                ),
            }
        }
        let children = std::mem::take(&mut node_stack);
        let root = ParseTree::non_terminal(NonTerminal::Grammar, children, Span::new(0, 0, 1, 1));
        Ok(root)
    }
}
