# Using a CST Cursor

The CST (Concrete Syntax Tree) cursor is a powerful tool that allows you to traverse a CST in a depth-first search (DFS) pre-order fashion.
This guide will walk you through the basics of using a CST cursor in your project.

## Creating a Cursor

First, you need to create an instance of the `Cursor` struct. This is done as follows:

```rust
let language = Language::new(Version::new(0, 8, 0))?;
let parse_tree = language.parse(RuleKind::SourceUnit, &source_code_str);
// ...
let cursor = parse_tree.create_tree_cursor();
```

## Traversing the CST procedurally

Once you have a cursor, you can use it to traverse the CST in the DFS order. The cursor provides several `go_to_*`/`goTo*` navigation functions
for this purpose; each returning `true` if the cursor was successfully moved, and `false` otherwise.

There are three main ways to do it:

-   according to the DFS order, i.e. next/previous nodes,
-   according to the relationship between the current node and the next node, e.g. siblings/children/parent,
-   according to the type of the next node, e.g. next token/rule.

As such, the cursor is stateful and keeps track of the path it has taken through the CST.
It starts at the root it was created at and is completed when it reaches its root when navigating forward.

### Example

The below example uses a cursor to collect the names of all contracts in a source file, and returns them as a `Vec<String>`:

```solidity title="input.sol"
--8<-- "/home/xanewok/repos/slang/crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.sol"
```

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.rs:example-list-contract-names"
```

## Traversing the CST using the cursor as an iterator

In addition to the procedural-style methods, the `Cursor` struct also implements the `Iterator` trait, which allows you to use it in a functional style.

The iterator yields `Node` structs, which represent the CST nodes.

It's important to note that `Iterator::next` first visits the current node, yields it, and then moves the cursor to the next node.
As such, accessor associated functions called on the `Cursor` that reference the "current" will point to the one that is not yet yielded by the iterator. This might be an important, when mixing the two styles.

### Example

```solidity title="input.sol"
--8<-- "/home/xanewok/repos/slang/crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.sol"
```

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.rs:example-using-iter"
```

## Visiting only a sub-tree

Sometimes, it's useful to only visit a sub-tree of the CST. In order to do that, we can use the `Cursor::spawn` function,
which creates a new cursor that starts at the given node, not copying the previous path history.

### Example

```solidity title="input.sol"
--8<-- "/home/xanewok/repos/slang/crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.sol"
```

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.rs:example-using-spawn"
```

## Accessing the current node

The `Cursor` struct provides several methods that allow you to access the currently visited node:

-   `node()`: This method returns the currently visited node.
-   `node_name()`: This method returns the name of the currently visited node.
-   `text_offset()`: This method returns the text offset that corresponds to the beginning of the currently pointed to node.
-   `text_range()`: This method returns the text range that corresponds to the currently pointed to node.
-   `depth()`: This method returns the depth of the current node in the CST, i.e. the number of ancestors.
-   `ancestors()`: This method returns an iterator over the current node's ancestors, starting from the cursor root node.

## Using a Cursor with Names

In addition to the basic ` Cursor`, there's also a `CursorWithNames` struct that keeps track of the names of the nodes it visits. You can create a `CursorWithNames` from a `Cursor` as follows:

```rust
let cursor_with_names = cursor.with_names();
```

You can then use the `CursorWithNames` in the same way as a regular `Cursor`.

### Example

```solidity title="input.sol"
--8<-- "/home/xanewok/repos/slang/crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.sol"
```

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.rs:example-using-cursor-with-names"
```
