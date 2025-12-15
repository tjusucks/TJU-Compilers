use lalr::Rhs;
use relex::Token;

use crate::parse_tree::{ParseError, ParseTree, Span};
use crate::symbol::{NonTerminal, Terminal};

pub trait SemanticAction {
    type ParseResult;
    type ParseError;

    fn on_reduce(&mut self, non_terminal: NonTerminal, rhs: &Rhs<Terminal, NonTerminal, ()>);
    fn on_shift(&mut self, token: Token<Terminal>);
    fn on_accept(&mut self) -> Self::ParseResult;
    fn on_error(&mut self);
}

pub struct DefaultAction {
    node_stack: Vec<ParseTree>,
}

impl DefaultAction {
    pub const fn new() -> Self {
        Self {
            node_stack: Vec::new(),
        }
    }
}

impl SemanticAction for DefaultAction {
    type ParseResult = ParseTree;
    type ParseError = ParseError;

    fn on_reduce(&mut self, non_terminal: NonTerminal, rhs: &Rhs<Terminal, NonTerminal, ()>) {
        let length = rhs.syms.len();
        match non_terminal {
            NonTerminal::Grammar
            | NonTerminal::GrammarRepetition
            | NonTerminal::Value
            | NonTerminal::ListRepetition
            | NonTerminal::Atom
            | NonTerminal::ExpressionRepetition
            | NonTerminal::TermRepetition => {}
            NonTerminal::List
            | NonTerminal::Expression
            | NonTerminal::Term
            | NonTerminal::FactorRepetition => {
                let mut children = Vec::with_capacity(rhs.syms.len());
                for _ in 0..length {
                    if let Some(child) = self.node_stack.pop() {
                        if child.is_non_terminal(non_terminal) {
                            children.extend(child.collect_children().into_iter().rev());
                        } else {
                            children.push(child);
                        }
                    }
                }
                children.reverse();
                let new_node =
                    ParseTree::non_terminal(non_terminal, children, Span::new(0, 0, 1, 1));
                self.node_stack.push(new_node);
            }
            _ => {
                let mut children = Vec::with_capacity(rhs.syms.len());
                for _ in 0..length {
                    if let Some(child) = self.node_stack.pop()
                        && (!child.is_non_terminal(NonTerminal::FactorRepetition)
                            || !child.is_empty())
                    {
                        children.push(child);
                    }
                }
                children.reverse();
                let new_node =
                    ParseTree::non_terminal(non_terminal, children, Span::new(0, 0, 1, 1));
                self.node_stack.push(new_node);
            }
        }
    }

    fn on_shift(&mut self, token: Token<Terminal>) {
        let new_node =
            ParseTree::terminal(token.kind, token.text.to_string(), Span::new(0, 0, 1, 1));
        self.node_stack.push(new_node);
    }

    fn on_accept(&mut self) -> Self::ParseResult {
        let children = std::mem::take(&mut self.node_stack);
        ParseTree::non_terminal(NonTerminal::Grammar, children, Span::new(0, 0, 1, 1))
    }

    fn on_error(&mut self) {
        todo!()
    }
}

// pub struct EmptyAction;

// impl SemanticAction for EmptyAction {
//     type ParseResult = ();
//     type ParseError = ();

//     fn on_reduce(&mut self, _: NonTerminal, _: &Rhs<Terminal, NonTerminal, ()>) {}
//     fn on_shift(&mut self, _: Token<Terminal>) {}
//     fn on_accept(&mut self) -> Self::ParseResult {}
//     fn on_error(&mut self) {}
// }
