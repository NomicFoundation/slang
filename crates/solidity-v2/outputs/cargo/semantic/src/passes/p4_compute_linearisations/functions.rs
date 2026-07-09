//! Flattening the hierarchy's functions into the contract's linearised function
//! list, resolving overrides.

use std::cmp::Ordering;
use std::sync::Arc;

use slang_solidity_v2_ir::ir;

use super::Lineariser;
use crate::binder::Binder;
use crate::types::TypeRegistry;

impl Lineariser<'_> {
    /// Flattens the per-base function lists (gathered most-base-first) into the
    /// hierarchy's function list: most-derived-first, dropping a function once a
    /// more-derived one overrides it, then sorted by name.
    pub(super) fn linearise_functions(&self) -> Vec<ir::FunctionDefinition> {
        let (binder, types) = (self.binder, self.types);
        let mut functions: Vec<ir::FunctionDefinition> = Vec::new();
        for base_functions in self.functions_per_base.iter().rev() {
            for function in base_functions {
                let already_overridden = functions
                    .iter()
                    .any(|kept| function_overrides(binder, types, kept, function));
                if !already_overridden {
                    functions.push(Arc::clone(function));
                }
                // TODO(validation): if overriding, function must have the `override` specifier and the overriden functions must be marked `virtual`
                // TODO(validation): if overriding multiple ancestors, the function needs to specify the bases in a specifier
            }
        }
        functions.sort_by(|a, b| match (&a.name, &b.name) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => a.unparse().cmp(b.unparse()),
        });
        functions
    }
}

/// Whether `overriding` overrides `overridden`: they share a name (or are the
/// same kind of unnamed function) and their signatures are in an override
/// relationship.
fn function_overrides(
    binder: &Binder,
    types: &TypeRegistry,
    overriding: &ir::FunctionDefinition,
    overridden: &ir::FunctionDefinition,
) -> bool {
    let name_matches = match (&overriding.name, &overridden.name) {
        (None, None) => overriding.kind == overridden.kind,
        (Some(name), Some(other_name)) => name.unparse() == other_name.unparse(),
        _ => false,
    };
    if !name_matches {
        return false;
    }
    let overriding_type_id = binder.node_typing(overriding.id()).as_type_id();
    let overridden_type_id = binder.node_typing(overridden.id()).as_type_id();
    match (overriding_type_id, overridden_type_id) {
        (Some(overriding_type_id), Some(overridden_type_id)) => {
            types.type_id_is_function_and_overrides(overriding_type_id, overridden_type_id)
        }
        _ => false,
    }
    // TODO(validation) SDR[6]: check also that the function mutability is stricter than the overridden one's
}
