use std::fmt;

use crate::common::symbol_table::{NonTerminal, Terminal};

/// Source location information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum ParseTree {
    Terminal {
        token: Terminal,
        lexeme: String,
        span: Span,
    },
    NonTerminal {
        symbol: NonTerminal,
        children: Vec<ParseTree>,
        span: Span,
    },
}

impl ParseTree {
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

    pub fn is_terminal(&self, token: Terminal) -> bool {
        matches!(self, Self::Terminal { token: t, .. } if *t == token)
    }

    pub fn is_non_terminal(&self, symbol: NonTerminal) -> bool {
        matches!(self, Self::NonTerminal { symbol: s, .. } if *s == symbol)
    }

    pub fn collect_children(self) -> Vec<Self> {
        match self {
            Self::Terminal { .. } => Vec::new(),
            Self::NonTerminal { children, .. } => children,
        }
    }
}

impl fmt::Display for ParseTree {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::generator::symbol_table::symbol_table;
        let table = symbol_table();

        fn fmt_sexpr(
            node: &ParseTree,
            fmt: &mut fmt::Formatter<'_>,
            indent: usize,
            table: &crate::common::symbol_table::SymbolTable,
        ) -> fmt::Result {
            let pad = "  ".repeat(indent);
            match node {
                ParseTree::Terminal { token, lexeme, .. } => {
                    let terminal_name = table
                        .get_terminal_name(*token)
                        .unwrap_or("UNKNOWN_TERMINAL");
                    writeln!(fmt, "{pad}({terminal_name} \"{lexeme}\")")
                }
                ParseTree::NonTerminal {
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
