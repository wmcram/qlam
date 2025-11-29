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
}

enum CircuitError {
    EmptyCircuit,
    InvalidChar,
    DimMismatch,
}

// Parses a textual circuit into memory.
fn parse_circuit(text: &str) -> Result<Circuit, CircuitError> {
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
                Block::I => acc += 1,
                Block::H => acc += 1,
                Block::T => acc += 1,
                Block::C => acc += 2,
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

// Compiles a circuit down to an equivalent lambda term.
fn circuit_to_lambda(c: &Circuit) -> Term {
    todo!()
}
