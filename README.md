# QLAM: An interpreter for Quantum Lambda Calculus
<p align="center">
  <img src="qlam.png" />
</p>

## Overview
QLAM (pronounced like "clam") is an interpreter for a language based on van Tonder's [Quantum Lambda Calculus](https://arxiv.org/abs/quant-ph/0307150). This is similar to the classical lambda calculus, but the objects we compute with are now able to be put in [quantum superposition](https://en.wikipedia.org/wiki/Quantum_superposition). In fact, van Tonder has shown that this calculus is equivalent in strength to a Quantum Turing Machine (or the quantum circuit model, if you prefer).

QLAM provides primitive quantum objects such as the basis states |0>, |1>, and the [universal gate set](https://en.wikipedia.org/wiki/Quantum_logic_gate#Universal_quantum_gates) {CNOT, H, T}. Everything else happens with regular beta-reduction, although we need to branch whenever one part of a function application is in superposition.

## Usage

First, ensure you have Rust and `cargo` installed. To install the binary to your `.cargo/bin`, you can run
```
cargo install --path .
```
You can then run `qlam` to open a REPL session.

## Roadmap
Currently, QLAM can parse input from the REPL, evaluate terms, and print their normal forms. It also supports assignment of variables to let you build up larger expressions. Here are some things that are planned for the future:
- Visualization of reduction steps
- Test bench for common quantum algorithms
- [Superoptimization](https://en.wikipedia.org/wiki/Superoptimization) of small lambda terms
