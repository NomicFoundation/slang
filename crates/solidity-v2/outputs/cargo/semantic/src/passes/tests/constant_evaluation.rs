//! Compile-time constant folding: folded literal values consumed as array
//! lengths and storage base slots.

use ruint::aliases::U256;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    ArrayLengthFractional, ArrayLengthNotConstant, ArrayLengthZero, ConstantArithmeticError,
    IncompatibleConstantOperator, StorageLayoutBaseNotConstant,
};
use slang_solidity_v2_ir::ir;

use super::support::{Analyse, Analysis, diagnostic_kind};
use crate::binder::Definition;
use crate::types::{FixedSizeArrayType, Type};

/// Runs the full pipeline over `source` and returns contract `name`'s folded
/// storage base slot together with the diagnostic emitted, if any. A rejected base
/// slot is reported as a diagnostic and leaves `base_slot` unset.
fn contract_base_slot(source: &str, name: &str) -> (Option<U256>, Option<DiagnosticKind>) {
    let source = format!("pragma solidity *;\n{source}");
    let analysis = Analysis::of_source(&source).run(Analyse::References);
    let binder = analysis.binder();
    let diagnostics = &analysis.diagnostics;
    let contract = analysis.find_contract(name);
    let base_slot = match binder
        .find_definition_by_id(contract.id())
        .expect("contract definition is registered")
    {
        Definition::Contract(contract_definition) => contract_definition.effective_base_slot,
        _ => panic!("expected a contract definition"),
    };
    (base_slot, diagnostic_kind(diagnostics))
}

/// Folds a fixed-size-array length through the real pipeline, returning the
/// computed `FixedSizeArrayType.size` together with the diagnostic emitted, if any.
/// `members` holds any contract-level constants the length references;
/// `array_type` is the variable type (e.g. `"uint256[10 / B]"`). A rejected
/// length reads back as `0`, same as a length that genuinely folds to `0`.
fn folded_array_length(members: &str, array_type: &str) -> (U256, Option<DiagnosticKind>) {
    let source = format!(
        r#"
        pragma solidity *;
        contract Test {{
            {members}
            {array_type} sized_array;
        }}
        "#
    );

    let analysis = Analysis::of_source(&source).run(Analyse::References);
    let binder = analysis.binder();
    let types = analysis.types();
    let diagnostics = &analysis.diagnostics;

    let contract = analysis.find_contract("Test");
    let state_variable = contract
        .members
        .iter()
        .find_map(|member| match member {
            ir::ContractMember::StateVariableDefinition(state_variable)
                if state_variable.name.unparse() == "sized_array" =>
            {
                Some(state_variable)
            }
            _ => None,
        })
        .expect("`sized_array` state variable not found");

    let type_id = binder
        .node_typing(state_variable.id())
        .as_type_id()
        .expect("state variable has a resolved type");
    let size = match types.get_type_by_id(type_id) {
        Type::FixedSizeArray(FixedSizeArrayType { size, .. }) => *size,
        other => panic!("expected a FixedSizeArray type, got {other:?}"),
    };
    (size, diagnostic_kind(diagnostics))
}

#[test]
fn test_array_length_folds_with_typed_constants() {
    let uint256_b = |value: &str| format!("uint256 constant B = {value};");

    // Division by a typed integer constant truncates toward zero (`10 / 3 = 3`).
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[10 / B]"),
        (U256::from(3), None)
    );
    // `7 / 3` truncates to `2`.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[7 / B]"),
        (U256::from(2), None)
    );
    // The fractional intermediate of `(1 / B) * B` is discarded at the typed
    // division, folding to `0` rather than `1`, which is then a zero length.
    assert_eq!(
        folded_array_length(&uint256_b("2"), "uint256[(1 / B) * B]"),
        (U256::ZERO, Some(ArrayLengthZero.into())),
    );
    // Whole literals combine with a typed integer fine: `2 * 7 / 2 = 7`.
    assert_eq!(
        folded_array_length(&uint256_b("2"), "uint256[(B * 7) / 2]"),
        (U256::from(7), None)
    );
    // Exponentiation with a typed base: `3 ** 2 = 9`.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[B ** 2]"),
        (U256::from(9), None)
    );

    // A small integer type widens the result to the literal's mobile type, so
    // `300 / B` (`B: uint8`) is `uint16` and folds to `100` instead of
    // overflowing `uint8`.
    assert_eq!(
        folded_array_length("uint8 constant B = 3;", "uint256[300 / B]"),
        (U256::from(100), None),
    );

    // But when no widening applies, an overflow of the result type is rejected:
    // `A + 255` (`A: uint8`) has common type `uint8` (255 fits `uint8`), and
    // `1 + 255 = 256` does not fit `uint8`, so it is an arithmetic overflow.
    assert_eq!(
        folded_array_length("uint8 constant A = 1;", "uint256[A + 255]"),
        (U256::ZERO, Some(ConstantArithmeticError.into())),
    );
}

#[test]
fn test_array_length_folds_all_literal_arithmetic() {
    // No typed constant: exact rational arithmetic, then the whole result feeds
    // the length. `100 / 8 * 2 = 25` (`100 / 8` is the exact `25/2`).
    assert_eq!(
        folded_array_length("", "uint256[(100 / 8) * 2]"),
        (U256::from(25), None)
    );
    assert_eq!(
        folded_array_length("", "uint256[2 ** 8]"),
        (U256::from(256), None)
    );
    // Lengths above the machine word are valid. The maximum is `2**256 - 1`.
    assert_eq!(
        folded_array_length("", "uint256[2 ** 64]"),
        (U256::from(1) << 64, None)
    );
}

#[test]
fn test_array_length_rejected_inputs_default_to_zero() {
    let uint256_b = |value: &str| format!("uint256 constant B = {value};");

    // A negative literal has no common type with an unsigned integer, so the
    // operator has no result type and is reported as incompatible.
    assert_eq!(
        folded_array_length(&uint256_b("2"), "uint256[(0 - 7) / B]"),
        (
            U256::ZERO,
            Some(
                IncompatibleConstantOperator {
                    operator: "/".to_owned(),
                    left_type: "int_const -7".to_owned(),
                    right_type: "uint256".to_owned(),
                }
                .into()
            ),
        ),
    );
    // `~B` over `uint256` folds to a negative value that overflows the
    // unsigned result type: an arithmetic error, not a plain non-constant
    // length.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[~B]"),
        (U256::ZERO, Some(ConstantArithmeticError.into())),
    );
    // Unary negation of an unsigned integer has no result type.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[-B]"),
        (U256::ZERO, Some(ArrayLengthNotConstant.into())),
    );
    // A literal exceeding 256 bits cannot meet a typed integer, so the
    // operator has no result type and is reported as incompatible.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[(2 ** 256) / B]"),
        (
            U256::ZERO,
            Some(
                IncompatibleConstantOperator {
                    operator: "/".to_owned(),
                    left_type: "int_const 1157...(70 digits omitted)...9936".to_owned(),
                    right_type: "uint256".to_owned(),
                }
                .into()
            ),
        ),
    );
    // All-literal division stays an exact rational, which is not a valid
    // (integer) length.
    assert_eq!(
        folded_array_length("", "uint256[10 / 4]"),
        (U256::ZERO, Some(ArrayLengthFractional.into())),
    );
}

#[test]
fn test_storage_base_slot_evaluation() {
    // The default when no specifier is given is 0.
    assert_eq!(
        contract_base_slot("contract C {}", "C"),
        (Some(U256::ZERO), None),
    );
    // A base slot referencing a constant declared *after* the contract resolves.
    assert_eq!(
        contract_base_slot("contract C layout at N {} uint256 constant N = 42;", "C"),
        (Some(U256::from(42)), None),
    );
    // Backward reference and a plain literal still resolve.
    assert_eq!(
        contract_base_slot("uint256 constant N = 42; contract C layout at N {}", "C"),
        (Some(U256::from(42)), None),
    );
    assert_eq!(
        contract_base_slot("contract C layout at 7 {}", "C"),
        (Some(U256::from(7)), None),
    );
    // A non-integer constant (here `address`) is not foldable to an integer, so
    // it is rejected as a non-constant base slot, even when forward-referenced.
    assert_eq!(
        contract_base_slot(
            "contract C layout at N {} address constant N = address(0);",
            "C",
        ),
        (None, Some(StorageLayoutBaseNotConstant.into())),
    );
}
