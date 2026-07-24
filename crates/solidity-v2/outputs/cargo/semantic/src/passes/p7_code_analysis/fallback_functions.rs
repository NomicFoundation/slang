//! Shape checks for the special `fallback` function.
//!
//! A fallback's accepted state mutability and its accepted signatures are
//! properties of its resolved type, and a library may not declare one.
//! The checks run here, in the code-analysis pass, over the fully
//! resolved program.
//!
//! Note that the v2 grammar already rejects most malformed special functions
//! as syntax errors (eg. `internal`/`public` visibility, or `view`/`pure`
//! mutability on a `receive`). Only the forms that parse cleanly need a
//! semantic check here.

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::structure::LibraryFallbackFunction;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    FallbackFunctionMutability, FallbackFunctionSignature,
};
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, Typing};
use crate::context::FileNodeMapper;
use crate::types::{FunctionType, FunctionTypeMutability, Type, TypeRegistry};

/// Validates the shape of every `fallback` function in the program.
pub(crate) fn check_fallback_functions(
    binder: &Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    for definition in binder.definitions().values() {
        let (members, enclosing_is_library) = match definition {
            Definition::Contract(contract) => (&contract.ir_node.members, false),
            Definition::Interface(interface) => (&interface.ir_node.members, false),
            Definition::Library(library) => (&library.ir_node.members, true),
            _ => continue,
        };

        for member in members {
            let ir::ContractMember::FunctionDefinition(function) = member else {
                continue;
            };

            if matches!(function.kind, ir::FunctionKind::Fallback) {
                check_fallback_function(
                    function,
                    enclosing_is_library,
                    binder,
                    types,
                    file_node_mapper,
                    diagnostics,
                );
            }
        }
    }
}

/// Emits the shape diagnostics for a single `fallback` function:
///
/// * libraries cannot declare a fallback function;
/// * a fallback must be `payable` or non-payable (not `pure`/`view`); and
/// * its signature must be exactly `fallback()` or
///   `fallback(bytes calldata) returns (bytes memory)`.
///
/// The checks are independent, mirroring solc, so a single fallback can emit
/// more than one of them.
fn check_fallback_function(
    node: &ir::FunctionDefinition,
    enclosing_is_library: bool,
    binder: &Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let file_id = file_node_mapper.file_id_from_node_id(node.id()).to_owned();
    let signature_range = node.signature_text_range();

    // Libraries cannot declare a fallback function.
    if enclosing_is_library {
        diagnostics.push(
            file_id.clone(),
            signature_range.clone(),
            LibraryFallbackFunction,
        );
    }

    // The mutability and signature can be extracted from the function's type,
    // computed during type definition. A function definition always types to a
    // function type.
    let Typing::Resolved(type_id) = binder.node_typing(node.id()) else {
        unreachable!("fallback function definition is not typed");
    };
    let Type::Function(function_type) = types.get_type_by_id(type_id) else {
        unreachable!("type of function definition is not a function");
    };

    // A fallback must be `payable` or non-payable, not `pure`/`view`.
    if let Some(mutability) = match function_type.mutability {
        FunctionTypeMutability::Pure => Some("pure"),
        FunctionTypeMutability::View => Some("view"),
        FunctionTypeMutability::NonPayable | FunctionTypeMutability::Payable => None,
    } {
        diagnostics.push(
            file_id.clone(),
            signature_range.clone(),
            FallbackFunctionMutability {
                mutability: mutability.to_owned(),
            },
        );
    }

    // Its signature must be exactly `fallback()` or
    // `fallback(bytes calldata) returns (bytes memory)`.
    if !has_valid_signature(function_type, types) {
        diagnostics.push(file_id, signature_range, FallbackFunctionSignature);
    }
}

/// Whether `function_type`'s parameters and returns match one of the two
/// accepted fallback signatures: `fallback()` or
/// `fallback(bytes calldata) returns (bytes memory)`.
fn has_valid_signature(function_type: &FunctionType, types: &TypeRegistry) -> bool {
    match function_type.parameter_types.as_slice() {
        // `fallback()` — no parameters and no return values.
        [] => function_type.return_type == types.void(),
        // `fallback(bytes calldata) returns (bytes memory)`.
        [parameter_type_id] => {
            *parameter_type_id == types.bytes_calldata()
                && function_type.return_type == types.bytes_memory()
        }
        _ => false,
    }
}
