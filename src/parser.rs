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

    pub fn parse<I>(mut iterator: I) -> Result<ParseTree, ParseError>
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

        let mut state_stack = vec![0];
        let mut node_stack: Vec<ParseTree> = vec![];
        let mut token = iterator.next().unwrap();
        loop {
            let state = *state_stack.last().unwrap();
            let action = if token.kind == Terminal::Eof {
                parser.parse_table.states[state].eof.as_ref()
            } else {
                parser.parse_table.states[state].lookahead.get(&token.kind)
            };
            match action {
                Some(LRAction::Reduce(non_terminal, rhs)) => {
                    let length = rhs.syms.len();
                    for _ in 0..length {
                        state_stack.pop();
                    }
                    let state = *state_stack.last().unwrap();
                    let next_state = parser.parse_table.states[state]
                        .goto
                        .get(non_terminal)
                        .unwrap();

                    state_stack.push(*next_state);

                    match **non_terminal {
                        NonTerminal::Grammar
                        | NonTerminal::GrammarRepetition
                        | NonTerminal::Value => {}
                        NonTerminal::ListRepetition => {
                            let mut children = Vec::with_capacity(rhs.syms.len());
                            for _ in 0..length {
                                if let Some(child) = node_stack.pop() {
                                    if child.is_non_terminal(NonTerminal::ListRepetition) {
                                        children.reverse();
                                        children.extend(child.collect_children());
                                        break;
                                    } else {
                                        children.push(child);
                                    }
                                }
                            }
                            let new_node = ParseTree::non_terminal(
                                **non_terminal,
                                children,
                                Span::new(0, 0, 1, 1),
                            );
                            node_stack.push(new_node);
                        }
                        _ => {
                            let mut children = Vec::with_capacity(rhs.syms.len());
                            for _ in 0..length {
                                if let Some(child) = node_stack.pop() {
                                    children.push(child);
                                }
                            }
                            children.reverse();
                            let new_node = ParseTree::non_terminal(
                                **non_terminal,
                                children,
                                Span::new(0, 0, 1, 1),
                            );
                            node_stack.push(new_node);
                        }
                    }
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
