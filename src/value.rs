#[derive(Clone, Copy)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
}

impl Value {
    pub fn print(&self) -> () {
        match self {
            Value::Nil => print!("nil"),
            Value::Bool(value) => print!("{value}"),
            Value::Number(value) => print!("{value}"),
        }
    }
}
