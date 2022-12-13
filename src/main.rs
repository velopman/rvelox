mod chunk;
mod compiler;
mod debug;
mod object;
mod scanner;
mod value;
mod velox;
mod vm;

fn main() {
    velox::Velox::new().main();
}
