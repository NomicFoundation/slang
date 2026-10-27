---
"@nomicfoundation/slang": patch
---

Bug fix: reject `DecimalLiteral` and `YulDecimalLiteral` with a leading zero (octal numbers), like `01`, `01.5` or `0_1`.
