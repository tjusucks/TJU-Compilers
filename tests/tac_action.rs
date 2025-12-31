use std::fs::File;
use std::io::{BufWriter, Write};

use relex::Token;
use rustcc::common::action::Action;
use rustcc::common::grammar::Rhs;
use rustcc::common::parse_tree::{ParseError, Span};
use rustcc::common::symbol_table::{NonTerminal, Terminal};

#[derive(Debug, Clone)]
pub enum SemanticValue {
    /// Represents a variable, number, or temporary (e.g., "a", "42", "t0")
    Addr(String),
    /// Represents an operator (e.g., "+", "*")
    Op(String),
    /// Represents a partial tail result: (Operator, Right-Hand-Side Operand)
    /// Used for: expression_tail -> OP term expression_tail
    Partial(String, String),
    /// Represents EPSILON or empty
    Empty,
}

pub struct TacAction {
    semantic_stack: Vec<SemanticValue>,
    temp_counter: usize,
    instructions: Vec<String>,
    output_path: String,
}

impl TacAction {
    pub fn new(output_path: String) -> Self {
        Self {
            semantic_stack: Vec::new(),
            temp_counter: 0,
            instructions: Vec::new(),
            output_path,
        }
    }

    fn new_temp(&mut self) -> String {
        let temp = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    fn emit(&mut self, op: &str, arg1: &str, arg2: &str, result: &str) {
        self.instructions
            .push(format!("{} = {} {} {}", result, arg1, op, arg2));
    }

    fn emit_unary(&mut self, op: &str, arg: &str, result: &str) {
        self.instructions
            .push(format!("{} = {} {}", result, op, arg));
    }

    fn write_file(&self) {
        if let Ok(file) = File::create(&self.output_path) {
            let mut writer = BufWriter::new(file);
            for line in &self.instructions {
                let _ = writeln!(writer, "{}", line);
            }
        } else {
            eprintln!("Failed to write TAC to {}", self.output_path);
        }
    }
}

impl Action for TacAction {
    type ParseResult = (); // We don't build a tree, we generate a file
    type ParseError = ParseError;

    fn on_shift(&mut self, token: Token<Terminal>) {
        // Push relevant tokens to the semantic stack
        let text = token.text.to_string();
        let kind = token.kind.0.as_ref();

        match kind {
            "NUMBER" | "VARIABLE" => self.semantic_stack.push(SemanticValue::Addr(text)),
            "PLUS" | "MINUS" | "MUL" | "DIV" | "POSITIVE" | "NEGATIVE" => {
                self.semantic_stack.push(SemanticValue::Op(text))
            }
            _ => {
                // For parens or other tokens, we might push them or ignore them.
                // Pushing them helps keep stack aligned with grammar symbols.
                self.semantic_stack.push(SemanticValue::Op(text));
            }
        }
    }

    fn on_reduce(&mut self, non_terminal: &NonTerminal, rhs: &Rhs<Terminal, NonTerminal, ()>) {
        let nt_name = non_terminal.0.as_ref();
        let rhs_len = rhs.syms.len();

        // Pop the RHS items from the stack
        let mut args = Vec::with_capacity(rhs_len);
        for _ in 0..rhs_len {
            if let Some(val) = self.semantic_stack.pop() {
                args.push(val);
            }
        }
        args.reverse(); // Now args matches the order of symbols in RHS

        match nt_name {
            "program" => {
                // program = program expression | EPSILON
                // If it's EPSILON, args is empty.
                // If it's program expression, we just keep going.
            }
            "expression" => {
                // expression = term expression_tail
                // args: [Addr(term), Partial(op, rhs) OR Empty]
                if let [SemanticValue::Addr(lhs), tail] = &args[..] {
                    match tail {
                        SemanticValue::Empty => {
                            // No tail, result is just the term
                            self.semantic_stack.push(SemanticValue::Addr(lhs.clone()));
                        }
                        SemanticValue::Partial(op, rhs_val) => {
                            // term OP rhs
                            let result = self.new_temp();
                            self.emit(op, lhs, rhs_val, &result);
                            self.semantic_stack.push(SemanticValue::Addr(result));
                        }
                        _ => panic!("Invalid stack state for expression"),
                    }
                }
            }
            "expression_tail" | "term_tail" => {
                // tail = OP term tail | EPSILON
                if args.is_empty() {
                    // EPSILON
                    self.semantic_stack.push(SemanticValue::Empty);
                } else if let [SemanticValue::Op(op), SemanticValue::Addr(term), tail] = &args[..] {
                    match tail {
                        SemanticValue::Empty => {
                            // End of chain: return (OP, term)
                            self.semantic_stack
                                .push(SemanticValue::Partial(op.clone(), term.clone()));
                        }
                        SemanticValue::Partial(next_op, next_val) => {
                            // Chained: OP term (next_op next_val)
                            // Because of tail recursion, we compute the inner part first (Right Associative)
                            let result = self.new_temp();
                            self.emit(next_op, term, next_val, &result);
                            self.semantic_stack
                                .push(SemanticValue::Partial(op.clone(), result));
                        }
                        _ => panic!("Invalid stack state for tail"),
                    }
                }
            }
            "term" => {
                // term = factor term_tail
                // Logic is identical to expression
                if let [SemanticValue::Addr(lhs), tail] = &args[..] {
                    match tail {
                        SemanticValue::Empty => {
                            self.semantic_stack.push(SemanticValue::Addr(lhs.clone()));
                        }
                        SemanticValue::Partial(op, rhs_val) => {
                            let result = self.new_temp();
                            self.emit(op, lhs, rhs_val, &result);
                            self.semantic_stack.push(SemanticValue::Addr(result));
                        }
                        _ => panic!("Invalid stack state for term"),
                    }
                }
            }
            "factor" => {
                // factor = sign factor_base | factor_base
                if args.len() == 1 {
                    // factor_base
                    // Just pass it up
                    self.semantic_stack.push(args[0].clone());
                } else if let [SemanticValue::Op(sign), SemanticValue::Addr(base)] = &args[..] {
                    // sign factor_base
                    if sign == "-" {
                        let result = self.new_temp();
                        self.emit_unary("-", base, &result);
                        self.semantic_stack.push(SemanticValue::Addr(result));
                    } else {
                        self.semantic_stack.push(SemanticValue::Addr(base.clone()));
                    }
                }
            }
            "factor_base" => {
                // NUMBER | VARIABLE | group
                if args.len() == 1 {
                    // NUMBER or VARIABLE
                    self.semantic_stack.push(args[0].clone());
                } else {
                    // group -> "(" expression ")"
                    // args: [Op("("), Addr(expr), Op(")")]
                    if let [_, val, _] = &args[..] {
                        self.semantic_stack.push(val.clone());
                    }
                }
            }
            "sign" => {
                // POSITIVE | NEGATIVE
                // Already pushed as Op in on_shift, just pass it up
                self.semantic_stack.push(args[0].clone());
            }
            "group" => {
                // group = "(" expression ")"
                // Handled in factor_base usually, but if grammar has explicit group rule:
                if let [_, val, _] = &args[..] {
                    self.semantic_stack.push(val.clone());
                }
            }
            _ => {}
        }
    }

    fn on_accept(&mut self) -> Self::ParseResult {
        self.write_file();
        println!(
            "TAC generation complete. Output written to {}",
            self.output_path
        );
    }

    fn on_error(&mut self, token: Token<Terminal>, span: Span) -> Self::ParseError {
        ParseError {
            message: format!(
                "Syntax Error at {:?}: Unexpected token {:?}",
                span, token.text
            ),
            span: Some(span),
        }
    }
}
