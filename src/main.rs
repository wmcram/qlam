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
}
