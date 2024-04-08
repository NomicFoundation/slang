# Introduction

Welcome to the Slang user guide! This aims to be an introduction to Slang itself, its concepts and also contains a collection of guides how you can achieve basic tasks with it.

## What is Slang?

Slang is intended to be a modular Solidity compiler, specifically targeting code analysis and developer tooling. This means servicing tools with domain-specific APIs and, in general, facilitating working with and analyzing the Solidity source code. If you're in the editor writing Solidity or performing linting or additional validation, there's a chance that you are, or could be, running Slang!

To get a good grasp on the concepts used in Slang, see the [Concepts](./concepts.md) section.

## What Slang is not?

First and foremost, it is not a replacement for `solc`, the standard Solidity compiler. We do not plan at the moment to support emitting optimized EVM bytecode for use in production. Secondly, it does not perform formal verification of contracts or Solidity logic in general. However, other tools that serve this purpose are intended to be built on top of it.

## Supporting multiple versions

The Solidity programming language has evolved quite a bit since its inception. Some features were introduced, some changed, while some eventually became obsolete and were removed altogether.

While it's good for a programming language to evolve and better serve the needs of its users, not being able to easily upgrade or re-deploy existing contracts poses a unique challenge. Developer tooling must be able to understand and consume older contracts that are still being used on the blockchain, written in older versions of Solidity.

Because of that, Slang must be able to reason about different versions of Solidity; how the language grammar, name binding rules, and semantics have changed across different versions. One of our goals is to document differences as part of our [Solidity Specification](../solidity-specification/supported-versions.md).

This is why, instead of having to download separate versions of the tool for each Solidity version, you can access the Slang language APIs by simply specifying the Solidity version that you want to work with.

## Distributions

Slang itself is written in Rust and we maintain a public Rust package on [crates.io](https://crates.io/crates/slang_solidity). At the moment, we also distribute it as an [npm package](https://www.npmjs.com/package/@nomicfoundation/slang) with TypeScript interface. In the future, we plan on expanding the language bindings with Python and possibly more.
