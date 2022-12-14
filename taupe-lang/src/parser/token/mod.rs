#[derive(new, Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // -- Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    DotDot,
    DotDotEqual,
    Minus,
    Plus,
    // SEMICOLON,
    Slash,
    Star,

    // -- One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // -- Literals.
    Identifier,
    String,
    Number,

    // -- Keywords.
    // AND,
    // CLASS,
    Else,
    False,
    // FUN,
    For,
    In,
    If,
    // NIL,
    // OR,
    Print,
    // RETURN,
    // SUPER,
    // THIS,
    True,
    Let,
    // WHILE,

    // EOF,
    EOL,
}

