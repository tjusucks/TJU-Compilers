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

    /// Get the span of this node.
    pub const fn span(&self) -> Span {
        match self {
            Self::Terminal { span, .. } | Self::NonTerminal { span, .. } => *span,
        }
    }

    /// Get all terminal tokens in order (for reconstructing source).
    pub fn terminals(&self) -> Vec<&Self> {
        let mut result = Vec::new();
        self.collect_terminals(&mut result);
        result
    }

    fn collect_terminals<'a>(&'a self, result: &mut Vec<&'a Self>) {
        match self {
            Self::Terminal { .. } => result.push(self),
            Self::NonTerminal { children, .. } => {
                for child in children {
                    child.collect_terminals(result);
                }
            }
        }
    }
}
