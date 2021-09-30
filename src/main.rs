mod language;

fn main() {
    let mut chunk: language::chunk::Chunk = language::chunk::Chunk::new();
    let mut constant = chunk.add_constant(1.2);

    chunk.write(language::opcode::OpCode::OpConstant, 1);
    chunk.write(language::opcode::OpCode::Index(constant), 1);

    constant = chunk.add_constant(3.4);

    chunk.write(language::opcode::OpCode::OpConstant, 1);
    chunk.write(language::opcode::OpCode::Index(constant), 1);

    chunk.write(language::opcode::OpCode::OpAdd, 1);

    constant = chunk.add_constant(5.6);

    chunk.write(language::opcode::OpCode::OpConstant, 1);
    chunk.write(language::opcode::OpCode::Index(constant), 1);

    chunk.write(language::opcode::OpCode::OpDivide, 1);

    chunk.write(language::opcode::OpCode::OpNegate, 1);
    chunk.write(language::opcode::OpCode::OpReturn, 1);
    language::debug::Disassemble::disassemble(&chunk,"test chunk");
    let mut vm = language::vm::VM::new();
    vm.interpret(chunk);
}