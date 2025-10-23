use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

// Gets the free variables of a lambda term.
fn free_vars(t: &Term) -> HashSet<String> {
    match t {
        Term::Var(x) => [x.clone()].into_iter().collect(),
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

// Steps this term one time, returning None if irreducible (we don't reduce under lambdas)
fn small_step(t: &Term) -> Option<Term> {
    match t {
        Term::Var(_) => None,
        Term::Abs(x, body) => None,
        Term::App(a, b) => match &**a {
            Term::Abs(x, body) => Some(subst(body, x, b)),
            _ => {
                if let Some(a2) = small_step(a) {
                    Some(Term::App(Box::new(a2), b.clone()))
                } else if let Some(b2) = small_step(b) {
                    Some(Term::App(a.clone(), Box::new(b2)))
                } else {
                    None
                }
            }
        },
    }
}

fn normalize(mut t: Term, limit: usize) -> Result<Term, String> {
    for _ in 0..limit {
        if let Some(next) = small_step(&t) {
            t = next;
        } else {
            return Ok(t);
        }
    }
    Err(format!("couldn't find normal form: {t}"))
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(x) => write!(f, "{x}"),
            Term::Abs(x, body) => write!(f, "(λ{x}. {body})"),
            Term::App(a, b) => write!(f, "({a} {b})"),
        }
    }
}

fn main() {
    let TRUE = Term::Abs(
        "x".into(),
        Box::new(Term::Abs("y".into(), Box::new(Term::Var("x".into())))),
    );
    println!("TRUE = {}", TRUE);
    println!(
        "{}",
        normalize(
            Term::App(Box::new(TRUE), Box::new(Term::Var("x".into()))),
            1000
        )
        .unwrap()
    );

    let omega = Term::App(
        Box::new(Term::Abs(
            "x".into(),
            Box::new(Term::App(
                Box::new(Term::Var("x".into())),
                Box::new(Term::Var("x".into())),
            )),
        )),
        Box::new(Term::Abs(
            "x".into(),
            Box::new(Term::App(
                Box::new(Term::Var("x".into())),
                Box::new(Term::Var("x".into())),
            )),
        )),
    );
    println!("Ω = {}", omega);
    println!("{}", normalize(omega, 1000).unwrap_err());
}
