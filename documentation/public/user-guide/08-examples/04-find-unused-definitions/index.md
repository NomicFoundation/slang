# 8.4. Find unused definitions

Let's now explore a more complex example. We want to find all definitions in a compilation unit that are not referenced from a main contract. The general idea of the algorithm to implement is as follows: starting from the given contract, mark all constructors and public functions, as well as any inherited contracts or implemented interfaces. After that, recursively process (visit) each marked definition following references we find inside them.

As a result, we'll get a list of definitions that are reachable from our main contract. We get the unused definitions by subtracting this set from all the definitions available in the compilation unit. An important thing to note is that this will include nested definitions. For example, variables and parameters in an unused function, or fields in an unused struct. To clean this up, we make a final pass to remove definitions nested inside other unused definitions.

**Disclaimer:** This is only an example and to keep it relatively short, we won't cover all possible corner cases. The resulting algorithm _may_ produce both false positives and false negatives.

```ts title="find-unused-definitions.mts"
--8<-- "documentation/public/user-guide/08-examples/04-find-unused-definitions/examples/find-unused-definitions.mts"
```

To implement the above we use several auxiliary functions. First and foremost, we need a way to collect definitions, both in the whole compilation unit, as well as under a specific cursor. We can accomplish this using the [Cursor API](../../05-syntax-trees/03-navigating-with-cursors/index.md) and the [Binding Graph API](../../07-semantic-analysis/02-binding-graph/index.md):

```ts title="collect-definitions.mts"
--8<-- "documentation/public/user-guide/08-examples/04-find-unused-definitions/examples/collect-definitions.mts"
```

The other important piece of the puzzle is the `visitDefinition` function. This is where we decide what is reachable from a definition, and what we do here depends on the kind we're visiting, aka. the `kind` of the definiens node of the definition. For `FunctionDefinition` and `ModifierDefinition`, we want to follow all references enclosed in them to visit the definitions they bind to. For libraries, structs and enums, we want to do no further processing to be able to detect which fields and members are actually being used.

As we said earlier, for a `ContractDefinition`, public state variables and functions should be marked and queued for visiting later. Constructors and other special functions do not reify definitions (as they are not named by an identifier), so we want to visit their declarations immediately to find references in them. All this behavior is complex enough to warrant its own, separate function `visitContract`. We use [queries](../../06-query-language/02-executing-queries/index.md) to find the different components of the contract.

```ts title="visit-definition.mts"
--8<-- "documentation/public/user-guide/08-examples/04-find-unused-definitions/examples/visit-definition.mts"
```

When analyzing functions bodies, expressions and blocks of statements, we need to follow all references to their bound definitions to mark them as used. This again is simple to implement with the [Cursor](../../05-syntax-trees/03-navigating-with-cursors/index.md) and [Binding Graph](../../07-semantic-analysis/02-binding-graph/index.md) APIs:

```ts title="follow-all-references.mts"
--8<-- "documentation/public/user-guide/08-examples/04-find-unused-definitions/examples/follow-all-references.mts"
```

Lastly, we need a way to locate the initial contract. This is similar to what we implemented in the [first example](../01-list-functions-in-contract/index.md), but now we want to return a Binding Graph `Definition` object to kickstart our algorithm:

```ts title="find-contract-by-name.mts"
--8<-- "documentation/public/user-guide/08-examples/04-find-unused-definitions/examples/find-contract-by-name.mts"
```

Finally, we can test the functionality on a slightly larger Solidity example:

```ts title="test-find-unused-definitions.mts"
--8<-- "documentation/public/user-guide/08-examples/04-find-unused-definitions/examples/test-find-unused-definitions.test.mts"
```
