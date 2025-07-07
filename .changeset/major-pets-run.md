---
"@nomicfoundation/slang": patch
---

Fixes to the binding rules in Solidity:

- Values of the deprecated `byte` type have a `length` member until 0.8.0
- Bind a qualified identifier in the same contract, ie. `Foo.x` in a method body of `Foo`
- Correctly bind external constants and built-ins in nested functions in assembly blocks
- Literal boolean values should bind to the `bool` type to chain extension functions
- Public state variables the generate getters should have members of external functions (such as `.selector`)
- Event types have a `selector` member
