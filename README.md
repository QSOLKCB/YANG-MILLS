# YANG-MILLS

**A deterministic, provenance-sealed computational and theorem-facing laboratory for pure Yang–Mills theory.**

YANG-MILLS studies lattice gauge dynamics, gauge-invariant observables, statistical ensembles, glueball spectra, continuum extrapolation, and the mathematical boundary surrounding the Yang–Mills existence and mass-gap problem.

> Numerical evidence is not a proof of the Millennium Problem.

## Current implementation surface

Phase 0 was merged in PR #1. It established the project constitution, claim boundary, GLUBALL engineering-donor boundary, machine-readable contracts, and a minimal four-dimensional periodic `SU(2)` Wilson-lattice reference core.

Phase 1A was merged in PR #2 and froze the finite-lattice `SU(2)` observable/mechanics surface, including deterministic configuration identity, oriented paths, Wilson loops, Polyakov loops, staples, and local update mechanics.

Phase 1B was merged in PR #3 and froze `SU(3)` algebra and the minimal periodic `SU(3)` Wilson lattice.

Phase 1C was merged in PR #4 and froze the distinction

```text
abstract C3 ~= Z3
        !=
non-central <Omega> subset SU(3)
        !=
genuine Z(SU(3)) = {I, omega I, omega^2 I}.
```

Phase 1D was merged in PR #5 and froze `YM-PATH-HOLONOMY-V1`: arbitrary bounded `SU(3)` open/closed lattice transport, Wilson/Polyakov holonomy, endpoint covariance, reversal/concatenation, and the matching continuum path-ordering convention.

Phase 1E was merged in PR #6 and froze `YM-CLASSICAL-GEOMETRY-V1`: the classical principal/associated-bundle, connection, curvature, Hodge-star, Yang–Mills-functional, self-duality, and lattice-to-continuum dictionary. See [`docs/CLASSICAL_CONTINUUM.md`](docs/CLASSICAL_CONTINUUM.md).

Phase 2A was merged in PR #7 and froze `YM-SU2-ENSEMBLE-V1`: bounded pure-`SU(2)` heat-bath/over-relaxation Markov chains, versioned pseudorandom stream identity, explicit thermalisation and measurement cadence, independent-chain identities, average-plaquette measurements, autocorrelation diagnostics, and effective sample size. See [`docs/SU2_ENSEMBLES.md`](docs/SU2_ENSEMBLES.md).

Phase 2B was merged in PR #8 and froze `YM-SU2-PLAQUETTE-BASELINE-V1`, reproducing a predeclared peer-reviewed `SU(2)` Wilson-action average-plaquette benchmark on a `10^4` lattice at `beta=2.3` with an additional within-chain post-thermalisation stationarity gate. See [`docs/PUBLISHED_PLAQUETTE_BASELINE.md`](docs/PUBLISHED_PLAQUETTE_BASELINE.md).

The predeclared published source-resolution envelope was `[0.59,0.62]`. Two independent frozen chains produced means `0.6026867251` and `0.6018972108`, with pooled mean

```text
0.6022919679
```

and chain-consistency `z ~= 0.946`. The minimum chain ESS was about `7.75`, and the maximum final `SU(2)` unit-link residual was `6.66e-16`. Review hardening split each post-thermalisation chain into first/last 24-measurement halves and required autocorrelation-aware drift `z <= 5`; the two chains produced stationarity `z ~= 2.396` and `z ~= 1.138`, respectively. This closes the finite-lattice Phase 2 graduation gate for the declared `SU(2)` regime. It does **not** establish confinement, continuum quantum-field existence, or a positive continuum mass gap.

Phase 2C1 was merged in PR #9. It froze the separately versioned `SU(3)` ensemble authority `YM-SU3-ENSEMBLE-V1`, using a correctness-first symmetric single-generator Metropolis kernel over the frozen Gell-Mann basis, a domain-separated `YM-SU3-STREAM-V1` chain identity, an exactly paired finite binary64 proposal-angle grid, allocation-free six-plaquette local action differences, a stress-tested 512-sweep reference bound, complete raw runtime evidence, and literal golden-chain regression vectors. See [`docs/SU3_ENSEMBLES.md`](docs/SU3_ENSEMBLES.md).

Phase 2C2 is implemented in PR #10. The first predeclared authority, `YM-SU3-PLAQUETTE-BASELINE-V1`, reproduced the peer-reviewed `4^4`, `beta=6.0` plaquette surface but failed its predeclared per-chain ESS floor, so it remains explicitly failed. The separately predeclared `YM-SU3-PLAQUETTE-BASELINE-V2` used four fresh independent streams without changing the external target, sampler, proposal scale, or per-chain schedule. It passed with pooled mean `0.5992920043` inside the frozen `[0.56,0.63]` source envelope, aggregate ESS `78.944`, maximum pairwise chain-consistency `z ~= 3.550`, and maximum stationarity `z ~= 1.465`. The V2 authority is frozen with `published_su3_baseline_reproduced=true`, `phase2c_complete=true`, and `glueball_analysis_enabled=true`. See [`docs/PUBLISHED_SU3_PLAQUETTE_BASELINE.md`](docs/PUBLISHED_SU3_PLAQUETTE_BASELINE.md).

Phase 3 glueball analysis is now authorized but has not yet been implemented. No current phase performs glueball extraction, a continuum/infinite-volume limit, construction of the interacting quantum theory, or a mass-gap proof.

## The target

The Clay Mathematics Institute problem asks, in substance, for a non-trivial quantum Yang–Mills theory on four-dimensional Euclidean space for any compact simple gauge group, together with a strictly positive mass gap. See [`docs/CLAY_PROBLEM.md`](docs/CLAY_PROBLEM.md).

The repository therefore maintains a strict implication boundary:

```text
finite lattice calculation / ensemble
        !=
classical continuum dictionary
        !=
continuum quantum Yang-Mills construction
        !=
proof of a positive continuum mass gap
```

## Frozen lattice/group conventions

The `SU(2)` reference uses

```text
U(q) = a0 I - i (a1 sigma1 + a2 sigma2 + a3 sigma3)
```

and

```text
S_W = beta * sum_p (1 - 1/2 Re Tr U_p).
```

The frozen `SU(3)` fundamental-representation convention is

```text
T^a = lambda_a / 2
Tr(lambda_a lambda_b) = 2 delta_ab
U(theta) = exp(i sum_a theta_a T^a),
```

with minimal lattice action

```text
S_W = beta * sum_p (1 - 1/3 Re Tr U_p).
```

The mathematical Lie algebra `su(3)` consists of traceless anti-Hermitian matrices. Phase 1E freezes the bridge

```text
Phase 1B Hermitian basis : T^a
anti-Hermitian su(3) basis: i T^a.
```

## Paths and classical continuum geometry

Phase 1D freezes the start-left transporter

```text
dU_gamma/dt = U_gamma a_gamma(t)
U_gamma^Omega = Omega(start) U_gamma Omega(end)^-1.
```

Phase 1E freezes conventional forward associated-fiber transport `P_gamma` by

```text
U_gamma = P_gamma^-1.
```

For a principal bundle `pi:P->M` with free right `G` action, a local section gives

```text
A = sigma^* omega_P,
```

and `sigma'=sigma g` gives

```text
A' = g^-1 A g + g^-1 d g.
```

With `Omega=g^-1`, this is the already-frozen Phase 1D law

```text
A^Omega = Omega A Omega^-1 - dOmega Omega^-1.
```

The global curvature type is

```text
F_A in Omega^2(M, ad P),
```

with local formula `F_A=dA+A wedge A` and Bianchi identity `D_A F_A=0`.

For an oriented Riemannian four-manifold, Phase 1E freezes

```text
*^2 = +1 on 2-forms
D_A^* F_A = - * D_A * F_A
YM(A) = (1/2) integral_M |F_A|^2 dvol
Euler-Lagrange: D_A^* F_A = 0.
```

A Yang–Mills connection is a critical point, not automatically a global minimizer. Classical self-duality does not imply a quantum mass gap.

## Phase 2A SU(2) ensembles

Phase 2A freezes `SPLITMIX64-V1` as a **reference reproducibility stream**, not as a physical parameter or cryptographic RNG. Chain identity includes the master seed, chain ID, derived seed, lattice/action parameters, thermalisation schedule, measurement cadence, over-relaxation cadence, rejection bound, and autocorrelation settings.

For one link with staple `V=k Vhat`, the frozen local heat-bath variable is

```text
R = U Vhat
alpha = beta k
```

with mathematical conditional distribution

```text
dP(R) proportional to exp(alpha r0) d_mu_Haar(R).
```

For `alpha > 1e-300`, the floating reference proposes `r0` from the truncated exponential inverse CDF. For `0 <= alpha <= 1e-300`, it uses the uniform `alpha -> 0` proposal limit to avoid subnormal inverse-CDF quantization that is much larger than the physically resolvable tilt in binary64. Both branches then apply the Haar Jacobian by bounded rejection, sample the quaternion direction uniformly on `S^2`, and reconstruct

```text
U_new = R Vhat^dagger.
```

The microcanonical over-relaxation step is

```text
U_new = Vhat^dagger U_old^dagger Vhat^dagger.
```

A complete reference Markov sweep is one full heat-bath sweep followed by the configured number of full over-relaxation sweeps. The checked total schedule

```text
thermalization_sweeps + measurements*sweeps_between_measurements
```

must not exceed the frozen reference bound.

The measured observable is the average plaquette. The reference diagnostics record the autocorrelation sequence, a versioned positive-window integrated autocorrelation estimate, effective sample size, and ESS-adjusted diagnostic standard error.

## Phase 2B published plaquette baseline

The peer-reviewed primary source is Langfeld, Lucini & Rago, *Physical Review Letters* **109**, 111601 (2012), DOI `10.1103/PhysRevLett.109.111601`. Its Figure 5 shows the four-dimensional `SU(2)` Wilson-action average plaquette on a `10^4` lattice and compares density-of-states and local-hybrid Monte Carlo results.

Because that comparison is graphical rather than a convenient table with a quoted point uncertainty, the repository froze a conservative **source-resolution envelope** before running the benchmark:

```text
lattice = 10^4
beta = 2.3
source-resolution envelope = [0.59, 0.62]
```

That interval is explicitly **not** represented as a paper error bar. The frozen benchmark must pass the external envelope, independent-chain/ESS gates, and a within-chain post-thermalisation stationarity gate. For each chain, the 48 recorded measurements are split into first/last 24-measurement halves; each half uses the frozen autocorrelation-aware diagnostic with `max_lag=6`, and the drift statistic must satisfy

```text
|mean_first - mean_second| / sqrt(SE_first^2 + SE_second^2) <= 5.
```

On the hardened frozen run the stationarity values were approximately `2.396` and `1.138`, so the maximum was below the frozen threshold. Therefore

```text
published_plaquette_baseline_reproduced = true
phase2_graduation_gate_satisfied       = true
```

The final CI runtime emits the two raw measurement series, derived seeds, per-chain diagnostics, stationarity evidence, and pooled comparison into the commit-tied Actions evidence log.

## Phase 2C1 frozen SU(3) sampler

Phase 2C1 does not extend the frozen `SU(2)` sampler. It defines an independent stochastic identity:

```text
sampler_id  = YM-SU3-ENSEMBLE-V1
proposal_id = YM-SU3-GENERATOR-METROPOLIS-V1
rng_id      = SPLITMIX64-V1
stream_id   = YM-SU3-STREAM-V1
```

At every link, the reference visits the eight frozen Gell-Mann directions in order. One raw `SplitMix64` word defines an exactly paired finite angle grid:

```text
m = raw_u64 & (2^52 - 1)
|theta| = proposal_angle * (2m + 1) / 2^53
sign(theta) = bit 63
R = exp(i theta T^a)
U_candidate = R U_old
A = min(1, exp(-DeltaS)).
```

Toggling bit 63 gives the exact floating inverse angle, the endpoints `+/-proposal_angle` are excluded, and each inverse pair has equal multiplicity. A second PRNG output is consumed for the acceptance draw for every proposal.

The proposal angle is bounded by `1e-8 <= proposal_angle <= 1`; smaller positive values fail closed as numerically inert. Candidate `DeltaS` is evaluated allocation-free from only the six affected plaquettes and is regression-checked against a complete Wilson-action difference.

The frozen binary64 reference admits at most `512` total sweeps and `512` measurements. CI executes the maximum accepted `2^4`, beta-zero, chain-0 schedule at `proposal_angle=1.0` and requires the final link unitarity and determinant residuals to remain below `5e-12`.

Phase 2C1 has its own domain-separated chain-seed derivation, fixed site/direction/generator update order, raw normalized `(1/3)ReTr` average-plaquette measurements, proposal/accept accounting, and final link unitarity/determinant residual gates. The canonical runtime emits the complete raw series, variance, full requested autocorrelation sequence, positive-window lag, `tau_int`, ESS, standard error, configuration/stream identity, and group residuals into the commit-tied Actions log. A literal golden 16-measurement beta-zero chain independently pins those outputs so deterministic semantic drift cannot pass merely by replaying itself.

A `beta=0` all-accept regression checks the transition kernel but is **not** represented as an exact Haar-equilibrium sample.

The Phase 2C1 sampler contract remains mechanically scoped and therefore retains

```text
published_su3_baseline_reproduced = false
glueball_analysis_enabled         = false
phase2c_complete                  = false
```

External validation is owned by the separately versioned Phase 2C2 V2 authority, which is frozen with all three corresponding graduation flags true. Those validation flags must not be back-propagated into `YM-SU3-ENSEMBLE-V1`.

## Lattice-to-continuum boundary

A lattice link approximates the frozen start-left transport along an edge. A plaquette is a finite small-loop holonomy whose smooth small-spacing expansion contains `F_mu_nu`. A finite plaquette is not equal to pointwise continuum curvature, and a finite-lattice ensemble is not a continuum theorem.

## Run the reference checks

Requires the pinned Rust toolchain in [`rust-toolchain.toml`](rust-toolchain.toml).

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
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
```

The unified contract entry points are authoritative for every currently advertised contract surface.

## Repository map

```text
contracts/             machine-readable contract surfaces
docs/                  constitution, mathematical/statistical dictionaries, evidence rules
reference/rust/        deterministic and stochastic reference implementations
test-vectors/          human- and machine-readable regression vectors
tests/                 integration tests
scripts/               fail-closed contract checks
.github/workflows/      CI gates
```

## GLUBALL donor boundary

[`QSOLKCB/GLUBALL`](https://github.com/QSOLKCB/GLUBALL) `v1.0.0` is an **engineering donor**, not a physics donor. See [`docs/DONOR_BOUNDARY.md`](docs/DONOR_BOUNDARY.md).

## Roadmap

The staged research plan is in [`ROADMAP.md`](ROADMAP.md). Phase 2B closes the published `SU(2)` plaquette validation gate. Phase 2C1 freezes a separately versioned `SU(3)` sampler, and Phase 2C2 now freezes its independent published finite-lattice validation after preserving the failed V1 attempt and passing the fresh-stream V2 confirmation. Phase 3 may now begin under separately versioned glueball-analysis authorities.

## Licence

Mozilla Public License 2.0.