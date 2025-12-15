use lalr::Rhs;
use relex::Token;

use crate::common::action::Action;
use crate::common::symbol_table::{NonTerminal, Terminal};
use crate::generator::parse_tree::{ParseError, ParseTree, Span};
use crate::generator::symbol_table::symbol_table;

pub struct ParseTreeAction {
    node_stack: Vec<ParseTree>,
}

impl ParseTreeAction {
    pub const fn new() -> Self {
        Self {
            node_stack: Vec::new(),
        }
    }
}

impl Action for ParseTreeAction {
    type ParseResult = ParseTree;
    type ParseError = ParseError;

    fn on_reduce(&mut self, non_terminal: NonTerminal, rhs: &Rhs<Terminal, NonTerminal, ()>) {
        let table = symbol_table();

        let grammar = table.get_non_terminal_id("Grammar").unwrap();
        let grammar_repetition = table.get_non_terminal_id("GrammarRepetition").unwrap();
        let value = table.get_non_terminal_id("Value").unwrap();
        let list_repetition = table.get_non_terminal_id("ListRepetition").unwrap();
        let atom = table.get_non_terminal_id("Atom").unwrap();
        let expression_repetition = table.get_non_terminal_id("ExpressionRepetition").unwrap();
        let term_repetition = table.get_non_terminal_id("TermRepetition").unwrap();
        let list = table.get_non_terminal_id("List").unwrap();
        let expression = table.get_non_terminal_id("Expression").unwrap();
        let term = table.get_non_terminal_id("Term").unwrap();
        let factor_repetition = table.get_non_terminal_id("FactorRepetition").unwrap();

        let length = rhs.syms.len();

        if non_terminal == grammar
            || non_terminal == grammar_repetition
            || non_terminal == value
            || non_terminal == list_repetition
            || non_terminal == atom
            || non_terminal == expression_repetition
            || non_terminal == term_repetition
        {
            // Remove these nonterminals from the parse tree.
        } else if non_terminal == list
            || non_terminal == expression
            || non_terminal == term
            || non_terminal == factor_repetition
        {
            // Handle repetition flattening.
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
            let new_node = ParseTree::non_terminal(non_terminal, children, Span::new(0, 0, 1, 1));
            self.node_stack.push(new_node);
        } else {
            // Default case: build normal nonterminal node, filtering out empty factor repetitions.
            let mut children = Vec::with_capacity(rhs.syms.len());
            for _ in 0..length {
                if let Some(child) = self.node_stack.pop() {
                    // Filter out empty factor repetitions.
                    if !(child.is_non_terminal(factor_repetition) && child.is_empty()) {
                        children.push(child);
                    }
                }
            }
            children.reverse();
            let new_node = ParseTree::non_terminal(non_terminal, children, Span::new(0, 0, 1, 1));
            self.node_stack.push(new_node);
        }
    }

    fn on_shift(&mut self, token: Token<Terminal>) {
        let new_node =
            ParseTree::terminal(token.kind, token.text.to_string(), Span::new(0, 0, 1, 1));
        self.node_stack.push(new_node);
    }

    fn on_accept(&mut self) -> Self::ParseResult {
        let table = symbol_table();
        let grammar_nt = table.get_non_terminal_id("Grammar").unwrap();

        let children = std::mem::take(&mut self.node_stack);
        ParseTree::non_terminal(grammar_nt, children, Span::new(0, 0, 1, 1))
    }
}
