#!/usr/bin/env python3
"""Run every currently advertised YANG-MILLS contract gate."""

from check_contracts import main as check_core_contracts
from check_su3_contracts import main as check_su3_contracts
from check_path_contracts import main as check_path_contracts
from check_classical_geometry_contracts import main as check_classical_geometry_contracts


def main() -> None:
    check_core_contracts()
    check_su3_contracts()
    check_path_contracts()
    check_classical_geometry_contracts()
    print("All YANG-MILLS contract checks: passed")


if __name__ == "__main__":
    main()
