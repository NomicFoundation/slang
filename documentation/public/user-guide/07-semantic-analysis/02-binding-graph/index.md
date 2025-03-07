# 7.2. Binding Graph

The binding graph is a graph structure that represents the relationships between identifiers across source files in a `CompilationUnit`.
It stores cursors to all definitions and references, and can resolve the edges between them.

Building this graph can be an expensive operation. So, it is constructed lazily on the first access, and cached thereafter.
You can use cursors to query the graph for definitions or references.

## Resolving Definitions

TODO: explain definitions, and how they can be used.

```ts title="resolving-definitions.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/01-resolving-definitions.test.mts"
```

## Resolving References

TODO: explain references, and how they can be used.

```ts title="resolving-references.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/02-resolving-references.test.mts"
```
