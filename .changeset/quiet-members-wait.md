---
"@nomicfoundation/slang": patch
---

Fixed priority between `StateVariableDefinition` and `UnnamedFunctionDefinition` when parsing:
`function() x;` is parsed as a `StateVariableDefinition` named `x`, rather than an `UnnamedFunctionDefinition` with an `x` modifier.
