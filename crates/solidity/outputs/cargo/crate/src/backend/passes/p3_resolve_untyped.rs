use std::collections::HashMap;
use std::rc::Rc;

use semver::Version;

use super::p2_collect_definitions::Output as Input;
use crate::backend::binder::{
    Binder, ContractDefinition, Definition, ImportDefinition, InterfaceDefinition,
    LibraryDefinition, Reference, Scope, ScopeId,
};
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::TypeRegistry;
use crate::compilation::CompilationUnit;
use crate::cst::NodeId;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
    pub types: TypeRegistry,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new(input.binder, compilation_unit.language_version());
    for source_unit in files.values() {
        pass.visit_file(source_unit);
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

const VERSION_0_5_0: Version = Version::new(0, 5, 0);

struct Pass {
    language_version: Version,
    scope_stack: Vec<ScopeId>,
    binder: Binder,
    types: TypeRegistry,
}

impl Pass {
    fn new(binder: Binder, language_version: &Version) -> Self {
        Self {
            language_version: language_version.clone(),
            scope_stack: Vec::new(),
            binder,
            types: TypeRegistry::default(),
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

    fn current_scope_id(&self) -> ScopeId {
        self.scope_stack.last().copied().unwrap()
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

    fn enter_function_definition(&mut self, node: &input_ir::FunctionDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_function_definition(&mut self, node: &input_ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_modifier_definition(&mut self, node: &input_ir::ModifierDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_constructor_definition(&mut self, node: &input_ir::ConstructorDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_constructor_definition(&mut self, node: &input_ir::ConstructorDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_fallback_function_definition(
        &mut self,
        node: &input_ir::FallbackFunctionDefinition,
    ) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_fallback_function_definition(&mut self, node: &input_ir::FallbackFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_receive_function_definition(
        &mut self,
        node: &input_ir::ReceiveFunctionDefinition,
    ) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_receive_function_definition(&mut self, node: &input_ir::ReceiveFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_unnamed_function_definition(
        &mut self,
        node: &input_ir::UnnamedFunctionDefinition,
    ) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_unnamed_function_definition(&mut self, node: &input_ir::UnnamedFunctionDefinition) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_block(&mut self, node: &input_ir::Block) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            self.enter_scope_for_node_id(node.node_id);
        }
        true
    }

    fn leave_block(&mut self, node: &input_ir::Block) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.node_id);
        }
    }

    fn enter_for_statement(&mut self, node: &input_ir::ForStatement) -> bool {
        if self.language_version >= VERSION_0_5_0 {
            self.enter_scope_for_node_id(node.node_id);
        }
        true
    }

    fn leave_for_statement(&mut self, node: &input_ir::ForStatement) {
        if self.language_version >= VERSION_0_5_0 {
            self.leave_scope_for_node_id(node.node_id);
        }
    }

    fn enter_yul_block(&mut self, node: &input_ir::YulBlock) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_yul_block(&mut self, node: &input_ir::YulBlock) {
        self.leave_scope_for_node_id(node.node_id);
    }

    fn enter_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) -> bool {
        self.enter_scope_for_node_id(node.node_id);
        true
    }

    fn leave_yul_function_definition(&mut self, node: &input_ir::YulFunctionDefinition) {
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

    fn enter_expression(&mut self, node: &input_ir::Expression) -> bool {
        if let input_ir::Expression::Identifier(identifier) = node {
            let scope_id = self.current_scope_id();
            // TODO: we cannot resolve function overloads yet, so that will
            // need to be paused; in any case, we cannot use resolve_single
            // here, as potentially there are multiple definitions to choose
            // from.
            // TODO: we also need to alter the scope id for resolution,
            // depending on the context
            let definition_id = self
                .binder
                .resolve_single_in_scope(scope_id, &identifier.unparse());
            let reference = Reference {
                identifier: Rc::clone(identifier),
                definition_id,
            };
            self.binder.insert_reference(reference);
        }
        true
    }

    fn enter_yul_path(&mut self, items: &input_ir::YulPath) -> bool {
        // resolve the first item in the path
        if let Some(identifier) = items.first() {
            let scope_id = self.current_scope_id();
            let definition_id = self
                .binder
                .resolve_single_in_scope(scope_id, &identifier.unparse());
            let reference = Reference {
                identifier: Rc::clone(identifier),
                definition_id,
            };
            self.binder.insert_reference(reference);
        }

        // TODO: resolve the rest of the items

        false
    }
}
