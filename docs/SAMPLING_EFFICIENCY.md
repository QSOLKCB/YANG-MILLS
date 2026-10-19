# Sampling-efficiency reporting

`YM-SAMPLING-EFFICIENCY-V1` is an additive reporting-mechanics candidate. It changes no frozen sampler, stream, historical evidence, fit window, phase gate, or production recovery. Run:

```sh
cargo run --quiet --locked --bin ym-sampling-efficiency
cargo test --locked --test sampling_efficiency
python3 scripts/check_all_contracts.py
python3 scripts/test_all_contract_validation.py
```

## Why these metrics

Bonanno et al., [arXiv:2610.20747v1](https://arxiv.org/abs/2610.20747v1), Eq. (14) report a normalized weight-efficiency estimator, distinct from serial autocorrelation. Their Figure 5 compares a cost proportional to `2*tau/weight_efficiency` times update work per retained measurement. Eq. (16) additionally contains an observable-variance factor, omitted here. This motivates reporting useful-sample cost instead of throughput alone. This repository does not reproduce their simulation, implement their flow, or import their topological-charge observable.

## API and interpretation

`reference/rust/src/sampling_efficiency.rs::evaluate` takes an ordered post-thermalisation observable series, optional paired finite log weights, a predeclared maximum lag, and three non-overlapping cost components in the same unit: setup/training, thermalisation/discarded work, and retained-sampling work. The API follows this repository's path-module integration pattern; see the executable and integration tests for imports.

- Existing Phase 2A positive-window diagnostics supply `tau` and autocorrelation ESS `N/(2*tau)`. Tau is in retained-measurement intervals, not sweeps.
- Weight-only Kish ESS is `(sum w)^2/sum(w^2)`; reweighting efficiency is that count divided by N. Computation uses max-shifted log weights. Omitted weights mean exactly one. Zero weights can be approximated by finite negative log weights, but infinities are rejected.
- The combined count `autocorrelation_ess * reweighting_efficiency` is explicitly a comparison proxy. Multiplication does not establish the actual variance/ESS of a correlated self-normalized weighted estimator. It omits dependence between weights and observable; no weighted uncertainty or reweighted mean is supplied. Scientific comparisons need observable-specific weighted-estimator covariance analysis.
- Sampling-only cost and total cost divided by that proxy are both reported. Setup and discarded work are never silently amortized away. The caller declares the unit and scope; costs must be finite, nonnegative, and sampling cost positive.

Predeclare the diagnostic series, measurement cadence, lag and cost scope. Keep independent chains separate. Inspect the full autocorrelation and positive-window lag; a window reaching the supplied maximum is truncated, and a short positive window is not proof of equilibration. Constant or numerically zero-variance series are rejected instead of advertising perfect mixing. Longer-lag sensitivity, stationarity, independent streams, and observable-specific validation remain necessary. Plaquette autocorrelation does not diagnose topological freezing.

## Executed reference surface

The executable reruns only the small, already-frozen Phase 2C1 beta-zero mechanics chain: `2^4`, 4 discarded sweeps and 16 measured sweeps. It preserves the full config, stream, raw series, autocorrelation, and diagnostic identities in its output. Work is counted as generator proposals: 512 per sweep, 2,048 discarded and 8,192 sampling proposals, totaling 10,240. Setup has zero generator proposals. This count excludes analysis and IO and is not wall-clock cost; it cannot compare hardware or different update kernels. The chain is not claimed thermalised or Haar-equilibrated. No flow exists here, so its weight efficiency is exactly one.

The analytic fixtures exercise unequal weights, common log-weight shifts, serial correlations, cost accounting, invalid inputs, and degenerate observations. They are mechanics tests, not physics evidence. Historical negative spectroscopy remains negative. These reports establish neither a topology-sampling improvement nor confinement, a continuum theory, or a mass-gap proof.
