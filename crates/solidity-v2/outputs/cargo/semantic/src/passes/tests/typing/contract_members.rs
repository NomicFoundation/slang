//! Typing of members reached through a contract, library or interface:
//! `this`/`super`, getters, data locations, and external signatures.

use slang_solidity_v2_common::diagnostics::kinds::resolution::AmbiguousReference;
use slang_solidity_v2_ir::ir;

use super::{Analyse, Analysis, expression, expression_statement_types, expressions};
use crate::binder::Typing;
use crate::types::{
    ByteArrayType, BytesType, ContractType, DataLocation, IntegerType, LibraryType, StringType,
    TupleType, Type,
};

/// The recovered type of each expression statement in the body of `function`,
/// declared by the contract or library `owner`, in source order. This is what
/// a test wants when it writes its own source rather than letting
/// [`ExpressionTyping`] synthesize one.
fn statement_types(analysis: &Analysis, owner: &str, function: &str) -> Vec<Option<Type>> {
    expression_statement_types(
        analysis.function_body(owner, function),
        analysis.binder(),
        analysis.types(),
    )
}

#[test]
fn test_super_keyword_types_as_super() {
    let source = r#"
        contract A {
            function f() public virtual {}
        }
        contract B is A {
            function g() public {
                super.f();
            }
        }
        "#;

    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();

    let body = analysis.function_body("B", "g");
    let statement = body.statements.first().expect("g has a statement");
    let ir::Statement::ExpressionStatement(expression_statement) = statement else {
        panic!("expected an expression statement");
    };
    let ir::Expression::FunctionCallExpression(call) = &expression_statement.expression else {
        panic!("expected a function call expression");
    };
    let ir::Expression::MemberAccessExpression(member_access) = &call.operand else {
        panic!("expected a member access expression");
    };
    let ir::Expression::SuperKeyword(super_keyword) = &member_access.operand else {
        panic!("expected a super keyword");
    };

    assert!(
        matches!(
            analysis.binder().node_typing(super_keyword.id()),
            Typing::Super
        ),
        "`super` should be typed as `Typing::Super`"
    );
}

#[test]
fn test_data_locations_of_state_variable_and_getter_accesses() {
    // In source order:
    //  - `bs` — internal access to a `bytes` storage variable: `bytes storage`.
    //  - `foo.xs` — `xs` is declared with `Inherited` location inside the
    //    struct; the member access propagates the operand's storage location.
    //  - `t.bs()` — external call to the auto-generated getter of `bytes bs`;
    //    the returned reference type lives in memory.
    //  - `t.foo()` — external call to the auto-generated getter of `Foo foo`.
    //    `Foo` has a single returnable field (`bytes xs`), so the getter
    //    returns just `bytes`, again in memory.
    let (typings, _) = expressions(&["bs", "foo.xs", "t.bs()", "t.foo()"])
        .with_members(
            r#"
            struct Foo { bytes xs; }
            bytes public bs;
            Foo public foo;
            Test t;
            "#,
        )
        .into_types();
    let expected = vec![
        Some(Type::Bytes(BytesType {
            location: DataLocation::Storage,
        })),
        Some(Type::Bytes(BytesType {
            location: DataLocation::Storage,
        })),
        Some(Type::Bytes(BytesType {
            location: DataLocation::Memory,
        })),
        Some(Type::Bytes(BytesType {
            location: DataLocation::Memory,
        })),
    ];
    assert_eq!(typings, expected);
}

#[test]
fn test_external_signature_relocates_parameters_and_results() {
    // The ABI boundary decodes a calldata-located reference into fresh memory,
    // so an externally callable signature names both its parameters and its
    // results there. In source order:
    //  - `keep` — an *internal* reference is not an ABI boundary, so its
    //    `bytes calldata` parameter and result both stay in calldata;
    //  - `this.echo` — the same signature reached externally: parameter and
    //    result both read back as `bytes memory`;
    //  - `this.echo(bs)` — the call result is `bytes memory`;
    //  - `this.echo(bs)[0]` — indexing that result reads memory rather than a
    //    calldata offset, yielding `bytes1`;
    //  - `this.split(bs)` — multiple results are modeled as a tuple, which
    //    carries no location of its own, so the relocation has to reach each
    //    element separately: the `bytes calldata` result becomes memory while
    //    the results that were not in calldata keep their declared type.
    let (typings, types) = expressions(&[
        "keep",
        "this.echo",
        "this.echo(bs)",
        "this.echo(bs)[0]",
        "this.split(bs)",
    ])
    .with_members(
        r#"
        bytes bs;
        function echo(bytes calldata xs) external pure returns (bytes calldata) { return xs; }
        function keep(bytes calldata xs) internal pure returns (bytes calldata) { return xs; }
        function split(bytes calldata xs)
            external
            pure
            returns (bytes calldata, string memory, uint)
        {
            return (xs, "", 1);
        }
        "#,
    )
    .into_types();
    let [
        internal_reference,
        external_reference,
        call,
        indexed,
        tuple_call,
    ] = typings.as_slice()
    else {
        panic!("expected five expression statements, got {typings:?}");
    };

    // Both function references carry the same declared signature, differing
    // only in whether it was relocated for the ABI boundary.
    let signature_locations = |typing: &Option<Type>| {
        let Some(Type::Function(function_type)) = typing else {
            panic!("expected a function type, got {typing:?}");
        };
        let [parameter_type_id] = function_type.parameter_types.as_slice() else {
            panic!("expected a single parameter");
        };
        (
            types.get_type_by_id(*parameter_type_id).clone(),
            types.get_type_by_id(function_type.return_type).clone(),
        )
    };

    let calldata_bytes = Type::Bytes(BytesType {
        location: DataLocation::Calldata,
    });
    let memory_bytes = Type::Bytes(BytesType {
        location: DataLocation::Memory,
    });

    assert_eq!(
        signature_locations(internal_reference),
        (calldata_bytes.clone(), calldata_bytes),
        "an internal reference keeps its declared calldata locations",
    );
    assert_eq!(
        signature_locations(external_reference),
        (memory_bytes.clone(), memory_bytes.clone()),
        "an external reference relocates both its parameter and its result",
    );
    assert_eq!(call, &Some(memory_bytes.clone()));
    assert_eq!(indexed, &Some(Type::ByteArray(ByteArrayType { width: 1 })));

    let Some(Type::Tuple(TupleType { types: elements })) = tuple_call else {
        panic!("expected `this.split(bs)` to be typed as a tuple, got {tuple_call:?}");
    };
    let element_types: Vec<Type> = elements
        .iter()
        .map(|type_id| types.get_type_by_id(*type_id).clone())
        .collect();
    assert_eq!(
        element_types,
        vec![
            memory_bytes,
            Type::String(StringType {
                location: DataLocation::Memory,
            }),
            Type::Integer(IntegerType {
                is_signed: false,
                bits: 256,
            }),
        ],
        "only the calldata result is relocated",
    );
}

#[test]
fn test_cast_address_to_library_is_library_typed() {
    // Casting an address to a library (`MyLib(x)`) is valid Solidity and
    // yields a value of the library type, which can then be compared against
    // another library value.
    let source = r#"
        library MyLib {
            function f() public pure returns (uint) { return 1; }
        }
        contract Test {
            function probe(address x, address y) internal pure {
                MyLib(x);
                MyLib(x) == MyLib(y);
            }
        }
    "#;
    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let typings = statement_types(&analysis, "Test", "probe");
    let [cast, comparison] = typings.as_slice() else {
        panic!("expected two expression statements, got {typings:?}");
    };

    assert!(
        matches!(cast, Some(Type::Library { .. })),
        "expected `MyLib(x)` to be typed as the library, got {cast:?}",
    );
    assert_eq!(
        comparison,
        &Some(Type::Boolean),
        "expected `MyLib(x) == MyLib(y)` to be a boolean",
    );
}

#[test]
fn test_getter_of_struct_with_function_member() {
    // The auto-generated getter of a public struct state variable returns a
    // tuple of its value-type fields.
    let (getter_type, types) = expression("other.s()")
        .with_members(
            r#"
            struct S { uint a; function() external fn; }
            S public s;
            Test other;
            "#,
        )
        .into_resolved_type();

    let Type::Tuple(TupleType { types: elements }) = getter_type else {
        panic!("expected the getter to return a tuple, got {getter_type:?}");
    };
    let element_types: Vec<&Type> = elements
        .iter()
        .map(|type_id| types.get_type_by_id(*type_id))
        .collect();

    assert!(
        matches!(
            element_types.as_slice(),
            [
                Type::Integer(IntegerType {
                    is_signed: false,
                    bits: 256
                }),
                Type::Function(_),
            ]
        ),
        "expected getter tuple `(uint256, function() external)`, got {element_types:?}",
    );
}

#[test]
fn test_getter_of_struct_with_struct_member() {
    // The auto-generated getter of a public struct state variable returns a
    // tuple of its value-type fields.
    let (getter_type, types) = expression("other.s()")
        .with_members(
            r#"
            struct P { bool a; }
            struct S { P p; uint a; }
            S public s;
            Test other;
            "#,
        )
        .into_resolved_type();

    let Type::Tuple(TupleType { types: elements }) = getter_type else {
        panic!("expected the getter to return a tuple, got {getter_type:?}");
    };
    let element_types: Vec<&Type> = elements
        .iter()
        .map(|type_id| types.get_type_by_id(*type_id))
        .collect();

    assert!(
        matches!(
            element_types.as_slice(),
            [
                Type::Struct(_),
                Type::Integer(IntegerType {
                    is_signed: false,
                    bits: 256
                }),
            ]
        ),
        "expected getter tuple `(Struct, uint256)`, got {element_types:?}",
    );
}

#[test]
fn test_this_in_library_is_library_typed() {
    // `this` inside a library function is valid Solidity and has the library
    // type
    let source = r#"
        library MyLib {
            function probe() internal view {
                this;
            }
        }
        contract Test {}
        "#;

    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let library = analysis.find_library("MyLib");
    let typings = statement_types(&analysis, "MyLib", "probe");
    assert!(
        matches!(typings.as_slice(), [Some(Type::Library(LibraryType { definition_id }))] if definition_id == &library.id()),
        "expected `this` to be typed as the library, got {typings:?}",
    );
}

#[test]
fn test_this_inside_contract() {
    let source = r#"
        contract MyContract {
            function probe() internal view {
                this;
            }
        }
        contract Test {}
        "#;

    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let contract = analysis.find_contract("MyContract");
    let typings = statement_types(&analysis, "MyContract", "probe");

    assert!(
        matches!(typings.as_slice(), [Some(Type::Contract(ContractType { definition_id }))] if definition_id == &contract.id())
    );
}

#[test]
fn test_partially_applied_function_does_not_unify_into_array() {
    // `L.inc` is attached to `uint` via `using for`, so `t.inc` binds the
    // receiver and becomes a partially applied function with no mobile type.
    let source = r#"
        library L {
            function inc(uint x) internal pure returns (uint) { return x + 1; }
        }
        contract Test {
            using L for uint;
            function inc_method(uint x) internal pure returns (uint) { return x; }
            function foo() external {}
            function __test() internal {
                uint t = 1;
                [inc_method, inc_method];
                [inc_method, t.inc];
                [this.foo, this.foo];
                [this.foo, this.foo{ gas: 4 }];
            }
        }
        "#;

    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let mut typings = statement_types(&analysis, "Test", "__test").into_iter();

    // Control: plain function pointers of the same signature still unify into a
    // fixed-size array.
    assert!(
        matches!(typings.next(), Some(Some(Type::FixedSizeArray(_)))),
        "plain function pointers should unify into an array",
    );

    // The bound element has no mobile type, so the array does not type.
    assert_eq!(
        typings.next(),
        Some(None::<Type>),
        "an array with a partially applied element should not type",
    );

    // Control: plain function pointers of the same signature still unify into a
    // fixed-size array.
    assert!(
        matches!(typings.next(), Some(Some(Type::FixedSizeArray(_)))),
        "plain function pointers should unify into an array",
    );

    // The bound element has no mobile type, so the array does not type.
    assert_eq!(
        typings.next(),
        Some(None::<Type>),
        "an array with a partially applied element should not type",
    );
}

// A partially applied function (bound first argument or pre-applied call
// options) is not
// implicitly convertible to its plain function pointer counterpart, even
// though they share the same signature.
#[test]
fn test_partially_applied_function_is_not_convertible() {
    let source = r#"
        library L {
            function inc(uint x) internal pure {}
        }
        contract Test {
            using L for uint;
            function foo() external {}
            
            function take_internal(function(uint) internal pure f) internal pure returns (bool) {}
            function take_internal(uint f) internal pure returns (uint) {}
            
            function take_external(function() external g) internal pure returns (bool) {}
            function take_external(uint g) internal pure returns (uint) {}
            
            function __test() internal view {
                uint t = 1;
        
                take_internal(L.inc); // <------------- works
                take_internal(t.inc); // <------------- fails
                take_external(this.foo); // <---------- works
                take_external(this.foo{gas: 4}); // <-- fails
            }
        }        
        "#;

    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let mut typings = statement_types(&analysis, "Test", "__test").into_iter();

    assert!(
        matches!(typings.next(), Some(Some(Type::Boolean))),
        "plain library function should be convertible",
    );

    assert!(
        matches!(typings.next(), Some(None)),
        "partially applied function pointers should not be convertible",
    );

    assert!(
        matches!(typings.next(), Some(Some(Type::Boolean))),
        "plain function pointers should be convertible",
    );

    assert!(
        matches!(typings.next(), Some(None)),
        "partially applied function pointers should not be convertible",
    );
}

#[test]
fn reference_type_constant_is_indexable() {
    let (element_type, _types) = expression("B[0]")
        .with_members(r#"bytes constant B = hex"1234";"#)
        .into_resolved_type();
    assert_eq!(element_type, Type::ByteArray(ByteArrayType { width: 1 }));
}

#[test]
fn test_event_selector() {
    // `.selector` on an event name types as `bytes32`: the event's `topics[0]`.
    let (type_, _) = expression("E.selector")
        .with_members("event E(uint a);")
        .into_resolved_type();
    assert_eq!(type_, Type::ByteArray(ByteArrayType { width: 32 }));

    // With *overloaded* events the name is ambiguous, and nothing narrows it
    // down: the member access uses it as a value rather than calling it.
    assert_eq!(
        Some(
            AmbiguousReference {
                name: "E".to_owned()
            }
            .into()
        ),
        expression("E.selector")
            .with_members("event E(uint a); event E(bool b);")
            .into_diagnostic(),
    );

    // An anonymous event emits no `topics[0]`, so it exposes no `selector`.
    let (type_, _) = expression("E.selector")
        .with_members("event E(uint a) anonymous;")
        .into_type();
    assert_eq!(None, type_);
}

#[test]
fn test_bytes_and_string_concat_typing() {
    // `concat` resolves as a member of the *meta-type* of `bytes`/`string`,
    // and the two built-ins stay distinct: `bytes.concat` yields
    // `bytes memory` while `string.concat` yields `string memory`.
    let (type_, _) = expression(r#"bytes.concat(hex"01", hex"02")"#).into_resolved_type();
    assert_eq!(
        type_,
        Type::Bytes(BytesType {
            location: DataLocation::Memory
        })
    );

    let (type_, _) = expression(r#"string.concat("a", "b")"#).into_resolved_type();
    assert_eq!(
        type_,
        Type::String(StringType {
            location: DataLocation::Memory
        })
    );
}

#[test]
fn test_static_library_call_is_not_partially_applied() {
    // With a matching `using` directive in scope, a *static* call through the
    // library name must still resolve to the full function: the type name `L`
    // is not a value receiver, so it must not bind the first parameter as a
    // partial application.
    let source = r#"
        library L {
            function f(uint x) internal pure returns (bool) { return x > 0; }
        }
        contract Test {
            using L for uint;
            function __test() internal pure {
                L.f(1);
            }
        }
        "#;
    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let typings = statement_types(&analysis, "Test", "__test");
    assert_eq!(typings, vec![Some(Type::Boolean)]);
}
