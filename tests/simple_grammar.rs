mod tac_action;

use rustcc::common::parse_table::ParseTable;
use rustcc::compiler::lexer::Lexer;
use rustcc::compiler::parser::Parser;
use rustcc::cpp::adapter::{CppLexerAdapter, LexerAdapter};
use rustcc::generator::action::GeneratorAction;
use rustcc::generator::grammar_rules::{grammar_rules, priority_of, reduce_on};
use rustcc::generator::processor::Processor;
use rustcc::generator::token_rules::token_rules;

use crate::tac_action::TacAction;

#[test]
fn simple_grammar() {
    // Example EBNF input string.
    let input = r#"
        # Arithmetic Grammar (BNF, conflict-free)
        @comment     = /#.*/
        @whitespace  = vertical
        @literalws   = both
        @ignorecase  = False
        @drop        = whitespace, strings

        program = program expression | EPSILON

        expression = term expression_tail

        expression_tail = PLUS term expression_tail
                        | MINUS term expression_tail
                        | EPSILON

        term = factor term_tail

        term_tail = MUL factor term_tail
                  | DIV factor term_tail
                  | EPSILON

        factor = sign factor_base
               | factor_base

        factor_base = NUMBER
                    | VARIABLE
                    | group

        sign = POSITIVE
             | NEGATIVE

        group = "(" expression ")"

        PLUS     = "+"
        MINUS    = "-"
        MUL      = "*"
        DIV      = "/"

        POSITIVE = /[+]/
        NEGATIVE = /[-]/

        NUMBER   = /(?:0|(?:[1-9]\d*))(?:\.\d+)?/
        VARIABLE = /[A-Za-z]/
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
    let result = parser.parse(processed).unwrap();

    // Build the lexer and parser based on the result.
    let parse_table = ParseTable::new(&result.grammar_rules, reduce_on, priority_of);
    let mut parser = Parser::new(
        &parse_table.parse_table,
        TacAction::new("output.txt".to_string()),
    );

    // Test the generated lexer and parser.
    let test_input = r#"
        a + b * (c - 42) / d
        c - 3 * x + (y / z)
    "#;

    let adapter = CppLexerAdapter;
    let tokens = adapter.tokenize(test_input);
    parser.parse(tokens.into_iter()).unwrap();
}
