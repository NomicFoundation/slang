# 3. Concepts

At its core, Slang is a collection of APIs that are meant to analyze the source code,
starting with the source code itself and ending with a rich structure that can be reasoned about.
This is a departure from the classic approach of "black-box" compilers,
which are handed the input and only their output can be observed.

## Language Grammar

The entire Solidity grammar (Across all supported versions) is defined in the [Solidity Grammar](../../solidity-grammar/index.md) section.
You can use this as a guide when developing your own tools using Slang. The grammar is also included as documentation on top of the corresponding APIs
(like `TerminalKind`, `NonterminalKind`, and `EdgeLabel` variants). It explains the structure of each node, and the relationships between each parent
and its children.

## Parser

The `Parser` API is used to produce a Concrete Syntax Tree (CST) from Solidity source code.
Each `Parser` object is initialized with a specific Solidity version.

With a `Parser` object, you can analyze any source text according to the grammar of that specific version.
Providing an accurate language version is important, as it affects the shape of the syntax tree, and possible errors produced.
Check [the Solidity Grammar](../../solidity-grammar/supported-versions.md) for a list of supported versions.

Each parse operation will produce a `ParseOutput` object, which contains the root of the CST corresponding to the input source code,
and any syntax errors found during parsing.

## Concrete Syntax Trees (CST)

Slang is capable of parsing the source code into a Concrete Syntax Tree (CST), which is a tree structure representing the entire source code.
It includes the contracts, functions, statements, and expressions within. It also includes things like comments, whitespace, and punctuation.
This is sometimes called a "full-fidelity" CST, and it can be used to reconstruct the original source code when needed.

The tree nodes are represented by the `Node` structure, which can be one of two kinds:

- `NonterminalNode` represents parent nodes, possibly containing children (sub-trees).
- `TerminalNode` represents leaves (i.e. an identifier, keyword, or punctuation) in the tree.

Nodes are connected through `Edge` objects, which contain a single child `Node`, and an `EdgeLabel` that describes the relationship between the parent and child.

You can find a complete list of all grammar nodes and edges in the [Solidity Grammar](../../solidity-grammar/index.md) section.

## Cursors

For many code analysis tasks, it is useful to traverse the parse tree and visit each node.
The `Cursor` object allows callers to traverse the parse tree in an efficient pre-order manner.

It provides several `goTo*()` navigation functions, each returning `true` if the
cursor was successfully moved, and `false` otherwise. There are three main ways
to do it:

- According to the order they appear in the tree, i.e. `goToNext()` and `goToPrevious()`,
- According to the relationship between the current node and the next node, i.e. `goToParent()`, `goToFirstChild()`, `goToNextNonDescendant()`
- According to the kind of the next node, i.e. `goToNextTerminalWithKind(kind)`, `goToNextNonterminalWithKind(kind)`

As such, the cursor is stateful and keeps track of the path it has taken through the CST.
It starts at the root it was created at and is completed when it reaches its root when navigating forward.

Cursors can also be cloned, to allow for multiple traversals of the same tree,
or spawned, to allow for traversing the subtree of the current node.

## Queries

The `Cursor` API is a low-level API that allows you to traverse the CST in a
procedural manner. However, it is often more convenient to use the declarative
`Query` API. Queries allow you to express your intent more concisely, and also
allows you to reuse the same query in multiple places. Queries can largely
replace the need for both internal (cursor), and external (visitor) iterator
patterns.

The tree query language is based on pattern matching, and the
execution semantics are closer to unification than to regular expression
matching i.e. a query returns all possible matches, not just the
longest/shortest/first/last match. There is no concept of a 'greedy' operator
for example.

Query execution is based on cursors, and the resulting matches and unification
captures are returned as cursors as well. This allows you to mix and match
manual traversal, cursors, and queries.

Multiple queries can be executed in a batch, and efficiently traverse the tree
looking for matches. This mode of operation can replace all visitor patterns.

## Abstract Syntax Trees (AST)

AST types are a set of abstractions that provide a typed view of the untyped CST nodes.
You can convert any untyped CST node to its corresponding AST type using their constructors.

There is a corresponding type for each `NonterminalKind` in the language. AST types are immutable.
Additionally, their fields are constructed lazily as they are accessed for the first time.

AST nodes maintain a reference to the CST node they were constructed from,
and can be used to navigate to the corresponding CST node.

## Compilation Unit

A `CompilationUnit` is a collection of all source files that should be compiled together.
This includes your main contract, and any imported files or dependencies that are used there.

A `CompilationBuilder` is used to build a `CompilationUnit`. It is provided with a Solidity version,
then the list of source files are incrementally added to it in any order. With each source file added,
the builder will analyze all import statements within, and ask the user to resolve them to the imported source files,
and continue loading/analyzing them, until it is complete.

## Binding Graph

The `BindingGraph` is a graph structure that represents the relationships between identifiers across source files in a `CompilationUnit`.
For each identifier, it will analyze if it is acting as a `Definition` or a `Reference` (aliases for example), and provide an API for users to resolve them.

It can also be used to query the relationships between them;
finding all references to a specific definition,
or all definitions that are bound by a specific reference.
