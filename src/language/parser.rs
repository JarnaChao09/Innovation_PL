use std::mem::swap;
use std::ops::{Deref, DerefMut};
use super::token::*;
use super::scanner::*;

#[derive(Debug)]
pub struct Parser {
    pub current: Box<Token>,
    pub previous: Box<Token>,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            current: Box::new(Token::new(TokenType::EOF, String::new(), 0)),
            previous: Box::new(Token::new(TokenType::EOF, String::new(), 0)),
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self, scanner: &mut Scanner) {
        swap(self.previous.deref_mut(), self.current.deref_mut());

        loop {
            self.current = Box::new(scanner.scan_token());

            if self.current.token_type != TokenType::Error {
                break;
            }

            let msg = self.current.lexme.to_string();
            self.error_at_current(msg);
        }
    }

    pub fn consume(&mut self, scanner: &mut Scanner, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance(scanner)
        }

        self.error_at_current(message);
    }

    pub fn error_at_current(&mut self, message: String) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.had_error = true;
        Parser::error_at(&self.current, message);
    }

    pub fn error(&mut self, message: String) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.had_error = true;
        Parser::error_at(&self.previous, message);
    }

    fn error_at(token: *const Box<Token>, message: String) {
        unsafe {
            eprint!("[line {}] Error", (**token).line);

            if (**token).token_type == TokenType::EOF {
                eprint!(" at end");
            } else if (**token).token_type == TokenType::Error {} else {
                eprint!(" at '{}'", (**token).lexme);
            }
        }

        eprintln!(": {}", message);
    }
}