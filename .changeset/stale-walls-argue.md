---
"@nomicfoundation/slang": patch
---

Fixes to bindings rules in Solidity:

- Allow binding of `using` directives inside interfaces in Solidity < 0.7.1
- Bind literal fixed arrays types
- Fix generating binding graph for built-ins: remove the `memory` location specifier from types so they bind properly
- Fix return type of `value()` and `gas()` legacy call options to allow chaining them
- Bind legacy call options in the result of `new` expressions
- Bind output type of public getters when the state variable is a nested mapping or array
- A `using` directive with the `global` modifier should impact the source unit's lexical scope
- Relax the Solidity version where the `transfer()` method works for non-payable addresses; this is a workaround for a Solidity quirk that makes it possible to do `address(uint160(to)).transfer(amount)` even after 0.5.0
- Fix bound return types of `wrap()` and `unwrap()` methods of a user value defined type
- Resolve the type of `min()` and `max()` of `type()` expressions for integer types to the integer type given in the expression operand
- Fix binding of fully qualified modifier invocations
- Fix #1321: `min()` and `max()` for `type()` expressions on `enum` types should bind only after Solidity 0.8.8
- Bound type for literal number expressions is `uint256` by default; this allows correctly binding extension methods operating on literal values
- The type `bytes` is an array type and should bind the `push()` and `pop()` methods
- Contract or interface reference values implicitly inherit from the `address` type on Solidity < 0.5.0
- Modifiers are allowed inside interfaces until Solidity 0.8.8 and thus should properly bind and be accessible from inheriting contracts
- Libraries before Solidity 0.5.0 allowed `this` in function methods and work as an `address` type
