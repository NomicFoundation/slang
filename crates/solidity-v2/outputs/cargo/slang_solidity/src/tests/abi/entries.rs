use super::fixtures;
use crate::abi::AbiEntry;

/// Convenience macro to test equality of tuple components for input/output parameters.
///
/// Usage:
/// ```
/// assert_components_eq!(inputs[0].components(), [
///     ("a", "uint256", []),
///     ("c", "tuple[]", [
///         ("x", "uint256", []),
///         ("y", "uint256", []),
///     ]),
/// ]);
/// ```
macro_rules! assert_components_eq {
    ($components:expr, [ $(($name:expr, $type_name:expr, [$($sub:tt)*])),* $(,)? ]) => {
        {
            let components = $components;
            let expected: &[(&str, &str)] = &[$(($name, $type_name)),*];
            assert_eq!(
                components.len(),
                expected.len(),
                "expected {} components, got {}",
                expected.len(),
                components.len()
            );
            assert_components_eq!(@step components, 0, $(($name, $type_name, [$($sub)*]),)*);
        }
    };
    (@step $components:ident, $index:expr, ) => {};
    (@step $components:ident, $index:expr, ($name:expr, $type_name:expr, [$($sub:tt)*]), $($rest:tt)*) => {
        assert_eq!($components[$index].name(), $name);
        assert_eq!($components[$index].type_name(), $type_name);
        assert_components_eq!($components[$index].components(), [$($sub)*]);
        assert_components_eq!(@step $components, $index + 1, $($rest)*);
    };
}

#[test]
fn test_get_contracts_abi() {
    let unit = fixtures::Counter::build_compilation_unit();

    let contracts = unit.compute_contracts_abi();
    assert_eq!(1, contracts.len());
    assert_eq!("Counter", contracts[0].name());
    assert_eq!("main.sol", contracts[0].file_id().to_string());
    assert_eq!(7, contracts[0].entries().len());

    let entries = contracts[0].entries();

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

    let entries = contracts_abi[0].entries();
    assert_eq!(entries.len(), 1);
    assert!(matches!(&entries[0], AbiEntry::Function(f) if f.name() == "xs"));

    let entries = contracts_abi[1].entries();
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

    let entries = contracts_abi[0].entries();

    let AbiEntry::Function(function) = &entries[0] else {
        panic!("expected ABI for function");
    };
    assert_eq!(function.name(), "f");
    let inputs = function.inputs();
    let outputs = function.outputs();
    assert_eq!(inputs.len(), 3);
    assert!(inputs[0].name().is_none());
    assert_eq!(inputs[0].type_name(), "tuple");
    assert_components_eq!(
        inputs[0].components(),
        [
            ("a", "uint256", []),
            ("b", "uint256[]", []),
            (
                "c",
                "tuple[]",
                [("x", "uint256", []), ("y", "uint256", []),]
            ),
        ]
    );

    assert!(inputs[1].name().is_none());
    assert_eq!(inputs[1].type_name(), "tuple");
    assert_components_eq!(
        inputs[1].components(),
        [("x", "uint256", []), ("y", "uint256", []),]
    );

    assert!(inputs[2].name().is_none());
    assert_eq!(inputs[2].type_name(), "uint256");
    assert!(inputs[2].components().is_empty());
    assert!(outputs.is_empty());

    let AbiEntry::Function(function) = &entries[1] else {
        panic!("expected ABI for function");
    };
    assert_eq!(function.name(), "g");
    let inputs = function.inputs();
    let outputs = function.outputs();
    assert!(inputs.is_empty());
    assert_eq!(outputs.len(), 3);
    assert!(outputs[0].name().is_none());
    assert_eq!(outputs[0].type_name(), "tuple");
    assert_components_eq!(
        outputs[0].components(),
        [
            ("a", "uint256", []),
            ("b", "uint256[]", []),
            (
                "c",
                "tuple[]",
                [("x", "uint256", []), ("y", "uint256", []),]
            ),
        ]
    );
    assert!(outputs[1].name().is_none());
    assert_eq!(outputs[1].type_name(), "tuple");
    assert_components_eq!(
        outputs[1].components(),
        [("x", "uint256", []), ("y", "uint256", []),]
    );
    assert!(outputs[2].name().is_none());
    assert_eq!(outputs[2].type_name(), "uint256");
    assert!(outputs[2].components().is_empty());
}
