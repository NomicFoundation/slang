# slang_solidity_cli

<!-- _PRODUCT_README_ (keep in sync) -->

[![release](https://img.shields.io/github/v/tag/NomicFoundation/slang?label=GitHub%20Release&logo=github&sort=semver&logoColor=white)](https://github.com/NomicFoundation/slang/releases)
[![npm](https://img.shields.io/npm/v/@nomicfoundation/slang?label=NPM%20Package&logo=npm&logoColor=white)](https://www.npmjs.com/package/@nomicfoundation/slang)
[![cargo](https://img.shields.io/crates/v/slang_solidity?label=Rust%20Crate&logo=rust&logoColor=white)](https://crates.io/crates/slang_solidity)

## Solidity compiler tooling by [@NomicFoundation](https://github.com/NomicFoundation)

A modular set of compiler APIs empowering the next generation of Solidity code analysis and developer tooling.
Written in Rust and distributed in multiple languages.

Slang analyzes Solidity source code and generates a rich concrete syntax tree (CST) that can be reasoned about. This is a departure from the classic approach of "black-box" compilers, which are handed the input and only their output can be observed.

### Supporting Multiple Solidity Versions

The Solidity programming language has evolved quite a bit since its inception. Some features were introduced, some changed, while some eventually became obsolete and were removed altogether. Developer tooling must be able to understand and consume older contracts that are still being used on the blockchain, written in older versions of Solidity.

Because of that, Slang must be able to reason about different versions of Solidity; how the language grammar, name capture rules, and semantics have changed across different versions. One of our goals is to document differences as part of our [Solidity Grammar](https://nomicfoundation.github.io/slang/latest/solidity-grammar/).

This is why, instead of having to download separate versions of Slang for each Solidity version, you specify the Solidity version that you want to work with when you use the Slang language APIs.

- [See our supported versions.](https://nomicfoundation.github.io/slang/latest/solidity-grammar/supported-versions/)

## Installation

You can install the [Slang NPM package](https://www.npmjs.com/package/@nomicfoundation/slang) with the following `npm` command:

```sh
npm install "@nomicfoundation/slang"
```

Or if you are using `yarn`:

```sh
yarn add "@nomicfoundation/slang"
```

- [Learn more about how to get started with Slang.](https://nomicfoundation.github.io/slang/latest/user-guide/04-getting-started/01-installation/)

## Example

```ts
import assert from "node:assert";
import { ParseOutput, Parser } from "@nomicfoundation/slang/parser";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

function createTree(): ParseOutput {
    const source = `
    contract Foo {}
    contract Bar {}
    contract Baz {}
  `;

    const parser = Parser.create(LanguageFacts.latestVersion());

    const parseOutput = parser.parseFileContents(source.trim());
    assert(parseOutput.isValid());

    return parseOutput;
}

test("Get contract names", () => {
    const tree = createTree();
    const cursor = tree.createTreeCursor();

    const contracts = [];

    while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
        assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));
        contracts.push(cursor.node.unparse());
    }

    assert.deepStrictEqual(contracts, ["Foo", "Bar", "Baz"]);
});
```

Slang is not a replacement for solc, the standard Solidity compiler. We do not plan at the moment to support emitting optimized EVM bytecode for use in production. It does not perform formal verification of contracts or Solidity logic in general. However, it is designed to empower such tools to be built on top of it.

- [Slang v1 Announcement - Blog Post](https://blog.nomic.foundation/slang-v1-a-reliable-way-to-analyze-solidity-code/)
- [Slang User Guide](https://nomicfoundation.github.io/slang/latest/user-guide/01-introduction/)

## Using Slang

We have [several examples](https://nomicfoundation.github.io/slang/latest/user-guide/08-examples/) showing some of the ways that you can use Slang APIs to develop your own Solidity tooling. For more detailed information about Slang concepts please check [our user guide](https://nomicfoundation.github.io/slang/latest/user-guide/).

## Core Concepts

### Concrete Syntax Trees

Slang is capable of parsing the source code into a Concrete Syntax Tree (CST), which is a tree structure representing the entire source code. It includes the contracts, functions, statements, and expressions within. It also includes things like comments, whitespace, and punctuation. This is sometimes called a "full-fidelity" CST, and it can be used to reconstruct the original source code when needed.

### Parser

The `Parser` API is used to produce a CST from Solidity source code. Each `Parser` object is initialized with a specific Solidity version.

With a `Parser` object, you can analyze any source text according to the grammar of that specific version. Therefore, providing an accurate language version is important as it affects the shape of the syntax tree and the set of possible errors.

- [Learn more about the Parser API.](https://nomicfoundation.github.io/slang/latest/user-guide/04-getting-started/01-installation/)

### Cursors

For many code analysis tasks it is useful to traverse the CST and visit each node. Slang provides this functionality through cursors. Cursors provide an efficient pre-order traversal API for both complete CSTs and arbitrary subtrees.

- [Learn more about using cursors.](https://nomicfoundation.github.io/slang/latest/user-guide/05-syntax-trees/03-navigating-with-cursors/)

### Queries

The `Cursor` API is a low-level API that allows you to traverse the CST in a procedural manner. However, it is often more convenient to use the declarative `Query` API. Queries allow you to express your intent more concisely, and also allows you to reuse the same query in multiple places. Queries can largely replace the need for both internal (cursor), and external (visitor) iterator patterns.

- [Learn more about using queries.](https://nomicfoundation.github.io/slang/latest/user-guide/06-query-language/01-query-syntax/)

### Compilation Units

Solidity projects are usually composed of multiple files. Slang has the concept of a `CompilationUnit`, which is built from parsing Solidity source files and their dependencies, transitively. This allows performing further analysis on the source code, such as semantic analysis.

- [Learn more about using compilation units.](https://nomicfoundation.github.io/slang/latest/user-guide/07-semantic-analysis/01-compilation-units/)

### Binding Graph

The binding graph is a structure that represents the relationships between identifiers across source files in a `CompilationUnit`. It stores `Cursor` objects to all Solidity definitions (contracts, functions, state variables, etc.), the references to them, and can resolve the links between the two.

- [Learn more about using the binding graph.](https://nomicfoundation.github.io/slang/latest/user-guide/07-semantic-analysis/02-binding-graph/)

## Contributing

Please check our [contributors guide](https://github.com/NomicFoundation/slang/blob/main/CONTRIBUTING.md) to learn about how you can get started on Slang development.

## Built with Slang

We're proud to support other projects in the Solidity ecosystem with our developer tools. Here are some of the projects currently using Slang:

- [Hardhat Solidity VSCode Extension](https://github.com/NomicFoundation/hardhat-vscode)
- [prettier-plugin-solidity](https://github.com/prettier-solidity/prettier-plugin-solidity/tree/v2)
- [openzeppelin-upgrades](https://github.com/OpenZeppelin/openzeppelin-upgrades)

- [Learn more about projects using Slang.](https://nomicfoundation.github.io/slang/latest/user-guide/02-powered-by-slang/)

---

- [Slang NPM Package](https://www.npmjs.com/package/@nomicfoundation/slang/)
- [Slang User Guide](https://nomicfoundation.github.io/slang/latest/user-guide/)
- [Slang Telegram Group](https://t.me/+pxApdT-Ssn5hMTFh)
- [We are hiring!](https://nomic.foundation/jobs)
