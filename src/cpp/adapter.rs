use std::sync::Arc;

use relex::{Token, TokenKind};

use crate::common::parse_tree::Span;
use crate::common::symbol_table::Terminal;
use crate::compiler::lexer::LocatedToken;
use crate::cpp::lexer::ffi;

pub trait LexerAdapter<'a> {
    fn tokenize(&self, input: &'a str) -> Vec<LocatedToken<'a>>;
}

pub struct CppLexerAdapter;

#[allow(clippy::cast_sign_loss)]
impl<'a> LexerAdapter<'a> for CppLexerAdapter {
    fn tokenize(&self, input: &'a str) -> Vec<LocatedToken<'a>> {
        let cpp_tokens = ffi::tokenize(input);
        let mut offset = 0;
        cpp_tokens
            .iter()
            .map(|token| {
                let kind_str = token.get_kind();
                let kind = if kind_str.is_empty() {
                    TokenKind::unrecognized()
                } else {
                    Terminal(Arc::from(kind_str))
                };
                let text = token.get_value();
                let leaked: &'static str = Box::leak(text.into_boxed_str());
                let start = offset;
                let end = start + leaked.len();
                offset = end;

                let line = token.get_line() as usize;
                let column = token.get_column() as usize;
                let span = Span {
                    start,
                    end,
                    line,
                    column,
                };
                LocatedToken {
                    token: Token::from_text(kind, leaked, start),
                    span,
                }
            })
            .collect()
    }
}
