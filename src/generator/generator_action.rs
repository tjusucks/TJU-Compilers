use std::collections::HashMap;
use std::collections::hash_map::Entry;

use relex::Token;

use crate::common::action::Action;
use crate::common::grammar_rules::GrammarRules;
use crate::common::symbol_table::{NonTerminal, SymbolTable, Terminal};
use crate::common::token_rules::TokenRules;
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
