mod language;

use std::env;
use std::io::*;
use std::fs::*;
use std::process::exit;
use crate::language::vm::InterpreterResult;

fn main() {
    let mut vm = language::vm::VM::new();

    let argc = env::args().len();

    match argc {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &env::args().nth(1).unwrap()),
        _ => {
            eprintln!("Usage: to access repl call cargo run, to run file call cargo run [path]");
            exit(64);
        }
    }
}

fn repl(vm: &mut language::vm::VM) {
    println!("running repl");
    return
    // let mut line = String::with_capacity(1024);
    //
    // loop {
    //     print!("(> ");
    //
    //     match stdin().read_line(&mut line) {
    //         Ok(_) => {
    //             vm.interpret(&line);
    //         }
    //         Err(_) => {
    //             println!();
    //             break;
    //         }
    //     }
    // }
}

fn run_file(vm: &mut language::vm::VM, path: &str) {
    let file = File::open(path);

    let mut source = String::new();

    match file {
        Ok(mut f) => {
            let len = f.read_to_string(&mut source).unwrap();

            if len < f.metadata().unwrap().len() as usize {
                eprintln!("Could not read file \"{}\"", path);
                exit(74);
            }
        }
        Err(_) => {
            eprintln!("Could not open file \"{}\"", path);
            exit(74);
        }
    };

    let result = vm.interpret(&source);

    match result {
        InterpreterResult::Ok => {}
        InterpreterResult::CompileError => { exit(65); }
        InterpreterResult::RuntimeError => { exit(70); }
    }
}

// fn main() {
//     let mut chunk: language::chunk::Chunk = language::chunk::Chunk::new();
//     let mut constant = chunk.add_constant(1.2);
//
//     chunk.write(language::opcode::OpCode::OpConstant, 1);
//     chunk.write(language::opcode::OpCode::Index(constant), 1);
//
//     constant = chunk.add_constant(3.4);
//
//     chunk.write(language::opcode::OpCode::OpConstant, 1);
//     chunk.write(language::opcode::OpCode::Index(constant), 1);
//
//     chunk.write(language::opcode::OpCode::OpAdd, 1);
//
//     constant = chunk.add_constant(5.6);
//
//     chunk.write(language::opcode::OpCode::OpConstant, 1);
//     chunk.write(language::opcode::OpCode::Index(constant), 1);
//
//     chunk.write(language::opcode::OpCode::OpDivide, 1);
//
//     chunk.write(language::opcode::OpCode::OpNegate, 1);
//     chunk.write(language::opcode::OpCode::OpReturn, 1);
//     language::debug::Disassemble::disassemble(&chunk,"test chunk");
//     let mut vm = language::vm::VM::new();
//     vm.interpret(chunk);
// }