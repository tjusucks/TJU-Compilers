use lalr::Symbol;

use crate::common::symbol_table::{NonTerminal, Terminal};

pub struct GrammarRules {
    pub start_symbol: NonTerminal,
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub non_terminal: NonTerminal,
    pub rhs: Vec<Symbol<Terminal, NonTerminal>>,
}
