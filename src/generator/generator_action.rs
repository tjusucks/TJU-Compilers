use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use regex::escape;
use relex::Token;

use crate::common::action::Action;
use crate::common::grammar_rules::{GrammarRules, Rule as GrammarRule};
use crate::common::symbol_table::{NonTerminal, SymbolTable, Terminal};
use crate::common::token_rules::{Rule as TokenRule, TokenRules};
use crate::generator::parse_tree::{ParseError, ParseTreeNode, Span, Symbol};
use crate::generator::symbol_table::symbol_table;

#[derive(Debug)]
pub struct GeneratorResult {
    /// Symbol table containing all terminals and non terminals.
    pub symbol_table: SymbolTable,

    /// Grammar rules containing productions for non terminals.
    pub grammar_rules: GrammarRules,

    /// Token rules containing regex patterns for terminals.
    pub token_rules: TokenRules,

    /// Parse tree built from the input.
    pub parse_tree: ParseTreeNode,
}

#[derive(Default)]
pub struct GeneratorAction {
    /// The node stack used to build the parse tree.
    node_stack: Vec<ParseTreeNode>,

    /// Productions collected during parsing.
    productions: HashMap<String, Vec<Vec<Symbol>>>,

    /// Regex patterns collected for terminals.
    regex_patterns: HashMap<String, String>,

    /// Literal patterns collected for terminals.
    literal_patterns: HashMap<String, Vec<String>>,

    /// Intermediate results for generating the grammar rules.
    rules: HashMap<NonTerminal, Vec<Vec<String>>>,

    /// Set of non terminal names that appear on the LHS of any production (candidates for start symbol).
    lhs_non_terminals: HashSet<String>,

    /// Set of non terminal names that appear on the RHS of any production (cannot be start symbol).
    rhs_non_terminals: HashSet<String>,

    /// Symbol table containing all terminals and non terminals.
    symbol_table: SymbolTable,

    /// Grammar rules containing productions for non terminals.
    grammar_rules: GrammarRules,

    /// Token rules containing regex patterns for terminals.
    token_rules: TokenRules,
}

/// Unquote a string, removing surrounding quotes if they are present.
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

/// Strip surrounding slashes from a regex pattern.
fn strip(pattern: &str) -> &str {
    if pattern.starts_with('/') && pattern.ends_with('/') && pattern.len() > 2 {
        &pattern[1..pattern.len() - 1]
    } else {
        pattern
    }
}

/// Combine multiple literal patterns into a single regex pattern.
fn combine_literals(literals: &[String]) -> String {
    let patterns: Vec<String> = literals
        .iter()
        .map(|literal| escape(unquote(literal)))
        .collect();

    match patterns.len() {
        0 => String::new(),
        1 => patterns[0].clone(),
        _ => format!("({})", patterns.join("|")),
    }
}

/// Insert a terminal into the symbol table, ensuring uniqueness by appending a number if necessary.
fn insert_unique(symbol_table: &mut SymbolTable, base: &str) -> Terminal {
    let mut name = base.to_string();
    let mut counter = 0;
    while symbol_table.get_terminal_id(&name).is_some() {
        counter += 1;
        name = format!("{base}{counter}");
    }
    symbol_table.insert_terminal(name)
}

/// Converts a Symbol vector into a string vector.
fn symbols_to_strings(symbols: &[Vec<Symbol>]) -> Vec<Vec<String>> {
    symbols
        .iter()
        .map(|symbols| symbols.iter().map(ToString::to_string).collect())
        .collect()
}

impl GeneratorResult {
    #[must_use]
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

impl GeneratorAction {
    fn add_production(&mut self, lhs: String, rhs: Vec<Vec<Symbol>>) {
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
        for (lhs, rhs_alternatives) in &self.productions {
            let mut is_terminal = true;

            // Check if this rule defines a terminal or nonterminal.
            for symbols in rhs_alternatives {
                for symbol in symbols {
                    match symbol {
                        Symbol::Literal(literal) => {
                            // Literal is terminal.
                            let literal = unquote(literal).to_string();
                            let terminal = self.symbol_table.insert_terminal(literal.clone());

                            // Add token rule for the literal.
                            self.token_rules.push(TokenRule {
                                kind: terminal,
                                regex: escape(&literal),
                                skip: false,
                            });

                            // Add the literal to the literal patterns of the LHS.
                            self.literal_patterns
                                .entry(lhs.clone())
                                .or_default()
                                .push(literal.clone());
                        }
                        Symbol::Regex(regex) => {
                            // Terminal can only be defined by a single regex pattern.
                            assert!(
                                !self.regex_patterns.contains_key(lhs),
                                "Multiple regex patterns for {lhs}"
                            );

                            // Add the regex to regex patterns of the LHS.
                            self.regex_patterns.insert(lhs.clone(), regex.clone());
                        }
                        Symbol::Identifier(identifier) => {
                            // If RHS contains identifiers, LHS is a non terminal.
                            is_terminal = false;

                            // Add the identifier to the RHS non terminals set.
                            self.rhs_non_terminals.insert(identifier.clone());
                        }
                        Symbol::Epsilon => {}
                    }
                }
            }

            // Build symbol table.
            if is_terminal {
                // Add terminal to the symbol table.
                let terminal = self.symbol_table.insert_terminal(lhs.clone());

                // Get the collected patterns for the LHS terminal.
                let regex_pattern = self.regex_patterns.get(lhs);
                let literal_pattern = self.literal_patterns.get(lhs);

                // Add token rule for the terminal.
                if regex_pattern.is_some() && literal_pattern.is_some() {
                    panic!("Regex patterns and literal patterns cannot be used together");
                } else if regex_pattern.is_some() {
                    self.token_rules.push(TokenRule {
                        kind: terminal,
                        regex: strip(regex_pattern.unwrap()).to_string(),
                        skip: false,
                    });
                } else if let Some(literals) = literal_pattern {
                    self.token_rules.push(TokenRule {
                        kind: terminal,
                        regex: combine_literals(literals),
                        skip: false,
                    });
                }
            } else {
                // Add to LHS non terminals set as start symbol candidate.
                self.lhs_non_terminals.insert(lhs.clone());

                // Add non terminal to the symbol table.
                let non_terminal = self.symbol_table.insert_non_terminal(lhs.clone());
                assert!(
                    !self.regex_patterns.contains_key(lhs),
                    "Regex patterns are not allowed for non terminals"
                );

                // Add the production to the grammar rules.
                let string_rhs = symbols_to_strings(rhs_alternatives);
                match self.rules.entry(non_terminal) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().extend(string_rhs);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(string_rhs);
                    }
                }
            }
        }

        // Add token rules for whitespace and comments.
        let comment = insert_unique(&mut self.symbol_table, "Comment");
        let whitespace = insert_unique(&mut self.symbol_table, "Whitespace");
        self.token_rules.push(TokenRule {
            kind: comment,
            regex: r"#.*".to_string(),
            skip: true,
        });
        self.token_rules.push(TokenRule {
            kind: whitespace,
            regex: r"\s+".to_string(),
            skip: true,
        });

        // Build grammar rules.
        for (lhs, rhs_alternatives) in &self.rules {
            for rhs in rhs_alternatives {
                let mut lalr_symbols: Vec<lalr::Symbol<Terminal, NonTerminal>> = Vec::new();
                for symbol in rhs {
                    let symbol = unquote(symbol);
                    if let Some(non_terminal) = self.symbol_table.get_non_terminal_id(symbol) {
                        lalr_symbols.push(lalr::Symbol::Nonterminal(non_terminal));
                    } else if let Some(terminal) = self.symbol_table.get_terminal_id(symbol) {
                        lalr_symbols.push(lalr::Symbol::Terminal(terminal));
                    } else {
                        panic!("Symbol not found in symbol table: {symbol}");
                    }
                }
                self.grammar_rules.rules.push(GrammarRule {
                    non_terminal: *lhs,
                    rhs: lalr_symbols,
                });
            }
        }

        // Determine the start symbol by calculating set difference.
        let start_symbols: HashSet<String> = self
            .lhs_non_terminals
            .difference(&self.rhs_non_terminals)
            .cloned()
            .collect();

        // There should be exactly one start symbol.
        match start_symbols.len() {
            0 => panic!("No start symbol found"),
            1 => {
                let start_name = start_symbols.iter().next().unwrap();
                let start_symbol = self.symbol_table.get_non_terminal_id(start_name).unwrap();
                self.grammar_rules.start_symbol = start_symbol;
            }
            _ => panic!("Multiple start symbols found: {:?}", start_symbols),
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
            // Remove these non_terminals from the parse tree.
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

        // Collect productions from the parse tree.
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
        self.generate_result();
        let table = symbol_table();
        let grammar = table.get_non_terminal_id("Grammar").unwrap();
        let children = std::mem::take(&mut self.node_stack);
        let root_node = ParseTreeNode::non_terminal(grammar, children, Span::new(0, 0, 1, 1));
        GeneratorResult::new(
            std::mem::take(&mut self.symbol_table),
            std::mem::take(&mut self.grammar_rules),
            std::mem::take(&mut self.token_rules),
            root_node,
        )
    }
}
