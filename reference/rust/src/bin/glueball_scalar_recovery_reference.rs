// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};

#[path = "../glueball_correlators.rs"]
mod glueball_correlators;
#[path = "../glueball_effective_mass.rs"]
mod glueball_effective_mass;
#[path = "../glueball_operators.rs"]
mod glueball_operators;
#[path = "../glueball_scalar_fit.rs"]
mod glueball_scalar_fit;
#[path = "../glueball_scalar_production.rs"]
mod glueball_scalar_production;
#[path = "../glueball_scalar_recovery.rs"]
mod glueball_scalar_recovery;
#[path = "../glueball_scalar_recovery_receipt.rs"]
mod glueball_scalar_recovery_receipt;
#[path = "../glueball_smearing.rs"]
mod glueball_smearing;
#[path = "../path_holonomy.rs"]
mod path_holonomy;
#[path = "../su2_ensemble.rs"]
mod su2_ensemble;
#[path = "../su3.rs"]
mod su3;
#[path = "../su3_ensemble.rs"]
mod su3_ensemble;
#[path = "../su3_lattice.rs"]
mod su3_lattice;

use glueball_scalar_production::{
    OperatorKind, ProductionError, CANDIDATE_WINDOWS, STABILITY_MAX_Z, STOUT_LEVELS,
};
use glueball_scalar_recovery::{
    run_scalar_recovery_analysis, BETA, CHAIN_COUNT, EXPECTED_TOTAL_PROPOSALS, EXTENTS,
    MASTER_SEED, PHASE3F_CHAIN_COUNT, PHASE3F_MASTER_SEED, POST_THERMALIZATION_SWEEPS,
    PROPOSAL_ANGLE, RECOVERY_CONTRACT_ID, RECOVERY_STATUS, THERMALIZATION_SWEEPS,
    TOTAL_SWEEPS_PER_CHAIN,
};
use glueball_scalar_recovery_receipt::{
    observed_phase3g_receipt_matches, OBSERVATION_HEAD, OBSERVATION_WORKFLOW_RUN_ID,
};

#[derive(Debug)]
struct Phase3gImplementationError(ProductionError);

impl fmt::Display for Phase3gImplementationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Phase 3G recovery implementation failure: {:?}", self.0)
    }
}

impl std::error::Error for Phase3gImplementationError {}

fn print_optional(label: &str, value: Option<f64>) {
    match value {
        Some(value) => println!("{label}={value:.17e}"),
        None => println!("{label}=unavailable"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let summary = run_scalar_recovery_analysis().map_err(Phase3gImplementationError)?;
    if !observed_phase3g_receipt_matches(&summary) {
        return Err(
            "Phase 3G deterministic replay no longer matches the pinned observation receipt".into(),
        );
    }

    println!("contract={RECOVERY_CONTRACT_ID}");
    println!("status={RECOVERY_STATUS}");
    println!("evidence_class=fresh-stream-finite-lattice-recovery-observation");
    println!("recovery_hypothesis=independent-statistics-recovery");
    println!("observation_head={OBSERVATION_HEAD}");
    println!("observation_workflow_run_id={OBSERVATION_WORKFLOW_RUN_ID}");
    println!("phase3f_scalar_pipeline_validated=false");
    println!("phase3f_master_seed=0x{PHASE3F_MASTER_SEED:016x}");
    println!("phase3f_independent_chains={PHASE3F_CHAIN_COUNT}");
    println!("sampler=YM-SU3-ENSEMBLE-V1");
    println!("proposal=YM-SU3-GENERATOR-METROPOLIS-V1");
    println!("rng=SPLITMIX64-V1");
    println!("stream=YM-SU3-STREAM-V1");
    println!(
        "extents={},{},{},{}",
        EXTENTS[0], EXTENTS[1], EXTENTS[2], EXTENTS[3]
    );
    println!("beta={BETA:.17e}");
    println!("master_seed=0x{MASTER_SEED:016x}");
    println!("independent_chains={CHAIN_COUNT}");
    println!("thermalization_sweeps={THERMALIZATION_SWEEPS}");
    println!("post_thermalization_sweeps={POST_THERMALIZATION_SWEEPS}");
    println!("total_sweeps_per_chain={TOTAL_SWEEPS_PER_CHAIN}");
    println!("proposal_angle={PROPOSAL_ANGLE:.17e}");
    println!("expected_total_proposals={EXPECTED_TOTAL_PROPOSALS}");
    println!("candidate_windows={:?}", CANDIDATE_WINDOWS);
    println!("stout_levels={:?}", STOUT_LEVELS);
    println!("stability_max_z={STABILITY_MAX_Z:.17e}");

    for evidence in &summary.chain_evidence {
        println!(
            "chain={} seed=0x{:016x} proposals={} accepts={} acceptance={:.17e} plaquette={:.17e} unitarity={:.17e} determinant={:.17e}",
            evidence.chain_id,
            evidence.derived_seed,
            evidence.proposals,
            evidence.accepts,
            evidence.acceptance,
            evidence.plaquette,
            evidence.unitarity_residual,
            evidence.determinant_residual
        );
    }

    for record in &summary.surfaces {
        let operator = match record.outcome.operator {
            OperatorKind::Plaquette1x1 => "plaquette-1x1",
            OperatorKind::Rectangle2x1 => "rectangle-2x1",
        };
        match &record.outcome.fit {
            Some(fit) => println!(
                "surface={} operator={} fit=success window={}-{} mass={:.17e} mass_se={:.17e} amplitude={:.17e} chi2_per_dof={:.17e}",
                record.outcome.transformation,
                operator,
                fit.selected_window.start,
                fit.selected_window.end,
                fit.fitted_mass,
                fit.mass_standard_error,
                fit.fitted_amplitude,
                fit.chi_square_per_dof
            ),
            None => println!(
                "surface={} operator={} fit=rejected reason={}",
                record.outcome.transformation,
                operator,
                record.outcome.error_code.unwrap_or("unknown")
            ),
        }
    }

    println!(
        "sampler_diagnostics_passed={}",
        summary.sampler_diagnostics_passed
    );
    println!(
        "primary_window_successes={}",
        summary.primary_window_successes
    );
    print_optional("primary_window_max_z", summary.primary_window_max_z);
    print_optional("operator_stability_z", summary.operator_stability_z);
    println!(
        "transformation_stability_successes={}",
        summary.transformation_stability_successes
    );
    print_optional(
        "transformation_stability_max_z",
        summary.transformation_stability_max_z,
    );
    println!(
        "scalar_pipeline_validated={}",
        summary.scalar_pipeline_validated
    );
    println!("physical_glueball_mass_claimed=false");
    println!("finite_volume_mass_gap_established=false");
    println!("confinement_established=false");
    println!("continuum_qft_constructed=false");
    println!("positive_continuum_mass_gap_proved=false");
    println!("clay_solution_claimed=false");
    println!("receipt_replay_matches=true");
    println!("result=completed");
    Ok(())
}

#[cfg(test)]
mod binary_tests {
    use super::*;

    #[test]
    fn recovery_implementation_errors_are_attributed_to_phase3g() {
        let message = Phase3gImplementationError(ProductionError::AllocationFailed).to_string();
        assert!(message.contains("Phase 3G"));
        assert!(!message.contains("Phase 3F"));
    }
}
