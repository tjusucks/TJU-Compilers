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
        let literal = table.get_terminal_id("Literal").unwrap();
        let regex = table.get_terminal_id("Regex").unwrap();
        let identifier = table.get_terminal_id("Identifier").unwrap();
        let left_identifier = table.get_terminal_id("LeftIdentifier").unwrap();

        // NonTerminal symbols.
        let grammar_nt = table.get_non_terminal_id("Grammar").unwrap();
        let grammar_repetition = table.get_non_terminal_id("GrammarRepetition").unwrap();
        let directive = table.get_non_terminal_id("Directive").unwrap();
        let value = table.get_non_terminal_id("Value").unwrap();
        let list = table.get_non_terminal_id("List").unwrap();
        let list_repetition = table.get_non_terminal_id("ListRepetition").unwrap();
        let rule = table.get_non_terminal_id("Rule").unwrap();
        let expression = table.get_non_terminal_id("Expression").unwrap();
        let expression_repetition = table.get_non_terminal_id("ExpressionRepetition").unwrap();
        let term = table.get_non_terminal_id("Term").unwrap();
        let term_repetition = table.get_non_terminal_id("TermRepetition").unwrap();
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
            non_terminal: grammar_nt,
            rhs: vec![
                Symbol::Nonterminal(grammar_nt),
                Symbol::Nonterminal(grammar_repetition),
            ],
        });
        rules.push(Rule {
            non_terminal: grammar_repetition,
            rhs: vec![Symbol::Nonterminal(directive)],
        });
        rules.push(Rule {
            non_terminal: grammar_repetition,
            rhs: vec![Symbol::Nonterminal(rule)],
        });
        rules.push(Rule {
            non_terminal: grammar_nt,
            rhs: vec![],
        });

        // directive = "@" IDENTIFIER "=" value
        rules.push(Rule {
            non_terminal: directive,
            rhs: vec![
                Symbol::Terminal(at),
                Symbol::Terminal(left_identifier),
                Symbol::Terminal(equal),
                Symbol::Nonterminal(value),
            ],
        });

        // value = LITERAL | REGEX | list
        rules.push(Rule {
            non_terminal: value,
            rhs: vec![Symbol::Terminal(literal)],
        });
        rules.push(Rule {
            non_terminal: value,
            rhs: vec![Symbol::Terminal(regex)],
        });
        rules.push(Rule {
            non_terminal: value,
            rhs: vec![Symbol::Nonterminal(list)],
        });

        // list = IDENTIFIER { "," IDENTIFIER }
        rules.push(Rule {
            non_terminal: list_repetition,
            rhs: vec![Symbol::Nonterminal(list)],
        });
        rules.push(Rule {
            non_terminal: list,
            rhs: vec![
                Symbol::Nonterminal(list),
                Symbol::Terminal(comma),
                Symbol::Terminal(identifier),
            ],
        });
        rules.push(Rule {
            non_terminal: list,
            rhs: vec![Symbol::Terminal(identifier)],
        });

        // rule = IDENTIFIER "=" expression
        rules.push(Rule {
            non_terminal: rule,
            rhs: vec![
                Symbol::Terminal(left_identifier),
                Symbol::Terminal(equal),
                Symbol::Nonterminal(expression),
            ],
        });

        // expression = term { "|" term }
        rules.push(Rule {
            non_terminal: expression_repetition,
            rhs: vec![Symbol::Nonterminal(expression)],
        });
        rules.push(Rule {
            non_terminal: expression,
            rhs: vec![
                Symbol::Nonterminal(expression),
                Symbol::Terminal(pipe),
                Symbol::Nonterminal(term),
            ],
        });
        rules.push(Rule {
            non_terminal: expression,
            rhs: vec![Symbol::Nonterminal(term)],
        });

        // term = factor { factor }
        rules.push(Rule {
            non_terminal: term_repetition,
            rhs: vec![Symbol::Nonterminal(term)],
        });
        rules.push(Rule {
            non_terminal: term,
            rhs: vec![Symbol::Nonterminal(term), Symbol::Nonterminal(factor)],
        });
        rules.push(Rule {
            non_terminal: term,
            rhs: vec![Symbol::Nonterminal(factor)],
        });

        // factor = { WHITESPACE } atom { WHITESPACE } [ lookahead ]
        rules.push(Rule {
            non_terminal: factor,
            rhs: vec![
                Symbol::Nonterminal(factor_repetition),
                Symbol::Nonterminal(atom),
                Symbol::Nonterminal(factor_repetition),
                Symbol::Nonterminal(lookahead),
            ],
        });
        rules.push(Rule {
            non_terminal: factor,
            rhs: vec![
                Symbol::Nonterminal(factor_repetition),
                Symbol::Nonterminal(atom),
                Symbol::Nonterminal(factor_repetition),
            ],
        });
        rules.push(Rule {
            non_terminal: factor_repetition,
            rhs: vec![
                Symbol::Nonterminal(factor_repetition),
                Symbol::Terminal(tilde),
            ],
        });
        rules.push(Rule {
            non_terminal: factor_repetition,
            rhs: vec![],
        });

        // atom = LITERAL | IDENTIFIER ! "=" | REGEX | group | optional | repetition
        rules.push(Rule {
            non_terminal: atom,
            rhs: vec![Symbol::Terminal(literal)],
        });
        rules.push(Rule {
            non_terminal: atom,
            rhs: vec![Symbol::Terminal(identifier)],
        });
        rules.push(Rule {
            non_terminal: atom,
            rhs: vec![Symbol::Terminal(regex)],
        });
        rules.push(Rule {
            non_terminal: atom,
            rhs: vec![Symbol::Nonterminal(group)],
        });
        rules.push(Rule {
            non_terminal: atom,
            rhs: vec![Symbol::Nonterminal(optional)],
        });
        rules.push(Rule {
            non_terminal: atom,
            rhs: vec![Symbol::Nonterminal(repetition)],
        });

        // group = "(" expression ")"
        rules.push(Rule {
            non_terminal: group,
            rhs: vec![
                Symbol::Terminal(left_parentheses),
                Symbol::Nonterminal(expression),
                Symbol::Terminal(right_parentheses),
            ],
        });

        // optional = "[" expression "]"
        rules.push(Rule {
            non_terminal: optional,
            rhs: vec![
                Symbol::Terminal(left_bracket),
                Symbol::Nonterminal(expression),
                Symbol::Terminal(right_bracket),
            ],
        });

        // repetition = "{" expression "}"
        rules.push(Rule {
            non_terminal: repetition,
            rhs: vec![
                Symbol::Terminal(left_brace),
                Symbol::Nonterminal(expression),
                Symbol::Terminal(right_brace),
            ],
        });

        // lookahead = (POSITIVE_LOOKAHEAD | NEGATIVE_LOOKAHEAD | POSITIVE_LOOKBEHIND | NEGATIVE_LOOKBEHIND) factor
        rules.push(Rule {
            non_terminal: lookahead,
            rhs: vec![
                Symbol::Nonterminal(lookahead_group),
                Symbol::Nonterminal(factor),
            ],
        });
        rules.push(Rule {
            non_terminal: lookahead_group,
            rhs: vec![Symbol::Terminal(positive_look_ahead)],
        });
        rules.push(Rule {
            non_terminal: lookahead_group,
            rhs: vec![Symbol::Terminal(negative_look_ahead)],
        });
        rules.push(Rule {
            non_terminal: lookahead_group,
            rhs: vec![Symbol::Terminal(positive_look_behind)],
        });
        rules.push(Rule {
            non_terminal: lookahead_group,
            rhs: vec![Symbol::Terminal(negative_look_behind)],
        });

        GrammarRules {
            start_symbol: grammar_nt,
            rules,
        }
    })
}

pub fn reduce_on(rhs: &Rhs<Terminal, NonTerminal, ()>, lookahead: Option<&Terminal>) -> bool {
    let table = symbol_table();
    let factor_repetition = table.get_non_terminal_id("FactorRepetition").unwrap();
    let atom = table.get_non_terminal_id("Atom").unwrap();
    let tilde = table.get_terminal_id("Tilde").unwrap();

    match (&rhs.syms[..], lookahead) {
        // Greedy whitespace consumption.
        (
            [
                Symbol::Nonterminal(nt1),
                Symbol::Nonterminal(nt2),
                Symbol::Nonterminal(nt3),
            ],
            Some(term),
        ) if *nt1 == factor_repetition
            && *nt2 == atom
            && *nt3 == factor_repetition
            && *term == tilde =>
        {
            false
        }
        _ => true,
    }
}

pub const fn priority_of(
    _rhs: &Rhs<Terminal, NonTerminal, ()>,
    _lookahead: Option<&Terminal>,
) -> i32 {
    0
}
