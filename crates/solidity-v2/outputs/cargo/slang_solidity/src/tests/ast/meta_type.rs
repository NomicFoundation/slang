//! Exercises the public AST wrappers for meta-types (`Type::MetaType` and
//! `Type::UserMetaType`) and their accessors, reached through
//! `Expression::get_type()`.

use crate::abi::AbiType;
use crate::{ast, define_fixture};

define_fixture!(
    MetaTypes,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract C {
    struct S { uint a; }

    function f(uint x, bytes memory b) internal pure {
        S(x);
        abi.decode(b, (uint[]));
        uint(x);
        (uint)(x);
        uint[](x);
    }
}
"#,
);

/// Returns the function call expressions of the statements in `C.f`, in source
/// order.
fn function_calls() -> Vec<ast::FunctionCallExpression> {
    let unit = MetaTypes::build_compilation_unit();

    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("can find C contract");
    let function = contract
        .members()
        .iter()
        .find_map(|member| {
            if let ast::ContractMember::FunctionDefinition(function_definition) = member
                && function_definition
                    .name()
                    .is_some_and(|name| name.name() == "f")
            {
                return Some(function_definition);
            }
            None
        })
        .expect("f function is found");
    let body = function.body().expect("f has a body");

    body.statements()
        .iter()
        .map(|statement| {
            let ast::Statement::ExpressionStatement(statement) = statement else {
                panic!("expected an expression statement");
            };
            let ast::Expression::FunctionCallExpression(call) = statement.expression() else {
                panic!("expected a function call expression");
            };
            call
        })
        .collect()
}

#[test]
fn meta_type_ast_wrappers_are_reachable_via_get_type() {
    let calls = function_calls();
    assert_eq!(calls.len(), 5);

    // `S(x)`: a construction through a type name is a type conversion, and the
    // operand `S` carries a `Type::UserMetaType` that resolves back to the
    // struct definition. Meta-types have no ABI representation.
    let construction = &calls[0];
    assert!(construction.is_type_conversion());
    let operand_type = construction
        .operand()
        .get_type()
        .expect("`S` operand is typed");
    let ast::Type::UserMetaType(user_meta) = &operand_type else {
        panic!("expected the `S` operand to carry a `Type::UserMetaType`");
    };
    assert_eq!(user_meta.definition().identifier().name(), "S");
    assert!(AbiType::try_from(&operand_type).is_err());

    // `abi.decode(b, (uint[]))`: a plain call, not a type conversion. The
    // `uint[]` type expression carries a `Type::MetaType` wrapping the array
    // type it denotes.
    let decode = &calls[1];
    assert!(!decode.is_type_conversion());
    let ast::ArgumentsDeclaration::PositionalArguments(arguments) = decode.arguments() else {
        panic!("expected positional arguments");
    };
    let ast::Expression::TupleExpression(tuple) = arguments
        .iter()
        .nth(1)
        .expect("abi.decode has a second argument")
    else {
        panic!("expected the type argument to be a tuple expression");
    };
    let type_expression = tuple
        .items()
        .iter()
        .next()
        .and_then(|item| item.expression())
        .expect("the type tuple has an expression item");
    let type_expression_type = type_expression
        .get_type()
        .expect("`uint[]` type expression is typed");
    let ast::Type::MetaType(meta) = &type_expression_type else {
        panic!("expected the `uint[]` type expression to carry a `Type::MetaType`");
    };
    assert!(matches!(meta.meta_type(), ast::Type::Array(_)));
    assert!(AbiType::try_from(&type_expression_type).is_err());

    // `uint(x)`: an elementary cast; the `uint` operand carries a stored
    // `Type::MetaType` typing of its own, wrapping the integer type.
    let cast = &calls[2];
    assert!(cast.is_type_conversion());
    let operand_type = cast.operand().get_type().expect("`uint` operand is typed");
    let ast::Type::MetaType(meta) = &operand_type else {
        panic!("expected the `uint` operand to carry a `Type::MetaType`");
    };
    assert!(matches!(meta.meta_type(), ast::Type::Integer(_)));
    assert!(AbiType::try_from(&operand_type).is_err());

    // `(uint)(x)`: a parenthesized cast; the tuple operand's typing passes the
    // meta-type through, so it is still a type conversion.
    let parenthesized_cast = &calls[3];
    assert!(parenthesized_cast.is_type_conversion());

    // `uint[](x)`: an array-type cast; the operand `uint[]` is an index-access
    // expression carrying a `Type::MetaType` wrapping the array type, so it is
    // a type conversion rather than a function call.
    let array_cast = &calls[4];
    assert!(array_cast.is_type_conversion());
    let operand_type = array_cast
        .operand()
        .get_type()
        .expect("`uint[]` operand is typed");
    let ast::Type::MetaType(meta) = &operand_type else {
        panic!("expected the `uint[]` operand to carry a `Type::MetaType`");
    };
    assert!(matches!(meta.meta_type(), ast::Type::Array(_)));
    assert!(AbiType::try_from(&operand_type).is_err());
}
