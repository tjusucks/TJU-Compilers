use lalr::Rhs;

use crate::parse_tree::{ParseTree, Span};
use crate::symbol::{NonTerminal, Terminal};

pub trait SemanticAction {
    fn on_reduce(
        &mut self,
        non_terminal: NonTerminal,
        rhs: &Rhs<Terminal, NonTerminal, ()>,
        node_stack: &mut Vec<ParseTree>,
    );
    // fn on_shift(&mut self);
    // fn on_accept(&mut self);
    // fn on_error(&mut self);
}

pub struct DefaultAction;

impl SemanticAction for DefaultAction {
    fn on_reduce(
        &mut self,
        non_terminal: NonTerminal,
        rhs: &Rhs<Terminal, NonTerminal, ()>,
        node_stack: &mut Vec<ParseTree>,
    ) {
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
                    if let Some(child) = node_stack.pop() {
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
                node_stack.push(new_node);
            }
            _ => {
                let mut children = Vec::with_capacity(rhs.syms.len());
                for _ in 0..length {
                    if let Some(child) = node_stack.pop()
                        && (!child.is_non_terminal(NonTerminal::FactorRepetition)
                            || !child.is_empty())
                    {
                        children.push(child);
                    }
                }
                children.reverse();
                let new_node =
                    ParseTree::non_terminal(non_terminal, children, Span::new(0, 0, 1, 1));
                node_stack.push(new_node);
            }
        }
    }
}

// pub struct EmptyAction;

// impl SemanticAction for EmptyAction {
//     fn on_reduce(
//         &mut self,
//         _: NonTerminal,
//         _: &Rhs<Terminal, NonTerminal, ()>,
//         _: &mut Vec<ParseTree>,
//     ) {
//     }
// }
