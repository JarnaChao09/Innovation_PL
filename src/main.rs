mod language;

use std::env;
use std::io::*;
use std::fs::*;
use std::process::exit;
use text_io::read;

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
    println!("Running REPL");
    loop {
        print!(">> ");
        std::io::stdout().flush().expect("flush failed!");

        let mut line: String = read!("{}\n");

        match line.as_ref() {
            "exit" => {
                println!("Exiting REPL");
                break;
            }
            _ => {
                vm.interpret(&mut line);
            }
        }
    }
}

fn run_file(vm: &mut language::vm::VM, path: &String) {
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

    let result = vm.interpret(&mut source);

    match result {
        language::vm::InterpreterResult::Ok => {}
        language::vm::InterpreterResult::CompileError => { exit(65); }
        language::vm::InterpreterResult::RuntimeError => { exit(70); }
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