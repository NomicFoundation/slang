use semver::Version;

use crate::backend::binder::{Binder, Definition, Scope, ScopeId};
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::backend::semantic::SemanticAnalysis;
use crate::backend::types::{Type, TypeId, TypeRegistry};
use crate::cst::NodeId;

mod evaluator;
mod resolution;
mod typing;
mod visitor;

/// This pass will determine and assign typing information to all the
/// definitions collected in the earlier pass and as a result register most
/// types used in the compilation unit, by instantiating a `TypeRegistry`
/// object. Another important milestone that happens in this pass is collecting
/// active `using` directives and associating them to the relevant scopes (or
/// registering them gloablly).
/// Finally, public state variables will be assigned an equivalent getter
/// function type. This happens after the main typing pass to ensure all types
/// are already registered.
pub fn run(semantic_analysis: &mut SemanticAnalysis) {
    let mut pass = Pass::new(
        semantic_analysis.language_version().clone(),
        &mut semantic_analysis.binder,
        &mut semantic_analysis.types,
    );
    for semantic_file in semantic_analysis.files.values() {
        pass.visit_file(semantic_file.ir_root());
    }
    for semantic_file in semantic_analysis.files.values() {
        pass.visit_file_type_getters(semantic_file.ir_root());
    }
}

struct Pass<'a> {
    language_version: Version,
    scope_stack: Vec<ScopeId>,
    binder: &'a mut Binder,
    types: &'a mut TypeRegistry,
    current_receiver_type: Option<TypeId>,
}

impl<'a> Pass<'a> {
    fn new(language_version: Version, binder: &'a mut Binder, types: &'a mut TypeRegistry) -> Self {
        Self {
            language_version,
            scope_stack: Vec::new(),
            binder,
            types,
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
                if !matches!(
                    state_var_definition.visibility,
                    input_ir::StateVariableVisibility::Public
                ) {
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
