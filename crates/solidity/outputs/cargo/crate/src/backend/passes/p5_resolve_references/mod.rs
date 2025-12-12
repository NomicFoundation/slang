use semver::Version;

use crate::backend::binder::{Binder, Scope, ScopeId};
use crate::backend::built_ins::BuiltInsResolver;
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::backend::semantic::SemanticAnalysis;
use crate::backend::types::TypeRegistry;
use crate::cst::NodeId;

mod disambiguation;
mod resolution;
mod typing;
mod visitor;

/// This pass will find identifiers used as references, resolve them to the
/// appropriate definitions, and compute typing information for AST nodes
/// containing expressions and statements. Both these actions are co-dependant
/// and happen concurrently for each node, and their results are store in the
/// `Binder` instance.
pub fn run(semantic_analysis: &mut SemanticAnalysis) {
    let mut pass = Pass::new(
        semantic_analysis.language_version().clone(),
        &mut semantic_analysis.binder,
        &mut semantic_analysis.types,
    );
    for semantic_file in semantic_analysis.files.values() {
        pass.visit_file(semantic_file.ir_root());
    }
}

struct ScopeFrame {
    // Scope associated with the node that created the stack frame. This is
    // solely used for integrity validation when popping the current frame.
    structural_scope_id: ScopeId,
    // Scope to use when resolving a symbol.
    lexical_scope_id: ScopeId,
}

struct Pass<'a> {
    language_version: Version,
    scope_stack: Vec<ScopeFrame>,
    binder: &'a mut Binder,
    types: &'a mut TypeRegistry,
}

impl<'a> Pass<'a> {
    fn new(language_version: Version, binder: &'a mut Binder, types: &'a mut TypeRegistry) -> Self {
        Self {
            language_version,
            scope_stack: Vec::new(),
            binder,
            types,
        }
    }

    fn built_ins_resolver(&self) -> BuiltInsResolver<'_> {
        BuiltInsResolver::new(self.language_version.clone(), self.binder, self.types)
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
