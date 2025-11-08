use crate::{
    examples::load_stdlib,
    parser::parse,
    term::{Value, eval},
};
use rustyline::{DefaultEditor, Result, error::ReadlineError};
use std::collections::HashMap;

pub struct Repl {
    env: HashMap<String, Value>,
}

impl Repl {
    // Creates a new Repl with a blank env.
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    // Puts a symbol in the environment.
    pub fn put_env(&mut self, name: String, value: Value) {
        self.env.insert(name, value);
    }

    // Gets the mapping for a symbol.
    pub fn get_env(&self, name: &str) -> Option<Value> {
        self.env.get(name).cloned()
    }

    // Processes a line of input and performs the corresponding effects.
    pub fn read_line(&mut self, line: &str) {
        if let Some((name, term)) = line.split_once('=') {
            match parse(&mut term.trim().chars()) {
                Ok(t) => match eval(t, &mut self.env) {
                    Ok(v) => self.put_env(name.trim().into(), v),
                    Err(e) => println!("Evaluation Error: {:?}", e),
                },
                Err(e) => println!("Parser Error: {:?}", e),
            }
        } else {
            match parse(&mut line.chars()) {
                Ok(t) => match eval(t, &mut self.env) {
                    Ok(v) => println!("{v}"),
                    Err(e) => println!("Evaluation Error: {:?}", e),
                },
                Err(e) => println!("Parser Error: {:?}", e),
            }
        }
    }
}

// Runs a new REPL until an error is encountered.
pub fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut repl = Repl::new();

    load_stdlib(&mut repl);

    loop {
        match rl.readline("qlam> ") {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line.as_str())?;
                repl.read_line(&line);
            }
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
