# 8.3. Jump to definition

Another often used feature of an IDE is the ability to jump to the definition of a given identifier. Again, we can use the [Binding Graph API](../../07-semantic-analysis/02-binding-graph/index.md) to do it:

```ts title="jump-to-definition.mts"
--8<-- "documentation/public/user-guide/08-examples/03-jump-to-definition/examples/jump-to-definition.mts"
```

The following example shows jumping to the definition of the parameter `delta` in line 11:

```solidity linenums="1" hl_lines="9 11"
--8<-- "documentation/public/user-guide/08-examples/common/sample-contract.sol"
```

```ts title="test-jump-to-definition.test.mts"
--8<-- "documentation/public/user-guide/08-examples/03-jump-to-definition/examples/test-jump-to-definition.test.mts"
```
