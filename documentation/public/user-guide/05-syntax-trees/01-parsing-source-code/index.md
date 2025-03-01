# 5.1. Using the Parser

Using the API directly provides us with a more fine-grained control over the parsing process.
It allows us to parse not just the input as a top-level source unit, but also individual constructs like contracts, various definitions, and even expressions.

## Parsing Source Files

Let's start with this simple source file, that contains a single contract, and parse it into a concrete syntax tree.
The parser will produce a `ParseOutput` object, which contains a `SourceUnit` root node:

```ts title="parsing-source-files.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/01-parsing-source-code/examples/01-parsing-source-files.test.mts"
```

## Parsing Nonterminals

The parser API also allows you to parse specific nonterminal nodes, like statements or expressions.
This is useful when you want to parse a snippet, and not an entire source file, like the `AdditiveExpression` node below:

```ts title="parsing-nonterminals.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/01-parsing-source-code/examples/02-parsing-nonterminals.test.mts"
```

## Handling Syntax Errors

If there are syntax errors with the input, the `ParseOutput` object will contain the list of errors found.
Each error will have a message, and a `TextRange` that indicates the location of the error in the input.

Additionally, the parsed tree will contain an error node for each error encountered. For example, the missing contract name below:

```ts title="handling-syntax-errors.mts"
--8<-- "documentation/public/user-guide/05-syntax-trees/01-parsing-source-code/examples/03-handling-syntax-errors.test.mts"
```
