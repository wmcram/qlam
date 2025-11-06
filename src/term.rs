use num::Complex;

use crate::superpos::Superpos;
use std::{collections::HashSet, f64::consts::PI, fmt::Display, iter::empty};

#[derive(Clone, Debug)]
pub enum Value {
    Term(Term),
    Superpos(Superpos),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Term(t) => t.fmt(f),
            Value::Superpos(s) => s.fmt(f),
        }
    }
}

// The possible syntactic forms for terms in the AST.
// TODO: Add pairs?
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Var(String),
    Const(Const),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(x) => write!(f, "{x}"),
            Term::Const(Const::Gate(g)) => write!(f, "{g}"),
            Term::Const(Const::Ket(k)) => {
                write!(f, "|")?;
                for q in k {
                    if *q {
                        write!(f, "1")?;
                    } else {
                        write!(f, "0")?;
                    }
                }
                write!(f, ">")
            }
            Term::Const(Const::Bit(b)) => {
                if *b {
                    write!(f, "1")
                } else {
                    write!(f, "0")
                }
            }
            Term::Abs(x, body) => write!(f, "(λ{x}. {body})"),
            Term::App(a, b) => write!(f, "({a} {b})"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Const {
    Ket(Vec<bool>),
    Bit(bool),
    Gate(String),
}

// Gets the free variables of a lambda term.
fn free_vars(t: &Term) -> HashSet<String> {
    match t {
        Term::Var(x) => [x.clone()].into_iter().collect(),
        Term::Const(_) => empty().collect(),
        Term::Abs(x, body) => &free_vars(body) - &[x.clone()].into_iter().collect(),
        Term::App(a, b) => &free_vars(a) | &free_vars(b),
    }
}

// Finds a fresh variable using the given base string and bound variables.
fn fresh_var(base: &str, in_use: &HashSet<String>) -> String {
    let mut n = 0;
    let mut name = base.to_string();
    while in_use.contains(&name) {
        n += 1;
        name = format!("{base}{n}");
    }
    name
}

// Safely substitues t[x -> s].
fn subst(t: &Term, x: &str, s: &Term) -> Term {
    match t {
        Term::Var(y) => {
            if x == y {
                s.clone()
            } else {
                t.clone()
            }
        }
        Term::Const(_) => t.clone(),
        Term::Abs(y, body) => {
            // shadowed case
            if y == x {
                t.clone()
            } else if free_vars(s).contains(y) {
                let used: HashSet<_> = free_vars(body).union(&free_vars(s)).cloned().collect();
                let fresh = fresh_var(y, &used);
                let renamed_body = subst(body, y, &Term::Var(fresh.clone()));
                Term::Abs(fresh, Box::new(subst(&renamed_body, x, s)))
            } else {
                Term::Abs(y.clone(), Box::new(subst(body, x, s)))
            }
        }
        Term::App(a, b) => Term::App(Box::new(subst(a, x, s)), Box::new(subst(b, x, s))),
    }
}

// Performs a classical beta reduction of two terms
fn beta_reduce(t1: Term, t2: Term) -> Term {
    match t1 {
        Term::Abs(x, body) => subst(&body, &x, &t2),
        _ => panic!("LHS of beta reduction wasn't a lambda abstraction"),
    }
}

// Applies the given quantum gate to the ket
fn apply_gate(g: &str, k: &Vec<bool>) -> Value {
    match g {
        "H" => {
            assert!(k.len() == 1);
            let s: f64 = f64::sqrt(0.5);
            match k[0] {
                false => Value::Superpos(Superpos(vec![
                    (Term::Const(Const::Ket(vec![false])), Complex::new(s, 0.0)),
                    (Term::Const(Const::Ket(vec![true])), Complex::new(s, 0.0)),
                ])),
                true => Value::Superpos(Superpos(vec![
                    (Term::Const(Const::Ket(vec![false])), Complex::new(s, 0.0)),
                    (Term::Const(Const::Ket(vec![true])), Complex::new(-s, 0.0)),
                ])),
            }
        }
        "C" => {
            assert!(k.len() == 2);
            match (k[0], k[1]) {
                (false, false) => Value::Superpos(Superpos(vec![(
                    Term::Const(Const::Ket(vec![false, false])),
                    Complex::new(1.0, 0.0),
                )])),
                (false, true) => Value::Superpos(Superpos(vec![(
                    Term::Const(Const::Ket(vec![false, true])),
                    Complex::new(1.0, 0.0),
                )])),
                (true, false) => Value::Superpos(Superpos(vec![(
                    Term::Const(Const::Ket(vec![true, true])),
                    Complex::new(1.0, 0.0),
                )])),
                (true, true) => Value::Superpos(Superpos(vec![(
                    Term::Const(Const::Ket(vec![true, false])),
                    Complex::new(1.0, 0.0),
                )])),
            }
        }
        "T" => {
            assert!(k.len() == 1);
            let phase = Complex::new(0.0, PI / 4.0).exp();
            match k[0] {
                false => Value::Superpos(Superpos(vec![(
                    Term::Const(Const::Ket(vec![false])),
                    Complex::new(1.0, 0.0),
                )])),
                true => Value::Superpos(Superpos(vec![(
                    Term::Const(Const::Ket(vec![false])),
                    phase,
                )])),
            }
        }
        _ => panic!("Undefined gate"),
    }
}

fn apply(v1: Value, v2: Value) -> Value {
    let res = match (v1, v2) {
        (Value::Term(Term::Const(Const::Gate(g))), Value::Term(Term::Const(Const::Ket(k)))) => {
            apply_gate(&g, &k)
        }
        (Value::Term(t1), Value::Term(t2)) => Value::Term(beta_reduce(t1, t2)),
        (Value::Term(t), Value::Superpos(s)) => {
            Value::Superpos(s.map_terms(|t2| apply(Value::Term(t.clone()), Value::Term(t2))))
        }
        (Value::Superpos(s), Value::Term(t)) => {
            Value::Superpos(s.map_terms(|t2| apply(Value::Term(t2), Value::Term(t.clone()))))
        }
        (Value::Superpos(s1), Value::Superpos(s2)) => {
            Value::Superpos(s1.zip_terms(&s2, |t1, t2| apply(Value::Term(t1), Value::Term(t2))))
        }
    };
    match res {
        Value::Term(t) => eval(t),
        Value::Superpos(mut s) => {
            s.merge();
            Value::Superpos(s)
        }
    }
}

pub fn eval(term: Term) -> Value {
    match term {
        Term::Const(_) | Term::Var(_) | Term::Abs(_, _) => Value::Term(term),
        Term::App(t1, t2) => {
            let v1 = eval(*t1);
            let v2 = eval(*t2);
            apply(v1, v2)
        }
    }
}
