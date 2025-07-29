# 8.5. Remove unused definitions

In the previous section we've seen how to find unused definitions. Let's now proceed to remove them. For that, we are going to use the `BaseRewriter`, an abstract class that allow us to replace nodes in a tree. In our case, what we're going to do is to simply remove those that are unused:

```ts title="cleaner.mts"
--8<-- "documentation/public/user-guide/08-examples/05-remove-unused-definitions/examples/cleaner.mts"
```

We can test the functionality on the same Solidity example from last section:

```ts title="test-find-unused-definitions.mts"
--8<-- "documentation/public/user-guide/08-examples/05-find-unused-definitions/examples/test-remove-unused-definitions.test.mts"
```
