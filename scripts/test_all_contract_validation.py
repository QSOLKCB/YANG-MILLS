#!/usr/bin/env python3
"""Run every currently advertised negative contract regression gate."""

from test_contract_validation import main as test_core_contracts
from test_su3_contract_validation import main as test_su3_contracts
from test_path_contract_validation import main as test_path_contracts
from test_classical_geometry_contract_validation import main as test_classical_geometry_contracts
from test_su2_ensemble_contract_validation import main as test_su2_ensemble_contracts
from test_su2_baseline_contract_validation import main as test_su2_baseline_contracts
from test_su3_ensemble_contract_validation import main as test_su3_ensemble_contracts
from test_su3_baseline_contract_validation import main as test_su3_baseline_contracts
from test_glueball_operator_contract_validation import main as test_glueball_operator_contracts
from test_glueball_smearing_contract_validation import main as test_glueball_smearing_contracts
from test_glueball_blocking_contract_validation import main as test_glueball_blocking_contracts
from test_glueball_correlator_contract_validation import main as test_glueball_correlator_contracts
from test_glueball_effective_mass_contract_validation import main as test_glueball_effective_mass_contracts
from test_glueball_scalar_fit_contract_validation import main as test_glueball_scalar_fit_contracts
from test_glueball_scalar_production_contract_validation import main as test_glueball_scalar_production_contracts
from test_glueball_scalar_recovery_contract_validation import main as test_glueball_scalar_recovery_contracts


def main() -> None:
    test_core_contracts()
    test_su3_contracts()
    test_path_contracts()
    test_classical_geometry_contracts()
    test_su2_ensemble_contracts()
    test_su2_baseline_contracts()
    test_su3_ensemble_contracts()
    test_su3_baseline_contracts()
    test_glueball_operator_contracts()
    test_glueball_smearing_contracts()
    test_glueball_blocking_contracts()
    test_glueball_correlator_contracts()
    test_glueball_effective_mass_contracts()
    test_glueball_scalar_fit_contracts()
    test_glueball_scalar_production_contracts()
    test_glueball_scalar_recovery_contracts()
    print("All negative YANG-MILLS contract regression checks: passed")


if __name__ == "__main__":
    main()
