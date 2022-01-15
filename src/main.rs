use std::fs;
mod parser;

fn main() {
    let content = fs::read_to_string("/Users/ivanmolinarebolledo/rust_brainfuck_jit/file.bs");
    println!("ss");
    let res = parser::parse(content.unwrap());
    println!("Hello, world! {:?}", res);
}
