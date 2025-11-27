use num::Complex;

use crate::{
    helpers::{abs, app, ket, nonlinear, nonlinear_abs, pair, superpos, var},
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
    NonlinearAbs(String, Box<Term>),
    Nonlinear(Box<Term>),
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

    pub fn to_classical(self) -> Term {
        match self {
            Term::Const(Const::Ket(b)) => {
                if b {
                    var("true")
                } else {
                    var("false")
                }
            }
            Term::Const(_) | Term::Var(_) => self,
            Term::Abs(x, body) => abs(&x, body.to_classical()),
            Term::App(t1, t2) => app(t1.to_classical(), t2.to_classical()),
            Term::NonlinearAbs(x, body) => nonlinear_abs(&x, body.to_classical()),
            Term::Nonlinear(t) => nonlinear(t.to_classical()),
        }
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
            Term::Abs(x, body) => write!(f, "(λ{x}. {body})"),
            Term::App(a, b) => write!(f, "({a} {b})"),
            Term::NonlinearAbs(x, body) => write!(f, "(#{x}. {body})"),
            Term::Nonlinear(t) => write!(f, "!({t})"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Const {
    Ket(bool),
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
        Term::NonlinearAbs(x, body) => &free_vars(body) - &[x.clone()].into_iter().collect(),
        Term::Nonlinear(t) => free_vars(t),
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

// Determines if a term is well-formed; that is, all free variables in nonlinear suspensions refer
// to nonlinear variables in an outer lambda.
fn well_formed(t: &Term) -> Result<(), String> {
    #[derive(Clone, Copy, Debug)]
    enum VarKind {
        Linear(usize),
        Nonlinear,
    }

    fn check(term: &Term, vars: &mut HashMap<String, VarKind>) -> Result<(), String> {
        match term {
            Term::Var(x) => match vars.get_mut(x) {
                Some(VarKind::Linear(count)) => {
                    *count += 1;
                    Ok(())
                }
                _ => Ok(()),
            },

            Term::Const(_) => Ok(()),

            Term::Abs(x, body) => {
                // save old binding if shadowed
                let old = vars.insert(x.clone(), VarKind::Linear(0));
                check(body, vars)?;
                match vars.remove(x) {
                    Some(VarKind::Linear(0)) => Err(format!("linear variable {x} unused")),
                    Some(VarKind::Linear(1)) => Ok(()),
                    Some(VarKind::Linear(n)) => Err(format!("linear variable {x} used {n} times")),
                    _ => unreachable!(),
                }?;
                // restore old binding
                if let Some(v) = old {
                    vars.insert(x.clone(), v);
                }
                Ok(())
            }

            Term::NonlinearAbs(x, body) => {
                let old = vars.insert(x.clone(), VarKind::Nonlinear);
                check(body, vars)?;
                vars.remove(x);
                if let Some(v) = old {
                    vars.insert(x.clone(), v);
                }
                Ok(())
            }

            Term::App(f, a) => {
                check(f, vars)?;
                check(a, vars)
            }

            Term::Nonlinear(t) => {
                let mut inner = vars.clone();
                check(t, &mut inner)?;
                for (x, kind) in inner {
                    if let VarKind::Linear(n) = kind {
                        if n > 0 {
                            return Err(format!("linear variable {x} appears inside !"));
                        }
                    }
                }
                Ok(())
            }
        }
    }

    check(t, &mut HashMap::new())
}

// Safely substitues t[x -> s].
fn subst(t: &Term, x: &str, s: &Term) -> Result<Term, EvalError> {
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
                    abs(&fresh, subst_helper(&renamed_body, x, s))
                } else {
                    abs(&y, subst_helper(body, x, s))
                }
            }
            Term::NonlinearAbs(y, body) => {
                // shadowed case
                if y == x {
                    t.clone()
                } else if free_vars(s).contains(y) {
                    let used: HashSet<_> = free_vars(body).union(&free_vars(s)).cloned().collect();
                    let fresh = fresh_var(y, &used);
                    let renamed_body = subst_helper(body, y, &Term::Var(fresh.clone()));
                    nonlinear_abs(&fresh, subst_helper(&renamed_body, x, s))
                } else {
                    nonlinear_abs(&y, subst_helper(body, x, s))
                }
            }
            Term::App(a, b) => app(subst_helper(a, x, s), subst_helper(b, x, s)),
            Term::Nonlinear(t2) => nonlinear(subst_helper(t2, x, s)),
        }
    }

    Ok(subst_helper(t, x, s))
}

// Performs a classical beta reduction of two terms
fn beta_reduce(t1: Term, t2: Term) -> Result<Term, EvalError> {
    match &t1 {
        Term::Abs(x, body) => Ok(subst(body, x, &t2)?),
        Term::NonlinearAbs(x, body) => match &t2 {
            Term::Nonlinear(t) => Ok(subst(body, x, t)?),
            _ => Err(EvalError::BadApplication(format!(
                "Failure to beta-reduce nonlinear application {} {}: RHS was linear",
                t1, t2
            ))),
        },
        _ => Err(EvalError::BadApplication(format!(
            "Failure to beta-reduce application {} {}: LHS was not a lambda",
            t1, t2
        ))),
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
                let s = f64::sqrt(0.5);
                let vec = vec![
                    (ket(false), Complex::new(s, 0.0)),
                    (ket(true), Complex::new(if *b { -s } else { s }, 0.0)),
                ];
                Ok(superpos(vec))
            } else {
                Err(EvalError::BadGate(format!("Hadamard failure: {}", t)))
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
                let out = match (b1, b2) {
                    (false, false) => pair(ket(false), ket(false)),
                    (false, true) => pair(ket(false), ket(true)),
                    (true, false) => pair(ket(true), ket(true)),
                    (true, true) => pair(ket(true), ket(false)),
                };
                Ok(superpos(vec![(out, Complex::new(1.0, 0.0))]))
            } else {
                Err(EvalError::BadGate("CNOT must take a pair of qubits".into()))
            }
        }
        "T" => {
            if let Term::Const(Const::Ket(b)) = t {
                let phase = Complex::new(0.0, PI / 4.0).exp();
                let vec = vec![(ket(*b), if *b { phase } else { Complex::new(1.0, 0.0) })];
                Ok(superpos(vec))
            } else {
                Err(EvalError::BadGate("T gate must take 1 qubit".into()))
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

pub fn eval(term: Term) -> Result<Value, EvalError> {
    // We do basic term-checking before evaluation to catch out linearity errors
    match well_formed(&term) {
        Err(e) => return Err(EvalError::LinearityViolation(e.into())),
        Ok(_) => (),
    }

    fn helper(term: Term) -> Result<Value, EvalError> {
        match term {
            Term::Const(_)
            | Term::Abs(_, _)
            | Term::NonlinearAbs(_, _)
            | Term::Nonlinear(_)
            | Term::Var(_) => Ok(Value::Term(term)),
            Term::App(t1, t2) => {
                let v1 = helper(*t1)?;
                let v2 = helper(*t2)?;
                let res = apply(v1, v2)?;
                match res {
                    Value::Term(t) => helper(t),
                    Value::Superpos(s) => {
                        let mut s_new = s.map_terms(eval)?;
                        s_new.merge();
                        Ok(Value::Superpos(s_new))
                    }
                }
            }
        }
    }
    helper(term)
}
