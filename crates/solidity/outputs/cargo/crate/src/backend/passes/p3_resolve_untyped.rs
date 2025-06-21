use std::collections::HashMap;
use std::rc::Rc;

use super::p2_collect_definitions::Output as Input;
use crate::backend::binder::{
    Binder, ContractDefinition, Definition, ImportDefinition, InterfaceDefinition,
    LibraryDefinition, Reference, Scope,
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
    for (file_id, source_unit) in &files {
        pass.visit_file(file_id, source_unit);
    }
    let binder = pass.binder;

    Output {
        compilation_unit,
        files,
        binder,
    }
}

struct Pass<'a> {
    current_file_id: Option<&'a str>,
    scope_stack: Vec<NodeId>,
    binder: Binder,
}

impl<'a> Pass<'a> {
    fn new(binder: Binder) -> Self {
        Self {
            // TODO: remove current_file_id and use the scope_stack exclusively
            current_file_id: None,
            scope_stack: Vec::new(),
            binder,
        }
    }

    fn visit_file<'b: 'a>(&mut self, file_id: &'b str, source_unit: &input_ir::SourceUnit) {
        assert!(self.scope_stack.is_empty());
        assert!(self.current_file_id.is_none());

        self.current_file_id = Some(file_id);
        input_ir::visitor::accept_source_unit(source_unit, self);
        self.current_file_id = None;

        assert!(self.scope_stack.is_empty());
    }

    fn enter_scope(&mut self, scope_id: NodeId) {
        assert!(self.binder.lookup_scope(scope_id).is_some());
        self.scope_stack.push(scope_id);
    }

    fn leave_scope(&mut self, scope_id: NodeId) {
        let Some(current_scope_id) = self.scope_stack.pop() else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(current_scope_id, scope_id);
    }

    fn current_contract_or_file_scope(&self) -> &Scope {
        for scope_id in self.scope_stack.iter().rev() {
            let scope = self.binder.get_scope(*scope_id);
            if matches!(scope, Scope::Contract(_) | Scope::File(_)) {
                return scope;
            }
        }
        unreachable!("attempt to get the current contract scope without a contract or file scope in the stack");
    }

    fn current_file(&self) -> &str {
        self.current_file_id
            .expect("visiting SourceUnit without a current file being set")
    }

    // Resolves an IdentifierPath that should be a top-level contract-like type,
    // ie. a contract, a library or an interface. As such, it starts resolution
    // at the file scope level.
    fn resolve_top_level_identifier_path(&mut self, identifier_path: &input_ir::IdentifierPath) {
        // start resolution from the current file
        let mut resolution_file_id = Some(self.current_file());

        for identifier in identifier_path {
            let definition_id = resolution_file_id.and_then(|file_id| {
                self.binder
                    .resolve_single_in_file_scope(file_id, &identifier.unparse())
            });

            let reference = Reference {
                identifier: Rc::clone(identifier),
                definition_id,
            };
            self.binder.insert_reference(reference);

            // recurse into file scopes pointed by the resolved definition
            // to resolve the next identifier in the path
            resolution_file_id = definition_id
                .and_then(|node_id| self.binder.find_definition_by_id(node_id))
                .and_then(|definition| {
                    if let Definition::Import(ImportDefinition {
                        resolved_file_id, ..
                    }) = definition
                    {
                        resolved_file_id.as_deref()
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
        let mut scope_id = Some(self.current_contract_or_file_scope().node_id());

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
                    }) => resolved_file_id.as_ref().map(|resolved_file_id| {
                        self.binder.get_scope_by_file_id(resolved_file_id).node_id()
                    }),
                    Definition::Contract(ContractDefinition { node_id, .. })
                    | Definition::Interface(InterfaceDefinition { node_id, .. })
                    | Definition::Library(LibraryDefinition { node_id, .. }) => Some(*node_id),
                    _ => None,
                });
        }
    }
}

impl Visitor for Pass<'_> {
    fn enter_source_unit(&mut self, node: &input_ir::SourceUnit) -> bool {
        self.enter_scope(node.node_id);
        true
    }

    fn leave_source_unit(&mut self, node: &input_ir::SourceUnit) {
        self.leave_scope(node.node_id);
    }

    fn enter_contract_definition(&mut self, node: &input_ir::ContractDefinition) -> bool {
        self.resolve_inheritance_types(&node.inheritance_types);
        // TODO: save the resolved types as bases of the contract
        self.enter_scope(node.node_id);

        true
    }

    fn leave_contract_definition(&mut self, node: &input_ir::ContractDefinition) {
        self.leave_scope(node.node_id);
    }

    fn enter_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) -> bool {
        if let Some(inheritance) = &node.inheritance {
            self.resolve_inheritance_types(&inheritance.types);
            // TODO: save the resolved types as bases of the interface
        }
        self.enter_scope(node.node_id);

        true
    }

    fn leave_interface_definition(&mut self, node: &input_ir::InterfaceDefinition) {
        self.leave_scope(node.node_id);
    }

    fn enter_library_definition(&mut self, node: &input_ir::LibraryDefinition) -> bool {
        self.enter_scope(node.node_id);
        true
    }

    fn leave_library_definition(&mut self, node: &input_ir::LibraryDefinition) {
        self.leave_scope(node.node_id);
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
