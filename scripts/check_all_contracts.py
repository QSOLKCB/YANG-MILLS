#!/usr/bin/env python3
"""Run every currently advertised YANG-MILLS contract gate."""

from check_contracts import main as check_core_contracts
from check_su3_contracts import main as check_su3_contracts


def main() -> None:
    check_core_contracts()
    check_su3_contracts()
    print("All YANG-MILLS contract checks: passed")


if __name__ == "__main__":
    main()
