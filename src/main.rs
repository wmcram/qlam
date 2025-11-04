pub mod examples;
pub mod helpers;
pub mod superpos;
pub mod term;

use crate::examples::even;
use crate::term::{Value, eval};

fn main() {
    let norm = eval(even());
    match norm {
        Value::Term(t) => println!("{t}"),
        Value::Superpos(s) => println!("{}", s.measure()),
    }
}
