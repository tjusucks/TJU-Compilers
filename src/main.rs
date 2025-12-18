use crate::common::parse_table::ParseTable;
use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;
use crate::generator::generator_action::GeneratorAction;
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
        grammar     = directive | rule

        # Directive.
        directive   = "@" IDENTIFIER "=" value
        value       = LITERAL | REGEX | list
        list        = IDENTIFIER "," IDENTIFIER

        # EBNF constructs.
        rule        = IDENTIFIER "=" expression
        expression  = term "|" term
        term        = factor factor | EMPTY
        factor      = WHITESPACE atom WHITESPACE lookahead
        atom        = LITERAL
                    | IDENTIFIER
                    | REGEX
                    | group
                    | optional
                    | repetition
        group       = "(" expression ")"
        optional    = "[" expression "]"
        repetition  = "{" expression "}"
        lookahead   = POSITIVE_LOOKAHEAD
                    | NEGATIVE_LOOKAHEAD
                    | POSITIVE_LOOKBEHIND
                    | NEGATIVE_LOOKBEHIND
                    factor

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

    let token_rules = token_rules();
    let lexer = Lexer::new(token_rules);
    let grammar_rules = grammar_rules();
    let parse_table = ParseTable::new(grammar_rules, reduce_on, priority_of);

    let mut parser = Parser::new(parse_table.parse_table, GeneratorAction::new());

    let tokens = lexer.tokenize(input);
    let processed = Processor::process(tokens);
    let result = parser.parse(processed).unwrap();

    println!("{}", result.parse_tree);

    println!();
    println!("{:?}", result.symbol_table);
    println!("{:?}", result.grammar_rules);
    println!("{:?}", result.token_rules);
}
