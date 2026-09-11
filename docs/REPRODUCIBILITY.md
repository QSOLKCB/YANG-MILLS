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
cargo run --quiet --locked --bin ym-path-holonomy
cargo run --quiet --locked --bin ym-classical-geometry
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

Phase 1D separates classical path definitions from discrete numerical transport:

- continuum path, pullback, transport, endpoint-covariance, reversal, concatenation, reparameterization, and small-loop statements are classical mathematical definitions/rules;
- exact finite-lattice vectors built from identity links or exact permutation fixtures may be C1;
- floating-point `SU(3)` path composition, reversal, endpoint covariance, closed-loop gauge invariance, cyclic trace, and Polyakov gauge-invariance residuals are C2;
- compatibility with frozen `YM-WILSON-LOOP-V1` step semantics is a C0 contract/implementation fact.

## Phase 1E classical geometry boundary

`YM-CLASSICAL-GEOMETRY-V1` is primarily a **semantic mathematical contract**. The executable Rust component does not claim to simulate a manifold, principal bundle, covariant derivative, curvature PDE, or continuum Yang–Mills dynamics.

The Rust fixture is only an oriented Euclidean `R^4` 2-form convention check. It verifies:

- the frozen Hodge-star basis signs;
- `*^2=+1` on 2-forms;
- `alpha wedge *beta = <alpha,beta> vol` for the declared orthonormal fixture;
- self-dual/anti-self-dual decomposition and orthogonality;
- fail-closed rejection of non-finite fixture coefficients.

These are C0 implementation/convention regressions. They are not continuum numerical observations, and they are not a proof of the general bundle theorems merely because they agree with the frozen formulas.

The principal-bundle, associated-bundle, covariant-derivative, curvature, Bianchi, Hodge, Yang–Mills functional, self-duality, and lattice-continuum statements are frozen as definitions/convention rules in the contract and `docs/CLASSICAL_CONTINUUM.md`. Reproducibility here means that every advertised convention is explicit, versioned, machine-checked for semantic drift, and backed by finite-dimensional sign regressions where executable checks are meaningful.

The Phase 1E contract also freezes the bridge

```text
U_gamma = P_gamma^-1
```

between Phase 1D's start-left transporter and conventional forward associated-fiber transport. This is a semantic compatibility rule, not a new physical measurement.

No Phase 1E result constructs the interacting quantum theory, performs an infinite-volume/continuum limit, establishes confinement, or proves a positive mass gap.

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
