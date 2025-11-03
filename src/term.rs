use crate::state::State;
use num::Complex;

// The possible syntactic forms for terms in the AST.
#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
    Split(State),
}
