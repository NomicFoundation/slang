# 5.2. Using Cursors

This guide will walk you through the basics of using a [CST cursor](../../03-concepts/index.md#cursors) in your project.
Let's start with this source file, that contains three contracts:

```ts title="common.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/02-navigating-with-cursors/examples/common.mts"
```

## Listing Contract Names

The below example uses a cursor to list the names of all contracts in a source file:

```ts title="listing-contract-names.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/02-navigating-with-cursors/examples/01-listing-contract-names.test.mts"
```

## Visiting Only a Sub-tree

In cases like the above, we needed to visit a sub-tree of the CST (to get the contract name).
But we also need to remember to return the cursor to its original position after each read,
which is inconvenient, and can lead to subtle bugs.

To avoid this, we can use the `spawn()` API,
which cheaply creates a new cursor that starts at the given node, without copying the previous path history.
This lets us visit the sub-tree of each contract, without modifying the original cursor:

```ts title="visiting-subtrees.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/02-navigating-with-cursors/examples/02-visiting-subtrees.test.mts"
```

## Accessing Node Positions

The `Cursor` API also tracks the position and range of the current node it is visiting.
Here is an example that records the source range of each contract, along with its text:

```ts title="accessing-node-positions.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/02-navigating-with-cursors/examples/03-accessing-node-positions.test.mts"
```
