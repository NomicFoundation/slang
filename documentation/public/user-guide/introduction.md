## What is Slang?

Slang is intended to be a modular Solidity compiler, specifically targeting code analysis and developer tooling. This means servicing tools with domain-specific APIs and, in general, facilitating working with and analyzing the Solidity source code. If you're in the editor writing Solidity or performing linting or additional validation, there's a chance that you are, or could be, running Slang!

## What Slang is not?

First and foremost, it is not a replacement to `solc`, the standard Solidity compiler. We do not plan at the moment to support emitting EVM bytecode. Secondly, it is not meant to be used for formal verification of contracts or Solidity logic in general.

## Supporting multiple versions

Over the years, the Solidity programming language has evolved quite a bit since its inception. Some features were introduced, some changed, while some eventually became obsolete and were removed altogether.

While it's good for a programming language to evolve and better serve the needs of its users, not being able to easily upgrade or re-deploy existing contracts poses a unique challenge. Older contracts that are still being used on the blockchain, written in older versions of Solidity, must remain understandable during the development process.

Because of that, Slang must be able to reason about different versions of Solidity - how the language grammar, name binding rules and semantics changed over multiple versions. One of our goals is to document differences as part of our [Solidity Specification](../solidity-specification/index.md).
