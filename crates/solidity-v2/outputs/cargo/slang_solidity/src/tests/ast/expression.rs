//! Exercises `Expression::unwrap_singleton_tuple` on parenthesized and other
//! expressions.

use crate::ast::NodeLocation;
use crate::{ast, define_fixture};

const SOURCE: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract C {
    function f(uint256 x, uint256 y) public pure {
        x;
        (x);
        (((x)));
        ((x, y));
        (((x), y));
        (x) + y;
        ();
        (x, y);
    }
}
"#;

define_fixture!(SingletonTuples, file: "main.sol", SOURCE);

#[derive(Default)]
struct StatementExpressions {
    expressions: Vec<ast::Expression>,
}

impl ast::visitor::Visitor for StatementExpressions {
    fn enter_expression_statement(&mut self, node: &ast::ExpressionStatement) -> bool {
        self.expressions.push(node.expression());
        true
    }
}

#[test]
fn test_unwrap_singleton_tuple() {
    let unit = SingletonTuples::build_compilation_unit();
    let main_ast = unit.file(&"main.sol".into()).unwrap().ast();

    let mut visitor = StatementExpressions::default();
    ast::visitor::accept_source_unit(&main_ast, &mut visitor);

    // Only the outer parentheses are stripped: inner tuples, operands, and
    // empty or multi-element tuples are left as is.
    let unwrapped: Vec<_> = visitor
        .expressions
        .iter()
        .map(|expression| {
            &SOURCE[expression
                .unwrap_singleton_tuple()
                .calculate_text_range()
                .unwrap()]
        })
        .collect();
    assert_eq!(
        unwrapped,
        [
            "x", "x", "x", "(x, y)", "((x), y)", "(x) + y", "()", "(x, y)"
        ],
    );

    // The unwrapped expression is the original node, so it keeps its binding.
    let ast::Expression::Identifier(x) = visitor.expressions[2].unwrap_singleton_tuple() else {
        panic!("`(((x)))` unwraps to an identifier");
    };
    assert!(x.resolve_to_definition().is_some());
}
