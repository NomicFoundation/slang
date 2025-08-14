---
"@nomicfoundation/slang": patch
---

Fixed the pragma grammar and CST nodes:

- `pragma abicoder <version>`:
    - Only enabled starting Solidity `0.7.5`.
    - `<version>` is restricted to new keywords (`v1` and `v2`).
- `pragma experimental <flag>`:
    - Only enabled starting Solidity `0.4.16`.
    - `<flag>` is restricted to be a string, or new keywords representing `ABIEncoderV2` and `SMTChecker`.
