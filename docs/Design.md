# Design of RustCC: A **C**ompiler-**C**ompiler in **Rust**

## 01. Overview

RustCC is a compiler-compiler written in Rust that generates LALR(1) parsers and lexers from BNF grammar definitions. Users provide grammar definition and RustCC produces the parsing tables and tokenization logic needed to build a compiler front-end. The parser's semantic actions are customizable via the `Action` trait, allowing users to inject code generation or other processing during parsing, making RustCC flexible enough to support syntax-directed translation and a wide range of compiler tasks.

While RustCC's parser and semantic action system are implemented in Rust, the lexer generator component relies on external libraries: either the Rust [`relex` crate](https://crates.io/crates/relex) or a C++ lexer generator from [cbx6666/Compilers](https://github.com/cbx6666/Compilers). This means RustCC is not fully self-contained, but it leverages mature lexer technology to support efficient tokenization for custom languages.

## 02. Key Features

### Grammar Input

RustCC supports **BNF (Backus-Naur Form)** input similar to [DHParser](https://github.com/jecki/DHParser). While **EBNF (Extended BNF)** grammar syntax input can be parsed, EBNF grammar sugar, such as `{}` for repetition, `[]` for optionality, and `()` for grouping, is **not currently supported** in the generator logic. Users must manually desugar these constructs into standard BNF recursive rules (e.g., replacing `{ A }` with a recursive `list` rule).

### In-Memory Generation

The RustCC generator produces **in-memory lexer and parser objects**. After processing a grammar definition, users receive ready-to-use Rust objects for both lexical analysis and parsing, without the need for code generation or external build steps.

### Default Action for Derivation Trees

RustCC provides a default semantic action (`rustcc::common::action::DefaultAction`) implementation. When used, this action automatically constructs and returns a full derivation tree for the input, making it easy to inspect grammar structure, debug grammars, or bootstrap further compiler development.

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

## 03. Project Structure

```plaintext
.
├── assets
│   ├── lexer_arithmetic.cpp          # C++ lexer for arithmetic expressions
│   ├── lexer_arithmetic.txt          # Expected output for arithmetic lexer
│   └── lexer_bridge.cpp              # C++ lexer bridge implementation
├── build.rs                          # Build script for C++ FFI integration
├── Cargo.toml                        # Project dependencies and configuration
├── docs
│   ├── Design.md                     # This design document
│   └── Design.pdf                    # Generated documentation
├── grammars
│   ├── arithmetic.py                 # Arithmetic grammar definitions
│   ├── ebnf.py                       # EBNF grammar parsing utilities
│   └── requirements.txt              # Python dependencies for grammar tools
├── LICENSE
├── prompt.md
├── README.md
├── rustfmt.toml                      # Rust formatting configuration
├── src
│   ├── common                        # Core data structures and utilities
│   │   ├── action.rs                 # Semantic action trait definition
│   │   ├── grammar.rs                # Grammar data structures and operations (LR(0), LR(1), LALR(1))
│   │   ├── grammar_rules.rs          # Grammar rule definitions and processing
│   │   ├── mod.rs                    # Module exports for common components
│   │   ├── parse_table.rs            # LALR(1) parse table structure and utilities
│   │   ├── parse_tree.rs             # Parse tree representation and utilities
│   │   ├── symbol_table.rs           # Terminal and NonTerminal symbol definitions
│   │   └── token_rules.rs            # Token rule definitions and processing
│   ├── compiler                      # Runtime compiler components
│   │   ├── lexer.rs                  # Rust lexer implementation using relex crate
│   │   ├── mod.rs                    # Module exports for compiler components
│   │   └── parser.rs                 # LALR(1) parser implementation
│   ├── cpp                           # C++ FFI integration
│   │   ├── adapter.rs                # Rust-C++ adapter for lexer integration
│   │   ├── bridge.cpp                # C++ bridge implementation for FFI
│   │   ├── bridge.h                  # C++ header for FFI bridge
│   │   ├── lexer.cpp                 # C++ lexer implementation
│   │   ├── lexer.rs                  # Rust bridge module for C++ FFI
│   │   └── mod.rs                    # Module exports for C++ integration
│   ├── generator                     # Grammar processing and code generation
│   │   ├── action.rs                 # Generator action implementations
│   │   ├── grammar_rules.rs          # Grammar rule generation utilities
│   │   ├── mod.rs                    # Module exports for generator components
│   │   ├── parse_tree.rs             # Parse tree generation utilities
│   │   ├── processor.rs              # Token stream processor for grammar rules
│   │   └── token_rules.rs            # Token rule generation utilities
│   ├── lib.rs                        # Main library exports
│   └── main.rs                       # CLI entry point
└── tests                             # Test suite
    ├── error_handling.rs             # Error handling and reporting tests
    ├── self_reference.rs             # Self-hosting and bootstrapping tests
    ├── simple_grammar.rs             # Basic grammar and TAC generation tests
    ├── tac_action.rs                 # Three-address code generation tests
    └── tokenize_cpp.rs               # C++ FFI lexer integration tests
```

## 04. Overall Pipeline

RustCC follows a multi-stage compilation pipeline:

### Grammar Input Processing

- **Input**: BNF grammar definition.
- **Process**: The grammar is tokenized, parsed and converted into internal representation (`rustcc::generator::action::GeneratorResult`).
- **Output**: Raw grammar rules with metadata.

### Grammar Rule Processing

- **Input**: Raw grammar rules from parser.
- **Process**: Grammar rules are processed by the `Processor` to handle special cases like left/right identifier recognition.
- **Output**: Processed grammar rules suitable for LALR(1) table generation.

### LALR(1) Table Generation

- **Input**: Processed grammar rules.
- **Process**:
  - LR(0) items are generated and closure operations are performed.
  - FIRST and FOLLOW sets are computed.
  - LALR(1) parse table is constructed with shift/reduce/reduce actions.
  - Conflict detection and reporting.
- **Output**: LALR(1) parse table with error handling capabilities.

### Lexer Generation

- **Option 1**: Rust lexer using `relex` crate from token rules.
- **Option 2**: C++ lexer generated from grammar and integrated via FFI.
- **Output**: Token stream generator with position tracking.

### Parsing Execution

- **Input**: Source code text and generated parse table.
- **Process**: LALR(1) parser processes token stream using action/goto table.
- **Output**: Result from semantic actions (parse tree, AST, TAC, etc.).

## 05. System Design Details

### Common Module

The `common` module contains the foundational data structures used throughout the system:

- **action.rs**: Defines the `Action` trait that allows users to customize semantic actions during parsing. The trait includes methods for handling shifts, reductions, acceptance, and error cases.
- **grammar.rs**: Implements core grammar representation and operations including:
  - `Symbol<T, N>`: Represents terminals and nonterminals.
  - `Rhs<T, N, A>`: Right-hand side of grammar rules with associated actions.
  - `Item<'a, T, N, A>`: LR(0) items for state machine construction.
  - `LR0StateMachine`: LR(0) state machine and closure operations.
  - `Grammar<T, N, A>`: Main grammar operations including `lr0_state_machine()`, `first_sets()`, `follow_sets()`, and `lalr1()` methods.
  - `LR1Conflict`: Enum for reporting shift/reduce and reduce/reduce conflicts.
- **grammar_rules.rs**: Handles parsing and representation of grammar rules including directives and metadata.
- **parse_table.rs**: Defines `LR1ParseTable` structure containing states with shift/reduce/accept actions and goto transitions.
- **parse_tree.rs**: Provides parse tree representations and traversal utilities.
- **symbol_table.rs**: Defines `Terminal` and `NonTerminal` enums for grammar symbols.
- **token_rules.rs**: Manages regular expressions and patterns for tokenization.

### Compiler Module

The `compiler` module contains runtime components:

- **lexer.rs**: Implements tokenization using the `relex` crate:
  - `LocatedToken`: Token with attached source location information (line, column, start/end offsets).
  - `Lexer`: Main lexer class that maps input text to token stream with position tracking.
  - `compute_line_col()`: Line and column calculation from character offset.
- **parser.rs**: Implements the LALR(1) parsing algorithm:
  - `Parser`: Main parser class with state stack and semantic action integration.
  - `parse()`: Core parsing loop that processes token stream using parse table.
  - Handles shift, reduce, accept, and error actions with proper stack management.

### Generator Module

The `generator` module handles grammar processing and code generation:

- **processor.rs**: Implements token stream processing to handle special grammar cases (e.g., distinguishing identifiers that appear on the left side of assignments)
- **action.rs**: Generator-specific semantic action implementations.
- **grammar_rules.rs**: Grammar rule generation utilities.
- **parse_tree.rs**: Parse tree generation utilities.
- **token_rules.rs**: Token rule generation utilities.

### C++ Integration Module

The `cpp` module provides C++ FFI integration:

- **lexer.rs**: CXX bridge definitions for C++ lexer integration.
- **bridge.h/cpp**: C++ interface for tokenization.
- **lexer.cpp**: C++ lexer implementation.
- **adapter.rs**: Rust adapter for C++ FFI integration.

### Parser Algorithm Design

The LALR(1) parser implementation follows these key principles:

1. **State Management**: Uses stack-based state management with separate stacks for parse states and semantic values.
2. **Action Table Lookup**: Performs O(1) lookups in the parse table for shift/reduce decisions.
3. **Error Recovery**: Provides hooks for custom error handling through the `Action` trait.
4. **Semantic Actions**: Integration points for user-defined actions during shift, reduce, accept, and error cases.

### Error Handling Design

Robust error handling includes:

- Precise location tracking in `LocatedToken` and `Span` structures
- Customizable error reporting through the `Action::on_error` method
- Result-based error propagation using `Result<T, E>` types
- Panic-free operation with proper error states in the parse table

### Memory Management

The system employs Rust's ownership model for memory safety:

- Zero-copy parsing with lifetime management
- Stack-based allocation for parser state
- Proper cleanup of temporary objects during parsing
- Integration with C++ memory management via CXX crate

## 06. System Implementation

### Context-Free Grammar Definitions

RustCC uses standard context-free grammar definitions represented by the following core data structures:

```rust
/// A context-free grammar.
#[derive(Debug)]
pub struct Grammar<T, N, A> {
    /// The rules for each nonterminal.
    pub rules: BTreeMap<N, Vec<Rhs<T, N, A>>>,
    /// The starting state.
    pub start: N,
}
```

Key components include:

- **`T`**: Terminal symbol type (e.g., `Terminal` enum)
- **`N`**: Nonterminal symbol type (e.g., `NonTerminal` enum)
- **`rules`**: Mapping from nonterminals to their production rules.
- **`Rhs<T, N, A>`**: Represents the right-hand side of a production rule, including symbols and associated semantic actions.
- **`start`**: The designated start symbol of the grammar.

### Parse Table Generation

The LALR(1) parse table generation is the core of RustCC's compiler-compiler functionality. The process begins with the `Grammar::lalr1()` method which orchestrates the entire table construction:

```rust
pub fn lalr1<ReduceFn, PriorityFn>(
    &self,
    mut reduce_on: ReduceFn,
    mut priority_of: PriorityFn,
) -> Result<LR1ParseTable<'_, T, N, A>, LR1Conflict<'_, T, N, A>>
where
    ReduceFn: FnMut(&Rhs<T, N, A>, Option<&T>) -> bool,
    PriorityFn: FnMut(&Rhs<T, N, A>, Option<&T>) -> i32,
{
    let state_machine = self.lr0_state_machine();
    let extended = state_machine.extended_grammar();
    let first_sets = extended.first_sets();
    let follow_sets = extended.follow_sets(&first_sets);

    // Initialize the parse table.
    let mut r = LR1ParseTable { ... }

    // Add shifts.
    // ...

    // Add reductions.
    // ...
}
```

The key steps in the parse table generation include:

1. **LR(0) State Machine Construction**: The `lr0_state_machine` method creates the foundational LR(0) items and closure operations. This involves:
   - Creating the initial item set with the augmented start rule
   - Computing the closure of each state (adding all possible productions)
   - Creating state transitions based on grammar symbols

2. **Extended Grammar Generation**: The `extended_grammar` method creates an LALR(1) extended grammar as described in standard compiler construction algorithms. This creates an extended grammar where each nonterminal includes its source state, enabling proper LALR(1) lookahead computation.

3. **FIRST and FOLLOW Set Computation**: These sets are computed using fixed-point iteration algorithms:
   - FIRST sets: For each nonterminal, determine which terminals can appear first in derivations
   - FOLLOW sets: For each nonterminal, determine which terminals can appear immediately after it in sentential forms

4. **Parse Table Construction**: The algorithm populates the action and goto tables:
   - **Shift actions**: When the parser should shift a token and move to a new state
   - **Reduce actions**: When the parser should reduce by applying a grammar rule
   - **Accept action**: When the parser recognizes the complete input
   - **Goto transitions**: State transitions for nonterminals after reductions

5. **Conflict Resolution**: The system detects and reports:
   - **Shift/Reduce conflicts**: When a state allows both shifting and reducing
   - **Reduce/Reduce conflicts**: When multiple reduction rules are applicable

### Semantic Action for IR Generation

The generator uses semantic actions to create an intermediate representation (IR) of the input grammar in a single pass. The key component is the `GeneratorResult` struct in the generator module:

```rust
pub struct GeneratorResult<'a> {
    pub grammar: Grammar<Terminal, NonTerminal, ()>,
    pub start: NonTerminal,
    pub token_rules: Vec<TokenRule>,
    pub parse_table: LR1ParseTable<'a, Terminal, NonTerminal, ()>,
}
```

The generation process uses the `Processor` iterator to handle special cases such as distinguishing between left-hand side and right-hand side identifiers. This is crucial for proper semantic action handling during parsing.

The semantic action system allows for one-pass IR generation by:

1. **Token Processing**: The `Processor` handles token stream transformations, specifically identifying when an identifier appears on the left-hand side of an assignment versus the right-hand side.
2. **Grammar Rule Construction**: Each grammar rule is processed with associated semantic actions that build the IR representation.
3. **Symbol Table Management**: Terminal and non-terminal symbols are properly categorized and tracked for the parsing phase.

### Left and Right Identifier Handling

The `Processor` implementation in `src/generator/processor.rs` plays a critical role in resolving LALR(1) conflicts in BNF grammar definitions by distinguishing between left-hand side identifiers (LEFT_IDENTIFIER) and right-hand side identifiers (IDENTIFIER). This distinction is essential for resolving ambiguities between identifiers that appear in assignment contexts versus expression contexts.

```rust
impl<'a, I: Iterator<Item = LocatedToken<'a>>> Iterator for Processor<'a, I> {
    type Item = LocatedToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.iterator.next();
        if let Some(mut previous_token) = self.previous_token.take() {
            if let Some(ref current_token) = token {
                if current_token.token.kind.0.as_ref() == "Equal"
                    && previous_token.token.kind.0.as_ref() == "Identifier"
                {
                    previous_token.token.kind = Terminal("LeftIdentifier".into());
                } else if current_token.token.kind.0.as_ref() == "="
                    && previous_token.token.kind.0.as_ref() == "IDENTIFIER"
                {
                    previous_token.token.kind = Terminal("LEFT_IDENTIFIER".into());
                }
            }
            self.previous_token = token;
            Some(previous_token)
        } else {
            None
        }
    }
}
```

This implementation:

1. **Looks ahead**: It examines the current token to determine the meaning of the previous token
2. **Context-sensitive transformation**: Identifiers followed by `=` or `Equal` are transformed to `LEFT_IDENTIFIER` tokens
3. **LALR conflict resolution**: By distinguishing left-hand side identifiers from right-hand side ones, the parser can resolve ambiguities in grammar rules that would otherwise cause shift/reduce conflicts

This approach allows the grammar to properly handle assignment statements like `identifier = expression` while still allowing the same identifier to appear in expression contexts. The processor essentially creates a "context-sensitive" transformation that helps the LALR(1) parser make correct parsing decisions without requiring more powerful parsing algorithms.

## 07. System Testing

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

## 08. AI Assistant Usage

> Humans and AI systems working as a team can do more than either on their own. AI systems should initially aim at removing the drudgery of current tasks. -- _David Patterson_

AI tools were utilized throughout the development process to accelerate implementation and ensure quality:

- **Code Generation**: AI assisted in writing boilerplate code, such as the `Action` trait and `ParseTable::new()` signatures.
- **Mutable-iteration issues**: AI suggested practical, safe patterns to implement maps manipulation while iterating (`rustcc::common::grammar::Grammar::first_sets()`) avoiding common pitfalls with mutable iteration in Rust.
- **Macro design**: AI helped design the macro pattern used to reduce repetitive trait impls for comparator/ordering logic (`comparators!`).
- **Debugging**: AI helped identify lifetime issues with `LocatedToken` and `ParseTable` references, suggesting the use of `Box::leak` for static lifetime promotion in generated code and `OnceLock` for singleton patterns.
- **Documentation**: AI synthesized the system design and implementation details into this documentation, ensuring consistency between the code and the design description.
