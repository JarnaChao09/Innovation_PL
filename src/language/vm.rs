use std::fmt::{Debug, Formatter};
use super::chunk::*;
use super::opcode::*;
use super::value::*;
use super::debug::*;
use super::scanner::*;
use super::token::*;
use super::parser::*;
use rawpointer::PointerExt;

const STACK_LIMIT: usize = 256;

#[derive(Clone, PartialEq)]
pub struct VM {
    pub chunk: Chunk,
    pub ip: *mut u8,
    pub stack: [Value; STACK_LIMIT],
    pub stack_top: *mut Value,
}

pub enum InterpreterResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> VM {
        let mut vm = VM {
            chunk: Chunk {
                code: Vec::with_capacity(0),
                lines: Vec::with_capacity(0),
                constants: Vec::with_capacity(0),
            },
            ip: 0 as *mut u8,
            stack: [0.0 as Value; STACK_LIMIT],
            stack_top: 0 as *const Value as *mut Value,
        };
        vm.reset_stack();
        vm
    }

    pub fn interpret(&mut self, source: String) -> InterpreterResult {
        let mut chunk = Chunk::new();
        self.compile(source, &mut chunk);
        // self.compile(source);
        InterpreterResult::Ok

        // self.chunk = chunk;
        // self.ip = self.chunk.code.as_mut_ptr();
        //
        // self.run()
    }

    pub fn compile(&mut self, source: String, chunk: &mut Chunk) -> bool {
        let mut scanner = Scanner::new(source);
        let mut parser = Parser::new();

        parser.advance(&mut scanner);
        parser.expression(&mut scanner, chunk);
        parser.consume(&mut scanner, TokenType::EOF, String::from("Expect end of expression."));

        parser.end_compiler(chunk);
        !parser.had_error
    }

    // pub fn compile(&mut self, source: String) {
    //     let mut scanner = Scanner::new(source);
    //     let mut line = -1;
    //     loop {
    //         let token: Token = scanner.scan_token();
    //         if token.line != line {
    //             print!("{:4} ", token.line);
    //             line = token.line;
    //         } else {
    //             print!("   | ");
    //         }
    //         println!("{:?} '{}'", token.token_type, token.lexme);
    //
    //         if token.token_type == TokenType::EOF {
    //             break;
    //         }
    //     }
    // }

    // pub fn interpret(&mut self, chunk: Chunk) -> InterpreterResult {
    //     self.chunk = chunk;
    //     self.ip = self.chunk.code.as_mut_ptr();
    //     self.run()
    // }

    pub fn reset_stack(&mut self) {
        self.stack_top = self.stack.as_mut_ptr();
    }

    pub fn run(&mut self) -> InterpreterResult {
        loop {
            unsafe {
                let instruction = OpCode::from(*self.ip.post_inc());
                match instruction {
                    OpCode::OpConstant => {
                        let constant = self.chunk.constants[*self.ip.post_inc() as usize];
                        self.push(constant);
                    }
                    OpCode::OpAdd => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a + b);
                    }
                    OpCode::OpSubtract => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a - b);
                    }
                    OpCode::OpMultiply => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a * b);
                    }
                    OpCode::OpDivide => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a / b);
                    }
                    OpCode::OpNegate => {
                        *self.stack_top.sub(1) = -*self.stack_top.sub(1);
                    }
                    OpCode::OpReturn => {
                        println!("{:.3}", self.pop());
                        return InterpreterResult::Ok
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn push(&mut self, value: Value) {
        unsafe {
            *self.stack_top.post_inc() = value;
        }
    }

    pub fn pop(&mut self) -> Value {
        unsafe {
            *self.stack_top.pre_dec()
        }
    }

    pub fn disassemble_current_instruction(self) {
        self.chunk.disassemble("VM DEBUG")
    }
}

impl Debug for VM {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.stack.iter()).finish()
    }
}