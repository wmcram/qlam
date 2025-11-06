# QLAM: An interpreter for the Quantum Lambda Calculus

QLAM (pronounced like "clam") is an interpreter for a language based on van Tonder's [Quantum Lambda Calculus](https://arxiv.org/abs/quant-ph/0307150). This is similar to the classical lambda calculus, but the objects we compute with are now able to be put in [quantum superposition](https://en.wikipedia.org/wiki/Quantum_superposition). In fact, van Tonder has shown that this calculus is equivalent in strength to a Quantum Turing Machine (or the quantum circuit model, if you prefer).

QLAM provides as primitives quantum kets such as |0>, |1>, or |010100>, and the [universal gate set](https://en.wikipedia.org/wiki/Quantum_logic_gate#Universal_quantum_gates) {CNOT, H, T}. Everything else happens with regular beta-reduction, although we need to branch whenever one part of a function application is in superposition.

Currently, QLAM can evaluate ASTs defined in source code. Here are some things that are planned for the future:
- Visualization of reduction steps
- Test bench for common quantum algorithms
- [Superoptimization](https://en.wikipedia.org/wiki/Superoptimization) of small lambda terms
