# 8. Examples

- [8.1. List functions in a contract](./01-list-functions-in-contract/index.md)
- [8.2. Find usages](./02-find-usages/index.md)
- [8.3. Jump to definition](./03-jump-to-definition/index.md)
- [8.4. Find unused definitions](./04-find-unused-definitions/index.md)
- [8.5. Remove unused definitions](./05-remove-unused-definitions/index.md)
- [8.6. Inject logging](./06-inject-logging/index.md)

We need some setup code to [create a `CompilationUnit`](../07-semantic-analysis/01-compilation-units/index.md) with the contents of one or more Solidity files. This will be common to all examples above:

```ts title="compilation-builder.mts"
--8<-- "documentation/public/user-guide/08-examples/common/compilation-builder.mts"
```

Unless specified otherwise, throughout the examples in this section we will use this simple contract for demonstration:

```ts title="sample-contract.mts"
--8<-- "documentation/public/user-guide/08-examples/common/sample-contract.mts"
```

We also need an easy way to obtain a `Cursor` pointing to a terminal node in a specific line and column in a file for a couple of examples. This can be achieved using the [Cursor Navigation API](../05-syntax-trees/03-navigating-with-cursors/index.md):

```ts title="find-terminal-node-at.mts"
--8<-- "documentation/public/user-guide/08-examples/common/find-terminal-node-at.mts"
```
