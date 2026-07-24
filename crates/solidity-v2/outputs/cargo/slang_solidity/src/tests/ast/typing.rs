use super::fixtures;
use crate::{ast, define_fixture};

#[test]
fn test_get_type() {
    let unit = fixtures::Counter::build_compilation_unit();

    let ownable = unit
        .find_contract_by_name("Ownable")
        .next()
        .expect("contract is found");

    let state_variables = ownable
        .members()
        .iter()
        .filter_map(|member| {
            if let ast::ContractMember::StateVariableDefinition(definition) = member {
                Some(definition)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(state_variables.len(), 1);
    let owner = &state_variables[0];
    assert_eq!(owner.name().name(), "_owner");

    let owner_type = owner
        .get_type()
        .expect("_owner state variable has resolved type");
    assert!(matches!(owner_type, ast::Type::Address(_)));
}

#[test]
fn test_function_get_type() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("contract is found");

    let increment = counter
        .members()
        .iter()
        .find_map(|member| {
            if let ast::ContractMember::FunctionDefinition(function_definition) = member {
                if function_definition
                    .name()
                    .is_some_and(|name| name.name() == "increment")
                {
                    Some(function_definition)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("increment method is found");

    let increment_type = increment.get_type().expect("increment method has a type");
    let ast::Type::Function(function_type) = increment_type else {
        panic!("method's type is expected to be a function");
    };
    assert!(function_type.is_externally_visible());
    assert!(matches!(function_type.return_type(), ast::Type::Integer(_)));
    assert!(
        function_type
            .associated_definition()
            .is_some_and(|definition| matches!(definition, ast::Definition::Function(_)))
    );
}

define_fixture!(
    ConditionalTernary,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract C {
    function pair() internal pure returns (uint256, uint256) {
        return (1, 2);
    }

    function f(bool c) internal pure returns (uint256, uint256) {
        (uint256 a, uint256 b) = c ? pair() : (3, uint256(4));
        return (a, b);
    }
}
"#,
);

#[test]
fn test_conditional_expression_get_type() {
    let unit = ConditionalTernary::build_compilation_unit();

    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract C is found");

    let function = contract
        .members()
        .iter()
        .find_map(|member| match member {
            ast::ContractMember::FunctionDefinition(function)
                if function.name().is_some_and(|name| name.name() == "f") =>
            {
                Some(function)
            }
            _ => None,
        })
        .expect("function f is found");

    // `f`'s first statement is `(uint256 a, uint256 b) = c ? pair() : (3, uint256(4));`.
    let declaration = function
        .body()
        .expect("f has a body")
        .statements()
        .iter()
        .find_map(|statement| match statement {
            ast::Statement::VariableDeclarationStatement(declaration) => Some(declaration),
            _ => None,
        })
        .expect("f has a variable declaration statement");

    // Its right-hand side (the multi-variable declaration's value) is the ternary.
    let ast::VariableDeclarationTarget::MultiTypedDeclaration(target) = declaration.target() else {
        panic!("expected a multi-variable declaration target");
    };
    let ast::Expression::ConditionalExpression(conditional) = target.value() else {
        panic!("expected the initializer to be a conditional expression");
    };

    // The ternary `c ? pair() : (3, uint256(4))` unifies the function-call
    // tuple `(uint256, uint256)` with the literal tuple, widening `3`, so the
    // whole conditional types as `(uint256, uint256)`.
    let ast::Type::Tuple(tuple) = conditional.get_type().expect("ternary has a resolved type")
    else {
        panic!("expected the ternary to type as a tuple");
    };

    let elements = tuple.types();
    assert_eq!(elements.len(), 2, "tuple has two elements");
    for element in elements {
        let ast::Type::Integer(integer) = element else {
            panic!("expected each tuple element to be an integer");
        };
        assert!(!integer.is_signed(), "element is unsigned");
        assert_eq!(integer.bits(), 256, "element is 256-bit");
    }
}
