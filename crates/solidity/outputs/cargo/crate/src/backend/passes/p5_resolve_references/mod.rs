use std::collections::HashMap;

use semver::Version;

use super::p4_type_definitions::Output as Input;
use crate::backend::binder::{Binder, Scope, ScopeId};
use crate::backend::built_ins::BuiltInsResolver;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::TypeRegistry;
use crate::compilation::CompilationUnit;
use crate::cst::NodeId;

mod disambiguation;
mod resolution;
mod typing;
mod visitor;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
    pub types: TypeRegistry,
}

/// This pass will find identifiers used as references, resolve them to the
/// appropriate definitions, and compute typing information for AST nodes
/// containing expressions and statements. Both these actions are co-dependant
/// and happen concurrently for each node, and their results are store in the
/// `Binder` instance.
pub fn run(input: Input) -> Output {
    let files = input.files;
    let compilation_unit = input.compilation_unit;
    let mut pass = Pass::new(
        input.binder,
        input.types,
        compilation_unit.language_version(),
    );
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

struct ScopeFrame {
    // Scope associated with the node that created the stack frame. This is
    // solely used for integrity validation when popping the current frame.
    structural_scope_id: ScopeId,
    // Scope to use when resolving a symbol.
    lexical_scope_id: ScopeId,
}

struct Pass {
    language_version: Version,
    scope_stack: Vec<ScopeFrame>,
    binder: Binder,
    types: TypeRegistry,
}

impl Pass {
    fn new(binder: Binder, types: TypeRegistry, language_version: &Version) -> Self {
        Self {
            language_version: language_version.clone(),
            scope_stack: Vec::new(),
            binder,
            types,
        }
    }

    fn built_ins_resolver(&self) -> BuiltInsResolver<'_> {
        BuiltInsResolver::new(self.language_version.clone(), &self.binder, &self.types)
    }

    fn visit_file(&mut self, source_unit: &input_ir::SourceUnit) {
        assert!(self.scope_stack.is_empty());
        input_ir::visitor::accept_source_unit(source_unit, self);
        assert!(self.scope_stack.is_empty());
    }

    fn enter_scope_for_node_id(&mut self, node_id: NodeId) {
        let scope_id = self.binder.scope_id_for_node_id(node_id).unwrap();
        self.scope_stack.push(ScopeFrame {
            structural_scope_id: scope_id,
            lexical_scope_id: scope_id,
        });
    }

    fn replace_scope_for_node_id(&mut self, node_id: NodeId) {
        let Some(ScopeFrame {
            structural_scope_id,
            ..
        }) = self.scope_stack.pop()
        else {
            unreachable!("scope stack cannot be empty");
        };
        let scope_id = self.binder.scope_id_for_node_id(node_id).unwrap();
        self.scope_stack.push(ScopeFrame {
            structural_scope_id,
            lexical_scope_id: scope_id,
        });
    }

    fn leave_scope_for_node_id(&mut self, node_id: NodeId) {
        let Some(ScopeFrame {
            structural_scope_id,
            ..
        }) = self.scope_stack.pop()
        else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(
            structural_scope_id,
            self.binder.scope_id_for_node_id(node_id).unwrap()
        );
    }

    fn current_scope_id(&self) -> ScopeId {
        self.scope_stack
            .last()
            .map(
                |ScopeFrame {
                     lexical_scope_id, ..
                 }| lexical_scope_id,
            )
            .copied()
            .unwrap()
    }

    fn current_contract_scope_id(&self) -> Option<ScopeId> {
        for ScopeFrame {
            lexical_scope_id, ..
        } in self.scope_stack.iter().rev()
        {
            let scope = self.binder.get_scope_by_id(*lexical_scope_id);
            if matches!(scope, Scope::Contract(_)) {
                return Some(*lexical_scope_id);
            }
        }
        None
    }
}
