use chunk::Chunk;
use chunk::OpCode;
use value::Value;

pub fn dissassemble_chunk(chunk: &Chunk, name: &str) -> () {
    println!("== {name} ==");

    let mut offset: usize = 0;
    while offset < chunk.code.len() {
        offset = dissassemble_instruction(chunk, offset);
    }
}

fn dissassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{offset:0>4} ");

    let line: i32 = chunk.lines[offset];
    if offset > 0 && line == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{line:>4} ");
    }

    let instruction: &OpCode = &chunk.code[offset];
    return match instruction {
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant(constant) => constant_instruction("OP_CONSTANT", chunk, *constant, offset),
        // _ => {
        //     println!("Unknown opcode {:?}", *instruction);

        //     offset + 1
        // }
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, constant: usize, offset: usize) -> usize {
    print!("{name:<16} {constant:>4} '");

    print_value(chunk.constants.values[constant]);

    println!("'");

    return offset + 1;
}

fn print_value(value: Value) -> () {
    print!("{value}");
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");

    return offset + 1;
}