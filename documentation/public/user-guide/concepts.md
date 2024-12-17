# Concepts

At its core, Slang is a collection of APIs that are meant to analyze the source code, starting with the source code itself and ending with a rich structure that can be reasoned about.
This is a departure from the classic approach of "black-box" compilers, which are handed the input and only their output can be observed.

## Language Versions

To use Slang, you start by initializing a `Parser` object with a specific version of the language.
The earliest Solidity version we support is `0.4.11`, and we plan on supporting all future versions as they are released.

From a `Parser` object, you can analyze any source text according to the nonterminals of that specific version.
Providing an accurate language version is important, as it affects the shape of the syntax tree, and possible errors produced.
You can use the `LanguageFacts::supportedVersions()` API to get a list of all supported versions for the current Slang release.

The `Parser::parse()` API is the main entry point for the parser, and to generate concrete syntax trees (CSTs) that can be used for further analysis.
Each `parse()` operation accepts the input source code, and a `NonterminalKind` variant.
This allows callers to parse entire source files (`NonterminalKind::SourceUnit`), individual contracts (`NonterminalKind::ContractDefinition`),
methods (`NonterminalKind::FunctionDefinition`), or any other syntax nodes.

The resulting `ParseOutput` object will contain syntax errors (if any), and the syntax tree corresponding to the input source code.

## Concrete Syntax Tree (CST)

Slang is capable of parsing the source code into a Concrete Syntax Tree (CST; also sometimes called "full-fidelity"),
which is a tree structure of the program that also includes things like punctuation or whitespace.

This is done by using the (standard) approach of lexical analysis followed by syntax analysis.
The source text as a sequence of characters is recognized into a sequence of
terminals (lexical analysis), which then in turn is _parsed_ into the CST.

The resulting CST is a regular tree data structure that you can visit.
The tree nodes are represented by the `Node` structure, which can be one of two kinds:

- `NonterminalNode` represent sub-trees, containing a vector of other `Node` children.
- `TerminalNode` are leaves and represent a terminal (i.e. an identifier, keyword, punctuation) in the source.

## CST Cursors

For many code analysis tasks, it is useful to traverse the parse tree and visit each node.
The `Cursor` object allows callers to traverse the parse tree in an efficient pre-order manner.

It provides several `goTo*()` navigation functions, each returning `true` if the
cursor was successfully moved, and `false` otherwise. There are three main ways
to do it:

- According to the DFS order, i.e. `goToNext()` and `goToPrevious()`,
- According to the relationship between the current node and the next node, i.e. `goToParent()`, `goToFirstChild()`, `goToNextNonDescendant()`
- According to the kind of the next node, i.e. `goToNextTerminalWithKind(kind)`, `goToNextNonterminalWithKind(kind)`

As such, the cursor is stateful and keeps track of the path it has taken through the CST.
It starts at the root it was created at and is completed when it reaches its root when navigating forward.

## CST Queries

The `Cursor` API is a low-level API that allows you to traverse the CST in a
procedural manner. However, it is often more convenient to use the declarative
`Query` API. Queries allow you to express your intent more concisely, and also
allows you to reuse the same query in multiple places. Queries can largely
replace the need for both internal (cursor), and external (visitor) iterator
patterns.

The [query language](./tree-query-language.md) is based on pattern matching, and the
execution semantics are closer to unification than to regular expression
matching i.e. a query returns all possible matches, not just the
longest/shortest/first/last match. There is no concept of a 'greedy' operator
for example.

Query execution is based on `Cursor`s, and the resulting matches and unification
captures are returned as `Cursor`s as well. This allows you to mix and match
manual traversal, cursors, and queries.

Multiple queries can be executed in a batch, and efficiently traverse the tree
looking for matches. This mode of operation can replace all visitor patterns.

## Abstract Syntax Tree (AST)

AST types are a set of abstractions that provide a typed view of the untyped CST nodes.
You can convert any untyped CST node to its corresponding AST type using their constructors.

There is a corresponding type for each `NonterminalKind` in the language. AST types are immutable.
Additionally, their fields are constructed lazily as they are accessed for the first time.

AST nodes maintain a reference to the CST node they were constructed from,
and can be used to navigate to the corresponding CST node.
