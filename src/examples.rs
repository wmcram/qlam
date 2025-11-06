use crate::helpers::{abs, app, gate, ket, var};
use crate::repl::{self, Env, repl_line};
use crate::term::Term;

// Loads in a "standard library" for classical lambda calculus
pub fn load_stdlib(env: &mut Env) {
    repl_line("true = \\x.\\y.x", env);
    repl_line("false = \\x.\\y.y", env);
    repl_line("cons = \\x.\\y.\\b.b x y", env);
    repl_line("fst = \\p.p true", env);
    repl_line("snd = \\p.p false", env);
}

// Loads in a library with some basic quantum algorithms
pub fn load_qntmlib(env: &mut Env) {
    repl_line("epr = C (cons (H |0>) |0>)", env);
}
