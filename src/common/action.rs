use lalr::Rhs;
use relex::Token;

use crate::common::symbol_table::{NonTerminal, Terminal};

pub trait Action {
    type ParseResult;
    type ParseError;

    fn on_reduce(&mut self, non_terminal: NonTerminal, rhs: &Rhs<Terminal, NonTerminal, ()>);
    fn on_shift(&mut self, token: Token<Terminal>);
    fn on_accept(&mut self) -> Self::ParseResult;
    // fn on_error(&mut self);
}
