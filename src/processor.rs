use relex::Token;

use crate::symbol::Terminal;

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
        let curr = self.iterator.next();
        if let Some(mut previous_token) = self.previous_token.take() {
            if let Some(ref current_token) = curr
                && current_token.kind == Terminal::Equal
                && previous_token.kind == Terminal::Identifier
            {
                previous_token.kind = Terminal::LeftIdentifier;
            }
            self.previous_token = curr;
            Some(previous_token)
        } else {
            None
        }
    }
}
