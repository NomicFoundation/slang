---
"@nomicfoundation/slang": patch
---

Fixed the old style revert calls (`revert("oops!")`) to be parsed as a `FunctionCallExpression` rather than a `RevertStatement`.
