use rustcc::common::action::DefaultAction;
use rustcc::common::parse_table::ParseTable;
use rustcc::compiler::lexer::Lexer;
use rustcc::compiler::parser::Parser;
use rustcc::generator::action::GeneratorAction;
use rustcc::generator::grammar_rules::{grammar_rules, priority_of, reduce_on};
use rustcc::generator::processor::Processor;
use rustcc::generator::token_rules::token_rules;

#[test]
fn conflict_detection() {
    // Example input string.
    let input = r#"
        # Dangling else grammar (causes shift/reduce conflict)
        S = if_stmt
        if_stmt = IF expr THEN if_stmt
                | IF expr THEN if_stmt ELSE if_stmt
                | OTHER

        IF    = "if"
        THEN  = "then"
        ELSE  = "else"
        OTHER = /[a-z]+/
        expr  = /[a-z]+/
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
    let result = parser
        .parse(processed)
        .expect("Failed to parse BNF grammar");

    // Build the lexer and parser based on the result.
    let lexer = Lexer::new(&result.token_rules);
    let parse_table = ParseTable::new(&result.grammar_rules, reduce_on, priority_of);
    let mut parser = Parser::new(
        &parse_table.parse_table,
        DefaultAction::new(result.grammar_rules.start_symbol),
    );

    // Test the generated lexer and parser.
    let input = r#"
        a
    "#;
    let tokens = lexer.tokenize(input);
    let processed = Processor::process(tokens);
    let _ = parser
        .parse(processed)
        .expect("Failed to parse input with generated parser");
}
