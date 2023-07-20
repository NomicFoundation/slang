# Cargo Crate

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

This is an internal iterator. The `Cursor` can drive an external iterator i.e. the `Visitor` trait, which is described below.

The below example uses a cursor to collect the names of all contracts in a source file, and returns them as a `Vec<String>`:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.rs"
```

## Visitor API

The `Visitor` trait allows callers to implement a visitor that will be called for each node in the tree.
The `VisitorEntryResponse` enum allows callers to control the traversal behavior.

For example, if the visitor is only interested in the top-level nodes, it can return `VisitorEntryResponse::StepOver` to skip the children of the current node.
If the visitor is interested in the children of the current node, it can return `VisitorEntryResponse::StepIn` to visit them.

The below example defines a visitor that collects the names of all contracts in a source file, and returns them as a `Vec<String>`:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/visitor_api.rs"
```
