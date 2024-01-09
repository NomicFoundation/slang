# Using a CST Cursor

The CST (Concrete Syntax Tree) cursor is a powerful tool that allows you to traverse a CST in a depth-first search (DFS) pre-order fashion.
This guide will walk you through the basics of using a CST cursor in your project.

## Creating a Cursor

First, you need to create an instance of the `Cursor` struct. This is done as follows:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/cursor-api.ts:create-cursor"
```

## Traversing the CST procedurally

Once you have a cursor, you can use it to traverse the CST in the DFS order. The cursor provides several `goTo*` navigation functions
for this purpose; each returning `true` if the cursor was successfully moved, and `false` otherwise.

There are three main ways to do it:

-   according to the DFS order, i.e. next/previous nodes,
-   according to the relationship between the current node and the next node, e.g. siblings/children/parent,
-   according to the type of the next node, e.g. next token/rule.

As such, the cursor is stateful and keeps track of the path it has taken through the CST.
It starts at the root it was created at and is completed when it reaches its root when navigating forward.

The below example uses a cursor to collect the names of all contracts in a source file, and returns them as a `Vec<String>`:

```solidity title="input.sol"
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api/base.sol"
```

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/cursor-api.ts:example-list-contract-names"
```

## Visiting only a sub-tree

Sometimes, it's useful to only visit a sub-tree of the CST. In order to do that, we can use the `Cursor::spawn` function,
which creates a new cursor that starts at the given node, not copying the previous path history.

```solidity title="input.sol"
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api/base.sol"
```

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/cursor-api.ts:example-using-spawn"
```

## Accessing the current node

The `Cursor` struct provides several methods that allow you to access the currently visited node, its position in the source code
and its ancestors.

```solidity title="input.sol"
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api/node_accessors.sol"
```

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/cursor-api.ts:example-node-accessors"
```

The CST children nodes are often named. Sometimes, it might be more convenient to lookup and access the node by its name, like so:

```solidity title="input.sol"
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api/base.sol"
```

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/cursor-api.ts:example-using-cursor-with-names"
```
