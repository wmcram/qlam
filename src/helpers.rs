use crate::term::{Const, Term};

// Convenience function for constructing variable terms.
pub fn var(name: &str) -> Term {
    Term::Var(name.to_string())
}

// Convenience function for constructing lambda terms.
pub fn abs(p: &str, body: Term) -> Term {
    Term::Abs(p.to_string(), Box::new(body))
}

// Convenience function for constructing application terms.
pub fn app(a: Term, b: Term) -> Term {
    Term::App(Box::new(a), Box::new(b))
}

// Convenience function for constructing gates.
pub fn gate(g: &str) -> Term {
    Term::Const(Const::Gate(g.to_string()))
}

// Convenience function for constructing kets.
pub fn ket(k: Vec<bool>) -> Term {
    Term::Const(Const::Ket(k))
}

// Convenience function for constructing bits.
pub fn bit(b: bool) -> Term {
    Term::Const(Const::Bit(b))
}
