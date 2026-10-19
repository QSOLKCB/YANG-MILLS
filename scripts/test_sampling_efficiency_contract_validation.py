#!/usr/bin/env python3
"""Every reporting contract/fixture field is authoritative, including false claims."""
import copy
from check_sampling_efficiency_contracts import EXPECTED_CONTRACT, EXPECTED_VECTORS, validate_contract, validate_vectors

def main():
    count = 0
    for original, validate in [(EXPECTED_CONTRACT, validate_contract), (EXPECTED_VECTORS, validate_vectors)]:
        validate(original)
        for key in original:
            for replacement in [None, "weakened-authority"]:
                changed = copy.deepcopy(original)
                changed[key] = replacement
                try:
                    validate(changed)
                except (SystemExit, ValueError, TypeError, KeyError):
                    count += 1
                else:
                    raise SystemExit(f"accepted mutated field: {key}")
    print(f"Sampling efficiency negative contract checks: {count} passed")
if __name__ == "__main__":
    main()
