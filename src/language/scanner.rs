use rawpointer::PointerExt;
use libc::memcmp;
use super::token::*;

pub struct Scanner {
    start: *mut char,
    current: *mut char,
    line: i32,
}

impl Scanner {
    pub fn new(source: &mut String) -> Scanner {
        Scanner {
            start: source.as_mut_ptr() as *mut char,
            current: source.as_mut_ptr() as *mut char,
            line: 1
        }
    }

    pub fn scan_token(&mut self) -> Token {
        unsafe {
            self.skip_whitespace();
            *self.start = *self.current;
            if self.is_at_end() {
                return Token::new(TokenType::EOF, self.start, (*self.current as u8 - *self.start as u8) as usize, self.line);
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
                _ => self.error_token(&mut String::from("Unexpected character."))
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

    pub fn error_token(&self, message: &mut String) -> Token {
        Token::new(
            TokenType::Error,
            message.as_mut_ptr() as *const char,
            message.len(),
            self.line
        )
    }

    pub fn make_token(&self, token: TokenType) -> Token {
        unsafe {
            Token::new(
                token,
                self.start,
                (*self.current as i32 - *self.start as i32) as usize,
                self.line,
            )
        }
    }

    pub fn is_at_end(&self) -> bool {
        unsafe { *self.current == (0 as char) }
    }

    pub fn advance(&mut self) -> char {
        unsafe { *self.current.post_inc() }
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        unsafe {
            if self.is_at_end() {
                return false;
            }

            if *self.current != expected {
                return false;
            }

            self.current.post_inc();

            return true;
        }
    }

    pub fn peek(&self) -> char {
        self.peek_n(0)
    }

    pub fn peek_next(&self) -> char {
        self.peek_n(1)
    }

    pub fn peek_n(&self, n: usize) -> char {
        unsafe { *self.current.add(n) }
    }

    pub fn make_string(&mut self) -> Token {
        while self.peek() != '"' && self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token(&mut String::from("Unterminated String"))
        }

        self.advance();

        self.make_token(TokenType::String)
    }

    pub fn make_number(&mut self) -> Token {
        while self.peek().is_numeric() {
            self.peek();
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
            match *self.start {
                'c' => self.match_keyword(1, &mut String::from("lass"), TokenType::Class),
                'e' => self.match_keyword(1, &mut String::from("lse"), TokenType::Else),
                'i' => self.match_keyword(1, &mut String::from("f"), TokenType::If),
                'n' => self.match_keyword(1, &mut String::from("ull"), TokenType::Null),
                'r' => self.match_keyword(1, &mut String::from("eturn"), TokenType::Return),
                's' => self.match_keyword(1, &mut String::from("uper"), TokenType::Super),
                'w' => self.match_keyword(1, &mut String::from("hile"), TokenType::While),
                'f' => {
                    if (self.current as i32) - (self.start as i32) > 1 {
                        match *self.start.add(1) {
                            'a' => self.match_keyword(2, &mut String::from("lse"), TokenType::False),
                            'u' => self.match_keyword(2, &mut String::from("n"), TokenType::Fun),
                            'o' => self.match_keyword(2, &mut String::from("r"), TokenType::For),
                            _ => TokenType::Identifier
                        }
                    } else {
                        TokenType::Identifier
                    }
                }
                'v' => {
                    if (self.current as i32) - (self.start as i32) > 2 {
                        match *self.start.add(1) {
                            'a' => {
                                match *self.start.add(2) {
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

    pub fn match_keyword(&self, start: usize, rest: &mut String, return_type: TokenType) -> TokenType {
        unsafe {
            if ((self.current as i32) - (self.start as i32)) as usize == start + rest.len() && memcmp(((self.start as usize) + start) as *mut u8 as *const libc::c_void, rest.as_mut_ptr() as *const libc::c_void, rest.len()) == 0 {
                return return_type
            }
            return TokenType::Identifier
        }
    }
}