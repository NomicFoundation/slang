use std::sync::Arc;

use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use super::Pass;
use crate::binder::{Definition, Reference, Resolution, Scope};
use crate::context::SemanticFile;

impl<F: SemanticFile> Visitor for Pass<'_, F> {
    // -------------------------------------------------------------------------
    // Solidity scope scaffolding
    //
    // These handlers re-establish the enclosing Solidity scope stack (entering
    // scopes already created by p1) so that a `YulBlock` parents to the correct
    // Solidity scope and Yul identifiers can chain up into Solidity definitions.
    // They mirror the scope-management half of p5's visitor; they intentionally
    // perform none of p5's Solidity reference resolution or typing.

    fn enter_source_unit(&mut self, node: &ir::SourceUnit) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_source_unit(&mut self, node: &ir::SourceUnit) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_contract_definition(&mut self, node: &ir::ContractDefinition) -> bool {
        // Unlike p5 we don't need to split out inheritance types / storage
        // layout into the parent scope: those subtrees contain no Yul, so it's
        // harmless to walk them in the contract scope (matching p1's structure).
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_contract_definition(&mut self, node: &ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_interface_definition(&mut self, node: &ir::InterfaceDefinition) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_interface_definition(&mut self, node: &ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_library_definition(&mut self, node: &ir::LibraryDefinition) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_library_definition(&mut self, node: &ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_function_definition(&mut self, node: &ir::FunctionDefinition) -> bool {
        // Covers modifiers too: p1 keys both function and modifier scopes on the
        // function-definition node id.
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_function_definition(&mut self, node: &ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_block(&mut self, node: &ir::Block) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_block(&mut self, node: &ir::Block) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_for_statement(&mut self, node: &ir::ForStatement) -> bool {
        self.enter_scope_for_node_id(node.id());
        true
    }

    fn leave_for_statement(&mut self, node: &ir::ForStatement) {
        self.leave_scope_for_node_id(node.id());
    }

    fn leave_variable_declaration_statement(&mut self, node: &ir::VariableDeclarationStatement) {
        // Swap to the chained scope p1 created here, so a later assembly block
        // in the same Solidity block sees locals declared before it.
        self.replace_scope_for_node_id(node.id());
    }

    // -------------------------------------------------------------------------
    // Yul handling: collection + resolution in a single traversal.

    fn enter_yul_block(&mut self, node: &ir::YulBlock) -> bool {
        let scope = Scope::new_yul_block(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        // Yul function definitions are hoisted: their names are visible in the
        // entire enclosing block, even before their definition statement. Insert
        // them before descending so later references resolve.
        for statement in &node.statements {
            if let ir::YulStatement::YulFunctionDefinition(function) = statement {
                let definition = Definition::new_yul_function(function);
                self.insert_definition_in_current_scope(definition);
            }
        }

        true
    }

    fn leave_yul_block(&mut self, node: &ir::YulBlock) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_yul_function_definition(&mut self, node: &ir::YulFunctionDefinition) -> bool {
        // The function name definition was already inserted (hoisted) when
        // entering the enclosing Yul block.
        let scope = Scope::new_yul_function(node.id(), self.current_scope_id());
        let scope_id = self.enter_scope(scope);

        for parameter in &node.parameters {
            let definition = Definition::new_yul_parameter(parameter);
            self.insert_definition_in_scope(definition, scope_id);
        }
        if let Some(returns) = &node.returns {
            for parameter in returns {
                let definition = Definition::new_yul_variable(parameter);
                self.insert_definition_in_scope(definition, scope_id);
            }
        }

        true
    }

    fn leave_yul_function_definition(&mut self, node: &ir::YulFunctionDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_yul_variable_declaration_statement(
        &mut self,
        node: &ir::YulVariableDeclarationStatement,
    ) -> bool {
        for variable in &node.variables {
            let definition = Definition::new_yul_variable(variable);
            self.insert_definition_in_current_scope(definition);
        }

        // Descend into the children so references in the initializer value
        // (`let v := <value>`) get resolved. The declared variable identifiers
        // themselves are not `YulPath` nodes, so they create no references.
        //
        // Contrary to Solidity variable declarations that need chained scopes
        // to ensure they are only visible after their declaration, we don't
        // need that here because we're collecting definitions and resolving
        // references on the same pass.

        true
    }

    fn enter_yul_for_statement(&mut self, node: &ir::YulForStatement) -> bool {
        // Visit the initialization block first. Because collection and
        // resolution now happen in the same traversal, this both creates the
        // initialization scope and resolves references inside it.
        ir::visitor::accept_yul_block(&node.initialization, self);

        // Visit the rest of the children, but in the scope of the
        // initialization block such that condition/iterator/body link to it.
        self.enter_scope_for_node_id(node.initialization.id());
        ir::visitor::accept_yul_expression(&node.condition, self);
        ir::visitor::accept_yul_block(&node.iterator, self);
        ir::visitor::accept_yul_block(&node.body, self);
        self.leave_scope_for_node_id(node.initialization.id());

        // We already visited our children
        false
    }

    fn enter_yul_path(&mut self, items: &ir::YulPath) -> bool {
        if items.is_empty() {
            return false;
        }

        let scope_id = self.current_scope_id();
        let identifier = &items[0];
        let resolution = self.resolve_symbol_in_yul_scope(scope_id, identifier.unparse());
        let reference = Reference::new(Arc::clone(identifier), resolution.clone());
        self.binder.insert_reference(reference);

        if items.len() > 1 {
            let suffix = &items[1];
            let resolution = self.resolve_yul_suffix(suffix.unparse(), &resolution);
            let reference = Reference::new(Arc::clone(suffix), resolution);
            self.binder.insert_reference(reference);
        }

        let consumed_identifiers = 2;

        // any remaining identifiers cannot be resolved, but we still want to
        // emit a reference for each of them
        for identifier in items.iter().skip(consumed_identifiers) {
            self.binder.insert_reference(Reference::new(
                Arc::clone(identifier),
                Resolution::Unresolved,
            ));
        }

        false
    }
}
