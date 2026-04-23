use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::interner::Interner;
use slang_solidity_v2_ir::ir::{self, NodeId};

use crate::binder::{Binder, Scope, ScopeId};
use crate::built_ins::BuiltInsResolver;
use crate::context::SemanticFile;
use crate::types::TypeRegistry;

mod disambiguation;
mod resolution;
mod typing;
mod visitor;

/// This pass will find identifiers used as references, resolve them to the
/// appropriate definitions, and compute typing information for AST nodes
/// containing expressions and statements. Both these actions are co-dependant
/// and happen concurrently for each node, and their results are store in the
/// `Binder` instance.
pub fn run(
    files: &[impl SemanticFile],
    binder: &mut Binder,
    types: &mut TypeRegistry,
    language_version: LanguageVersion,
    interner: &Interner,
) {
    for file in files {
        Pass::visit_file(file, binder, types, language_version, interner);
    }
    // update definition->references reverse mapping
    binder.update_definitions_to_references_index();
}

struct ScopeFrame {
    // Scope associated with the node that created the stack frame. This is
    // solely used for integrity validation when popping the current frame.
    structural_scope_id: ScopeId,
    // Scope to use when resolving a symbol.
    lexical_scope_id: ScopeId,
}

struct Pass<'a> {
    language_version: LanguageVersion,
    scope_stack: Vec<ScopeFrame>,
    binder: &'a mut Binder,
    types: &'a mut TypeRegistry,
    interner: &'a Interner,
}

impl<'a> Pass<'a> {
    fn visit_file(
        file: &impl SemanticFile,
        binder: &'a mut Binder,
        types: &'a mut TypeRegistry,
        language_version: LanguageVersion,
        interner: &'a Interner,
    ) {
        let mut pass = Self {
            language_version,
            scope_stack: Vec::new(),
            binder,
            types,
            interner,
        };
        ir::visitor::accept_source_unit(file.ir_root(), &mut pass);
        assert!(pass.scope_stack.is_empty());
    }

    fn built_ins_resolver(&self) -> BuiltInsResolver<'_> {
        BuiltInsResolver::new(self.language_version, self.binder, self.types)
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

    fn is_in_modifier_scope(&self) -> bool {
        self.scope_stack.iter().any(|frame| {
            matches!(
                self.binder.get_scope_by_id(frame.structural_scope_id),
                Scope::Modifier(_)
            )
        })
    }

    fn current_contract_scope_id(&self) -> Option<ScopeId> {
        for ScopeFrame {
            structural_scope_id,
            ..
        } in self.scope_stack.iter().rev()
        {
            let scope = self.binder.get_scope_by_id(*structural_scope_id);
            if matches!(scope, Scope::Contract(_)) {
                return Some(*structural_scope_id);
            }
        }
        None
    }
}
