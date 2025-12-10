use std::fmt;

use crate::symbol::{NonTerminal, Terminal};

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
}

impl fmt::Display for ParseTree {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_sexpr(node: &ParseTree, fmt: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
            let pad = "  ".repeat(indent);
            match node {
                ParseTree::Terminal { token, lexeme, .. } => {
                    writeln!(fmt, "{}({:?} \"{}\")", pad, token, lexeme)
                }
                ParseTree::NonTerminal {
                    symbol, children, ..
                } => {
                    writeln!(fmt, "{}({:?}", pad, symbol)?;
                    for child in children {
                        fmt_sexpr(child, fmt, indent + 1)?;
                    }
                    writeln!(fmt, "{})", pad)
                }
            }
        }
        fmt_sexpr(self, fmt, 0)
    }
}
