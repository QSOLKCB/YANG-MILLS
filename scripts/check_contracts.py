#!/usr/bin/env python3
"""Fail-closed Phase 0 checks for machine-readable and semantic contract surfaces."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

EXPECTED = {
    "contracts/YM-GAUGE-GROUP-V1.json": "YM-GAUGE-GROUP-V1",
    "contracts/YM-LATTICE-V1.json": "YM-LATTICE-V1",
    "contracts/YM-EVIDENCE-V1.json": "YM-EVIDENCE-V1",
}


def load(relative: str) -> dict:
    path = ROOT / relative
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"contract check failed: {message}")


def main() -> None:
    for relative, contract_id in EXPECTED.items():
        data = load(relative)
        require(data.get("contract_id") == contract_id, f"{relative}: contract_id drift")
        require(data.get("status") == "phase-0-draft", f"{relative}: unexpected status")

    gauge = load("contracts/YM-GAUGE-GROUP-V1.json")
    require(gauge.get("gauge_group") == "SU(2)", "Phase 0 gauge group must be SU(2)")
    require(
        gauge.get("representation", {}).get("matrix_convention")
        == "U(q)=a0*I-i*(a1*sigma1+a2*sigma2+a3*sigma3)",
        "SU(2) matrix convention drift",
    )

    lattice = load("contracts/YM-LATTICE-V1.json")
    require(lattice.get("dimension") == 4, "lattice dimension must remain four")
    require(lattice.get("boundary_conditions") == "periodic", "boundary condition drift")
    require(lattice.get("plaquette_sum") == "mu<nu", "plaquette counting drift")

    evidence = load("contracts/YM-EVIDENCE-V1.json")
    require(evidence.get("numerical_evidence_is_proof") is False, "evidence/proof boundary weakened")
    require(evidence.get("accelerator_is_mathematical_authority") is False, "accelerator boundary weakened")
    require(evidence.get("clay_solution_claimed") is False, "Phase 0 cannot claim a Clay solution")

    vectors = load("test-vectors/su2-wilson-v1.json")
    require(vectors.get("contract_id") == "YM-LATTICE-V1", "test-vector contract drift")
    require(len(vectors.get("vectors", [])) >= 2, "expected Phase 0 regression vectors")

    claim_text = (ROOT / "docs/CLAIM_BOUNDARY.md").read_text(encoding="utf-8")
    require("finite-lattice" in claim_text.lower(), "claim boundary lost finite-lattice distinction")
    require("Clay" in claim_text, "claim boundary must retain Clay distinction")

    donor_text = (ROOT / "docs/DONOR_BOUNDARY.md").read_text(encoding="utf-8")
    require("engineering donor" in donor_text, "donor semantics drift")
    require("80941183d14531093117e122da0fc32c13d2464b" in donor_text, "GLUBALL donor SHA drift")

    print("Phase 0 contract checks: passed")


if __name__ == "__main__":
    main()
