pub mod examples;
pub mod helpers;
pub mod parser;
pub mod repl;
pub mod superpos;
pub mod term;

use rustyline::Result;

use crate::repl::repl;

fn main() -> Result<()> {
    println!("Welcome to qlam. Type Ctrl-D to exit.");
    println!("Reserved identifiers are:");
    println!("|0>, |1>: Qubit basis states");
    println!("H: Hadamard Gate");
    println!("C: CNOT Gate");
    println!("T: T Gate");
    println!("M: Measurement");
    repl()
}
