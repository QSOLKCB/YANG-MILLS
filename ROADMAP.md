# YANG-MILLS roadmap

The roadmap is ordered to keep implementation validation, numerical evidence, physical inference, and mathematical proof claims distinct.

## Phase 0 — constitution + minimal `SU(2)` reference core

**Status: implemented by PR #1 candidate.**

### Constitution

- [x] State the Clay problem target without claiming a solution.
- [x] Define a hard claim boundary between finite lattice evidence and continuum proof.
- [x] Record GLUBALL `v1.0.0` as an engineering donor only.
- [x] Define draft machine-readable gauge-group, lattice, and evidence contracts.
- [x] Define reproducibility and provenance requirements.
- [x] Add agent rules that fail closed on semantic overclaiming.

### Minimal reference implementation

- [x] Add a pinned, dependency-free Rust reference crate.
- [x] Represent `SU(2)` by unit quaternions with explicit matrix convention.
- [x] Add periodic four-dimensional lattice indexing.
- [x] Add oriented plaquette construction.
- [x] Add the Wilson action.
- [x] Check identity-lattice zero action.
- [x] Check a one-link analytic action vector `S = 6 beta` on a `3^4` lattice.
- [x] Check local gauge invariance numerically to a declared tolerance.
- [x] Check transformed links remain unit `SU(2)` elements to tolerance.
- [x] Run Rust tests and contract validation in CI.

### Explicit Phase 0 non-goals

- Monte Carlo ensemble generation;
- thermalisation/autocorrelation analysis;
- `SU(3)` production calculations;
- glueball operator construction or mass extraction;
- continuum or infinite-volume extrapolation;
- confinement claims;
- proof of existence of continuum quantum Yang–Mills theory;
- proof of a positive continuum mass gap;
- GPU/CUDA authority;
- GLUBALL knot/topology import.

## Phase 1 — finite lattice core

- [ ] Freeze `YM-SU2-LATTICE-V1` after independent review of conventions.
- [ ] Add validated deterministic configuration serialisation.
- [ ] Add Wilson loops beyond elementary plaquettes.
- [ ] Add Polyakov loops as observables without prematurely assigning a confinement claim.
- [ ] Add staple construction and local action-difference checks.
- [ ] Add `SU(2)` update-kernel primitives needed by later ensemble algorithms.
- [ ] Add bounded lattice-size and overflow validation.
- [ ] Add independent golden vectors derived from analytic configurations.
- [ ] Add `SU(3)` algebra/group representation only after the `SU(2)` reference is stable.

**Graduation gate:** all advertised observables are gauge invariant under independent local transformations within declared numerical tolerances, and all golden vectors are sealed.

## Phase 2 — statistical ensemble generation

- [ ] Specify a versioned sampling algorithm before implementation.
- [ ] Implement a standard `SU(2)` heat-bath and/or over-relaxation method from published literature.
- [ ] Separate pseudorandom seed identity from physical claims.
- [ ] Record thermalisation, sweep count, measurement cadence, and chain identity.
- [ ] Measure autocorrelation and effective sample size.
- [ ] Add independent reproducibility runs.
- [ ] Add `SU(3)` ensemble generation only after `SU(2)` diagnostics are trustworthy.

**Graduation gate:** ensemble results reproduce basic published plaquette expectations in a declared parameter regime within statistical uncertainty.

## Phase 3 — glueball laboratory

- [ ] Define gauge-invariant spatial loop operator bases.
- [ ] Add smearing/blocking as separately versioned transformations.
- [ ] Construct connected Euclidean correlators.
- [ ] Add effective-mass analysis with uncertainty propagation.
- [ ] Start with the scalar `0++` channel.
- [ ] Add `2++` and `0-+` only after the scalar pipeline is validated.
- [ ] Keep finite-volume spectral masses explicitly distinct from the continuum Clay gap.

**Graduation gate:** a complete analysis can be reproduced from sealed ensemble evidence and produces stable finite-lattice estimates under reasonable fit-window/operator-basis variations.

## Phase 4 — published baseline reproduction

- [ ] Select peer-reviewed pure-gauge benchmark results before tuning the implementation.
- [ ] Reproduce dimensionless observables and selected glueball mass ratios.
- [ ] Document disagreements rather than adjusting hidden conventions.
- [ ] Archive exact source commit, toolchain, inputs, seeds, raw measurements, and analysis outputs.

**Graduation gate:** the reference implementation reproduces selected published benchmarks within justified uncertainty and discretisation/volume expectations.

## Phase 5 — accelerator and HPC sidecars

GLUBALL contributes engineering patterns here, not Yang–Mills physics.

- [ ] Define a stable CPU/reference surface first.
- [ ] Add GPU kernels only as accelerators of already specified calculations.
- [ ] Recompute bounded GPU outputs or residuals against the CPU reference.
- [ ] Record device, driver, runtime, compiler, launch, and precision metadata.
- [ ] Keep accelerator output non-authoritative for mathematical contracts.
- [ ] Add multi-device scaling only after single-device acceptance is clean.

## Phase 6 — finite-volume and continuum scaling

- [ ] Run multiple physical volumes and lattice spacings.
- [ ] Track scale-setting assumptions explicitly.
- [ ] Quantify finite-volume effects.
- [ ] Quantify discretisation effects.
- [ ] Perform continuum extrapolations only with a declared model and uncertainty treatment.
- [ ] Keep observed positive extrapolated masses classified as numerical/physical evidence, not proof.

## Phase 7 — constructive boundary

This phase asks what is still missing between controlled lattice results and the Clay theorem.

- [ ] Inventory the exact continuum-limit and infinite-volume statements required.
- [ ] Identify where uniform bounds would be necessary.
- [ ] Track reflection positivity / reconstruction requirements relevant to the chosen route.
- [ ] Separate perturbative renormalisation inputs from nonperturbative existence results.
- [ ] Study rigorous literature on lattice-to-continuum constructions and known obstructions.
- [ ] State candidate lemmas as conjectures until proved.
- [ ] Never infer a theorem from finite numerical sampling.

## Phase 8 — formalisation of stable theorem surfaces

Formalisation begins only after the underlying definitions are stable enough to deserve it.

Potential targets include:

- exact periodic lattice indexing;
- group closure and inverse identities for symbolic `SU(2)`;
- plaquette covariance under local gauge transformations;
- gauge invariance of the Wilson action;
- combinatorial facts behind analytic regression configurations;
- finite-lattice statements that can be proved without floating-point assumptions.

Continuum existence and the positive mass gap remain separate theorem targets and may not be represented as established unless a genuine proof exists.

## Publication discipline

Any release that reports scientific results must include:

1. exact source commit and release tag;
2. machine-readable contract versions;
3. toolchain/runtime identity;
4. complete input/seed/configuration identity;
5. raw or content-addressed evidence sufficient to reproduce advertised results;
6. uncertainty and systematic-error treatment where statistical inference is involved;
7. an explicit claim classification from `docs/CLAIM_BOUNDARY.md`.
