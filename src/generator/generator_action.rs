use std::collections::HashMap;
use std::collections::hash_map::Entry;

use regex::escape;
use relex::Token;

use crate::common::action::Action;
use crate::common::grammar_rules::GrammarRules;
use crate::common::symbol_table::{NonTerminal, SymbolTable, Terminal};
use crate::common::token_rules::{Rule, TokenRules};
use crate::generator::parse_tree::{ParseError, ParseTreeNode, Span, Symbol};
use crate::generator::symbol_table::symbol_table;

#[derive(Debug)]
pub struct GeneratorResult {
    pub symbol_table: SymbolTable,
    pub grammar_rules: GrammarRules,
    pub token_rules: TokenRules,
    pub parse_tree: ParseTreeNode,
}

impl GeneratorResult {
    pub const fn new(
        symbol_table: SymbolTable,
        grammar_rules: GrammarRules,
        token_rules: TokenRules,
        parse_tree: ParseTreeNode,
    ) -> Self {
        Self {
            symbol_table,
            grammar_rules,
            token_rules,
            parse_tree,
        }
    }
}

pub struct GeneratorAction {
    pub symbol_table: SymbolTable,
    pub grammar_rules: GrammarRules,
    pub token_rules: TokenRules,
    productions: HashMap<String, Vec<Vec<Symbol>>>,
    node_stack: Vec<ParseTreeNode>,
}

impl GeneratorAction {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::default(),
            grammar_rules: GrammarRules::default(),
            token_rules: TokenRules::default(),
            productions: HashMap::new(),
            node_stack: Vec::new(),
        }
    }

    fn add_production(&mut self, lhs: String, rhs: Vec<Vec<Symbol>>) {
        println!("Adding production: {} -> {:?}", lhs, rhs);

        match self.productions.entry(lhs) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().extend(rhs);
            }
            Entry::Vacant(entry) => {
                entry.insert(rhs);
            }
        }
    }

    fn generate_result(&mut self) {
        // Collect all symbols and classify them.
        let mut regex_patterns = HashMap::new();
        let mut literal_patterns: HashMap<String, Vec<String>> = HashMap::new();
        for (lhs, rhs_alternatives) in self.productions.iter() {
            let mut is_terminal = true;

            // Check if this rule defines a terminal or nonterminal.
            for symbols in rhs_alternatives {
                for symbol in symbols {
                    match symbol {
                        Symbol::Literal(literal) => {
                            // Literals are OK for terminals.
                            self.symbol_table
                                .insert_terminal(GeneratorAction::unquote(literal).to_string());
                            if let Some(literal_pattern) = literal_patterns.get_mut(lhs) {
                                literal_pattern.push(literal.clone());
                            } else {
                                literal_patterns.insert(lhs.clone(), vec![literal.clone()]);
                            }
                        }
                        Symbol::Regex(regex) => {
                            if regex_patterns.contains_key(lhs) {
                                panic!("Multiple regex patterns for {}", lhs);
                            }
                            regex_patterns.insert(lhs.clone(), regex.clone());
                        }
                        Symbol::Identifier(_) => {
                            // If RHS contains identifiers, LHS is a nonterminal.
                            is_terminal = false;
                        }
                        Symbol::Epsilon => {
                            // Epsilon is OK for both.
                        }
                    }
                }
            }

            // Build symbol table.
            if is_terminal {
                let terminal = self.symbol_table.insert_terminal(lhs.clone());
                let regex_pattern = regex_patterns.get(lhs);
                let literal_pattern = literal_patterns.get(lhs);
                if regex_pattern.is_some() && literal_pattern.is_some() {
                    panic!("Regex patterns and literal patterns cannot be used together");
                } else if regex_pattern.is_some() {
                    self.token_rules.push(Rule {
                        kind: terminal,
                        regex: regex_pattern.unwrap().clone(),
                        skip: false,
                    });
                } else if let Some(literals) = literal_pattern {
                    self.token_rules.push(Rule {
                        kind: terminal,
                        regex: GeneratorAction::combine_literals(literals),
                        skip: false,
                    });
                }
            } else {
                self.symbol_table.insert_non_terminal(lhs.clone());
                if regex_patterns.contains_key(lhs) {
                    panic!("Regex patterns are not allowed for nonterminals");
                }
            }
        }
    }

    fn unquote(str: &str) -> &str {
        if str.len() >= 2
            && (str.starts_with('"') && str.ends_with('"')
                || str.starts_with('\'') && str.ends_with('\''))
        {
            &str[1..str.len() - 1]
        } else {
            str
        }
    }

    fn combine_literals(literals: &[String]) -> String {
        let patterns: Vec<String> = literals
            .iter()
            .map(|literal| escape(GeneratorAction::unquote(literal)))
            .collect();

        match patterns.len() {
            0 => String::new(),
            1 => patterns[0].clone(),
            _ => format!("({})", patterns.join("|")),
        }
    }
}

impl Action for GeneratorAction {
    type ParseResult = GeneratorResult;
    type ParseError = ParseError;

    fn on_reduce(&mut self, non_terminal: NonTerminal, rhs: &lalr::Rhs<Terminal, NonTerminal, ()>) {
        let table = symbol_table();

        let grammar = table.get_non_terminal_id("Grammar").unwrap();
        let grammar_repetition = table.get_non_terminal_id("GrammarRepetition").unwrap();
        let atom = table.get_non_terminal_id("Atom").unwrap();
        let list = table.get_non_terminal_id("List").unwrap();
        let expression = table.get_non_terminal_id("Expression").unwrap();
        let term = table.get_non_terminal_id("Term").unwrap();
        let factor_repetition = table.get_non_terminal_id("FactorRepetition").unwrap();

        let length = rhs.syms.len();

        if non_terminal == grammar || non_terminal == grammar_repetition || non_terminal == atom {
            // Remove these nonterminals from the parse tree.
        } else if non_terminal == list
            || non_terminal == expression
            || non_terminal == term
            || non_terminal == factor_repetition
        {
            // Handle repetition flattening.
            let mut children = Vec::with_capacity(rhs.syms.len());
            for _ in 0..length {
                if let Some(child) = self.node_stack.pop() {
                    if child.is_non_terminal(non_terminal) {
                        children.extend(child.collect_children().into_iter().rev());
                    } else {
                        children.push(child);
                    }
                }
            }
            children.reverse();
            let new_node =
                ParseTreeNode::non_terminal(non_terminal, children, Span::new(0, 0, 1, 1));
            self.node_stack.push(new_node);
        } else {
            // Default case: build normal nonterminal node, filtering out empty factor repetitions.
            let mut children = Vec::with_capacity(rhs.syms.len());
            for _ in 0..length {
                if let Some(child) = self.node_stack.pop() {
                    // Filter out empty factor repetitions.
                    if !(child.is_non_terminal(factor_repetition) && child.is_empty()) {
                        children.push(child);
                    }
                }
            }
            children.reverse();
            let new_node =
                ParseTreeNode::non_terminal(non_terminal, children, Span::new(0, 0, 1, 1));
            self.node_stack.push(new_node);
        }

        if non_terminal == table.get_non_terminal_id("Rule").unwrap() {
            let node = self.node_stack.last().unwrap();
            let children = node.get_children();

            // rule = IDENTIFIER "=" expression
            assert!(children.len() == 3);
            self.add_production(children[0].get_lexeme(), children[2].get_terms().unwrap());
        }
    }

    fn on_shift(&mut self, token: Token<Terminal>) {
        let new_node =
            ParseTreeNode::terminal(token.kind, token.text.to_string(), Span::new(0, 0, 1, 1));
        self.node_stack.push(new_node);
    }

    fn on_accept(&mut self) -> Self::ParseResult {
        println!("Productions: {:?}", self.productions);
        self.generate_result();

        let table = symbol_table();
        let grammar_nt = table.get_non_terminal_id("Grammar").unwrap();

        let children = std::mem::take(&mut self.node_stack);
        let root_node = ParseTreeNode::non_terminal(grammar_nt, children, Span::new(0, 0, 1, 1));
        GeneratorResult::new(
            std::mem::take(&mut self.symbol_table),
            std::mem::take(&mut self.grammar_rules),
            std::mem::take(&mut self.token_rules),
            root_node,
        )
    }
}
