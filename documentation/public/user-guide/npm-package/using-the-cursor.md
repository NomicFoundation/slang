# Using the Cursor

This guide will walk you through the basics of using a [CST cursor](../concepts.md#cst-cursors) in your project.
Let's start with this source file, that contains three contracts:

```solidity title="input.sol"
--8<-- "documentation/public/user-guide/inputs/using-the-cursor.sol"
```

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-cursor.test.mts:imports"

--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-cursor.test.mts:parse-input"
```

## Listing Contract Names

The below example uses a cursor to list the names of all contracts in a source file:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-cursor.test.mts:listing-contract-names"
```

## Visiting Only a Sub-tree

In cases like the above, we needed to visit a sub-tree of the CST (to get the contract name).
But we also need to remember to return the cursor to its original position after each read,
which is inconvenient, and can lead to subtle bugs.

To avoid this, we can use the `spawn()` API,
which cheaply creates a new cursor that starts at the given node, without copying the previous path history.
This lets us visit the sub-tree of each contract, without modifying the original cursor:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-cursor.test.mts:visiting-sub-tree"
```

## Accessing Node Positions

The `Cursor` API also tracks the position and range of the current node it is visiting.
Here is an example that records the source range of each contract, along with its text:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-cursor.test.mts:accessing-node-positions"
```
