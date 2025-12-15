use crate::common::symbol_table::Terminal;

pub type TokenRules = Vec<Rule>;

#[derive(Debug, Clone)]
pub struct Rule {
    pub kind: Terminal,
    pub regex: String,
    pub skip: bool,
}
