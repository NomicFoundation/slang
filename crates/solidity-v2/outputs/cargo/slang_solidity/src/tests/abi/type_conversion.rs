use ruint::aliases::U256;
use slang_solidity_v2_ast::ast::ContractDefinition;

use crate::abi::{AbiEntry, AbiType, TupleComponent};
use crate::ast::{StateVariableDefinition, Type};
use crate::define_fixture;

define_fixture!(
    TypeConversion,
    file: "main.sol", r#"
pragma solidity *;
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

/// Get contract `C` from the fixture compilation unit.
fn get_contract_c() -> ContractDefinition {
    let unit = TypeConversion::build_compilation_unit();

    unit.find_contract_by_name("C")
        .next()
        .expect("contract C exists")
}

/// Resolves the named state variable definition in `contract`.
fn state_var(contract: &ContractDefinition, name: &str) -> StateVariableDefinition {
    for state_var in contract.state_variables() {
        if state_var.name().name() == name {
            return state_var;
        }
    }
    panic!("state variable `{name}` not found");
}

/// Resolves the AST `Type` of the named state variable in `contract`.
fn state_variable_type(contract: &ContractDefinition, name: &str) -> Type {
    state_var(contract, name)
        .get_type()
        .expect("state variable has a type")
}

/// The first output type of the named public variable's generated getter,
/// obtained via the semantic (`TypeId`) conversion path.
fn getter_output_abi_type(name: &str) -> AbiType {
    let AbiEntry::Function(getter) = state_var(&get_contract_c(), name)
        .compute_abi_entry()
        .expect("public variable has a getter")
    else {
        panic!("a state variable getter is a function");
    };
    getter.outputs()[0].abi_type()
}

fn uint256() -> AbiType {
    AbiType::Integer {
        is_signed: false,
        bits: 256,
    }
}

#[test]
fn converts_a_leaf_type() {
    assert_eq!(
        AbiType::try_from(&state_variable_type(&get_contract_c(), "u")),
        Ok(uint256())
    );
}

#[test]
fn converts_a_struct_to_a_tuple() {
    let expected = AbiType::Tuple(vec![
        TupleComponent::new("x", uint256()),
        TupleComponent::new("y", uint256()),
    ]);
    assert_eq!(
        AbiType::try_from(&state_variable_type(&get_contract_c(), "t")),
        Ok(expected)
    );
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

    assert_eq!(
        AbiType::try_from(&state_variable_type(&get_contract_c(), "s")),
        Ok(expected)
    );
}

#[test]
fn rejects_a_type_with_no_abi_representation() {
    // A mapping has no ABI type. `NotAnAbiType` is `#[non_exhaustive]`, so we
    // assert on `is_err` rather than constructing it.
    assert!(AbiType::try_from(&state_variable_type(&get_contract_c(), "m")).is_err());
}

#[test]
fn rejects_a_recursive_struct() {
    // `struct Node { Node[] children; ... }` recurses through a dynamic array,
    // which is legal Solidity but has no finite ABI representation. The cycle
    // guard makes the conversion fail rather than recurse forever.
    assert!(AbiType::try_from(&state_variable_type(&get_contract_c(), "n")).is_err());
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
                size: U256::from(3),
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

    let contract = get_contract_c();

    for (name, expected, spelling) in cases {
        let abi =
            AbiType::try_from(&state_variable_type(&contract, name)).expect("has an ABI type");
        assert_eq!(&abi, expected, "value mismatch for `{name}`");
        assert_eq!(&abi.to_string(), spelling, "spelling mismatch for `{name}`");
    }
}

#[test]
fn semantic_and_ast_paths_agree() {
    let contract = get_contract_c();

    // The semantic-`TypeId` path (via the generated getter) and the public AST
    // `TryFrom` path must produce the same `AbiType` for the same declaration.
    for name in ["u", "u8", "flag", "b8", "color", "id"] {
        let ast =
            AbiType::try_from(&state_variable_type(&contract, name)).expect("has an ABI type");
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

define_fixture!(
    ParameterKinds,
    file: "main.sol", r#"
pragma solidity ^0.8.0;
interface I { function f() external; }
library L {}
type Wad is uint256;
contract C {
    enum E { A }
    struct T { uint x; E e; }
    struct WithMapping { mapping(uint => uint) m; }
    struct Node { Node[] children; }

    function accepted(
        address payable a, bool b, bytes calldata c, string memory s, function(uint) external f,
        int128 i, E e, bytes4 b4, uint[] memory arr, uint[2] memory fixedArr, T memory t, Wad w,
        C self, I iface
    ) external pure {}
    function mappingParameter(mapping(uint => uint) storage m) external {}
    function structWithMapping(WithMapping storage m) external {}
    function recursiveStruct(Node memory n) external pure {}
    function libraryParameter(L l) external pure {}
}
"#,
);

#[test]
fn abi_entries_accept_exactly_the_parameters_with_an_abi_type() {
    // `AbiParameter` decides whether a type has an ABI representation without building it, so
    // an entry must exist exactly when the public `TryFrom` conversion succeeds for every
    // parameter, and the two must then agree on the `AbiType`.
    let unit = ParameterKinds::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract C exists");

    let mut checked = 0;
    for function in contract.linearised_functions() {
        let name = function
            .name()
            .expect("functions are named")
            .name()
            .to_owned();
        let parameters = function.parameters().iter().collect::<Vec<_>>();
        let convertible = parameters.iter().all(|parameter| {
            parameter
                .get_type()
                .is_some_and(|parameter_type| AbiType::try_from(&parameter_type).is_ok())
        });

        let entry = function.compute_abi_entry();
        assert_eq!(
            entry.is_some(),
            convertible,
            "acceptance mismatch for `{name}`"
        );
        match entry {
            None => {}
            Some(AbiEntry::Function(entry)) => {
                assert_eq!(
                    entry.inputs().len(),
                    parameters.len(),
                    "the entry dropped a parameter of `{name}`"
                );
                for (input, parameter) in entry.inputs().iter().zip(&parameters) {
                    let expected = AbiType::try_from(&parameter.get_type().expect("typed above"))
                        .expect("converted above");
                    assert_eq!(input.abi_type(), expected, "type mismatch in `{name}`");
                    assert_eq!(
                        AbiType::try_from(&input.get_type()),
                        Ok(expected),
                        "the parameter's view resolves to a different type in `{name}`"
                    );
                }
            }
            Some(other) => {
                panic!("`{name}` is a regular function, but its ABI entry is {other:?}")
            }
        }
        checked += 1;
    }
    assert_eq!(checked, 5, "the fixture declares 5 functions");
}
