# Published SU(3) plaquette baseline — Phase 2C2

Phase 2C2 is the **independent finite-lattice validation gate** for the frozen Phase 2C1 `SU(3)` sampler. It does not modify `YM-SU3-ENSEMBLE-V1`, and it does not inherit the Phase 2B `SU(2)` benchmark result.

Phase 2C2 preserves both experimental outcomes:

- `YM-SU3-PLAQUETTE-BASELINE-V1` is the first predeclared attempt and remains frozen as a **failed statistical gate**;
- `YM-SU3-PLAQUETTE-BASELINE-V2` is a fresh-stream confirmatory protocol and is frozen as the successful Phase 2C2 validation authority.

V1 is not retroactively reclassified as a pass.

## Primary source

R. C. Edgar, L. McCrossen and K. J. M. Moriarty, “The specific heat of SU(3) lattice gauge theory,” *Journal of Physics G: Nuclear Physics* **7** (1981) L85–L88, DOI `10.1088/0305-4616/7/5/001`.

The paper studies four-dimensional pure `SU(3)` lattice gauge theory with the Wilson action, reports the average action per plaquette as a function of `beta`, and uses both `4^4` and `5^4` lattices. The `4^4` calculation averages over 400 iterations through the lattice.

The source presents the beta scan graphically rather than as a table of per-point means and uncertainties. Phase 2C2 therefore freezes a **conservative graphical source-resolution envelope**, not a fabricated statistical error bar.

## Frozen comparison surface

Both V1 and V2 use the same external target:

```text
gauge_group = SU(3)
action = Wilson
boundary_conditions = periodic
lattice = 4 x 4 x 4 x 4
beta = 6.0
observable = (1/(6V)) sum_x sum_mu<nu (1/3) ReTr P_mu_nu(x)
source_resolution_envelope = [0.56, 0.63]
source_envelope_is_paper_error_bar = false
proposal_angle = 0.35
thermalization_sweeps = 192
measurements_per_chain = 128
sweeps_between_measurements = 2
total_sweeps_per_chain = 448
autocorrelation_max_lag = 24
stationarity_half_measurements = 64
stationarity_autocorrelation_max_lag = 12
maximum_stationarity_z = 5
minimum_metropolis_acceptance = 0.10
maximum_metropolis_acceptance = 0.95
expected_metropolis_proposals_per_chain = 3670016
maximum_link_unitarity_residual = 5e-12
maximum_link_determinant_residual = 5e-12
```

The 448-sweep per-chain schedule remains inside the frozen Phase 2C1 maximum of 512 total sweeps. Neither V1 nor V2 relaxes the Phase 2C1 sampler authority.

## V1 — preserved failed predeclared attempt

V1 froze two independent chains before execution:

```text
contract = YM-SU3-PLAQUETTE-BASELINE-V1
master_seed = 2c02198105010001
chain_ids = [0, 1]
derived_seeds = [2ff3bf48550d1457, ef7636a56e8ea29a]
minimum_ess_per_chain = 8
maximum_chain_consistency_z = 5
```

GitHub Actions run `34760576995` produced:

```text
pooled_mean = 0.6018083560823128
source_envelope_reproduced = true
minimum_effective_sample_size = 4.51005377363487
required_minimum_effective_sample_size = 8
chain_consistency_z = 0.4307895037511404
maximum_stationarity_z = 3.7834285113688733
chain_acceptances = [0.8674455370, 0.8678583418]
```

The published plaquette surface, chain consistency, stationarity, acceptance, accounting, and group diagnostics were acceptable, but one chain failed the **predeclared per-chain ESS floor**. Therefore V1 remains:

```text
status = phase-2c2-v1-failed
published_su3_baseline_reproduced = false
phase2c_complete = false
glueball_analysis_enabled = false
failed_gate = minimum_effective_sample_size_per_chain
```

The threshold was not reduced after seeing the result.

## V2 — fresh-stream confirmatory protocol

After V1 failed only its statistical-power gate, V2 was committed **before any V2 result was inspected**. It retained the source surface, source envelope, sampler, proposal scale, per-chain schedule, and diagnostic thresholds while introducing a fresh master seed and four independent streams.

```text
contract = YM-SU3-PLAQUETTE-BASELINE-V2
master_seed = 2c02198105010002
chain_ids = [0, 1, 2, 3]
derived_seeds = [
  dfcdab179476617a,
  151dd841ccef81c6,
  9c28ededca3d1ae2,
  17dbbf4a00214611
]
minimum_aggregate_effective_sample_size = 16
maximum_pairwise_chain_consistency_z = 5
```

The aggregate ESS floor of 16 preserves V1's originally intended minimum total information target of `2 chains x ESS 8`. It does not lower the failed V1 threshold or reuse the failed streams.

## Frozen V2 evidence

GitHub Actions run `34761701867`, on evidence head `f5a221fdad2046779358cc59460fae488626459b`, executed the four fresh chains and passed the predeclared V2 gate:

```text
pooled_mean = 0.5992920042685975
pooled_standard_error = 0.0008268453534905086

chain_means = [
  0.5951228942049566,
  0.6010578916054883,
  0.6006811934765629,
  0.6003060377873822
]

chain_effective_sample_sizes = [
  37.108298427827265,
  12.12458490228564,
  19.780522239783117,
  9.930599191453949
]

aggregate_effective_sample_size = 78.94400476134997
required_aggregate_effective_sample_size = 16
maximum_pairwise_chain_consistency_z = 3.5504261086461475
maximum_stationarity_z = 1.4650733209908853
maximum_link_unitarity_residual = 3.1308289294429414e-14
maximum_link_determinant_residual = 3.553115420753491e-14
source_envelope_reproduced = true
```

All four acceptance fractions were approximately `0.8675`, comfortably inside the frozen diagnostic window `[0.10, 0.95]`. Exact proposal accounting was `3,670,016` proposals per chain.

The frozen V2 authority therefore records:

```text
status = phase-2c2-frozen
published_su3_baseline_reproduced = true
phase2c_complete = true
glueball_analysis_enabled = true
result_status = passed-predeclared-v2-confirmation
```

`glueball_analysis_enabled=true` authorizes Phase 3 to begin. It is **not** a glueball result.

## Fail-closed interpretation

A successful Phase 2C2 validation requires the external source envelope and the independently declared numerical/statistical gates to pass together. Raw measurements are retained, diagnostics are recomputed from those measurements, proposal accounting must match the frozen schedule, chain identities must remain independent, and link residuals must stay within the Phase 2C1 numerical limits.

The repository preserves the failed V1 attempt because the ordering matters scientifically: predeclare the test, execute it, record failure honestly, then predeclare a distinct confirmatory protocol rather than editing the original gate after inspection.

## Claim boundary

The successful V2 result is finite-lattice validation of the frozen `SU(3)` sampler in the declared regime. It does **not**:

- prove confinement;
- construct the interacting continuum quantum Yang–Mills theory;
- prove a positive continuum mass gap; or
- itself provide a glueball mass measurement.

Phase 3 may now introduce separately versioned operator, correlator, smearing/blocking, fitting, finite-volume, scale-setting, and extrapolation authorities. Those future results must remain distinct from the Clay existence-and-mass-gap claim.
