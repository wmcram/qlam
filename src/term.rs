use crate::superpos::Superpos;
use num::Complex;

#[derive(Clone, Debug)]
pub enum Value {
    Term(Term),
    Superpos(Superpos),
}

// The possible syntactic forms for terms in the AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Var(String),
    Const(Const),
    Abs(String, Box<Term>),
    NonlinearAbs(String, Box<Term>),
    Bang(Box<Term>),
    App(Box<Term>, Box<Term>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Const {
    Ket(Vec<bool>),
    Gate(String),
}
