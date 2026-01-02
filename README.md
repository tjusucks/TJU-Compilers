# RustCC: A **C**ompiler-**C**ompiler in **Rust**

RustCC is a compiler-compiler written in Rust. It generates LALR(1) parsers and lexers from BNF grammar definitions, making it easy to build custom language front-ends and experiment with compiler technology.

## Features

- LALR(1) parsing table generation with conflict detection
- Customizable semantic actions in Rust
- Integration with Rust and C++ lexer generators
- Modular and extensible design

## Building

RustCC requires the Rust nightly toolchain. First, install [`rustup`](https://www.rust-lang.org/tools/install) and then add the nightly toolchain:

```sh
rustup toolchain install nightly
rustup override set nightly
```

To build RustCC, run:

```sh
cargo build --release
```

## Testing

To run the test suite, use:

```bash
cargo test
```

Some test cases require additional setup. Please refer design documentation at [docs/Design.md](docs/Design.md) for detailed instructions.

## Example Usage

Define the grammar and test input in `src/main.rs`:

```rust
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
    // Simple arithmetic grammar.
    let input = r#"
        # Arithmetic Grammar.
        program         = program expression | EPSILON
        expression      = term expression_tail
        expression_tail = PLUS term expression_tail
                        | MINUS term expression_tail
                        | EPSILON
        term            = factor term_tail
        term_tail       = MUL factor term_tail
                        | DIV factor term_tail
                        | EPSILON
        factor          = NUMBER
                        | VARIABLE
                        | group

        group = "(" expression ")"

        PLUS     = "+"
        MINUS    = "-"
        MUL      = "*"
        DIV      = "/"

        NUMBER   = /[0-9]+/
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
    let test_input = r#"
        a + b * (c - 42) / d
        c - 3 * x + (y / z)
    "#;
    let tokens = lexer.tokenize(test_input);
    let result = parser.parse(tokens);
    match result {
        Ok(parse_tree) => {
            println!("Parse succeeded unexpectedly:\n{}", parse_tree);
        }
        Err(parse_error) => {
            println!("{parse_error}");
        }
    }
}
```

Run the example with the following command to view the output derivation tree:

```bash
cargo run
```

## System Design

Detailed documentation of the system design can be found at [docs/Design.md](docs/Design.md).
