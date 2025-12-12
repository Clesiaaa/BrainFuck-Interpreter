mod parser;
mod tokenizer;
mod virtual_machine;

use crate::tokenizer::Token;
use std::fs;
use std::env;


fn read_bf(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Can't read file.")
        .chars()
        .filter(|c| matches!(c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: brainfuck <fichier.bf>");
        std::process::exit(1);
    }

    let filename = &args[1];

    let source = read_bf(&filename);

    let tokens = Token::tokenize(&source);
    let mut vm = virtual_machine::VirtualMachine::new(256);

    vm.run(tokens);
}
