use std::slice;

use chunk::{Chunk, OpCode};
use value::{Value};
use debug::{
    DEBUG_TRACE_EXECUTION,
    dissassemble_instruction,
    print_value,
};

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

const STACK_MAX: usize = 256;

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub ip: slice::Iter<'a, OpCode>,
    pub stack: Vec<Value>,
}

macro_rules! binary_op {
    ($self:ident, $result_type:ident, $op:tt) => {
        {
            let b = $self.pop();
            let a = $self.pop();

            $self.push(a $op b);
        }
    };
}

impl<'a> VM<'a> {
    // pub fn new() -> VM<'a> {
    //     return VM {
    //         chunk: None,
    //         ip: None,
    //     };
    // }

    pub fn free(&mut self) -> () {

    }

    pub fn interpret(chunk: &'a Chunk) -> InterpretResult {
        let mut vm: VM = VM {
            chunk: chunk,
            ip: chunk.code.iter(),
            stack: Vec::new(),
        };

        return vm.run();

        // self.chunk = chunk;
        // self.ip = self.chunk.code.iter();

        // return self.run();
    }

    fn instruction_offset(&self) -> usize {
        self.chunk.code.len() - self.ip.as_slice().len()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("          ");
                for value in self.stack.iter() {
                    print!("[ ");
                    print_value(*value);
                    print!(" ]");
                }
                println!("");

                dissassemble_instruction(self.chunk, self.instruction_offset());
            }

            let instruction: OpCode = self.read_byte();
            match instruction {
                OpCode::OpConstant(idx) => {
                    let constant: Value = self.read_constant(idx);
                    self.push(constant);
                },
                OpCode::OpAdd => binary_op!(self, Number, +),
                OpCode::OpSubtract => binary_op!(self, Number, -),
                OpCode::OpMultiply => binary_op!(self, Number, *),
                OpCode::OpDivide => binary_op!(self, Number, /),
                OpCode::OpNegate => {
                    let value: Value = self.pop();
                    self.push(-value);
                },
                OpCode::OpReturn => {
                    print_value(self.pop());
                    println!("");

                    return InterpretResult::InterpretOk
                },
                // _ => println!("Test"),
            }
        }
    }

    fn read_byte(&mut self) -> OpCode {
        return unsafe { *self.ip.next().unwrap_unchecked() };
    }

    fn read_constant(&self, constant: usize) -> Value {
        return self.chunk.constants.values[constant];
    }

    fn push(&mut self, value: Value) -> () {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        return unsafe { self.stack.pop().unwrap_unchecked() };
    }
}
