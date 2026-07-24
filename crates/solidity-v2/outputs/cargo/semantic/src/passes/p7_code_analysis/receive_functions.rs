//! Shape checks for the special `receive` function.
//!
//! A `receive` function may not declare parameters,
//! and a library may not declare one.
//!
//! Note that the v2 grammar already rejects most malformed special functions
//! as syntax errors (a `receive` with `internal`/`public` visibility, with
//! `view`/`pure` mutability, or with a `returns` clause). Only the forms that
//! parse cleanly need a semantic check here.

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::structure::LibraryReceiveFunction;
use slang_solidity_v2_common::diagnostics::kinds::type_system::ReceiveFunctionParameters;
use slang_solidity_v2_ir::ir::{self, TextRange};

use crate::binder::{Binder, Definition, Typing};
use crate::context::FileNodeMapper;
use crate::types::{Type, TypeRegistry};

/// Validates the shape of every `receive` function in the program.
pub(crate) fn check_receive_functions(
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

            if matches!(function.kind, ir::FunctionKind::Receive) {
                check_receive_function(
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

/// Emits the shape diagnostics for a single `receive` function:
///
/// * libraries cannot declare a receive function; and
/// * a receive function cannot take parameters.
///
/// The checks are independent, mirroring solc, so a single receive can emit
/// more than one of them.
fn check_receive_function(
    node: &ir::FunctionDefinition,
    enclosing_is_library: bool,
    binder: &Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let file_id = file_node_mapper.file_id_from_node_id(node.id()).to_owned();
    let signature_range = node.signature_text_range();

    // Libraries cannot declare a receive function.
    if enclosing_is_library {
        diagnostics.push(
            file_id.clone(),
            signature_range.clone(),
            LibraryReceiveFunction,
        );
    }

    let Typing::Resolved(type_id) = binder.node_typing(node.id()) else {
        unreachable!("receive function definition is not typed");
    };
    let Type::Function(function_type) = types.get_type_by_id(type_id) else {
        unreachable!("type of function definition is not a function");
    };

    // A receive function cannot take parameters.
    //
    // Note: It also cannot return values, but that is already rejected by the grammar.
    if !function_type.parameter_types.is_empty() {
        let parameters_range = node
            .parameters
            .calculate_text_range()
            .expect("Parameters are not empty");
        diagnostics.push(file_id, parameters_range, ReceiveFunctionParameters);
    }
}
