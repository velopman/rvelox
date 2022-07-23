mod debug;
mod chunk;
mod value;

fn main() {
    let mut c: chunk::Chunk = chunk::Chunk::new();

    let constant: usize = c.add_constant(1.2);
    c.write(chunk::OpCode::OpConstant(constant), 123);

    c.write(chunk::OpCode::OpReturn, 123);

    debug::dissassemble_chunk(&c, "test chunk");

    c.free();
}
