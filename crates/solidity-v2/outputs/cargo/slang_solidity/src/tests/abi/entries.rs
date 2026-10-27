use super::fixtures;
use crate::abi::{AbiEntry, AbiParameter};

#[test]
fn test_get_contracts_abi() {
    let unit = fixtures::Counter::build_compilation_unit();

    let contracts = unit.compute_contracts_abi();
    assert_eq!(1, contracts.len());
    assert_eq!("Counter", contracts[0].name());
    assert_eq!("main.sol", contracts[0].file_id().as_str());
    assert_eq!(7, contracts[0].entries().len());

    let entries = contracts[0].entries().collect::<Vec<_>>();

    assert!(matches!(entries[0], AbiEntry::Constructor(_)));
    assert!(matches!(&entries[1], AbiEntry::Function(f) if f.name() == "click"));
    assert!(matches!(&entries[2], AbiEntry::Function(f) if f.name() == "count"));
    assert!(matches!(&entries[3], AbiEntry::Function(f) if f.name() == "disable"));
    assert!(matches!(&entries[4], AbiEntry::Function(f) if f.name() == "enable"));
    assert!(matches!(&entries[5], AbiEntry::Function(f) if f.name() == "increment"));
    assert!(matches!(&entries[6], AbiEntry::Function(f) if f.name() == "isEnabled"));
}

#[test]
fn test_full_abi_with_events_and_errors() {
    let unit = super::FullAbi::build_compilation_unit();

    let contracts_abi = unit.compute_contracts_abi();
    assert_eq!(contracts_abi.len(), 2);
    assert_eq!(contracts_abi[0].name(), "Base");
    assert_eq!(contracts_abi[1].name(), "Test");

    let entries = contracts_abi[0].entries().collect::<Vec<_>>();
    assert_eq!(entries.len(), 1);
    assert!(matches!(&entries[0], AbiEntry::Function(f) if f.name() == "xs"));

    let entries = contracts_abi[1].entries().collect::<Vec<_>>();
    assert_eq!(entries.len(), 10);
    assert!(matches!(entries[0], AbiEntry::Constructor(_)));
    assert!(matches!(&entries[1], AbiEntry::Error(e) if e.name() == "InsufficientBalance"));
    assert!(matches!(&entries[2], AbiEntry::Error(e) if e.name() == "SomethingWrong"));
    assert!(matches!(&entries[3], AbiEntry::Event(e) if e.name() == "BaseEvent"));
    assert!(matches!(&entries[4], AbiEntry::Event(e) if e.name() == "Event"));
    assert!(matches!(entries[5], AbiEntry::Fallback(_)));
    assert!(matches!(&entries[6], AbiEntry::Function(f) if f.name() == "b"));
    assert!(matches!(&entries[7], AbiEntry::Function(f) if f.name() == "foo"));
    assert!(matches!(&entries[8], AbiEntry::Function(f) if f.name() == "xs"));
    assert!(matches!(entries[9], AbiEntry::Receive(_)));
}

#[test]
fn test_abi_entries_with_tuples() {
    let unit = super::AbiWithTuples::build_compilation_unit();

    let contracts_abi = unit.compute_contracts_abi();
    assert_eq!(contracts_abi.len(), 1);
    assert_eq!(contracts_abi[0].name(), "Test");

    let entries = contracts_abi[0].entries().collect::<Vec<_>>();
    assert_eq!(entries.len(), 5);

    // The parameters' types are pinned by `json::tuples_and_getters_serialize`, on the same
    // fixture.

    // f(S memory, T memory, uint x) returns ()
    let AbiEntry::Function(f) = &entries[0] else {
        panic!("expected ABI for function");
    };
    assert_eq!(f.name(), "f");
    let inputs = f.inputs().collect::<Vec<_>>();
    assert_eq!(inputs.len(), 3);
    assert_eq!(inputs[0].name(), None);
    assert_eq!(inputs[1].name(), None);
    assert_eq!(inputs[2].name(), Some("x"));
    assert_eq!(f.outputs().len(), 0);

    // g() returns (S memory s, T memory t, uint)
    let AbiEntry::Function(g) = &entries[1] else {
        panic!("expected ABI for function");
    };
    assert_eq!(g.name(), "g");
    assert_eq!(g.inputs().len(), 0);
    let outputs = g.outputs().collect::<Vec<_>>();
    assert_eq!(outputs.len(), 3);
    assert_eq!(outputs[0].name(), Some("s"));
    assert_eq!(outputs[1].name(), Some("t"));
    assert_eq!(outputs[2].name(), None);

    // T public (getter) -> t() returns (uint256 x, uint256 y).
    // The getter flattens the struct into its members; their names aren't tracked.
    let AbiEntry::Function(getter) = &entries[2] else {
        panic!("expected ABI for function");
    };
    assert_eq!(getter.name(), "t");
    assert_eq!(getter.inputs().len(), 0);
    let outputs = getter.outputs().collect::<Vec<_>>();
    assert_eq!(outputs.len(), 2);

    // t_components() returns (uint, uint)
    let AbiEntry::Function(t_components) = &entries[3] else {
        panic!("expected ABI for function");
    };
    assert_eq!(t_components.name(), "t_components");
    assert_eq!(t_components.inputs().len(), 0);
    let outputs = t_components.outputs().collect::<Vec<_>>();
    assert_eq!(outputs.len(), 2);
    assert_eq!(outputs[0].name(), None);
    assert_eq!(outputs[1].name(), None);

    // t_struct() returns (T memory)
    let AbiEntry::Function(t_struct) = &entries[4] else {
        panic!("expected ABI for function");
    };
    assert_eq!(t_struct.name(), "t_struct");
    assert_eq!(t_struct.inputs().len(), 0);
    let outputs = t_struct.outputs().collect::<Vec<_>>();
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].name(), None);
}

/// A parameter spells its JSON `type`, selector type and `internalType`, and a `tuple` lists its
/// struct's members as components, through any arrays.
#[test]
fn parameters_expose_type_names_and_components() {
    let unit = super::AbiWithTuples::build_compilation_unit();
    let contracts_abi = unit.compute_contracts_abi();
    let Some(AbiEntry::Function(f)) = contracts_abi[0].entries().next() else {
        panic!("expected ABI for function");
    };
    assert_eq!(f.name(), "f");

    let inputs = f.inputs().collect::<Vec<_>>();
    let types = |parameter: &AbiParameter<'_>| {
        (
            parameter.json_type_name(),
            parameter.selector_type_name(),
            parameter.internal_type_name(),
        )
    };
    assert_eq!(
        inputs.iter().map(types).collect::<Vec<_>>(),
        [
            (
                "tuple".to_owned(),
                "(uint256,uint256[],(uint256,uint256)[])".to_owned(),
                "struct Test.S".to_owned()
            ),
            (
                "tuple".to_owned(),
                "(uint256,uint256)".to_owned(),
                "struct Test.T".to_owned()
            ),
            (
                "uint256".to_owned(),
                "uint256".to_owned(),
                "uint256".to_owned()
            ),
        ]
    );
    assert!(inputs[2].components().is_none());

    let s_members = inputs[0]
        .components()
        .expect("a struct has components")
        .collect::<Vec<_>>();
    assert_eq!(
        s_members
            .iter()
            .map(|member| member.name())
            .collect::<Vec<_>>(),
        [Some("a"), Some("b"), Some("c")]
    );
    assert_eq!(
        s_members.iter().map(types).collect::<Vec<_>>(),
        [
            (
                "uint256".to_owned(),
                "uint256".to_owned(),
                "uint256".to_owned()
            ),
            (
                "uint256[]".to_owned(),
                "uint256[]".to_owned(),
                "uint256[]".to_owned()
            ),
            (
                "tuple[]".to_owned(),
                "(uint256,uint256)[]".to_owned(),
                "struct Test.T[]".to_owned()
            ),
        ]
    );
    assert!(s_members[1].components().is_none());

    // The components of `T[] c` are `T`'s members, through the array.
    let t_members = s_members[2]
        .components()
        .expect("an array of structs has components")
        .map(|member| member.name())
        .collect::<Vec<_>>();
    assert_eq!(t_members, [Some("x"), Some("y")]);
}

/// An abstract contract cannot run its constructor, so solc leaves it out of the ABI.
#[test]
fn abstract_contract_has_no_constructor_entry() {
    let unit = super::AbstractContract::build_compilation_unit();
    let abi = unit
        .find_contract_by_name("AB")
        .next()
        .expect("contract AB exists")
        .compute_abi()
        .expect("the ABI is computable");
    assert!(
        matches!(abi.entries().collect::<Vec<_>>()[..], [AbiEntry::Function(f)] if f.name() == "extra")
    );
}

/// With the `function` keyword, `receive` and `fallback` are ordinary names: solc lists both
/// as functions with selectors, next to the real receive and fallback functions.
#[test]
fn functions_named_receive_and_fallback_are_regular_functions() {
    let unit = super::FunctionsNamedReceiveAndFallback::build_compilation_unit();
    let contracts_abi = unit.compute_contracts_abi();
    let entries = contracts_abi[0].entries().collect::<Vec<_>>();
    assert_eq!(entries.len(), 4);
    assert!(matches!(entries[0], AbiEntry::Fallback(_)));
    assert!(matches!(&entries[1], AbiEntry::Function(f) if f.name() == "fallback"));
    assert!(matches!(&entries[2], AbiEntry::Function(f) if f.name() == "receive"));
    assert!(matches!(entries[3], AbiEntry::Receive(_)));
}

/// In a library, the selector type names user-defined types, as the library's selectors do,
/// while the JSON `type` still expands them.
#[test]
fn library_parameters_name_user_types_in_selector_type() {
    let unit = super::LibraryUserTypes::build_compilation_unit();
    let abi = fixtures::find_library(&unit, "L")
        .compute_abi()
        .expect("the ABI is computable");
    let Some(AbiEntry::Function(f)) = abi.entries().next() else {
        panic!("expected ABI for function");
    };

    let types = |parameter: AbiParameter<'_>| {
        (
            parameter.json_type_name(),
            parameter.selector_type_name(),
            parameter.internal_type_name(),
        )
    };
    let inputs = f.inputs().collect::<Vec<_>>();
    assert_eq!(
        inputs.iter().copied().map(types).collect::<Vec<_>>(),
        [
            (
                "tuple[]".to_owned(),
                "L.S[]".to_owned(),
                "struct L.S[]".to_owned()
            ),
            ("L.E".to_owned(), "L.E".to_owned(), "enum L.E".to_owned()),
        ]
    );
    assert_eq!(
        inputs[0]
            .components()
            .expect("an array of structs has components")
            .map(types)
            .collect::<Vec<_>>(),
        [
            ("L.E".to_owned(), "L.E".to_owned(), "enum L.E".to_owned()),
            (
                "uint256[]".to_owned(),
                "uint256[]".to_owned(),
                "uint256[]".to_owned()
            ),
        ]
    );
}
