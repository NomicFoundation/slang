# 8.5. Inject logging

We are going to use the `BaseRewriter` class to achieve something different: insert calls to a `log` function.

```ts title="logging-rewriter.mts"
--8<-- "documentation/public/user-guide/08-examples/05-remove-unused-definitions/examples/cleaner.mts"
```

Again, we test the functionality on the ongoing Solidity example:

```ts title="test-logging-rewriter.mts"
--8<-- "documentation/public/user-guide/08-examples/06-inject-logging/examples/test-logging-rewriter.test.mts"
```
