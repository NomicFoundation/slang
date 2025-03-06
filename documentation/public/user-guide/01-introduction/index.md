# 1. Introduction

Welcome to the Slang user guide! This aims to be an introduction to Slang itself, its concepts and also contains a collection of guides how you can achieve basic tasks with it.

## What is Slang?

Slang is intended to be a modular Solidity compiler, specifically targeting code analysis and developer tooling. This means servicing tools with domain-specific APIs and, in general, facilitating working with and analyzing the Solidity source code. If you're in the editor writing Solidity or performing linting or additional validation, there's a chance that you are, or could be, running Slang!

To get a good grasp on the concepts used in Slang, see the [Concepts](../03-concepts/index.md) page.

## What Slang is not?

First and foremost, it is not a replacement for `solc`, the standard Solidity compiler. We do not plan at the moment to support emitting optimized EVM bytecode for use in production. Secondly, it does not perform formal verification of contracts or Solidity logic in general. However, other tools that serve this purpose are intended to be built on top of it.

## Supporting multiple versions

The Solidity programming language has evolved quite a bit since its inception. Some features were introduced, some changed, while some eventually became obsolete and were removed altogether.

While it's good for a programming language to evolve and better serve the needs of its users, not being able to easily upgrade or re-deploy existing contracts poses a unique challenge. Developer tooling must be able to understand and consume older contracts that are still being used on the blockchain, written in older versions of Solidity.

Because of that, Slang must be able to reason about different versions of Solidity; how the language grammar, name capture rules, and semantics have changed [across different versions](../../solidity-grammar/supported-versions.md). One of our goals is to document differences as part of our [Solidity Grammar](../../solidity-grammar/index.md).

This is why, instead of having to download separate versions of the tool for each Solidity version, you can access the Slang language APIs by simply specifying the Solidity version that you want to work with.

## Error Tolerance as a feature

A core feature of Slang is that it is designed to be tolerant of errors. All of its APIs don't expect the input to be correct or complete.

For example, when users are actively editing code in an IDE, the code will often have syntax errors or incomplete constructs.
In that case, the parser will detect syntax errors, and instead of just stopping at the first error, it will mark the erroneous
part as either unrecognized or missing, creating an error node, and continue to parse the rest of the file
([example](../05-syntax-trees/01-parsing-source-code/index.md#handling-syntax-errors)).

Another example is the binding graph, which will work even when there are missing source files. The graph will mark
any missing definitions as `undefined`, and still resolve all other symbols in files that do exist.
This is critical for tools that need to work with Solidity code in any environment,
even when dependencies are missing or not yet installed locally
([example](../07-semantic-analysis/02-binding-graph/index.md#resolving-definitions)).

## Distributions

Slang itself is written in Rust, compiled as a WASM component, and distributed as an [npm package](https://www.npmjs.com/package/@nomicfoundation/slang) with a TypeScript interface. In the future, we are also looking into publishing it as a Rust crate, a Python library, and possibly more.
