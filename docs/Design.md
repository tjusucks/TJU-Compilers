# Design of RustCC: A Parser Generator in Rust

## Overview

RustCC is a parser generator written in Rust that allows developers to define grammars and generate parsers for various programming languages and data formats. It takes EBNF-like grammar definitions and produces efficient Rust code for parsing input according to those grammars.

For demonstration, RustCC includes a test language with semantic actions defined for its grammar rules. This enables the parser to generate intermediate code simultaneously during parsing, illustrating how semantic actions can be injected or overridden to support syntax-directed translation.

## Key Features

RustCC includes the following key features, which is a simplified version of the features found in [DHParser](https://gitlab.lrz.de/badw-it/DHParser)

* EBNF Grammar Support: Allows users to define grammars using an extended Backus-Naur Form (EBNF) syntax.
* Compiler Directives: Supports various directives to customize parsing behavior and simplify grammar definitions.
* Semantic Actions Interface: The generated parser reserves interfaces for semantic actions, enabling users to inject or override code generation logic for specific grammar rules.

Supported compiler directives are subset of [DHParser Directives](https://dhparser.readthedocs.io/en/latest/Reference.html#directives), including:

* `@comment = <regex>`: Regular expression for comments to be ignored.
* `@whitespace = <regex>`: Regular expression for whitespace.
* `@literalws = <side>`: Implicitly assume insignificant whitespace adjacent to string-literals, left, right, both or none.
* `@ignorecase = <bool>`: Global case-sensitivity, True or False.
* `@hide = <token, token, ...>`: List of symbols that shall produce anonymous nodes instead of nodes named by the symbol.
* `@drop = <token, token, ...>`: List of symbols that shall be dropped entirely from the tree.
