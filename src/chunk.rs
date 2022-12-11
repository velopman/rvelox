use std::convert::TryFrom;

use value::Value;

pub enum Op {
    Constant,
    Nil,
    True,
    False,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
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
            x if x == Op::Nil as u8 => Op::Nil,
            x if x == Op::True as u8 => Op::True,
            x if x == Op::False as u8 => Op::False,
            x if x == Op::Equal as u8 => Op::Equal,
            x if x == Op::Greater as u8 => Op::Greater,
            x if x == Op::Less as u8 => Op::Less,
            x if x == Op::Add as u8 => Op::Add,
            x if x == Op::Subtract as u8 => Op::Subtract,
            x if x == Op::Multiply as u8 => Op::Multiply,
            x if x == Op::Divide as u8 => Op::Divide,
            x if x == Op::Not as u8 => Op::Not,
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
