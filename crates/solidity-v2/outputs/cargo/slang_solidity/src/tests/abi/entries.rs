use super::fixtures;
use crate::abi::{AbiEntry, AbiType, TupleComponent};

#[test]
fn test_get_abi() {
    let abis = fixtures::Counter::build_compilation_unit().compute_abi();

    let [activatable, counter, ownable] = abis.as_slice() else {
        panic!("Expected 3 ABIs");
    };

    assert_eq!(activatable.name(), "Activatable");
    assert_eq!(counter.name(), "Counter");
    assert_eq!(ownable.name(), "Ownable");

    assert_eq!("main.sol", counter.file_id().as_str());

    let entries = counter.entries();
    assert_eq!(7, entries.len());
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
    let abis = super::FullAbi::build_compilation_unit().compute_abi();

    let [itest, base, test, lib] = abis.as_slice() else {
        panic!("Expected 4 ABIs")
    };

    assert_eq!(itest.name(), "ITest");

    let entries = itest.entries();
    assert_eq!(entries.len(), 2);
    assert!(matches!(&entries[0], AbiEntry::Error(e) if e.name() == "SomethingWrong"));
    assert!(matches!(&entries[1], AbiEntry::Event(e) if e.name() == "BaseEvent"));

    assert_eq!(base.name(), "Base");

    let entries = base.entries();
    assert_eq!(entries.len(), 1);
    assert!(matches!(&entries[0], AbiEntry::Function(f) if f.name() == "xs"));

    assert_eq!(test.name(), "Test");

    let entries = test.entries();
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

    assert_eq!(lib.name(), "Lib");

    let entries = lib.entries();
    assert_eq!(entries.len(), 1);
    assert!(matches!(&entries[0], AbiEntry::Function(f) if f.name() == "version"));
}

#[test]
fn test_abi_entries_with_tuples() {
    let abis = super::AbiWithTuples::build_compilation_unit().compute_abi();

    let [test] = abis.as_slice() else {
        panic!("Expected 1 ABI");
    };

    assert_eq!(test.name(), "Test");
    let entries = test.entries();
    assert_eq!(entries.len(), 5);

    let uint256 = AbiType::Integer {
        is_signed: false,
        bits: 256,
    };

    // The two structs used throughout, expanded to their ABI tuple types:
    //   struct S { uint a; uint[] b; T[] c; }
    //   struct T { uint x; uint y; }
    let t = AbiType::Tuple(vec![
        TupleComponent::new("x", uint256.clone()),
        TupleComponent::new("y", uint256.clone()),
    ]);
    let s = AbiType::Tuple(vec![
        TupleComponent::new("a", uint256.clone()),
        TupleComponent::new(
            "b",
            AbiType::Array {
                element: Box::new(uint256.clone()),
            },
        ),
        TupleComponent::new(
            "c",
            AbiType::Array {
                element: Box::new(t.clone()),
            },
        ),
    ]);

    // f(S memory, T memory, uint x) returns ()
    let AbiEntry::Function(f) = &entries[0] else {
        panic!("expected ABI for function");
    };
    assert_eq!(f.name(), "f");
    let inputs = f.inputs();
    assert_eq!(inputs.len(), 3);
    assert_eq!(inputs[0].name(), None);
    assert_eq!(inputs[0].abi_type(), s);
    assert_eq!(inputs[1].name(), None);
    assert_eq!(inputs[1].abi_type(), t);
    assert_eq!(inputs[2].name(), Some("x"));
    assert_eq!(inputs[2].abi_type(), uint256);
    assert!(f.outputs().is_empty());

    // g() returns (S memory s, T memory t, uint)
    let AbiEntry::Function(g) = &entries[1] else {
        panic!("expected ABI for function");
    };
    assert_eq!(g.name(), "g");
    assert!(g.inputs().is_empty());
    let outputs = g.outputs();
    assert_eq!(outputs.len(), 3);
    assert_eq!(outputs[0].name(), Some("s"));
    assert_eq!(outputs[0].abi_type(), s);
    assert_eq!(outputs[1].name(), Some("t"));
    assert_eq!(outputs[1].abi_type(), t);
    assert_eq!(outputs[2].name(), None);
    assert_eq!(outputs[2].abi_type(), uint256);

    // T public (getter) -> t() returns (uint256 x, uint256 y).
    // The getter flattens the struct into its members; their names aren't tracked.
    let AbiEntry::Function(getter) = &entries[2] else {
        panic!("expected ABI for function");
    };
    assert_eq!(getter.name(), "t");
    assert!(getter.inputs().is_empty());
    let outputs = getter.outputs();
    assert_eq!(outputs.len(), 2);
    assert_eq!(outputs[0].abi_type(), uint256);
    assert_eq!(outputs[1].abi_type(), uint256);

    // t_components() returns (uint, uint)
    let AbiEntry::Function(t_components) = &entries[3] else {
        panic!("expected ABI for function");
    };
    assert_eq!(t_components.name(), "t_components");
    assert!(t_components.inputs().is_empty());
    let outputs = t_components.outputs();
    assert_eq!(outputs.len(), 2);
    assert_eq!(outputs[0].name(), None);
    assert_eq!(outputs[0].abi_type(), uint256);
    assert_eq!(outputs[1].name(), None);
    assert_eq!(outputs[1].abi_type(), uint256);

    // t_struct() returns (T memory)
    let AbiEntry::Function(t_struct) = &entries[4] else {
        panic!("expected ABI for function");
    };
    assert_eq!(t_struct.name(), "t_struct");
    assert!(t_struct.inputs().is_empty());
    let outputs = t_struct.outputs();
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].name(), None);
    assert_eq!(outputs[0].abi_type(), t);
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
    assert!(matches!(abi.entries(), [AbiEntry::Function(f)] if f.name() == "extra"));
}

/// With the `function` keyword, `receive` and `fallback` are ordinary names: solc lists both
/// as functions with selectors, next to the real receive and fallback functions.
#[test]
fn functions_named_receive_and_fallback_are_regular_functions() {
    let abis = super::FunctionsNamedReceiveAndFallback::build_compilation_unit().compute_abi();
    let [c] = abis.as_slice() else {
        panic!("Expected 1 ABI");
    };

    assert_eq!(c.name(), "C");

    let entries = c.entries();
    assert_eq!(entries.len(), 4);
    assert!(matches!(entries[0], AbiEntry::Fallback(_)));
    assert!(matches!(&entries[1], AbiEntry::Function(f) if f.name() == "fallback"));
    assert!(matches!(&entries[2], AbiEntry::Function(f) if f.name() == "receive"));
    assert!(matches!(entries[3], AbiEntry::Receive(_)));
}
