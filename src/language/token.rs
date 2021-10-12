#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexme: String,
    pub line: i32
}

#[derive(Debug, Eq, PartialEq)]
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
    Val,
    Fun,
    Class,
    Null,
    This,
    Super,

    Error,
    EOF,
}

impl From<TokenType> for usize {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::LeftParen => {0}
            TokenType::RightParen => {1}
            TokenType::LeftBrace => {2}
            TokenType::RightBrace => {3}
            TokenType::LeftBracket => {4}
            TokenType::RightBracket => {5}
            TokenType::Comma => {6}
            TokenType::Dot => {7}
            TokenType::Plus => {8}
            TokenType::Minus => {9}
            TokenType::Star => {10}
            TokenType::Slash => {11}
            TokenType::Semicolon => {12}
            TokenType::Bang => {13}
            TokenType::BangEq => {14}
            TokenType::BitComplement => {15}
            TokenType::EQ => {16}
            TokenType::EqEq => {17}
            TokenType::Gt => {18}
            TokenType::Ge => {19}
            TokenType::Le => {20}
            TokenType::Lt => {21}
            TokenType::BitAnd => {22}
            TokenType::And => {23}
            TokenType::BitOr => {24}
            TokenType::Or => {25}
            TokenType::XOR => {26}
            TokenType::Identifier => {27}
            TokenType::String => {28}
            TokenType::Number => {29}
            TokenType::True => {30}
            TokenType::False => {31}
            TokenType::If => {32}
            TokenType::Else => {33}
            TokenType::Return => {34}
            TokenType::While => {35}
            TokenType::For => {36}
            TokenType::Var => {37}
            TokenType::Val => {38}
            TokenType::Fun => {39}
            TokenType::Class => {40}
            TokenType::Null => {41}
            TokenType::This => {42}
            TokenType::Super => {43}
            TokenType::Error => {44}
            TokenType::EOF => {45}
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexme: String, line: i32) -> Token {
        Token {
            token_type,
            lexme,
            line,
        }
    }
}