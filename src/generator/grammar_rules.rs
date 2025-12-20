use std::sync::{Arc, OnceLock};

use lalr::{Rhs, Symbol};

use crate::common::grammar_rules::{GrammarRules, Rule};
use crate::common::symbol_table::{NonTerminal, Terminal};

#[allow(clippy::vec_init_then_push)]
#[allow(clippy::too_many_lines)]
pub fn grammar_rules() -> &'static GrammarRules {
    static GRAMMAR_RULES: OnceLock<GrammarRules> = OnceLock::new();
    GRAMMAR_RULES.get_or_init(|| {

        // Terminal symbols.
        let at = Terminal(Arc::from("At"));
        let equal = Terminal(Arc::from("Equal"));
        let pipe = Terminal(Arc::from("Pipe"));
        let left_brace = Terminal(Arc::from("LeftBrace"));
        let right_brace = Terminal(Arc::from("RightBrace"));
        let left_bracket = Terminal(Arc::from("LeftBracket"));
        let right_bracket = Terminal(Arc::from("RightBracket"));
        let left_parentheses = Terminal(Arc::from("LeftParentheses"));
        let right_parentheses = Terminal(Arc::from("RightParentheses"));
        let comma = Terminal(Arc::from("Comma"));
        let tilde = Terminal(Arc::from("Tilde"));
        let positive_look_ahead = Terminal(Arc::from("PositiveLookAhead"));
        let negative_look_ahead = Terminal(Arc::from("NegativeLookAhead"));
        let positive_look_behind = Terminal(Arc::from("PositiveLookBehind"));
        let negative_look_behind = Terminal(Arc::from("NegativeLookBehind"));
        let empty = Terminal(Arc::from("Empty"));
        let literal = Terminal(Arc::from("Literal"));
        let regex = Terminal(Arc::from("Regex"));
        let identifier = Terminal(Arc::from("Identifier"));
        let left_identifier = Terminal(Arc::from("LeftIdentifier"));

        // NonTerminal symbols.
        let grammar = NonTerminal(Arc::from("Grammar"));
        let directive = NonTerminal(Arc::from("Directive"));
        let value = NonTerminal(Arc::from("Value"));
        let list = NonTerminal(Arc::from("List"));
        let rule = NonTerminal(Arc::from("Rule"));
        let expression = NonTerminal(Arc::from("Expression"));
        let term = NonTerminal(Arc::from("Term"));
        let factor = NonTerminal(Arc::from("Factor"));
        let factor_repetition = NonTerminal(Arc::from("FactorRepetition"));
        let atom = NonTerminal(Arc::from("Atom"));
        let group = NonTerminal(Arc::from("Group"));
        let optional = NonTerminal(Arc::from("Optional"));
        let repetition = NonTerminal(Arc::from("Repetition"));
        let lookahead = NonTerminal(Arc::from("Lookahead"));
        let lookahead_group = NonTerminal(Arc::from("LookaheadGroup"));

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
            non_terminal: directive,
            rhs: vec![
                Symbol::Terminal(at),
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
            non_terminal: value,
            rhs: vec![Symbol::Nonterminal(list.clone())],
        });

        // list = IDENTIFIER { "," IDENTIFIER }
        rules.push(Rule {
            non_terminal: list.clone(),
            rhs: vec![
                Symbol::Nonterminal(list.clone()),
                Symbol::Terminal(comma),
                Symbol::Terminal(identifier.clone()),
            ],
        });
        rules.push(Rule {
            non_terminal: list,
            rhs: vec![Symbol::Terminal(identifier.clone())],
        });

        // rule = IDENTIFIER "=" expression
        rules.push(Rule {
            non_terminal: rule,
            rhs: vec![
                Symbol::Terminal(left_identifier),
                Symbol::Terminal(equal),
                Symbol::Nonterminal(expression.clone()),
            ],
        });

        // expression = term { "|" term }
        rules.push(Rule {
            non_terminal: expression.clone(),
            rhs: vec![
                Symbol::Nonterminal(expression.clone()),
                Symbol::Terminal(pipe),
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
            non_terminal: term,
            rhs: vec![Symbol::Terminal(empty)],
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
                Symbol::Terminal(tilde),
            ],
        });
        rules.push(Rule {
            non_terminal: factor_repetition,
            rhs: vec![],
        });

        // atom = LITERAL | IDENTIFIER ! "=" | REGEX | group | optional | repetition
        rules.push(Rule {
            non_terminal: atom.clone(),
            rhs: vec![Symbol::Terminal(literal)],
        });
        rules.push(Rule {
            non_terminal: atom.clone(),
            rhs: vec![Symbol::Terminal(identifier)],
        });
        rules.push(Rule {
            non_terminal: atom.clone(),
            rhs: vec![Symbol::Terminal(regex)],
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
            non_terminal: atom,
            rhs: vec![Symbol::Nonterminal(repetition.clone())],
        });

        // group = "(" expression ")"
        rules.push(Rule {
            non_terminal: group,
            rhs: vec![
                Symbol::Terminal(left_parentheses),
                Symbol::Nonterminal(expression.clone()),
                Symbol::Terminal(right_parentheses),
            ],
        });

        // optional = "[" expression "]"
        rules.push(Rule {
            non_terminal: optional,
            rhs: vec![
                Symbol::Terminal(left_bracket),
                Symbol::Nonterminal(expression.clone()),
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
                Symbol::Nonterminal(lookahead_group.clone()),
                Symbol::Nonterminal(factor),
            ],
        });
        rules.push(Rule {
            non_terminal: lookahead_group.clone(),
            rhs: vec![Symbol::Terminal(positive_look_ahead)],
        });
        rules.push(Rule {
            non_terminal: lookahead_group.clone(),
            rhs: vec![Symbol::Terminal(negative_look_ahead)],
        });
        rules.push(Rule {
            non_terminal: lookahead_group.clone(),
            rhs: vec![Symbol::Terminal(positive_look_behind)],
        });
        rules.push(Rule {
            non_terminal: lookahead_group,
            rhs: vec![Symbol::Terminal(negative_look_behind)],
        });

        GrammarRules {
            start_symbol: grammar,
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
