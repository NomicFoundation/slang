# 7.3. Solidity Builtins

When resolving references, Slang can differentiate between user provided definitions and Solidity built-ins. This makes it possible to perform deeper semantic analysis which is not possible with only the results of parsing.

For example, we can easily detect incorrect usages of the deprecated `tx.origin` (as recommended [here](https://docs.soliditylang.org/en/latest/security-considerations.html#tx-origin)) but distinguish it from valid user code that syntactically looks exactly the same:

```ts title="solidity-builtins.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/03-solidity-builtins/examples/01-solidity-builtins.test.mts"
```
