use slang_solidity_v2_common::diagnostics::kinds::resolution::IdentifierRedeclaration;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, Scope, ScopeId};
use crate::context::SemanticFile;
use crate::passes::p1_collect_definitions::conflicts;
use crate::types::TypeRegistry;

mod resolution;
mod visitor;

/// This pass processes all Yul/`assembly` code. In a single traversal it both
/// collects Yul definitions and scopes (creating `YulBlockScope`/
/// `YulFunctionScope`, hoisting Yul function names) and resolves Yul identifier
/// references. Both can happen together because Yul functions are hoisted, and
/// Yul resolution needs neither type nor linearisation information for its own
/// definitions.
///
/// It runs after `p3_type_definitions`/`p4_compute_linearisations` because
/// `filter_overriden_definitions` consults function typing when a Yul
/// identifier resolves to an ambiguous set of Solidity functions.
///
/// Assembly blocks are isolated: Yul scopes only chain *up* into the enclosing
/// Solidity scopes (so Yul can reference Solidity declarations), but no Solidity
/// scope ever points into a Yul scope, and nothing outside an assembly block
/// references a Yul definition.
pub fn run(
    files: &[impl SemanticFile],
    binder: &mut Binder,
    types: &TypeRegistry,
    diagnostics: &mut DiagnosticCollection,
) {
    for file in files {
        Pass::visit_file(file, binder, types, diagnostics);
    }
    // Yul references may resolve to Solidity definitions, so rebuild the
    // definition->references reverse index to include them. This pass runs
    // after p5, whose own rebuild ran before any Yul reference existed.
    binder.update_definitions_to_references_index();
}

struct ScopeFrame {
    // Scope associated with the node that created the stack frame. This is
    // solely used for integrity validation when popping the current frame.
    structural_scope_id: ScopeId,
    // Scope to use when resolving a symbol.
    lexical_scope_id: ScopeId,
}

struct Pass<'a, F: SemanticFile> {
    current_file: &'a F,
    scope_stack: Vec<ScopeFrame>,
    binder: &'a mut Binder,
    types: &'a TypeRegistry,
    diagnostics: &'a mut DiagnosticCollection,
}

impl<'a, F: SemanticFile> Pass<'a, F> {
    fn visit_file(
        file: &'a F,
        binder: &'a mut Binder,
        types: &'a TypeRegistry,
        diagnostics: &'a mut DiagnosticCollection,
    ) {
        let mut pass = Self {
            current_file: file,
            scope_stack: Vec::new(),
            binder,
            types,
            diagnostics,
        };
        ir::visitor::accept_source_unit(file.ir_root(), &mut pass);
        assert!(pass.scope_stack.is_empty());
    }

    // Creates a brand-new scope (used for the Yul scopes this pass owns) and
    // pushes it onto the stack.
    fn enter_scope(&mut self, scope: Scope) -> ScopeId {
        let scope_id = self.binder.insert_scope(scope);
        self.scope_stack.push(ScopeFrame {
            structural_scope_id: scope_id,
            lexical_scope_id: scope_id,
        });
        scope_id
    }

    // Re-enters a scope already created in p1 (Solidity scopes, and the Yul
    // for-loop initialization block when revisited).
    fn enter_scope_for_node_id(&mut self, node_id: NodeId) {
        let scope_id = self.binder.scope_id_for_node_id(node_id).unwrap();
        self.scope_stack.push(ScopeFrame {
            structural_scope_id: scope_id,
            lexical_scope_id: scope_id,
        });
    }

    // Swaps the lexical scope of the current frame to the (already existing)
    // chained scope keyed on `node_id`, preserving the structural scope so the
    // pop-time integrity check still matches the enclosing block.
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
        let Some(ScopeFrame {
            lexical_scope_id, ..
        }) = self.scope_stack.last()
        else {
            unreachable!("empty scope stack");
        };
        *lexical_scope_id
    }

    fn insert_definition_in_current_scope(&mut self, definition: Definition) {
        self.insert_definition_in_scope(definition, self.current_scope_id());
    }

    // Registers `definition` under the given scope, first checking whether its
    // identifier collides with a pre-existing definition in that scope. If so,
    // an `IdentifierRedeclaration` diagnostic is emitted; the definition is
    // registered regardless, so references to it can still be resolved.
    fn insert_definition_in_scope(&mut self, definition: Definition, scope_id: ScopeId) {
        let symbol = definition.identifier().unparse();
        if conflicts::find_conflicting_definition(self.binder, scope_id, symbol, &definition)
            .is_some()
        {
            self.diagnostics.push(
                self.current_file.id().to_owned(),
                definition.identifier().range.clone(),
                IdentifierRedeclaration,
            );
        }
        self.binder.insert_definition_in_scope(definition, scope_id);
    }
}
