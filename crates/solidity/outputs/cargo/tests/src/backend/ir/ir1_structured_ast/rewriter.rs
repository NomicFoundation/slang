use std::rc::Rc;

use anyhow::Result;
use slang_solidity::backend::ir::ir1_structured_ast;
use slang_solidity::backend::ir::ir1_structured_ast::rewriter::Rewriter;
use slang_solidity::cst::{TerminalKind, TerminalNode};
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

struct Cloner {}

impl Rewriter for Cloner {}

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

    let source = ir1_structured_ast::builder::build_source_unit(output.tree()).unwrap();

    let mut cloner = Cloner {};
    let ast = cloner.rewrite_source_unit(&source);

    assert_eq!(2, ast.members.len());
    assert!(matches!(
        ast.members[0],
        ir1_structured_ast::SourceUnitMember::PragmaDirective(_)
    ));
    assert!(matches!(
        ast.members[1],
        ir1_structured_ast::SourceUnitMember::ContractDefinition(_)
    ));

    let ir1_structured_ast::SourceUnitMember::ContractDefinition(ref contract) = ast.members[1]
    else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("MyContract", contract.name.unparse());
    assert_eq!(3, contract.members.len());

    let ir1_structured_ast::ContractMember::StateVariableDefinition(ref state_var) =
        contract.members[0]
    else {
        panic!("Expected StateVariableDefinition");
    };
    assert_eq!("owner", state_var.name.unparse());
    assert!(matches!(
        state_var.type_name,
        ir1_structured_ast::TypeName::ElementaryType(_)
    ));

    let ir1_structured_ast::ContractMember::ConstructorDefinition(ref ctor) = contract.members[1]
    else {
        panic!("Expected ConstructorDefinition");
    };
    assert_eq!(1, ctor.body.statements.len());

    let ir1_structured_ast::ContractMember::FunctionDefinition(ref function) = contract.members[2]
    else {
        panic!("Expected FunctionDefinition");
    };
    let ir1_structured_ast::FunctionName::Identifier(ref name) = function.name else {
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
    assert!(matches!(
        function.body,
        ir1_structured_ast::FunctionBody::Block(_)
    ));
    let ir1_structured_ast::FunctionBody::Block(ref block) = function.body else {
        panic!("Expected Block");
    };
    assert_eq!(1, block.statements.len());

    Ok(())
}

// A constant folding rewriter that can only fold multiplication of unit-less
// decimal numbers that use an underlying 64-bit floating point type
struct ConstantFolder {}

impl Rewriter for ConstantFolder {
    fn rewrite_expression(
        &mut self,
        source: &ir1_structured_ast::Expression,
    ) -> ir1_structured_ast::Expression {
        if let ir1_structured_ast::Expression::MultiplicativeExpression(multiplicative_expression) =
            source
        {
            let left_operand = self.rewrite_expression(&multiplicative_expression.left_operand);
            let right_operand = self.rewrite_expression(&multiplicative_expression.right_operand);
            return if let (
                ir1_structured_ast::Expression::DecimalNumberExpression(left_decimal),
                ir1_structured_ast::Expression::DecimalNumberExpression(right_decimal),
            ) = (&left_operand, &right_operand)
            {
                // we don't support units in this test
                assert!(left_decimal.unit.is_none());
                assert!(right_decimal.unit.is_none());
                // also, any decimal number should be parseable as a 64-bit floating point
                let result = left_decimal.literal.unparse().parse::<f64>().unwrap()
                    * right_decimal.literal.unparse().parse::<f64>().unwrap();
                let number = Rc::new(ir1_structured_ast::DecimalNumberExpressionStruct {
                    node_id: multiplicative_expression.node_id,
                    literal: Rc::new(TerminalNode {
                        kind: TerminalKind::DecimalLiteral,
                        text: format!("{result}"),
                    }),
                    unit: None,
                });
                ir1_structured_ast::Expression::DecimalNumberExpression(number)
            } else {
                ir1_structured_ast::Expression::MultiplicativeExpression(Rc::new(
                    ir1_structured_ast::MultiplicativeExpressionStruct {
                        node_id: multiplicative_expression.node_id,
                        operator: Rc::clone(&multiplicative_expression.operator),
                        left_operand,
                        right_operand,
                    },
                ))
            };
        }
        self.default_rewrite_expression(source)
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

    let source = ir1_structured_ast::builder::build_source_unit(output.tree()).unwrap();

    let mut constant_folder = ConstantFolder {};
    let ast = constant_folder.rewrite_source_unit(&source);

    let ir1_structured_ast::SourceUnitMember::FunctionDefinition(weeks_to_seconds) =
        &ast.members[0]
    else {
        panic!("Expected FunctionDefinition")
    };
    let ir1_structured_ast::FunctionBody::Block(body) = &weeks_to_seconds.body else {
        panic!("Expected Block");
    };
    let ir1_structured_ast::Statement::VariableDeclarationStatement(var_stmt) = &body.statements[0]
    else {
        panic!("Expected VariableDeclarationStatement");
    };
    let ir1_structured_ast::Expression::DecimalNumberExpression(seconds_per_hour) = &var_stmt
        .value
        .as_ref()
        .expect("var declaration contains initialization value")
        .expression
    else {
        panic!("Expected DecimalNumberExpression");
    };
    assert_eq!(seconds_per_hour.literal.unparse(), "3600");

    let ir1_structured_ast::Statement::ReturnStatement(return_stmt) = &body.statements[1] else {
        panic!("Expected ReturnStatement");
    };
    let ir1_structured_ast::Expression::MultiplicativeExpression(outer_mult) = &return_stmt
        .expression
        .as_ref()
        .expect("should return an expression")
    else {
        panic!("Expected MultiplicativeExpression");
    };
    let ir1_structured_ast::Expression::MultiplicativeExpression(inner_mult) =
        &outer_mult.left_operand
    else {
        panic!("Expected MultiplicativeExpression");
    };
    let ir1_structured_ast::Expression::DecimalNumberExpression(days_per_week) =
        &inner_mult.left_operand
    else {
        panic!("Expected DecimalNumberExpression");
    };
    assert_eq!(days_per_week.literal.unparse(), "168");

    Ok(())
}
