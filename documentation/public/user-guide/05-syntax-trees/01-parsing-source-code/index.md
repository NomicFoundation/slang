# 5.1. Using the Parser

The Parser API provides us with fine-grained control over the parsing process.
It allows us to parse not just the input as a top-level source unit, but also individual constructs like contracts, various definitions, and even expressions.

## Parsing Source Files

Let's start with this simple source file, that contains a single contract, and parse it into a concrete syntax tree.
The parser will produce a `ParseOutput` object, which contains a [`SourceUnit`](../../../solidity-grammar/01-file-structure/02-source-unit.md) root node:

```ts title="parsing-source-files.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/01-parsing-source-code/examples/01-parsing-source-files.test.mts"
```

## Parsing Nonterminals

The parser API also allows you to parse specific nonterminal nodes, like statements or expressions.
This is useful when you want to parse a snippet, and not an entire source file, like the [`AdditiveExpression`](../../../solidity-grammar/05-expressions/01-base-expressions.md) node below:

```ts title="parsing-nonterminals.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/01-parsing-source-code/examples/02-parsing-nonterminals.test.mts"
```

## Handling Trivia Nodes

Trivia nodes represent comments, whitespace, newlines, and other non-essential terminals
that can appear anywhere in the source code, and they are categorized by the parser into two groups:

- Leading Trivia: terminals that appear before a significant terminal, and can span multiple lines (for example, documentation comments).
- Trailing Trivia: terminals that appear after a significant terminal on the same line, leading to, and including, the following newline terminal.

You can use the `TerminalKindExtensions.isTrivia()` API to check if a terminal is a trivia terminal.

```ts title="handling-trivia.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/01-parsing-source-code/examples/03-handling-trivia.test.mts"
```
