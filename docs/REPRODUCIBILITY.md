# Reproducibility

## Deterministic reference surfaces

The reference implementation is intentionally dependency-free beyond the Rust standard library and pins a Rust toolchain in `rust-toolchain.toml`.

The complete currently advertised local readiness checks are:

```bash
cargo fmt --all -- --check
cargo test --all-targets --locked
cargo run --quiet --locked --bin ym-su2-reference
cargo run --quiet --locked --bin ym-su3-algebra
cargo run --quiet --locked --bin ym-su3-lattice
cargo run --quiet --locked --bin ym-su3-center
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
```

The unified contract entry points are authoritative. A phase that adds a new machine-readable contract or vector set must register it there before the phase can be called reproducible or review-ready.

The deterministic reference executables use no random numbers, wall-clock time, network input, thread scheduling, or hardware-specific acceleration.

## Floating-point boundary

The reference uses IEEE-754 `f64` arithmetic through Rust. It does not claim bit-identical results across every compiler/architecture. Tests use explicit tolerances where operations are not algebraically exact in machine arithmetic.

Analytic vectors should preferentially use values such as `0`, `1`, exact integer combinations, or independently specified closed forms where possible, but this preference is not a substitute for an explicit numerical contract.

Phase 1B's standalone `SU(3)` matrix exponential is a bounded numerical reference calculation. The exact mathematical map is `U(theta)=exp(i theta_a T^a)`, while reproduced floating-point matrix/residual outputs remain implementation facts with declared tolerances rather than finite-lattice exact results.

The Phase 1B `SU(3)` lattice layer deliberately separates exact and numerical regressions:

- the identity action and the single exact three-cycle-link combinatorial action are C1 finite-lattice facts;
- local gauge-invariance residuals and transformed-link residuals are C2 numerical observations with explicit tolerances.

Phase 1C likewise separates exact abstract/embedding checks from numerical center-transform observations:

- `C3` modular composition, `Omega^3=I`, and the exact non-central witness are C0 contract/group regressions;
- floating-point scalar-center residuals remain C0 implementation facts with explicit tolerances;
- Polyakov-loop rotation and Wilson-action invariance after a center slice transformation are C2 finite-lattice numerical observations.

No Phase 1C vector is an ensemble measurement, confinement/deconfinement result, or spontaneous-symmetry-breaking observation.

## Future stochastic work

Before Monte Carlo work is considered reproducible, records must include:

- pseudorandom generator algorithm and version;
- complete seed/stream identity;
- update algorithm and ordering;
- thermalisation policy;
- sweep/measurement cadence;
- lattice/action parameters;
- autocorrelation diagnostics;
- independent-chain strategy;
- raw measurements or stable content-addressed references.

## Future HPC work

Accelerator campaigns must additionally record device model, device count, driver/runtime/compiler versions, build flags, precision mode, launch configuration, and acceptance results against the stable reference implementation.
