#!/usr/bin/env python3
"""Adapt and execute the one-shot PR #10 fifth review patch against the current head."""

from pathlib import Path

patch_path = Path("scripts/apply_pr10_review_fixes_round5.py")
text = patch_path.read_text()

# The current checker already pins V2 predeclaration as one exact object, while
# the original one-shot patch was authored against the earlier field-by-field
# form. Replace that one patch operation in the patch source before executing it.
needle = '''replace_once(
    "scripts/check_su3_baseline_contracts.py",
    \'\'\'    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\\n    pre = payload["predeclaration"]\\n\'\'\',
    \'\'\'    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\\n    _require(payload["replay_tolerance"] == EXPECTED_V2_REPLAY_TOLERANCE, "Phase 2C2 V2 replay-tolerance drift")\\n    pre = payload["predeclaration"]\\n\'\'\',
)
'''
replacement = '''replace_once(
    "scripts/check_su3_baseline_contracts.py",
    \'\'\'    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\\n    _require(payload["predeclaration"] == V2_PREDECLARATION, "Phase 2C2 V2 predeclaration drift")\\n\'\'\',
    \'\'\'    _require(payload["comparison_rule"] == V2_COMPARISON_RULE, "Phase 2C2 V2 comparison-rule drift")\\n    _require(payload["replay_tolerance"] == EXPECTED_V2_REPLAY_TOLERANCE, "Phase 2C2 V2 replay-tolerance drift")\\n    _require(payload["predeclaration"] == V2_PREDECLARATION, "Phase 2C2 V2 predeclaration drift")\\n\'\'\',
)
'''
if needle not in text:
    raise SystemExit("could not adapt the V2 replay-tolerance checker patch")
text = text.replace(needle, replacement, 1)
text = text.replace(
    'raise SystemExit(f"expected review-fix block not found in {path_str}")',
    'raise SystemExit(f"expected review-fix block not found in {path_str}: {old[:180]!r}")',
)

exec(compile(text, str(patch_path), "exec"), {"__name__": "__main__"})
