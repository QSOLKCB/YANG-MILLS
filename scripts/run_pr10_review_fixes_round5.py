#!/usr/bin/env python3
"""Adapt and execute the one-shot PR #10 fifth review patch against the current head."""

from pathlib import Path

patch_path = Path("scripts/apply_pr10_review_fixes_round5.py")
text = patch_path.read_text()

text = text.replace(
    '''    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\n    pre = payload["predeclaration"]\n''',
    '''    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\n    _require(payload["predeclaration"] == V2_PREDECLARATION, "Phase 2C2 V2 predeclaration drift")\n''',
)
text = text.replace(
    '''    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\n    _require(payload["replay_tolerance"] == EXPECTED_V2_REPLAY_TOLERANCE, "Phase 2C2 V2 replay-tolerance drift")\n    pre = payload["predeclaration"]\n''',
    '''    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\n    _require(payload["replay_tolerance"] == EXPECTED_V2_REPLAY_TOLERANCE, "Phase 2C2 V2 replay-tolerance drift")\n    _require(payload["predeclaration"] == V2_PREDECLARATION, "Phase 2C2 V2 predeclaration drift")\n''',
)
text = text.replace(
    'raise SystemExit(f"expected review-fix block not found in {path_str}")',
    'raise SystemExit(f"expected review-fix block not found in {path_str}: {old[:180]!r}")',
)

exec(compile(text, str(patch_path), "exec"), {"__name__": "__main__"})
