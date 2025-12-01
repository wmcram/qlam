use crate::parser::{ParseError, parse};
use crate::term::Term;

pub struct Circuit {
    layers: Vec<Vec<Block>>,
    input: Vec<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    I,
    H,
    T,
    C,
    S,
}

#[derive(Debug, Clone, Copy)]
pub enum CircuitError {
    EmptyCircuit,
    InvalidChar,
    DimMismatch,
}

// Parses a textual circuit into memory.
// The format is given by a series of lines.
// The first line should be the input layer of the circuit,
// which must be computational basis states '0' or '1'.
// Each subsequent line should be the gates to apply for a certain layer, ordered top to bottom.
// For example, the line 'H T C' will apply a Hadamard to the first wire, T to the second, and
// a CNOT on the third and fourth wires.
pub fn parse_circuit(text: &str) -> Result<Circuit, CircuitError> {
    // Parse input layer
    let mut input = Vec::new();
    let mut lines = text.lines();
    if let Some(first_line) = lines.next() {
        for c in first_line.chars() {
            match c {
                '0' => input.push(false),
                '1' => input.push(true),
                c if c.is_whitespace() => continue,
                _ => return Err(CircuitError::InvalidChar),
            }
        }
    } else {
        return Err(CircuitError::EmptyCircuit);
    }

    // Parse layers
    let mut layers = Vec::new();
    let mut cur = Vec::new();
    for line in lines {
        for c in line.chars() {
            match c {
                'I' => cur.push(Block::I),
                'H' => cur.push(Block::H),
                'T' => cur.push(Block::T),
                'C' => cur.push(Block::C),
                'S' => cur.push(Block::S),
                c if c.is_whitespace() => continue,
                _ => return Err(CircuitError::InvalidChar),
            }
        }
        layers.push(cur);
        cur = Vec::new();
    }

    // Do basic checking of dimension for each layer
    let dim = input.len();
    for layer in &layers {
        let mut acc = 0;
        for block in layer {
            match block {
                Block::C | Block::S => acc += 2,
                _ => acc += 1,
            }
        }
        if acc != dim {
            return Err(CircuitError::DimMismatch);
        }
    }

    Ok(Circuit {
        layers: layers,
        input: input,
    })
}

impl Circuit {
    // Compiles a circuit down to an equivalent lambda term.
    pub fn to_lambda(&self) -> Result<Term, ParseError> {
        // Following this block, input will be a church-encoded n-tuple representing
        // the input layer.
        let mut input = "(\\f.f".to_string();
        for b in &self.input {
            if *b {
                input += " |1>";
            } else {
                input += " |0>";
            }
        }
        input += ") ";

        // Construct a layer to apply to the above n-tuple in continuation-passing style.
        let mut layers: Vec<String> = vec![input];

        for layer in &self.layers {
            // Gather up the CNOT and SWAP indices to move them to the front
            let mut cnots = Vec::new();
            let mut idx: usize = 0;
            for block in layer {
                match block {
                    Block::C => {
                        cnots.push((idx, idx + 1));
                        idx += 2
                    }
                    Block::S => idx += 2,
                    _ => idx += 1,
                }
            }

            // Construct the prefix
            let mut cur = "(".to_string();
            for i in 0..self.input.len() {
                cur += &format!("\\x{i}.");
            }

            // Create the CNOT chain
            for (q1, q2) in &cnots {
                cur += &format!("(C (pair x{q1} x{q2}))");
                cur += &format!(" (\\'x{q1}.\\'x{q2}.");
            }
            cur += "\\f.f";

            // Construct the output tuple
            let mut idx = 0;
            for block in layer {
                cur += " (";
                match block {
                    Block::I => (),
                    Block::H => cur += "H ",
                    Block::T => cur += "T ",
                    Block::C => {
                        cur += &format!("'x{idx})");
                        idx += 1;
                        cur += &format!(" ('x{idx})");
                        idx += 1;
                        continue;
                    }
                    Block::S => {
                        idx += 1;
                        cur += &format!("x{idx})");
                        idx -= 1;
                        cur += &format!(" (x{idx})");
                        idx += 2;
                        continue;
                    }
                }

                cur += &format!("x{idx})");
                idx += 1;
            }

            cur += &")".repeat(1 + cnots.len());
            layers.push(cur);
        }

        let full_str = layers.join(" ");
        return parse(&mut full_str.chars());
    }
}
