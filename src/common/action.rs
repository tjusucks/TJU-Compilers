use relex::Token;

use crate::common::grammar::Rhs;
use crate::common::parse_tree::{ParseError, ParseTreeNode, Span};
use crate::common::symbol_table::{NonTerminal, Terminal};

pub trait Action {
    type ParseResult;
    type ParseError;

    fn on_reduce(&mut self, non_terminal: &NonTerminal, rhs: &Rhs<Terminal, NonTerminal, ()>);
    fn on_shift(&mut self, token: Token<Terminal>);
    fn on_accept(&mut self) -> Self::ParseResult;
}

pub struct DefaultAction {
    node_stack: Vec<ParseTreeNode>,
    start_symbol: NonTerminal,
}

impl DefaultAction {
    #[must_use]
    pub const fn new(start_symbol: NonTerminal) -> Self {
        Self {
            node_stack: Vec::new(),
            start_symbol,
        }
    }
}

impl Action for DefaultAction {
    type ParseResult = ParseTreeNode;
    type ParseError = ParseError;

    fn on_reduce(
        &mut self,
        non_terminal: &NonTerminal,
        rhs: &crate::common::grammar::Rhs<Terminal, NonTerminal, ()>,
    ) {
        // Build nonterminal node.
        let length = rhs.syms.len();
        let mut children = Vec::with_capacity(length);
        for _ in 0..length {
            if let Some(child) = self.node_stack.pop() {
                // Filter out empty factor repetitions.
                children.push(child);
            }
        }
        children.reverse();
        let new_node =
            ParseTreeNode::non_terminal(non_terminal.clone(), children, Span::new(0, 0, 1, 1));
        self.node_stack.push(new_node);
    }

    fn on_shift(&mut self, token: Token<Terminal>) {
        let new_node =
            ParseTreeNode::terminal(token.kind, token.text.to_string(), Span::new(0, 0, 1, 1));
        self.node_stack.push(new_node);
    }

    fn on_accept(&mut self) -> Self::ParseResult {
        let children = std::mem::take(&mut self.node_stack);
        ParseTreeNode::non_terminal(self.start_symbol.clone(), children, Span::new(0, 0, 1, 1))
    }
}
