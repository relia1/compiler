mod tokenizer;
mod source;
mod parser;
mod ast;
mod generator;

use std::{env, io::Write};
use generator::emit_asm_in_order;
use tokenizer::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("file {}", &args[1]);

    let source_code = source::Source::new(&args[1]);
    let tokens = source_code.file_contents.parse::<Tokens>().unwrap();
    println!("parse: {:?}", tokens);
    let ast = parser::parse_program(&tokens);
    println!("ast: {:?}", ast);
    let asm = emit_asm_in_order(&ast);
    let mut output_file = File::create("return_2.s").expect("Unable to create file");
    output_file.write_all(asm.as_bytes()).expect("Unable to write data");
    //let tokens: Tokens = tokenizer::Tokens::new(&source_code);
    //println!("tokens: {:?}", tokens);
}
