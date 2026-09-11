# YANG-MILLS roadmap

The roadmap is ordered to keep implementation validation, numerical evidence, physical inference, classical continuum geometry, and mathematical proof claims distinct.

## Phase 0 — constitution + minimal `SU(2)` reference core

**Status: completed and merged in PR #1.**

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

## Phase 1 — finite lattice core and invariant observables

Phase 1 grows the reference surface from elementary plaquettes to gauge-invariant loop observables, then introduces `SU(3)` only after the `SU(2)` conventions are stable.

### Phase 1A — `SU(2)` observables and local mechanics

**Status: completed, independently reviewed, CI-green, frozen, and merged in PR #2.**

- [x] Freeze `YM-SU2-LATTICE-V1` after independent review of conventions.
- [x] Add validated deterministic configuration serialisation.
- [x] Define `YM-WILSON-LOOP-V1` for arbitrary closed oriented lattice paths.
- [x] Add Wilson loops beyond elementary plaquettes.
- [x] Add rectangular `R x T` Wilson-loop golden vectors.
- [x] Check path closure, reverse-link dagger handling, cyclic trace invariance, and local gauge invariance.
- [x] Check the identity configuration gives `W(C) = 1` for every valid closed loop.
- [x] Add Polyakov loops as observables without prematurely assigning a confinement claim.
- [x] Add staple construction and local action-difference checks.
- [x] Add `SU(2)` update-kernel primitives needed by later ensemble algorithms.
- [x] Add bounded lattice-size and overflow validation.
- [x] Add independent golden vectors derived from analytic configurations.
- [x] Make the lattice-to-continuum bridge explicit: plaquette holonomy is the discrete object whose small-spacing expansion contains `F_mu_nu`; do not treat a finite-spacing plaquette as the continuum field-strength tensor itself.

**Phase 1A freeze gate:** `YM-SU2-LATTICE-V1`, `YM-WILSON-LOOP-V1`, `YM-SU2-CONFIG-V1`, and `SU2-OBSERVABLES-PHASE1A-V1` are sealed on the reviewed PR #2 surface. Incompatible semantic changes require new contract identifiers rather than silent mutation.

### Phase 1B — `SU(3)` algebra and invariant layer

**Status: completed, independently reviewed, CI-green, frozen, and merged in PR #3.**

The `SU(3)` layer is independently derived and tested as gauge mathematics. Cross-domain QSOL-IMC work may suggest useful invariant checks, but it is not physics authority.

- [x] Add the canonical eight Gell-Mann matrices `lambda_1 ... lambda_8`.
- [x] Verify Hermiticity and tracelessness of every generator.
- [x] Verify `Tr(lambda_a lambda_b) = 2 delta_ab`.
- [x] Define `T^a = lambda_a / 2` and verify `[T^a,T^b] = i f^{abc} T^c`.
- [x] Verify the symmetric `d^{abc}` anticommutator structure where used.
- [x] Verify the fundamental quadratic Casimir `sum_a T^a T^a = (4/3) I`.
- [x] Add a separately specified cubic-Casimir regression surface before using cubic invariants computationally.
- [x] Add stable exponentiation from traceless Hermitian algebra elements into `SU(3)`.
- [x] Fail closed on unitarity and determinant-one residuals outside declared tolerances.
- [x] Add `SU(3)` lattice links only after the standalone algebra/group tests are sealed.

The final lattice slice is intentionally minimal: bounded periodic positive-direction links, plaquettes, the standard `SU(3)` Wilson action, and local gauge transformations. It does **not** add Monte Carlo sampling, center-symmetry physics, confinement claims, or glueball extraction.

**Phase 1B freeze gate:** `YM-SU3-ALGEBRA-V1` and `SU3-ALGEBRA-PHASE1B-V1` were sealed first as `phase-1b-algebra-frozen`. The dependent `YM-SU3-LATTICE-V1` and `SU3-LATTICE-PHASE1B-V1` were introduced only after that algebra review gate, then independently tested/reviewed and sealed as `phase-1b-frozen`.

### Phase 1C — `C3`, `Z3`, and the genuine `SU(3)` center

**Status: completed, independently reviewed, CI-green, frozen, and merged in PR #4.**

- [x] Define the abstract cyclic group `C3 = <g | g^3 = e>` and record `C3 ~= Z3` as an abstract-group isomorphism.
- [x] Define the three-cycle basis permutation `Omega |k> = |k+1 mod 3>` and verify `Omega^3 = I`.
- [x] Verify the canonical three-cycle permutation matrix has determinant `+1` and is unitary, so `<Omega>` forms a non-central `C3` subgroup of `SU(3)`.
- [x] Exhibit an explicit `SU(3)` element that does not commute with `Omega`, proving that this embedding is not central.
- [x] Define the center of `SU(3)` explicitly as `Z(SU(3)) = {I, omega I, omega^2 I}`, with `omega = exp(2 pi i/3)`.
- [x] Verify every center element is unitary, has determinant one, and commutes with independently generated tested `SU(3)` matrices.
- [x] Verify `Z(SU(3)) ~= C3 ~= Z3` while keeping it distinct from the non-central subgroup `<Omega>`.
- [x] Treat “same abstract group” and “same embedded subgroup/action” as different claims in tests and documentation.
- [x] Add center transformations to the Polyakov-loop test surface without making any confinement/deconfinement inference.

**Phase 1C freeze gate:** `YM-SU3-CENTER-V1` and `SU3-CENTER-PHASE1C-V1` are sealed as `phase-1c-frozen`. This phase establishes transformation laws and bounded finite-lattice observations only; it does not claim confinement, deconfinement, spontaneous center-symmetry breaking, an area law, or a thermodynamic order-parameter measurement.

### Phase 1D — paths, parallel transport, Wilson/Polyakov holonomy

**Status: completed, independently reviewed, CI-green, frozen, and merged in PR #5.**

- [x] Define `YM-PATH-HOLONOMY-V1` for oriented piecewise-smooth paths and their lattice discretisations.
- [x] Define a path `gamma:[0,1] -> M`, its tangent `gamma_dot(t) in T_(gamma(t))M`, and the pullback `a_gamma(t)=A_(gamma(t))(gamma_dot(t))` of the local gauge connection along the path.
- [x] Freeze continuum parallel transport as `dU_gamma/dt=U_gamma a_gamma(t)`, with the local gauge convention `A^Omega=Omega A Omega^-1-dOmega Omega^-1` and endpoint law `U_gamma^Omega=Omega(start) U_gamma Omega(end)^-1`.
- [x] Record the matching right-ordered exponential convention `U_gamma=P_R exp(integral a_gamma dt)`.
- [x] Record and justify orientation-preserving reparameterisation invariance.
- [x] Verify path reversal gives `U_(gamma^-1) = U_gamma^-1`.
- [x] Verify path concatenation under the frozen traversal convention: if `gamma2 o gamma1` means traverse `gamma1` first and then `gamma2`, `U_(gamma2 o gamma1) = U_gamma1 U_gamma2`.
- [x] Distinguish open Wilson lines from closed Wilson loops.
- [x] Retain `(1/2)ReTr` for frozen `SU(2)` and use the full complex `(1/3)Tr` for the `SU(3)` path surface.
- [x] Add deterministic ordered-product tests for arbitrary closed lattice paths, including a non-planar regression.
- [x] Exercise Polyakov holonomy wrapping a full periodic direction.
- [x] Add gauge-transformation regression vectors for open endpoint covariance, closed Wilson loops, cyclic closed-loop traces, and periodic Polyakov observables.
- [x] Relate infinitesimal closed-loop holonomy to gauge curvature only through the stated small-loop expansion.
- [x] Keep Frenet curvature, torsion, and osculating geometry distinct from gauge curvature `F_A`.
- [x] Keep finite-lattice holonomy measurements distinct from continuum confinement theorems and Phase 1C center transformations distinct from ordinary periodic local gauge transformations.
- [x] Prepare bounded rectangular `R x T` path geometry for later static-potential analysis without evaluating an ensemble expectation or claiming an area law.

**Phase 1D freeze gate:** `YM-PATH-HOLONOMY-V1` and `PATH-HOLONOMY-PHASE1D-V1` are sealed as `phase-1d-frozen`. The contract fixes traversal/product ordering, local-chart containment for its single-chart formula, endpoint covariance, reversal/concatenation, normalized-trace semantics, periodic Polyakov gauge invariance, and the small-loop/curve-geometry firewalls.

### Phase 1E — classical continuum bundle/connection dictionary

**Status: implemented, independently reviewed, CI-green, and frozen in PR #6 pending merge.**

This checkpoint freezes the standard classical geometric objects that the lattice/path theory approximates. It does not construct the quantum theory required by the Clay problem.

- [x] Define a smooth base manifold `M`, its tangent bundle `TM`, and cotangent bundle `T^*M`; keep these base-manifold objects distinct from gauge fibers.
- [x] Define differential `k`-forms and record that a 2-form is an alternating bilinear form on pairs of tangent vectors.
- [x] Define a principal `G`-bundle `pi:P -> M` with compact Lie group `G` acting freely on the right, including the connection-form reproduction/equivariance convention.
- [x] Distinguish the principal bundle from an associated vector bundle `E = P x_rho V` obtained from a representation `rho:G -> GL(V)`.
- [x] Define a bundle section `s:M -> E` by `pi_E o s = id_M`, and distinguish sections of associated bundles from the principal connection itself.
- [x] Record that representation-valued/matter fields are sections of associated bundles while the gauge connection is fundamentally a principal-bundle connection; pure Yang–Mills remains the physical model.
- [x] Define the induced covariant derivative `nabla_A:Gamma(E)->Gamma(T^*M tensor E)`, its local rule `D_A=d+rho_*(A)`, Leibniz rule, and gauge covariance.
- [x] Record the local potential as `A=sigma^*omega_P` only after choosing a trivialization; do not mistake it for one global ordinary 1-form on a nontrivial bundle.
- [x] Reconcile the standard right-bundle section-change rule `A'=g^-1 A g+g^-1 dg` with frozen Phase 1D via `Omega=g^-1`.
- [x] State the global curvature type precisely as `F_A in Omega^2(M,ad P)`.
- [x] Freeze local curvature conventions `F_A=dA+A wedge A` and equivalently `F_A=dA+(1/2)[A wedge A]`, plus curvature gauge covariance.
- [x] Distinguish gauge curvature from Riemann curvature, Gaussian/surface curvature, Frenet curvature, torsion, and osculating geometry.
- [x] Distinguish the gauge connection from the Levi-Civita connection on `TM` and distinguish their parallel transports/curvatures.
- [x] Freeze the `SU(3)` Lie-algebra bridge: Phase 1B Hermitian `T^a=lambda_a/2` corresponds to anti-Hermitian basis `iT^a` for mathematical `su(3)`.
- [x] Record how a Riemannian metric and orientation induce the Hodge star, while an independent `Ad`-invariant Lie-algebra inner product supplies the gauge-fiber norm.
- [x] Freeze the 4D Euclidean sign convention `*^2=+1` on 2-forms and `D_A^*F_A=-*D_A*F_A`.
- [x] Record Hermitian fiber metrics/unitary representations as distinct from the base-manifold Riemannian metric.
- [x] Define flatness by `F_A=0`, local pure-gauge behavior on sufficiently small simply connected trivializing neighborhoods, and the possibility of nontrivial global holonomy.
- [x] Define forward associated-fiber transport `P_gamma`, freeze `U_gamma=P_gamma^-1` for Phase 1D's start-left transporter, and record multi-chart transition-function patching.
- [x] Record the Bianchi identity `D_A F_A=0`.
- [x] Define `YM(A)=(1/2) integral_M |F_A|^2 dvol` using an `Ad`-invariant Lie-algebra inner product.
- [x] Record the Euler–Lagrange equation `D_A^*F_A=0`.
- [x] State explicitly that a Yang–Mills connection is a **critical point**, not automatically a global minimizer.
- [x] In four dimensions, define `F_A^+=(F_A+*F_A)/2`, `F_A^-=(F_A-*F_A)/2`, self-/anti-self-duality, and the Bianchi implication `F_A=+/-*F_A => D_A^*F_A=0`.
- [x] Keep instanton/Chern/Donaldson applications as a separate future classical/topological branch unless separately contracted.
- [x] Add a precise lattice-continuum dictionary: link holonomy, plaquette small-loop holonomy, Wilson loop, and Wilson action are approximating/discretized objects rather than finite-spacing equalities with continuum fields/functionals.
- [x] Assign separately versioned semantic definition IDs for base geometry, forms, bundles, sections, connection, covariant derivative, curvature, Hodge star, parallel transport, Yang–Mills functional, gauge transformation, and lattice-continuum dictionary.
- [x] Add a bounded executable oriented-Euclidean-`R^4` two-form fixture verifying Hodge signs, `*^2`, wedge/Hodge inner product, self-dual decomposition, and fail-closed finite inputs without pretending to implement a general continuum solver.

**Phase 1E freeze gate:** `YM-CLASSICAL-GEOMETRY-V1` and `CLASSICAL-GEOMETRY-PHASE1E-V1` are sealed as `phase-1e-frozen` on the reviewed PR #6 surface. The frozen authority fixes the right principal action, local gauge transformation reconciliation, associated-bundle convention, curvature type/formula, `SU(3)` Hermitian↔anti-Hermitian bridge, Hodge/formal-adjoint signs, Bianchi/Yang–Mills/self-duality dictionary, `U_gamma=P_gamma^-1`, Levi-Civita/gauge firewalls, and finite-lattice↔continuum non-equivalence. Incompatible semantic changes require new identifiers.

**Classical-continuum boundary:** establishing these classical equations, bundle definitions, Hodge identities, self-duality implications, or lattice approximations does not construct the interacting quantum Yang–Mills theory and does not establish a mass gap.

### Cross-domain donor quarantine

Previous QSOL-IMC work involving triadic `SU(3)`, semantic gauge fields, relational curvature, or semantic mass-gap language may inspire software tests or invariant-first methodology, but the following boundaries are mandatory.

- [ ] Algebraic identities are re-derived independently before entering the Yang–Mills contract surface.
- [ ] The triadic paper's `Omega` may motivate `C3` subgroup tests, but its semantic ontology is not imported as QCD physics.
- [ ] Query/Key/Value-to-color assignments are semantic-model hypotheses, not QCD color physics.
- [ ] A semantic or cognitive “mass gap” is not evidence for the physical Yang–Mills mass gap.
- [ ] Semantic confinement, hallucination curvature, thermodynamic cognition, and similar analogies remain outside physical evidence and theorem claims.
- [ ] Holonomy/Wilson-loop ideas may motivate test surfaces only after independently restating them in standard gauge-theory form.
- [ ] Any imported equation is checked against standard gauge-theory conventions; notation errors or missing operators are corrected by derivation rather than copied as authority.
- [ ] Gauge-dependent implementation state is not promoted to scientific output; preferred published observables are gauge invariant or have an explicitly justified gauge fixing.
- [ ] “Mills” number-theory objects such as Mills' theorem, Mills' constant/primes, and Mills ratio are surname collisions and remain outside this project unless an independent mathematical connection is explicitly proved.

**Graduation gate:** all advertised `SU(2)` observables are gauge invariant under independent local transformations within declared numerical tolerances; all selected `SU(3)` algebra identities are independently verified; the abstract `C3 ~= Z3` relation is distinguished from concrete subgroup embeddings; genuine center elements are distinguished from non-central cyclic operators; path/holonomy composition and gauge-covariance rules are internally consistent; the classical bundle/connection dictionary is internally consistent; and all golden vectors are sealed.

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
- `SU(3)` generator identities and Casimir statements after the algebra contract is frozen;
- distinction between the non-central `<Omega> ~= C3` embedding and the central `Z(SU(3)) ~= C3` embedding;
- path reversal, concatenation, and reparameterisation identities for parallel transport;
- endpoint covariance of open transport and gauge invariance of closed Wilson-loop traces;
- principal-bundle and associated-vector-bundle definitions;
- bundle-section and tangent/cotangent-bundle definitions;
- induced covariant derivatives on associated bundles;
- the bundle-valued 2-form type of curvature and curvature covariance;
- separation of Levi-Civita curvature from gauge curvature;
- the Bianchi identity;
- the implication `F_A = +/- *F_A => D_A^*F_A = 0` in four dimensions under the frozen conventions;
- finite-lattice and finite-dimensional Hodge statements that can be proved without floating-point assumptions.

Continuum quantum existence and the positive mass gap remain separate theorem targets and may not be represented as established unless a genuine proof exists.

## Publication discipline

Any release that reports scientific results must include:

1. exact source commit and release tag;
2. machine-readable contract versions;
3. toolchain/runtime identity;
4. complete input/seed/configuration identity;
5. raw or content-addressed evidence sufficient to reproduce advertised results;
6. uncertainty and systematic-error treatment where statistical inference is involved;
7. an explicit claim classification from `docs/CLAIM_BOUNDARY.md`.
