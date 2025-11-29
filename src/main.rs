pub mod circuit;
pub mod helpers;
pub mod parser;
pub mod repl;
pub mod superpos;
pub mod term;

use rustyline::Result;

use crate::circuit::parse_circuit;
use crate::repl::repl;
use std::env;
use std::fs;
use std::process::exit;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match (args.next().as_deref(), args.next()) {
        (Some("compile"), Some(path)) => {
            let src = fs::read_to_string(&path).unwrap();
            let circ = parse_circuit(&src).unwrap();
            let term = circ.to_lambda().unwrap();
            println!("{}", term);
            exit(0);
        }

        (Some("compile"), None) => {
            println!("qlam compile must take a filename as an additional argument.");
            exit(1);
        }

        _ => (),
    }

    println!("Welcome to qlam. Type Ctrl-D to exit.");
    repl()
}
