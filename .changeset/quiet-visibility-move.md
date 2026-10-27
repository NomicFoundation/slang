---
"@nomicfoundation/slang": patch
---

Fixed parsing of function typed state variables, `function() external public x;` is now parsed as a state variable `x` with `public` visibility, of type `function() external`, rather than `function() external public`.
