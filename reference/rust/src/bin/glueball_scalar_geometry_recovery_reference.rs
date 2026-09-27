// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};

#[path = "../glueball_blocking.rs"]
mod glueball_blocking;
#[path = "../glueball_correlators.rs"]
mod glueball_correlators;
#[path = "../glueball_effective_mass.rs"]
mod glueball_effective_mass;
#[path = "../glueball_operators.rs"]
mod glueball_operators;
#[path = "../glueball_scalar_fit.rs"]
mod glueball_scalar_fit;
#[path = "../glueball_scalar_geometry_recovery.rs"]
mod glueball_scalar_geometry_recovery;
#[path = "../glueball_scalar_geometry_recovery_receipt.rs"]
mod glueball_scalar_geometry_recovery_receipt;
#[path = "../glueball_scalar_production.rs"]
mod glueball_scalar_production;
#[path = "../glueball_scalar_recovery.rs"]
mod glueball_scalar_recovery;
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

use glueball_scalar_geometry_recovery::{
    run_scalar_geometry_recovery_analysis, BETA, BLOCKED_EXTENTS, CHAIN_COUNT,
    EXPECTED_PROPOSALS_PER_CHAIN, EXPECTED_TOTAL_PROPOSALS, EXTENTS, GEOMETRY_RECOVERY_CONTRACT_ID,
    GEOMETRY_RECOVERY_STATUS, MASTER_SEED, POST_THERMALIZATION_SWEEPS, PRIMARY_TRANSFORMATION,
    PROPOSAL_ANGLE, THERMALIZATION_SWEEPS, TOTAL_SWEEPS_PER_CHAIN, TRANSFORMATION_LABELS,
};
use glueball_scalar_geometry_recovery_receipt::{
    observed_phase3h_receipt_matches, OBSERVATION_HEAD, OBSERVATION_WORKFLOW_RUN_ID,
};
use glueball_scalar_production::{OperatorKind, CANDIDATE_WINDOWS, STABILITY_MAX_Z, STOUT_LEVELS};

fn print_optional(label: &str, value: Option<f64>) {
    match value {
        Some(value) => println!("{label}={value:.17e}"),
        None => println!("{label}=unavailable"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let summary = run_scalar_geometry_recovery_analysis()?;
    if !observed_phase3h_receipt_matches(&summary) {
        return Err("Phase 3H deterministic replay no longer matches the pinned observation receipt".into());
    }

    println!("contract={GEOMETRY_RECOVERY_CONTRACT_ID}");
    println!("status={GEOMETRY_RECOVERY_STATUS}");
    println!("evidence_class=blocking-capable-finite-lattice-geometry-recovery-observation");
    println!("recovery_hypothesis=blocking-geometry-overlap-recovery");
    println!("observation_head={OBSERVATION_HEAD}");
    println!("observation_workflow_run_id={OBSERVATION_WORKFLOW_RUN_ID}");
    println!("phase3g_scalar_pipeline_validated=false");
    println!("sampler=YM-SU3-ENSEMBLE-V1");
    println!("proposal=YM-SU3-GENERATOR-METROPOLIS-V1");
    println!("rng=SPLITMIX64-V1");
    println!("stream=YM-SU3-STREAM-V1");
    println!(
        "extents={},{},{},{}",
        EXTENTS[0], EXTENTS[1], EXTENTS[2], EXTENTS[3]
    );
    println!(
        "blocked_extents={},{},{},{}",
        BLOCKED_EXTENTS[0], BLOCKED_EXTENTS[1], BLOCKED_EXTENTS[2], BLOCKED_EXTENTS[3]
    );
    println!("beta={BETA:.17e}");
    println!("master_seed=0x{MASTER_SEED:016x}");
    println!("independent_chains={CHAIN_COUNT}");
    println!("thermalization_sweeps={THERMALIZATION_SWEEPS}");
    println!("post_thermalization_sweeps={POST_THERMALIZATION_SWEEPS}");
    println!("total_sweeps_per_chain={TOTAL_SWEEPS_PER_CHAIN}");
    println!("proposal_angle={PROPOSAL_ANGLE:.17e}");
    println!("expected_proposals_per_chain={EXPECTED_PROPOSALS_PER_CHAIN}");
    println!("expected_total_proposals={EXPECTED_TOTAL_PROPOSALS}");
    println!("candidate_windows={:?}", CANDIDATE_WINDOWS);
    println!("stout_levels={:?}", STOUT_LEVELS);
    println!("transformations={:?}", TRANSFORMATION_LABELS);
    println!("primary_transformation={PRIMARY_TRANSFORMATION}");
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
