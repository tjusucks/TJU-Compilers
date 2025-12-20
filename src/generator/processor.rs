use relex::Token;

use crate::common::symbol_table::Terminal;

pub struct Processor<I: Iterator<Item = Token<'static, Terminal>>> {
    iterator: I,
    previous_token: Option<Token<'static, Terminal>>,
}

impl<I: Iterator<Item = Token<'static, Terminal>>> Processor<I> {
    pub fn process(mut iterator: I) -> Self {
        let previous_token = iterator.next();
        Self {
            iterator,
            previous_token,
        }
    }
}

impl<I: Iterator<Item = Token<'static, Terminal>>> Iterator for Processor<I> {
    type Item = Token<'static, Terminal>;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.iterator.next();
        if let Some(mut previous_token) = self.previous_token.take() {
            if let Some(ref current_token) = token {
                if current_token.kind.0.as_ref() == "Equal"
                    && previous_token.kind.0.as_ref() == "Identifier"
                {
                    previous_token.kind = Terminal("LeftIdentifier".into());
                } else if current_token.kind.0.as_ref() == "="
                    && previous_token.kind.0.as_ref() == "IDENTIFIER"
                {
                    previous_token.kind = Terminal("LEFT_IDENTIFIER".into());
                }
            }
            self.previous_token = token;
            Some(previous_token)
        } else {
            None
        }
    }
}
