use crate::common::symbol_table::Terminal;
use crate::compiler::lexer::LocatedToken;

pub struct Processor<'a, I: Iterator<Item = LocatedToken<'a>>> {
    iterator: I,
    previous_token: Option<LocatedToken<'a>>,
}

impl<'a, I: Iterator<Item = LocatedToken<'a>>> Processor<'a, I> {
    pub fn process(mut iterator: I) -> Self {
        let previous_token = iterator.next();
        Self {
            iterator,
            previous_token,
        }
    }
}

impl<'a, I: Iterator<Item = LocatedToken<'a>>> Iterator for Processor<'a, I> {
    type Item = LocatedToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.iterator.next();
        if let Some(mut previous_token) = self.previous_token.take() {
            if let Some(ref current_token) = token {
                if current_token.token.kind.0.as_ref() == "Equal"
                    && previous_token.token.kind.0.as_ref() == "Identifier"
                {
                    previous_token.token.kind = Terminal("LeftIdentifier".into());
                } else if current_token.token.kind.0.as_ref() == "="
                    && previous_token.token.kind.0.as_ref() == "IDENTIFIER"
                {
                    previous_token.token.kind = Terminal("LEFT_IDENTIFIER".into());
                }
            }
            self.previous_token = token;
            Some(previous_token)
        } else {
            None
        }
    }
}
