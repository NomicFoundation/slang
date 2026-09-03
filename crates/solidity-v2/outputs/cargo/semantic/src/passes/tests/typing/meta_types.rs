//! Type names in expression position: the meta-types they carry, the casts
//! and array types built from them, and the members reached through them.

use ruint::aliases::U256;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_ir::ir::{self, NodeIdentity};

use super::support::find_function;
use super::{Analyse, Analysis, expression, expressions};
use crate::binder::Typing;
use crate::types::{
    ArrayType, ByteArrayType, DataLocation, FixedSizeArrayType, IntegerType, MetaType, StructType,
    TupleType, Type,
};

#[test]
fn test_index_access_on_elementary_meta_type_yields_array_meta_type() {
    // Control: indexing the meta-type of an elementary type (`uint[]`) yields
    // the meta-type of an array of that elementary type.
    let (meta, types) = expression("uint[]").into_resolved_type();

    let Type::MetaType(MetaType { type_id: array_id }) = meta else {
        panic!("expected the `uint[]` expression to be a MetaType, got {meta:?}");
    };
    let Type::Array(ArrayType {
        element_type,
        location,
    }) = types.get_type_by_id(array_id).clone()
    else {
        panic!(
            "expected the meta-type to wrap an Array, got {:?}",
            types.get_type_by_id(array_id)
        );
    };
    assert_eq!(location, DataLocation::Memory);
    assert_eq!(element_type, types.uint256());
}

#[test]
fn test_index_access_on_elementary_meta_type_with_literal_index_yields_fixed_size_array() {
    // A number literal index is the array's length, whether written directly,
    // folded from literal arithmetic, or written in hex.
    let inputs = ["uint[3]", "uint[1 + 2]", "uint[0x3]"];
    let (typings, types) = expressions(&inputs).into_types();

    for (input, typing) in inputs.iter().zip(&typings) {
        let Some(Type::MetaType(MetaType { type_id: array_id })) = typing else {
            panic!("expected `{input}` to be a MetaType, got {typing:?}");
        };
        let Type::FixedSizeArray(FixedSizeArrayType {
            element_type,
            size,
            location,
        }) = types.get_type_by_id(*array_id).clone()
        else {
            panic!(
                "expected `{input}` to wrap a FixedSizeArray, got {:?}",
                types.get_type_by_id(*array_id)
            );
        };
        assert_eq!(size, U256::from(3), "`{input}` has length 3");
        assert_eq!(element_type, types.uint256());
        assert_eq!(location, DataLocation::Memory);
    }
}

#[test]
fn test_index_access_on_elementary_meta_type_with_non_literal_index_is_unresolved() {
    // Only a literal is a length in expression position, so neither a constant
    // nor a cast is one (matches solc error 3940).
    for input in ["uint[N]", "uint[uint8(3)]"] {
        let (typing, _) = expression(input)
            .with_members("uint constant N = 3;")
            .into_type();
        assert_eq!(typing, None, "`{input}` is not a valid array length");
    }
}

#[test]
fn test_index_access_on_user_meta_type_yields_array_meta_type() {
    // `MyStruct[]` is a *type expression*: indexing the user meta-type of a
    // struct produces the meta-type of an array whose element is that struct.
    let (meta, types) = expression("MyStruct[]")
        .with_members("struct MyStruct { uint a; }")
        .into_resolved_type();

    let Type::MetaType(MetaType { type_id: array_id }) = meta else {
        panic!("expected the `MyStruct[]` expression to be a MetaType, got {meta:?}");
    };
    let Type::Array(ArrayType {
        element_type,
        location,
    }) = types.get_type_by_id(array_id).clone()
    else {
        panic!(
            "expected the meta-type to wrap an Array, got {:?}",
            types.get_type_by_id(array_id)
        );
    };
    assert_eq!(location, DataLocation::Memory);

    // The array element is the struct's own value type.
    assert!(
        matches!(
            types.get_type_by_id(element_type),
            Type::Struct(StructType { .. })
        ),
        "expected the array element to be the struct type, got {:?}",
        types.get_type_by_id(element_type),
    );
}

#[test]
fn test_index_access_on_user_meta_type_with_literal_index_yields_fixed_size_array() {
    let (meta, types) = expression("MyStruct[3]")
        .with_members("struct MyStruct { uint a; }")
        .into_resolved_type();

    let Type::MetaType(MetaType { type_id: array_id }) = meta else {
        panic!("expected the `MyStruct[3]` expression to be a MetaType, got {meta:?}");
    };
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type,
        size,
        location,
    }) = types.get_type_by_id(array_id).clone()
    else {
        panic!(
            "expected the meta-type to wrap a FixedSizeArray, got {:?}",
            types.get_type_by_id(array_id)
        );
    };
    assert_eq!(size, U256::from(3));
    assert_eq!(location, DataLocation::Memory);
    assert!(
        matches!(
            types.get_type_by_id(element_type),
            Type::Struct(StructType { .. })
        ),
        "expected the array element to be the struct type, got {:?}",
        types.get_type_by_id(element_type),
    );
}

#[test]
fn test_user_meta_type_built_in_members() {
    // Built-in members of a *type name* resolve through its meta-type: errors
    // expose `selector`, and UDVTs expose `wrap`/`unwrap`.
    let (type_, _) = expression("Err.selector")
        .with_members("error Err(uint x);")
        .into_resolved_type();
    assert_eq!(type_, Type::ByteArray(ByteArrayType { width: 4 }));

    let (type_, _) = expression("T.wrap(1)")
        .with_members("type T is uint256;")
        .into_resolved_type();
    assert!(
        matches!(type_, Type::UserDefinedValue(_)),
        "expected `T.wrap(1)` to type as the UDVT, got {type_:?}",
    );

    let (type_, _) = expression("T.unwrap(T.wrap(1))")
        .with_members("type T is uint256;")
        .into_resolved_type();
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: false,
            bits: 256
        })
    );
}

#[test]
fn test_explicit_enum_cast() {
    // Explicit conversion from an integer to an enum is valid Solidity and
    // types as the enum.
    let (type_, _) = expression("E(1)")
        .with_members("enum E { A, B }")
        .into_resolved_type();
    assert!(
        matches!(type_, Type::Enum(_)),
        "expected `E(1)` to type as the enum, got {type_:?}",
    );

    // User defined value types are not castable by name: conversion goes
    // through `wrap`/`unwrap`.
    let (type_, _) = expression("T(1)")
        .with_members("type T is uint256;")
        .into_type();
    assert_eq!(type_, None);
}

#[test]
fn test_meta_type_internal_names() {
    // Meta-types print in solc's `type(T)` notation: `type(uint256)` for an
    // elementary type, `type(C.E)` for a named definition.
    let source = r#"
        pragma solidity *;
        contract C {
            enum E { A }
            function g() internal pure {
                uint(1);
                E;
            }
        }
    "#;
    let context = Analysis::of_source(source)
        .target(EvmTarget::LATEST)
        .run(Analyse::Context)
        .expect_no_diagnostics()
        .into_context();

    let contract = context
        .find_contract_by_name("C")
        .next()
        .expect("contract `C` not found");
    let function = find_function(&contract.members, "g").expect("g function");
    let body = function.body.as_ref().expect("g has a body");
    let mut nodes = body.statements.iter().filter_map(|stmt| match stmt {
        ir::Statement::ExpressionStatement(s) => Some(&s.expression),
        _ => None,
    });

    // `uint(1)`: the call operand `uint` carries the elementary meta-type.
    let cast = nodes.next().expect("cast statement");
    let ir::Expression::FunctionCallExpression(call) = cast else {
        panic!("expected a function call expression");
    };
    let operand_node_id = call.operand.node_id().expect("operand has a node id");
    let uint_meta_id = context
        .binder()
        .node_typing(operand_node_id)
        .as_type_id()
        .expect("cast operand is typed");
    assert_eq!(context.type_internal_name(uint_meta_id), "type(uint256)");

    // `E`: the bare enum name carries the user meta-type.
    let enum_expression = nodes.next().expect("enum statement");
    let enum_node_id = enum_expression.node_id().expect("expression has a node id");
    let enum_meta_id = context
        .binder()
        .node_typing(enum_node_id)
        .as_type_id()
        .expect("enum name is typed");
    assert_eq!(context.type_internal_name(enum_meta_id), "type(C.E)");
}

#[test]
fn test_abi_decode_tuple_of_types() {
    // Multi-element `abi.decode` types as the tuple of the *decoded* value
    // types, unwrapping each element's meta-type.
    let (decoded, types) = expression("abi.decode(b, (uint, S))")
        .with_members("bytes b; struct S { uint a; }")
        .into_resolved_type();
    let Type::Tuple(TupleType { types: element_ids }) = decoded else {
        panic!("expected a tuple type, got {decoded:?}");
    };
    assert_eq!(element_ids.len(), 2);
    assert_eq!(element_ids[0], types.uint256());
    assert!(
        matches!(types.get_type_by_id(element_ids[1]), Type::Struct(_)),
        "expected the second element to decode to the struct",
    );

    // A tuple element that is not a type name doesn't decode.
    let (decoded, _) = expression("abi.decode(b, (uint, 5))")
        .with_members("bytes b;")
        .into_type();
    assert_eq!(decoded, None);

    // A nested tuple element is not a type name, so it doesn't decode either
    // (matching solc, which rejects `abi.decode(b, (uint, (bool, bool)))`).
    let (decoded, _) = expression("abi.decode(b, (uint, (bool, bool)))")
        .with_members("bytes b;")
        .into_type();
    assert_eq!(decoded, None);

    // Neither does a second argument that is not a type or tuple of types.
    let (decoded, _) = expression("abi.decode(b, x)")
        .with_members("bytes b; uint x;")
        .into_type();
    assert_eq!(decoded, None);
}

#[test]
fn test_tuple_of_type_names_is_a_tuple_of_meta_types() {
    // A tuple of type names is a *tuple of meta-types* (not a meta-type
    // itself): `(uint, bool)` types as `Tuple(type(uint256), type(bool))`.
    // This matches solc, which rejects a nested tuple element (it is not a
    // type name) — see `test_abi_decode_tuple_of_types`.
    let (type_, registry) = expression("(uint, bool)").into_resolved_type();
    let Type::Tuple(TupleType { types: element_ids }) = type_ else {
        panic!("expected `(uint, bool)` to type as a tuple, got {type_:?}");
    };
    assert!(matches!(
        registry.get_type_by_id(element_ids[0]),
        Type::MetaType(_)
    ));
    assert!(matches!(
        registry.get_type_by_id(element_ids[1]),
        Type::MetaType(_)
    ));
}

#[test]
fn test_function_declaration_via_type_name_has_no_mobile_type() {
    // A function reached through a contract/interface *type name* (`C.g`) is a
    // non-value declaration with no mobile type — only good for `.selector` —
    // not a function value. This mirrors solc's `FunctionType::Kind::Declaration`.
    // `external` functions are always declarations via the type name; `public`
    // ones only from a non-deriving ("foreign") scope; `internal` and local
    // `public` stay callable function values.
    let source = r#"
        pragma solidity *;
        interface I {
            function h() external;
        }
        contract C {
            function g() external {}
            function pub() public {}
            function intl() internal pure {}
        }
        contract D is C {
            function tD() internal {
                C.g;
                C.g.selector;
                C.pub;
                C.intl;
                I.h;
            }
        }
        contract E {
            function tE() internal {
                C.pub;
            }
        }
    "#;

    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let binder = analysis.binder();
    let types = analysis.types();

    let statement_typings = |contract: &str, function: &str| -> Vec<Typing> {
        analysis
            .function_body(contract, function)
            .statements
            .iter()
            .filter_map(|stmt| match stmt {
                ir::Statement::ExpressionStatement(s) => {
                    let node_id = s.expression.node_id().expect("expression has a node id");
                    Some(binder.node_typing(node_id))
                }
                _ => None,
            })
            .collect()
    };

    let is_function_value = |typing: &Typing| {
        matches!(
            typing.as_type_id().map(|id| types.get_type_by_id(id)),
            Some(Type::Function(_))
        )
    };

    let is_meta_type = |typing: &Typing| {
        typing
            .as_type_id()
            .is_some_and(|id| types.get_type_by_id(id).is_meta_type())
    };

    let d = statement_typings("D", "tD");
    // `C.g` (external) is a declaration with no mobile type.
    assert!(
        is_meta_type(&d[0]),
        "C.g should be a function declaration, got {:?}",
        d[0]
    );
    // `C.g.selector` still resolves to `bytes4`.
    assert!(
        matches!(
            d[1].as_type_id().map(|id| types.get_type_by_id(id)),
            Some(Type::ByteArray(ByteArrayType { width: 4 }))
        ),
        "C.g.selector should be bytes4, got {:?}",
        d[1]
    );
    // `C.pub` (public) accessed from a deriving contract stays a callable value.
    assert!(
        is_function_value(&d[2]),
        "local C.pub should be a callable function value, got {:?}",
        d[2]
    );
    // `C.intl` (internal) accessed locally stays a callable value.
    assert!(
        is_function_value(&d[3]),
        "local C.intl should be a callable function value, got {:?}",
        d[3]
    );
    // `I.h` (interface, external) is a declaration.
    assert!(
        is_meta_type(&d[4]),
        "I.h should be a function declaration, got {:?}",
        d[4]
    );

    // `C.pub` accessed from an unrelated contract (foreign) is a declaration.
    let e = statement_typings("E", "tE");
    assert!(
        is_meta_type(&e[0]),
        "foreign C.pub should be a function declaration, got {:?}",
        e[0]
    );
}
