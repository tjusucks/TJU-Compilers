use relex::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Terminal {
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
    LeftIdentifier, // Identifier before an equal sign.

    // Tokens to be skipped.
    Comment,
    Whitespace,

    // Special tokens.
    Eof,
    Unrecognized,
}

impl TokenKind for Terminal {
    fn eof() -> Self {
        Self::Eof
    }
    fn unrecognized() -> Self {
        Self::Unrecognized
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonTerminal {
    // Grammar.
    Grammar,
    GrammarRepetition,

    // Directive.
    Directive,
    Value,
    List,
    ListRepetition,

    // EBNF constructs.
    Rule,
    Expression,
    Term,
    Factor,
    FactorRepetition,
    Atom,
    Group,
    Optional,
    Repetition,
    Lookahead,
    LookaheadGroup,
}
