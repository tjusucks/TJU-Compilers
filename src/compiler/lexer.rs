use std::iter;

use relex::{Recognizer, RecognizerBuilder, Rule, Token};

use crate::common::symbol_table::Terminal;
use crate::common::token_rules::TokenRules;

pub struct Lexer {
    recognizer: Recognizer<Terminal>,
}

impl Lexer {
    pub fn new(token_rules: &TokenRules) -> Self {
        let mut builder = RecognizerBuilder::new();
        for rule in token_rules {
            builder = builder.token(Rule::new(rule.kind, &rule.regex).unwrap().skip(rule.skip));
        }
        let recognizer = builder.build();
        Self { recognizer }
    }

    pub fn tokenize(self, input: &str) -> impl Iterator<Item = Token<'_, Terminal>> {
        let iterator = self.recognizer.into_lexer(input, 0);
        iterator.chain(iter::once(Token::eof(input)))
    }
}
