use std::iter;

use relex::{Recognizer, RecognizerBuilder, Rule, Token};

use crate::common::symbol_table::Terminal;
use crate::common::token_rules::TokenRules;

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

    pub fn tokenize(self, input: &str) -> impl Iterator<Item = Token<'_, Terminal>> {
        let iterator = self.recognizer.into_lexer(input, 0);
        iterator.chain(iter::once(Token::eof(input)))
    }
}
