use crate::token::Token;
use relex::{RecognizerBuilder, Rule, Token as RelexToken};

pub fn tokenize(input: &str) -> Vec<RelexToken<'_, Token>> {
    let lexer = RecognizerBuilder::new()
        .token(Rule::new(Token::At, r"@").unwrap())
        .token(Rule::new(Token::Equal, r"=").unwrap())
        .token(Rule::new(Token::Pipe, r"\|").unwrap())
        .token(Rule::new(Token::LeftBrace, r"\{").unwrap())
        .token(Rule::new(Token::RightBrace, r"\}").unwrap())
        .token(Rule::new(Token::LeftBracket, r"\[").unwrap())
        .token(Rule::new(Token::RightBracket, r"\]").unwrap())
        .token(Rule::new(Token::LeftParentheses, r"\(").unwrap())
        .token(Rule::new(Token::RightParentheses, r"\)").unwrap())
        .token(Rule::new(Token::Comma, r",").unwrap())
        .token(Rule::new(Token::Tilde, r"~").unwrap())
        .token(Rule::new(Token::PositiveLookAhead, r"&").unwrap())
        .token(Rule::new(Token::NegativeLookAhead, r"!").unwrap())
        .token(Rule::new(Token::PositiveLookBehind, r"<-&").unwrap())
        .token(Rule::new(Token::NegativeLookBehind, r"<-!").unwrap())
        .token(Rule::new(Token::Literal, r#""([^"\\]|\\.)*""#).unwrap())
        .token(Rule::new(Token::Regex, r"/(?:[^/\\]|\\.)*/").unwrap())
        .token(Rule::new(Token::Identifier, r"[A-Za-z_][A-Za-z_0-9]*").unwrap())
        .token(Rule::new(Token::Comment, r"#.*").unwrap().skip(true))
        .token(Rule::new(Token::Whitespace, r"\s+").unwrap().skip(true))
        .build()
        .into_lexer(input, 0);

    let mut tokens = Vec::new();
    for token in lexer {
        tokens.push(token);
    }
    tokens
}
