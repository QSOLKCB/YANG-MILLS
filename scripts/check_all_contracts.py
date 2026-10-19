#!/usr/bin/env python3
"""Run every currently advertised YANG-MILLS contract gate."""

from check_contracts import main as check_core_contracts
from check_su3_contracts import main as check_su3_contracts
from check_path_contracts import main as check_path_contracts
from check_classical_geometry_contracts import main as check_classical_geometry_contracts
from check_su2_ensemble_contracts import main as check_su2_ensemble_contracts
from check_su2_baseline_contracts import main as check_su2_baseline_contracts
from check_su3_ensemble_contracts import main as check_su3_ensemble_contracts
from check_su3_baseline_contracts import main as check_su3_baseline_contracts
from check_glueball_operator_contracts import main as check_glueball_operator_contracts
from check_glueball_smearing_contracts import main as check_glueball_smearing_contracts
from check_glueball_blocking_contracts import main as check_glueball_blocking_contracts
from check_glueball_correlator_contracts import main as check_glueball_correlator_contracts
from check_glueball_effective_mass_contracts import main as check_glueball_effective_mass_contracts
from check_glueball_scalar_fit_contracts import main as check_glueball_scalar_fit_contracts
from check_glueball_scalar_production_contracts import main as check_glueball_scalar_production_contracts
from check_glueball_scalar_recovery_contracts import main as check_glueball_scalar_recovery_contracts
from check_glueball_scalar_geometry_recovery_contracts import main as check_glueball_scalar_geometry_recovery_contracts
from check_glueball_scalar_temporal_recovery_contracts import main as check_glueball_scalar_temporal_recovery_contracts
from check_glueball_scalar_variational_contracts import main as check_glueball_scalar_variational_contracts


from check_sampling_efficiency_contracts import main as check_sampling_efficiency

def main() -> None:
    check_core_contracts()
    check_su3_contracts()
    check_path_contracts()
    check_classical_geometry_contracts()
    check_su2_ensemble_contracts()
    check_su2_baseline_contracts()
    check_su3_ensemble_contracts()
    check_su3_baseline_contracts()
    check_glueball_operator_contracts()
    check_glueball_smearing_contracts()
    check_glueball_blocking_contracts()
    check_glueball_correlator_contracts()
    check_glueball_effective_mass_contracts()
    check_glueball_scalar_fit_contracts()
    check_glueball_scalar_production_contracts()
    check_glueball_scalar_recovery_contracts()
    check_glueball_scalar_geometry_recovery_contracts()
    check_glueball_scalar_temporal_recovery_contracts()
    check_glueball_scalar_variational_contracts()
    check_sampling_efficiency()
    print("All YANG-MILLS contract checks: passed")


if __name__ == "__main__":
    main()
