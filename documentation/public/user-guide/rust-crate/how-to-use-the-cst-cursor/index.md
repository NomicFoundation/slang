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

Once you have a cursor, you can use it to traverse the CST in the DFS order. The cursor provides several methods for this purpose:

### Navigating the tree

The basic navigation associated functions are:

-   `reset()`: This method resets the cursor to the root of the CST.
-   `is_completed()`: This method returns `true` if the cursor is finished, and `false` otherwise.
-   `go_to_next()`: This method attempts to move the cursor to the next node.
-   `go_to_previous()`: This method attempts to move the cursor to the previous node.

To navigate to a node with a specific relationship to the current node, you can use the following methods:

-   `go_to_parent()`: This method attempts to move the cursor to the parent of the current node.
-   `go_to_next_sibling()`: This method attempts to move the cursor to the next sibling of the current node.
-   `go_to_previous_sibling()`: This method attempts to move the cursor to the previous sibling of the current node.
-   `go_to_next_non_descendent()`: This method attempts to move the cursor to the next node that is not a descendent of the current node (i.e. sibling, falling back to the parent if none left).
-   `go_to_first_child()`: This method attempts to move the cursor to the first child of the current node.
-   `go_to_nth_child(n: usize)`: This method attempts to move the cursor to the nth child of the current node.
-   `go_to_last_child()`: This method attempts to move the cursor to the last child of the current node.

These methods return `false` if the cursor is finished or there's no node with expected relationship to go to.

### Navigating to specific rules/tokens

-   `go_to_next_token()`: This method attempts to move the cursor to the next token node.
-   `go_to_next_token_with_kind(kind: TokenKind)`: This method attempts to move the cursor to the next token of the given kind.
-   `go_to_next_rule(rule: RuleKind)`: This method attempts to move the cursor to the next node of the given rule kind.
-   `go_to_next_token_of_kind(kind: TokenKind)`: This method attempts to move the cursor to the next token of the given kind.
-   `go_to_next_matching(predicate: impl Fn(&Node) -> bool)`: This method attempts to move the cursor to the next node that matches the given predicate.

These methods return `false` if the cursor is finished or there's no token/rule to go to.

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
