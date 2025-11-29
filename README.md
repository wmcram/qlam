# QLAM: An interpreter for Quantum Lambda Calculus
<p align="center">
  <img src="qlam.png" />
</p>

## Overview
QLAM (pronounced like "clam") is an interpreter for a language based on van Tonder's [Quantum Lambda Calculus](https://arxiv.org/abs/quant-ph/0307150). This is similar to the classical lambda calculus, but the objects we compute with are now able to be put in [quantum superposition](https://en.wikipedia.org/wiki/Quantum_superposition). In fact, van Tonder has shown that this calculus is equivalent in strength to a Quantum Turing Machine (or the quantum circuit model, if you prefer).

QLAM provides primitive quantum objects such as the basis states |0>, |1>, and the [universal gate set](https://en.wikipedia.org/wiki/Quantum_logic_gate#Universal_quantum_gates) {CNOT, H, T}. Everything else happens with regular beta-reduction, although we need to branch whenever one part of a function application is in superposition.

QLAM also features a compiler for a rudimentary quantum circuit language, which makes it easier to translate arbitrary quantum algorithms into lambda terms. The circuit format is detailed more below.

## Usage

First, ensure you have Rust and `cargo` installed. To install the binary to your `.cargo/bin`, you can run
```
cargo install --path .
```
You can then run `qlam` to open a REPL session.

To use the circuit compiler, run `qlam compile <FILEPATH>`. If successful, the compiled lambda term will be printed to `stdout`.

## Circuit Format

To describe a quantum circuit for use with the compiler, the layers of the quantum circuit must be written line-by-line into a file. The first line must contain only the characters '0' and '1', and represents the input layer of the circuit (many quantum algorithms just start with all '0's).

Each subsequent line after the first then describes a layer of gates, where the leftmost gate is applied to the leftmost qubit. Keep in mind that the two-qubit CNOT gate will be applied to the next two qubits; for instance, the line 'H C T' will apply a Hadamard to the first qubit, a CNOT to the second and third qubits, and a T gate to the fourth. 

Every layer of the circuit must have the same dimensionality in order to compile; you can use the identity gate 'I' to skip qubits while keeping the circuit well-formed.

## Roadmap
Currently, QLAM can parse input from the REPL, evaluate terms, and print their normal forms. The compiler can reduce circuits in the format above to continuation-passing-style lambda terms. The REPL supports assignment of variables to let you build up larger expressions. Here are some things that are planned for the future:
- Visualization of reduction steps
- Test bench for common quantum algorithms
