pub type Value = f64;

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray {
            values: Vec::new(),
        }
    }

    pub fn count(&mut self) -> usize {
        return self.values.len();
    }

    pub fn free(&mut self) -> () {
        self.values.clear();
    }

    pub fn write(&mut self, value: Value) -> () {
        self.values.push(value);
    }
}
