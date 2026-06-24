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

/// Like [`assert_components_eq!`], but for top-level input/output parameters,
/// whose names are optional. For each parameter, the name is `None` to assert
/// it is unnamed, or `_` to skip the name check.
macro_rules! assert_params_eq {
    ($params:expr, [ $(($name:tt, $type_name:expr, [$($sub:tt)*])),* $(,)? ]) => {
        {
            let params = $params;
            let expected: &[&str] = &[$($type_name),*];
            assert_eq!(
                params.len(),
                expected.len(),
                "expected {} parameters, got {}",
                expected.len(),
                params.len()
            );
            assert_params_eq!(@step params, 0, $(($name, $type_name, [$($sub)*]),)*);
        }
    };
    (@step $params:ident, $index:expr, ) => {};
    (@step $params:ident, $index:expr, ($name:tt, $type_name:expr, [$($sub:tt)*]), $($rest:tt)*) => {
        assert_params_eq!(@name $params[$index], $name);
        assert_eq!($params[$index].type_name(), $type_name);
        assert_components_eq!($params[$index].components(), [$($sub)*]);
        assert_params_eq!(@step $params, $index + 1, $($rest)*);
    };
    (@name $param:expr, _) => {};
    (@name $param:expr, None) => { assert!($param.name().is_none()); };
}

/// Asserts that an [`AbiEntry`] is a function with the given name, inputs and
/// outputs (each described as for [`assert_params_eq!`]).
macro_rules! assert_function_eq {
    ($entry:expr, $name:expr, inputs: [$($inputs:tt)*], outputs: [$($outputs:tt)*] $(,)?) => {
        {
            let AbiEntry::Function(function) = $entry else {
                panic!("expected ABI for function");
            };
            assert_eq!(function.name(), $name);
            assert_params_eq!(function.inputs(), [$($inputs)*]);
            assert_params_eq!(function.outputs(), [$($outputs)*]);
        }
    };
}

#[test]
fn test_get_contracts_abi() {
    let unit = fixtures::Counter::build_compilation_unit();

    let contracts = unit.compute_contracts_abi();
    assert_eq!(1, contracts.len());
    assert_eq!("Counter", contracts[0].name());
    assert_eq!("main.sol", contracts[0].file_id());
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
    assert_eq!(entries.len(), 5);

    // f(S memory, T memory, uint) returns ()
    assert_function_eq!(
        &entries[0],
        "f",
        inputs: [
            (None, "tuple", [
                ("a", "uint256", []),
                ("b", "uint256[]", []),
                ("c", "tuple[]", [("x", "uint256", []), ("y", "uint256", [])]),
            ]),
            (None, "tuple", [("x", "uint256", []), ("y", "uint256", [])]),
            (None, "uint256", []),
        ],
        outputs: [],
    );

    // g() returns (S memory, T memory, uint)
    assert_function_eq!(
        &entries[1],
        "g",
        inputs: [],
        outputs: [
            (None, "tuple", [
                ("a", "uint256", []),
                ("b", "uint256[]", []),
                ("c", "tuple[]", [("x", "uint256", []), ("y", "uint256", [])]),
            ]),
            (None, "tuple", [("x", "uint256", []), ("y", "uint256", [])]),
            (None, "uint256", []),
        ],
    );

    // T public (getter) -> t() returns (uint256 x, uint256 y)
    assert_function_eq!(
        &entries[2],
        "t",
        inputs: [],
        outputs: [(_, "uint256", []), (_, "uint256", [])],
    );

    // t_components() returns (uint, uint)
    assert_function_eq!(
        &entries[3],
        "t_components",
        inputs: [],
        outputs: [(None, "uint256", []), (None, "uint256", [])],
    );

    // t_struct() returns (T memory)
    assert_function_eq!(
        &entries[4],
        "t_struct",
        inputs: [],
        outputs: [(None, "tuple", [("x", "uint256", []), ("y", "uint256", [])])],
    );
}
