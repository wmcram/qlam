use std::{collections::HashMap, mem};

use crate::term::Term;
use num_complex::Complex;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct State(pub Vec<(Term, Complex<f64>)>);

impl State {
    // Creates the trivial superposition from a classical term.
    pub fn trivial(init: Term) -> Self {
        Self(vec![(init, Complex::new(1.0, 0.0))])
    }

    // Normalizes this state, combining amplitudes as well.
    fn normalize(&mut self) {
        let old = mem::take(&mut self.0);
        let mut map: HashMap<Term, Complex<f64>> = HashMap::new();

        for (term, amp) in old {
            // filter out vanishing amplitudes
            if amp.norm_sqr() > 1e-9 {
                *map.entry(term).or_default() += amp;
            }
        }

        self.0 = map.into_iter().collect();
    }

    // Samples a term from the quantum superposition, consuming this state.
    // The state vector must be nonempty to avoid panics.
    pub fn measure(mut self) -> Term {
        self.normalize();
        let mut rng = rand::thread_rng();
        let probs: Vec<f64> = self.0.iter().map(|(_, amp)| amp.norm_sqr()).collect();

        let r: f64 = rng.r#gen();
        let mut res = 0.0;
        for (i, p) in probs.iter().enumerate() {
            res += p;
            if res >= r {
                return self.0.into_iter().nth(i).unwrap().0;
            }
        }
        self.0
            .into_iter()
            .last()
            .expect("State vector cannot be empty")
            .0
    }
}
