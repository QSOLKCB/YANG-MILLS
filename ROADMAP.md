# YANG-MILLS roadmap

The roadmap is ordered to keep implementation validation, numerical evidence, physical inference, classical continuum geometry, and mathematical proof claims distinct.

## Scientific integrity invariants

Every phase is governed by [`docs/SCIENTIFIC_INTEGRITY_INVARIANTS.md`](docs/SCIENTIFIC_INTEGRITY_INVARIANTS.md). These rules were distilled from YANG-MILLS review history and cross-domain QSOL-IMC audit/stress-test work; they are methodological authority only and do not import non-Yang-Mills physics.

- [x] **SI-1 — No outcome by construction.** A scientific observable must emerge from frozen dynamics/analysis and the pipeline must be able to return a negative result.
- [x] **SI-2 — Claim ↔ implementation ↔ evidence traceability.** Every computational claim maps to an actual mathematical/code object, evidence object, and discriminating test.
- [x] **SI-3 — Adversarial invariant regressions.** Gauge invariance, covariance, conservation, accounting, group membership, replay, and other invariants require tests designed to break them, not only happy-path examples.
- [x] **SI-4 — Approximation obligations.** Every approximation declares its validity domain, neglected remainder, quantitative bound/tolerance or unresolved proof obligation, and the conclusions invalid outside that domain.
- [x] **SI-5 — Regulator obligations.** Every cutoff, lattice spacing, finite volume, smoothing scale, projection kernel, or regulator records which observables depend on it and what limit would be required to remove that dependence.
- [x] **SI-6 — No silent claim-class promotion.** Finite-lattice facts, numerical observations, physical inferences, continuum theorems, and Clay claims remain distinct regardless of numerical precision.
- [x] **SI-7 — Preserve failure as evidence.** A failed predeclared gate is retained; seeds, windows, thresholds, models, operators, or geometry are not changed under the same authority to manufacture a pass.
- [x] **SI-8 — Sufficient is not necessary.** A successful mechanism is not declared necessary, unique, or fundamental without a separate argument.
- [x] **SI-9 — Decorative mathematics is not physical structure.** Cross-domain algebras, lattices, fields, entropies, curvatures, or operators may donate tests/fixtures/architecture only after independent derivation; notation resemblance is not physical evidence.
- [x] **SI-10 — New interpretation requires new authority.** A semantic change in observable, transformation, recovery hypothesis, physical interpretation, or theorem bridge receives a new versioned contract rather than retroactive relabeling.

The reusable lessons from the QSOL-IMC bundle are therefore explicit: claim-to-implementation stress testing, “no outcome by construction,” proof obligations for singular/regularized/approximate models, and independently re-derived `A2/SU(3)` algebra fixtures. The surrounding speculative ontologies remain quarantined from Yang-Mills physical authority.

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

**Phase 1D freeze gate:** `YM-PATH-HOLONOMY-V1` and `PATH-HOLONOMY-PHASE1D-V1` are sealed as `phase-1d-frozen`.

### Phase 1E — classical continuum bundle/connection dictionary

**Status: completed, independently reviewed, CI-green, frozen, and merged in PR #6.**

This checkpoint freezes the standard classical geometric objects that the lattice/path theory approximates. It does not construct the quantum theory required by the Clay problem.

- [x] Define the smooth base-manifold/tangent/cotangent/form dictionary.
- [x] Define principal and associated bundles, bundle sections, principal connections, local potentials, and induced covariant derivatives.
- [x] Reconcile the standard right-bundle section-change rule with frozen Phase 1D via `Omega=g^-1`.
- [x] Freeze `F_A in Omega^2(M,ad P)`, local curvature formulas, gauge covariance, and Bianchi.
- [x] Keep gauge, Levi-Civita, Riemann, Gaussian, Frenet, torsion, and osculating curvatures/transport distinct.
- [x] Freeze the `SU(3)` Hermitian↔anti-Hermitian Lie-algebra bridge.
- [x] Freeze the oriented 4D Riemannian Hodge/formal-adjoint conventions and independent gauge-fiber inner product.
- [x] Define flatness, local pure-gauge behavior, possible nontrivial global holonomy, and multi-chart transport patching.
- [x] Define the classical Yang–Mills functional/equation and critical-point boundary.
- [x] Define self-/anti-self-duality and its Bianchi implication without a quantum mass-gap implication.
- [x] Keep instanton/Chern/Donaldson work as a separately contracted future branch.
- [x] Freeze the lattice↔continuum dictionary and separately versioned semantic definition IDs.
- [x] Add the bounded executable oriented-Euclidean-`R^4` Hodge/two-form convention fixture.

**Phase 1E freeze gate:** `YM-CLASSICAL-GEOMETRY-V1` and `CLASSICAL-GEOMETRY-PHASE1E-V1` are sealed as `phase-1e-frozen` on the reviewed PR #6 surface. Incompatible semantic changes require new identifiers.

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

## Phase 2 — statistical ensemble generation

Phase 2 separates sampler mechanics from external finite-lattice validation so neither can silently stand in for the other.

### Phase 2A — frozen `SU(2)` sampler and diagnostics

**Status: completed, independently reviewed, CI-green, frozen, and merged in PR #7.**

- [x] Specify a versioned sampling algorithm before implementation as `YM-SU2-ENSEMBLE-V1`.
- [x] Implement a literature-grounded `SU(2)` heat-bath kernel under the frozen local staple/action convention.
- [x] Implement microcanonical `SU(2)` over-relaxation and verify Wilson-action preservation within declared tolerance.
- [x] Freeze the reference pseudorandom stream/seed derivation while separating stream identity from physical claims.
- [x] Record thermalisation, checked total sweep count, measurement cadence, update order, and chain identity.
- [x] Retain raw average-plaquette measurements in the reference chain report.
- [x] Measure a versioned autocorrelation sequence, integrated autocorrelation time, effective sample size, and diagnostic standard error.
- [x] Add fixed-chain deterministic replay and distinct-chain identity regressions.
- [x] Add a `beta=0` Haar sanity surface with exact ensemble expectation zero, broad finite-sample acceptance band, ESS floor, and unit-link residual gate.
- [x] Bound heat-bath rejection attempts and fail closed rather than silently biasing an update.
- [x] Freeze `SPLITMIX64-V1` golden integer outputs while explicitly declining universal cross-platform bitwise floating identity.
- [x] Keep all Phase 2A ensemble measurements classified as finite-lattice numerical observations rather than physical/theorem claims.

**Phase 2A freeze gate:** `YM-SU2-ENSEMBLE-V1` and `SU2-ENSEMBLE-PHASE2A-V1` are sealed as `phase-2a-frozen`. Phase 2A itself did not claim the external published benchmark; that claim is owned by Phase 2B.

### Phase 2B — published plaquette baseline and Phase 2 graduation

**Status: completed, independently reviewed, CI-green, frozen, and merged in PR #8.**

- [x] Select a peer-reviewed pure-`SU(2)` Wilson-action plaquette benchmark before tuning the reference implementation.
- [x] Match the benchmark action normalization, lattice geometry/volume, coupling regime, observable definition, and source-resolution/statistical treatment explicitly.
- [x] Freeze two independent chain identities with declared thermalisation and measurement cadence before inspecting the benchmark result.
- [x] Reproduce the selected published plaquette surface within the predeclared source-resolution envelope while satisfying the independent-chain diagnostics.
- [x] Archive contract/source provenance, pinned toolchain context, seeds/chain IDs, raw measurements, diagnostics, and comparison analysis in the commit-tied runtime evidence log.
- [x] Preserve the claim firewall: benchmark agreement is finite-lattice validation and does not imply confinement, continuum QFT existence, or a positive mass gap.

Primary source: Langfeld, Lucini & Rago, *Physical Review Letters* **109**, 111601 (2012), DOI `10.1103/PhysRevLett.109.111601`, Figure 5. The frozen comparison is `SU(2)` Wilson action on `10^4` at `beta=2.3` with source-resolution envelope `[0.59,0.62]` for the normalized average plaquette. The first predeclared two-chain candidate produced pooled mean `0.6022919679`, chain-consistency `z ~= 0.946`, minimum ESS `~=7.75`, and maximum final link-unit residual `~=6.66e-16`.

**Phase 2 graduation gate:** `YM-SU2-PLAQUETTE-BASELINE-V1` and `SU2-PLAQUETTE-BASELINE-PHASE2B-V1` are sealed as `phase-2b-frozen`; `published_plaquette_baseline_reproduced=true` and `phase2_graduation_gate_satisfied=true`. Literature agreement validates the finite-lattice `SU(2)` sampler in this regime; it does not establish confinement, construct continuum quantum Yang–Mills theory, or prove a positive mass gap.

### Phase 2C1 — separately versioned `SU(3)` sampler mechanics

**Status: completed, independently reviewed, CI-green, and frozen in PR #9 pending merge.**

- [x] Specify a new `SU(3)` ensemble contract, `YM-SU3-ENSEMBLE-V1`, rather than extending `YM-SU2-ENSEMBLE-V1`.
- [x] Select and justify a correctness-first symmetric single-generator Metropolis algorithm under the frozen `SU(3)` lattice/action conventions, while explicitly not claiming Cabibbo–Marinari or Kennedy–Pendleton heatbath semantics.
- [x] Version RNG/stream identity, site/direction/generator update order, thermalisation, diagnostics, raw evidence, proposal/accept accounting, and link-group residuals independently.
- [x] Add golden domain-separated chain seeds, fixed-chain replay, distinct-chain evidence, exact `beta=0` all-accept accounting, and fail-closed bounds.
- [x] Require `1e-8 <= proposal_angle <= 1` so numerically inert positive proposal scales fail closed.
- [x] Compute candidate local action differences allocation-free from exactly the six affected plaquettes and regress them against a complete Wilson-action difference.

**Phase 2C1 freeze gate:** `YM-SU3-ENSEMBLE-V1` and `SU3-ENSEMBLE-PHASE2C1-V1` are sealed as `phase-2c1-frozen`. The boundary flags `published_su3_baseline_reproduced=false`, `glueball_analysis_enabled=false`, and `phase2c_complete=false` remain mandatory; incompatible sampler, proposal, stream, update-order, local-action, diagnostic, or evidence semantics require a new identifier.

### Phase 2C2 — independent published `SU(3)` validation

**Status: completed on PR #10 branch, V2 validation authority frozen pending review/merge.**

Phase 2C1 does **not** authorize later `SU(3)` physics analysis by itself. Phase 2C2 preserves the failed first attempt rather than rewriting it: `YM-SU3-PLAQUETTE-BASELINE-V1` reproduced the published plaquette envelope but failed its predeclared per-chain ESS floor (`4.510 < 8`). A separately predeclared fresh-stream `YM-SU3-PLAQUETTE-BASELINE-V2` then retained the same source surface, sampler, proposal scale, per-chain schedule, acceptance/stationarity/consistency thresholds, and group-residual limits while using four fresh independent chains. V2 explicitly changed the ESS criterion from a per-chain floor of 8 to finite-positive ESS per chain plus an aggregate ESS floor of 16.

- [x] Select a peer-reviewed pure-`SU(3)` Wilson-action finite-lattice benchmark only after the Phase 2C1 sampler mechanics are frozen.
- [x] Predeclare its action normalization, lattice geometry/volume, coupling, observable, source treatment, chain identities, thermalisation, cadence, and statistical gates before inspecting the result.
- [x] Preserve the failed V1 attempt without lowering its threshold after inspection.
- [x] Reproduce the benchmark with fresh independent chains and retained raw evidence under the separately versioned V2 validation authority.
- [x] Keep glueball analysis disabled until the independent `SU(3)` benchmark gate passes.

Primary source: Edgar, McCrossen & Moriarty, *Journal of Physics G: Nuclear Physics* **7** (1981) L85–L88, DOI `10.1088/0305-4616/7/5/001`. The frozen comparison is pure `SU(3)` Wilson action on `4^4` at `beta=6.0`, with conservative graphical normalized-plaquette envelope `[0.56,0.63]`.

The successful V2 confirmation (Actions run `34761701867`, evidence head `f5a221fdad2046779358cc59460fae488626459b`) produced pooled mean `0.5992920043`, aggregate ESS `78.944`, maximum pairwise chain-consistency `z ~= 3.550`, and maximum stationarity `z ~= 1.465`. All predeclared acceptance, accounting, source-envelope, and group-residual gates passed.

**Phase 2C2 freeze gate:** `YM-SU3-PLAQUETTE-BASELINE-V2` and `SU3-PLAQUETTE-BASELINE-PHASE2C2-V2` are sealed as `phase-2c2-frozen`; `published_su3_baseline_reproduced=true`, `phase2c_complete=true`, and `glueball_analysis_enabled=true`. V1 remains `phase-2c2-v1-failed`. This authorizes Phase 3 work but is not itself a glueball result, confinement claim, continuum construction, or mass-gap proof.

## Phase 3 — glueball laboratory

Phase 3A was merged in PR #11 and is frozen. Phase 3B1 was merged in PR #12 and is frozen. Phase 3B2 was merged in PR #13 and is frozen. Phase 3C was merged in PR #14 and is frozen. Phase 3D was merged in PR #15 and is frozen. Phase 3E was merged in PR #16 and is frozen. Phase 3F was merged in PR #17 and is frozen with its first negative production receipt preserved. Phase 3G was merged in PR #18 and is frozen with its negative fresh-stream recovery receipt preserved. Phase 3H was merged in PR #19 and is frozen with its negative blocking-capable geometry receipt preserved. Phase 3I is the candidate temporal-extent recovery authority. Phase 3J is a mechanics-only scalar variational/GEVP authority that deliberately assumes no Phase 3I outcome.

- [x] Define gauge-invariant spatial loop operator bases (`YM-GLUEBALL-OPERATORS-V1`, frozen).
- [x] Add smearing/blocking as separately versioned transformations.
  - [x] Phase 3B1 spatial stout smearing (`YM-GLUEBALL-SPATIAL-STOUT-V1`, frozen).
  - [x] Phase 3B2 stride-2 spatial blocking (`YM-GLUEBALL-SPATIAL-BLOCK-V1`, frozen).
- [x] Construct connected Euclidean correlators (`YM-GLUEBALL-CONNECTED-CORRELATOR-V1`, frozen).
- [x] Add effective-mass analysis with uncertainty propagation (`YM-GLUEBALL-EFFECTIVE-MASS-V1`, Phase 3D frozen).
- [x] Add predeclared plateau/window selection and full-covariance scalar fitting (`YM-GLUEBALL-SCALAR-FIT-V1`, Phase 3E frozen).
- [x] Start with the scalar `0++` channel through frozen finite-lattice `A1++` mechanics.
- [x] Predeclare, execute, and preserve the first production scalar evidence/stability run (`YM-GLUEBALL-SCALAR-PRODUCTION-V1`, Phase 3F frozen after PR #17 merge).
- [x] Preserve the Phase 3F negative result without weakening seeds, windows, thresholds, or rejection semantics.
- [x] Predeclare, execute, and preserve the fresh-stream independent-statistics recovery (`YM-GLUEBALL-SCALAR-RECOVERY-V1`, Phase 3G frozen after PR #18 merge).
- [x] Freeze the observed Phase 3G receipt without changing its fresh stream, chain count, geometry, windows, thresholds, or scientific classification.
- [x] Predeclare a new scalar recovery authority that changes geometry/operator overlap rather than increasing chain count again (`YM-GLUEBALL-SCALAR-GEOMETRY-RECOVERY-V1`, Phase 3H candidate).
- [x] Freeze Phase 3H before observation at `6 x 6 x 6 x 10`, 32 fresh independent chains, `beta=5.7`, the unchanged 68-sweep schedule, and master seed `0x3f02202609150001`.
- [x] Add the frozen Phase 3B2 stride-2 `block-1` transformation, producing `3 x 3 x 3 x 10` coarse configurations, and predeclare `block-1/plaquette-1x1` as the primary surface.
- [x] Keep the Phase 3E windows, full-covariance fit mechanics, correlator replay tolerance, and `z <= 5` stability threshold unchanged.
- [x] Execute the first Phase 3H observation and preserve its outcome without post-observation tuning: all eight scalar surfaces were `effective-mass-rejected`; `scalar_pipeline_validated=false`.
- [x] Freeze the Phase 3H negative receipt from head `c241991e58860795d247f70d7f92b9e984c96a64` / workflow `34901768744` and enforce deterministic replay.
- [x] Apply the Phase 3H failure branch by predeclaring `YM-GLUEBALL-SCALAR-TEMPORAL-RECOVERY-V1` (Phase 3I candidate).
- [x] Change only temporal extent `N_t: 10 -> 16` plus a fresh master stream; preserve spatial geometry, chain count, operators, transformations, windows, fit semantics, and thresholds.
- [ ] Execute the first Phase 3I observation and preserve its outcome without post-observation tuning.
- [x] Build a separately versioned mechanics-only two-operator scalar variational/GEVP authority (`YM-GLUEBALL-SCALAR-VARIATIONAL-BASIS-V1`, Phase 3J candidate) without assuming the Phase 3I outcome.
- [x] Freeze the Phase 3J candidate mechanics to a symmetric connected `2 x 2` cross-correlator matrix, `t0=1`, `t1=2`, positive-definite `C(t0)`, Cholesky whitening, largest generalized eigenvalue, and `v^T C(t0) v = 1` normalization.
- [ ] If Phase 3I passes, run a fresh-stream independent confirmation of the same temporal surface before any production variational reinterpretation or new irreducible channels.
- [ ] If Phase 3I fails, use frozen/reviewed Phase 3J mechanics only as a prerequisite for a new separately predeclared variational scalar recovery authority rather than retuning Phase 3I.
- [ ] Add `2++` and `0-+` only after the scalar production pipeline has passed an independently reviewed stability gate.
- [ ] Keep finite-volume spectral masses explicitly distinct from the continuum Clay gap.

**Phase 3F evidence boundary:** the predeclared `4 x 4 x 4 x 10` run used sixteen independently seeded frozen `SU(3)` chains and one retained configuration per chain. Sampler diagnostics passed but all six raw/stout scalar surfaces were `effective-mass-rejected`. The result is frozen historical evidence; Phase 3F is not a tunable baseline.

**Phase 3G evidence boundary:** the independently predeclared recovery run retained the same `4 x 4 x 4 x 10`, `beta=5.7`, 68-sweep-per-chain geometry/schedule and all Phase 3F analysis semantics, changed to fresh master seed `0x3f01202609150001`, and doubled the independent chain count from 16 to 32. The observation on head `65dacc9f03cb0fd74826cd211ee7bd586668e08a` / workflow `34875522823` again rejected all six scalar surfaces. Therefore this specific independent-statistics-only recovery hypothesis failed. The negative result is machine-pinned and must not be converted into a pass by later threshold or stream changes under the same identifier.

**Phase 3H evidence boundary:** the geometry/operator-overlap recovery kept the Phase 3G chain count and scalar-fit thresholds fixed, changed to fresh stream `0x3f02202609150001`, used fine extents `6 x 6 x 6 x 10`, and evaluated eight raw/stout/block surfaces. The observation on head `c241991e58860795d247f70d7f92b9e984c96a64` / workflow `34901768744` passed sampler diagnostics but all eight surfaces were `effective-mass-rejected`, leaving `scalar_pipeline_validated=false`. The negative result is frozen and replay-checked.

**Phase 3I predeclaration boundary:** the temporal recovery changes only `N_t` from 10 to 16 and the fresh master stream to `0x3f03202609270001`. Spatial extent remains `6^3`, the blocked surface remains `3^3`, chain count remains 32, and all operators, transformations, windows, covariance/fit mechanics, replay tolerance, and `z <= 5` thresholds remain unchanged. The exact workload is `7,520,256` proposals per chain and `240,648,192` proposals total.

**Phase 3J mechanics boundary:** the two frozen scalar `A1++` operators are now admitted jointly through a symmetric connected cross-correlator matrix and bounded `2 x 2` GEVP mechanics candidate. Phase 3J assumes neither a Phase 3I pass nor failure and activates no production recovery. Its deterministic two-state fixture, gauge-invariance check, basis-permutation invariance, and independent basis-rescaling invariance are mechanics evidence only.

**Current scalar direction:** Phase 3I remains the unresolved production observation. Phase 3J can mature in parallel as analysis mechanics without contaminating that observation. After Phase 3I is frozen, the roadmap still branches exactly as predeclared: confirmation if it passes; a new variational recovery authority built on Phase 3J if it fails.

**Graduation gate:** a complete scalar production analysis can be reproduced from sealed ensemble evidence and produces stable finite-lattice estimates under predeclared fit-window/operator/transformation scans, followed by an independently reviewed confirmation on a deliberately declared recovery/confirmation surface. Passing this finite-lattice gate still does not establish a continuum mass gap.

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
- [ ] Build a machine-readable proof-obligation graph from finite lattice → uniform bounds → thermodynamic limit → continuum limit → OS/reconstruction → spectral gap.
- [ ] Treat conditional or heuristic literature results as route maps, not established bridge lemmas, until every hypothesis is discharged.

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
- exact sampler/RNG/update-order statements that can be isolated from floating statistical observations;
- finite-lattice and finite-dimensional Hodge statements that can be proved without floating-point assumptions.

Continuum quantum existence and the positive mass gap remain separate theorem targets and may not be represented as established unless a genuine proof exists.

## Publication discipline

Any release that reports scientific results must include:

1. exact source commit and release tag;
2. machine-readable contract versions;
3. toolchain/runtime identity;
4. complete input/seed/chain/configuration identity;
5. raw or content-addressed evidence sufficient to reproduce advertised results;
6. uncertainty and systematic-error treatment where statistical inference is involved;
7. an explicit claim classification from `docs/CLAIM_BOUNDARY.md`.
