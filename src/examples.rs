use crate::repl::Repl;

// Loads in a "standard library" for classical lambda calculus
pub fn load_stdlib(repl: &mut Repl) {
    repl.read_line("true = \\x.\\y.x");
    repl.read_line("false = \\x.\\y.y");
    repl.read_line("cons = \\x.\\y.\\b.b x y");
    repl.read_line("fst = \\p.p true");
    repl.read_line("snd = \\p.p false");
}

// Loads in a library with some basic quantum algorithms
pub fn load_qntmlib(repl: &mut Repl) {
    repl.read_line("epr = C (cons (H |0>) |0>)");
}
