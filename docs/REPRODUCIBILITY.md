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

The reference stream is `SPLITMIX64-V1`. Reproducibility records must preserve:

- master seed;
- chain ID;
- derived seed rule;
- exact RNG identifier/version;
- integer-to-`[0,1)` conversion rule.

A seed is not a physical parameter. Distinct chain IDs are distinct stream identities.

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
- measurement count;
- complete Markov sweeps between measurements;
- observable definition;
- raw measurement series;
- autocorrelation maximum lag and estimator convention;
- derived diagnostics, including ESS.

Fixed master seed + chain ID under the same reference implementation/toolchain must replay the same reference measurement series. Independent chain IDs must not be treated as interchangeable replays of one chain.

### Statistical diagnostics

Phase 2A freezes a simple positive-window autocorrelation estimator:

```text
tau_int = 1/2 + sum consecutive positive rho_lag
ESS = min(N, N/(2 tau_int)).
```

The recorded autocorrelation sequence uses the frozen covariance normalization from the contract. These are finite-sample diagnostics, not guarantees that thermalisation or systematic effects are absent.

Raw measurements are retained in the reference `ChainReport`; a future publication or large campaign must archive them directly or by a stable content-addressed evidence reference.

### Beta-zero sanity check

At `beta=0`, the exact ensemble plaquette expectation is zero under Haar measure. Phase 2A uses a fixed-seed `2^4` chain to check that the implemented sampler stays inside a deliberately broad near-zero sanity band while preserving link unitarity and producing nontrivial ESS.

This sanity vector is C2 finite-lattice numerical evidence. It is **not** a substitute for a published finite-beta benchmark.

### Phase 2 graduation boundary

The frozen Phase 2A contract retains:

```text
published_plaquette_baseline_reproduced = false
phase2_graduation_gate_satisfied       = false
su3_ensemble_enabled                   = false.
```

Full Phase 2 graduation requires a separately reviewed comparison to a published pure-gauge plaquette result with matching action, normalization, lattice geometry/volume, coupling regime, observable definition, and uncertainty treatment. Agreement will validate the finite-lattice sampler in that regime; it will not imply confinement, continuum QFT existence, or a mass-gap proof.

## Future ensemble campaigns

Beyond the small bounded reference chain, scientific campaigns should additionally record multiple independent chains, thermalisation diagnostics, chain lengths, measurement thinning/cadence rationale, autocorrelation stability under window changes, uncertainty aggregation across chains, and raw evidence sufficient for independent reanalysis.

## Future HPC work

Accelerator campaigns must additionally record device model, device count, driver/runtime/compiler versions, build flags, precision mode, launch configuration, and acceptance results against the stable CPU/reference implementation. Accelerator throughput does not make accelerator output mathematical or physical authority.
