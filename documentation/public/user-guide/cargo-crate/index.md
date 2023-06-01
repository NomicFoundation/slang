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

```rust
use slang_solidity::syntax::{
    language::Language,
    nodes::{Node, RuleKind, TokenKind},
    parser::{ProductionKind},
};

let language = Language::new(Version::parse("0.8.0")?)?;
let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}")?;
```

The resulting `ParseOutput` object will contain syntax errors (if any), and the parse tree corresponding to the input source code.
You can then iterate over the resulting children, and assert that they match the expected syntax nodes:

```rust
let parse_tree = parse_output.parse_tree().unwrap();

let children = match parse_tree.as_ref() {
    Node::Rule { kind, children, .. } => {
        assert_eq!(*kind, RuleKind::ContractDefinition);
        children
    }
    _ => {
      panic!("Unexpected parse_tree");
    }
};

assert_eq!(children.len(), 4);

assert!(matches!(children[0].as_ref(), Node::Token { kind, .. } if *kind == TokenKind::ContractKeyword));
assert!(matches!(children[1].as_ref(), Node::Token { kind, .. } if *kind == TokenKind::Identifier));
assert!(matches!(children[2].as_ref(), Node::Token { kind, .. } if *kind == TokenKind::OpenBrace));
assert!(matches!(children[3].as_ref(), Node::Token { kind, .. } if *kind == TokenKind::CloseBrace));
```

## Visitor API

For many code analysis tasks, it is useful to traverse the parse tree and visit each node.
The `Visitor` trait allows callers to implement a visitor that will be called for each node in the tree.
The `VisitorEntryResponse` enum allows callers to control the traversal behavior.

For example, if the visitor is only interested in the top-level nodes, it can return `VisitorEntryResponse::StepOver` to skip the children of the current node.
If the visitor is interested in the children of the current node, it can return `VisitorEntryResponse::StepIn` to visit them.

The below example defines a visitor that collects the names of all contracts in a source file, and returns them as a `Vec<String>`:

```rust
use anyhow::{bail, Error, Result};
use slang_solidity::syntax::{
    nodes::{Node, RuleKind, TokenKind},
    visitors::{Visitable, Visitor, VisitorEntryResponse},
};
use std::{ops::Range, rc::Rc};

struct ContractCollector {
    source: String,
    contract_names: Vec<String>,
}

impl ContractCollector {
    pub fn collect(source: String, parse_tree: &Rc<Node>) -> Result<Vec<String>> {
        let mut collector = ContractCollector {
            source,
            contract_names: Vec::new(),
        };

        parse_tree.accept_visitor(&mut collector)?;

        return Ok(collector.contract_names);
    }
}

impl Visitor<Error> for ContractCollector {
    fn enter_rule(
        &mut self,
        kind: RuleKind,
        _range: &Range<usize>,
        children: &Vec<Rc<Node>>,
        _node: &Rc<Node>,
        _path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse> {
        if kind != RuleKind::ContractDefinition {
            return Ok(VisitorEntryResponse::StepIn);
        }

        let identifier_range = if let Node::Token { kind, range, .. } = children[1].as_ref() {
            assert_eq!(*kind, TokenKind::Identifier);
            range
        } else {
            bail!("Expected contract identifier");
        };

        let identifier_bytes = self
            .source
            .bytes()
            .skip(identifier_range.start)
            .take(identifier_range.end - identifier_range.start)
            .collect();

        let identifier_text = String::from_utf8(identifier_bytes)?;
        self.contract_names.push(identifier_text);

        return Ok(VisitorEntryResponse::StepOver);
    }
}
```
