use lalr::{Rhs, Symbol};
use rustcc::common::action::DefaultAction;
use rustcc::common::parse_table::ParseTable;
use rustcc::common::symbol_table::{NonTerminal, Terminal};
use rustcc::compiler::lexer::Lexer;
use rustcc::compiler::parser::Parser;
use rustcc::generator::action::GeneratorAction;
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
    let mut parser = Parser::new(parse_table.parse_table, GeneratorAction::default());

    println!(
        "Start symbol: {}",
        grammar_rules
            .start_symbol
            .to_string(symbol_table())
            .unwrap()
    );
    for rule in &grammar_rules.rules {
        let lhs = rule.non_terminal.to_string(symbol_table()).unwrap();
        let rhs = rule
            .rhs
            .iter()
            .map(|sym| match sym {
                Symbol::Terminal(t) => t.to_string(symbol_table()).unwrap(),
                Symbol::Nonterminal(nt) => nt.to_string(symbol_table()).unwrap(),
            })
            .collect::<Vec<_>>()
            .join(" ");
        println!("{} -> {}", lhs, rhs);
    }

    // Tokenize and parse the input.
    let tokens = lexer.tokenize(input);
    let processed = Processor::process(tokens);
    let result = parser.parse(processed).unwrap();
    // println!("{}", result.parse_tree.to_string(symbol_table()));
    //
    println!("{:?}", result.symbol_table);

    // Build the lexer and parser based on the result.
    let lexer = Lexer::new(&result.token_rules);

    println!(
        "Start symbol: {}",
        result
            .grammar_rules
            .start_symbol
            .to_string(&result.symbol_table)
            .unwrap()
    );
    for rule in &result.grammar_rules.rules {
        let lhs = rule.non_terminal.to_string(&result.symbol_table).unwrap();
        let rhs = rule
            .rhs
            .iter()
            .map(|sym| match sym {
                Symbol::Terminal(t) => t.to_string(&result.symbol_table).unwrap(),
                Symbol::Nonterminal(nt) => nt.to_string(&result.symbol_table).unwrap(),
            })
            .collect::<Vec<_>>()
            .join(" ");
        println!("{} -> {}", lhs, rhs);
    }

    let reduce_on_test = |rhs: &Rhs<Terminal, NonTerminal, ()>, lookahead: Option<&Terminal>| {
        // let table = &result.symbol_table;
        // let factor_repetition = table.get_non_terminal_id("factor_repetition").unwrap();
        // let atom = table.get_non_terminal_id("atom").unwrap();
        // let tilde = table.get_terminal_id("~").unwrap();
        // let whitespace = table.get_terminal_id("WHITESPACE").unwrap();

        // match (&rhs.syms[..], lookahead) {
        //     // Greedy whitespace consumption.
        //     (
        //         [
        //             Symbol::Nonterminal(nt1),
        //             Symbol::Nonterminal(nt2),
        //             Symbol::Nonterminal(nt3),
        //         ],
        //         Some(terminal),
        //     ) if *nt1 == factor_repetition
        //         && *nt2 == atom
        //         && *nt3 == factor_repetition
        //         && (*terminal == tilde || *terminal == whitespace) =>
        //     {
        //         false
        //     }
        //     _ => true,
        // }
        true
    };

    let parse_table = ParseTable::new(&result.grammar_rules, reduce_on_test, priority_of);
    for state in &parse_table.parse_table.states {
        println!("{:?}", state);
    }
    let mut parser = Parser::new(
        parse_table.parse_table,
        DefaultAction::new(result.grammar_rules.start_symbol),
    );
    // Test the generated lexer and parser.
    let tokens = lexer.tokenize(input);
    let processed = Processor::process(tokens);
    let tree = parser.parse(processed).unwrap();
    println!("{}", tree.to_string(&result.symbol_table));
}
