#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum OpCode {
    OpConstant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
    Index(usize),
}

impl From<OpCode> for u8 {
    fn from(opcode: OpCode) -> Self {
        (match opcode {
            OpCode::OpConstant => { 1 }
            OpCode::OpAdd => { 2 }
            OpCode::OpSubtract => { 3 }
            OpCode::OpMultiply => { 4 }
            OpCode::OpDivide => { 5 }
            OpCode::OpNegate => { 6 }
            OpCode::OpReturn => { 7 }
            OpCode::Index(index) => { index }
        }) as u8
    }
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            1 => OpCode::OpConstant,
            2 => OpCode::OpAdd,
            3 => OpCode::OpSubtract,
            4 => OpCode::OpMultiply,
            5 => OpCode::OpDivide,
            6 => OpCode::OpNegate,
            7 => OpCode::OpReturn,
            _ => OpCode::Index(byte as usize),
        }
    }
}