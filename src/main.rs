mod ast;
mod generator;
mod parser;
mod source;
mod tokenizer;

use generator::emit_asm_in_order;
use std::fs::File;
use std::{env, io::Write};
use tokenizer::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("file {}", &args[1]);

    let source_code = source::Source::new(&args[1]);
    let tokens = source_code.file_contents.parse::<Tokens>().unwrap();
    println!("parse: {:?}", tokens);
    let ast = match parser::parse_program(&tokens) {
        Ok(ast) => ast,
        Err(e) => {
            panic!("Parsing error: {}", e);
        }
    };
    println!("ast: {:?}", ast);
    let asm = emit_asm_in_order(&ast);
    let mut output_file = File::create("return_2.s").expect("Unable to create file");
    output_file
        .write_all(asm.as_bytes())
        .expect("Unable to write data");
}
