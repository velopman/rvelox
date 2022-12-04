use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    iter::FromIterator,
    process::exit,
};

use vm::{InterpretResult, VM};

pub struct Velox {
    vm: VM,
}

impl Velox {
    pub fn new() -> Velox {
        return Velox {
            vm: VM::new(),
        };
    }

    pub fn main(&mut self) -> () {
        let args: Vec<String> = Vec::from_iter(env::args().skip(1));

        match args.len() {
            0 => self.run_prompt(),
            1 => self.run_file(&args[0]),
            _ => {
                eprintln!("Usage: rvelox [path]");
                exit(64);
            },
        }
    }

    fn interpret(&mut self, source: &str) -> InterpretResult {
        return self.vm.interpret(source);
    }

    fn run_file(&mut self, path: &String) -> () {
        let mut file = File::open(path)
            .expect("Could not open file \"{path}\".");

        let mut source = String::new();
        file.read_to_string(&mut source)
            .expect("Could not read file \"{path}\".");

        match self.interpret(&source) {
            InterpretResult::Ok => (),
            InterpretResult::CompileError => exit(65),
            InterpretResult::RuntimeError => exit(70),
        }
    }

    fn run_prompt(&mut self) -> () {
        let mut lines = io::stdin().lines();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            match lines.next() {
                Some(line) => {
                    self.interpret(&line.unwrap());
                },
                _ => {
                    return
                },
            }
        }
    }
}
