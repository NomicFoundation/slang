use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::resolution::IdentifierRedeclaration;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, Scope, ScopeId};
use crate::passes::common::conflicts;
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
pub fn run(binder: &mut Binder, types: &TypeRegistry, diagnostics: &mut DiagnosticCollection) {
    // The `assembly` blocks were collected in `p1_collect_definitions` (each
    // with its enclosing Solidity scope), so we only process those branches
    // instead of walking every file's full IR tree.
    //
    // Snapshot them first: processing a block borrows `binder` mutably
    // (inserting Yul scopes/definitions/references), so we can't hold a borrow
    // into `binder.assembly_blocks()` across the loop.
    let blocks: Vec<_> = binder
        .assembly_blocks()
        .values()
        .map(|block| {
            (
                Arc::clone(&block.ir_node),
                block.file_id.clone(),
                block.enclosing_scope_id,
            )
        })
        .collect();

    for (statement, file_id, enclosing_scope_id) in blocks {
        let solidity_references = Pass::visit_assembly_statement(
            binder,
            types,
            diagnostics,
            &statement,
            file_id,
            enclosing_scope_id,
        );
        binder.set_assembly_block_solidity_references(statement.id(), solidity_references);
    }
}

struct Pass<'a> {
    file_id: String,
    // We don't need to chain Yul scopes, so `ScopeId` is enough to track the scope stack
    scope_stack: Vec<ScopeId>,
    binder: &'a mut Binder,
    types: &'a TypeRegistry,
    diagnostics: &'a mut DiagnosticCollection,
    /// Distinct Solidity definitions referenced anywhere in the assembly block
    /// being processed, accumulated as its Yul paths are resolved.
    solidity_references: Vec<NodeId>,
}

impl<'a> Pass<'a> {
    /// Processes a single `assembly` block, returning the distinct Solidity
    /// definitions it references.
    fn visit_assembly_statement(
        binder: &'a mut Binder,
        types: &'a TypeRegistry,
        diagnostics: &'a mut DiagnosticCollection,
        statement: &ir::AssemblyStatement,
        file_id: String,
        enclosing_scope_id: ScopeId,
    ) -> Vec<NodeId> {
        let mut pass = Self {
            file_id,
            // Seed the stack with the enclosing Solidity scope (created in p1)
            // so the block's Yul scope parents correctly and Yul identifiers
            // chain up into the enclosing Solidity definitions.
            scope_stack: vec![enclosing_scope_id],
            binder,
            types,
            diagnostics,
            solidity_references: Vec::new(),
        };
        ir::visitor::accept_yul_block(&statement.body, &mut pass);
        // Only the seeded enclosing scope should remain.
        assert_eq!(pass.scope_stack.len(), 1);
        pass.solidity_references
    }

    // Creates a brand-new scope (used for the Yul scopes this pass owns) and
    // pushes it onto the stack.
    fn enter_scope(&mut self, scope: Scope) -> ScopeId {
        let scope_id = self.binder.insert_scope(scope);
        self.scope_stack.push(scope_id);
        scope_id
    }

    // Re-enters a scope already created in p1 (the Yul for-loop initialization
    // block when revisited).
    fn enter_scope_for_node_id(&mut self, node_id: NodeId) {
        let scope_id = self.binder.scope_id_for_node_id(node_id).unwrap();
        self.scope_stack.push(scope_id);
    }

    fn leave_scope_for_node_id(&mut self, node_id: NodeId) {
        let Some(scope_id) = self.scope_stack.pop() else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(scope_id, self.binder.scope_id_for_node_id(node_id).unwrap());
    }

    fn current_scope_id(&self) -> ScopeId {
        let Some(scope_id) = self.scope_stack.last() else {
            unreachable!("empty scope stack");
        };
        *scope_id
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
                self.file_id.clone(),
                definition.identifier().range.clone(),
                IdentifierRedeclaration,
            );
        }
        self.binder.insert_definition_in_scope(definition, scope_id);
    }
}
