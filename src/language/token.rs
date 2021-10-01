use std::borrow::Borrow;

pub struct Token {
    token_type: TokenType,
    start: *const char,
    length: usize,
    line: i32
}

pub enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,

    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,

    Semicolon,

    Bang, BangEq,
    BitComplement,
    EQ, EqEq,
    Gt, Ge,
    Le, Lt,
    BitAnd, And,
    BitOr, Or,
    XOR,

    Identifier, String, Number,

    True,
    False,
    If,
    Else,
    Return,
    While,
    For,
    Var,
    Fun,
    Class,
    Null,
    This,
    Super,

    Error,
    EOF,
}

impl Token {
    pub fn new(token_type: TokenType, start: *const char, length: usize, line: i32) -> Token {
        Token {
            token_type,
            start,
            length,
            line,
        }
    }
}