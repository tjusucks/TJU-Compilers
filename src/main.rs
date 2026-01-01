#![warn(clippy::unwrap_used)]

use std::sync::Arc;

use crate::common::action::DefaultAction;
use crate::common::grammar::Symbol;
use crate::common::grammar_rules::GrammarRules;
use crate::common::parse_table::ParseTable;
use crate::common::symbol_table::Terminal;
use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;
use crate::generator::action::GeneratorAction;
use crate::generator::grammar_rules::{grammar_rules, priority_of, reduce_on};
use crate::generator::processor::Processor;
use crate::generator::token_rules::token_rules;

mod common;
mod compiler;
mod generator;

fn main() {
    // Example EBNF input string.
    let input = r#"
        # EBNF Grammar.
        # Directives.
        @comment    = /#.*/                # Comments range from a '#'-character to the end of the line.
        @whitespace = vertical             # Implicit whitespace, denoted by ~, includes any number of line feeds.
        @literalws  = both                 # Literals have implicit whitespace on the right hand side.
        @ignorecase = False                # Literals and regular expressions are case-sensitive.
        @hide       = atom, factor         # Hide some layers of the grammar for cleaner output.
        @drop       = whitespace, strings  # Drop anonymous whitespace and (anonymous) string literals.

        # Grammar.
        # grammar = { directive | rule }
        grammar = grammar directive
                | grammar rule
                | EPSILON

        # Directive.
        directive = "@" IDENTIFIER "=" value
        value = LITERAL | REGEX | list

        # list = IDENTIFIER { "," IDENTIFIER }
        list = list "," IDENTIFIER | IDENTIFIER

        # EBNF constructs.
        rule = IDENTIFIER "=" expression

        # expression = term { "|" term }
        expression = expression "|" term | term

        # term = factor { factor } | EMPTY
        term = term factor | factor | EMPTY

        # factor = { WHITESPACE } atom { WHITESPACE } [ lookahead ]
        factor = factor_repetition atom factor_repetition lookahead
               | factor_repetition atom factor_repetition
        factor_repetition = factor_repetition WHITESPACE | EPSILON

        # IDENTIFIER ! "=" negative lookahead is handled by the parser.
        atom        = LITERAL
                    | IDENTIFIER
                    | REGEX
                    | group
                    | optional
                    | repetition

        group       = "(" expression ")"
        optional    = "[" expression "]"
        repetition  = "{" expression "}"

        # lookahead = (
        #     POSITIVE_LOOKAHEAD | NEGATIVE_LOOKAHEAD | POSITIVE_LOOKBEHIND | NEGATIVE_LOOKBEHIN
        # ) factor
        lookahead = lookahead_group factor
        lookahead_group = POSITIVE_LOOKAHEAD
                        | NEGATIVE_LOOKAHEAD
                        | POSITIVE_LOOKBEHIND
                        | NEGATIVE_LOOKBEHIND

        # Look ahead / behind.
        POSITIVE_LOOKAHEAD  = "&"
        NEGATIVE_LOOKAHEAD  = "!"
        POSITIVE_LOOKBEHIND = "<-&"
        NEGATIVE_LOOKBEHIND = "<-!"

        # Whitespace.
        WHITESPACE  = "~"

        # Epsilon.
        EMPTY       = "EPSILON"

        # Tokens.
        LITERAL     = /"[^"]*"/~
        REGEX       = /\/(?:[^\/\\]|\\.)*\//~
        IDENTIFIER  = /[A-Za-z_][A-Za-z_0-9]*/~
        "#;

    // Build the lexer and parser.
    let token_rules = token_rules();
    let lexer = Lexer::new(token_rules);
    let grammar_rules = grammar_rules();
    let parse_table = ParseTable::new(grammar_rules, reduce_on, priority_of);
    let mut parser = Parser::new(&parse_table.parse_table, GeneratorAction::default());

    // Tokenize and parse the input.
    let tokens = lexer.tokenize(input);
    let processed = Processor::process(tokens);
    let mut result = parser
        .parse(processed)
        .expect("Failed to parse EBNF grammar");
    println!("{}", result.parse_tree);

    // Build the lexer and parser based on the result.
    let lexer = Lexer::new(&result.token_rules);
    process_rules(&mut result.grammar_rules);

    let parse_table = ParseTable::new(&result.grammar_rules, reduce_on, priority_of);
    let mut parser = Parser::new(
        &parse_table.parse_table,
        DefaultAction::new(result.grammar_rules.start_symbol),
    );

    // Test the generated lexer and parser.
    let tokens = lexer.tokenize(input);
    let processed = Processor::process(tokens);
    let _ = parser
        .parse(processed)
        .expect("Failed to parse input with generated parser");
}

fn process_rules(grammar_rules: &mut GrammarRules) {
    // Handle negative lookahead for identifier.
    let left_identifier = Terminal(Arc::from("LEFT_IDENTIFIER"));
    for rule in &mut grammar_rules.rules {
        if rule.non_terminal.0.as_ref() == "directive" {
            // directive = "@" IDENTIFIER "=" value
            rule.rhs[1] = Symbol::Terminal(left_identifier.clone());
        } else if rule.non_terminal.0.as_ref() == "rule" {
            // rule = IDENTIFIER "=" expression
            rule.rhs[0] = Symbol::Terminal(left_identifier.clone());
        }
    }
}
