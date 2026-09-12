// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use yang_mills_reference::{Lattice4, ReferenceError, Su2};

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
    derive_su3_chain_seed, local_action_difference, metropolis_sweep, run_chain, Su3EnsembleConfig,
    Su3EnsembleError, LINK_RESIDUAL_LIMIT, PROPOSAL_ID, RNG_ID, SAMPLER_ID, STREAM_ID,
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
fn beta_zero_sweep_accepts_every_generator_proposal() {
    let mut lattice = Su3Lattice4::new_identity([2, 2, 2, 2]).unwrap();
    let mut rng = SplitMix64::new(derive_su3_chain_seed(MASTER_SEED, 0));
    let accounting = metropolis_sweep(&mut lattice, 0.0, 0.35, &mut rng).unwrap();
    assert_eq!(accounting.proposals, (lattice.link_count() * 8) as u64);
    assert_eq!(accounting.accepts, accounting.proposals);
    assert!(lattice.max_link_unitarity_residual() <= LINK_RESIDUAL_LIMIT);
    assert!(lattice.max_link_determinant_residual() <= LINK_RESIDUAL_LIMIT);
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
}
