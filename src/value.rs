#[derive(Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Value {
    pub fn print(&self) -> () {
        match self {
            Value::Nil => print!("nil"),
            Value::Bool(value) => print!("{value}"),
            Value::Number(value) => print!("{value}"),
            Value::String(value) => print!("{value}"),
        }
    }

    pub fn is_falsy(&self) -> bool {
        match self {
            Value::Bool(value) => !value,
            Value::Nil => true,
            _ => false,
        }
    }
}
