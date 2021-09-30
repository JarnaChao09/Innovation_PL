use super::opcode::*;
use super::chunk::*;

pub trait Disassemble {
    fn disassemble(&self, name: &str);
}

impl Disassemble for Chunk {
    fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0 as usize;

        while offset < self.count() {
            offset = disassemble_instruction(self, offset)
        }
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let line = chunk.get_line(offset);

    if offset > 0 && line == chunk.get_line(offset - 1) {
        print!("   | ");
    } else {
        print!("{:4} ", line);
    }

    let instruction = chunk.code[offset];

    match OpCode::from(instruction) {
        OpCode::OpConstant => {
            let constant = chunk.code[offset + 1];
            println!("{:?} {:?} '{:.3e}'", OpCode::from(chunk.code[offset]), constant, chunk.constants[constant as usize]);
            offset + 2
        },
        OpCode::OpAdd => { simple_instruction(instruction, offset) },
        OpCode::OpSubtract => { simple_instruction(instruction, offset) },
        OpCode::OpMultiply => { simple_instruction(instruction, offset) },
        OpCode::OpDivide => { simple_instruction(instruction, offset) },
        OpCode::OpNegate => { simple_instruction(instruction, offset) },
        OpCode::OpReturn => { simple_instruction(instruction, offset) },
        OpCode::Index(_) => {
            offset + 1
        }
    }
}

fn simple_instruction(instruction: u8, offset: usize) -> usize {
    println!("{:?}", OpCode::from(instruction));
    offset + 1
}