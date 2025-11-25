use crate::repl::Repl;

// Loads in a "standard library" for classical lambda calculus
pub fn load_stdlib(repl: &mut Repl) {
    repl.read_line("id = #x. x");
    repl.read_line("pair = \\x.\\y.\\b.b x y");
    repl.read_line("trip = \\x.\\y.\\z.\\f.f x y z");
    repl.read_line("swap = \\p.p (\\x.\\y. pair y x)")
}

// Loads in a library with some basic quantum algorithms
pub fn load_qntmlib(repl: &mut Repl) {
    repl.read_line("deutsch = \\U.(U (pair (H |0>) (H |1>))) (\\x.\\y. pair (H x) y)");
    repl.read_line("epr = C (pair (H |0>) |0>)");
}
