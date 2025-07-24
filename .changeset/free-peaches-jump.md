---
"@nomicfoundation/slang": patch
---

Changed the way pragmas are processed:

- `pragma abicoder <version>`:
    - No longer enabled for Solidity < 0.7.5.
    - For versions >=0.7.5, `<version>` is restricted to new keywords (`v1` and `v2`).
- `pragma experimental <flag>`:
    - No longer enabled for Solidity < 0.4.16.
    - For versions >=0.4.16, `<flag>` is restricted to be a string, or new keywords representing `ABIEncoderV2` and `SMTChecker`.
