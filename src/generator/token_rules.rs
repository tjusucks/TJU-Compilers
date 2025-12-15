use std::sync::OnceLock;

use crate::common::token_rules::{Rule, TokenRules};
use crate::generator::symbol_table::symbol_table;

static TOKEN_RULES: OnceLock<TokenRules> = OnceLock::new();

pub fn token_rules() -> &'static TokenRules {
    TOKEN_RULES.get_or_init(|| {
        let table = symbol_table();

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
        let comment = table.get_terminal_id("Comment").unwrap();
        let whitespace = table.get_terminal_id("Whitespace").unwrap();

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
