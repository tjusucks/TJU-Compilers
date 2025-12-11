use std::sync::OnceLock;

use lalr::{Grammar, Rhs, Symbol};

use crate::symbol::{NonTerminal, Terminal};

pub struct ParserConfig;

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

impl ParserConfig {
    pub fn grammar() -> &'static Grammar<Terminal, NonTerminal, ()> {
        static GRAMMAR: OnceLock<Grammar<Terminal, NonTerminal, ()>> = OnceLock::new();
        GRAMMAR.get_or_init(|| {
            let mut grammar: Grammar<Terminal, NonTerminal, ()> = Grammar {
                rules: std::collections::BTreeMap::new(),
                start: NonTerminal::Grammar,
            };

            // grammar = { directive | rule }
            add_rules! {
                grammar,
                NonTerminal::Grammar => [
                    Symbol::Nonterminal(NonTerminal::Grammar),
                    Symbol::Nonterminal(NonTerminal::GrammarRepetition)
                ],
                NonTerminal::GrammarRepetition => [
                    Symbol::Nonterminal(NonTerminal::Directive)
                ],
                NonTerminal::GrammarRepetition => [
                    Symbol::Nonterminal(NonTerminal::Rule)
                ],
                NonTerminal::Grammar => [],
            };

            // directive = "@" IDENTIFIER "=" value
            add_rules! {
                grammar,
                NonTerminal::Directive => [
                    Symbol::Terminal(Terminal::At),
                    Symbol::Terminal(Terminal::LeftIdentifier),
                    Symbol::Terminal(Terminal::Equal),
                    Symbol::Nonterminal(NonTerminal::Value)
                ],
            };

            // value = LITERAL | REGEX | list
            add_rules! {
                grammar,
                NonTerminal::Value => [
                    Symbol::Terminal(Terminal::Literal)
                ],
                NonTerminal::Value => [
                    Symbol::Terminal(Terminal::Regex)
                ],
                NonTerminal::Value => [
                    Symbol::Nonterminal(NonTerminal::List)
                ],
            };

            // list = IDENTIFIER { "," IDENTIFIER }
            add_rules! {
                grammar,
                NonTerminal::ListRepetition => [
                    Symbol::Nonterminal(NonTerminal::List)
                ],
                NonTerminal::List => [
                    Symbol::Nonterminal(NonTerminal::List),
                    Symbol::Terminal(Terminal::Comma),
                    Symbol::Terminal(Terminal::Identifier)
                ],
                NonTerminal::List => [
                    Symbol::Terminal(Terminal::Identifier)
                ]
            };

            // EBNF constructs.
            // rule = IDENTIFIER "=" expression
            add_rules! {
                grammar,
                NonTerminal::Rule => [
                    Symbol::Terminal(Terminal::LeftIdentifier),
                    Symbol::Terminal(Terminal::Equal),
                    Symbol::Nonterminal(NonTerminal::Expression)
                ],
            };

            // expression = term { "|" term }
            add_rules! {
                grammar,
                NonTerminal::ExpressionRepetition => [
                    Symbol::Nonterminal(NonTerminal::Expression)
                ],
                NonTerminal::Expression => [
                    Symbol::Nonterminal(NonTerminal::Expression),
                    Symbol::Terminal(Terminal::Pipe),
                    Symbol::Nonterminal(NonTerminal::Term)
                ],
                NonTerminal::Expression => [
                    Symbol::Nonterminal(NonTerminal::Term)
                ],
            };

            // term = factor { factor }
            add_rules! {
                grammar,
                NonTerminal::TermRepetition => [
                    Symbol::Nonterminal(NonTerminal::Term)
                ],
                NonTerminal::Term => [
                    Symbol::Nonterminal(NonTerminal::Term),
                    Symbol::Nonterminal(NonTerminal::Factor)
                ],
                NonTerminal::Term => [
                    Symbol::Nonterminal(NonTerminal::Factor)
                ],
            };

            // factor = { WHITESPACE } atom { WHITESPACE } [ lookahead ]
            add_rules! {
                grammar,
                NonTerminal::Factor => [
                    Symbol::Nonterminal(NonTerminal::FactorRepetition),
                    Symbol::Nonterminal(NonTerminal::Atom),
                    Symbol::Nonterminal(NonTerminal::FactorRepetition),
                    Symbol::Nonterminal(NonTerminal::Lookahead)
                ],
                NonTerminal::Factor => [
                    Symbol::Nonterminal(NonTerminal::FactorRepetition),
                    Symbol::Nonterminal(NonTerminal::Atom),
                    Symbol::Nonterminal(NonTerminal::FactorRepetition)
                ],
                NonTerminal::FactorRepetition => [
                    Symbol::Nonterminal(NonTerminal::FactorRepetition),
                    Symbol::Terminal(Terminal::Tilde)
                ],
                NonTerminal::FactorRepetition => [],
            };

            // atom = LITERAL | IDENTIFIER ! "=" | REGEX | group | optional | repetition
            add_rules! {
                grammar,
                NonTerminal::Atom => [
                    Symbol::Terminal(Terminal::Literal)
                ],
                NonTerminal::Atom => [
                    Symbol::Terminal(Terminal::Identifier)
                ],
                NonTerminal::Atom => [
                    Symbol::Terminal(Terminal::Regex)
                ],
                NonTerminal::Atom => [
                    Symbol::Nonterminal(NonTerminal::Group)
                ],
                NonTerminal::Atom => [
                    Symbol::Nonterminal(NonTerminal::Optional)
                ],
                NonTerminal::Atom => [
                    Symbol::Nonterminal(NonTerminal::Repetition)
                ],
            };

            // group = "(" expression ")"
            add_rules! {
                grammar,
                NonTerminal::Group => [
                    Symbol::Terminal(Terminal::LeftParentheses),
                    Symbol::Nonterminal(NonTerminal::Expression),
                    Symbol::Terminal(Terminal::RightParentheses)
                ],
            };

            // optional = "[" expression "]"
            add_rules! {
                grammar,
                NonTerminal::Optional => [
                    Symbol::Terminal(Terminal::LeftBracket),
                    Symbol::Nonterminal(NonTerminal::Expression),
                    Symbol::Terminal(Terminal::RightBracket)
                ],
            };

            // repetition = "{" expression "}"
            add_rules! {
                grammar,
                NonTerminal::Repetition => [
                    Symbol::Terminal(Terminal::LeftBrace),
                    Symbol::Nonterminal(NonTerminal::Expression),
                    Symbol::Terminal(Terminal::RightBrace)
                ],
            };

            // lookahead = (POSITIVE_LOOKAHEAD | NEGATIVE_LOOKAHEAD | POSITIVE_LOOKBEHIND | NEGATIVE_LOOKBEHIND) factor
            add_rules! {
                grammar,
                NonTerminal::Lookahead => [
                    Symbol::Nonterminal(NonTerminal::LookaheadGroup),
                    Symbol::Nonterminal(NonTerminal::Factor)
                ],
                NonTerminal::LookaheadGroup => [
                    Symbol::Terminal(Terminal::PositiveLookAhead)
                ],
                NonTerminal::LookaheadGroup => [
                    Symbol::Terminal(Terminal::NegativeLookAhead)
                ],
                NonTerminal::LookaheadGroup => [
                    Symbol::Terminal(Terminal::PositiveLookBehind)
                ],
                NonTerminal::LookaheadGroup => [
                    Symbol::Terminal(Terminal::NegativeLookBehind)
                ],
            };

            grammar
        })
    }

    pub fn reduce_on(rhs: &Rhs<Terminal, NonTerminal, ()>, lookahead: Option<&Terminal>) -> bool {
        match (&rhs.syms[..], lookahead) {
            // Greedy whitespace consumption.
            (
                [
                    Symbol::Nonterminal(NonTerminal::FactorRepetition),
                    Symbol::Nonterminal(NonTerminal::Atom),
                    Symbol::Nonterminal(NonTerminal::FactorRepetition),
                ],
                Some(Terminal::Tilde),
            ) => false,
            _ => true,
        }
    }

    pub const fn priority_of(
        _rhs: &Rhs<Terminal, NonTerminal, ()>,
        _lookahead: Option<&Terminal>,
    ) -> i32 {
        0
    }
}
