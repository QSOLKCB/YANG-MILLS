# Published SU(3) plaquette baseline — Phase 2C2

Phase 2C2 is an **independent finite-lattice validation gate** for the frozen Phase 2C1 `SU(3)` sampler. It does not modify `YM-SU3-ENSEMBLE-V1` and it does not inherit the Phase 2B `SU(2)` benchmark result.

## Primary source

R. C. Edgar, L. McCrossen and K. J. M. Moriarty, “The specific heat of SU(3) lattice gauge theory,” *Journal of Physics G: Nuclear Physics* **7** (1981) L85–L88, DOI `10.1088/0305-4616/7/5/001`.

The paper studies four-dimensional pure `SU(3)` lattice gauge theory with the Wilson action, reports the average action per plaquette as a function of `beta`, and uses both `4^4` and `5^4` lattices. The `4^4` calculation averages over 400 iterations through the lattice.

The source presents this beta scan graphically rather than as a table of per-point means and uncertainties. The Phase 2C2 authority therefore freezes a **conservative graphical source-resolution envelope**, not a fabricated statistical error bar.

## Selected finite-lattice surface

The predeclared comparison surface is:

```text
gauge_group = SU(3)
action = Wilson
boundary_conditions = periodic
lattice = 4 x 4 x 4 x 4
beta = 6.0
observable = (1/(6V)) sum_x sum_mu<nu (1/3) ReTr P_mu_nu(x)
source_resolution_envelope = [0.56, 0.63]
source_envelope_is_paper_error_bar = false
```

This is deliberately a finite-lattice benchmark. Agreement is evidence that the frozen sampler reproduces a peer-reviewed Wilson-action plaquette surface in the declared regime; it is not a continuum extrapolation.

## Predeclared executable run

The following parameters were committed before inspecting the reference result:

```text
sampler_id = YM-SU3-ENSEMBLE-V1
proposal_id = YM-SU3-GENERATOR-METROPOLIS-V1
rng_id = SPLITMIX64-V1
stream_id = YM-SU3-STREAM-V1
master_seed = 2c02198105010001
chain_ids = [0, 1]
derived_seeds = [2ff3bf48550d1457, ef7636a56e8ea29a]
proposal_angle = 0.35
thermalization_sweeps = 192
measurements_per_chain = 128
sweeps_between_measurements = 2
total_sweeps_per_chain = 448
autocorrelation_max_lag = 24
stationarity_half_measurements = 64
stationarity_autocorrelation_max_lag = 12
minimum_ess_per_chain = 8
maximum_chain_consistency_z = 5
maximum_stationarity_z = 5
minimum_metropolis_acceptance = 0.10
maximum_metropolis_acceptance = 0.95
expected_metropolis_proposals_per_chain = 3670016
maximum_link_unitarity_residual = 5e-12
maximum_link_determinant_residual = 5e-12
```

The 448-sweep schedule stays inside the frozen Phase 2C1 maximum of 512 total sweeps. No relaxation of the Phase 2C1 sampler authority is needed for the published validation run.

## Fail-closed comparison rule

A successful Phase 2C2 run requires all of the following at once:

1. both reports match the frozen Phase 2C2 geometry, coupling, proposal, schedule, chain identities, and deterministic derived seeds;
2. raw measurements from both chains are retained and diagnostics are recomputed from those measurements;
3. exact Metropolis proposal accounting matches the predeclared 4D `4^4` schedule;
4. the measured acceptance fraction for each chain lies in the predeclared diagnostic window;
5. every plaquette sample lies in the physical normalized `SU(3)` trace range;
6. each chain meets the effective-sample-size floor;
7. the independent chain means satisfy the predeclared consistency-z gate;
8. each chain passes the within-chain first-half/second-half stationarity gate;
9. final link unitarity and determinant-one residuals remain below the frozen Phase 2C1 numerical limits; and
10. the pooled two-chain mean lies inside the published source-resolution envelope.

No single passing statistic can override a failure elsewhere.

## Candidate-to-frozen transition

The first committed Phase 2C2 authority intentionally records:

```text
published_su3_baseline_reproduced = false
phase2c_complete = false
glueball_analysis_enabled = false
result_status = pending-executable-evidence
```

Those flags may change only after the already-predeclared executable run passes without changing the benchmark parameters. This makes the ordering auditable in Git history: **freeze the test first, then inspect the result**.

## Claim boundary

Even after a successful Phase 2C2 validation:

- finite-lattice benchmark agreement does not prove confinement;
- it does not construct the continuum quantum Yang–Mills theory;
- it does not prove a positive continuum mass gap; and
- authorization to begin glueball analysis is not itself a glueball result.

Phase 3 must introduce its own separately versioned operator, correlator, fitting, finite-volume, scale-setting, and extrapolation authorities before stronger physical claims can be considered.
