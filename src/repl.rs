use crate::{
    parser::parse,
    term::{Term, eval},
};
use rustyline::{DefaultEditor, Result, error::ReadlineError};
use std::collections::HashMap;

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
            Ok(t) => println!("{}", eval(t)),
            Err(e) => println!("{:?}", e),
        }
    }
}

// Runs the REPL.
pub fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    let mut env = Env::new();
    loop {
        match rl.readline("qlam> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                repl_line(&line, &mut env);
            }
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
