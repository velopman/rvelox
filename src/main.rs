mod debug;
mod chunk;
mod value;
mod vm;

fn main() {
    // let vm: vm::VM = vm::VM::new();

    let mut c: chunk::Chunk = chunk::Chunk::new();

    let mut constant: usize = c.add_constant(1.2);
    c.write(chunk::OpCode::OpConstant(constant), 123);

    constant = c.add_constant(3.4);
    c.write(chunk::OpCode::OpConstant(constant), 123);

    c.write(chunk::OpCode::OpAdd, 123);

    constant = c.add_constant(5.6);
    c.write(chunk::OpCode::OpConstant(constant), 123);

    c.write(chunk::OpCode::OpDivide, 123);
    c.write(chunk::OpCode::OpNegate, 123);

    c.write(chunk::OpCode::OpReturn, 123);

    vm::VM::interpret(&c);

    // vm.free();
    c.free();
}
