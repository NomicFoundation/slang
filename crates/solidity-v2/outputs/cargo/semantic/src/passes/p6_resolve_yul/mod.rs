use slang_solidity_v2_common::diagnostics::kinds::resolution::{
    BuiltInRedeclaration, ExternalDeclarationShadowing, IdentifierRedeclaration,
};
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{AssemblyBlock, Binder, Definition, Scope, ScopeId};
use crate::context::FileNodeMapper;
use crate::types::TypeRegistry;

mod conflicts;
mod resolution;
mod visitor;

use conflicts::YulConflict;

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
    binder: &mut Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    // The `assembly` blocks were collected in `p1_collect_definitions` (each
    // with its enclosing Solidity scope), so we only process those branches
    // instead of walking every file's full IR tree.
    //
    // Take ownership of the blocks out of the binder so we can mutate each one
    // in place (recording its referenced definitions) while still holding a
    // mutable borrow of the rest of the binder. They're returned afterwards.
    let mut assembly_blocks = binder.take_assembly_blocks();
    for block in assembly_blocks.values_mut() {
        Pass::visit_assembly_statement(binder, types, file_node_mapper, diagnostics, block);
    }
    binder.restore_assembly_blocks(assembly_blocks);
}

struct Pass<'a> {
    file_node_mapper: &'a FileNodeMapper,
    // We don't need to chain Yul scopes, so `ScopeId` is enough to track the scope stack
    scope_stack: Vec<ScopeId>,
    binder: &'a mut Binder,
    types: &'a TypeRegistry,
    diagnostics: &'a mut DiagnosticCollection,
    /// Distinct Solidity definitions referenced anywhere in the assembly block
    /// being processed, recorded in place as its Yul paths are resolved.
    solidity_references: &'a mut Vec<NodeId>,
}

impl<'a> Pass<'a> {
    /// Processes a single `assembly` block, recording the distinct Solidity
    /// definitions it references directly into the block.
    fn visit_assembly_statement(
        binder: &'a mut Binder,
        types: &'a TypeRegistry,
        file_node_mapper: &'a FileNodeMapper,
        diagnostics: &'a mut DiagnosticCollection,
        block: &'a mut AssemblyBlock,
    ) {
        // Split the borrow of the block: we traverse the (immutable) Yul body
        // while recording referenced definitions into its `solidity_references`.
        let AssemblyBlock {
            ir_node,
            enclosing_scope_id,
            solidity_references,
        } = block;
        let mut pass = Self {
            file_node_mapper,
            // Seed the stack with the enclosing Solidity scope (created in p1)
            // so the block's Yul scope parents correctly and Yul identifiers
            // chain up into the enclosing Solidity definitions.
            scope_stack: vec![*enclosing_scope_id],
            binder,
            types,
            diagnostics,
            solidity_references,
        };
        ir::visitor::accept_yul_block(&ir_node.body, &mut pass);
        // Only the seeded enclosing scope should remain.
        assert_eq!(pass.scope_stack.len(), 1);
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

    // This is always the bottom of the stack (ie. the initial scope) by
    // construction, since it's the enclosing scope of the assembly block being
    // processed
    fn current_solidity_scope_id(&self) -> ScopeId {
        self.scope_stack
            .first()
            .copied()
            .expect("empty scope stack")
    }

    fn insert_definition_in_current_scope(&mut self, definition: Definition) {
        self.insert_definition_in_scope(definition, self.current_scope_id());
    }

    // Registers `definition` under the given scope, reporting a diagnostic if
    // its identifier is not a valid Yul declaration. `find_conflicting_yul_definition`
    // determines the kind of conflict (see its docs); here we just map it to the
    // matching diagnostic. Three errors are possible:
    //
    // - the name reuses a reserved Yul built-in (e.g. `let add := 1`),
    // - it redeclares another definition in the same assembly block, or
    // - a Yul variable shadows a declaration visible from outside the block —
    //   a Solidity declaration or a built-in (e.g. `let msg := 1`).
    //
    // Either way the definition is still registered, so references to it can
    // still be resolved (except Yul built-ins which are always resolved to
    // built-ins anyways).
    fn insert_definition_in_scope(&mut self, definition: Definition, scope_id: ScopeId) {
        let symbol = definition.identifier().unparse();

        let conflict_kind: Option<DiagnosticKind> = match conflicts::find_conflicting_yul_definition(
            self.binder,
            scope_id,
            symbol,
            &definition,
        ) {
            Some(YulConflict::BuiltInRedeclaration) => Some(
                BuiltInRedeclaration {
                    name: symbol.to_owned(),
                }
                .into(),
            ),
            Some(YulConflict::Redeclaration) => Some(IdentifierRedeclaration.into()),
            Some(YulConflict::ExternalDeclarationShadowing) => {
                Some(ExternalDeclarationShadowing.into())
            }
            None => None,
        };

        if let Some(conflict_kind) = conflict_kind {
            let file_id = self
                .file_node_mapper
                .file_id_from_node_id(definition.identifier().id())
                .to_owned();
            self.diagnostics.push(
                file_id,
                definition.identifier().range.clone(),
                conflict_kind,
            );
        }

        self.binder.insert_definition_in_scope(definition, scope_id);
    }
}
