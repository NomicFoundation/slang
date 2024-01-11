# Concepts

At its core, Slang is a collection of APIs that are meant to analyze the source code, starting with the source code itself and ending with a rich structure that can be reasoned about.
This is a departure from the classic approach of "black-box" compilers, which are handed the input and only their output can be observed.

## Language Versions

To use Slang, you start by initializing a `Language` object with a specific version of the language.
The earliest Solidity version we support is `0.4.11`, and we plan on supporting all future versions as they are released.

From a `Language` object, you can analyze any source text according to the rules of that specific version.
Providing an accurate language version is important, as it affects the shape of the syntax tree, and possible errors produced.
You can use the `Language::getSupportedVersions()` API to get a list of all supported versions for the current Slang release.

The `Language::parse()` API is the main entry point for the parser, and to generate concrete syntax trees (CSTs) that can be used for further analysis.
Each `parse()` operation accepts the input source code, and a `RuleKind` variant.
This allows callers to parse entire source files (`RuleKind::SourceUnit`), individual contracts (`RuleKind::ContractDefinition`),
methods (`RuleKind::FunctionDefinition`), or any other syntax nodes.

The resulting `ParseOutput` object will contain syntax errors (if any), and the syntax tree corresponding to the input source code.

## Concrete Syntax Tree (CST)

Slang is capable of parsing the source code into a Concrete Syntax Tree (CST; also sometimes called "full-fidelity"),
which is a tree structure of the program that also includes things like punctuation or whitespace.

This is done by using the (standard) approach of lexical analysis followed by syntax analysis.
The source text as a sequence of characters is recognized into a sequence of tokens (lexical analysis), which then in turn is _parsed_ into the CST.

The resulting CST is a regular tree data structure that you can visit.
The tree nodes are represented by the `Node` structure, which can be one of two kinds:

-   `RuleNode` (aka _non-terminals_) represent sub-trees, containing a a vector of other `Node` children.
-   `TokenNode` (aka _terminals_) are leaves and represent a lexical token (i.e. an identifier, keyword, punctuation) in the source.

## CST Cursors

For many code analysis tasks, it is useful to traverse the parse tree and visit each node.
The `Cursor` object allows callers to traverse the parse tree in an efficient pre-order manner.

It provides several `goTo*()` navigation functions, each returning `true` if the cursor was successfully moved, and `false` otherwise.
There are three main ways to do it:

-   According to the DFS order, i.e. `goToNext()` and `goToPrevious()`,
-   According to the relationship between the current node and the next node, i.e. `goToParent()`, `goToFirstChild()`, `goToNextNonDescendent()`
-   According to the kind of the next node, i.e. `goToNextTokenWithKind(kind)`, `goToNextRuleWithKind(kind)`

As such, the cursor is stateful and keeps track of the path it has taken through the CST.
It starts at the root it was created at and is completed when it reaches its root when navigating forward.

## Abstract Syntax Tree (AST)

AST types are a set of abstractions that provide a typed view of the untyped CST nodes.
You can convert any untyped CST node to its corresponding AST type using their constructors.

There is a corresponding type for each `RuleKind` (non-terminal) in the language. AST types are immutable.
Additionally, their fields are constructed lazily as they are accessed for the first time.
