use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, Scope, ScopeId};
use crate::context::{FileNodeMapper, SemanticFile};
use crate::types::{ContractType, LibraryType, Type, TypeId, TypeRegistry, UserMetaType};

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
pub fn run(
    files: &[impl SemanticFile],
    binder: &mut Binder,
    language_version: LanguageVersion,
    types: &mut TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    for file in files {
        Pass::visit_file(
            file,
            binder,
            language_version,
            types,
            file_node_mapper,
            diagnostics,
        );
    }
    for file in files {
        Pass::visit_file_type_getters(
            file,
            binder,
            language_version,
            types,
            file_node_mapper,
            diagnostics,
        );
    }
}

struct Pass<'a> {
    scope_stack: Vec<ScopeId>,
    binder: &'a mut Binder,
    language_version: LanguageVersion,
    types: &'a mut TypeRegistry,
    file_node_mapper: &'a FileNodeMapper,
    diagnostics: &'a mut DiagnosticCollection,
    current_receiver_type: Option<TypeId>,
    // Number of upcoming mapping type nodes that belong to a mapping value
    // chain already covered by the enclosing chain's parameter-name conflict
    // check, and so must be skipped rather than checked as their own chain root.
    nested_mappings_to_skip: usize,
}

impl<'a> Pass<'a> {
    /// Registers a [`Type::UserMetaType`] for the given definition node and
    /// records it as the node's typing.
    fn mark_user_meta_type_node(&mut self, node_id: NodeId) {
        let type_id = self.types.register_type(Type::UserMetaType(UserMetaType {
            definition_id: node_id,
        }));
        self.binder.set_node_type(node_id, Some(type_id));
    }

    fn visit_file(
        file: &'a impl SemanticFile,
        binder: &'a mut Binder,
        language_version: LanguageVersion,
        types: &'a mut TypeRegistry,
        file_node_mapper: &'a FileNodeMapper,
        diagnostics: &'a mut DiagnosticCollection,
    ) {
        let mut pass = Self {
            scope_stack: Vec::new(),
            binder,
            language_version,
            types,
            file_node_mapper,
            diagnostics,
            current_receiver_type: None,
            nested_mappings_to_skip: 0,
        };
        ir::visitor::accept_source_unit(file.ir_root(), &mut pass);
        assert!(pass.scope_stack.is_empty());
        assert!(pass.current_receiver_type.is_none());
    }

    // This is a short second pass to compute and register the types of the
    // getter functions for public state variables. Computing the type of the
    // getter requires all struct fields to be typed already thus why this
    // cannot happen concurrently with the typing of the definitions in the main
    // pass.
    fn visit_file_type_getters(
        file: &'a impl SemanticFile,
        binder: &'a mut Binder,
        language_version: LanguageVersion,
        types: &'a mut TypeRegistry,
        file_node_mapper: &'a FileNodeMapper,
        diagnostics: &'a mut DiagnosticCollection,
    ) {
        let mut pass = Self {
            scope_stack: Vec::new(),
            binder,
            language_version,
            types,
            file_node_mapper,
            diagnostics,
            current_receiver_type: None,
            nested_mappings_to_skip: 0,
        };
        pass.type_getters_from(file.ir_root());
    }

    fn type_getters_from(&mut self, source_unit: &ir::SourceUnit) {
        for source_unit_member in source_unit.members.iter() {
            let (receiver_type, members) = match source_unit_member {
                ir::SourceUnitMember::ContractDefinition(contract_definition) => (
                    Type::Contract(ContractType {
                        definition_id: contract_definition.id(),
                    }),
                    &contract_definition.members,
                ),
                ir::SourceUnitMember::LibraryDefinition(library_definition) => (
                    Type::Library(LibraryType {
                        definition_id: library_definition.id(),
                    }),
                    &library_definition.members,
                ),
                _ => continue,
            };
            let receiver_type_id = self.types.register_type(receiver_type);
            for member in members.iter() {
                let ir::ContractMember::StateVariableDefinition(state_var_definition) = member
                else {
                    continue;
                };
                if !matches!(
                    state_var_definition.attributes.visibility,
                    ir::StateVariableVisibility::Public
                ) {
                    continue;
                }

                let node_id = state_var_definition.id();
                let Some(type_id) = self.binder.node_typing(node_id).as_type_id() else {
                    continue;
                };

                let Some((getter_type_id, getter_member_ids)) =
                    self.compute_getter_type(receiver_type_id, node_id, type_id)
                else {
                    continue;
                };
                let Definition::StateVariable(definition) = self.binder.get_definition_mut(node_id)
                else {
                    unreachable!("definition is not a state variable");
                };
                definition.getter_type_id = Some(getter_type_id);
                definition.getter_member_ids = getter_member_ids;
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
        unreachable!(
            "attempt to get the current contract scope without a contract or file scope in the stack"
        );
    }
}
