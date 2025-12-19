use rustcc::common::parse_table::ParseTable;
use rustcc::compiler::lexer::Lexer;
use rustcc::compiler::parser::Parser;
use rustcc::generator::generator_action::GeneratorAction;
use rustcc::generator::grammar_rules::{grammar_rules, priority_of, reduce_on};
use rustcc::generator::processor::Processor;
use rustcc::generator::symbol_table::symbol_table;
use rustcc::generator::token_rules::token_rules;

#[test]
fn test_generated_lexer_tokenization() {
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
    let mut parser = Parser::new(parse_table.parse_table, GeneratorAction::default());

    let tokens = lexer.tokenize(input);
    let processed = Processor::process(tokens);
    let result = parser.parse(processed).unwrap();

    println!("{}", result.parse_tree);

    println!();
    println!("{:?}", result.symbol_table);

    // println!("{:?}", result.grammar_rules);
    println!("Generated grammar rules");
    println!(
        "Start symbol: {}",
        result
            .symbol_table
            .get_non_terminal_name(result.grammar_rules.start_symbol)
            .unwrap()
    );

    for rule in &result.grammar_rules.rules {
        let lhs_name = result
            .symbol_table
            .get_non_terminal_name(rule.non_terminal)
            .unwrap_or("UNKNOWN_NONTERMINAL");

        let rhs_names: Vec<String> = rule
            .rhs
            .iter()
            .map(|sym| match sym {
                lalr::Symbol::Terminal(t) => result
                    .symbol_table
                    .get_terminal_name(*t)
                    .unwrap_or("UNKNOWN_TERMINAL")
                    .to_string(),
                lalr::Symbol::Nonterminal(nt) => result
                    .symbol_table
                    .get_non_terminal_name(*nt)
                    .unwrap_or("UNKNOWN_NONTERMINAL")
                    .to_string(),
            })
            .collect();

        println!("Rule: LHS: {}, RHS: {:?}", lhs_name, rhs_names);
    }

    println!("Generated token rules");
    for rule in &result.token_rules {
        println!(
            "Rule: Kind: {:?}, Regex: {}, Skip: {}",
            result.symbol_table.get_terminal_name(rule.kind),
            rule.regex,
            rule.skip
        );
    }
}
