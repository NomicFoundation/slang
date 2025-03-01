# 6.2. Executing Queries

If not specified otherwise, let's assume we already parsed a Solidity source and have a [Cursor](../../05-syntax-trees/02-navigating-with-cursors/index.md) pointing
to the root node of the CST.

## Creating and executing queries

You can create a `Query` object using `Query.create()`, which accepts a string value.
These can be then used by `cursor.query()` to execute it.

You can pass multiple queries to a cursor to and efficiently traverse the tree looking for matches.
They will be executed simultaneously, returning matches in the order they appear in input.

```ts title="common.mts"
--8<-- "documentation/public/user-guide/06-query-language/02-executing-queries/examples/common.mts"
```

## Iterating over node patterns

Queries allow you to iterate over all node patterns that match the query, which can replace your need for manual iteration via cursors or visitors.
Each match has a cursor that points to the root matched node.

Let's use this to list all the contract definitions in a source file:

```ts title="match-roots.mts"
--8<-- "documentation/public/user-guide/06-query-language/02-executing-queries/examples/01-match-roots.test.mts"
```

## Capturing nodes by name

You can also capture specific nodes in the query by name, and get a cursor to each of them:

Let's use this to list all the contract names:

```ts title="match-captures.mts"
--8<-- "documentation/public/user-guide/06-query-language/02-executing-queries/examples/02-match-captures.test.mts"
```

## Detecting Query errors

If there is a mistake in your query, for example, if you use an invalid node kind, you will get a `QueryError` exception.
The error will contain a message to indicate what went wrong, and the text range of the error.

```ts title="query-errors.mts"
--8<-- "documentation/public/user-guide/06-query-language/02-executing-queries/examples/03-query-errors.test.mts"
```

## Multiple queries simultaneously

We can also execute multiple queries simultaneously, which will return all the matches for as they are found in the tree.
This can be useful when you want to match multiple types of nodes in a single pass.
Results will be reported in order, and each will have an index that can be used to identify which query is matched.

```ts title="multiple-queries.mts"
--8<-- "documentation/public/user-guide/06-query-language/02-executing-queries/examples/04-multiple-queries.test.mts"
```
