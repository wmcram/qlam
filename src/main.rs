pub mod examples;
pub mod helpers;
pub mod parser;
pub mod repl;
pub mod superpos;
pub mod term;

use crate::repl::repl;

fn main() {
    println!("Welcome to qlam. Type Ctrl-C to exit.");
    repl();
}
