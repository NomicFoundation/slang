# 7.2. Binding Graph

The binding graph is a graph structure that represents the relationships between identifiers across source files in a `CompilationUnit`.
It stores cursors to all definitions and references, and can resolve the edges between them.

Building this graph can be an expensive operation. So, it is constructed lazily on the first access, and cached thereafter.
You can use cursors to query the graph for definitions or references.

## Resolving Definitions

Using the example compilation unit from the [last chapter](../01-compilation-units/index.md), let's find the definition of the event `Log` that is emitted in the contract. The easiest way to do this is to get a cursor to the `Log` identifier in the `events.sol` file.

Then, from that cursor pointing to the `Identifier`, we can get the `EventDefinition` itself that the identifier names, as well as all places in the compilation unit that reference this event type.

```ts title="resolving-definitions.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/01-resolving-definitions.test.mts"
```

This example uses some utility functions to abstract common operations, defined as follows:

```ts title="utils.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/utils.mts"
```

## Resolving References

Conversely, we can start from an `Identifier` acting as a reference and navigate to definition through the binding graph. This time, we'll search for `Log` in `contract.sol`:

```ts title="resolving-references.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/02-resolving-references.test.mts"
```
