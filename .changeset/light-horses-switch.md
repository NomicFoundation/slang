---
"@nomicfoundation/slang": patch
---

Fixes to the binding rules in Solidity:

- Make the `.length` member available in all static-size byte arrays
- Allow assembly blocks (and nested Yul functions) to access inherited state variables
- Allow assembly blocks access to constructor/modifier/fallback parameters
- `msg.sender` is of `address` type (not `payable`) until 0.5.0
- Top-level constants need to be visible from assembly blocks in files that import them
- Resolve named arguments when calling an extension function
