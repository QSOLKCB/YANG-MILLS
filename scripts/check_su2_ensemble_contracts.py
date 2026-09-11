#!/usr/bin/env python3
"""Fail-closed exact checks for the frozen Phase 2A SU(2) ensemble surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

CONTRACT_PATH = "contracts/YM-SU2-ENSEMBLE-V1.json"
VECTOR_PATH = "test-vectors/su2-ensemble-v1.json"

EXPECTED_ENSEMBLE_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-SU2-ENSEMBLE-V1",
    "status": "phase-2a-frozen",
    "scope": "bounded reproducible SU(2) Wilson-action Markov-chain ensemble generation and diagnostics",
    "depends_on": [
        {"contract_id": "YM-SU2-LATTICE-V1", "required_status": "phase-1a-frozen"},
        {"contract_id": "YM-WILSON-LOOP-V1", "required_status": "phase-1a-frozen"},
    ],
    "literature_basis": {
        "heatbath_reference": "Michael Creutz, Phys. Rev. D 21, 2308 (1980), DOI 10.1103/PhysRevD.21.2308",
        "overrelaxation_reference": "Frank R. Brown and Thomas J. Woch, Phys. Rev. Lett. 58, 2394 (1987), DOI 10.1103/PhysRevLett.58.2394",
        "implementation_claim": "the code independently implements the stated local conditional density and microcanonical reflection under the frozen repository conventions",
    },
    "rng": {
        "id": "SPLITMIX64-V1",
        "state_transition": "state=state+0x9e3779b97f4a7c15 mod 2^64 followed by the frozen SplitMix64 mixing function",
        "uniform_f64": "high 53 output bits multiplied by 2^-53, yielding [0,1)",
        "cryptographic_rng": False,
        "seed_is_physical_parameter": False,
        "same_seed_same_chain_id_replays_same_reference_stream": True,
        "cross_platform_bitwise_floating_identity_claimed": False,
    },
    "chain_identity": {
        "fields": [
            "master_seed",
            "chain_id",
            "derived_seed",
            "extents",
            "beta",
            "thermalization_sweeps",
            "measurements",
            "sweeps_between_measurements",
            "overrelaxation_sweeps_per_heatbath",
            "max_heatbath_attempts_per_link",
            "autocorrelation_max_lag",
        ],
        "derived_seed_rule": "mix64(master_seed XOR ((chain_id+1)*0x9e3779b97f4a7c15 mod 2^64))",
        "distinct_chain_ids_are_distinct_stream_identities": True,
    },
    "reference_bounds": {
        "beta_min": 0.0,
        "beta_max": 8.0,
        "max_schedule_sweeps": 1048576,
        "max_measurements": 1048576,
        "max_overrelaxation_sweeps_per_heatbath": 16,
        "total_complete_markov_sweeps_must_be_at_most_max_schedule_sweeps": True,
        "heatbath_attempt_limit_must_be_positive": True,
        "autocorrelation_max_lag_must_be_less_than_measurements": True,
    },
    "update_order": {
        "site_order": "lexicographic x0,x1,x2,x3",
        "direction_order": "mu=0,1,2,3 at each site",
        "markov_sweep": "one complete heatbath sweep followed by the configured number of complete overrelaxation sweeps",
    },
    "su2_heatbath": {
        "staple": "V=sum of the six oriented staples from frozen YM-SU2-LATTICE-V1",
        "factorization": "V=k Vhat with k=sqrt(sum_i V_i^2) and Vhat in SU(2) when k>0",
        "local_variable": "R=U Vhat",
        "conditional_density": "dP(R) proportional to exp(alpha r0) d_mu_Haar(R), alpha=beta*k",
        "scalar_proposal": "r0 is proposed from the truncated exponential density proportional to exp(alpha r0) on [-1,1], with the uniform alpha->0 floating-point limit used for alpha<=1e-300",
        "alpha_zero_limit": 1e-300,
        "alpha_zero_limit_reason": "avoid subnormal inverse-CDF quantization where the physical tilt is far below binary64 resolution",
        "haar_jacobian_acceptance": "accept scalar proposal with probability sqrt(1-r0^2)",
        "vector_direction": "conditional on accepted r0, vector direction is uniform on S^2",
        "link_reconstruction": "U_new=R Vhat^dagger; when k=0 use U_new=R",
        "bounded_rejection": "failure to accept within max_heatbath_attempts_per_link returns an error",
    },
    "su2_overrelaxation": {
        "formula": "U_new=Vhat^dagger U_old^dagger Vhat^dagger for nonzero staple norm",
        "zero_staple_rule": "leave link unchanged",
        "local_wilson_action_preserved": True,
        "purpose": "microcanonical decorrelation step; it is not an ergodic sampler by itself",
    },
    "schedule": {
        "initial_configuration": "cold identity lattice",
        "thermalization": "discard thermalization_sweeps complete Markov sweeps before measurements",
        "measurement_cadence": "perform sweeps_between_measurements complete Markov sweeps before each recorded measurement",
        "total_sweep_count": "thermalization_sweeps + measurements*sweeps_between_measurements, checked for overflow and bounded by max_schedule_sweeps",
        "observable": "average plaquette=(1/(6V))*sum_x,sum_mu<nu (1/2)ReTr P_mu_nu(x)",
    },
    "autocorrelation_diagnostics": {
        "mean": "arithmetic mean of recorded plaquette measurements",
        "variance": "population variance using divisor N",
        "rho_lag": "C_lag/C_0 with C_lag using divisor N-lag",
        "window": "sum positive rho_lag values from lag 1 until the first non-positive lag; later lags are reported but not added",
        "tau_int": "0.5 + sum of positive-window rho_lag",
        "effective_sample_size": "min(N,N/(2*tau_int))",
        "standard_error": "sqrt(variance/effective_sample_size)",
        "constant_series_rule": "tau_int=0.5, ESS=N, standard_error=0",
    },
    "reproducibility": {
        "fixed_chain_replay_required": True,
        "independent_chain_ids_required": True,
        "thermalization_metadata_required": True,
        "measurement_cadence_metadata_required": True,
        "autocorrelation_and_ess_required": True,
        "raw_measurement_series_retained_in_reference_report": True,
    },
    "phase2_gate": {
        "beta_zero_haar_sanity_check_enabled": True,
        "published_plaquette_baseline_reproduced": False,
        "published_baseline_freeze_requires_matching_action_volume_and_parameter_regime": True,
        "phase2_graduation_gate_satisfied": False,
        "su3_ensemble_enabled": False,
    },
    "claim_boundary": {
        "ensemble_measurements_are_finite_lattice_numerical_observations": True,
        "single_seed_is_physical_truth": False,
        "small_autocorrelation_implies_independent_physics": False,
        "plaquette_agreement_implies_confinement": False,
        "finite_lattice_ensemble_implies_continuum_qft": False,
        "finite_lattice_ensemble_implies_positive_mass_gap": False,
    },
}

EXPECTED_ENSEMBLE_VECTORS: dict[str, Any] = {
    "contract_id": "YM-SU2-ENSEMBLE-V1",
    "vector_set": "SU2-ENSEMBLE-PHASE2A-V1",
    "status": "phase-2a-frozen",
    "vectors": [
        {
            "id": "splitmix64-seed-zero-first-five",
            "seed": "0000000000000000",
            "outputs_hex": [
                "e220a8397b1dcdaf",
                "6e789e6aa1b965f4",
                "06c45d188009454f",
                "f88bb8a8724c81ec",
                "1b39896a51a8749b",
            ],
            "claim_class": "C0",
        },
        {
            "id": "fixed-chain-replay",
            "master_seed": "0123456789abcdef",
            "chain_id": 0,
            "expected": "identical derived seed, raw measurement series, and diagnostics for repeated execution under the same reference implementation/toolchain",
            "claim_class": "C0",
        },
        {
            "id": "independent-chain-identity",
            "master_seed": "0123456789abcdef",
            "chain_ids": [0, 1],
            "expected": "distinct derived seeds and distinct raw measurement series",
            "claim_class": "C0",
        },
        {
            "id": "beta-zero-haar-plaquette-sanity",
            "extents": [2, 2, 2, 2],
            "beta": 0.0,
            "thermalization_sweeps": 1,
            "measurements": 128,
            "sweeps_between_measurements": 1,
            "overrelaxation_sweeps_per_heatbath": 1,
            "observable": "average plaquette",
            "analytic_expectation": 0.0,
            "absolute_mean_max": 0.08,
            "effective_sample_size_min": 16.0,
            "heatbath_acceptance_min": 0.5,
            "link_unit_residual_max": 1e-12,
            "claim_class": "C2",
        },
        {
            "id": "overrelaxation-microcanonical-action",
            "extents": [2, 2, 2, 2],
            "beta": 2.0,
            "precondition": "one heatbath sweep from the cold lattice with fixed seed feedfacecafebeef",
            "observable": "absolute Wilson-action difference after one complete overrelaxation sweep",
            "expected_max": 2e-10,
            "claim_class": "C2",
        },
        {
            "id": "phase2-graduation-boundary",
            "published_plaquette_baseline_reproduced": False,
            "phase2_graduation_gate_satisfied": False,
            "su3_ensemble_enabled": False,
            "expected": "Phase 2A may be reviewed without claiming the full Phase 2 graduation gate",
            "claim_class": "C0",
        },
    ],
}


def validate_ensemble_contract(payload: dict[str, Any]) -> None:
    validate_exact(CONTRACT_PATH, payload, EXPECTED_ENSEMBLE_CONTRACT)


def validate_ensemble_vectors(payload: dict[str, Any]) -> None:
    validate_exact(VECTOR_PATH, payload, EXPECTED_ENSEMBLE_VECTORS)


def main() -> None:
    validate_ensemble_contract(load(CONTRACT_PATH))
    validate_ensemble_vectors(load(VECTOR_PATH))
    print("Phase 2A SU(2) ensemble contract checks: passed")


if __name__ == "__main__":
    main()
