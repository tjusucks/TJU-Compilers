use lalr::{Grammar, LR1ParseTable, Rhs};

use crate::common::grammar_rules::GrammarRules;
use crate::common::symbol_table::{NonTerminal, Terminal};

pub struct ParseTable {
    pub grammar: &'static Grammar<Terminal, NonTerminal, ()>,
    pub parse_table: LR1ParseTable<'static, Terminal, NonTerminal, ()>,
}

impl ParseTable {
    pub fn new<ReduceFn, PriorityFn>(
        grammar_rules: &GrammarRules,
        reduce_on: ReduceFn,
        priority_of: PriorityFn,
    ) -> Self
    where
        ReduceFn: FnMut(&Rhs<Terminal, NonTerminal, ()>, Option<&Terminal>) -> bool,
        PriorityFn: FnMut(&Rhs<Terminal, NonTerminal, ()>, Option<&Terminal>) -> i32,
    {
        let mut grammar: Grammar<Terminal, NonTerminal, ()> = Grammar {
            rules: std::collections::BTreeMap::new(),
            start: grammar_rules.start_symbol.clone(),
        };

        for rule in &grammar_rules.rules {
            // The entire rule.rhs should be one production, not individual symbols
            grammar
                .rules
                .entry(rule.non_terminal.clone())
                .or_default()
                .push(Rhs {
                    syms: rule.rhs.clone(),
                    act: (),
                });
        }

        let grammar_ref: &'static Grammar<Terminal, NonTerminal, ()> = Box::leak(Box::new(grammar));
        let parse_table = match grammar_ref.lalr1(reduce_on, priority_of) {
            Ok(parse_table) => parse_table,
            Err(conflict) => {
                panic!("Grammar is not LALR(1), conflict detected: {conflict:?}");
            }
        };

        Self {
            grammar: grammar_ref,
            parse_table,
        }
    }
}
