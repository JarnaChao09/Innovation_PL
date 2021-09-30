use super::line_start::*;
use super::opcode::*;
use super::value::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<LineStart>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::with_capacity(0),
            lines: Vec::with_capacity(0),
            constants: Vec::with_capacity(0),
        }
    }

    pub fn count(&self) -> usize {
        self.code.len()
    }

    pub fn capacity(&self) -> usize {
        self.code.capacity()
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line_capacity(&self) -> usize {
        self.lines.capacity()
    }

    pub fn write(&mut self, opcode: OpCode, line: i32) {
        self.code.push(u8::from(opcode));
        let line_count = self.line_count();
        if line_count > 0 && self.lines[line_count - 1].line == line {
            return
        }
        self.lines.push(LineStart {
            offset: self.count() - 1,
            line
        });
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_line(&self, instruction: usize) -> i32 {
        let mut start = 0;
        let mut end = (self.line_count() - 1) as i32;
        loop {
            let mid = (start + end) / 2;
            let line: &LineStart = &self.lines[mid as usize];
            if instruction < line.offset {
                end = mid - 1;
            } else if mid == (self.line_count() - 1) as i32 || instruction < self.lines[(mid + 1) as usize].offset {
                return line.line
            } else {
                start = mid + 1;
            }
        }
    }
}
