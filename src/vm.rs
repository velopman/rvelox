use std::{
    convert::TryInto,
    slice
};

use chunk::{Chunk, Op};
use compiler::Compiler;
use debug::DEBUG_TRACE_EXECUTION;
use value::Value;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

const STACK_MAX: usize = 256;

pub struct VM {
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        return VM {
            stack: Vec::with_capacity(STACK_MAX),
        };
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(source, &mut chunk);

        if !compiler.compile() {
            return InterpretResult::CompileError;
        }

        return Runner::new(&mut self.stack, &chunk).run();
    }
}

macro_rules! binary_op {
    ($self:ident, $result_type:ident, $op:tt) => {
        {
            let b = $self.peek(0).clone();
            let a = $self.peek(1).clone();

            if let Value::Number(b) = b {
                if let Value::Number(a) = a {
                    $self.pop();
                    $self.pop();

                    $self.push(Value::$result_type(a $op b));
                } else {
                    // $self.runtime_error("Operands must be numbers.");
                }
            } else {
                // $self.runtime_error("Operands must be numbers.");
            }

        }
    };
}

struct Runner<'a> {
    stack: &'a mut Vec<Value>,
    chunk: &'a Chunk,
    ip: slice::Iter<'a, u8>,
}

impl<'a> Runner<'a> {
    pub fn new(stack: &'a mut Vec<Value>, chunk: &'a Chunk) -> Self {
        Self {
            stack: stack,
            chunk: chunk,
            ip: chunk.code.iter(),
        }
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
                    value.print();
                    print!(" ]");
                }
                println!("");

                self.chunk.dissassemble_instruction(self.instruction_offset());
            }

            let instruction: u8 = self.read_byte();
            let op: Op = unsafe { instruction.try_into().unwrap_unchecked() };
            match op {
                Op::Constant => {
                    let constant: Value = self.read_constant();
                    self.push(constant);
                },
                Op::Add => binary_op!(self, Number, +),
                Op::Subtract => binary_op!(self, Number, -),
                Op::Multiply => binary_op!(self, Number, *),
                Op::Divide => binary_op!(self, Number, /),
                Op::Negate => {
                    if let Value::Number(value) = self.peek(0) {
                        *value = -*value;
                    } else {
                        // self.runtime_error("Operand must be a number");
                    }
                },
                Op::Return => {
                    self.pop().print();
                    println!("");

                    return InterpretResult::Ok
                },
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        return unsafe { *self.ip.next().unwrap_unchecked() };
    }

    fn read_constant(&mut self) -> Value {
        return self.chunk.constants[self.read_byte() as usize];
    }

    fn peek(&mut self, index: usize) -> &mut Value {
        unsafe {
            let index = self.stack.len() - 1 - index;
            return self.stack.get_unchecked_mut(index);
        }
    }

    fn push(&mut self, value: Value) -> () {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        return unsafe { self.stack.pop().unwrap_unchecked() };
    }
}
