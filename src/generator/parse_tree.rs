use std::sync::Arc;

use crate::common::parse_tree::{ParseTreeNode, Symbol};
use crate::common::symbol_table::{NonTerminal, Terminal};

impl ParseTreeNode {
    /// Converts a terminal parse tree node to a Symbol.
    ///
    /// # Errors
    /// Returns an error message if the terminal token is not recognized.
    pub fn to_symbol(&self) -> Result<Symbol, String> {
        match self {
            Self::Terminal { token, lexeme, .. } => {
                let literal = Terminal(Arc::from("Literal"));
                let regex = Terminal(Arc::from("Regex"));
                let identifier = Terminal(Arc::from("Identifier"));
                let epsilon = Terminal(Arc::from("Empty"));
                if *token == literal {
                    Ok(Symbol::Literal(lexeme.clone()))
                } else if *token == regex {
                    Ok(Symbol::Regex(lexeme.clone()))
                } else if *token == identifier {
                    Ok(Symbol::Identifier(lexeme.clone()))
                } else if *token == epsilon {
                    Ok(Symbol::Epsilon)
                } else {
                    Err(format!("Unexpected terminal token: {token:?}"))
                }
            }
            Self::NonTerminal { symbol, .. } => Err(format!(
                "Cannot convert non-terminal symbol to symbol: {symbol:?}"
            )),
        }
    }

    /// Gets terms from an expression node in the parse tree.
    ///
    /// # Errors
    /// Returns an error if the node is not an expression, has no children,
    /// or has unexpected child types.
    ///
    /// # Panics
    /// Panics if there's an error retrieving factors from child nodes.
    pub fn get_terms(&self) -> Result<Vec<Vec<Symbol>>, String> {
        // expression  = term { "|" term }
        let expression = NonTerminal(Arc::from("Expression"));
        let term = NonTerminal(Arc::from("Term"));
        let pipe = Terminal(Arc::from("Pipe"));

        // Only collect terms if the node is an expression.
        if !self.is_non_terminal(&expression) {
            return Err("Collect terms call on non-expression node".to_string());
        }
        let children = self.get_children();

        if children.is_empty() {
            return Err("Expression has no children".to_string());
        }

        let mut terminals = Vec::new();
        for child in children {
            if child.is_non_terminal(&term) {
                terminals.push(
                    child
                        .get_factors()
                        .expect("Failed to get factors from term"),
                );
            } else if !child.is_terminal(&pipe) {
                return Err(format!("Unexpected child in expression: {child}"));
            }
        }
        Ok(terminals)
    }

    /// Gets factors from a term node in the parse tree.
    ///
    /// # Errors
    /// Returns an error if the node is not a term or has unexpected children.
    ///
    /// # Panics
    /// Panics if there's an error retrieving atoms from child nodes.
    pub fn get_factors(&self) -> Result<Vec<Symbol>, String> {
        // term  = factor { factor } | EMPTY
        let term = NonTerminal(Arc::from("Term"));
        let factor = NonTerminal(Arc::from("Factor"));
        let empty = Terminal(Arc::from("Empty"));

        // Only collect factors if the node is a term.
        if !self.is_non_terminal(&term) {
            return Err("Collect factors call on non-term node".to_string());
        }
        let children = self.get_children();

        if children.len() == 1 && children[0].is_terminal(&empty) {
            // Child is epsilon.
            Ok(vec![Symbol::Epsilon])
        } else if children[0].is_non_terminal(&factor) {
            // Children are factors.
            let mut terminals = Vec::new();
            for child in children {
                terminals.push(child.get_atom().expect("Failed to get atom from factor"));
            }
            Ok(terminals)
        } else {
            Err(format!("Unexpected children in term: {children:?}"))
        }
    }

    /// Gets atom from a factor node in the parse tree.
    ///
    /// # Errors
    /// Returns an error if the node is not a factor or has no children.
    pub fn get_atom(&self) -> Result<Symbol, String> {
        // factor = { WHITESPACE } atom { WHITESPACE } [ lookahead ]
        let factor = NonTerminal(Arc::from("Factor"));
        let factor_repetition = NonTerminal(Arc::from("FactorRepetition"));

        // Only collect atoms if the node is a factor.
        if !self.is_non_terminal(&factor) {
            return Err("Collect atom call on non-factor node".to_string());
        }
        let children = self.get_children();
        if children.is_empty() {
            return Err("Factor has no children".to_string());
        }
        // Skip leading and trailing repetitions.
        if children[0].is_non_terminal(&factor_repetition) {
            // Skip leading repetition.
            children[1].to_symbol()
        } else {
            children[0].to_symbol()
        }
    }
}
