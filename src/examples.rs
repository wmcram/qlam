use crate::helpers::{abs, app, gate, ket, var};
use crate::term::Term;

// Returns a term representing an even superposition of |0> and |1>
pub fn even() -> Term {
    let app_had = abs("x", app(gate("H"), var("x")));
    app(app_had, ket(vec![false]))
}

// Returns the Omega term, which diverges in classical lambda calculus.
pub fn omega() -> Term {
    let inner = abs("x", app(var("x"), var("x")));
    app(inner.clone(), inner)
}
