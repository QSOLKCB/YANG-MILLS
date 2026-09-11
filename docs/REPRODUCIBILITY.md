# Reproducibility

## Phase 0

The initial reference implementation is intentionally dependency-free beyond the Rust standard library and pins a Rust toolchain in `rust-toolchain.toml`.

Required checks:

```bash
cargo test --all-targets
cargo run --quiet --bin ym-su2-reference
python3 scripts/check_contracts.py
```

The Phase 0 executable uses no random numbers, wall-clock time, network input, thread scheduling, or hardware-specific acceleration.

## Floating-point boundary

The reference uses IEEE-754 `f64` arithmetic through Rust. Phase 0 does not claim bit-identical results across every compiler/architecture. Tests use explicit tolerances where operations are not algebraically exact in machine arithmetic.

Analytic vectors should preferentially use values such as `0`, `1`, and exact integer combinations where possible, but this preference is not a substitute for an explicit numerical contract.

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
