# Clay problem boundary

## Official target

The Clay Mathematics Institute currently lists **Yang–Mills and the Mass Gap** as an unsolved Millennium Prize Problem.

The official problem description by Arthur Jaffe and Edward Witten requires, in substance, a proof that for every compact simple gauge group `G`, a non-trivial quantum Yang–Mills theory exists on four-dimensional Euclidean space and has a strictly positive mass gap `Delta`.

Primary references:

- Clay overview: https://www.claymath.org/millennium/yang-mills-the-maths-gap/
- Jaffe–Witten official description: https://www.claymath.org/wp-content/uploads/2022/06/yangmills.pdf

This repository does not replace those sources. Where project language and the official problem description differ, the official problem description governs what counts as solving the Millennium Problem.

## What "mass gap" means at the theorem boundary

At the continuum quantum-field-theory level, the vacuum has zero energy and the Hamiltonian has non-negative spectrum. A positive mass gap means there is a non-zero interval immediately above the vacuum containing no spectrum.

This is a statement about the rigorously constructed continuum quantum theory. It is not defined here as "the lowest positive number produced by a lattice fit."

## Why lattice Yang–Mills is relevant

A finite lattice provides a mathematically and computationally concrete regularisation. Link variables take values in a compact gauge group; Wilson loops and plaquettes provide gauge-invariant observables; Euclidean correlation functions can be used to estimate finite-volume spectral quantities such as glueball masses.

These calculations are scientifically relevant because controlled lattice results provide nonperturbative evidence and can probe the expected low-energy spectrum.

They do not by themselves establish:

- existence of the required continuum quantum field theory;
- the axiomatic properties required by the official problem;
- a justified infinite-volume limit;
- a justified continuum limit;
- a uniform strictly positive gap surviving those limits.

## Repository research question

YANG-MILLS therefore asks two different questions and never conflates them:

1. **Computational/physical:** what can be reproduced and measured reliably in finite lattice gauge theory, including glueball observables and scaling behaviour?
2. **Mathematical:** what additional rigorous statements and uniform controls are required to bridge from lattice regularisations to the continuum existence-and-mass-gap theorem?

A useful answer to the first question is not automatically an answer to the second.
