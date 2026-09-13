#!/usr/bin/env python3
from pathlib import Path

path = Path("scripts/apply_pr12_review_hardening.py")
text = path.read_text(encoding="utf-8")
old = '''use glueball_smearing::{\\n    max_link_residual, spatial_stout_smear, CANONICAL_RHO, GAUGE_COVARIANCE_TOLERANCE,\\n    MAX_LEVELS, SMEARING_CONTRACT_ID, TRANSFORMATION_ID,\\n};\\n'''
new = '''use glueball_smearing::{\\n    max_link_residual, spatial_stout_smear, CANONICAL_RHO, GAUGE_COVARIANCE_TOLERANCE, MAX_LEVELS,\\n    SMEARING_CONTRACT_ID, TRANSFORMATION_ID,\\n};\\n'''
if old not in text:
    raise SystemExit("review-helper import patch target missing")
path.write_text(text.replace(old, new, 1), encoding="utf-8")
