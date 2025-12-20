use std::sync::{Arc, OnceLock};

use crate::common::symbol_table::Terminal;
use crate::common::token_rules::{Rule, TokenRules};

static TOKEN_RULES: OnceLock<TokenRules> = OnceLock::new();

pub fn token_rules() -> &'static TokenRules {
    TOKEN_RULES.get_or_init(|| {
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
        let comment = Terminal(Arc::from("Comment"));
        let whitespace = Terminal(Arc::from("Whitespace"));

        vec![
            Rule {
                kind: at,
                regex: r"@".to_string(),
                skip: false,
            },
            Rule {
                kind: equal,
                regex: r"=".to_string(),
                skip: false,
            },
            Rule {
                kind: pipe,
                regex: r"\|".to_string(),
                skip: false,
            },
            Rule {
                kind: left_brace,
                regex: r"\{".to_string(),
                skip: false,
            },
            Rule {
                kind: right_brace,
                regex: r"\}".to_string(),
                skip: false,
            },
            Rule {
                kind: left_bracket,
                regex: r"\[".to_string(),
                skip: false,
            },
            Rule {
                kind: right_bracket,
                regex: r"\]".to_string(),
                skip: false,
            },
            Rule {
                kind: left_parentheses,
                regex: r"\(".to_string(),
                skip: false,
            },
            Rule {
                kind: right_parentheses,
                regex: r"\)".to_string(),
                skip: false,
            },
            Rule {
                kind: comma,
                regex: r",".to_string(),
                skip: false,
            },
            Rule {
                kind: tilde,
                regex: r"~".to_string(),
                skip: false,
            },
            Rule {
                kind: positive_look_ahead,
                regex: r"&".to_string(),
                skip: false,
            },
            Rule {
                kind: negative_look_ahead,
                regex: r"!".to_string(),
                skip: false,
            },
            Rule {
                kind: positive_look_behind,
                regex: r"<-&".to_string(),
                skip: false,
            },
            Rule {
                kind: negative_look_behind,
                regex: r"<-!".to_string(),
                skip: false,
            },
            Rule {
                kind: empty,
                regex: r"EPSILON".to_string(),
                skip: false,
            },
            Rule {
                kind: literal,
                regex: r#""([^"\\]|\\.)*""#.to_string(),
                skip: false,
            },
            Rule {
                kind: regex,
                regex: r"/(?:[^/\\]|\\.)*/".to_string(),
                skip: false,
            },
            Rule {
                kind: identifier,
                regex: r"[A-Za-z_][A-Za-z_0-9]*".to_string(),
                skip: false,
            },
            Rule {
                kind: comment,
                regex: r"#.*".to_string(),
                skip: true,
            },
            Rule {
                kind: whitespace,
                regex: r"\s+".to_string(),
                skip: true,
            },
        ]
    })
}
