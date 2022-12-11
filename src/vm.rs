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
            // let b = $self.peek(0).clone();
            // let a = $self.peek(1).clone();

            let (b, a) = ($self.pop(), $self.pop());

            match (&a, &b) {
                (Value::Number(a), Value::Number(b)) => {
                    $self.push(Value::$result_type(a $op b));

                    None
                }
                _ => {
                    $self.push(a);
                    $self.push(b);

                    $self.runtime_error("Operands must be numbers.")
                }
            }

            // if let Value::Number(b) = b {
            //     if let Value::Number(a) = a {
            //         $self.pop();
            //         $self.pop();

            //         $self.push(Value::$result_type(a $op b));

            //         None
            //     } else {
            //         $self.runtime_error("Operands must be numbers.")
            //     }
            // } else {
            //     $self.runtime_error("Operands must be numbers.")
            // }

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
            let result: Option<InterpretResult> = match op {
                Op::Constant => {
                    let constant: Value = self.read_constant();
                    self.push(constant);
                    None
                },
                Op::Nil => {
                    self.push(Value::Nil);
                    None
                }
                Op::True => {
                    self.push(Value::Bool(true));
                    None
                }
                Op::False => {
                    self.push(Value::Bool(false));
                    None
                }
                Op::Equal => {
                    let a: Value = self.pop();
                    let b: Value = self.pop();

                    self.push(Value::Bool(a == b));

                    None
                }
                Op::Greater => binary_op!(self, Bool, >),
                Op::Less => binary_op!(self, Bool, <),
                Op::Add => {
                    let (b, a) = (self.pop(), self.pop());

                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.push(Value::Number(a + b));

                            None
                        },
                        (Value::String(a), Value::String(b)) => {
                            let value: String = format!("{a}{b}");
                            self.push(Value::String(value));

                            None
                        },
                        _ => {
                            self.push(a);
                            self.push(b);

                            self.runtime_error("Operands must be numbers.")
                        }
                    }
                },
                Op::Subtract => binary_op !(self, Number, -),
                Op::Multiply => binary_op!(self, Number, *),
                Op::Divide => binary_op!(self, Number, /),
                Op::Not => {
                    let value = self.peek(0);
                    *value = Value::Bool(value.is_falsy());

                    None
                }
                Op::Negate => {
                    match self.peek(0) {
                        Value::Number(value) => {
                            *value = -*value;
                            None
                        },
                        _ => self.runtime_error("Operand must be a number"),
                    }
                },
                Op::Return => {
                    self.pop().print();
                    println!("");

                    Some(InterpretResult::Ok)
                },
            };

            if let Some(result) = result {
                return result;
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        return unsafe { *self.ip.next().unwrap_unchecked() };
    }

    fn read_constant(&mut self) -> Value {
        return self.chunk.constants[self.read_byte() as usize].clone(); // TODO: Fix this when GC
    }

    fn runtime_error(&mut self, message: &str) -> Option<InterpretResult> {
        eprintln!("{message}");

        let instruction: usize = self.instruction_offset() - 1;
        let line: usize = self.chunk.lines[instruction];

        eprintln!("[line {line}] in script");

        self.stack.clear();

        return Some(InterpretResult::RuntimeError);
    }

    fn peek(&mut self, distance: usize) -> &mut Value {
        unsafe {
            let index: usize = self.stack.len() - 1 - distance;
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
