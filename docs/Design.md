# Design of RustCC: A **C**ompiler-**C**ompiler in **Rust**

## 01. Overview

RustCC is a compiler-compiler written in Rust that generates LALR(1) parsers and lexers from BNF grammar definitions. Users provide grammar definition and RustCC produces the parsing tables and tokenization logic needed to build a compiler front-end. The parser's semantic actions are customizable via the `Action` trait, allowing users to inject code generation or other processing during parsing, making RustCC flexible enough to support syntax-directed translation and a wide range of compiler tasks.

While RustCC's parser and semantic action system are implemented in Rust, the lexer generator component relies on external libraries: either the Rust [`relex` crate](https://crates.io/crates/relex) or a C++ lexer generator from [cbx6666/Compilers](https://github.com/cbx6666/Compilers). This means RustCC is not fully self-contained, but it leverages mature lexer technology to support efficient tokenization for custom languages.

## 02. Key Features

### Grammar Input

RustCC supports **BNF (Backus-Naur Form)** input similar to [DHParser](https://github.com/jecki/DHParser). While **EBNF (Extended BNF)** grammar syntax input can be parsed, EBNF grammar sugar, such as `{}` for repetition, `[]` for optionality, and `()` for grouping, is **not currently supported** in the generator logic. Users must manually desugar these constructs into standard BNF recursive rules (e.g., replacing `{ A }` with a recursive `list` rule).

### Error Handling

The system provides robust error handling with precise source positioning:

- **Position Tracking**: The lexer/parser uses `LocatedToken` which carries `Span` information (line, column, start/end indices).
- **Error Reporting**: When a syntax error occurs (no valid action in the parse table), the parser calls the `on_error` method of the `Action` trait, allowing users to generate custom error messages including the location of the failure.
- **Result Type**: The `parse` method returns a `Result<A::ParseResult, A::ParseError>`, ensuring errors are propagated safely.

### LALR(1) Conflict Detection

RustCC implements a rigorous LALR(1) table generation algorithm that detects conflicts:

- **Shift/Reduce Conflicts**: Detected when a state allows both shifting a token and reducing a rule.
- **Reduce/Reduce Conflicts**: Detected when a state allows reducing multiple different rules on the same lookahead.
- **Conflict Reporting**: The generator reports these conflicts via the `LR1Conflict` enum, providing details about the state, token, and conflicting rules involved.

### 04. System Testing

### Test Case Design

RustCC's test suite covers a broad range of scenarios to ensure correctness, robustness, and extensibility. The four main types of tests are:

- **Error Handling** (`error_handling.rs`): Ensures invalid grammars or malformed input are rejected with precise error messages and correct position reporting.
- **Self-Reference and Bootstrapping** (`self_reference.rs`): Validates that RustCC can parse and process its own grammar definition, supporting bootstrapping and grammar self-hosting.
- **Simple Grammar Parsing** (`simple_grammar.rs`, `tac_action.rs`): Confirms that the parser and lexer can handle basic, conflict-free arithmetic grammar and generate three-address code (TAC) for arithmetic expressions using a custom Action implementation, testing the SDT interface.
- **C++ Lexer Integration** (`tokenize_cpp.rs`): Tests the ability to tokenize input using a C++-generated lexer via FFI, ensuring cross-language compatibility and correct token stream conversion.

### Test Results

**`error_handling.rs`**: Attempts to parse an invalid directive (`@drop = whitespace | strings` with an unexpected `|`).

Result: The parser returns an error, and the test asserts `result.is_err()`, confirming robust error detection and reporting.

```bash
$ cargo test error_handling -- --nocapture
running 1 test
Parse Error at input:2:34, Unexpected token: Terminal("|")
test error_handing ... ok
```

**`self_reference.rs`**: Parses the RustCC grammar definition using the generated parser and lexer, then re-parses the same grammar to ensure self-hosting.

Result: The parser successfully processes its own grammar, generate a derivation tree, demonstrating bootstrapping capability.

```bash
$ cargo test self_reference -- --nocapture
running 1 test
(grammar
...
  (rule
    (LEFT_IDENTIFIER "IDENTIFIER")
    (= "=")
    (expression
      (term
        (factor
          (factor_repetition
          )
          (atom
            (REGEX "/[A-Za-z_][A-Za-z_0-9]*/")
          )
          (factor_repetition
            (factor_repetition
            )
            (WHITESPACE "~")
          )
        )
      )
    )
  )
)

test self_reference ... ok
```

`simple_grammar.rs`: Parses a basic arithmetic grammar and then parses arithmetic expressions using the generated parser and a C++ lexer adapter.

Note: To run the `simple_grammar` test with C++ lexer integration, replace the contents of `src/cpp/lexer.cpp` with the source code from `assets/lexer_arithmetic.cpp` before compiling the project.

To run this test, compile the project by replacing `src/cpp/lexer.cpp` with source code from `assets/lexer_arithmetic.cpp`.

Result: The parser produces correct Three Address Codes and integrates seamlessly with the C++ lexer.

```bash
$ cargo test simple_grammar -- --nocapture
running 1 test
TAC generation complete. Output written to output.txt
test simple_grammar ... ok
```

```plaintext
// output.txt
t0 = c - 42
t1 = t0 / d
t2 = b * t1
t3 = a + t2
t4 = 3 * x
t5 = y / z
t6 = t4 + t5
t7 = c - t6
```

`tokenize_cpp.rs`: Tokenizes a C-like code snippet using the C++ lexer via FFI and prints the resulting tokens with their kind, value, line, and column.

Note: To run the `tokenize_cpp` test, replace the contents of `src/cpp/lexer.cpp` with the source code from `assets/lexer_bridge.cpp` before compiling the project.

Result: The test demonstrates successful cross-language tokenization and correct token metadata extraction.

```bash
$ cargo test tokenize_cpp -- --nocapture
running 1 test
[1] kind: "INT", value: "int", line: 1, column: 1
[2] kind: "ID", value: "x", line: 1, column: 5
[3] kind: "EQUAL", value: "=", line: 1, column: 7
[4] kind: "NUMBER", value: "123", line: 1, column: 9
[5] kind: "SEMICOLON", value: ";", line: 1, column: 12
[6] kind: "FLOAT", value: "float", line: 2, column: 9
[7] kind: "ID", value: "y", line: 2, column: 15
[8] kind: "EQUAL", value: "=", line: 2, column: 17
[9] kind: "FLOAT_NUMBER", value: "45.67", line: 2, column: 19
[10] kind: "SEMICOLON", value: ";", line: 2, column: 24
[11] kind: "IF", value: "if", line: 3, column: 9
[12] kind: "LPAREN", value: "(", line: 3, column: 12
[13] kind: "ID", value: "x", line: 3, column: 13
[14] kind: "GREATER", value: ">", line: 3, column: 15
[15] kind: "NUMBER", value: "100", line: 3, column: 17
[16] kind: "RPAREN", value: ")", line: 3, column: 20
[17] kind: "LBRACE", value: "{", line: 3, column: 22
[18] kind: "RETURN", value: "return", line: 4, column: 9
[19] kind: "ID", value: "x", line: 4, column: 16
[20] kind: "PLUS", value: "+", line: 4, column: 18
[21] kind: "ID", value: "y", line: 4, column: 20
[22] kind: "SEMICOLON", value: ";", line: 4, column: 21
[23] kind: "RBRACE", value: "}", line: 5, column: 9
[24] kind: "STRING", value: "string", line: 7, column: 9
[25] kind: "ID", value: "name", line: 7, column: 16
[26] kind: "EQUAL", value: "=", line: 7, column: 21
[27] kind: "STRING_LITERAL", value: "\"hello\"", line: 7, column: 23
[28] kind: "SEMICOLON", value: ";", line: 7, column: 30
test tokenize_cpp ... ok
```

These tests collectively ensure that RustCC is robust, extensible, and ready for both research and practical compiler construction tasks.
