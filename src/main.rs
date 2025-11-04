pub mod examples;
pub mod helpers;
pub mod parser;
pub mod superpos;
pub mod term;

use std::io::Write;

use crate::parser::{parse, tokenize};

fn main() {
    let mut buf = String::new();
    print_prompt();
    let stdin = std::io::stdin();
    while let Ok(_) = stdin.read_line(&mut buf) {
        let tokens = tokenize(&mut buf.chars());
        match parse(&tokens) {
            Ok(term) => println!("{term}"),
            Err(e) => println!("{:?}", e),
        }
        buf = String::new();
        print_prompt();
    }
}

fn print_prompt() {
    print!("qlam> ");
    std::io::stdout().flush().unwrap();
}
