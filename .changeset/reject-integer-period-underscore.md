---
"@nomicfoundation/slang": patch
---

Bug fix: reject `DecimalLiteral` followed by `._` (e.g. `1._2`) starting from Solidity `0.5.0`.
