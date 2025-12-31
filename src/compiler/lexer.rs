use std::iter;

use relex::{Recognizer, RecognizerBuilder, Rule, Token};

use crate::common::parse_tree::Span;
use crate::common::symbol_table::Terminal;
use crate::common::token_rules::TokenRules;

/// A token with an attached source `Span` (start/end offsets plus line/column).
pub struct LocatedToken<'a> {
    pub token: Token<'a, Terminal>,
    pub span: Span,
}

pub struct Lexer {
    recognizer: Recognizer<Terminal>,
}

impl Lexer {
    /// Creates a new Lexer from token rules.
    ///
    /// # Panics
    ///
    /// This function will panic if any of the regex patterns in the token rules fail to compile.
    #[must_use]
    pub fn new(token_rules: &TokenRules) -> Self {
        let mut builder = RecognizerBuilder::new();
        for rule in token_rules {
            builder = builder.token(
                Rule::new(rule.kind.clone(), &rule.regex)
                    .expect("Failed to compile regex for lexer rule")
                    .skip(rule.skip),
            );
        }
        let recognizer = builder.build();
        Self { recognizer }
    }

    /// Compute (line, column) from byte offset.
    fn compute_line_col(input: &str, offset: usize) -> (usize, usize) {
        let end = offset.min(input.len());
        let prefix = &input[..end];

        // Number of newline characters gives the line index (1-based).
        let line = prefix.chars().filter(|&c| c == '\n').count() + 1;

        // Column = number of characters after the last newline in prefix + 1
        let col = prefix.rfind('\n').map_or_else(
            || prefix.chars().count() + 1,
            |last_newline_pos| prefix[last_newline_pos + 1..].chars().count() + 1,
        );

        (line, col)
    }

    /// Tokenizes the input string and returns an iterator of `LocatedToken`.
    ///
    /// # Panics
    ///
    /// Panics when an unrecognized token is encountered.
    pub fn tokenize(self, input: &str) -> impl Iterator<Item = LocatedToken<'_>> {
        let base_iter = self.recognizer.into_lexer(input, 0);

        // Map tokens into LocatedToken and panic immediately on UNRECOGNIZED.
        let mapped = base_iter.map(move |token: Token<'_, Terminal>| {
            // Compute line/column based on the token start offset.
            let (line, column) = Self::compute_line_col(input, token.start);
            let span = Span::new(token.start, token.end, line, column);

            assert!(
                token.kind.0.as_ref() != "<UNRECOGNIZED>",
                "Lexical error: unrecognized token {:?} at input:{}:{}.",
                token.text,
                span.line,
                span.column,
            );

            LocatedToken { token, span }
        });

        // Append EOF token at the end.
        let eof_token = {
            // create eof token and span at end of input
            let token = Token::eof(input);
            let (line, column) = Self::compute_line_col(input, token.start);
            let start = token.start;
            let end = token.end;
            LocatedToken {
                token,
                span: Span::new(start, end, line, column),
            }
        };

        mapped.chain(iter::once(eof_token))
    }
}
