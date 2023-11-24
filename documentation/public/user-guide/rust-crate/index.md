# Rust Crate

The Rust package is published to crates.io as [`slang_solidity`](https://crates.io/crates/slang_solidity).

It can be used both as a regular Rust dependency and as a standalone CLI (installable with Cargo).

## Installation

You can install `slang` CLI as a cargo binary using:

```bash
cargo install "slang_solidity"
```

Or you can add its API as a dependency to your project:

```bash
cargo add "slang_solidity"
```

## Using the CLI

The `parse` command will take a path to a Solidity file, and a `--version` flag.
Specifying the correct version is important, as it will affect the grammar used to parse inputs.

By default, it will print syntax errors (if any), and exit with code equal to the number of errors found.
You can also print the CST serialized as JSON by passing the `--json` flag:

```bash
slang_solidity parse "path/to/input.sol" --version "0.8.0" [--json]
```

## Parser API

Similar to the CLI, the API also requires passing a language version to create a `Language` object.
You can then use it to parse different inputs belonging to that version.
Each `parse()` operation accepts the input source code, and a `ProductionKind` variant.
This allows callers to parse entire source files (`ProductionKind::SourceUnit`), individual contracts (`ProductionKind::ContractDefinition`),
methods (`ProductionKind::FunctionDefinition`), or any other syntax nodes.

The resulting `ParseOutput` object will contain syntax errors (if any), and the parse tree corresponding to the input source code.
You can then iterate over the resulting children, and assert that they match the expected syntax nodes:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/simple_contract.rs"
```

## Cursor API

For many code analysis tasks, it is useful to traverse the parse tree and visit each node.
The `Cursor` object allows callers to traverse the parse tree in a pre-order depth-first manner.

The below example uses a cursor to collect the names of all contracts in a source file, and returns them as a `Vec<String>`:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.rs"
```
