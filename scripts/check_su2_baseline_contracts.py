#!/usr/bin/env python3
"""Fail-closed checks for the Phase 2B published SU(2) plaquette baseline."""

from __future__ import annotations

from typing import Any

from check_contracts import load, validate_exact

CONTRACT_PATH = "contracts/YM-SU2-PLAQUETTE-BASELINE-V1.json"
VECTOR_PATH = "test-vectors/su2-plaquette-baseline-v1.json"

EXPECTED_BASELINE_CONTRACT: dict[str, Any] = {
    "contract_id": "YM-SU2-PLAQUETTE-BASELINE-V1",
    "status": "phase-2b-frozen",
    "depends_on": [
        {"contract_id": "YM-SU2-ENSEMBLE-V1", "required_status": "phase-2a-frozen"}
    ],
    "primary_source": {
        "title": "Density of States in Gauge Theories",
        "authors": ["Kurt Langfeld", "Biagio Lucini", "Antonio Rago"],
        "journal": "Physical Review Letters 109, 111601 (2012)",
        "doi": "10.1103/PhysRevLett.109.111601",
        "peer_reviewed": True,
        "source_surface": "Figure 5: four-dimensional SU(2) Wilson-action average plaquette on a 10^4 lattice, comparing density-of-states and local-hybrid Monte Carlo",
        "numeric_extraction": "conservative graphical source-resolution envelope; not a quoted tabulated statistical error",
    },
    "benchmark": {
        "gauge_group": "SU(2)",
        "action": "Wilson",
        "extents": [10, 10, 10, 10],
        "beta": 2.3,
        "observable": "average plaquette=(1/(6V))*sum_x,sum_mu<nu (1/2)ReTr P_mu_nu(x)",
        "source_envelope_min": 0.59,
        "source_envelope_max": 0.62,
    },
    "reference_run": {
        "master_seed": "2b00201211160101",
        "chain_ids": [0, 1],
        "thermalization_sweeps": 40,
        "measurements_per_chain": 48,
        "sweeps_between_measurements": 1,
        "overrelaxation_sweeps_per_heatbath": 1,
        "max_heatbath_attempts_per_link": 10000,
        "autocorrelation_max_lag": 12,
        "minimum_ess_per_chain": 6.0,
        "maximum_chain_consistency_z": 5.0,
        "stationarity_half_measurements": 24,
        "stationarity_autocorrelation_max_lag": 6,
        "maximum_stationarity_z": 5.0,
        "maximum_link_unit_residual": 1e-12,
    },
    "comparison_rule": {
        "pooled_mean_must_lie_inside_source_envelope": True,
        "independent_chain_diagnostics_required": True,
        "within_chain_stationarity_required": True,
        "stationarity_rule": "for each chain compare the first 24 and last 24 post-thermalization measurements using the frozen Phase 2A diagnostics with max_lag=6; require |mean_first-mean_second|/sqrt(SE_first^2+SE_second^2) <= 5; reject if either half has zero empirical variance or the combined SE scale is zero",
        "zero_variance_stationarity_rejected": True,
        "source_envelope_is_paper_error_bar": False,
        "raw_measurements_retained_by_chain_reports": True,
    },
    "phase2_gate": {
        "published_plaquette_baseline_reproduced": True,
        "phase2_graduation_gate_satisfied": True,
        "su3_ensemble_enabled": False,
    },
    "claim_boundary": {
        "benchmark_agreement_is_finite_lattice_validation": True,
        "benchmark_agreement_implies_confinement": False,
        "benchmark_agreement_implies_continuum_qft": False,
        "benchmark_agreement_implies_positive_mass_gap": False,
    },
}

EXPECTED_BASELINE_VECTORS: dict[str, Any] = {
    "contract_id": "YM-SU2-PLAQUETTE-BASELINE-V1",
    "vector_set": "SU2-PLAQUETTE-BASELINE-PHASE2B-V1",
    "status": "phase-2b-frozen",
    "vectors": [
        {
            "id": "published-source-envelope",
            "doi": "10.1103/PhysRevLett.109.111601",
            "figure": 5,
            "extents": [10, 10, 10, 10],
            "beta": 2.3,
            "observable": "average plaquette=(1/(6V))*sum_x,sum_mu<nu (1/2)ReTr P_mu_nu(x)",
            "expected_interval": [0.59, 0.62],
            "interval_semantics": "conservative graphical source-resolution envelope, not a quoted paper error bar",
            "claim_class": "C0",
        },
        {
            "id": "two-independent-chain-comparison",
            "master_seed": "2b00201211160101",
            "chain_ids": [0, 1],
            "thermalization_sweeps": 40,
            "measurements_per_chain": 48,
            "sweeps_between_measurements": 1,
            "overrelaxation_sweeps_per_heatbath": 1,
            "minimum_ess_per_chain": 6.0,
            "maximum_chain_consistency_z": 5.0,
            "maximum_link_unit_residual": 1e-12,
            "expected": "pooled mean lies inside the published source-resolution envelope and independent-chain diagnostics satisfy all declared gates",
            "claim_class": "C2",
        },
        {
            "id": "post-thermalization-stationarity",
            "chain_ids": [0, 1],
            "measurements_per_chain": 48,
            "half_measurements": 24,
            "autocorrelation_max_lag_per_half": 6,
            "maximum_half_drift_z": 5.0,
            "zero_variance_half_rejected": True,
            "expected": "each chain's first and second post-thermalization measurement halves have nonzero empirical variance and are consistent within the frozen autocorrelation-aware drift-z gate",
            "claim_class": "C2",
        },
        {
            "id": "phase2-graduation-boundary",
            "published_plaquette_baseline_reproduced": True,
            "phase2_graduation_gate_satisfied": True,
            "benchmark_agreement_implies_confinement": False,
            "benchmark_agreement_implies_continuum_qft": False,
            "benchmark_agreement_implies_positive_mass_gap": False,
            "su3_ensemble_enabled": False,
            "claim_class": "C0",
        },
    ],
}


def main() -> None:
    validate_exact(CONTRACT_PATH, load(CONTRACT_PATH), EXPECTED_BASELINE_CONTRACT)
    validate_exact(VECTOR_PATH, load(VECTOR_PATH), EXPECTED_BASELINE_VECTORS)
    print("Phase 2B published SU(2) plaquette baseline contract checks: passed")


if __name__ == "__main__":
    main()
