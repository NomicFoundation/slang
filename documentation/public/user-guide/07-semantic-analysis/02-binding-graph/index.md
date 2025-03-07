# 7.2. Binding Graph

The binding graph is a graph structure that represents the relationships between identifiers across source files in a `CompilationUnit`.
It stores cursors to all definitions and references, and can resolve the edges between them.

Building this graph can be an expensive operation. So, it is constructed lazily on the first access, and cached thereafter.
You can use cursors to query the graph for definitions or references.

## Resolving Definitions

To resolve definitions we need to provide the binding graph with a cursor pointing to the identifier. Some identifiers in the code may not be acting as definitions. In those cases, `definitionAt()` will return `undefined`.

Identifiers are terminal nodes whose `kind` test true for `TerminalKindExtensions.isIdentifier()`. In Solidity, there are two kinds: `Identifier` and `YulIdentifier`.

`Definition` objects will contain two binding locations:

- `nameLocation` referring to the identifier that resolved to the definition
- `definiensLocation` referring to the CST node that is being defined (a contract, function, struct, etc)

Because binding graphs span multiple files, these locations are not simple `Cursor` objects. Instead they are `BindingLocation` objects, which can refer to locations in user files or built-ins:

- `UserFileLocation` in turn contains the `fileId` and the `cursor` in the CST tree of the file.
- `BuiltInLocation` refers to a location in system defined built-in. You may get a definition on such a location when finding which definitions a `Reference` binds to (see section below), but never when resolving to a definition from a cursor.

```ts title="find-definitions.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/find-definitions.mts"
```

User file binding locations will also contain a `cursor` to the underlying identifier or CST node of the entity defined. Using the same contract from last section, we can look for definitions in the `contract.sol` file:

```ts title="resolving-definitions.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/01-resolving-definitions.test.mts"
```

We find 3 definitions:

- the `Log` imported symbol
- the `MyContract` contract
- the `test` method

The `Log` import symbol is a special case and also acts as a reference to the actual event type defined in `events.sol`. Let's find all references in the file next.

## Resolving References

In the same way to resolving definitions, we can also attempt to resolve a cursor to an identifier to a reference. If the resolution is successful, the returned `Reference` will have a `location` pointing to the identifier. As before, we can expect this location to be in the user file whose CST tree we are querying.

```ts title="find-references.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/find-references.mts"
```

We can now find all the references in the same `contract.sol` file:

```ts title="resolving-references.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/02-resolving-references.test.mts"
```

## Navigating between definitions and references

Iterating over the references found in the last section, we can find where are the definitions they refer to by using the `definitions()` method of `Reference`:

```ts title="references-to-definitions.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/03-references-to-definitions.test.mts"
```

There are two interesting observations here:

- Slang recognizes the Solidity built-in global variable `msg` and its member `sender`. See next section for more details and an example use case.
- There may be multiple definitions bound to our reference: the `Log` identifier in the `emit` statement refers to the imported symbol, and also to the event type which is declared in a different file.

There are other cases where Slang may return multiple definitions for a reference. Function overloads and virtual method calls are typical examples.

Starting from a definition, we can also query the binding graph for all places where it's being referred to with the method `references()` of `Definition` objects. In this example, we navigate from the `Log` event definition in `events.sol` back to the two references in `contract.sol`:

```ts title="definitions-to-references.mts"
--8<-- "documentation/public/user-guide/07-semantic-analysis/02-binding-graph/examples/04-definitions-to-references.test.mts"
```
