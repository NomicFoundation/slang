# 5.3. Using Cursors

This guide will walk you through the basics of using a [CST cursor](../../03-concepts/index.md#cursors) in your project.
Let's start with this source file, that contains three contracts:

```ts title="common.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/03-navigating-with-cursors/examples/common.mts"
```

## Listing Contract Names

The below example uses a cursor to list the names of all contracts in a source file:

```ts title="listing-contract-names.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/03-navigating-with-cursors/examples/01-listing-contract-names.test.mts"
```

## Visiting Only a Sub-tree

Next, we will try to get the names of all contract functions, grouped by the contract name.
In this case, it is not enough to just visit all instances of [`FunctionDefinition`](../../../solidity-grammar/02-definitions/08-functions.md) nodes, since we want to exclude the ones that are not part of a contract.

We need first to find all [`ContractDefinition`](../../../solidity-grammar/02-definitions/01-contracts.md) nodes, and then for each contract, look for all [`FunctionDefinition`](../../../solidity-grammar/02-definitions/08-functions.md) nodes,
limiting the search to just the contract's subtree.
To do that, we need to use the `cursor.spawn()` API, which cheaply creates a new cursor that starts at the given node,
without copying the ancestry information, so it will only be able to see the sub-tree of the current node.

```ts title="visiting-subtrees.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/03-navigating-with-cursors/examples/02-visiting-subtrees.test.mts"
```

## Accessing Node Positions

The `Cursor` API also tracks the position and range of the current node it is visiting.
Here is an example that records the line number of each function, along with its text:

```ts title="accessing-node-positions.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/03-navigating-with-cursors/examples/03-accessing-node-positions.test.mts"
```
