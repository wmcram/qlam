use crate::{
    helpers::{abs, app, nonlinear, nonlinear_abs},
    parser::parse,
    term::{Term, eval},
};
use rustyline::{DefaultEditor, Result, error::ReadlineError};
use std::{collections::HashMap, fs::File, process::exit};
use std::{io::Read, path::Path};

pub struct Repl {
    env: HashMap<String, Term>,
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

impl Repl {
    // Creates a new Repl with an empty environment.
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    // Puts a symbol in the environment.
    pub fn put_env(&mut self, name: String, term: Term) {
        self.env.insert(name, term);
    }

    // Gets the mapping for a symbol.
    pub fn get_env(&self, name: &str) -> Option<Term> {
        self.env.get(name).cloned()
    }

    // Prints the (name,term) pairs in the environment to console.
    fn print_env(&self) {
        for (k, v) in &self.env {
            println!("{}: {}", k, v);
        }
    }

    // Resets the environment back to empty.
    fn reset_env(&mut self) {
        self.env.clear();
    }

    // Processes a line of input and performs the corresponding effects.
    pub fn read_line(&mut self, line: &str) {
        // Check for keyword commands
        match line {
            "quit" => {
                exit(0);
            }
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
                Ok(t) => {
                    let t = populate_term(t, &self.env);
                    self.put_env(name.trim().into(), t);
                }
                Err(e) => println!("Parser Error: {:?}", e),
            }
        } else {
            match parse(&mut line.chars()) {
                Ok(t) => {
                    let t = populate_term(t, &self.env);
                    match eval(t) {
                        Ok(v) => println!("{v}"),
                        Err(e) => println!("Evaluation Error: {:?}", e),
                    }
                }
                Err(e) => println!("Parser Error: {:?}", e),
            }
        }
    }
}

// Replaces symbols in this term with their corresponding term in the environment.
pub fn populate_term(t: Term, env: &HashMap<String, Term>) -> Term {
    match t {
        Term::Const(_) => t,
        Term::Var(ref x) => match env.get(x) {
            Some(t2) => t2.clone(),
            None => t,
        },
        Term::Abs(x, body) => abs(&x, populate_term(*body, env)),
        Term::NonlinearAbs(x, body) => nonlinear_abs(&x, populate_term(*body, env)),
        Term::Nonlinear(t2) => nonlinear(populate_term(*t2, env)),
        Term::App(t1, t2) => app(populate_term(*t1, env), populate_term(*t2, env)),
    }
}

// Loads the contents of the specified file into the environment line-by-line.
fn load_file(path: &Path, repl: &mut Repl) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        repl.read_line(line);
    }

    Ok(())
}

// Runs a new REPL until an error is encountered.
pub fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut repl = Repl::new();

    match load_file(Path::new("stdlib.conf"), &mut repl) {
        Ok(_) => (),
        Err(e) => println!("Failed to open stdlib.conf: {e}"),
    }

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
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
