use crate::{parser::parse, term::Term};
use std::{collections::HashMap, io::Write};

// A symbol -> term mapping for the REPL environment.
pub struct Env(HashMap<String, Term>);

impl Env {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // Puts a symbol in the environment.
    pub fn put(&mut self, name: String, term: Term) {
        self.0.insert(name, term);
    }

    // Gets the mapping for a symbol.
    pub fn get(&self, name: String) -> Option<Term> {
        self.0.get(&name).cloned()
    }
}

// Processes a line of input and performs the corresponding effects.
fn repl_line(line: &str, env: &mut Env) {
    if let Some((name, term)) = line.split_once('=') {
        match parse(&mut term.chars()) {
            Ok(t) => env.put(name.into(), t),
            Err(e) => println!("{:?}", e),
        }
    } else {
        match parse(&mut line.chars()) {
            Ok(t) => println!("{t}"),
            Err(e) => println!("{:?}", e),
        }
    }
}

// Prints the terminal prompt for the REPL.
fn print_prompt() {
    print!("qlam> ");
    std::io::stdout().flush().unwrap();
}

// Runs the REPL.
pub fn repl() {
    let mut env = Env::new();
    let mut buf = String::new();
    print_prompt();
    let stdin = std::io::stdin();
    while let Ok(_) = stdin.read_line(&mut buf) {
        repl_line(&mut buf, &mut env);
        buf = String::new();
        print_prompt();
    }
}
