# Cargo Crate

## Installation

```bash
cargo add "slang_solidity"
```

## CLI

```bash
cargo install "slang_solidity"

slang_solidity parse "path/to/input.sol" --version "0.8.0" --json
```

## Parser API

```rust
use slang_solidity::syntax::{
    nodes::{Node, RuleKind, TokenKind},
    parser::{Language, ProductionKind},
};

let language = Language::new(Version::parse("0.8.0")?)?;
let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}");

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
