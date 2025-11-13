use crate::repl::Repl;

// Loads in a "standard library" for classical lambda calculus
pub fn load_stdlib(repl: &mut Repl) {
    repl.read_line("id = \\x. x");
}

// Loads in a library with some basic quantum algorithms
pub fn load_qntmlib(repl: &mut Repl) {}
