use std::mem::swap;
use std::ops::{Deref, DerefMut};
use super::token::*;
use super::scanner::*;
use super::chunk::*;
use super::opcode::*;
use super::value::*;

#[derive(Debug)]
pub struct Parser {
    pub current: Box<Token>,
    pub previous: Box<Token>,
    pub had_error: bool,
    pub panic_mode: bool,
}

#[derive(Copy, Clone, Debug)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Eq,
    Comp,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<Precedence> for u8 {
    fn from(precedence: Precedence) -> Self {
        match precedence {
            Precedence::None => {0}
            Precedence::Assignment => {1}
            Precedence::Or => {2}
            Precedence::And => {3}
            Precedence::Eq => {4}
            Precedence::Comp => {5}
            Precedence::Term => {6}
            Precedence::Factor => {7}
            Precedence::Unary => {8}
            Precedence::Call => {9}
            Precedence::Primary => {10}
        }
    }
}

impl From<u8> for Precedence {
    fn from(byte: u8) -> Self {
        match byte {
            0 => {Precedence::None}
            1 => {Precedence::Assignment}
            2 => {Precedence::Or}
            3 => {Precedence::And}
            4 => {Precedence::Eq}
            5 => {Precedence::Comp}
            6 => {Precedence::Term}
            7 => {Precedence::Factor}
            8 => {Precedence::Unary}
            9 => {Precedence::Call}
            10 => {Precedence::Primary}
            _ => {Precedence::None}
        }
    }
}

type ParseFn = fn(&mut Parser, &mut Scanner, &mut Chunk) -> ();

#[derive(Clone, Copy)]
struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

impl ParseRule {
    pub fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> ParseRule {
        ParseRule {
            prefix,
            infix,
            precedence,
        }
    }

    pub fn empty() -> ParseRule {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }
}

static RULE: [ParseRule; 46] = [
    ParseRule::new(Some(Parser::grouping), None, Precedence::None),              // (
    ParseRule::empty(),                                                          // )
    ParseRule::empty(),                                                          // {
    ParseRule::empty(),                                                          // }
    ParseRule::empty(),                                                          // [
    ParseRule::empty(),                                                          // ]
    ParseRule::empty(),                                                          // ,
    ParseRule::empty(),                                                          // .
    ParseRule::new(None, Some(Parser::binary), Precedence::Term),                // +
    ParseRule::new(Some(Parser::unary), Some(Parser::binary), Precedence::Term), // -
    ParseRule::new(None, Some(Parser::binary), Precedence::Factor),              // *
    ParseRule::new(None, Some(Parser::binary), Precedence::Factor),              // /
    ParseRule::empty(),                                                          // ;
    ParseRule::empty(),                                                          // !
    ParseRule::empty(),                                                          // !=
    ParseRule::empty(),                                                          // ~
    ParseRule::empty(),                                                          // =
    ParseRule::empty(),                                                          // ==
    ParseRule::empty(),                                                          // >
    ParseRule::empty(),                                                          // >=
    ParseRule::empty(),                                                          // <=
    ParseRule::empty(),                                                          // <
    ParseRule::empty(),                                                          // &
    ParseRule::empty(),                                                          // &&
    ParseRule::empty(),                                                          // |
    ParseRule::empty(),                                                          // ||
    ParseRule::empty(),                                                          // ^
    ParseRule::empty(),                                                          // identifier
    ParseRule::empty(),                                                          // string
    ParseRule::new(Some(Parser::number), None, Precedence::None),                // number
    ParseRule::empty(),                                                          // true
    ParseRule::empty(),                                                          // false
    ParseRule::empty(),                                                          // if
    ParseRule::empty(),                                                          // else
    ParseRule::empty(),                                                          // return
    ParseRule::empty(),                                                          // while
    ParseRule::empty(),                                                          // for
    ParseRule::empty(),                                                          // var
    ParseRule::empty(),                                                          // val
    ParseRule::empty(),                                                          // fun
    ParseRule::empty(),                                                          // class
    ParseRule::empty(),                                                          // null
    ParseRule::empty(),                                                          // this
    ParseRule::empty(),                                                          // super
    ParseRule::empty(),                                                          // error
    ParseRule::empty(),                                                          // eof
];


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

    pub fn expression(&mut self, scanner: &mut Scanner, chunk: &mut Chunk) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn grouping(&mut self, scanner: &mut Scanner, chunk: &mut Chunk) {
        self.expression(scanner, chunk);
        self.consume(scanner, TokenType::RightParen, String::from("Expect ')' after expression."));
    }

    pub fn unary(&mut self, scanner: &mut Scanner, chunk: &mut Chunk) {
        let operator_type = &self.previous.token_type;

        // self.expression(scanner, chunk);
        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => {
                self.emit_byte(chunk, OpCode::OpNegate);
            }
            _ => {
                return;
            }
        }
    }

    pub fn binary(&mut self, scanner: &mut Scanner, chunk: &mut Chunk) {
        let operator_type = &self.previous.token_type;
        let rule = self.get_rule(operator_type);

        let new_precedence = Precedence::from(u8::from(rule.precedence) + 1);
        self.parse_precedence(new_precedence);

        match operator_type {
            TokenType::Plus => {
                self.emit_byte(chunk, OpCode::OpAdd);
            }
            TokenType::Minus => {
                self.emit_byte(chunk, OpCode::OpSubtract);
            }
            TokenType::Star => {
                self.emit_byte(chunk, OpCode::OpMultiply);
            }
            TokenType::Slash => {
                self.emit_byte(chunk, OpCode::OpDivide);
            }
            _ => {
                return;
            }
        }
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) {

    }

    pub fn get_rule(&self, token_type: TokenType) -> ParseRule {
        RULE[usize::from(token_type)]
    }

    pub fn number(&mut self, scanner: &mut Scanner, chunk: &mut Chunk) {
        let value = self.previous.lexme.parse::<Value>().unwrap();
        self.emit_constant(chunk, value);
    }

    pub fn emit_byte(&self, chunk: &mut Chunk, opcode: OpCode) {
        chunk.write(opcode, self.previous.line);
    }

    pub fn emit_bytes(&self, chunk: &mut Chunk, opcode1: OpCode, opcode2: OpCode) {
        self.emit_byte(chunk, opcode1);
        self.emit_byte(chunk, opcode2);
    }

    pub fn emit_return(&self, chunk: &mut Chunk) {
        self.emit_byte(chunk, OpCode::OpReturn);
    }

    pub fn emit_constant(&mut self, chunk: &mut Chunk, value: Value) {
        let constant = self.make_constant(chunk, value);
        self.emit_bytes(chunk, OpCode::OpConstant, constant);
    }

    pub fn end_compiler(&self, chunk: &mut Chunk) {
        self.emit_return(chunk);
    }

    pub fn make_constant(&mut self, chunk: &mut Chunk, value: Value) -> OpCode {
        let constant = chunk.add_constant(value);

        if constant as u8 > u8::MAX {
            self.error(String::from("Too many constants in one chunk"));
            return OpCode::Index(0);
        };

        return OpCode::Index(constant);
    }

    pub fn consume(&mut self, scanner: &mut Scanner, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance(scanner);
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