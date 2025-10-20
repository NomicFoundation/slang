use std::collections::HashMap;

use semver::Version;

use super::p3_linearise_contracts::Output as Input;
use crate::backend::binder::{Binder, Definition, Scope, ScopeId};
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::backend::types::{Type, TypeId, TypeRegistry};
use crate::compilation::CompilationUnit;
use crate::cst::NodeId;

mod resolution;
mod typing;
mod visitor;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
    pub types: TypeRegistry,
}

/// This pass will determine and assign typing information to all the
/// definitions collected in the earlier pass and as a result register most
/// types used in the compilation unit, by instantiating a `TypeRegistry`
/// object. Another important milestone that happens in this pass is collecting
/// active `using` directives and associating them to the relevant scopes (or
/// registering them gloablly).
/// Finally, public state variables will be assigned an equivalent getter
/// function type. This happens after the main typing pass to ensure all types
/// are already registered.
pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new(input.binder, compilation_unit.language_version());
    for source_unit in files.values() {
        pass.visit_file(source_unit);
    }
    for source_unit in files.values() {
        pass.visit_file_type_getters(source_unit);
    }
    let binder = pass.binder;
    let types = pass.types;

    Output {
        compilation_unit,
        files,
        binder,
        types,
    }
}

struct Pass {
    language_version: Version,
    scope_stack: Vec<ScopeId>,
    binder: Binder,
    types: TypeRegistry,
    current_receiver_type: Option<TypeId>,
}

impl Pass {
    fn new(binder: Binder, language_version: &Version) -> Self {
        Self {
            language_version: language_version.clone(),
            scope_stack: Vec::new(),
            binder,
            types: TypeRegistry::new(language_version.clone()),
            current_receiver_type: None,
        }
    }

    fn visit_file(&mut self, source_unit: &input_ir::SourceUnit) {
        assert!(self.current_receiver_type.is_none());
        assert!(self.scope_stack.is_empty());
        input_ir::visitor::accept_source_unit(source_unit, self);
        assert!(self.scope_stack.is_empty());
        assert!(self.current_receiver_type.is_none());
    }

    // This is a short second pass to compute and register the types of the
    // getter functions for public state variables. Computing the type of the
    // getter requires all struct fields to be typed already thus why this
    // cannot happen concurrently with the typing of the definitions in the main
    // pass.
    fn visit_file_type_getters(&mut self, source_unit: &input_ir::SourceUnit) {
        for source_unit_member in &source_unit.members {
            let input_ir::SourceUnitMember::ContractDefinition(contract_definition) =
                source_unit_member
            else {
                continue;
            };
            let receiver_type_id = self.types.register_type(Type::Contract {
                definition_id: contract_definition.node_id,
            });
            for contract_member in &contract_definition.members {
                let input_ir::ContractMember::StateVariableDefinition(state_var_definition) =
                    contract_member
                else {
                    continue;
                };
                if state_var_definition.attributes.iter().all(|attribute| {
                    !matches!(attribute, input_ir::StateVariableAttribute::PublicKeyword)
                }) {
                    continue;
                }

                let node_id = state_var_definition.node_id;
                let Some(type_id) = self.binder.node_typing(node_id).as_type_id() else {
                    continue;
                };

                let Some(getter_type_id) =
                    self.compute_getter_type(receiver_type_id, node_id, type_id)
                else {
                    continue;
                };
                let Definition::StateVariable(definition) = self.binder.get_definition_mut(node_id)
                else {
                    unreachable!("definition is not a state variable");
                };
                definition.getter_type_id = Some(getter_type_id);
            }
        }
    }

    fn enter_scope_for_node_id(&mut self, node_id: NodeId) {
        let scope_id = self.binder.scope_id_for_node_id(node_id).unwrap();
        self.scope_stack.push(scope_id);
    }

    fn leave_scope_for_node_id(&mut self, node_id: NodeId) {
        let Some(current_scope_id) = self.scope_stack.pop() else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(
            current_scope_id,
            self.binder.scope_id_for_node_id(node_id).unwrap()
        );
    }

    fn current_contract_or_file_scope_id(&self) -> ScopeId {
        for scope_id in self.scope_stack.iter().rev() {
            let scope = self.binder.get_scope_by_id(*scope_id);
            if matches!(scope, Scope::Contract(_) | Scope::File(_)) {
                return *scope_id;
            }
        }
        unreachable!("attempt to get the current contract scope without a contract or file scope in the stack");
    }
}
