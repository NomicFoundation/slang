use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::structure::{
    BreakOutsideLoop, ContinueOutsideLoop, EmptyEnum, EmptyStruct, EmptyTupleComponent,
    EmptyTupleOnLhs, EnumWithTooManyMembers, FunctionNameMatchesContainer,
    ModifierBodyWithoutPlaceholder, NestedUncheckedBlock, PlaceholderInUncheckedBlock,
    RedefinedBuiltInError,
};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use super::Pass;
use crate::binder::{AssemblyBlock, Definition, Scope};
use crate::context::SemanticFile;

impl<F: SemanticFile> Pass<'_, F> {
    /// Walks the left hand side of an assignment, which is a write (l-value)
    /// position. An empty tuple `()` found here is not a valid assignment target
    /// and is flagged. The recursion follows tuple components only, mirroring how
    /// the write context propagates: l-value tuples are traversed here (and so
    /// never reach `enter_tuple_expression`, where they would be wrongly flagged
    /// for their legal omitted slots), while sub-expressions such as index or
    /// member accesses are read positions and are handed back to the regular
    /// traversal so any tuples inside them are still checked.
    fn visit_lvalue(&mut self, expression: &ir::Expression) {
        match expression {
            ir::Expression::TupleExpression(tuple) => {
                if tuple.is_empty_tuple() {
                    self.report(tuple, EmptyTupleOnLhs);
                    return;
                }

                for item in tuple.items.iter() {
                    if let Some(inner) = &item.expression {
                        self.visit_lvalue(inner);
                    }
                }
            }
            other => ir::visitor::accept_expression(other, self),
        }
    }
}

impl<F: SemanticFile> Visitor for Pass<'_, F> {
    fn enter_source_unit(&mut self, node: &ir::SourceUnit) -> bool {
        let scope = Scope::new_file(node.id(), self.current_file.id());
        self.enter_scope(scope);

        true
    }

    fn leave_source_unit(&mut self, node: &ir::SourceUnit) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_contract_definition(&mut self, node: &ir::ContractDefinition) -> bool {
        let definition = Definition::new_contract(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_contract_definition(&mut self, node: &ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_library_definition(&mut self, node: &ir::LibraryDefinition) -> bool {
        let definition = Definition::new_library(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_library_definition(&mut self, node: &ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_interface_definition(&mut self, node: &ir::InterfaceDefinition) -> bool {
        let definition = Definition::new_interface(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_interface_definition(&mut self, node: &ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_using_directive(&mut self, node: &ir::UsingDirective) -> bool {
        self.check_using_directive(node);

        true
    }

    fn enter_path_import(&mut self, node: &ir::PathImport) -> bool {
        let imported_file_id = self.resolve_import_path(node.id());

        if node.alias.is_some() {
            let definition = Definition::new_import(node, imported_file_id);
            self.insert_definition_in_current_scope(definition);
        } else if let Some(imported_file_id) = imported_file_id {
            self.current_file_scope()
                .add_default_import(imported_file_id, node.range.clone());
        }

        false
    }

    fn enter_import_deconstruction(&mut self, node: &ir::ImportDeconstruction) -> bool {
        let imported_file_id = self.resolve_import_path(node.id());

        for symbol in node.symbols.iter() {
            let definition = Definition::new_imported_symbol(
                symbol,
                symbol.name.unparse().to_owned(),
                imported_file_id.clone(),
            );
            self.insert_definition_in_current_scope(definition);
        }

        false
    }

    fn enter_function_definition(&mut self, node: &ir::FunctionDefinition) -> bool {
        match node.kind {
            ir::FunctionKind::Regular
            | ir::FunctionKind::Constructor
            | ir::FunctionKind::Fallback
            | ir::FunctionKind::Receive => {
                if matches!(node.kind, ir::FunctionKind::Constructor) {
                    self.check_constructor_attributes(node);
                } else {
                    self.check_function_attributes(node);
                }

                let parameters_scope_id = self.collect_parameters(&node.parameters);

                if let Some(name) = &node.name {
                    let definition = Definition::new_function(node, parameters_scope_id);

                    let enclosing_definition = self.enclosing_definition();
                    let enclosing_container_name = enclosing_definition
                        .filter(|enclosing_definition| {
                            matches!(
                                enclosing_definition,
                                Definition::Contract(_)
                                    | Definition::Interface(_)
                                    | Definition::Library(_)
                            )
                        })
                        .map(|definition| definition.identifier().unparse());

                    if enclosing_container_name
                        .is_some_and(|container_name| container_name == name.unparse())
                    {
                        self.report(node, FunctionNameMatchesContainer);

                        // Skip registering the function symbol in the current scope
                        // to avoid interference with resolution.
                        self.binder.insert_definition_no_scope(definition);
                    } else {
                        self.insert_definition_in_current_scope(definition);
                    }
                } else if matches!(node.kind, ir::FunctionKind::Constructor) {
                    // Register the constructor to resolve named parameters when
                    // constructing this contract
                    self.register_constructor(node, parameters_scope_id);
                }

                self.check_constructor_or_function_body(node);

                let function_scope =
                    Scope::new_function(node.id(), self.current_scope_id(), parameters_scope_id);
                let function_scope_id = self.enter_scope(function_scope);

                if let Some(returns) = &node.returns {
                    self.collect_named_parameters_into_scope(returns, function_scope_id);
                }
            }

            ir::FunctionKind::Modifier => {
                self.check_modifier_attributes(node);

                // An implemented modifier (i.e. one with a body) must contain a
                // placeholder statement (`_`). Start tracking whether one is
                // seen while its body is traversed.
                if node.body.is_some() {
                    self.modifier_placeholder_found = Some(false);
                }

                let definition = Definition::new_modifier(node);
                self.insert_definition_in_current_scope(definition);

                let modifier_scope = Scope::new_modifier(node.id(), self.current_scope_id());
                let modifier_scope_id = self.enter_scope(modifier_scope);
                self.collect_named_parameters_into_scope(&node.parameters, modifier_scope_id);
            }
        }
        true
    }

    fn leave_function_definition(&mut self, node: &ir::FunctionDefinition) {
        // If we were tracking an implemented modifier's body and no placeholder
        // statement (`_`) was found, flag it. `take()` also clears the state for
        // any non-modifier function (where it is already `None`).
        if self.modifier_placeholder_found.take() == Some(false) {
            self.report(node, ModifierBodyWithoutPlaceholder);
        }

        self.leave_scope_for_node_id(node.id());
    }

    fn enter_enum_definition(&mut self, node: &ir::EnumDefinition) -> bool {
        let definition = Definition::new_enum(node);
        self.insert_definition_in_current_scope(definition);

        // An enum must declare at least one member, and at most 256 (its values
        // must fit in a single byte).
        if node.members.is_empty() {
            self.report(node, EmptyEnum);
        } else if node.members.len() > 256 {
            self.report(node, EnumWithTooManyMembers);
        }

        let enum_scope = Scope::new_enum(node.id());
        let enum_scope_id = self.binder.insert_scope(enum_scope);
        for member in node.members.iter() {
            let definition = Definition::new_enum_member(member);
            self.insert_definition_in_scope(definition, enum_scope_id);
        }

        false
    }

    fn enter_struct_definition(&mut self, node: &ir::StructDefinition) -> bool {
        let definition = Definition::new_struct(node);
        self.insert_definition_in_current_scope(definition);

        // A struct must declare at least one member.
        if node.members.is_empty() {
            self.report(node, EmptyStruct);
        }

        let struct_scope = Scope::new_struct(node.id());
        let struct_scope_id = self.binder.insert_scope(struct_scope);
        for member in node.members.iter() {
            let definition = Definition::new_struct_member(member);
            self.insert_definition_in_scope(definition, struct_scope_id);
        }

        true
    }

    fn enter_error_definition(&mut self, node: &ir::ErrorDefinition) -> bool {
        // `Error` and `Panic` are built-in errors and cannot be re-defined.
        // Custom errors were introduced in 0.8.4; before that the error-tolerant
        // parser still yields an `ErrorDefinition` node, but it is already
        // flagged as invalid syntax for the version, so don't pile a semantic
        // diagnostic on top of it.
        if self.language_version >= LanguageVersion::V0_8_4
            && matches!(node.name.text.as_str(), "Error" | "Panic")
        {
            self.report(node.as_ref(), RedefinedBuiltInError);
        }

        let parameters_scope_id = self.collect_parameters(&node.parameters);
        let definition = Definition::new_error(node, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_event_definition(&mut self, node: &ir::EventDefinition) -> bool {
        self.check_event_indexed_parameters(node);

        let parameters_scope_id = self.collect_parameters(&node.parameters);
        let definition = Definition::new_event(node, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_state_variable_definition(&mut self, node: &ir::StateVariableDefinition) -> bool {
        let definition = Definition::new_state_variable(node, self.current_scope_id());
        self.insert_definition_in_current_scope(definition);

        self.check_state_variable_container(node);

        // there may be more definitions in the type of the state variable (eg.
        // key/value names in mappings)
        true
    }

    fn enter_constant_definition(&mut self, node: &ir::ConstantDefinition) -> bool {
        let definition = Definition::new_constant(node, self.current_scope_id());
        self.insert_definition_in_current_scope(definition);

        self.check_constant_container(node);

        false
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &ir::UserDefinedValueTypeDefinition,
    ) -> bool {
        let definition = Definition::new_user_defined_value_type(node);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn leave_variable_declaration_statement(&mut self, node: &ir::VariableDeclarationStatement) {
        // Open a new scope that replaces but is linked to the current one so
        // definitions declared here are only available for statements after
        // this one. This is a "chained" scope that continues the parent's
        // lexical scope, not a new lexical scope of its own.
        let scope = Scope::new_chained(node.id(), self.current_scope_id());
        self.replace_scope(scope);

        match &node.target {
            ir::VariableDeclarationTarget::SingleTypedDeclaration(single) => {
                let definition = Definition::new_variable(&single.declaration);
                self.insert_definition_in_current_scope(definition);
            }
            ir::VariableDeclarationTarget::MultiTypedDeclaration(multi) => {
                for element in multi.elements.iter() {
                    if let Some(member) = &element.member {
                        let definition = Definition::new_variable(member);
                        self.insert_definition_in_current_scope(definition);
                    }
                }
            }
        }
    }

    fn enter_block(&mut self, node: &ir::Block) -> bool {
        let scope = Scope::new_block(node.id(), self.current_scope_id());
        self.enter_scope(scope);
        true
    }

    fn leave_block(&mut self, node: &ir::Block) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_if_statement(&mut self, node: &ir::IfStatement) -> bool {
        self.check_control_flow_body(&node.body);
        if let Some(else_branch) = &node.else_branch {
            self.check_control_flow_body(else_branch);
        }
        true
    }

    fn enter_for_statement(&mut self, node: &ir::ForStatement) -> bool {
        self.check_control_flow_body(&node.body);

        // Open a new block here to hold declarations in the initialization
        // clause. This is a new lexical scope.
        let scope = Scope::new_block(node.id(), self.current_scope_id());
        self.enter_scope(scope);
        self.loop_depth += 1;
        true
    }

    fn leave_for_statement(&mut self, node: &ir::ForStatement) {
        self.loop_depth -= 1;
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_while_statement(&mut self, node: &ir::WhileStatement) -> bool {
        self.check_control_flow_body(&node.body);
        self.loop_depth += 1;
        true
    }

    fn leave_while_statement(&mut self, _node: &ir::WhileStatement) {
        self.loop_depth -= 1;
    }

    fn enter_do_while_statement(&mut self, node: &ir::DoWhileStatement) -> bool {
        self.check_control_flow_body(&node.body);
        self.loop_depth += 1;
        true
    }

    fn leave_do_while_statement(&mut self, _node: &ir::DoWhileStatement) {
        self.loop_depth -= 1;
    }

    fn enter_unchecked_block(&mut self, node: &ir::UncheckedBlock) -> bool {
        // An `unchecked` block cannot be nested inside another one.
        if self.unchecked_depth > 0 {
            self.report(node, NestedUncheckedBlock);
        }
        self.unchecked_depth += 1;
        true
    }

    fn leave_unchecked_block(&mut self, _node: &ir::UncheckedBlock) {
        self.unchecked_depth -= 1;
    }

    fn enter_break_statement(&mut self, node: &ir::BreakStatement) -> bool {
        // A `break` statement is only valid inside a `for`, `while` or
        // `do-while` loop.
        if self.loop_depth == 0 {
            self.report(node, BreakOutsideLoop);
        }
        true
    }

    fn enter_continue_statement(&mut self, node: &ir::ContinueStatement) -> bool {
        // A `continue` statement is only valid inside a `for`, `while` or
        // `do-while` loop.
        if self.loop_depth == 0 {
            self.report(node, ContinueOutsideLoop);
        }
        true
    }

    fn enter_expression_statement(&mut self, node: &ir::ExpressionStatement) -> bool {
        // A placeholder statement (`_`) parses as an expression statement whose
        // expression is the `_` identifier. It is only meaningful inside a
        // modifier body (possibly nested within control-flow statements).
        let is_placeholder = self.modifier_placeholder_found.is_some()
            && matches!(&node.expression, ir::Expression::Identifier(identifier) if identifier.unparse() == "_");

        if is_placeholder {
            // The placeholder counts as present regardless of where it appears,
            // but it cannot be used inside an `unchecked` block.
            self.modifier_placeholder_found = Some(true);
            if self.unchecked_depth > 0 {
                self.report(node, PlaceholderInUncheckedBlock);
            }
        }

        true
    }

    fn enter_assignment_expression(&mut self, node: &ir::AssignmentExpression) -> bool {
        // The left hand side of an assignment is a write (l-value) position, so
        // walk it specially (see `visit_lvalue`). The right hand side is a read
        // position, so hand it back to the normal traversal.
        self.visit_lvalue(&node.left_operand);
        ir::visitor::accept_expression(&node.right_operand, self);
        // We've driven recursion into both operands ourselves.
        false
    }

    fn enter_tuple_expression(&mut self, node: &ir::TupleExpression) -> bool {
        // Any tuple reaching here is in a read position: l-value tuples are
        // traversed by `visit_lvalue` and never recursed into here. A missing
        // component slot is not allowed in a read position. The empty tuple `()`
        // (a single empty component) is a valid value, so only tuples with more
        // than one component are considered.
        if node.items.len() > 1 && node.items.iter().any(|item| item.expression.is_none()) {
            self.report(node, EmptyTupleComponent);
        }

        true
    }

    fn leave_try_statement(&mut self, node: &ir::TryStatement) {
        if let Some(returns) = &node.returns {
            // Collect the parameters in the returns declaration of the try
            // statement and make them available in the body block.
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.id()).unwrap();
            self.collect_named_parameters_into_scope(returns, body_scope_id);
        }

        self.check_try_catch_clauses(node);
    }

    fn leave_catch_clause(&mut self, node: &ir::CatchClause) {
        if let Some(error) = &node.error {
            // Collect the parameters in the catch declaration and make them
            // available in the body block.
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.id()).unwrap();
            self.collect_named_parameters_into_scope(&error.parameters, body_scope_id);
        }
    }

    fn enter_mapping_type(&mut self, node: &ir::MappingType) -> bool {
        if node.key_type.name.is_some() {
            let definition = Definition::new_type_parameter(&node.key_type);
            self.binder.insert_definition_no_scope(definition);
        }
        if node.value_type.name.is_some() {
            let definition = Definition::new_type_parameter(&node.value_type);
            self.binder.insert_definition_no_scope(definition);
        }

        true
    }

    fn enter_function_type(&mut self, node: &ir::FunctionType) -> bool {
        for parameter in node.parameters.iter() {
            if parameter.name.is_some() {
                let definition = Definition::new_type_parameter(parameter);
                self.binder.insert_definition_no_scope(definition);
            }
        }
        if let Some(returns) = &node.returns {
            for parameter in returns.iter() {
                if parameter.name.is_some() {
                    let definition = Definition::new_type_parameter(parameter);
                    self.binder.insert_definition_no_scope(definition);
                }
            }
        }

        false
    }

    fn enter_assembly_statement(&mut self, node: &ir::AssemblyStatement) -> bool {
        // Record the assembly block (with the enclosing Solidity scope) so that
        // `p6_resolve_yul` can process only these branches instead of walking
        // the full IR tree, and so the backend has a per-block record of the
        // Solidity definitions it references (filled in by p6).
        self.binder.insert_assembly_block(AssemblyBlock {
            ir_node: Arc::clone(node),
            enclosing_scope_id: self.current_scope_id(),
            solidity_references: Vec::new(),
        });
        // Keep visiting the statement's label/flags; `enter_yul_block` below
        // still skips the Yul body.
        true
    }

    fn enter_yul_block(&mut self, _node: &ir::YulBlock) -> bool {
        // All Yul is collected and resolved in `p6_resolve_yul`, so there's
        // nothing to do here. Skip the assembly body entirely. (The enclosing
        // `AssemblyStatement`'s flags/label are still visited, since we don't
        // skip from `enter_assembly_statement`.)
        false
    }
}
