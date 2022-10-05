use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    iter::{FromIterator},
    process
};

mod chunk;
mod debug;
mod scanner;
use scanner::{Scanner, Token, TokenType};
mod value;
mod vm;

fn main() {
    // let vm: vm::VM = vm::VM::new();


    let args: Vec<String> = Vec::from_iter(env::args().skip(1));

    match args.len() {
        0 => repl(),
        1 => run_file(&args[0]),
        _ => {
            eprintln!("Usage: rvelox [path]");
            process::exit(64);
        },
    }

    // vm::VM::interpret(&c);

    // vm.free();
}

fn compile(source: &String) -> () {
    let mut scanner: Scanner = Scanner::new(&source);

    loop {
        let token: Token = scanner.scan_token();

        if token.line != scanner.line {
            print!("{:<15} ", token.line);
            // scanner.line = token.line;
        } else {
            print!("   | ");
        }

        println!("{:0<2} '{}'", token.token_type, &scanner.source.chars().skip(token.start).take(token.length).collect::<String>());

        if token.token_type == TokenType::Eof {
            break;
        }
    }
}

fn interpret(source: &String) -> vm::InterpretResult {
    println!("source: {source}");

    compile(&source);

    return vm::InterpretResult::InterpretOk;
}

fn repl() -> () {
    let mut lines = io::stdin().lines();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match lines.next() {
            Some(line) => {
                interpret(&line.unwrap());
            },
            _ => {
                println!("");
                return
            },
        }
    }
}

fn run_file(path: &String) -> () {
    let mut file = File::open(path)
        .expect("Could not open file \"{path}\".");

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("Could not read file \"{path}\".");

    match interpret(&source) {
        vm::InterpretResult::InterpretCompileError => process::exit(65),
        vm::InterpretResult::InterpretRuntimeError => process::exit(70),
        _ => (),
    }
}
