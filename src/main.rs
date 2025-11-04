pub mod superpos;
pub mod term;

use crate::term::{Const, Term, Value, eval};

fn main() {
    let had = Term::Const(Const::Gate("H".into()));
    let app_had = Term::Abs(
        "x".into(),
        Box::new(Term::App(Box::new(had), Box::new(Term::Var("x".into())))),
    );
    let even = Term::App(
        Box::new(app_had),
        Box::new(Term::Const(Const::Ket(vec![false]))),
    );
    let norm = eval(even);
    match norm {
        Value::Term(t) => println!("{t}"),
        Value::Superpos(s) => println!("{}", s.measure()),
    }
}
