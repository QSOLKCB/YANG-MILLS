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
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
```

The unified contract entry points are authoritative. A phase that adds a machine-readable contract or vector set must register it there before the phase can be called reproducible or review-ready.

## Floating-point boundary

The reference uses IEEE-754 `f64` arithmetic through Rust. It does not claim bit-identical floating results across every compiler/architecture. Tests use explicit tolerances where operations are not algebraically exact in machine arithmetic.

The integer `SPLITMIX64-V1` pseudorandom stream is exactly versioned; floating transcendental functions used to turn that stream into Haar directions are not advertised as universally bit-identical across platforms.

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

The first predeclared candidate run produced:

```text
chain 0 mean = 0.6026867250625984
chain 1 mean = 0.6018972107756385
pooled mean  = 0.6022919679191184
pooled diagnostic SE ~= 0.000417328609449
chain-consistency z ~= 0.9459144054
minimum chain ESS ~= 7.74585617
maximum link-unit residual ~= 6.66e-16
```

This lies inside the frozen published source-resolution envelope and satisfies all declared chain-diagnostic gates.

### Evidence archive

The canonical `ym-su2-plaquette-baseline` runtime emits into the commit-tied GitHub Actions log:

- source contract and DOI;
- lattice/coupling and source envelope;
- master seed, chain IDs, and derived seeds;
- full raw measurement series for both chains;
- per-chain means, standard errors, ESS, integrated autocorrelation times, and heat-bath acceptance;
- pooled mean/diagnostic standard error and chain-consistency statistic;
- maximum final link-unit residual;
- explicit Phase 2 graduation and claim-boundary flags.

Thus the Actions run itself is a source-commit/toolchain-bound evidence record. Any scientific release should additionally preserve those measurements as an explicit artifact or content-addressed dataset rather than relying only on CI retention.

Phase 2B freezes

```text
published_plaquette_baseline_reproduced = true
phase2_graduation_gate_satisfied       = true
su3_ensemble_enabled                   = false
```

This agreement validates the finite-lattice `SU(2)` sampler in the declared regime. It does not imply confinement, continuum QFT existence, or a mass-gap proof.

## Future ensemble campaigns

A future `SU(3)` generator requires a separately versioned sampler contract and independent review. Larger scientific campaigns should record multiple independent chains, thermalisation diagnostics, chain lengths, cadence rationale, autocorrelation stability under window changes, uncertainty aggregation across chains, and raw evidence sufficient for independent reanalysis.

## Future HPC work

Accelerator campaigns must additionally record device model, device count, driver/runtime/compiler versions, build flags, precision mode, launch configuration, and acceptance results against the stable CPU/reference implementation. Accelerator throughput does not make accelerator output mathematical or physical authority.
