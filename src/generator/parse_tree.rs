use std::fmt;

use crate::common::symbol_table::{NonTerminal, Terminal};
use crate::generator::symbol_table::symbol_table;

/// Source location information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

/// Parse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub span: Option<Span>,
}

/// Parse tree node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseTreeNode {
    Terminal {
        token: Terminal,
        lexeme: String,
        span: Span,
    },
    NonTerminal {
        symbol: NonTerminal,
        children: Vec<ParseTreeNode>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Literal(String),    // Literals like "="
    Regex(String),      // Regexes like /[a-z]+/
    Identifier(String), // Identifiers like "expression"
    Epsilon,            // Empty production
}

impl ParseTreeNode {
    pub const fn non_terminal(symbol: NonTerminal, children: Vec<Self>, span: Span) -> Self {
        Self::NonTerminal {
            symbol,
            children,
            span,
        }
    }

    pub const fn terminal(token: Terminal, lexeme: String, span: Span) -> Self {
        Self::Terminal {
            token,
            lexeme,
            span,
        }
    }

    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Terminal { .. } => true,
            Self::NonTerminal { children, .. } => children.is_empty(),
        }
    }

    pub fn get_lexeme(&self) -> String {
        match self {
            Self::Terminal { lexeme, .. } => lexeme.clone(),
            Self::NonTerminal { .. } => "".to_string(),
        }
    }

    pub fn to_symbol(&self) -> Result<Symbol, String> {
        match self {
            Self::Terminal { token, lexeme, .. } => {
                let literal = symbol_table().get_terminal_id("Literal").unwrap();
                let regex = symbol_table().get_terminal_id("Regex").unwrap();
                let identifier = symbol_table().get_terminal_id("Identifier").unwrap();
                let epsilon = symbol_table().get_terminal_id("Empty").unwrap();
                if *token == literal {
                    Ok(Symbol::Literal(lexeme.clone()))
                } else if *token == regex {
                    Ok(Symbol::Regex(lexeme.clone()))
                } else if *token == identifier {
                    Ok(Symbol::Identifier(lexeme.clone()))
                } else if *token == epsilon {
                    Ok(Symbol::Epsilon)
                } else {
                    Err(format!("Unexpected terminal token: {:?}", token))
                }
            }
            Self::NonTerminal { symbol, .. } => Err(format!(
                "Cannot convert non-terminal symbol to symbol: {:?}",
                symbol
            )),
        }
    }

    pub fn is_terminal(&self, token: Terminal) -> bool {
        matches!(self, Self::Terminal { token: t, .. } if *t == token)
    }

    pub fn is_non_terminal(&self, symbol: NonTerminal) -> bool {
        matches!(self, Self::NonTerminal { symbol: s, .. } if *s == symbol)
    }

    pub fn get_children(&self) -> &[Self] {
        match self {
            Self::Terminal { .. } => &[],
            Self::NonTerminal { children, .. } => children,
        }
    }

    pub fn collect_children(self) -> Vec<Self> {
        match self {
            Self::Terminal { .. } => Vec::new(),
            Self::NonTerminal { children, .. } => children,
        }
    }

    pub fn get_terms(&self) -> Result<Vec<Vec<Symbol>>, String> {
        // expression  = term { "|" term }
        let expression = symbol_table().get_non_terminal_id("Expression").unwrap();
        let term = symbol_table().get_non_terminal_id("Term").unwrap();
        let pipe = symbol_table().get_terminal_id("Pipe").unwrap();

        // Only collect terms if the node is an expression.
        if !self.is_non_terminal(expression) {
            return Err("Collect terms call on non-expression node".to_string());
        }
        let children = self.get_children();

        if children.is_empty() {
            return Err("Expression has no children".to_string());
        }

        let mut terminals = Vec::new();
        for child in children {
            if child.is_non_terminal(term) {
                terminals.push(child.get_factors().unwrap());
            } else if !child.is_terminal(pipe) {
                return Err(format!("Unexpected child in expression: {child}"));
            }
        }
        Ok(terminals)
    }

    pub fn get_factors(&self) -> Result<Vec<Symbol>, String> {
        // term  = factor { factor } | EMPTY
        let term = symbol_table().get_non_terminal_id("Term").unwrap();
        let factor = symbol_table().get_non_terminal_id("Factor").unwrap();
        let empty = symbol_table().get_terminal_id("Empty").unwrap();

        // Only collect factors if the node is a term.
        if !self.is_non_terminal(term) {
            return Err("Collect factors call on non-term node".to_string());
        }
        let children = self.get_children();

        if children.len() == 1 && children[0].is_terminal(empty) {
            // Child is epsilon.
            Ok(vec![Symbol::Epsilon])
        } else if children[0].is_non_terminal(factor) {
            // Children are factors.
            let mut terminals = Vec::new();
            for child in children {
                terminals.push(child.get_atom().unwrap());
            }
            Ok(terminals)
        } else {
            Err(format!("Unexpected children in term: {children:?}"))
        }
    }

    pub fn get_atom(&self) -> Result<Symbol, String> {
        // factor = { WHITESPACE } atom { WHITESPACE } [ lookahead ]
        let factor = symbol_table().get_non_terminal_id("Factor").unwrap();
        let factor_repetition = symbol_table()
            .get_non_terminal_id("FactorRepetition")
            .unwrap();

        // Only collect atoms if the node is a factor.
        if !self.is_non_terminal(factor) {
            return Err("Collect atom call on non-factor node".to_string());
        }
        let children = self.get_children();
        if children.is_empty() {
            return Err("Factor has no children".to_string());
        }
        // Skip leading and trailing repetitions.
        if children[0].is_non_terminal(factor_repetition) {
            // Skip leading repetition.
            children[1].to_symbol()
        } else {
            children[0].to_symbol()
        }
    }
}

impl fmt::Display for ParseTreeNode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::generator::symbol_table::symbol_table;
        let table = symbol_table();

        fn fmt_sexpr(
            node: &ParseTreeNode,
            fmt: &mut fmt::Formatter<'_>,
            indent: usize,
            table: &crate::common::symbol_table::SymbolTable,
        ) -> fmt::Result {
            let pad = "  ".repeat(indent);
            match node {
                ParseTreeNode::Terminal { token, lexeme, .. } => {
                    let terminal_name = table
                        .get_terminal_name(*token)
                        .unwrap_or("UNKNOWN_TERMINAL");
                    writeln!(fmt, "{pad}({terminal_name} \"{lexeme}\")")
                }
                ParseTreeNode::NonTerminal {
                    symbol, children, ..
                } => {
                    let nonterminal_name = table
                        .get_non_terminal_name(*symbol)
                        .unwrap_or("UNKNOWN_NONTERMINAL");
                    writeln!(fmt, "{pad}({nonterminal_name}")?;
                    for child in children {
                        fmt_sexpr(child, fmt, indent + 1, table)?;
                    }
                    writeln!(fmt, "{pad})")
                }
            }
        }
        fmt_sexpr(self, fmt, 0, table)
    }
}
