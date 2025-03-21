use std::rc::Rc;

use anyhow::{anyhow, Result};
use slang_solidity::ast;
use slang_solidity::ast::mutator::Mutator;
use slang_solidity::cst::{TerminalKind, TerminalNode};
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

struct Cloner {}

impl Mutator for Cloner {}

#[test]
fn test_copy_source_unit_ast() -> Result<()> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION)?;
    let output = parser.parse_file_contents(
        r###"
// SPDX-License-Identifier: UNLICENSED
pragma solidity >= 0.8.17;

contract MyContract {
    address owner;
    constructor() {
         owner = msg.sender;
    }
    function test() view public returns (bool) {
         return owner == msg.sender;
    }
}
    "###,
    );
    assert!(output.is_valid());

    let source =
        ast::builder::build_source_unit(output.create_tree_cursor()).map_err(|s| anyhow!(s))?;

    let mut cloner = Cloner {};
    let ast = cloner.mutate_source_unit(&source);

    assert_eq!(2, ast.members.len());
    assert!(matches!(
        ast.members[0],
        ast::SourceUnitMember::PragmaDirective(_)
    ));
    assert!(matches!(
        ast.members[1],
        ast::SourceUnitMember::ContractDefinition(_)
    ));

    let ast::SourceUnitMember::ContractDefinition(ref contract) = ast.members[1] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("MyContract", contract.name.unparse());
    assert_eq!(3, contract.members.len());

    let ast::ContractMember::StateVariableDefinition(ref state_var) = contract.members[0] else {
        panic!("Expected StateVariableDefinition");
    };
    assert_eq!("owner", state_var.name.unparse());
    assert!(matches!(
        state_var.type_name,
        ast::TypeName::ElementaryType(_)
    ));

    let ast::ContractMember::ConstructorDefinition(ref ctor) = contract.members[1] else {
        panic!("Expected ConstructorDefinition");
    };
    assert_eq!(1, ctor.body.statements.len());

    let ast::ContractMember::FunctionDefinition(ref function) = contract.members[2] else {
        panic!("Expected FunctionDefinition");
    };
    let ast::FunctionName::Identifier(ref name) = function.name else {
        panic!("Expected identifier in FunctionName");
    };
    assert_eq!("test", name.unparse());
    assert_eq!(2, function.attributes.len());
    assert_eq!(0, function.parameters.parameters.len());
    assert!(function.returns.is_some());
    assert_eq!(
        1,
        function
            .returns
            .as_ref()
            .unwrap()
            .variables
            .parameters
            .len()
    );
    assert!(matches!(function.body, ast::FunctionBody::Block(_)));
    let ast::FunctionBody::Block(ref block) = function.body else {
        panic!("Expected Block");
    };
    assert_eq!(1, block.statements.len());

    Ok(())
}

// A constant folding mutator that can only fold multiplication of unit-less
// decimal numbers that use an underlying 64-bit floating point type
struct ConstantFolder {}

impl Mutator for ConstantFolder {
    fn mutate_expression(&mut self, source: &ast::Expression) -> ast::Expression {
        if let ast::Expression::MultiplicativeExpression(multiplicative_expression) = source {
            let left_operand = self.mutate_expression(&multiplicative_expression.left_operand);
            let right_operand = self.mutate_expression(&multiplicative_expression.right_operand);
            return if let (
                ast::Expression::DecimalNumberExpression(left_decimal),
                ast::Expression::DecimalNumberExpression(right_decimal),
            ) = (&left_operand, &right_operand)
            {
                // we don't support units in this test
                assert!(left_decimal.unit.is_none());
                assert!(right_decimal.unit.is_none());
                // also, any decimal number should be parseable as a 64-bit floating point
                let result = left_decimal.literal.unparse().parse::<f64>().unwrap()
                    * right_decimal.literal.unparse().parse::<f64>().unwrap();
                let number = Rc::new(ast::DecimalNumberExpressionStruct {
                    node_id: multiplicative_expression.node_id,
                    literal: Rc::new(TerminalNode {
                        kind: TerminalKind::DecimalLiteral,
                        text: format!("{result}"),
                    }),
                    unit: None,
                });
                ast::Expression::DecimalNumberExpression(number)
            } else {
                ast::Expression::MultiplicativeExpression(Rc::new(
                    ast::MultiplicativeExpressionStruct {
                        node_id: multiplicative_expression.node_id,
                        left_operand,
                        right_operand,
                    },
                ))
            };
        }
        self.default_mutate_expression(source)
    }
}

#[test]
fn test_constant_folding() -> Result<()> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION)?;
    let output = parser.parse_file_contents(
        r###"
function weeksToSeconds(uint _weeks) returns (uint) {
  uint secondsPerHour = 60 * 6 * 10;
  return 24 * 7 * secondsPerHour * _weeks;
}
    "###,
    );
    assert!(output.is_valid());

    let source =
        ast::builder::build_source_unit(output.create_tree_cursor()).map_err(|s| anyhow!(s))?;

    let mut constant_folder = ConstantFolder {};
    let ast = constant_folder.mutate_source_unit(&source);

    let ast::SourceUnitMember::FunctionDefinition(weeks_to_seconds) = &ast.members[0] else {
        panic!("Expected FunctionDefinition")
    };
    let ast::FunctionBody::Block(body) = &weeks_to_seconds.body else {
        panic!("Expected Block");
    };
    let ast::Statement::VariableDeclarationStatement(var_stmt) = &body.statements[0] else {
        panic!("Expected VariableDeclarationStatement");
    };
    let ast::Expression::DecimalNumberExpression(seconds_per_hour) = &var_stmt
        .value
        .as_ref()
        .expect("var declaration contains initialization value")
        .expression
    else {
        panic!("Expected DecimalNumberExpression");
    };
    assert_eq!(seconds_per_hour.literal.unparse(), "3600");

    let ast::Statement::ReturnStatement(return_stmt) = &body.statements[1] else {
        panic!("Expected ReturnStatement");
    };
    let ast::Expression::MultiplicativeExpression(outer_mult) = &return_stmt
        .expression
        .as_ref()
        .expect("should return an expression")
    else {
        panic!("Expected MultiplicativeExpression");
    };
    let ast::Expression::MultiplicativeExpression(inner_mult) = &outer_mult.left_operand else {
        panic!("Expected MultiplicativeExpression");
    };
    let ast::Expression::DecimalNumberExpression(days_per_week) = &inner_mult.left_operand else {
        panic!("Expected DecimalNumberExpression");
    };
    assert_eq!(days_per_week.literal.unparse(), "168");

    Ok(())
}
