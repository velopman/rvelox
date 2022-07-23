use value::Value;
use value::ValueArray;

#[derive(Debug)]
pub enum OpCode {
    OpConstant(usize),
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: ValueArray,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        return self.constants.count() - 1;
    }

    pub fn free(&mut self) -> () {
        self.code.clear();
        self.constants.free();
        self.lines.clear();
    }

    pub fn write(&mut self, code: OpCode, line: i32) -> () {
        self.code.push(code);
        self.lines.push(line);
    }
}
