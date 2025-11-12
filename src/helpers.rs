use num::Complex;

use crate::{
    superpos::Superpos,
    term::{Const, Term, Value},
};

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

// Convenience function for constructing superpositions.
pub fn superpos(v: Vec<(Term, Complex<f64>)>) -> Value {
    Value::Superpos(Superpos(v))
}

// Convenience function for constructing kets.
pub fn ket(k: bool) -> Term {
    Term::Const(Const::Ket(k))
}

// Convenience function for constructing measurements.
pub fn meas() -> Term {
    Term::Const(Const::Meas)
}

pub fn pair(t1: Term, t2: Term) -> Term {
    abs("b", app(app(var("b"), t1), t2))
}
