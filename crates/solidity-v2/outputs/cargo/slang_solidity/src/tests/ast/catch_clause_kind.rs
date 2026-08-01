//! Exercises `CatchClause::kind`, which names a clause `Error`, `Panic` or
//! low-level. `Error` and `Panic` both bind to the same built-in, so the kind
//! is not recoverable from the binding.

use crate::ast::CatchClauseKind;
use crate::{ast, define_fixture};

define_fixture!(
    CatchClauses,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract Callee {
    function run() external pure returns (uint256) { return 1; }
}

contract Guarded {
    function all(Callee callee) public view {
        try callee.run() returns (uint256) {
        } catch Error(string memory) {
        } catch Panic(uint256) {
        } catch (bytes memory) {
        }
    }

    function unbound(Callee callee) public view {
        try callee.run() returns (uint256) {
        } catch {
        }
    }
}
"#,
);

#[derive(Default)]
struct CatchClauseCollector {
    kinds: Vec<Option<CatchClauseKind>>,
}

impl ast::visitor::Visitor for CatchClauseCollector {
    fn enter_catch_clause(&mut self, node: &ast::CatchClause) -> bool {
        self.kinds.push(node.kind());
        true
    }
}

#[test]
fn catch_clauses_report_their_kind() {
    let unit = CatchClauses::build_compilation_unit();
    let ast = unit.file(&"main.sol".into()).unwrap().ast();

    let mut collector = CatchClauseCollector::default();
    ast::visitor::accept_source_unit(&ast, &mut collector);

    assert_eq!(
        collector.kinds,
        vec![
            Some(CatchClauseKind::Error),
            Some(CatchClauseKind::Panic),
            Some(CatchClauseKind::LowLevel),
            Some(CatchClauseKind::LowLevel),
        ]
    );
}
