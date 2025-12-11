use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser_config::ParserConfig;
use crate::processor::Processor;
use crate::semantic_action::DefaultAction;

mod lexer;
mod parse_tree;
mod parser;
mod parser_config;
mod processor;
mod semantic_action;
mod symbol;

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
        grammar     = { directive | rule }

        # Directive.
        directive   = "@" IDENTIFIER "=" value
        value       = LITERAL | REGEX | list
        list        = IDENTIFIER { "," IDENTIFIER }

        # EBNF constructs.
        rule        = IDENTIFIER "=" expression
        expression  = term { "|" term }
        term        = factor { factor }
        factor      = { WHITESPACE } atom { WHITESPACE } [ lookahead ]
        atom        = LITERAL
                    | IDENTIFIER ! "="
                    | REGEX
                    | group
                    | optional
                    | repetition
        group       = "(" expression ")"
        optional    = "[" expression "]"
        repetition  = "{" expression "}"
        lookahead   = ( POSITIVE_LOOKAHEAD
                    | NEGATIVE_LOOKAHEAD
                    | POSITIVE_LOOKBEHIND
                    | NEGATIVE_LOOKBEHIND
                    ) factor

        # Look ahead / behind.
        POSITIVE_LOOKAHEAD  = "&"
        NEGATIVE_LOOKAHEAD  = "!"
        POSITIVE_LOOKBEHIND = "<-&"
        NEGATIVE_LOOKBEHIND = "<-!"

        # Whitespace.
        WHITESPACE  = "~"

        # Tokens.
        LITERAL     = /"[^"]*"/~
        REGEX       = /\/(?:[^\/\\]|\\.)*\//~
        IDENTIFIER  = /[A-Za-z_][A-Za-z_0-9]*/~
        "#;

    // Tokenize the input string.
    let tokens = Lexer::tokenize(input);

    // Process the tokens for lookahead / behind rules.
    let processed = Processor::process(tokens);

    // Parse the processed tokens.
    let mut parser = Parser::new(
        ParserConfig::grammar(),
        ParserConfig::reduce_on,
        ParserConfig::priority_of,
        DefaultAction,
    );
    let tree = parser.parse(processed).unwrap();
    println!("{tree}");
}
