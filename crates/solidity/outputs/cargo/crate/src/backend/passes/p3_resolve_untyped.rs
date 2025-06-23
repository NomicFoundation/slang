use std::collections::HashMap;
use std::rc::Rc;

use super::p2_collect_definitions::Output as Input;
use crate::backend::binder::{
    Binder, ContractDefinition, Definition, ImportDefinition, InterfaceDefinition,
    LibraryDefinition, Reference, Scope, ScopeId,
};
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::compilation::CompilationUnit;
use crate::cst::NodeId;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new(input.binder);
    for source_unit in files.values() {
        pass.visit_file(source_unit);
    }
    let binder = pass.binder;

    Output {
        compilation_unit,
        files,
        binder,
    }
}

struct Pass {
    scope_stack: Vec<ScopeId>,
    binder: Binder,
}

impl Pass {
    fn new(binder: Binder) -> Self {
        Self {
            scope_stack: Vec::new(),
            binder,
        }
    }

    fn visit_file(&mut self, source_unit: &input_ir::SourceUnit) {
        assert!(self.scope_stack.is_empty());
        input_ir::visitor::accept_source_unit(source_unit, self);
        assert!(self.scope_stack.is_empty());
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

    fn current_file_scope_id(&self) -> ScopeId {
        // we assume the file scope is the first scope in the stack
        let Some(scope_id) = self.scope_stack.first() else {
            unreachable!("attempt to get the current file scope with an empty scope stack");
        };
        let Scope::File(_) = self.binder.get_scope_by_id(*scope_id) else {
            unreachable!("top of the scope stack is not a file scope");
        };
        *scope_id
    }

    // Resolves an IdentifierPath that should be a top-level contract-like type,
    // ie. a contract, a library or an interface. As such, it starts resolution
    // at the file scope level.
    fn resolve_top_level_identifier_path(&mut self, identifier_path: &input_ir::IdentifierPath) {
        // start resolution from the current file
        let mut resolution_scope_id = Some(self.current_file_scope_id());

        for identifier in identifier_path {
            let definition_id = resolution_scope_id.and_then(|scope_id| {
                self.binder
                    .resolve_single_in_scope(scope_id, &identifier.unparse())
            });

            let reference = Reference {
                identifier: Rc::clone(identifier),
                definition_id,
            };
            self.binder.insert_reference(reference);

            // recurse into file scopes pointed by the resolved definition to
            // resolve the next identifier in the path
            resolution_scope_id = definition_id
                .and_then(|node_id| self.binder.find_definition_by_id(node_id))
                .and_then(|definition| {
                    if let Definition::Import(ImportDefinition {
                        resolved_file_id: Some(resolved_file_id),
                        ..
                    }) = definition
                    {
                        self.binder.scope_id_for_file_id(resolved_file_id)
                    } else {
                        None
                    }
                });
        }
    }

    fn resolve_inheritance_types(&mut self, types: &input_ir::InheritanceTypes) {
        for inheritance_type in types {
            self.resolve_top_level_identifier_path(&inheritance_type.type_name);
        }
        // TODO: return the resolved types (ie. the definition for the last
        // identifier in each path)?
    }

    // Resolves an IdentifierPath that works as type name. It starts resolution
    // at the "contract" scope level, or at the file level if there's no
    // contract scope open.
    fn resolve_type_identifier_path(&mut self, identifier_path: &input_ir::IdentifierPath) {
        // start resolution from the current contract (or file if there's no
        // contract scope open)
        let mut scope_id = Some(self.current_contract_or_file_scope_id());

        for identifier in identifier_path {
            let definition_id = scope_id.and_then(|scope_id| {
                self.binder
                    .resolve_single_in_scope(scope_id, &identifier.unparse())
            });

            let reference = Reference {
                identifier: Rc::clone(identifier),
                definition_id,
            };
            self.binder.insert_reference(reference);

            // recurse into file scopes pointed by the resolved definition
            // to resolve the next identifier in the path
            scope_id = definition_id
                .and_then(|node_id| self.binder.find_definition_by_id(node_id))
                .and_then(|definition| match definition {
                    Definition::Import(ImportDefinition {
                        resolved_file_id, ..
                    }) => resolved_file_id.as_ref().and_then(|resolved_file_id| {
                        self.binder.scope_id_for_file_id(resolved_file_id)
                    }),
                    Definition::Contract(ContractDefinition { node_id, .. })
                    | Definition::Interface(InterfaceDefinition { node_id, .. })
                    | Definition::Library(LibraryDefinition { node_id, .. }) => {
                        self.binder.scope_id_for_node_id(*node_id)
                    }
                    _ => None,
                });
        }
    }
}

impl Visitor for Pass {
    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_source_unit(&mut self, node: &input_ir::SourceUnit) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        self.resolve_inheritance_types(&node.inheritance_types);
        // TODO: save the resolved types as bases of the contract
        self.enter_scope_for_node_id(node.node_id);

        true
    }

    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        if let Some(inheritance) = &node.inheritance {
            self.resolve_inheritance_types(&inheritance.types);
            // TODO: save the resolved types as bases of the interface
        }
        self.enter_scope_for_node_id(node.node_id);

        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_type_name(&mut self, node: &input_ir::TypeName) -> bool {
        match node {
            input_ir::TypeName::IdentifierPath(identifier_path) => {
                self.resolve_type_identifier_path(identifier_path);
                false
            }
            _ => true,
        }
    }
}
