use crate::{
    examples::{load_qntmlib, load_stdlib},
    parser::parse,
    term::{Value, eval},
};
use rustyline::{DefaultEditor, Result, error::ReadlineError};
use std::collections::HashMap;

pub struct Repl {
    env: HashMap<String, Value>,
    verbose: bool,
}

impl Repl {
    // Creates a new Repl with a blank env.
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            verbose: false,
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

    fn print_env(&self) {
        for (k, v) in &self.env {
            println!("{}: {}", k, v);
        }
    }

    fn reset_env(&mut self) {
        self.env.clear();
    }

    // Processes a line of input and performs the corresponding effects.
    pub fn read_line(&mut self, line: &str) {
        // Check for keyword commands
        match line {
            "env" => {
                self.print_env();
                return;
            }
            "reset" => {
                self.reset_env();
                return;
            }
            "help" => {
                println!("Reserved identifiers are:");
                println!("|0>, |1>: Qubit basis states");
                println!("H: Hadamard Gate");
                println!("C: CNOT Gate");
                println!("T: T Gate");
                println!("M: Measurement");
                println!("You can create lambdas with syntax like \\x.x");
                println!("You can assign variables like \"NAME = VALUE\"");
            }
            _ => (),
        }

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
    load_qntmlib(&mut repl);

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
