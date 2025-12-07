use relex::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Symbolic tokens.
    At,               // '@'
    Equal,            // '='
    Pipe,             // '|'
    LeftBrace,        // '{'
    RightBrace,       // '}'
    LeftBracket,      // '['
    RightBracket,     // ']'
    LeftParentheses,  // '('
    RightParentheses, // ')'
    Comma,            // ','
    Tilde,            // '~'

    // Lookahead / lookbehind operators.
    PositiveLookAhead,  // '&'
    NegativeLookAhead,  // '!'
    PositiveLookBehind, // '<-&'
    NegativeLookBehind, // '<-!'

    // Literal tokens, for strings and regexes.
    Literal,
    Regex,

    // Identifier, for nonterminal names, directive names, etc.
    Identifier,

    // Tokens to be skipped.
    Comment,
    Whitespace,

    // Special tokens.
    Eof,
    Unrecognized,
}

impl TokenKind for Token {
    fn eof() -> Self {
        Token::Eof
    }
    fn unrecognized() -> Self {
        Token::Unrecognized
    }
}
