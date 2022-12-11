use std::convert::TryInto;

use chunk::{Chunk, Op};

pub static DEBUG_TRACE_EXECUTION: bool = true;
pub static DEBUG_PRINT_CODE: bool = true;

impl Chunk {
    pub fn dissassemble_chunk(&self, name: &str) -> () {
        println!("== {name} ==");

        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = self.dissassemble_instruction(offset);
        }
    }

    pub fn dissassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:>4} ", self.lines[offset]);
        }

        let instruction: u8 = self.code[offset];
        let op_code: Result<Op, ()> = instruction.try_into();
        return match op_code {
            Ok(op_code) => match op_code {
                Op::Constant => self.constant_instruction("OP_CONSTANT", offset),
                Op::Nil => self.simple_instruction("OP_NIL", offset),
                Op::True => self.simple_instruction("OP_TRUE", offset),
                Op::False => self.simple_instruction("OP_FALSE", offset),
                Op::Equal => self.simple_instruction("OP_EQUAL", offset),
                Op::Greater => self.simple_instruction("OP_GREATER", offset),
                Op::Less => self.simple_instruction("OP_LESS", offset),
                Op::Add => self.simple_instruction("OP_ADD", offset),
                Op::Subtract => self.simple_instruction("OP_SUBTRACT", offset),
                Op::Multiply => self.simple_instruction("OP_MULTIPLY", offset),
                Op::Divide => self.simple_instruction("OP_DIVIDE", offset),
                Op::Not => self.simple_instruction("OP_NOT", offset),
                Op::Negate => self.simple_instruction("OP_NEGATE", offset),
                Op::Return => self.simple_instruction("OP_RETURN", offset),
            },
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant: u8 = self.code[offset + 1];

        print!("{name:<16} {constant:>4} '");
        self.constants[constant as usize].print();
        println!("'");

        return offset + 2;
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");

        return offset + 1;
    }
}
