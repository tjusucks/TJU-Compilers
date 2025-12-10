use lalr::{Grammar, LR1ParseTable, LRAction, Rhs};
use relex::Token;

use crate::parse_tree::{ParseError, ParseTree, Span};
use crate::parser_rule::ParserRule;
use crate::symbol::{NonTerminal, Terminal};

pub struct Parser<'a> {
    parse_table: LR1ParseTable<'a, Terminal, NonTerminal, ()>,
}

impl<'a> Parser<'a> {
    pub fn new<ReduceFn, PriorityFn>(
        grammar: &'a Grammar<Terminal, NonTerminal, ()>,
        reduce_on: ReduceFn,
        priority_of: PriorityFn,
    ) -> Self
    where
        ReduceFn: FnMut(&Rhs<Terminal, NonTerminal, ()>, Option<&Terminal>) -> bool,
        PriorityFn: FnMut(&Rhs<Terminal, NonTerminal, ()>, Option<&Terminal>) -> i32,
    {
        match grammar.lalr1(reduce_on, priority_of) {
            Ok(parse_table) => Self { parse_table },
            Err(conflict) => panic!("Grammar is not LALR(1), conflict detected: {conflict:?}"),
        }
    }

    pub fn parse<I>(iterator: I) -> Result<ParseTree, ParseError>
    where
        I: Iterator<Item = Token<'a, Terminal>>,
    {
        let parser = Parser::new(
            ParserRule::grammar(),
            ParserRule::reduce_on,
            ParserRule::priority_of,
        );

        println!("LALR(1) parse table built successfully!");
        for (state_id, state) in parser.parse_table.states.iter().enumerate() {
            println!("State {state_id}: {state:?}");
        }
        println!();

        let input = iterator.collect::<Vec<_>>();

        for token in &input {
            println!("Token: {token:?}");
        }

        let mut state_stack = vec![0];
        let mut node_stack = vec![ParseTree::non_terminal(
            NonTerminal::Grammar,
            vec![],
            Span::new(0, 0, 1, 1),
        )];
        let mut input_position = 0;
        loop {
            let state = *state_stack.last().unwrap();
            let terminal = &input[input_position];
            let action = parser.parse_table.states[state]
                .lookahead
                .get(&terminal.kind);
            match action {
                Some(LRAction::Reduce(non_terminal, rhs)) => {
                    let mut children = Vec::with_capacity(rhs.syms.len());
                    for _ in 0..rhs.syms.len() {
                        if let Some(child) = node_stack.pop() {
                            children.push(child);
                        }
                        state_stack.pop();
                    }
                    children.reverse();
                    let state = *state_stack.last().unwrap();
                    let next_state = parser.parse_table.states[state]
                        .goto
                        .get(non_terminal)
                        .unwrap();
                    let new_node =
                        ParseTree::non_terminal(**non_terminal, children, Span::new(0, 0, 1, 1));
                    state_stack.push(*next_state);
                    node_stack.push(new_node);
                }
                Some(LRAction::Shift(next_state)) => {
                    let new_node = ParseTree::terminal(
                        terminal.kind,
                        terminal.text.to_string(),
                        Span::new(0, 0, 1, 1),
                    );
                    state_stack.push(*next_state);
                    node_stack.push(new_node);
                    input_position += 1;
                }
                Some(LRAction::Accept) => {
                    break;
                }
                _ => panic!(
                    "Failed to get action for state {} and token {:?}",
                    state, terminal.kind
                ),
            }
        }

        let root = node_stack.pop().expect("Node stack is empty.");
        Ok(root)
    }
}
