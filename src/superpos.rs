use crate::term::{EvalError, Term, Value};
use num_complex::Complex;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Superpos(pub Vec<(Term, Complex<f64>)>);

impl Superpos {
    // Creates the trivial superposition from a classical term.
    pub fn trivial(init: Term) -> Self {
        Self(vec![(init, Complex::new(1.0, 0.0))])
    }

    // Merges identical terms in the branches of the superposition.
    pub fn merge(&mut self) {
        let mut merged: Vec<(Term, Complex<f64>)> = Vec::new();
        for (t, amp) in self.0.iter() {
            if let Some((_, cur)) = merged.iter_mut().find(|(u, _)| *u == *t) {
                *cur += amp;
            } else {
                merged.push((t.clone(), *amp));
            }
        }
        merged.retain(|(_, amp)| amp.norm_sqr() > 1e-9);
        self.0 = merged;
    }

    // Maps the function over the branches of the superposition, flattening any newly-generated
    // superpositions into the toplevel one.
    pub fn map_terms<F>(&self, f: F) -> Result<Self, EvalError>
    where
        F: Fn(Term) -> Result<Value, EvalError>,
    {
        let mut out = Vec::new();
        for (t, amp) in &self.0 {
            match f(t.clone())? {
                Value::Term(t2) => out.push((t2, *amp)),
                // When we flatten here we need to multiply to get the joint probability
                Value::Superpos(s) => {
                    for (u, amp2) in s.0 {
                        out.push((u.clone(), amp * amp2))
                    }
                }
            }
        }
        Ok(Self(out))
    }

    // Maps the binary function over both superpositions, taking their branchwise product and
    // flattening as in map_terms.
    pub fn zip_terms<F>(&self, other: &Superpos, f: F) -> Result<Self, EvalError>
    where
        F: Fn(Term, Term) -> Result<Value, EvalError>,
    {
        let mut out = Vec::new();
        for (t1, amp1) in &self.0 {
            for (t2, amp2) in &other.0 {
                match f(t1.clone(), t2.clone())? {
                    Value::Term(t3) => out.push((t3, amp1 * amp2)),
                    Value::Superpos(s) => {
                        for (u, amp3) in &s.0 {
                            out.push((u.clone(), amp1 * amp2 * amp3))
                        }
                    }
                }
            }
        }

        Ok(Self(out))
    }

    // Samples a term from the quantum superposition, consuming this state.
    // The state vector must be nonempty to avoid panics.
    pub fn measure(mut self) -> Term {
        self.merge();
        let mut rng = rand::thread_rng();
        let probs: Vec<f64> = self.0.iter().map(|(_, amp)| amp.norm_sqr()).collect();

        let r: f64 = rng.r#gen();
        let mut res = 0.0;
        for (i, p) in probs.iter().enumerate() {
            res += p;
            if res >= r {
                return self.0.into_iter().nth(i).unwrap().0.to_classical();
            }
        }
        self.0
            .into_iter()
            .last()
            .expect("State vector cannot be empty")
            .0
    }
}

impl std::fmt::Display for Superpos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for (t, amp) in &self.0 {
            writeln!(f, "({t}): {amp},")?;
        }
        writeln!(f, "]")
    }
}
