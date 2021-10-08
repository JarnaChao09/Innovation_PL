use rawpointer::PointerExt;
use super::token::*;

pub struct Scanner {
    source: String,
    start_index: usize,
    current_index: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start_index: 0,
            current_index: 0,
            line: 1
        }
    }

    pub fn scan_token(&mut self) -> Token {
        unsafe {
            self.skip_whitespace();
            self.start_index = self.current_index;
            if self.is_at_end() {
                return Token::new(TokenType::EOF, String::from(""), self.line);
            }

            let char = self.advance();

            match char {
                '0'..='9' => self.make_number(),
                'a'..='z' | 'A'..='Z' => self.make_identifier(),
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                '[' => self.make_token(TokenType::LeftBracket),
                ']' => self.make_token(TokenType::RightBracket),
                ';' => self.make_token(TokenType::Semicolon),
                '\n' => {
                    self.line += 1;
                    self.make_token(TokenType::Semicolon)
                }
                '.' => self.make_token(TokenType::Dot),
                ',' => self.make_token(TokenType::Comma),
                '+' => self.make_token(TokenType::Plus),
                '-' => self.make_token(TokenType::Minus),
                '*' => self.make_token(TokenType::Star),
                '/' => self.make_token(TokenType::Slash),
                '!' => {
                    let token = if self.match_char('=') {
                        TokenType::BangEq
                    } else {
                        TokenType::Bang
                    };
                    self.make_token(token)
                }
                '=' => {
                    let token= if self.match_char('=') {
                        TokenType::EqEq
                    } else {
                        TokenType::EQ
                    };
                    self.make_token(token)
                },
                '<' => {
                    let token= if self.match_char('=') {
                        TokenType::Le
                    } else {
                        TokenType::Lt
                    };
                    self.make_token(token)
                },
                '>' => {
                    let token = if self.match_char('=') {
                        TokenType::Ge
                    } else {
                        TokenType::Gt
                    };
                    self.make_token(token)
                },
                '"' => self.make_string(),
                err_char => {
                    let mut msg = String::from("Unexpected character: ");
                    msg.push(err_char);
                    self.error_token(msg)
                }
            }
        }
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => { self.advance(); }
                '/' => {
                    if self.peek() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            };
        }
    }

    pub fn error_token(&self, message: String) -> Token {
        Token::new(
            TokenType::Error,
            message,
            self.line
        )
    }

    pub fn make_token(&self, token: TokenType) -> Token {
        Token::new(
            token,
            self.source[self.start_index..self.current_index].parse().unwrap(),
            self.line,
        )
    }

    pub fn is_at_end(&self) -> bool {
        self.current_index == self.source.len()
    }

    pub fn advance(&mut self) -> char {
        self.current_index += 1;
        self.source.chars().nth(self.current_index - 1).unwrap()
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current_index).unwrap() != expected {
            return false;
        }

        self.current_index += 1;

        return true;
    }

    pub fn peek(&self) -> char {
        self.peek_n(0)
    }

    pub fn peek_next(&self) -> char {
        self.peek_n(1)
    }

    pub fn peek_n(&self, n: usize) -> char {
        self.source.chars().nth(self.current_index + n).unwrap_or_default()
    }

    pub fn make_string(&mut self) -> Token {
        while self.peek() != '"' && self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token(String::from("Unterminated String"))
        }

        self.advance();

        self.make_token(TokenType::String)
    }

    pub fn make_number(&mut self) -> Token {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    pub fn make_identifier(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_numeric() {
            self.advance();
        }

        let id_type = self.identifier_type();
        self.make_token(id_type)
    }

    pub fn identifier_type(&mut self) -> TokenType {
        unsafe {
            match self.source.chars().nth(self.start_index).unwrap() {
                'c' => self.match_keyword(1, String::from("lass"), TokenType::Class),
                'e' => self.match_keyword(1, String::from("lse"), TokenType::Else),
                'i' => self.match_keyword(1, String::from("f"), TokenType::If),
                'n' => self.match_keyword(1, String::from("ull"), TokenType::Null),
                'r' => self.match_keyword(1, String::from("eturn"), TokenType::Return),
                's' => self.match_keyword(1, String::from("uper"), TokenType::Super),
                'w' => self.match_keyword(1, String::from("hile"), TokenType::While),
                'f' => {
                    if self.current_index - self.start_index > 1 {
                        match self.source.chars().nth(self.start_index + 1).unwrap() {
                            'a' => self.match_keyword(2, String::from("lse"), TokenType::False),
                            'u' => self.match_keyword(2, String::from("n"), TokenType::Fun),
                            'o' => self.match_keyword(2, String::from("r"), TokenType::For),
                            _ => TokenType::Identifier
                        }
                    } else {
                        TokenType::Identifier
                    }
                }
                'v' => {
                    if self.current_index - self.start_index > 2 {
                        match self.source.chars().nth(self.start_index + 1).unwrap() {
                            'a' => {
                                match self.source.chars().nth(self.start_index + 2).unwrap() {
                                    'r' => TokenType::Var,
                                    'l' => TokenType::Val,
                                    _ => TokenType::Identifier
                                }
                            }
                            _ => TokenType::Identifier
                        }
                    } else {
                        TokenType::Identifier
                    }
                }
                _ => TokenType::Identifier
            }
        }
    }

    pub fn match_keyword(&self, start_index: usize, rest: String, return_type: TokenType) -> TokenType {
        if self.current_index - self.start_index == start_index + rest.len() &&
            self.source[(self.start_index + start_index)..(self.start_index + start_index + rest.len())] == rest {
            return_type
        } else {
            TokenType::Identifier
        }
    }
}