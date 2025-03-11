# 8.2. Find usages

A typical use case for an IDE is finding where some variable, function, or type is used in the code base. In Slang this can be easily accomplished by using the [Binding Graph API](../../07-semantic-analysis/02-binding-graph/index.md):

```ts title="find-usages.mts"
--8<-- "documentation/public/user-guide/08-examples/02-find-usages/examples/find-usages.mts"
```

For example, we can look for usages of the `_count` state variable defined in line 2 of the sample contract:

```solidity linenums="1" hl_lines="2 4 7 11 12"
--8<-- "documentation/public/user-guide/08-examples/common/sample-contract.sol"
```

```ts title="test-find-usages.mts"
--8<-- "documentation/public/user-guide/08-examples/02-find-usages/examples/test-find-usages.test.mts"
```
