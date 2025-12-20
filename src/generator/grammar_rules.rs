use std::sync::OnceLock;

use lalr::{Rhs, Symbol};

use crate::common::grammar_rules::{GrammarRules, Rule};
use crate::common::symbol_table::{NonTerminal, Terminal};
use crate::generator::symbol_table::symbol_table;

#[allow(clippy::vec_init_then_push)]
pub fn grammar_rules() -> &'static GrammarRules {
    static GRAMMAR_RULES: OnceLock<GrammarRules> = OnceLock::new();
    GRAMMAR_RULES.get_or_init(|| {
        let table = symbol_table();

        // Terminal symbols.
        let at = table.get_terminal_id("At").unwrap();
        let equal = table.get_terminal_id("Equal").unwrap();
        let pipe = table.get_terminal_id("Pipe").unwrap();
        let left_brace = table.get_terminal_id("LeftBrace").unwrap();
        let right_brace = table.get_terminal_id("RightBrace").unwrap();
        let left_bracket = table.get_terminal_id("LeftBracket").unwrap();
        let right_bracket = table.get_terminal_id("RightBracket").unwrap();
        let left_parentheses = table.get_terminal_id("LeftParentheses").unwrap();
        let right_parentheses = table.get_terminal_id("RightParentheses").unwrap();
        let comma = table.get_terminal_id("Comma").unwrap();
        let tilde = table.get_terminal_id("Tilde").unwrap();
        let positive_look_ahead = table.get_terminal_id("PositiveLookAhead").unwrap();
        let negative_look_ahead = table.get_terminal_id("NegativeLookAhead").unwrap();
        let positive_look_behind = table.get_terminal_id("PositiveLookBehind").unwrap();
        let negative_look_behind = table.get_terminal_id("NegativeLookBehind").unwrap();
        let empty = table.get_terminal_id("Empty").unwrap();
        let literal = table.get_terminal_id("Literal").unwrap();
        let regex = table.get_terminal_id("Regex").unwrap();
        let identifier = table.get_terminal_id("Identifier").unwrap();
        let left_identifier = table.get_terminal_id("LeftIdentifier").unwrap();

        // NonTerminal symbols.
        let grammar = table.get_non_terminal_id("Grammar").unwrap();
        let directive = table.get_non_terminal_id("Directive").unwrap();
        let value = table.get_non_terminal_id("Value").unwrap();
        let list = table.get_non_terminal_id("List").unwrap();
        let rule = table.get_non_terminal_id("Rule").unwrap();
        let expression = table.get_non_terminal_id("Expression").unwrap();
        let term = table.get_non_terminal_id("Term").unwrap();
        let factor = table.get_non_terminal_id("Factor").unwrap();
        let factor_repetition = table.get_non_terminal_id("FactorRepetition").unwrap();
        let atom = table.get_non_terminal_id("Atom").unwrap();
        let group = table.get_non_terminal_id("Group").unwrap();
        let optional = table.get_non_terminal_id("Optional").unwrap();
        let repetition = table.get_non_terminal_id("Repetition").unwrap();
        let lookahead = table.get_non_terminal_id("Lookahead").unwrap();
        let lookahead_group = table.get_non_terminal_id("LookaheadGroup").unwrap();

        let mut rules = Vec::new();

        // grammar = { directive | rule }
        rules.push(Rule {
            non_terminal: grammar.clone(),
            rhs: vec![
                Symbol::Nonterminal(grammar.clone()),
                Symbol::Nonterminal(directive.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: grammar.clone(),
            rhs: vec![
                Symbol::Nonterminal(grammar.clone()),
                Symbol::Nonterminal(rule.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: grammar.clone(),
            rhs: vec![],
        });

        // directive = "@" IDENTIFIER "=" value
        rules.push(Rule {
            non_terminal: directive.clone(),
            rhs: vec![
                Symbol::Terminal(at.clone()),
                Symbol::Terminal(left_identifier.clone()),
                Symbol::Terminal(equal.clone()),
                Symbol::Nonterminal(value.clone()),
            ],
        });

        // value = LITERAL | REGEX | list
        rules.push(Rule {
            non_terminal: value.clone(),
            rhs: vec![Symbol::Terminal(literal.clone())],
        });
        rules.push(Rule {
            non_terminal: value.clone(),
            rhs: vec![Symbol::Terminal(regex.clone())],
        });
        rules.push(Rule {
            non_terminal: value.clone(),
            rhs: vec![Symbol::Nonterminal(list.clone())],
        });

        // list = IDENTIFIER { "," IDENTIFIER }
        rules.push(Rule {
            non_terminal: list.clone(),
            rhs: vec![
                Symbol::Nonterminal(list.clone()),
                Symbol::Terminal(comma.clone()),
                Symbol::Terminal(identifier.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: list.clone(),
            rhs: vec![Symbol::Terminal(identifier.clone())],
        });

        // rule = IDENTIFIER "=" expression
        rules.push(Rule {
            non_terminal: rule.clone(),
            rhs: vec![
                Symbol::Terminal(left_identifier.clone()),
                Symbol::Terminal(equal.clone()),
                Symbol::Nonterminal(expression.clone()),
            ],
        });

        // expression = term { "|" term }
        rules.push(Rule {
            non_terminal: expression.clone(),
            rhs: vec![
                Symbol::Nonterminal(expression.clone()),
                Symbol::Terminal(pipe.clone()),
                Symbol::Nonterminal(term.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: expression.clone(),
            rhs: vec![Symbol::Nonterminal(term.clone())],
        });

        // term = factor { factor } | EMPTY
        rules.push(Rule {
            non_terminal: term.clone(),
            rhs: vec![
                Symbol::Nonterminal(term.clone()),
                Symbol::Nonterminal(factor.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: term.clone(),
            rhs: vec![Symbol::Nonterminal(factor.clone())],
        });
        rules.push(Rule {
            non_terminal: term.clone(),
            rhs: vec![Symbol::Terminal(empty.clone())],
        });

        // factor = { WHITESPACE } atom { WHITESPACE } [ lookahead ]
        rules.push(Rule {
            non_terminal: factor.clone(),
            rhs: vec![
                Symbol::Nonterminal(factor_repetition.clone()),
                Symbol::Nonterminal(atom.clone()),
                Symbol::Nonterminal(factor_repetition.clone()),
                Symbol::Nonterminal(lookahead.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: factor.clone(),
            rhs: vec![
                Symbol::Nonterminal(factor_repetition.clone()),
                Symbol::Nonterminal(atom.clone()),
                Symbol::Nonterminal(factor_repetition.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: factor_repetition.clone(),
            rhs: vec![
                Symbol::Nonterminal(factor_repetition.clone()),
                Symbol::Terminal(tilde.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: factor_repetition.clone(),
            rhs: vec![],
        });

        // atom = LITERAL | IDENTIFIER ! "=" | REGEX | group | optional | repetition
        rules.push(Rule {
            non_terminal: atom.clone(),
            rhs: vec![Symbol::Terminal(literal.clone())],
        });
        rules.push(Rule {
            non_terminal: atom.clone(),
            rhs: vec![Symbol::Terminal(identifier.clone())],
        });
        rules.push(Rule {
            non_terminal: atom.clone(),
            rhs: vec![Symbol::Terminal(regex.clone())],
        });
        rules.push(Rule {
            non_terminal: atom.clone(),
            rhs: vec![Symbol::Nonterminal(group.clone())],
        });
        rules.push(Rule {
            non_terminal: atom.clone(),
            rhs: vec![Symbol::Nonterminal(optional.clone())],
        });
        rules.push(Rule {
            non_terminal: atom.clone(),
            rhs: vec![Symbol::Nonterminal(repetition.clone())],
        });

        // group = "(" expression ")"
        rules.push(Rule {
            non_terminal: group.clone(),
            rhs: vec![
                Symbol::Terminal(left_parentheses.clone()),
                Symbol::Nonterminal(expression.clone()),
                Symbol::Terminal(right_parentheses.clone()),
            ],
        });

        // optional = "[" expression "]"
        rules.push(Rule {
            non_terminal: optional.clone(),
            rhs: vec![
                Symbol::Terminal(left_bracket.clone()),
                Symbol::Nonterminal(expression.clone()),
                Symbol::Terminal(right_bracket.clone()),
            ],
        });

        // repetition = "{" expression "}"
        rules.push(Rule {
            non_terminal: repetition.clone(),
            rhs: vec![
                Symbol::Terminal(left_brace.clone()),
                Symbol::Nonterminal(expression.clone()),
                Symbol::Terminal(right_brace.clone()),
            ],
        });

        // lookahead = (POSITIVE_LOOKAHEAD | NEGATIVE_LOOKAHEAD | POSITIVE_LOOKBEHIND | NEGATIVE_LOOKBEHIND) factor
        rules.push(Rule {
            non_terminal: lookahead.clone(),
            rhs: vec![
                Symbol::Nonterminal(lookahead_group.clone()),
                Symbol::Nonterminal(factor.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: lookahead_group.clone(),
            rhs: vec![Symbol::Terminal(positive_look_ahead.clone())],
        });
        rules.push(Rule {
            non_terminal: lookahead_group.clone(),
            rhs: vec![Symbol::Terminal(negative_look_ahead.clone())],
        });
        rules.push(Rule {
            non_terminal: lookahead_group.clone(),
            rhs: vec![Symbol::Terminal(positive_look_behind.clone())],
        });
        rules.push(Rule {
            non_terminal: lookahead_group.clone(),
            rhs: vec![Symbol::Terminal(negative_look_behind.clone())],
        });

        GrammarRules {
            start_symbol: grammar.clone(),
            rules,
        }
    })
}

#[must_use]
pub fn reduce_on(rhs: &Rhs<Terminal, NonTerminal, ()>, lookahead: Option<&Terminal>) -> bool {
    // Greedy whitespace consumption.
    !matches!((&rhs.syms[..], lookahead), (
    [
        Symbol::Nonterminal(nt1),
        Symbol::Nonterminal(nt2),
        Symbol::Nonterminal(nt3),
    ],
    Some(terminal),
    ) if (
            // snake_case, generated rules.
            (nt1.0.as_ref() == "factor_repetition"
                && nt2.0.as_ref() == "atom"
                && nt3.0.as_ref() == "factor_repetition"
                && terminal.0.as_ref() == "WHITESPACE")
            ||
            // CamelCase, hard coded rules.
            (nt1.0.as_ref() == "FactorRepetition"
                && nt2.0.as_ref() == "Atom"
                && nt3.0.as_ref() == "FactorRepetition"
                && terminal.0.as_ref() == "Tilde")
        ))
}

#[must_use]
pub const fn priority_of(
    _rhs: &Rhs<Terminal, NonTerminal, ()>,
    _lookahead: Option<&Terminal>,
) -> i32 {
    0
}
