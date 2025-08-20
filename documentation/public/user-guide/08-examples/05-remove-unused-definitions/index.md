# 8.5. Remove unused definitions

In the previous section we've seen how to find unused definitions. Let's now proceed to remove them. For that, we are going to use `BaseRewriter`, an abstract class that allow us to replace nodes in a tree.

The task, simply put, is to delete those nodes whose id is present in the list of unused definitions (constructed previously). For this example, we focus only in three definitions: functions, state variables, and modifiers. Expanding to other definitions is trivial.

Our `RemoveUnusedDefs` class inherits from `BaseRewriter`, and overrides those methods of interest: `rewriteFunctionDefinition`, `rewriteStateVariableDefinition`, and `rewriteModifierDefinition`. In turn, each method forwards the execution to the helper function `removeUnused`, which is the one checking if the node id is present in the provided list of unused definitions.

If the node corresponds to an unused definition, then the helper ——and therefore, the overwritten method—— returns `undefined`. This is the way in which we signal the `BaseRewriter` class to remove the node.

If the node is not in the list, it's returned as-is.

```ts title="remove-unused-defs.mts"
--8<-- "documentation/public/user-guide/08-examples/05-remove-unused-definitions/examples/remove-unused-defs.mts"
```

We can test the functionality on the same Solidity example from last section, noting that functions `checkOwner` in `Ownable` and `unusedDecrement` in `Owner` are not present in the result, as well as the state variable `_unused` in the latter contract.

```ts title="test-find-unused-definitions.mts"
--8<-- "documentation/public/user-guide/08-examples/05-remove-unused-definitions/examples/test-remove-unused-definitions.test.mts"
```
