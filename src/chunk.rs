use std::convert::TryFrom;

use value::Value;

pub enum Op {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

impl Into<u8> for Op {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Op {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == Op::Constant as u8 => Op::Constant,
            x if x == Op::Add as u8 => Op::Add,
            x if x == Op::Subtract as u8 => Op::Subtract,
            x if x == Op::Multiply as u8 => Op::Multiply,
            x if x == Op::Divide as u8 => Op::Divide,
            x if x == Op::Negate as u8 => Op::Negate,
            x if x == Op::Return as u8 => Op::Return,
            _ => return Err(()),
        })
    }
}


pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        let location: usize = self.constants.len();

        self.constants.push(value);

        return location;
    }

    pub fn write(&mut self, code: u8, line: usize) -> () {
        self.code.push(code);
        self.lines.push(line);
    }
}
