#!/usr/bin/env python3
"""Fail-closed checks for the Phase 2A SU(2) ensemble surface."""

from __future__ import annotations

from typing import Any

from check_contracts import load, require

CONTRACT_PATH = "contracts/YM-SU2-ENSEMBLE-V1.json"
VECTOR_PATH = "test-vectors/su2-ensemble-v1.json"


def exact_keys(payload: dict[str, Any], expected: set[str], label: str) -> None:
    require(set(payload) == expected, f"{label}: key set drift")


def validate_ensemble_contract(payload: dict[str, Any]) -> None:
    exact_keys(
        payload,
        {
            "contract_id",
            "status",
            "scope",
            "depends_on",
            "literature_basis",
            "rng",
            "chain_identity",
            "reference_bounds",
            "update_order",
            "su2_heatbath",
            "su2_overrelaxation",
            "schedule",
            "autocorrelation_diagnostics",
            "reproducibility",
            "phase2_gate",
            "claim_boundary",
        },
        CONTRACT_PATH,
    )
    require(payload["contract_id"] == "YM-SU2-ENSEMBLE-V1", "ensemble contract id drift")
    require(payload["status"] == "phase-2a-review-candidate", "ensemble status drift")
    require(
        payload["depends_on"]
        == [
            {"contract_id": "YM-SU2-LATTICE-V1", "required_status": "phase-1a-frozen"},
            {"contract_id": "YM-WILSON-LOOP-V1", "required_status": "phase-1a-frozen"},
        ],
        "ensemble dependency drift",
    )

    rng = payload["rng"]
    exact_keys(
        rng,
        {
            "id",
            "state_transition",
            "uniform_f64",
            "cryptographic_rng",
            "seed_is_physical_parameter",
            "same_seed_same_chain_id_replays_same_reference_stream",
            "cross_platform_bitwise_floating_identity_claimed",
        },
        "rng",
    )
    require(rng["id"] == "SPLITMIX64-V1", "RNG id drift")
    require(rng["cryptographic_rng"] is False, "RNG may not be advertised as cryptographic")
    require(rng["seed_is_physical_parameter"] is False, "seed may not become a physical parameter")
    require(
        rng["same_seed_same_chain_id_replays_same_reference_stream"] is True,
        "fixed-chain replay requirement weakened",
    )
    require(
        rng["cross_platform_bitwise_floating_identity_claimed"] is False,
        "cross-platform floating bitwise identity overclaim",
    )

    bounds = payload["reference_bounds"]
    exact_keys(
        bounds,
        {
            "beta_min",
            "beta_max",
            "max_schedule_sweeps",
            "max_measurements",
            "max_overrelaxation_sweeps_per_heatbath",
            "heatbath_attempt_limit_must_be_positive",
            "autocorrelation_max_lag_must_be_less_than_measurements",
        },
        "reference_bounds",
    )
    require(bounds["beta_min"] == 0.0 and bounds["beta_max"] == 8.0, "beta bounds drift")
    require(bounds["max_schedule_sweeps"] == 1048576, "schedule bound drift")
    require(bounds["max_measurements"] == 1048576, "measurement bound drift")
    require(bounds["max_overrelaxation_sweeps_per_heatbath"] == 16, "over-relaxation bound drift")
    require(bounds["heatbath_attempt_limit_must_be_positive"] is True, "heat-bath bound weakened")
    require(
        bounds["autocorrelation_max_lag_must_be_less_than_measurements"] is True,
        "autocorrelation lag bound weakened",
    )

    update = payload["update_order"]
    require(update["site_order"] == "lexicographic x0,x1,x2,x3", "site update order drift")
    require(update["direction_order"] == "mu=0,1,2,3 at each site", "direction update order drift")
    require(
        update["markov_sweep"]
        == "one complete heatbath sweep followed by the configured number of complete overrelaxation sweeps",
        "Markov sweep definition drift",
    )

    heatbath = payload["su2_heatbath"]
    require(heatbath["local_variable"] == "R=U Vhat", "heat-bath local variable drift")
    require(
        heatbath["conditional_density"]
        == "dP(R) proportional to exp(alpha r0) d_mu_Haar(R), alpha=beta*k",
        "heat-bath conditional density drift",
    )
    require(
        heatbath["haar_jacobian_acceptance"]
        == "accept scalar proposal with probability sqrt(1-r0^2)",
        "heat-bath Haar acceptance drift",
    )
    require(
        heatbath["link_reconstruction"] == "U_new=R Vhat^dagger; when k=0 use U_new=R",
        "heat-bath link reconstruction drift",
    )

    overrelax = payload["su2_overrelaxation"]
    require(
        overrelax["formula"] == "U_new=Vhat^dagger U_old^dagger Vhat^dagger for nonzero staple norm",
        "over-relaxation formula drift",
    )
    require(overrelax["local_wilson_action_preserved"] is True, "microcanonical invariant weakened")
    require(
        overrelax["purpose"] == "microcanonical decorrelation step; it is not an ergodic sampler by itself",
        "over-relaxation role overclaim",
    )

    diagnostics = payload["autocorrelation_diagnostics"]
    require(
        diagnostics["tau_int"] == "0.5 + sum of positive-window rho_lag",
        "tau_int estimator drift",
    )
    require(
        diagnostics["effective_sample_size"] == "min(N,N/(2*tau_int))",
        "ESS estimator drift",
    )

    gate = payload["phase2_gate"]
    exact_keys(
        gate,
        {
            "beta_zero_haar_sanity_check_enabled",
            "published_plaquette_baseline_reproduced",
            "published_baseline_freeze_requires_matching_action_volume_and_parameter_regime",
            "phase2_graduation_gate_satisfied",
            "su3_ensemble_enabled",
        },
        "phase2_gate",
    )
    require(gate["beta_zero_haar_sanity_check_enabled"] is True, "beta=0 sanity gate disabled")
    require(gate["published_plaquette_baseline_reproduced"] is False, "published baseline claimed prematurely")
    require(
        gate["published_baseline_freeze_requires_matching_action_volume_and_parameter_regime"] is True,
        "published baseline matching rule weakened",
    )
    require(gate["phase2_graduation_gate_satisfied"] is False, "Phase 2 graduated prematurely")
    require(gate["su3_ensemble_enabled"] is False, "SU(3) ensemble enabled before SU(2) graduation")

    claims = payload["claim_boundary"]
    require(claims["ensemble_measurements_are_finite_lattice_numerical_observations"] is True, "C2 boundary weakened")
    for key in [
        "single_seed_is_physical_truth",
        "small_autocorrelation_implies_independent_physics",
        "plaquette_agreement_implies_confinement",
        "finite_lattice_ensemble_implies_continuum_qft",
        "finite_lattice_ensemble_implies_positive_mass_gap",
    ]:
        require(claims[key] is False, f"claim boundary weakened: {key}")


def validate_ensemble_vectors(payload: dict[str, Any]) -> None:
    exact_keys(payload, {"contract_id", "vector_set", "status", "vectors"}, VECTOR_PATH)
    require(payload["contract_id"] == "YM-SU2-ENSEMBLE-V1", "vector contract id drift")
    require(payload["vector_set"] == "SU2-ENSEMBLE-PHASE2A-V1", "vector set id drift")
    require(payload["status"] == "phase-2a-review-candidate", "vector status drift")
    ids = [vector.get("id") for vector in payload["vectors"]]
    require(
        ids
        == [
            "splitmix64-seed-zero-first-five",
            "fixed-chain-replay",
            "independent-chain-identity",
            "beta-zero-haar-plaquette-sanity",
            "overrelaxation-microcanonical-action",
            "phase2-graduation-boundary",
        ],
        "Phase 2A vector identity/order drift",
    )
    require(
        payload["vectors"][0]["outputs_hex"]
        == [
            "e220a8397b1dcdaf",
            "6e789e6aa1b965f4",
            "06c45d188009454f",
            "f88bb8a8724c81ec",
            "1b39896a51a8749b",
        ],
        "SplitMix64 golden stream drift",
    )
    beta0 = payload["vectors"][3]
    require(beta0["beta"] == 0.0 and beta0["analytic_expectation"] == 0.0, "beta=0 vector drift")
    require(beta0["absolute_mean_max"] == 0.08, "beta=0 mean band drift")
    require(beta0["effective_sample_size_min"] == 16.0, "beta=0 ESS floor drift")
    require(beta0["link_unit_residual_max"] == 1e-12, "unit residual tolerance drift")
    require(payload["vectors"][4]["expected_max"] == 2e-10, "over-relaxation tolerance drift")
    gate = payload["vectors"][5]
    require(gate["published_plaquette_baseline_reproduced"] is False, "vector baseline overclaim")
    require(gate["phase2_graduation_gate_satisfied"] is False, "vector graduation overclaim")
    require(gate["su3_ensemble_enabled"] is False, "vector SU(3) overreach")


def main() -> None:
    validate_ensemble_contract(load(CONTRACT_PATH))
    validate_ensemble_vectors(load(VECTOR_PATH))
    print("Phase 2A SU(2) ensemble contract checks: passed")


if __name__ == "__main__":
    main()
