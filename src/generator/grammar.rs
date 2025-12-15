use std::sync::OnceLock;

use lalr::{Grammar, Rhs, Symbol};

use crate::common::symbol_table::{NonTerminal, Terminal};
use crate::generator::symbol_table::symbol_table;

macro_rules! add_rules {
    ($grammar:expr, $($lhs:expr => [$($rhs:expr),*]),* $(,)?) => {
        $(
            $grammar.rules.entry($lhs).or_default().push(Rhs {
                syms: vec![$($rhs),*],
                act: (),
            });
        )*
    };
}

pub fn grammar() -> &'static Grammar<Terminal, NonTerminal, ()> {
    static GRAMMAR: OnceLock<Grammar<Terminal, NonTerminal, ()>> = OnceLock::new();
    GRAMMAR.get_or_init(|| {
        let table = symbol_table();

        // Terminal symbols
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

        // NonTerminal symbols
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

        let mut grammar: Grammar<Terminal, NonTerminal, ()> = Grammar {
            rules: std::collections::BTreeMap::new(),
            start: grammar_nt,
        };

        // grammar = { directive | rule }
        add_rules! {
            grammar,
            grammar_nt => [
                Symbol::Nonterminal(grammar_nt),
                Symbol::Nonterminal(grammar_repetition)
            ],
            grammar_repetition => [
                Symbol::Nonterminal(directive)
            ],
            grammar_repetition => [
                Symbol::Nonterminal(rule)
            ],
            grammar_nt => [],
        };

        // directive = "@" IDENTIFIER "=" value
        add_rules! {
            grammar,
            directive => [
                Symbol::Terminal(at),
                Symbol::Terminal(left_identifier),
                Symbol::Terminal(equal),
                Symbol::Nonterminal(value)
            ],
        };

        // value = LITERAL | REGEX | list
        add_rules! {
            grammar,
            value => [
                Symbol::Terminal(literal)
            ],
            value => [
                Symbol::Terminal(regex)
            ],
            value => [
                Symbol::Nonterminal(list)
            ],
        };

        // list = IDENTIFIER { "," IDENTIFIER }
        add_rules! {
            grammar,
            list_repetition => [
                Symbol::Nonterminal(list)
            ],
            list => [
                Symbol::Nonterminal(list),
                Symbol::Terminal(comma),
                Symbol::Terminal(identifier)
            ],
            list => [
                Symbol::Terminal(identifier)
            ]
        };

        // EBNF constructs.
        // rule = IDENTIFIER "=" expression
        add_rules! {
            grammar,
            rule => [
                Symbol::Terminal(left_identifier),
                Symbol::Terminal(equal),
                Symbol::Nonterminal(expression)
            ],
        };

        // expression = term { "|" term }
        add_rules! {
            grammar,
            expression_repetition => [
                Symbol::Nonterminal(expression)
            ],
            expression => [
                Symbol::Nonterminal(expression),
                Symbol::Terminal(pipe),
                Symbol::Nonterminal(term)
            ],
            expression => [
                Symbol::Nonterminal(term)
            ],
        };

        // term = factor { factor }
        add_rules! {
            grammar,
            term_repetition => [
                Symbol::Nonterminal(term)
            ],
            term => [
                Symbol::Nonterminal(term),
                Symbol::Nonterminal(factor)
            ],
            term => [
                Symbol::Nonterminal(factor)
            ],
        };

        // factor = { WHITESPACE } atom { WHITESPACE } [ lookahead ]
        add_rules! {
            grammar,
            factor => [
                Symbol::Nonterminal(factor_repetition),
                Symbol::Nonterminal(atom),
                Symbol::Nonterminal(factor_repetition),
                Symbol::Nonterminal(lookahead)
            ],
            factor => [
                Symbol::Nonterminal(factor_repetition),
                Symbol::Nonterminal(atom),
                Symbol::Nonterminal(factor_repetition)
            ],
            factor_repetition => [
                Symbol::Nonterminal(factor_repetition),
                Symbol::Terminal(tilde)
            ],
            factor_repetition => [],
        };

        // atom = LITERAL | IDENTIFIER ! "=" | REGEX | group | optional | repetition
        add_rules! {
            grammar,
            atom => [
                Symbol::Terminal(literal)
            ],
            atom => [
                Symbol::Terminal(identifier)
            ],
            atom => [
                Symbol::Terminal(regex)
            ],
            atom => [
                Symbol::Nonterminal(group)
            ],
            atom => [
                Symbol::Nonterminal(optional)
            ],
            atom => [
                Symbol::Nonterminal(repetition)
            ],
        };

        // group = "(" expression ")"
        add_rules! {
            grammar,
            group => [
                Symbol::Terminal(left_parentheses),
                Symbol::Nonterminal(expression),
                Symbol::Terminal(right_parentheses)
            ],
        };

        // optional = "[" expression "]"
        add_rules! {
            grammar,
            optional => [
                Symbol::Terminal(left_bracket),
                Symbol::Nonterminal(expression),
                Symbol::Terminal(right_bracket)
            ],
        };

        // repetition = "{" expression "}"
        add_rules! {
            grammar,
            repetition => [
                Symbol::Terminal(left_brace),
                Symbol::Nonterminal(expression),
                Symbol::Terminal(right_brace)
            ],
        };

        // lookahead = (POSITIVE_LOOKAHEAD | NEGATIVE_LOOKAHEAD | POSITIVE_LOOKBEHIND | NEGATIVE_LOOKBEHIND) factor
        add_rules! {
            grammar,
            lookahead => [
                Symbol::Nonterminal(lookahead_group),
                Symbol::Nonterminal(factor)
            ],
            lookahead_group => [
                Symbol::Terminal(positive_look_ahead)
            ],
            lookahead_group => [
                Symbol::Terminal(negative_look_ahead)
            ],
            lookahead_group => [
                Symbol::Terminal(positive_look_behind)
            ],
            lookahead_group => [
                Symbol::Terminal(negative_look_behind)
            ],
        };

        grammar
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
