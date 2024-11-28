# Using the Cursor

This guide will walk you through the basics of using a [CST cursor](../concepts.md#cst-cursors) in your project.
Let's start with this source file, that contains three contracts:

```solidity title="input.sol"
--8<-- "documentation/public/user-guide/inputs/using-the-cursor.sol"
```

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_cursor.rs:imports"

--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_cursor.rs:parse-input"
```

## Listing Contract Names

The below example uses a cursor to list the names of all contracts in a source file:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_cursor.rs:listing-contract-names"
```

## Visiting Only a Sub-tree

In cases like the above, we needed to visit a sub-tree of the CST (to get the contract name).
But we also need to remember to return the cursor to its original position after each read,
which is inconvenient, and can lead to subtle bugs.

To avoid this, we can use the `spawn()` API,
which cheaply creates a new cursor that starts at the given node, without copying the previous path history.
This lets us visit the sub-tree of each contract, without modifying the original cursor:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_cursor.rs:visiting-sub-tree"
```

## Accessing Node Positions

The `Cursor` API also tracks the position and range of the current node it is visiting.
Here is an example that records the source range of each contract, along with its text:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_cursor.rs:accessing-node-positions"
```

## Using Iterator API

In addition to the procedural-style methods, the `Cursor` struct also implements the `Iterator` trait, which allows you to use it in a functional style.

Let's use that to extract all `Identifier` nodes from the source text using that API:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_cursor.rs:using-iterator-api"
```

!!! note

    It's important to note that `Iterator::next` first visits the current node,
    yields it, and then moves the cursor to the next node.
    As such, accessor associated functions called on the `Cursor` that reference
    the "current" will point to the one that is not yet yielded by the iterator.
    This might be an important, when mixing the two styles.

## Using a Cursor with Labels

The cursor also keeps track of the labels of the nodes it visits.
Let's use that to extract all nodes that are labeled `Name`:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_cursor.rs:using-cursors-with-labels"
```
