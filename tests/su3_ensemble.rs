// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, ReferenceError, Su2, DIM};

#[path = "../reference/rust/src/su2_ensemble.rs"]
mod su2_ensemble;
#[path = "../reference/rust/src/su3.rs"]
mod su3;
#[path = "../reference/rust/src/su3_ensemble.rs"]
mod su3_ensemble;
#[path = "../reference/rust/src/su3_lattice.rs"]
mod su3_lattice;

use su2_ensemble::SplitMix64;
use su3::Su3;
use su3_ensemble::{
    derive_su3_chain_seed, local_action_difference, metropolis_sweep, proposal_angle_from_u64,
    run_chain, Su3EnsembleConfig, Su3EnsembleError, ANGLE_GRID_MAGNITUDE_BITS,
    LINK_RESIDUAL_LIMIT, MAX_PROPOSAL_ANGLE, MAX_REFERENCE_MEASUREMENTS, MAX_REFERENCE_SWEEPS,
    MIN_PROPOSAL_ANGLE, PROPOSAL_ID, RNG_ID, SAMPLER_ID, STREAM_ID,
};
use su3_lattice::Su3Lattice4;

const MASTER_SEED: u64 = 0x3c00_1982_1223_0101;

fn config(chain_id: u64) -> Su3EnsembleConfig {
    Su3EnsembleConfig {
        extents: [2, 2, 2, 2],
        beta: 0.0,
        master_seed: MASTER_SEED,
        chain_id,
        thermalization_sweeps: 2,
        measurements: 8,
        sweeps_between_measurements: 1,
        proposal_angle: 0.35,
        autocorrelation_max_lag: 3,
    }
}

#[test]
fn su3_stream_derivation_has_golden_values() {
    assert_eq!(SAMPLER_ID, "YM-SU3-ENSEMBLE-V1");
    assert_eq!(PROPOSAL_ID, "YM-SU3-GENERATOR-METROPOLIS-V1");
    assert_eq!(RNG_ID, "SPLITMIX64-V1");
    assert_eq!(STREAM_ID, "YM-SU3-STREAM-V1");
    assert_eq!(derive_su3_chain_seed(MASTER_SEED, 0), 0x94c7_1563_0ee8_e90a);
    assert_eq!(derive_su3_chain_seed(MASTER_SEED, 1), 0x8aba_19af_6781_dc09);
}

#[test]
fn proposal_angle_grid_is_exactly_paired() {
    assert_eq!(ANGLE_GRID_MAGNITUDE_BITS, 52);
    let proposal_angle = 0.35;
    for raw in [0_u64, 1, 0x000f_ffff_ffff_ffff, 0x1234_5678_9abc_def0] {
        let negative_raw = raw & !(1_u64 << 63);
        let positive_raw = negative_raw | (1_u64 << 63);
        let negative = proposal_angle_from_u64(negative_raw, proposal_angle).unwrap();
        let positive = proposal_angle_from_u64(positive_raw, proposal_angle).unwrap();
        assert_eq!(positive, -negative);
        assert!(positive > 0.0 && positive < proposal_angle);
        assert!(negative < 0.0 && negative > -proposal_angle);
    }
}

#[test]
fn beta_zero_sweep_accepts_every_generator_proposal_and_consumes_two_rng_outputs_each() {
    let mut lattice = Su3Lattice4::new_identity([2, 2, 2, 2]).unwrap();
    let seed = derive_su3_chain_seed(MASTER_SEED, 0);
    let mut rng = SplitMix64::new(seed);
    let accounting = metropolis_sweep(&mut lattice, 0.0, 0.35, &mut rng).unwrap();
    assert_eq!(accounting.proposals, (lattice.link_count() * 8) as u64);
    assert_eq!(accounting.accepts, accounting.proposals);
    assert!(lattice.max_link_unitarity_residual() <= LINK_RESIDUAL_LIMIT);
    assert!(lattice.max_link_determinant_residual() <= LINK_RESIDUAL_LIMIT);

    let mut expected_rng = SplitMix64::new(seed);
    for _ in 0..accounting.proposals * 2 {
        expected_rng.next_u64();
    }
    assert_eq!(rng.next_u64(), expected_rng.next_u64());
}

#[test]
fn local_delta_matches_full_wilson_action_difference() {
    let lattice = Su3Lattice4::new_identity([2, 2, 2, 2]).unwrap();
    let beta = 5.7;
    let mut coefficients = [0.0; 8];
    coefficients[4] = 0.21;
    let candidate = Su3::from_algebra_coefficients(coefficients).unwrap();
    let local = local_action_difference(&lattice, [0, 0, 0, 0], 2, candidate, beta).unwrap();

    let mut proposed = lattice.clone();
    proposed.set_link([0, 0, 0, 0], 2, candidate).unwrap();
    let full = proposed.wilson_action(beta).unwrap() - lattice.wilson_action(beta).unwrap();
    assert!(
        (local - full).abs() <= 2.0e-11,
        "local={local:.17e} full={full:.17e}"
    );
}

#[test]
fn fixed_su3_chain_replays_exactly() {
    let first = run_chain(config(0)).unwrap();
    let second = run_chain(config(0)).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.mean_metropolis_acceptance, 1.0);
    assert!(first.final_link_unitarity_residual <= LINK_RESIDUAL_LIMIT);
    assert!(first.final_link_determinant_residual <= LINK_RESIDUAL_LIMIT);
}

#[test]
fn distinct_su3_chain_ids_produce_distinct_evidence() {
    let first = run_chain(config(0)).unwrap();
    let second = run_chain(config(1)).unwrap();
    assert_ne!(first.derived_seed, second.derived_seed);
    assert_ne!(first.measurements, second.measurements);
}

#[test]
fn maximum_accepted_schedule_preserves_group_bounds() {
    assert_eq!(MAX_REFERENCE_SWEEPS, 512);
    assert_eq!(MAX_REFERENCE_MEASUREMENTS, 512);
    let stress = Su3EnsembleConfig {
        extents: [2, 2, 2, 2],
        beta: 0.0,
        master_seed: MASTER_SEED,
        chain_id: 0,
        thermalization_sweeps: MAX_REFERENCE_SWEEPS - 2,
        measurements: 2,
        sweeps_between_measurements: 1,
        proposal_angle: MAX_PROPOSAL_ANGLE,
        autocorrelation_max_lag: 1,
    };
    let report = run_chain(stress).unwrap();
    assert_eq!(report.total_sweeps, MAX_REFERENCE_SWEEPS);
    assert!(report.final_link_unitarity_residual <= LINK_RESIDUAL_LIMIT);
    assert!(report.final_link_determinant_residual <= LINK_RESIDUAL_LIMIT);
}

#[test]
fn proposal_angle_floor_is_executable_and_fail_closed() {
    assert_eq!(MIN_PROPOSAL_ANGLE, 1.0e-8);

    let mut boundary = config(0);
    boundary.proposal_angle = MIN_PROPOSAL_ANGLE;
    assert!(boundary.validate().is_ok());

    let mut inert = config(0);
    inert.proposal_angle = f64::MIN_POSITIVE;
    assert_eq!(
        inert.validate().unwrap_err(),
        Su3EnsembleError::ProposalAngleOutsideReferenceRange
    );

    let mut below = config(0);
    below.proposal_angle = MIN_PROPOSAL_ANGLE * 0.5;
    assert_eq!(
        below.validate().unwrap_err(),
        Su3EnsembleError::ProposalAngleOutsideReferenceRange
    );
}

#[test]
fn invalid_su3_sampler_inputs_fail_closed() {
    let mut bad = config(0);
    bad.proposal_angle = 0.0;
    assert_eq!(
        run_chain(bad).unwrap_err(),
        Su3EnsembleError::ProposalAngleOutsideReferenceRange
    );

    let mut bad = config(0);
    bad.beta = f64::NAN;
    assert_eq!(
        run_chain(bad).unwrap_err(),
        Su3EnsembleError::BetaOutsideReferenceRange
    );

    let mut bad = config(0);
    bad.sweeps_between_measurements = 0;
    assert_eq!(
        run_chain(bad).unwrap_err(),
        Su3EnsembleError::ZeroMeasurementStride
    );

    let mut bad = config(0);
    bad.thermalization_sweeps = MAX_REFERENCE_SWEEPS;
    assert_eq!(
        run_chain(bad).unwrap_err(),
        Su3EnsembleError::ScheduleOutsideReferenceRange
    );
}
