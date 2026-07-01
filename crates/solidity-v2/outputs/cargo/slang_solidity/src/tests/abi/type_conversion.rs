use crate::abi::{AbiEntry, AbiType, TupleComponent};
use crate::ast::{StateVariableDefinition, Type};
use crate::define_fixture;

define_fixture!(
    TypeConversion,
    file: "main.sol", r#"
contract C {
    type Id is uint64;
    enum Color { Red, Green, Blue }

    struct T { uint x; uint y; }
    struct S { uint a; uint[] b; T[] c; }
    struct Node { Node[] children; uint value; }

    uint256 public u;
    T public t;
    S public s;
    mapping(uint => uint) public m;

    // Scalars (also used to cross-check the semantic and AST paths via getters).
    uint8 public u8;
    bool public flag;
    bytes8 public b8;
    Color public color;
    Id public id;

    // Arrays.
    uint[3] public fixedArr;
    uint[][] public nested;

    // Recursive struct (legal via the dynamic array); internal, no getter.
    Node n;
}
"#,
);

/// Resolves the named state variable definition in contract `C`.
fn state_var(name: &str) -> StateVariableDefinition {
    let unit = TypeConversion::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract C exists");

    for state_var in contract.state_variables() {
        if state_var.name().name() == name {
            return state_var;
        }
    }
    panic!("state variable `{name}` not found");
}

/// Resolves the AST `Type` of the named state variable in contract `C`.
fn variable_type(name: &str) -> Type {
    state_var(name)
        .get_type()
        .expect("state variable has a type")
}

/// The first output type of the named public variable's generated getter,
/// obtained via the semantic (`TypeId`) conversion path.
fn getter_output_abi_type(name: &str) -> AbiType {
    let AbiEntry::Function(getter) = state_var(name)
        .compute_abi_entry()
        .expect("public variable has a getter")
    else {
        panic!("a state variable getter is a function");
    };
    getter.outputs()[0].abi_type().clone()
}

fn uint256() -> AbiType {
    AbiType::Integer {
        is_signed: false,
        bits: 256,
    }
}

#[test]
fn converts_a_leaf_type() {
    assert_eq!(AbiType::try_from(&variable_type("u")), Ok(uint256()));
}

#[test]
fn converts_a_struct_to_a_tuple() {
    let expected = AbiType::Tuple(vec![
        TupleComponent::new("x", uint256()),
        TupleComponent::new("y", uint256()),
    ]);
    assert_eq!(AbiType::try_from(&variable_type("t")), Ok(expected));
}

#[test]
fn converts_nested_structs_and_arrays() {
    // struct S { uint a; uint[] b; T[] c; }  where  struct T { uint x; uint y; }
    let t = AbiType::Tuple(vec![
        TupleComponent::new("x", uint256()),
        TupleComponent::new("y", uint256()),
    ]);
    let expected = AbiType::Tuple(vec![
        TupleComponent::new("a", uint256()),
        TupleComponent::new(
            "b",
            AbiType::Array {
                element: Box::new(uint256()),
            },
        ),
        TupleComponent::new(
            "c",
            AbiType::Array {
                element: Box::new(t),
            },
        ),
    ]);

    assert_eq!(AbiType::try_from(&variable_type("s")), Ok(expected));
}

#[test]
fn rejects_a_type_with_no_abi_representation() {
    // A mapping has no ABI type. `NotAnAbiType` is `#[non_exhaustive]`, so we
    // assert on `is_err` rather than constructing it.
    assert!(AbiType::try_from(&variable_type("m")).is_err());
}

#[test]
fn rejects_a_recursive_struct() {
    // `struct Node { Node[] children; ... }` recurses through a dynamic array,
    // which is legal Solidity but has no finite ABI representation. The cycle
    // guard makes the conversion fail rather than recurse forever.
    assert!(AbiType::try_from(&variable_type("n")).is_err());
}

#[test]
fn converts_non_identity_leaf_types() {
    // Conversions whose mapping is not the identity, and are most likely to
    // regress: enum -> uint8, UDVT -> underlying, bytesN, bool, fixed/multi-dim
    // arrays.
    let cases: &[(&str, AbiType, &str)] = &[
        (
            "u8",
            AbiType::Integer {
                is_signed: false,
                bits: 8,
            },
            "uint8",
        ),
        ("flag", AbiType::Boolean, "bool"),
        ("b8", AbiType::ByteArray { width: 8 }, "bytes8"),
        (
            "color",
            AbiType::Integer {
                is_signed: false,
                bits: 8,
            },
            "uint8",
        ),
        (
            "id",
            AbiType::Integer {
                is_signed: false,
                bits: 64,
            },
            "uint64",
        ),
        (
            "fixedArr",
            AbiType::FixedSizeArray {
                element: Box::new(uint256()),
                size: 3,
            },
            "uint256[3]",
        ),
        (
            "nested",
            AbiType::Array {
                element: Box::new(AbiType::Array {
                    element: Box::new(uint256()),
                }),
            },
            "uint256[][]",
        ),
    ];

    for (name, expected, spelling) in cases {
        let abi = AbiType::try_from(&variable_type(name)).expect("has an ABI type");
        assert_eq!(&abi, expected, "value mismatch for `{name}`");
        assert_eq!(&abi.to_string(), spelling, "spelling mismatch for `{name}`");
    }
}

#[test]
fn semantic_and_ast_paths_agree() {
    // The semantic-`TypeId` path (via the generated getter) and the public AST
    // `TryFrom` path must produce the same `AbiType` for the same declaration.
    for name in ["u", "u8", "flag", "b8", "color", "id"] {
        let ast = AbiType::try_from(&variable_type(name)).expect("has an ABI type");
        let semantic = getter_output_abi_type(name);
        assert_eq!(ast, semantic, "path mismatch for `{name}`");
    }
}

#[test]
fn display_spellings() {
    // An empty struct cannot be declared in Solidity, but the canonical
    // spelling of an empty tuple is still well-defined.
    assert_eq!(AbiType::Tuple(vec![]).to_string(), "()");
    assert_eq!(uint256().to_string(), "uint256");
    assert_eq!(
        AbiType::Array {
            element: Box::new(uint256()),
        }
        .to_string(),
        "uint256[]"
    );
}
