use std::fmt::{self, Display};

use crate::common::symbol_table::{NonTerminal, Terminal};

/// Source location information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
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

impl Span {
    #[must_use]
    pub const fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

impl ParseTreeNode {
    #[must_use]
    pub const fn non_terminal(symbol: NonTerminal, children: Vec<Self>, span: Span) -> Self {
        Self::NonTerminal {
            symbol,
            children,
            span,
        }
    }

    #[must_use]
    pub const fn terminal(token: Terminal, lexeme: String, span: Span) -> Self {
        Self::Terminal {
            token,
            lexeme,
            span,
        }
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Terminal { .. } => true,
            Self::NonTerminal { children, .. } => children.is_empty(),
        }
    }

    #[must_use]
    pub fn get_lexeme(&self) -> String {
        match self {
            Self::Terminal { lexeme, .. } => lexeme.clone(),
            Self::NonTerminal { .. } => String::new(),
        }
    }

    #[must_use]
    pub fn is_terminal(&self, token: &Terminal) -> bool {
        matches!(self, Self::Terminal { token: t, .. } if t == token)
    }

    #[must_use]
    pub fn is_non_terminal(&self, symbol: &NonTerminal) -> bool {
        matches!(self, Self::NonTerminal { symbol: s, .. } if s == symbol)
    }

    #[must_use]
    pub fn get_children(&self) -> &[Self] {
        match self {
            Self::Terminal { .. } => &[],
            Self::NonTerminal { children, .. } => children,
        }
    }

    #[must_use]
    pub fn collect_children(self) -> Vec<Self> {
        match self {
            Self::Terminal { .. } => Vec::new(),
            Self::NonTerminal { children, .. } => children,
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(str) | Self::Regex(str) | Self::Identifier(str) => {
                write!(f, "{str}")
            }
            Self::Epsilon => write!(f, "EPSILON"),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = self.span {
            write!(
                f,
                "Parse Error at input:{}:{}, {}",
                span.line, span.column, self.message
            )
        } else {
            write!(f, "Parse Error: {}", self.message)
        }
    }
}

impl Display for ParseTreeNode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_sexpr(
            node: &ParseTreeNode,
            fmt: &mut fmt::Formatter<'_>,
            indent: usize,
        ) -> fmt::Result {
            let pad = "  ".repeat(indent);
            match node {
                ParseTreeNode::Terminal { token, lexeme, .. } => {
                    writeln!(fmt, "{pad}({} \"{}\")", token.0, lexeme)
                }
                ParseTreeNode::NonTerminal {
                    symbol, children, ..
                } => {
                    writeln!(fmt, "{pad}({}", symbol.0)?;
                    for child in children {
                        fmt_sexpr(child, fmt, indent + 1)?;
                    }
                    writeln!(fmt, "{pad})")
                }
            }
        }
        fmt_sexpr(self, fmt, 0)
    }
}
