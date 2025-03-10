# 8.1. List functions in a contract

This function finds a contract in the compilation unit with the given name and returns a list of `FunctionDefinition` within it. We use a combination of the [Query API](../../06-query-language/02-executing-queries/index.md) and the [AST types](../../05-syntax-trees/04-using-ast-types/index.md):

```ts title="list-functions-in-contract.mts"
--8<-- "documentation/public/user-guide/08-examples/01-list-functions-in-contract/examples/list-functions-in-contract.mts"
```

From the list of `FunctionDefinition`s it's easy to obtain the names of the functions:

```ts title="test-list-functions.test.mts"
--8<-- "documentation/public/user-guide/08-examples/01-list-functions-in-contract/examples/test-list-functions.test.mts"
```
