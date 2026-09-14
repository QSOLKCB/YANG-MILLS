# Scientific integrity invariants

This document turns recurring lessons from YANG-MILLS reviews, donor audits, failed experiments, and cross-domain QSOL-IMC stress tests into explicit project rules.

These invariants govern implementation, numerical experiments, literature use, and theorem-facing work. They are methodological authority, not new Yang-Mills physics.

## SI-1 — No outcome by construction

A reported observable must emerge from the frozen dynamics and analysis pipeline. Do not define a derived quantity so that the desired effect is guaranteed by the formula itself.

Before accepting a new observable, ask:

```text
Does the quantity emerge from the specified dynamics?

or

Did we write a formula that guarantees the quantity
we later claim to have discovered?
```

A scientific pipeline must be able to return a negative result.

## SI-2 — Every scientific claim maps to an implemented object

For each advertised computational claim, identify:

1. the mathematical object that must exist;
2. the code object that implements it;
3. the evidence object that records it;
4. the regression that distinguishes implementation from decorative scaffolding.

Architecture, class names, metadata, or prose do not count as implementation of a claimed mathematical structure.

## SI-3 — Every invariant gets an adversarial regression

If the project claims gauge invariance, reversibility, conservation, exact accounting, covariance, group membership, deterministic replay, or another invariant, add a test designed to break it.

A positive happy-path test is insufficient when an invalid state could be accepted silently.

## SI-4 — Every approximation declares its domain and remainder

Any approximation must state:

- the limit or scale separation in which it is used;
- the neglected term or remainder class;
- a quantitative bound, tolerance, or clearly marked unresolved proof obligation;
- what conclusions become invalid outside the declared domain.

An approximation may not be promoted to an exact identity by repetition in later phases.

## SI-5 — Every regulator declares what depends on it

If a cutoff, smoothing scale, finite volume, lattice spacing, projection kernel, regularizer, or other regulator is introduced, record which observables depend on it and what limit would be required to remove that dependence.

A regulator-dependent scale is not automatically an emergent physical scale.

## SI-6 — Claim classes cannot be crossed silently

Finite-lattice identities, numerical observations, physical inferences, continuum theorems, and Clay-solution claims remain distinct.

No amount of numerical precision changes the claim class by itself.

Promotion to a higher class requires a separately stated argument and the proof obligations appropriate to that class.

## SI-7 — Failure is preserved as evidence

If a predeclared scientific gate fails, preserve the failure.

Do not change seeds, windows, thresholds, fit models, geometry, operators, or acceptance rules under the same authority merely to obtain a pass. A changed scientific question requires a new contract and, where stochastic evidence is involved, a fresh stream.

## SI-8 — Sufficient is not necessary

Showing that one mechanism is sufficient for an effect does not establish that the mechanism is necessary, unique, or fundamental.

Likewise, observing that a mechanism is efficient, stable, or natural does not establish physical necessity.

## SI-9 — Decorative mathematics is not physical structure

A named algebra, lattice, symmetry, topology, field, entropy, curvature, or operator is not imported as Yang-Mills physics merely because the notation resembles a standard object.

Cross-domain donors may contribute:

- test methodology;
- algebraic fixtures independently re-derived in standard conventions;
- software architecture;
- visualization;
- proof-obligation checklists;
- adversarial examples.

They do not contribute physical authority without an independently justified mapping.

## SI-10 — New interpretation requires new authority

A new physical interpretation, observable definition, recovery hypothesis, transformation surface, or theorem bridge must receive a separately versioned authority when it changes semantics.

Historical evidence is not retroactively relabeled.

## Donor-specific guidance

### QSOL-IMC audit/stress-test work

The reusable contribution is methodological: claim-to-implementation traceability, adversarial regressions, and detection of outcomes encoded into the analysis rule itself.

### Vortex-membrane review work

The reusable contribution is the proof-obligation discipline around singular objects, weak-solution spaces, regulator dependence, approximation remainders, Hamiltonian claims, and conserved quantities. The membrane model itself is not Yang-Mills authority.

### E8/qutrit work

Only independently re-derived standard algebra may enter YANG-MILLS. In particular, A2/SU(3) root-and-weight geometry may be used for deterministic algebra fixtures or visualization. E8 does not enter the Yang-Mills dynamics without a new, independently justified physical derivation.

## Review checklist

Before a new scientific phase is marked ready, reviewers should be able to answer `yes` to all applicable questions:

- Can the experiment return a negative result?
- Is every advertised object actually implemented?
- Does every important invariant have a negative regression?
- Are approximation and regulator dependencies explicit?
- Are stochastic identities, seeds, and schedules predeclared?
- Are failure results preserved rather than tuned away?
- Are sufficiency and necessity kept separate?
- Are cross-domain donors quarantined from physical authority?
- Does every semantic change receive a new contract identifier?
- Is the claimed evidence class no stronger than what the implementation actually establishes?
