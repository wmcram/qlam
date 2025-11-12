use crate::repl::Repl;

// Loads in a "standard library" for classical lambda calculus
pub fn load_stdlib(repl: &mut Repl) {
    repl.read_line("true = \\x.\\y.x");
    repl.read_line("false = \\x.\\y.y");
    repl.read_line("if = \\b.\\x.\\y.b x y");
    repl.read_line("cons = \\x.\\y.\\b.b x y");
    repl.read_line("fst = \\p.p true");
    repl.read_line("snd = \\p.p false");
    repl.read_line("id = \\x.x");
    repl.read_line("zero = \\f.\\x.x");
    repl.read_line("succ = \\n.\\f.\\x.f (n f x)");
    repl.read_line("plus = \\m.\\n.n succ m");
    repl.read_line("fix = \\f.(\\x.f(\\v.x x v)) (\\x.f(\\v.x x v))");
}

// Loads in a library with some basic quantum algorithms
pub fn load_qntmlib(repl: &mut Repl) {
    repl.read_line("bell = C (cons (H |0>) |0>)");
    repl.read_line("2bits = C (cons (H |0>) (H |0>))");
}
