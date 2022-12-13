use object::ObjRef;

#[derive(Clone, Copy, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(ObjRef<String>),
}

impl Value {
    pub fn print(&self) -> () {
        match self {
            Value::Nil => print!("nil"),
            Value::Bool(value) => print!("{value}"),
            Value::Number(value) => print!("{value}"),
            Value::String(reference) => print!("Some String"), // TODO: Update to support lookups
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
