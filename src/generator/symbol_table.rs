use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use crate::common::symbol_table::{NonTerminal, SymbolTable, Terminal};

static SYMBOL_TABLE: OnceLock<SymbolTable> = OnceLock::new();

pub fn symbol_table() -> &'static SymbolTable {
    SYMBOL_TABLE.get_or_init(|| {
        let terminal_names = [
            // Symbolic tokens.
            "At",               // '@'
            "Equal",            // '='
            "Pipe",             // '|'
            "LeftBrace",        // '{'
            "RightBrace",       // '}'
            "LeftBracket",      // '['
            "RightBracket",     // ']'
            "LeftParentheses",  // '('
            "RightParentheses", // ')'
            "Comma",            // ','
            "Tilde",            // '~'
            // Lookahead / lookbehind operators.
            "PositiveLookAhead",  // '&'
            "NegativeLookAhead",  // '!'
            "PositiveLookBehind", // '<-&'
            "NegativeLookBehind", // '<-!'
            // Epsilon token.
            "Empty", // epsilon
            // Literal tokens, for strings and regexes.
            "Literal", // string literal
            "Regex",   // regex literal
            // Identifier, for nonterminal names, directive names, etc.
            "Identifier",     // identifier
            "LeftIdentifier", // identifier before an equal sign
            // Tokens to be skipped.
            "Comment",    // comment
            "Whitespace", // whitespace
        ];

        let non_terminal_names = [
            // Grammar.
            "Grammar",
            // Directive.
            "Directive",
            "Value",
            "List",
            // EBNF constructs.
            "Rule",
            "Expression",
            "Term",
            "Factor",
            "FactorRepetition",
            "Atom",
            "Group",
            "Optional",
            "Repetition",
            "Lookahead",
            "LookaheadGroup",
        ];

        let terminals: HashMap<String, Terminal> = terminal_names
            .iter()
            .map(|name| ((*name).to_string(), Terminal(Arc::from(*name))))
            .collect();

        let non_terminals: HashMap<String, NonTerminal> = non_terminal_names
            .iter()
            .map(|name| ((*name).to_string(), NonTerminal(Arc::from(*name))))
            .collect();

        SymbolTable::from_maps(terminals, non_terminals)
    })
}
