# 5.2. Handling Syntax Errors

If there are syntax errors with the input, the `ParseOutput` object will contain the list of errors found.
Each error will have a message, and a `TextRange` that indicates the location of the error in the input.

Additionally, the parsed tree will contain an error node for each error encountered, which can be one of two kinds, missing or unrecognized, depending on the error.

You can use the `TerminalKindExtensions.isValid()` API to check if a terminal is valid or is an error node.

## Missing Nodes

These occur when the parser expects a certain node to be present, but it is not.
It will create a missing node in its place, and continue parsing as if it was present.

```ts title="missing-error-nodes.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/02-handling-syntax-errors/examples/01-missing-error-nodes.test.mts"
```

## Unrecognized Nodes

These occur when the parser encounters a token that it does not recognize, and it cannot parse.
It will create an unrecognized node in its place, and continue parsing as if it doesn't exist.

```ts title="unrecognized-error-nodes.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/02-handling-syntax-errors/examples/02-unrecognized-error-nodes.test.mts"
```
