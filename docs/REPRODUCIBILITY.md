# Reproducibility

## Reference surfaces

The reference implementation is intentionally dependency-free beyond the Rust standard library and pins a Rust toolchain in `rust-toolchain.toml`.

The complete currently advertised local readiness checks are:

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
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
```

The unified contract entry points are authoritative. A phase that adds a machine-readable contract or vector set must register it there before the phase can be called reproducible or review-ready.

## Floating-point boundary

The reference uses IEEE-754 `f64` arithmetic through Rust. It does not claim bit-identical floating results across every compiler/architecture. Tests use explicit tolerances where operations are not algebraically exact in machine arithmetic.

The integer `SPLITMIX64-V1` pseudorandom stream is exactly versioned; floating transcendental functions used to turn that stream into group-valued proposals are not advertised as universally bit-identical across platforms.

## Deterministic Phase 0–1E surfaces

Earlier phases separate exact contract facts from floating numerical regressions. The Phase 1E Rust fixture is only an oriented Euclidean `R^4` Hodge/sign check; it is not a continuum manifold or PDE solver. The principal-bundle, curvature, Bianchi, Yang–Mills, and transport statements are versioned mathematical/convention rules, not quantum-construction results.

## Phase 2A stochastic reproducibility

`YM-SU2-ENSEMBLE-V1` freezes the complete reference identity needed to replay or compare a finite-lattice `SU(2)` chain.

### Pseudorandom stream

The reference stream is `SPLITMIX64-V1`. Reproducibility records must preserve master seed, chain ID, derived-seed rule, exact RNG identifier/version, and integer-to-`[0,1)` conversion rule. A seed is not a physical parameter. Distinct chain IDs are distinct stream identities.

### Sampler identity

A Phase 2A chain must record:

- `YM-SU2-ENSEMBLE-V1` contract identity;
- lattice extents and Wilson-action `beta`;
- cold-start policy;
- deterministic site/direction update ordering;
- heat-bath conditional distribution and reconstruction convention;
- heat-bath rejection-attempt bound;
- number of over-relaxation sweeps after each heat-bath sweep;
- discarded thermalisation sweep count;
- measurement count and cadence;
- observable definition and raw measurement series;
- autocorrelation estimator identity and requested lag;
- derived diagnostics, including ESS.

Fixed master seed + chain ID under the same reference implementation/toolchain must replay the same reference measurement series. Independent chain IDs must not be treated as interchangeable replays of one chain.

### Statistical diagnostics

Phase 2A freezes

```text
tau_int = 1/2 + sum consecutive positive rho_lag
ESS = min(N, N/(2 tau_int)).
```

The recorded autocorrelation sequence uses the frozen covariance normalization from the contract. These are finite-sample diagnostics, not guarantees that thermalisation or systematic effects are absent.

### Beta-zero sanity check

At `beta=0`, the exact ensemble plaquette expectation is zero under Haar measure. Phase 2A uses a fixed-seed `2^4` chain to check a broad near-zero sanity band while preserving link unitarity and nontrivial ESS. This C2 vector is not a substitute for an external finite-beta benchmark.

## Phase 2B published-baseline evidence

`YM-SU2-PLAQUETTE-BASELINE-V1` freezes the published finite-lattice validation surface.

The peer-reviewed primary source is Langfeld, Lucini & Rago, *Physical Review Letters* **109**, 111601 (2012), DOI `10.1103/PhysRevLett.109.111601`. The matching surface is four-dimensional `SU(2)` Wilson gauge theory on a `10^4` lattice at `beta=2.3`, with average plaquette

```text
(1/(6V)) sum_x sum_(mu<nu) (1/2) ReTr P_mu_nu(x).
```

Because the source comparison is graphical, the predeclared `[0.59,0.62]` interval is a conservative **source-resolution envelope** and must not be reported as a quoted paper statistical error bar.

The frozen Phase 2B run uses master seed `2b00201211160101`, chain IDs `0` and `1`, 40 discarded thermalisation sweeps, 48 measurements per chain, one complete Markov sweep between measurements, one over-relaxation sweep per heat-bath sweep, and autocorrelation lag 12.

The first predeclared external-comparison run produced:

```text
chain 0 mean = 0.6026867250625984
chain 1 mean = 0.6018972107756385
pooled mean  = 0.6022919679191184
pooled diagnostic SE ~= 0.000417328609449
chain-consistency z ~= 0.9459144054
minimum chain ESS ~= 7.74585617
maximum link-unit residual ~= 6.66e-16
```

This lies inside the frozen published source-resolution envelope and satisfies the independent-chain gates. Because both chains share the cold identity start, that agreement alone is not treated as an equilibration proof.

### Post-thermalisation stationarity gate

The hardened Phase 2B authority additionally requires a within-chain stationarity check for both reference chains. Each 48-measurement post-thermalisation series is split into first/last 24-measurement halves. Each half is analyzed with the frozen Phase 2A autocorrelation-aware diagnostic using `max_lag=6`, and Phase 2B records

```text
stationarity_z = |mean_first - mean_second|
                 / sqrt(SE_first^2 + SE_second^2).
```

Both chains must satisfy `stationarity_z <= 5`. This directly checks for detectable drift remaining after the declared 40 discarded thermalisation sweeps. It is still a bounded diagnostic rather than proof that all equilibration or systematic errors are absent.

The stationarity split, lag, threshold, half-means, half standard errors, and drift-z values are part of the frozen benchmark evidence and may not be silently omitted or weakened.

### Evidence archive

The canonical `ym-su2-plaquette-baseline` runtime emits into the commit-tied GitHub Actions log:

- source contract and DOI;
- lattice/coupling and source envelope;
- master seed, chain IDs, and derived seeds;
- full raw measurement series for both chains;
- per-chain means, standard errors, ESS, integrated autocorrelation times, and heat-bath acceptance;
- first-half/second-half stationarity means and drift-z statistics;
- pooled mean/diagnostic standard error and chain-consistency statistic;
- maximum final link-unit residual;
- explicit Phase 2 graduation and claim-boundary flags.

Thus the Actions run itself is a source-commit/toolchain-bound evidence record. Any scientific release should additionally preserve those measurements as an explicit artifact or content-addressed dataset rather than relying only on CI retention.

Phase 2B freezes

```text
published_plaquette_baseline_reproduced = true
phase2_graduation_gate_satisfied       = true
```

only because the external envelope, independent-chain, within-chain stationarity, ESS, acceptance, and group-invariant gates all passed on the frozen head.

This agreement validates the finite-lattice `SU(2)` sampler in the declared regime. It does not imply confinement, continuum QFT existence, or a mass-gap proof.

## Phase 2C1 `SU(3)` sampler reproducibility

`YM-SU3-ENSEMBLE-V1` and `SU3-ENSEMBLE-PHASE2C1-V1` are frozen as `phase-2c1-frozen`, independently versioned from the Phase 2A sampler. Reusing the `SPLITMIX64-V1` core and Phase 2A diagnostic arithmetic does not reuse the `SU(2)` stream identity or external benchmark authority.

A Phase 2C1 evidence record must preserve:

- sampler ID `YM-SU3-ENSEMBLE-V1`;
- proposal ID `YM-SU3-GENERATOR-METROPOLIS-V1`;
- RNG ID `SPLITMIX64-V1`;
- stream ID `YM-SU3-STREAM-V1` and its domain separator;
- master seed, chain ID, and derived seed;
- lattice extents, Wilson `beta`, and proposal angle;
- cold-start policy;
- lexicographic site/direction/generator update order;
- exactly two uniform draws per generator proposal;
- discarded thermalisation sweep count and measurement cadence;
- total Metropolis proposal and accept counts;
- complete raw normalized `(1/3)ReTr` average-plaquette series;
- autocorrelation/ESS diagnostic settings and results;
- final maximum link unitarity and determinant residuals.

The bounded reference requires `1e-8 <= proposal_angle <= 1`. Positive values below the floor fail closed as numerically inert; the floor is a binary64 reference-resolution guard rather than a physical scale.

Candidate-link local action differences are computed allocation-free from the six affected plaquettes. Replacing that path with a full-lattice clone per proposal changes the computational/fail-closed contract and is forbidden under the same identifier.

For master seed `3c00198212230101`, the golden Phase 2C1 derived seeds are

```text
chain 0 = 94c715630ee8e90a
chain 1 = 8aba19af6781dc09
```

Fixed chain identity under the same reference/toolchain must replay identical proposal accounting, raw measurements, diagnostics, and residuals. Distinct chain IDs must produce distinct stream identities and evidence.

The `beta=0` all-accept vector validates the transition-kernel accounting only. It is not represented as an exact finite-sample Haar-equilibrium result.

Phase 2C1 must retain

```text
published_su3_baseline_reproduced = false
glueball_analysis_enabled         = false
phase2c_complete                  = false
```

until Phase 2C2 predeclares and reproduces an independent peer-reviewed `SU(3)` finite-lattice benchmark. Scientific campaigns beyond the reference vectors should record multiple independent chains, thermalisation diagnostics, cadence rationale, autocorrelation stability under window changes, uncertainty aggregation across chains, and raw evidence sufficient for independent reanalysis.

## Future HPC work

Accelerator campaigns must additionally record device model, device count, driver/runtime/compiler versions, build flags, precision mode, launch configuration, and acceptance results against the stable CPU/reference implementation. Accelerator throughput does not make accelerator output mathematical or physical authority.