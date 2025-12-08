use crate::symbol::Terminal;
use relex::{RecognizerBuilder, Rule, Token};

pub fn tokenize(input: &str) -> Vec<Token<'_, Terminal>> {
    let lexer = RecognizerBuilder::new()
        .token(Rule::new(Terminal::At, r"@").unwrap())
        .token(Rule::new(Terminal::Equal, r"=").unwrap())
        .token(Rule::new(Terminal::Pipe, r"\|").unwrap())
        .token(Rule::new(Terminal::LeftBrace, r"\{").unwrap())
        .token(Rule::new(Terminal::RightBrace, r"\}").unwrap())
        .token(Rule::new(Terminal::LeftBracket, r"\[").unwrap())
        .token(Rule::new(Terminal::RightBracket, r"\]").unwrap())
        .token(Rule::new(Terminal::LeftParentheses, r"\(").unwrap())
        .token(Rule::new(Terminal::RightParentheses, r"\)").unwrap())
        .token(Rule::new(Terminal::Comma, r",").unwrap())
        .token(Rule::new(Terminal::Tilde, r"~").unwrap())
        .token(Rule::new(Terminal::PositiveLookAhead, r"&").unwrap())
        .token(Rule::new(Terminal::NegativeLookAhead, r"!").unwrap())
        .token(Rule::new(Terminal::PositiveLookBehind, r"<-&").unwrap())
        .token(Rule::new(Terminal::NegativeLookBehind, r"<-!").unwrap())
        .token(Rule::new(Terminal::Literal, r#""([^"\\]|\\.)*""#).unwrap())
        .token(Rule::new(Terminal::Regex, r"/(?:[^/\\]|\\.)*/").unwrap())
        .token(Rule::new(Terminal::Identifier, r"[A-Za-z_][A-Za-z_0-9]*").unwrap())
        .token(Rule::new(Terminal::Comment, r"#.*").unwrap().skip(true))
        .token(Rule::new(Terminal::Whitespace, r"\s+").unwrap().skip(true))
        .build()
        .into_lexer(input, 0);

    let mut tokens = Vec::new();
    for token in lexer {
        tokens.push(token);
    }
    tokens
}
