use num::Complex;

use crate::{
    helpers::{ket, pair},
    superpos::Superpos,
};
use std::{
    collections::{HashMap, HashSet},
    f64::consts::PI,
    fmt::Display,
    iter::empty,
};

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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Var(String),
    Const(Const),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    fn as_var(&self) -> Option<&str> {
        if let Term::Var(x) = self {
            Some(x)
        } else {
            None
        }
    }

    fn as_app(&self) -> Option<(&Term, &Term)> {
        if let Term::App(l, r) = self {
            Some((&*l, &*r))
        } else {
            None
        }
    }

    fn as_abs(&self) -> Option<(&str, &Term)> {
        if let Term::Abs(x, body) = self {
            Some((x.as_str(), &*body))
        } else {
            None
        }
    }

    fn as_pair(&self) -> Option<(&Term, &Term)> {
        let (x, body) = self.as_abs()?;
        let (left, b) = body.as_app()?;
        let (right, a) = left.as_app()?;

        if let Some(v) = right.as_var() {
            if v == x {
                return Some((a, b));
            }
        }
        None
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(x) => write!(f, "{x}"),
            Term::Const(Const::Gate(g)) => write!(f, "{g}"),
            Term::Const(Const::Ket(b)) => {
                write!(f, "|")?;
                if *b {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
                write!(f, ">")
            }
            Term::Const(Const::Meas) => {
                write!(f, "M")
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
    Ket(bool),
    Bit(bool),
    Gate(String),
    Meas,
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

// Determines the number of occurences of variable x in this term.
fn num_occurrences(x: &str, t: &Term) -> u32 {
    match t {
        Term::Var(y) => {
            if x == y {
                1
            } else {
                0
            }
        }
        Term::Const(_) => 0,
        Term::Abs(y, body) => {
            if x == y {
                0
            } else {
                num_occurrences(x, body)
            }
        }
        Term::App(t1, t2) => num_occurrences(x, t1) + num_occurrences(x, t2),
    }
}

// Safely substitues t[x -> s].
fn subst(t: &Term, x: &str, s: &Term) -> Result<Term, EvalError> {
    // Check for ket substitution, in this case we need to ensure linearity holds
    match s {
        Term::Const(Const::Ket(_)) => {
            if num_occurrences(x, t) != 1 {
                return Err(EvalError::LinearityViolation(format!(
                    "substitution of `{}` in `{}`",
                    x, t
                )));
            }
        }
        _ => (),
    }

    // The recursive procedure for substitution.
    fn subst_helper(t: &Term, x: &str, s: &Term) -> Term {
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
                    let renamed_body = subst_helper(body, y, &Term::Var(fresh.clone()));
                    Term::Abs(fresh, Box::new(subst_helper(&renamed_body, x, s)))
                } else {
                    Term::Abs(y.clone(), Box::new(subst_helper(body, x, s)))
                }
            }
            Term::App(a, b) => Term::App(
                Box::new(subst_helper(a, x, s)),
                Box::new(subst_helper(b, x, s)),
            ),
        }
    }

    Ok(subst_helper(t, x, s))
}

// Performs a classical beta reduction of two terms
fn beta_reduce(t1: Term, t2: Term) -> Result<Term, EvalError> {
    match t1 {
        Term::Abs(x, body) => Ok(subst(&body, &x, &t2)?),
        _ => Err(EvalError::BadApplication(
            "LHS of beta reduction wasn't a lambda abstraction".into(),
        )),
    }
}

#[derive(Debug, Clone)]
pub enum EvalError {
    BadApplication(String),
    BadGate(String),
    UndefinedSymbol(String),
    LinearityViolation(String),
}

// Applies the given quantum gate to the ket
fn apply_gate(g: &str, t: &Term) -> Result<Value, EvalError> {
    match g {
        "H" => {
            if let Term::Const(Const::Ket(b)) = t {
                let s: f64 = f64::sqrt(0.5);
                return match b {
                    false => Ok(Value::Superpos(Superpos(vec![
                        (Term::Const(Const::Ket(false)), Complex::new(s, 0.0)),
                        (Term::Const(Const::Ket(true)), Complex::new(s, 0.0)),
                    ]))),
                    true => Ok(Value::Superpos(Superpos(vec![
                        (Term::Const(Const::Ket(false)), Complex::new(s, 0.0)),
                        (Term::Const(Const::Ket(true)), Complex::new(-s, 0.0)),
                    ]))),
                };
            } else {
                return Err(EvalError::BadGate("Hadamard gate must take 1 qubit".into()));
            }
        }
        "C" => {
            if let Some((a, b)) = t.as_pair() {
                let Term::Const(Const::Ket(b1)) = a else {
                    return Err(EvalError::BadGate("CNOT must take a pair of qubits".into()));
                };
                let Term::Const(Const::Ket(b2)) = b else {
                    return Err(EvalError::BadGate("CNOT must take a pair of qubits".into()));
                };
                return match (b1, b2) {
                    (false, false) => Ok(Value::Superpos(Superpos(vec![(
                        pair(ket(false), ket(false)),
                        Complex::new(1.0, 0.0),
                    )]))),
                    (false, true) => Ok(Value::Superpos(Superpos(vec![(
                        pair(ket(false), ket(true)),
                        Complex::new(1.0, 0.0),
                    )]))),
                    (true, false) => Ok(Value::Superpos(Superpos(vec![(
                        pair(ket(true), ket(true)),
                        Complex::new(1.0, 0.0),
                    )]))),
                    (true, true) => Ok(Value::Superpos(Superpos(vec![(
                        pair(ket(true), ket(false)),
                        Complex::new(1.0, 0.0),
                    )]))),
                };
            } else {
                return Err(EvalError::BadGate("CNOT must take a pair of qubits".into()));
            }
        }
        "T" => {
            if let Term::Const(Const::Ket(b)) = t {
                let phase = Complex::new(0.0, PI / 4.0).exp();
                match b {
                    false => Ok(Value::Superpos(Superpos(vec![(
                        Term::Const(Const::Ket(false)),
                        Complex::new(1.0, 0.0),
                    )]))),
                    true => Ok(Value::Superpos(Superpos(vec![(
                        Term::Const(Const::Ket(true)),
                        phase,
                    )]))),
                }
            } else {
                return Err(EvalError::BadGate("T gate must take 1 qubit".into()));
            }
        }
        _ => Err(EvalError::BadApplication(format!("Gate not found: {}", g))),
    }
}

fn apply(v1: Value, v2: Value) -> Result<Value, EvalError> {
    match (v1, v2) {
        (Value::Term(Term::Const(Const::Gate(g))), Value::Term(t)) => apply_gate(&g, &t),
        (Value::Term(Term::Const(Const::Meas)), Value::Superpos(s)) => Ok(Value::Term(s.measure())),
        (Value::Term(t1), Value::Term(t2)) => Ok(Value::Term(beta_reduce(t1, t2)?)),
        (Value::Term(t), Value::Superpos(s)) => {
            Ok(Value::Superpos(s.map_terms(|t2| {
                apply(Value::Term(t.clone()), Value::Term(t2))
            })?))
        }
        (Value::Superpos(s), Value::Term(t)) => {
            Ok(Value::Superpos(s.map_terms(|t2| {
                apply(Value::Term(t2), Value::Term(t.clone()))
            })?))
        }
        (Value::Superpos(s1), Value::Superpos(s2)) => {
            Ok(Value::Superpos(s1.zip_terms(&s2, |t1, t2| {
                apply(Value::Term(t1), Value::Term(t2))
            })?))
        }
    }
}

pub fn eval(term: Term, env: &mut HashMap<String, Value>) -> Result<Value, EvalError> {
    match term {
        Term::Const(_) | Term::Abs(_, _) => Ok(Value::Term(term)),
        Term::Var(ref x) => match env.get(x) {
            Some(v) => Ok(v.clone()),
            None => Err(EvalError::UndefinedSymbol(x.into())),
        },
        Term::App(t1, t2) => {
            let v1 = eval(*t1, env)?;
            let v2 = eval(*t2, env)?;
            let res = apply(v1, v2)?;
            match res {
                Value::Term(t) => eval(t, env),
                Value::Superpos(mut s) => {
                    s.merge();
                    Ok(Value::Superpos(s))
                }
            }
        }
    }
}
