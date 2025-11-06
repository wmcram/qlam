use crate::{
    parser::parse,
    term::{Value, eval},
};
use rustyline::{DefaultEditor, Result, error::ReadlineError};
use std::collections::HashMap;

// A symbol -> term mapping for the REPL environment.
pub struct Env(HashMap<String, Value>);

impl Env {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // Puts a symbol in the environment.
    pub fn put(&mut self, name: String, value: Value) {
        self.0.insert(name, value);
    }

    // Gets the mapping for a symbol.
    pub fn get(&self, name: &str) -> Option<Value> {
        self.0.get(name).cloned()
    }
}

// Processes a line of input and performs the corresponding effects.
fn repl_line(line: &str, env: &mut Env) {
    if let Some((name, term)) = line.split_once('=') {
        match parse(&mut term.trim().chars()) {
            Ok(t) => match eval(t, env) {
                Ok(v) => env.put(name.trim().into(), v),
                Err(e) => println!("Evaluation Error: {:?}", e),
            },
            Err(e) => println!("Parser Error: {:?}", e),
        }
    } else {
        match parse(&mut line.chars()) {
            Ok(t) => match eval(t, &env) {
                Ok(v) => println!("{v}"),
                Err(e) => println!("Evaluation Error: {:?}", e),
            },
            Err(e) => println!("Parser Error: {:?}", e),
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
                if line.is_empty() {
                    continue;
                }
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
