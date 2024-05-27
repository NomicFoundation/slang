# Using Queries

It's often more convenient to use the declarative `Query` API to traverse the CST, as they allow you to express your intent more concisely and can largely replace the need for both internal (cursor), and external (visitor) iterator patterns.

The [query language](./../tree-query-language.md) is based on pattern matching, and the execution semantics are closer to unification than to regular expression matching. A query returns all possible matches, not just the longest/shortest/first/last match.

If not specified otherwise, let's assume we already parsed a Solidity source and have a `cursor` pointing to the root node of the CST (created with `createTreeCursor`, see [Using the Cursor](./using-the-cursor.md)).

## Creating and executing queries

You can create a `Query` object using `Query.parse`, which accepts a string value. These can be then used by `Cursor.query` to execute it.

You can pass multiple queries to a cursor to and efficiently traverse the tree looking for matches. They will be executed concurrently, returning matches in the order they appear in input.

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-queries.ts:creating-a-query"
```

## Iterating over node patterns

Queries allow you to iterate over all node patterns that match the query, which can replace your need for manual iteration via cursors or visitors. In order to get a `Cursor` that points to the matched node, you need to capture them with a name capture (`@capture_name`) to a specific node in the query pattern.

Let's use this to list all the contract definitions in the source file:

```solidity title="input.sol"
--8<-- "documentation/public/user-guide/inputs/using-the-cursor.sol"
```

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-queries.ts:visiting-contracts"
```

### Multiple patterns simultaneously

We can also intersperse multiple patterns in a single query, which will return all the matches for each pattern. This can be useful when you want to match multiple types of nodes in a single pass.

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-queries.ts:multiple-patterns"
```

## Matching on node's label

We can match not only on the node's kind, but also on its label. This can be useful if there may be two children with the same kind but different labels or to be more declarative.

To do so, we use `[label: _]` syntax. Here, we also use `_` to allow matching any kind of node, as long as it matches the given label.

```solidity title="input.sol"
--8<-- "documentation/public/user-guide/inputs/typed-tuple.sol"
```

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-queries.ts:matching-on-label"
```

## Matching on node's literal content

Lastly, we can also match on the node's literal content. This can be useful when you want to match a specific identifier, string, or number.

Let's say we prefer our code to be explicit and prefer using `uint256` instead of `uint`. To find all instances of the `uint` alias we could do the following:

```solidity title="input.sol"
--8<-- "documentation/public/user-guide/inputs/typed-tuple.sol"
```

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-queries.ts:matching-on-literal-value"
```

## Example: Finding `tx.origin` patterns

As a more realistic example, let's say we want to write a linter that unconditionally lints against all [`tx.origin`](https://docs.soliditylang.org/en/latest/security-considerations.html#tx-origin) accesses.

Let's use the motivating example from [https://soliditylang.org](https://docs.soliditylang.org/en/latest/security-considerations.html#tx-origin):

```solidity title="input.sol"
--8<-- "documentation/public/user-guide/inputs/tx-origin.sol"
```

Now, we can above features to write a query that matches all `tx.origin` patterns:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-queries.ts:tx-origin"
```
