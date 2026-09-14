# YANG-MILLS agent contract

This repository intentionally separates software correctness, numerical evidence, physical inference, classical geometry, continuum quantum construction, and mathematical proof. Coding and research agents must preserve that separation.

## Non-negotiable claim hierarchy

Use these classes:

1. **implementation fact** — a property established about code/tests/contracts;
2. **finite-lattice mathematical fact** — an exact or proved statement about the discrete model;
3. **numerical observation** — a computed result with declared parameters and tolerances/uncertainties;
4. **physical inference** — an interpretation supported by a specified model and evidence;
5. **continuum theorem** — a mathematically proved statement about the continuum theory;
6. **Clay solution claim** — only permissible if the full official existence-and-mass-gap problem is actually proved.

A correct classical identity or statistically stable finite-lattice chain is not automatically a construction of the quantum theory. Stochastic ensemble measurements and published finite-lattice benchmark agreement remain C2 numerical evidence unless a stronger claim is independently justified.

Never promote a result because a simulation is large, a GPU agrees with a CPU, an extrapolation looks stable, a chain has a large ESS, or a literature value is reproduced.

## Required semantic refusals

An agent must refuse to write or approve language asserting that this repository has solved the Yang–Mills Millennium Problem merely because it has any of the following:

- a positive finite-lattice energy or mass estimate;
- a glueball mass estimate;
- an apparently positive continuum extrapolation;
- agreement with lattice literature;
- gauge-invariant numerical observables;
- a formal proof about the finite lattice;
- a classical Yang–Mills connection or self-dual solution;
- a complete principal-bundle/curvature dictionary;
- a thermalised-looking Markov chain, low autocorrelation, large effective sample size, or reproduced plaquette benchmark;
- cross-runtime or CPU/GPU agreement.

## Mathematical authority

Phase 0 / Phase 1A freeze the `SU(2)` finite-lattice authority. Do not silently change the sign convention, plaquette orientation, direction ordering, boundary conditions, lattice dimension, Wilson normalisation, path orientation, staple orientation, or frozen contract identifiers.

The `SU(2)` convention is

```text
U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3)
```

with Wilson action

```text
S_W = beta * sum_{x, mu<nu} (1 - 1/2 Re Tr U_mu_nu(x)).
```

Phase 1B freezes a separate `SU(3)` authority. The algebra convention is

```text
T^a = lambda_a / 2
Tr(lambda_a lambda_b) = 2 delta_ab
U(theta) = exp(i sum_a theta_a T^a).
```

The dependent `YM-SU3-LATTICE-V1` may use but may not redefine the frozen algebra/group conventions, residuals, or tolerances.

Phase 1C freezes a separate center authority. Keep abstract `C3 ~= Z3`, the non-central `<Omega>` embedding, and genuine `Z(SU(3))` distinct.

### Phase 1D path/transport authority

Phase 1D preserves the path ordering already frozen by `YM-WILSON-LOOP-V1`:

```text
+mu : use U_mu(x), then advance x->x+mu
-mu : first advance x->x-mu, then use U_mu(x-mu)^dagger
path product : multiply segments left-to-right in traversal order.
```

The matching continuum convention is

```text
dU_gamma/dt = U_gamma a_gamma(t)
A^Omega = Omega A Omega^-1 - dOmega Omega^-1
U_gamma^Omega = Omega(start) U_gamma Omega(end)^-1.
```

If `gamma2 o gamma1` means traverse `gamma1` first and `gamma2` second, then `U_(gamma2 o gamma1)=U_gamma1 U_gamma2`.

### Phase 1E classical geometry authority

Phase 1E freezes `YM-CLASSICAL-GEOMETRY-V1`. It is a classical dictionary, not a quantum construction.

Principal-bundle, associated-bundle, connection, curvature, Hodge, Yang–Mills functional, self-duality, and the transport relation `U_gamma=P_gamma^-1` must remain internally consistent with the frozen contract. Gauge curvature must not be collapsed into Riemann/Frenet/Gaussian curvature. Classical self-duality plus Bianchi implies the classical Yang–Mills equation; it does not imply a quantum mass gap.

### Phase 2A stochastic authority

Phase 2A freezes `YM-SU2-ENSEMBLE-V1` as the first stochastic authority. It depends on the frozen `SU(2)` lattice/staple semantics and does **not** reopen those contracts.

The reference pseudorandom stream is `SPLITMIX64-V1`, with exact integer stream semantics and versioned seed/chain derivation. A seed is provenance and stream identity, not a physical parameter. Do not replace the RNG, seed derivation, floating conversion, update order, or chain identity fields under the frozen identifier.

For staple `V=k Vhat`, the heat-bath local variable is

```text
R = U Vhat
alpha = beta k
```

with mathematical conditional law

```text
dP(R) proportional to exp(alpha r0) d_mu_Haar(R).
```

For `alpha > 1e-300`, the floating reference uses the truncated-exponential inverse proposal. For `0 <= alpha <= 1e-300`, it uses the uniform `alpha -> 0` proposal limit to avoid subnormal quantisation. Both branches then accept with Haar-Jacobian probability `sqrt(1-r0^2)`, choose the vector direction uniformly on `S^2`, and reconstruct

```text
U_new = R Vhat^dagger.
```

A rejection-attempt bound is a fail-closed computational guard. Exhausting it must return an error; agents may not silently fall back to a biased proposal.

The frozen over-relaxation reflection is

```text
U_new = Vhat^dagger U_old^dagger Vhat^dagger
```

for nonzero staple norm. It is microcanonical decorrelation, not an ergodic sampler by itself.

Reference update order is lexicographic sites `x0,x1,x2,x3`, then directions `mu=0,1,2,3`. A complete Markov sweep is one heat-bath sweep followed by the configured number of over-relaxation sweeps. The checked total schedule is

```text
thermalization_sweeps + measurements*sweeps_between_measurements
```

and must remain inside the frozen reference bound.

Every reported chain must preserve action/lattice parameters, full seed/chain identity, thermalisation, measurement cadence, over-relaxation cadence, rejection bound, raw measurement series, autocorrelation estimator identity, and ESS.

### Phase 2B published benchmark authority

Phase 2B freezes `YM-SU2-PLAQUETTE-BASELINE-V1` as an external finite-lattice validation surface for the Phase 2A sampler.

The primary peer-reviewed source is Langfeld, Lucini & Rago, *Physical Review Letters* **109**, 111601 (2012), DOI `10.1103/PhysRevLett.109.111601`. The frozen comparison is Figure 5 at

```text
SU(2) Wilson action
lattice = 10^4
beta = 2.3
observable = (1/(6V)) sum_x sum_(mu<nu) (1/2) ReTr P_mu_nu(x)
```

Because the source comparison is graphical, the predeclared `[0.59,0.62]` interval is a **source-resolution envelope**, not a quoted paper statistical error bar. Agents may not relabel it as a paper uncertainty or widen it after inspecting the benchmark result.

Two independent cold-start chains agreeing is not sufficient to close Phase 2B. The frozen benchmark must also pass the within-chain post-thermalisation stationarity gate: split each 48-measurement chain into first/last 24-measurement halves, evaluate each half with the frozen Phase 2A autocorrelation-aware diagnostic using `max_lag=6`, and require

```text
|mean_first - mean_second| / sqrt(SE_first^2 + SE_second^2) <= 5
```

for both chains. This is a bounded stationarity check against a shared residual cold-start transient; it is not a proof that every systematic effect is absent. Do not remove or weaken this gate under the frozen Phase 2B identifier.

The frozen reference run may close the finite-lattice Phase 2 graduation gate only if the published envelope, independent-chain, within-chain stationarity, ESS, acceptance, and link-unit gates all pass. It retains

```text
published_plaquette_baseline_reproduced = true
phase2_graduation_gate_satisfied       = true
```

Benchmark agreement validates the frozen `SU(2)` sampler only in the declared finite-lattice regime. It does **not** imply confinement, continuum QFT existence, or a positive continuum mass gap.

### Phase 2C1 `SU(3)` stochastic authority

Phase 2C1 freezes `YM-SU3-ENSEMBLE-V1` and `SU3-ENSEMBLE-PHASE2C1-V1` as a **separate** stochastic authority with status `phase-2c1-frozen`. It may reuse the frozen `SPLITMIX64-V1` core and Phase 2A diagnostic arithmetic as numerical utilities, but it does not inherit the `SU(2)` sampler algorithm, seed derivation, benchmark result, or physical validation.

The frozen identities are

```text
sampler_id  = YM-SU3-ENSEMBLE-V1
proposal_id = YM-SU3-GENERATOR-METROPOLIS-V1
rng_id      = SPLITMIX64-V1
stream_id   = YM-SU3-STREAM-V1
```

with a distinct stream-domain separator and golden derived seeds. Do not replace or alias the `SU(3)` stream derivation under these identifiers.

The correctness-first proposal visits generators `a=0..7` for each link after lexicographic sites and directions `mu=0..3`. The angle draw is an exact finite paired binary64 grid, not a direct `2u-1` transform:

```text
m = raw_u64 & (2^52 - 1)
|theta| = proposal_angle * (2m + 1) / 2^53
sign(theta) = bit 63
R = exp(i theta T^a)
U_candidate = R U_old
accept = min(1, exp(-DeltaS)).
```

Bits `52..62` are ignored. Toggling only bit 63 produces the exact floating inverse angle while preserving equal multiplicity, and neither endpoint `+/-proposal_angle` is in the support. Replacing this with a half-open `2u-1` map is a semantic change because that finite grid has an unmatched endpoint.

The reference consumes exactly **two PRNG outputs per generator proposal**: one raw `u64` for the paired angle grid and one acceptance draw, including for `DeltaS <= 0`. A data-dependent RNG-consumption shortcut is a semantic change.

The binary64 reference requires

```text
1e-8 <= proposal_angle <= 1
maximum total sweeps = 512
maximum measurements = 512.
```

Values below `1e-8` fail closed as numerically inert. The lower bound is a numerical-resolution guard, not a physical scale. The previous exploratory `262144`-sweep admission is not part of V1: the frozen maximum is 512 and CI executes a maximum-schedule beta-zero stress chain with `proposal_angle=1.0` to verify the group residual gates on the admitted boundary.

`DeltaS` is computed from exactly the six plaquettes affected by `U_mu(x)`. Candidate-link evaluation must remain allocation-free: substitute the proposed link directly into those six products; do **not** clone the full lattice for each proposal. The local result is regression-checked against a complete Wilson-action difference.

At `beta=0`, every valid proposal must be accepted. This checks the Metropolis transition kernel only; it is not an exact Haar-equilibrium claim for a short cold-start chain.

Every `SU(3)` chain report must preserve sampler/proposal/RNG/stream IDs, master/chain/derived seed metadata, extents/beta/proposal angle, thermalisation and measurement cadence, proposal/accept counts, complete raw normalized average-plaquette evidence, the full requested autocorrelation evidence, ESS/SE diagnostics, and final maximum link unitarity and determinant residuals. The canonical runtime must emit that complete evidence to the commit-tied Actions log.

The frozen reference chain is additionally pinned against literal golden measurements, proposal accounting, diagnostics, and residuals; a self-comparison of two executions is not sufficient authority for preserving V1 semantics.

The frozen Phase 2C1 sampler authority itself must retain

```text
published_su3_baseline_reproduced = false
glueball_analysis_enabled         = false
phase2c_complete                  = false
```

because external validation does not belong to the sampler contract. Phase 2C2 is now carried by the separately versioned `YM-SU3-PLAQUETTE-BASELINE-V2` authority, which records `published_su3_baseline_reproduced=true`, `glueball_analysis_enabled=true`, and `phase2c_complete=true` after the predeclared fresh-stream V2 confirmation passed. Do **not** back-propagate those true values into `YM-SU3-ENSEMBLE-V1`, and do not reclassify the preserved failed V1 validation attempt as a pass. Do not use the Phase 2B `SU(2)` result as external validation for `SU(3)`. Incompatible sampler, proposal, stream, update-order, local-action, diagnostic, evidence, or admitted-schedule semantics require a new contract identifier rather than mutation of the frozen V1 sampler surface.

Do not import triadic ontology, Q/K/V-to-colour semantics, or other cross-domain interpretations into these authorities.

### Phase 3A frozen glueball-operator authority

Phase 3A freezes `YM-GLUEBALL-OPERATORS-V1` after PR #11 merged. It defines unsmeared closed spatial Wilson-loop operators only. The reference uses Euclidean time direction `3`, spatial directions `[0,1,2]`, requires every spatial extent to be at least `3`, and requires equal spatial extents before using the cubic-lattice `A1++` label. Its continuum channel intent is `0++`; that is not an exact finite-spacing spin assignment.

Phase 3A itself does not authorize connected correlators, effective masses, a glueball spectrum, a finite-volume mass-gap claim, confinement, continuum QFT construction, or a Clay solution. Later surfaces require separate contract identifiers.

### Phase 3B1 frozen spatial stout-smearing authority

Phase 3B1 freezes `YM-GLUEBALL-SPATIAL-STOUT-V1` after PR #12 merged. It is a deterministic transformation of the frozen Phase 3A operator input. Directions `[0,1,2]` are smeared synchronously, direction `3` is copied exactly, spatial extents must be equal and at least `3`, `0 < rho <= 0.25`, and at most `8` levels are admitted. The canonical regression uses `rho=0.10` and one level.

The stout parameter is an analysis/transformation parameter, not a physical observable. Phase 3B1 does not establish improved overlap, construct connected correlators, extract an effective mass or spectrum, establish confinement, construct continuum QFT, or prove a mass gap.

### Phase 3B2 frozen spatial blocking authority

Phase 3B2 freezes `YM-GLUEBALL-SPATIAL-BLOCK-V1` after PR #13 merged. It is a deterministic parameter-free stride-2 spatial coarse-link map. Fine spatial extents must be equal, even, and at least `6`; the output extents are `[Lx/2,Ly/2,Lz/2,Nt]`. A coarse site `y` embeds as `(2y0,2y1,2y2,y3)`. Spatial coarse links are the ordered product of the two fine spatial links along the blocked edge; temporal links are sampled exactly at the embedded fine site. No projection, averaging, or smearing is part of this blocking authority.

The gauge-covariance regression uses the restricted coarse gauge field `G_c(y)=G_f(2y0,2y1,2y2,y3)`. Phase 3B2 does not establish improved overlap, validate combined smear/block tuning, construct connected correlators, extract an effective mass or spectrum, establish confinement, construct continuum QFT, or prove a mass gap.

### Phase 3C frozen connected-correlator authority

Phase 3C freezes `YM-GLUEBALL-CONNECTED-CORRELATOR-V1` after PR #14 merged for the two frozen scalar `A1++` basis elements. It defines bounded periodic finite-sample correlator mechanics and explicit same-sample vacuum subtraction only. For each scalar operator `O`,

```text
O_bar = (1/(N_cfg*N_t)) sum_(c,t) O_c(t)
G_O(tau) = (1/(N_cfg*N_t))*sum_(c,t) O_c(t) O_c((t+tau) mod N_t)
C_O(tau) = G_O(tau) - O_bar^2.
```

Every supplied configuration and every Euclidean time origin has equal arithmetic weight. The reference admits `1 <= N_cfg <= 64` and `2 <= N_t <= 64`, requires identical lattice extents, and applies no hidden time-reflection symmetrization. Raw, frozen-stout, or frozen-blocked lattices may feed the operator layer, but Phase 3C does not tune those transformations or validate a combined smear/block choice.

The canonical four-configuration shifted-defect fixture is an analytic mechanics regression, **not** a Monte Carlo ensemble and not a physical correlator measurement. Phase 3C attaches no statistical uncertainty to it. It does not extract an effective mass, select a fit window, construct a variational/GEVP spectrum, establish a finite-volume mass gap, establish confinement, construct continuum QFT, or prove a positive continuum mass gap. Those require separately versioned later authorities.

### Phase 3D effective-mass candidate authority

Phase 3D introduces `YM-GLUEBALL-EFFECTIVE-MASS-V1` as a bounded scalar `A1++` / continuum-target `0++` analysis surface. It depends on frozen Phase 3C correlator semantics and evaluates

```text
m_eff(tau) = acosh((C(tau-1) + C(tau+1)) / (2 C(tau)))
```

only for predeclared strictly increasing lags with `1 <= tau < N_t/2`. The reference requires `4 <= N_t <= 64`, `C(tau)>0`, and an `acosh` argument at least one; invalid or non-finite inputs fail closed.

Uncertainty is propagated with delete-one-configuration jackknife replicas. Replica `i` must omit configuration `i` and recompute the **complete** connected correlator, including its own vacuum mean and subtraction, before the nonlinear effective-mass transform is applied. Reusing the full-sample vacuum mean inside replicas is outside the authority. The reference admits `3 <= N <= 64` replicas and reports the standard jackknife error

```text
sqrt((N-1)/N * sum_i (m_i - m_bar)^2).
```

The canonical periodic single-state fixture is analytic mechanics evidence only, not a Monte Carlo ensemble or physical glueball measurement. Phase 3D does not select a plateau/fit window, perform a correlated spectral fit, solve a variational GEVP, establish a finite-volume mass gap, establish confinement, construct continuum QFT, or prove a positive continuum mass gap.

## Donor boundary

GLUBALL `v1.0.0` is an engineering donor only. The recorded tag target is:

```text
80941183d14531093117e122da0fc32c13d2464b
```

Allowed donor ideas include provenance sealing, contract/version discipline, deterministic test vectors, reference-vs-accelerator separation, residual acceptance, and bounded stress/testing patterns. GLUBALL geometry/topology is not Yang–Mills physics.

## Change rules

When modifying an existing mathematical/statistical contract:

- state whether the change is editorial, implementation-only, or semantic;
- add/update regression vectors for semantic changes;
- do not reuse a frozen contract identifier for incompatible semantics;
- keep historical provenance intact;
- update `ROADMAP.md` if a phase gate changes.

## Testing rules

Before declaring a PR ready, run the complete currently advertised surface:

```bash
cargo fmt --all -- --check
cargo test --all-targets --locked
cargo run --quiet --locked --bin ym-su2-reference
cargo run --quiet --locked --bin ym-su3-algebra
cargo run --quiet --locked --bin ym-su3-lattice
cargo run --quiet --locked --bin ym-su3-center
cargo run --quiet --locked --bin ym-path-holonomy
cargo run --quiet --locked --bin ym-classical-geometry
cargo run --quiet --locked --bin ym-su2-ensemble
cargo run --quiet --locked --bin ym-su2-plaquette-baseline
cargo run --quiet --locked --bin ym-su3-ensemble
cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline
cargo run --release --quiet --locked --bin ym-su3-plaquette-baseline-v2
cargo run --quiet --locked --bin ym-glueball-operators
cargo run --quiet --locked --bin ym-glueball-smearing
cargo run --quiet --locked --bin ym-glueball-blocking
cargo run --quiet --locked --bin ym-glueball-correlator
cargo run --quiet --locked --bin ym-glueball-effective-mass
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
```

The unified contract entry points are authoritative. Adding a new contract surface without registering it in those commands is a readiness failure.

Tests must fail closed on invalid lattice extents, invalid group construction, invalid/oversized paths, invalid ensemble schedules, non-finite inputs/statistics, rejection-limit exhaustion, numerically inert or asymmetric stochastic proposals, broken contract structure, weakened benchmark provenance, weakened stationarity/equilibration gates, incomplete runtime evidence, or weakened claim boundaries. Public `Result` APIs may not turn admissible numerical/allocation failures into panics.

Floating-point equality is not assumed. Every numerical/statistical comparison must have an explicit tolerance or uncertainty rule and must not be advertised as bitwise cross-platform identity unless separately established.

## Formalisation

Do not add a theorem prover merely for prestige. Formalise only stable definitions and theorem surfaces. A finite-lattice, finite-dimensional group/Hodge, classical bundle/connection, sampler-correctness, or benchmark-provenance theorem is valuable but must not be described as the continuum quantum Yang–Mills existence/mass-gap proof.
