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

**Phase 1B freeze gate:** `YM-SU3-ALGEBRA-V1` and `SU3-ALGEBRA-PHASE1B-V1` were sealed first as `phase-1b-algebra-frozen`. The dependent `YM-SU3-LATTICE-V1` and `SU3-LATTICE-PHASE1B-V1` were introduced only after that algebra review gate, then independently tested/reviewed and sealed as `phase-1b-frozen`. The exact three-cycle permutation matrix is used only as a traceless `SU(3)` finite-lattice regression fixture; no center-symmetry interpretation is imported into Phase 1B.

### Phase 1C — `C3`, `Z3`, and the genuine `SU(3)` center

**Status: implemented, independently reviewed, CI-green, and frozen in PR #4 pending merge.**

`C3` is the unique abstract group of order three and is isomorphic to the additive group `Z3`. The research task is therefore not merely to recognize the abstract group, but to distinguish different embeddings and actions inside `SU(3)`.

Reference lead: [Wolfram MathWorld — Cyclic Group C3](https://mathworld.wolfram.com/CyclicGroupC3.html).

- [x] Define the abstract cyclic group `C3 = <g | g^3 = e>` and record `C3 ~= Z3` as an abstract-group isomorphism.
- [x] Define the three-cycle basis permutation `Omega |k> = |k+1 mod 3>` and verify `Omega^3 = I`.
- [x] Verify the canonical three-cycle permutation matrix has determinant `+1` and is unitary, so `<Omega>` forms a non-central `C3` subgroup of `SU(3)`.
- [x] Exhibit an explicit `SU(3)` element that does not commute with `Omega`, proving that this embedding is not central.
- [x] Define the center of `SU(3)` explicitly as `Z(SU(3)) = {I, omega I, omega^2 I}`, with `omega = exp(2 pi i/3)`.
- [x] Verify every center element is unitary, has determinant one, and commutes with independently generated tested `SU(3)` matrices.
- [x] Verify `Z(SU(3)) ~= C3 ~= Z3` while keeping it distinct from the non-central subgroup `<Omega>`.
- [x] Treat “same abstract group” and “same embedded subgroup/action” as different claims in tests and documentation.
- [x] Add center transformations to the Polyakov-loop test surface without making any confinement/deconfinement inference.

The frozen center surface uses the forward basis-cycle convention `Omega|k>=|k+1 mod 3>` and the exact group-valued witness `diag(-1,-1,1)` to prove non-centrality. The genuine center is represented separately by scalar matrices `omega^k I`. A center-slice transformation leaves the finite-lattice Wilson action invariant within declared tolerance and rotates a periodic Polyakov holonomy/trace by the chosen center element.

**Phase 1C freeze gate:** `YM-SU3-CENTER-V1` and `SU3-CENTER-PHASE1C-V1` are sealed as `phase-1c-frozen` on the reviewed PR #4 surface. Incompatible changes to the abstract-group convention, cycle embedding, genuine-center embedding, or Polyakov center-transformation semantics require a new contract identifier. This phase establishes transformation laws and bounded finite-lattice observations only; it does not claim confinement, deconfinement, spontaneous center-symmetry breaking, an area law, or a thermodynamic order-parameter measurement.

### Phase 1D — paths, parallel transport, Wilson/Polyakov holonomy

This checkpoint connects the differential geometry of curves to the gauge-theory observables used on the lattice. Curve geometry is useful here because a connection is sampled along an oriented path; the curvature of the path itself is not Yang–Mills curvature.

- [ ] Define `YM-PATH-HOLONOMY-V1` for oriented piecewise-smooth paths and their lattice discretisations.
- [ ] Define a path `gamma:[0,1] -> M`, its tangent `gamma_dot(t) in T_(gamma(t))M`, and the pullback of the local gauge connection along the path.
- [ ] Define continuum parallel transport by the connection along `gamma`, with path ordering and sign/convention choices stated explicitly.
- [ ] Record the path-ordered exponential schematically as `U_gamma = P exp(- integral_gamma A)` or the convention-equivalent form chosen by the project.
- [ ] Verify reparameterisation invariance under orientation-preserving changes of path parameter.
- [ ] Verify path reversal gives `U_(gamma^-1) = U_gamma^-1`.
- [ ] Verify path concatenation gives the convention-consistent ordered product `U_(gamma2 o gamma1) = U_gamma2 U_gamma1`.
- [ ] Distinguish open Wilson lines from closed Wilson loops: open transport transforms at both endpoints, while the normalized trace of closed holonomy is gauge invariant.
- [ ] Treat Wilson loops as path-ordered closed-loop holonomies with normalized trace as the gauge-invariant observable.
- [ ] Add deterministic ordered-product tests for arbitrary closed lattice paths.
- [ ] Add Polyakov loops wrapping the periodic Euclidean-time direction.
- [ ] Add gauge-transformation regression vectors for Wilson and Polyakov observables.
- [ ] Relate infinitesimal closed-loop holonomy to the gauge curvature `F_A` only through a stated small-loop expansion; do not identify a finite loop directly with a pointwise curvature tensor.
- [ ] State explicitly that Frenet curvature, torsion, osculating circles/planes, and other embedded-curve invariants are distinct from the bundle curvature `F_A` unless a separate theorem connects them in a specific construction.
- [ ] Keep finite-lattice holonomy measurements distinct from continuum confinement theorems.
- [ ] Prepare the later static-potential analysis surface `<W(R,T)> ~ exp(-V(R) T)` without claiming an area law before ensemble evidence exists.

### Phase 1E — classical continuum bundle/connection dictionary

This checkpoint records the standard geometric objects that the lattice theory approximates. It is a classical mathematical layer and must not be confused with construction of the quantum theory required by the Clay problem.

Reference leads include Wolfram MathWorld entries on [Principal Bundles](https://mathworld.wolfram.com/PrincipalBundle.html), [Vector Bundle Connections](https://mathworld.wolfram.com/VectorBundleConnection.html), [Fiber Bundles](https://mathworld.wolfram.com/FiberBundle.html), [Tangent Bundles](https://mathworld.wolfram.com/TangentBundle.html), [Parallel Transport](https://mathworld.wolfram.com/ParallelTransport.html), and related differential-geometry entries supplied during Phase 0 research.

- [ ] Define a smooth base manifold `M`, its tangent bundle `TM`, and cotangent bundle `T^*M`; keep these base-manifold objects distinct from gauge fibers.
- [ ] Define differential `k`-forms and record that a 2-form is an alternating bilinear form on pairs of tangent vectors.
- [ ] Define a principal `G`-bundle `pi:P -> M` with compact Lie group `G` acting freely on each fiber.
- [ ] Distinguish the principal bundle from an associated vector bundle `E = P x_rho V` obtained from a representation `rho:G -> GL(V)`.
- [ ] Define a bundle section `s:M -> E` by `pi o s = id_M`, and distinguish sections of associated bundles from the principal connection itself.
- [ ] Record that matter/vector fields are sections of associated bundles, while the gauge connection is fundamentally a connection on the principal bundle.
- [ ] Define the induced covariant derivative `nabla_A: Gamma(E) -> Gamma(T^*M tensor E)` and its Leibniz rule on sections.
- [ ] Record the local connection form `A` only after choosing a trivialization; do not mistake local gauge potentials for globally defined ordinary one-forms on a nontrivial bundle.
- [ ] State the bundle-valued type of Yang–Mills curvature precisely, e.g. `F_A in Omega^2(M, ad P)` (or the representation-equivalent associated-bundle form).
- [ ] Define curvature geometrically and locally: `F_A = dA + A wedge A`, with Lie-bracket/wedge conventions stated explicitly.
- [ ] Distinguish gauge curvature `F_A` from Riemannian curvature of the base manifold, Gaussian/surface curvature, and curvature of an individual path.
- [ ] Distinguish the gauge connection `A` from the Levi-Civita connection on `TM`: the latter is the unique torsion-free metric-compatible connection determined by a Riemannian metric, while the former is a connection on the gauge bundle.
- [ ] Record how a Riemannian metric and orientation on `M` induce the Hodge star needed for `*F_A`, the Yang–Mills norm, and the formal adjoint `d_A^*`; freeze sign conventions before theorem work.
- [ ] Where complex associated bundles are used, record the role of Hermitian fiber metrics and unitary representations without conflating them with the base-manifold Riemannian metric.
- [ ] Define flatness by `F_A = 0` and distinguish local gauge triviality from global bundle topology/holonomy.
- [ ] Define parallel transport on associated fibers from the gauge connection and distinguish it from Levi-Civita parallel transport of tangent vectors.
- [ ] Record the Bianchi identity `d_A F_A = 0`.
- [ ] Define the classical Yang–Mills functional using an invariant inner product on the Lie algebra, schematically `YM(A) = integral_M <F_A, F_A> dvol`.
- [ ] Derive the Euler–Lagrange equation `d_A^* F_A = 0` from variation of the classical functional.
- [ ] State explicitly that a Yang–Mills connection is a **critical point** of the functional; it is not automatically a global minimizer.
- [ ] In four dimensions, define self-duality/anti-self-duality `F_A = +/- *F_A` and verify that, together with the Bianchi identity, it implies the Yang–Mills equation.
- [ ] Keep instanton/topological applications (Chern classes, second Chern number, Donaldson-theory connections) as a separate classical/topological research branch unless they become directly relevant to the mass-gap route.
- [ ] Add a precise lattice-continuum dictionary connecting link holonomy, plaquette holonomy, continuum connection, and curvature without claiming equality at finite lattice spacing.
- [ ] Treat principal-bundle, associated-vector-bundle, section, tangent/cotangent bundle, connection, covariant derivative, 2-form curvature, parallel transport, holonomy, Wilson loop, Hodge star, and gauge transformation as separately versioned definitions before formalisation.

**Classical-continuum boundary:** establishing the classical equations, bundle geometry, instanton identities, curve/parallel-transport identities, or finite-lattice approximations does not construct the interacting quantum Yang–Mills theory and does not establish a mass gap.

### Cross-domain donor quarantine

Previous QSOL-IMC work involving triadic `SU(3)`, semantic gauge fields, relational curvature, or semantic mass-gap language may inspire software tests or invariant-first methodology, but the following boundaries are mandatory.

Research leads currently include *A Triadic Ontology with SU(3) Invariants*, *Spectral Signatures of Relational Curvature in Generative Models: An SU(3) Gauge Theoretic Framework*, and *Relational Gauge Thermodynamics: Curvature as Entropy Production in Generative Cognition*. These are cross-domain research inputs, not Yang–Mills physics authorities.

- [ ] Algebraic identities are re-derived independently before entering the Yang–Mills contract surface.
- [ ] The triadic paper's `Omega` may motivate `C3` subgroup tests, but its semantic ontology is not imported as QCD physics.
- [ ] Query/Key/Value-to-color assignments are semantic-model hypotheses, not QCD color physics.
- [ ] A semantic or cognitive “mass gap” is not evidence for the physical Yang–Mills mass gap.
- [ ] Semantic confinement, hallucination curvature, thermodynamic cognition, and similar analogies remain outside physical evidence and theorem claims.
- [ ] Holonomy/Wilson-loop ideas may motivate test surfaces only after they are independently restated in standard gauge-theory form.
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
- the implication `F_A = +/- *F_A => d_A^*F_A = 0` in four dimensions, once conventions are fixed;
- finite-lattice statements that can be proved without floating-point assumptions.

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
