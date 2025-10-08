// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// Transformer from previous language implementation
#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use super::{input, nodes as output};

pub trait Transformer {
    //
    // Sequences:
    //

    fn transform_source_unit(&mut self, source: &input::SourceUnit) -> output::SourceUnit {
        let members = self.transform_source_unit_members(&source.members);

        Rc::new(output::SourceUnitStruct {
            node_id: source.node_id,
            members,
        })
    }

    fn transform_pragma_directive(
        &mut self,
        source: &input::PragmaDirective,
    ) -> output::PragmaDirective {
        let pragma = self.transform_pragma(&source.pragma);

        Rc::new(output::PragmaDirectiveStruct {
            node_id: source.node_id,
            pragma,
        })
    }

    fn transform_abicoder_pragma(
        &mut self,
        source: &input::AbicoderPragma,
    ) -> output::AbicoderPragma {
        let version = self.transform_abicoder_version(&source.version);

        Rc::new(output::AbicoderPragmaStruct {
            node_id: source.node_id,
            version,
        })
    }

    fn transform_experimental_pragma(
        &mut self,
        source: &input::ExperimentalPragma,
    ) -> output::ExperimentalPragma {
        let feature = self.transform_experimental_feature(&source.feature);

        Rc::new(output::ExperimentalPragmaStruct {
            node_id: source.node_id,
            feature,
        })
    }

    fn transform_version_pragma(&mut self, source: &input::VersionPragma) -> output::VersionPragma {
        let sets = self.transform_version_expression_sets(&source.sets);

        Rc::new(output::VersionPragmaStruct {
            node_id: source.node_id,
            sets,
        })
    }

    fn transform_version_range(&mut self, source: &input::VersionRange) -> output::VersionRange {
        let start = self.transform_version_literal(&source.start);
        let end = self.transform_version_literal(&source.end);

        Rc::new(output::VersionRangeStruct {
            node_id: source.node_id,
            start,
            end,
        })
    }

    fn transform_version_term(&mut self, source: &input::VersionTerm) -> output::VersionTerm {
        let operator = source
            .operator
            .as_ref()
            .map(|value| self.transform_version_operator(value));
        let literal = self.transform_version_literal(&source.literal);

        Rc::new(output::VersionTermStruct {
            node_id: source.node_id,
            operator,
            literal,
        })
    }

    fn transform_import_directive(
        &mut self,
        source: &input::ImportDirective,
    ) -> output::ImportDirective {
        let clause = self.transform_import_clause(&source.clause);

        Rc::new(output::ImportDirectiveStruct {
            node_id: source.node_id,
            clause,
        })
    }

    fn transform_path_import(&mut self, source: &input::PathImport) -> output::PathImport;

    fn transform_named_import(&mut self, source: &input::NamedImport) -> output::NamedImport;

    fn transform_import_deconstruction(
        &mut self,
        source: &input::ImportDeconstruction,
    ) -> output::ImportDeconstruction {
        let symbols = self.transform_import_deconstruction_symbols(&source.symbols);
        let path = self.transform_string_literal(&source.path);

        Rc::new(output::ImportDeconstructionStruct {
            node_id: source.node_id,
            symbols,
            path,
        })
    }

    fn transform_import_deconstruction_symbol(
        &mut self,
        source: &input::ImportDeconstructionSymbol,
    ) -> output::ImportDeconstructionSymbol;

    fn transform_using_directive(
        &mut self,
        source: &input::UsingDirective,
    ) -> output::UsingDirective {
        let clause = self.transform_using_clause(&source.clause);
        let target = self.transform_using_target(&source.target);
        let global_keyword = source.global_keyword.as_ref().map(Rc::clone);

        Rc::new(output::UsingDirectiveStruct {
            node_id: source.node_id,
            clause,
            target,
            global_keyword,
        })
    }

    fn transform_using_deconstruction(
        &mut self,
        source: &input::UsingDeconstruction,
    ) -> output::UsingDeconstruction {
        let symbols = self.transform_using_deconstruction_symbols(&source.symbols);

        Rc::new(output::UsingDeconstructionStruct {
            node_id: source.node_id,
            symbols,
        })
    }

    fn transform_using_deconstruction_symbol(
        &mut self,
        source: &input::UsingDeconstructionSymbol,
    ) -> output::UsingDeconstructionSymbol {
        let name = self.transform_identifier_path(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.transform_using_alias(value));

        Rc::new(output::UsingDeconstructionSymbolStruct {
            node_id: source.node_id,
            name,
            alias,
        })
    }

    fn transform_using_alias(&mut self, source: &input::UsingAlias) -> output::UsingAlias {
        let operator = self.transform_using_operator(&source.operator);

        Rc::new(output::UsingAliasStruct {
            node_id: source.node_id,
            operator,
        })
    }

    fn transform_contract_definition(
        &mut self,
        source: &input::ContractDefinition,
    ) -> output::ContractDefinition;

    fn transform_inheritance_specifier(
        &mut self,
        source: &input::InheritanceSpecifier,
    ) -> output::InheritanceSpecifier {
        let types = self.transform_inheritance_types(&source.types);

        Rc::new(output::InheritanceSpecifierStruct {
            node_id: source.node_id,
            types,
        })
    }

    fn transform_inheritance_type(
        &mut self,
        source: &input::InheritanceType,
    ) -> output::InheritanceType {
        let type_name = self.transform_identifier_path(&source.type_name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.transform_arguments_declaration(value));

        Rc::new(output::InheritanceTypeStruct {
            node_id: source.node_id,
            type_name,
            arguments,
        })
    }

    fn transform_storage_layout_specifier(
        &mut self,
        source: &input::StorageLayoutSpecifier,
    ) -> output::StorageLayoutSpecifier {
        let expression = self.transform_expression(&source.expression);

        Rc::new(output::StorageLayoutSpecifierStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn transform_interface_definition(
        &mut self,
        source: &input::InterfaceDefinition,
    ) -> output::InterfaceDefinition {
        let name = Rc::clone(&source.name);
        let inheritance = source
            .inheritance
            .as_ref()
            .map(|value| self.transform_inheritance_specifier(value));
        let members = self.transform_interface_members(&source.members);

        Rc::new(output::InterfaceDefinitionStruct {
            node_id: source.node_id,
            name,
            inheritance,
            members,
        })
    }

    fn transform_library_definition(
        &mut self,
        source: &input::LibraryDefinition,
    ) -> output::LibraryDefinition {
        let name = Rc::clone(&source.name);
        let members = self.transform_library_members(&source.members);

        Rc::new(output::LibraryDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn transform_struct_definition(
        &mut self,
        source: &input::StructDefinition,
    ) -> output::StructDefinition {
        let name = Rc::clone(&source.name);
        let members = self.transform_struct_members(&source.members);

        Rc::new(output::StructDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn transform_struct_member(&mut self, source: &input::StructMember) -> output::StructMember {
        let type_name = self.transform_type_name(&source.type_name);
        let name = Rc::clone(&source.name);

        Rc::new(output::StructMemberStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn transform_enum_definition(
        &mut self,
        source: &input::EnumDefinition,
    ) -> output::EnumDefinition {
        let name = Rc::clone(&source.name);
        let members = self.transform_enum_members(&source.members);

        Rc::new(output::EnumDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn transform_constant_definition(
        &mut self,
        source: &input::ConstantDefinition,
    ) -> output::ConstantDefinition {
        let type_name = self.transform_type_name(&source.type_name);
        let name = Rc::clone(&source.name);
        let value = self.transform_expression(&source.value);

        Rc::new(output::ConstantDefinitionStruct {
            node_id: source.node_id,
            type_name,
            name,
            value,
        })
    }

    fn transform_state_variable_definition(
        &mut self,
        source: &input::StateVariableDefinition,
    ) -> output::StateVariableDefinition {
        let type_name = self.transform_type_name(&source.type_name);
        let attributes = self.transform_state_variable_attributes(&source.attributes);
        let name = Rc::clone(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.transform_state_variable_definition_value(value));

        Rc::new(output::StateVariableDefinitionStruct {
            node_id: source.node_id,
            type_name,
            attributes,
            name,
            value,
        })
    }

    fn transform_state_variable_definition_value(
        &mut self,
        source: &input::StateVariableDefinitionValue,
    ) -> output::StateVariableDefinitionValue {
        let value = self.transform_expression(&source.value);

        Rc::new(output::StateVariableDefinitionValueStruct {
            node_id: source.node_id,
            value,
        })
    }

    fn transform_function_definition(
        &mut self,
        source: &input::FunctionDefinition,
    ) -> output::FunctionDefinition;

    fn transform_parameter(&mut self, source: &input::Parameter) -> output::Parameter {
        let type_name = self.transform_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.transform_storage_location(value));
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::ParameterStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
        })
    }

    fn transform_override_specifier(
        &mut self,
        source: &input::OverrideSpecifier,
    ) -> output::OverrideSpecifier {
        let overridden = source
            .overridden
            .as_ref()
            .map(|value| self.transform_override_paths_declaration(value));

        Rc::new(output::OverrideSpecifierStruct {
            node_id: source.node_id,
            overridden,
        })
    }

    fn transform_override_paths_declaration(
        &mut self,
        source: &input::OverridePathsDeclaration,
    ) -> output::OverridePathsDeclaration {
        let paths = self.transform_override_paths(&source.paths);

        Rc::new(output::OverridePathsDeclarationStruct {
            node_id: source.node_id,
            paths,
        })
    }

    fn transform_modifier_invocation(
        &mut self,
        source: &input::ModifierInvocation,
    ) -> output::ModifierInvocation {
        let name = self.transform_identifier_path(&source.name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.transform_arguments_declaration(value));

        Rc::new(output::ModifierInvocationStruct {
            node_id: source.node_id,
            name,
            arguments,
        })
    }

    fn transform_event_definition(
        &mut self,
        source: &input::EventDefinition,
    ) -> output::EventDefinition {
        let name = Rc::clone(&source.name);
        let parameters = self.transform_event_parameters_declaration(&source.parameters);
        let anonymous_keyword = source.anonymous_keyword.as_ref().map(Rc::clone);

        Rc::new(output::EventDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            anonymous_keyword,
        })
    }

    fn transform_event_parameters_declaration(
        &mut self,
        source: &input::EventParametersDeclaration,
    ) -> output::EventParametersDeclaration {
        let parameters = self.transform_event_parameters(&source.parameters);

        Rc::new(output::EventParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn transform_event_parameter(
        &mut self,
        source: &input::EventParameter,
    ) -> output::EventParameter {
        let type_name = self.transform_type_name(&source.type_name);
        let indexed_keyword = source.indexed_keyword.as_ref().map(Rc::clone);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::EventParameterStruct {
            node_id: source.node_id,
            type_name,
            indexed_keyword,
            name,
        })
    }

    fn transform_user_defined_value_type_definition(
        &mut self,
        source: &input::UserDefinedValueTypeDefinition,
    ) -> output::UserDefinedValueTypeDefinition {
        let name = Rc::clone(&source.name);
        let value_type = self.transform_elementary_type(&source.value_type);

        Rc::new(output::UserDefinedValueTypeDefinitionStruct {
            node_id: source.node_id,
            name,
            value_type,
        })
    }

    fn transform_error_definition(
        &mut self,
        source: &input::ErrorDefinition,
    ) -> output::ErrorDefinition {
        let name = Rc::clone(&source.name);
        let members = self.transform_error_parameters_declaration(&source.members);

        Rc::new(output::ErrorDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn transform_error_parameters_declaration(
        &mut self,
        source: &input::ErrorParametersDeclaration,
    ) -> output::ErrorParametersDeclaration {
        let parameters = self.transform_error_parameters(&source.parameters);

        Rc::new(output::ErrorParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn transform_error_parameter(
        &mut self,
        source: &input::ErrorParameter,
    ) -> output::ErrorParameter {
        let type_name = self.transform_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::ErrorParameterStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn transform_array_type_name(
        &mut self,
        source: &input::ArrayTypeName,
    ) -> output::ArrayTypeName {
        let operand = self.transform_type_name(&source.operand);
        let index = source
            .index
            .as_ref()
            .map(|value| self.transform_expression(value));

        Rc::new(output::ArrayTypeNameStruct {
            node_id: source.node_id,
            operand,
            index,
        })
    }

    fn transform_function_type(&mut self, source: &input::FunctionType) -> output::FunctionType;

    fn transform_mapping_type(&mut self, source: &input::MappingType) -> output::MappingType {
        let key_type = self.transform_mapping_key(&source.key_type);
        let value_type = self.transform_mapping_value(&source.value_type);

        Rc::new(output::MappingTypeStruct {
            node_id: source.node_id,
            key_type,
            value_type,
        })
    }

    fn transform_mapping_key(&mut self, source: &input::MappingKey) -> output::MappingKey {
        let key_type = self.transform_mapping_key_type(&source.key_type);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::MappingKeyStruct {
            node_id: source.node_id,
            key_type,
            name,
        })
    }

    fn transform_mapping_value(&mut self, source: &input::MappingValue) -> output::MappingValue {
        let type_name = self.transform_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::MappingValueStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn transform_address_type(&mut self, source: &input::AddressType) -> output::AddressType {
        let payable_keyword = source.payable_keyword.as_ref().map(Rc::clone);

        Rc::new(output::AddressTypeStruct {
            node_id: source.node_id,
            payable_keyword,
        })
    }

    fn transform_block(&mut self, source: &input::Block) -> output::Block {
        let statements = self.transform_statements(&source.statements);

        Rc::new(output::BlockStruct {
            node_id: source.node_id,
            statements,
        })
    }

    fn transform_unchecked_block(
        &mut self,
        source: &input::UncheckedBlock,
    ) -> output::UncheckedBlock {
        let block = self.transform_block(&source.block);

        Rc::new(output::UncheckedBlockStruct {
            node_id: source.node_id,
            block,
        })
    }

    fn transform_expression_statement(
        &mut self,
        source: &input::ExpressionStatement,
    ) -> output::ExpressionStatement {
        let expression = self.transform_expression(&source.expression);

        Rc::new(output::ExpressionStatementStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn transform_assembly_statement(
        &mut self,
        source: &input::AssemblyStatement,
    ) -> output::AssemblyStatement {
        let label = source
            .label
            .as_ref()
            .map(|value| self.transform_string_literal(value));
        let flags = source
            .flags
            .as_ref()
            .map(|value| self.transform_assembly_flags_declaration(value));
        let body = self.transform_yul_block(&source.body);

        Rc::new(output::AssemblyStatementStruct {
            node_id: source.node_id,
            label,
            flags,
            body,
        })
    }

    fn transform_assembly_flags_declaration(
        &mut self,
        source: &input::AssemblyFlagsDeclaration,
    ) -> output::AssemblyFlagsDeclaration {
        let flags = self.transform_assembly_flags(&source.flags);

        Rc::new(output::AssemblyFlagsDeclarationStruct {
            node_id: source.node_id,
            flags,
        })
    }

    fn transform_tuple_deconstruction_statement(
        &mut self,
        source: &input::TupleDeconstructionStatement,
    ) -> output::TupleDeconstructionStatement {
        let var_keyword = source.var_keyword.as_ref().map(Rc::clone);
        let elements = self.transform_tuple_deconstruction_elements(&source.elements);
        let expression = self.transform_expression(&source.expression);

        Rc::new(output::TupleDeconstructionStatementStruct {
            node_id: source.node_id,
            var_keyword,
            elements,
            expression,
        })
    }

    fn transform_tuple_deconstruction_element(
        &mut self,
        source: &input::TupleDeconstructionElement,
    ) -> output::TupleDeconstructionElement {
        let member = source
            .member
            .as_ref()
            .map(|value| self.transform_tuple_member(value));

        Rc::new(output::TupleDeconstructionElementStruct {
            node_id: source.node_id,
            member,
        })
    }

    fn transform_typed_tuple_member(
        &mut self,
        source: &input::TypedTupleMember,
    ) -> output::TypedTupleMember {
        let type_name = self.transform_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.transform_storage_location(value));
        let name = Rc::clone(&source.name);

        Rc::new(output::TypedTupleMemberStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
        })
    }

    fn transform_untyped_tuple_member(
        &mut self,
        source: &input::UntypedTupleMember,
    ) -> output::UntypedTupleMember {
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.transform_storage_location(value));
        let name = Rc::clone(&source.name);

        Rc::new(output::UntypedTupleMemberStruct {
            node_id: source.node_id,
            storage_location,
            name,
        })
    }

    fn transform_variable_declaration_statement(
        &mut self,
        source: &input::VariableDeclarationStatement,
    ) -> output::VariableDeclarationStatement {
        let variable_type = self.transform_variable_declaration_type(&source.variable_type);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.transform_storage_location(value));
        let name = Rc::clone(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.transform_variable_declaration_value(value));

        Rc::new(output::VariableDeclarationStatementStruct {
            node_id: source.node_id,
            variable_type,
            storage_location,
            name,
            value,
        })
    }

    fn transform_variable_declaration_value(
        &mut self,
        source: &input::VariableDeclarationValue,
    ) -> output::VariableDeclarationValue {
        let expression = self.transform_expression(&source.expression);

        Rc::new(output::VariableDeclarationValueStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn transform_if_statement(&mut self, source: &input::IfStatement) -> output::IfStatement {
        let condition = self.transform_expression(&source.condition);
        let body = self.transform_statement(&source.body);
        let else_branch = source
            .else_branch
            .as_ref()
            .map(|value| self.transform_else_branch(value));

        Rc::new(output::IfStatementStruct {
            node_id: source.node_id,
            condition,
            body,
            else_branch,
        })
    }

    fn transform_else_branch(&mut self, source: &input::ElseBranch) -> output::ElseBranch {
        let body = self.transform_statement(&source.body);

        Rc::new(output::ElseBranchStruct {
            node_id: source.node_id,
            body,
        })
    }

    fn transform_for_statement(&mut self, source: &input::ForStatement) -> output::ForStatement {
        let initialization = self.transform_for_statement_initialization(&source.initialization);
        let condition = self.transform_for_statement_condition(&source.condition);
        let iterator = source
            .iterator
            .as_ref()
            .map(|value| self.transform_expression(value));
        let body = self.transform_statement(&source.body);

        Rc::new(output::ForStatementStruct {
            node_id: source.node_id,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    fn transform_while_statement(
        &mut self,
        source: &input::WhileStatement,
    ) -> output::WhileStatement {
        let condition = self.transform_expression(&source.condition);
        let body = self.transform_statement(&source.body);

        Rc::new(output::WhileStatementStruct {
            node_id: source.node_id,
            condition,
            body,
        })
    }

    fn transform_do_while_statement(
        &mut self,
        source: &input::DoWhileStatement,
    ) -> output::DoWhileStatement {
        let body = self.transform_statement(&source.body);
        let condition = self.transform_expression(&source.condition);

        Rc::new(output::DoWhileStatementStruct {
            node_id: source.node_id,
            body,
            condition,
        })
    }

    fn transform_continue_statement(
        &mut self,
        source: &input::ContinueStatement,
    ) -> output::ContinueStatement {
        Rc::new(output::ContinueStatementStruct {
            node_id: source.node_id,
        })
    }

    fn transform_break_statement(
        &mut self,
        source: &input::BreakStatement,
    ) -> output::BreakStatement {
        Rc::new(output::BreakStatementStruct {
            node_id: source.node_id,
        })
    }

    fn transform_return_statement(
        &mut self,
        source: &input::ReturnStatement,
    ) -> output::ReturnStatement {
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.transform_expression(value));

        Rc::new(output::ReturnStatementStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn transform_emit_statement(&mut self, source: &input::EmitStatement) -> output::EmitStatement {
        let event = self.transform_identifier_path(&source.event);
        let arguments = self.transform_arguments_declaration(&source.arguments);

        Rc::new(output::EmitStatementStruct {
            node_id: source.node_id,
            event,
            arguments,
        })
    }

    fn transform_try_statement(&mut self, source: &input::TryStatement) -> output::TryStatement;

    fn transform_catch_clause(&mut self, source: &input::CatchClause) -> output::CatchClause {
        let error = source
            .error
            .as_ref()
            .map(|value| self.transform_catch_clause_error(value));
        let body = self.transform_block(&source.body);

        Rc::new(output::CatchClauseStruct {
            node_id: source.node_id,
            error,
            body,
        })
    }

    fn transform_catch_clause_error(
        &mut self,
        source: &input::CatchClauseError,
    ) -> output::CatchClauseError;

    fn transform_revert_statement(
        &mut self,
        source: &input::RevertStatement,
    ) -> output::RevertStatement {
        let error = source
            .error
            .as_ref()
            .map(|value| self.transform_identifier_path(value));
        let arguments = self.transform_arguments_declaration(&source.arguments);

        Rc::new(output::RevertStatementStruct {
            node_id: source.node_id,
            error,
            arguments,
        })
    }

    fn transform_throw_statement(
        &mut self,
        source: &input::ThrowStatement,
    ) -> output::ThrowStatement {
        Rc::new(output::ThrowStatementStruct {
            node_id: source.node_id,
        })
    }

    fn transform_assignment_expression(
        &mut self,
        source: &input::AssignmentExpression,
    ) -> output::AssignmentExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::AssignmentExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn transform_conditional_expression(
        &mut self,
        source: &input::ConditionalExpression,
    ) -> output::ConditionalExpression {
        let operand = self.transform_expression(&source.operand);
        let true_expression = self.transform_expression(&source.true_expression);
        let false_expression = self.transform_expression(&source.false_expression);

        Rc::new(output::ConditionalExpressionStruct {
            node_id: source.node_id,
            operand,
            true_expression,
            false_expression,
        })
    }

    fn transform_or_expression(&mut self, source: &input::OrExpression) -> output::OrExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::OrExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn transform_and_expression(&mut self, source: &input::AndExpression) -> output::AndExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::AndExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn transform_equality_expression(
        &mut self,
        source: &input::EqualityExpression,
    ) -> output::EqualityExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::EqualityExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn transform_inequality_expression(
        &mut self,
        source: &input::InequalityExpression,
    ) -> output::InequalityExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::InequalityExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn transform_bitwise_or_expression(
        &mut self,
        source: &input::BitwiseOrExpression,
    ) -> output::BitwiseOrExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::BitwiseOrExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn transform_bitwise_xor_expression(
        &mut self,
        source: &input::BitwiseXorExpression,
    ) -> output::BitwiseXorExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::BitwiseXorExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn transform_bitwise_and_expression(
        &mut self,
        source: &input::BitwiseAndExpression,
    ) -> output::BitwiseAndExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::BitwiseAndExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn transform_shift_expression(
        &mut self,
        source: &input::ShiftExpression,
    ) -> output::ShiftExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::ShiftExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn transform_additive_expression(
        &mut self,
        source: &input::AdditiveExpression,
    ) -> output::AdditiveExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::AdditiveExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn transform_multiplicative_expression(
        &mut self,
        source: &input::MultiplicativeExpression,
    ) -> output::MultiplicativeExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::MultiplicativeExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn transform_exponentiation_expression(
        &mut self,
        source: &input::ExponentiationExpression,
    ) -> output::ExponentiationExpression {
        let left_operand = self.transform_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.transform_expression(&source.right_operand);

        Rc::new(output::ExponentiationExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn transform_postfix_expression(
        &mut self,
        source: &input::PostfixExpression,
    ) -> output::PostfixExpression {
        let operand = self.transform_expression(&source.operand);
        let operator = Rc::clone(&source.operator);

        Rc::new(output::PostfixExpressionStruct {
            node_id: source.node_id,
            operand,
            operator,
        })
    }

    fn transform_prefix_expression(
        &mut self,
        source: &input::PrefixExpression,
    ) -> output::PrefixExpression {
        let operator = Rc::clone(&source.operator);
        let operand = self.transform_expression(&source.operand);

        Rc::new(output::PrefixExpressionStruct {
            node_id: source.node_id,
            operator,
            operand,
        })
    }

    fn transform_function_call_expression(
        &mut self,
        source: &input::FunctionCallExpression,
    ) -> output::FunctionCallExpression {
        let operand = self.transform_expression(&source.operand);
        let arguments = self.transform_arguments_declaration(&source.arguments);

        Rc::new(output::FunctionCallExpressionStruct {
            node_id: source.node_id,
            operand,
            arguments,
        })
    }

    fn transform_call_options_expression(
        &mut self,
        source: &input::CallOptionsExpression,
    ) -> output::CallOptionsExpression {
        let operand = self.transform_expression(&source.operand);
        let options = self.transform_call_options(&source.options);

        Rc::new(output::CallOptionsExpressionStruct {
            node_id: source.node_id,
            operand,
            options,
        })
    }

    fn transform_member_access_expression(
        &mut self,
        source: &input::MemberAccessExpression,
    ) -> output::MemberAccessExpression {
        let operand = self.transform_expression(&source.operand);
        let member = Rc::clone(&source.member);

        Rc::new(output::MemberAccessExpressionStruct {
            node_id: source.node_id,
            operand,
            member,
        })
    }

    fn transform_index_access_expression(
        &mut self,
        source: &input::IndexAccessExpression,
    ) -> output::IndexAccessExpression {
        let operand = self.transform_expression(&source.operand);
        let start = source
            .start
            .as_ref()
            .map(|value| self.transform_expression(value));
        let end = source
            .end
            .as_ref()
            .map(|value| self.transform_index_access_end(value));

        Rc::new(output::IndexAccessExpressionStruct {
            node_id: source.node_id,
            operand,
            start,
            end,
        })
    }

    fn transform_index_access_end(
        &mut self,
        source: &input::IndexAccessEnd,
    ) -> output::IndexAccessEnd {
        let end = source
            .end
            .as_ref()
            .map(|value| self.transform_expression(value));

        Rc::new(output::IndexAccessEndStruct {
            node_id: source.node_id,
            end,
        })
    }

    fn transform_positional_arguments_declaration(
        &mut self,
        source: &input::PositionalArgumentsDeclaration,
    ) -> output::PositionalArgumentsDeclaration {
        let arguments = self.transform_positional_arguments(&source.arguments);

        Rc::new(output::PositionalArgumentsDeclarationStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn transform_named_arguments_declaration(
        &mut self,
        source: &input::NamedArgumentsDeclaration,
    ) -> output::NamedArgumentsDeclaration {
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.transform_named_argument_group(value));

        Rc::new(output::NamedArgumentsDeclarationStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn transform_named_argument_group(
        &mut self,
        source: &input::NamedArgumentGroup,
    ) -> output::NamedArgumentGroup {
        let arguments = self.transform_named_arguments(&source.arguments);

        Rc::new(output::NamedArgumentGroupStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn transform_named_argument(&mut self, source: &input::NamedArgument) -> output::NamedArgument {
        let name = Rc::clone(&source.name);
        let value = self.transform_expression(&source.value);

        Rc::new(output::NamedArgumentStruct {
            node_id: source.node_id,
            name,
            value,
        })
    }

    fn transform_type_expression(
        &mut self,
        source: &input::TypeExpression,
    ) -> output::TypeExpression {
        let type_name = self.transform_type_name(&source.type_name);

        Rc::new(output::TypeExpressionStruct {
            node_id: source.node_id,
            type_name,
        })
    }

    fn transform_new_expression(&mut self, source: &input::NewExpression) -> output::NewExpression {
        let type_name = self.transform_type_name(&source.type_name);

        Rc::new(output::NewExpressionStruct {
            node_id: source.node_id,
            type_name,
        })
    }

    fn transform_tuple_expression(
        &mut self,
        source: &input::TupleExpression,
    ) -> output::TupleExpression {
        let items = self.transform_tuple_values(&source.items);

        Rc::new(output::TupleExpressionStruct {
            node_id: source.node_id,
            items,
        })
    }

    fn transform_tuple_value(&mut self, source: &input::TupleValue) -> output::TupleValue {
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.transform_expression(value));

        Rc::new(output::TupleValueStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn transform_array_expression(
        &mut self,
        source: &input::ArrayExpression,
    ) -> output::ArrayExpression {
        let items = self.transform_array_values(&source.items);

        Rc::new(output::ArrayExpressionStruct {
            node_id: source.node_id,
            items,
        })
    }

    fn transform_hex_number_expression(
        &mut self,
        source: &input::HexNumberExpression,
    ) -> output::HexNumberExpression {
        let literal = Rc::clone(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.transform_number_unit(value));

        Rc::new(output::HexNumberExpressionStruct {
            node_id: source.node_id,
            literal,
            unit,
        })
    }

    fn transform_decimal_number_expression(
        &mut self,
        source: &input::DecimalNumberExpression,
    ) -> output::DecimalNumberExpression {
        let literal = Rc::clone(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.transform_number_unit(value));

        Rc::new(output::DecimalNumberExpressionStruct {
            node_id: source.node_id,
            literal,
            unit,
        })
    }

    fn transform_yul_block(&mut self, source: &input::YulBlock) -> output::YulBlock {
        let statements = self.transform_yul_statements(&source.statements);

        Rc::new(output::YulBlockStruct {
            node_id: source.node_id,
            statements,
        })
    }

    fn transform_yul_function_definition(
        &mut self,
        source: &input::YulFunctionDefinition,
    ) -> output::YulFunctionDefinition;

    fn transform_yul_variable_declaration_statement(
        &mut self,
        source: &input::YulVariableDeclarationStatement,
    ) -> output::YulVariableDeclarationStatement {
        let variables = self.transform_yul_variable_names(&source.variables);
        let value = source
            .value
            .as_ref()
            .map(|value| self.transform_yul_variable_declaration_value(value));

        Rc::new(output::YulVariableDeclarationStatementStruct {
            node_id: source.node_id,
            variables,
            value,
        })
    }

    fn transform_yul_variable_declaration_value(
        &mut self,
        source: &input::YulVariableDeclarationValue,
    ) -> output::YulVariableDeclarationValue {
        let assignment = self.transform_yul_assignment_operator(&source.assignment);
        let expression = self.transform_yul_expression(&source.expression);

        Rc::new(output::YulVariableDeclarationValueStruct {
            node_id: source.node_id,
            assignment,
            expression,
        })
    }

    fn transform_yul_variable_assignment_statement(
        &mut self,
        source: &input::YulVariableAssignmentStatement,
    ) -> output::YulVariableAssignmentStatement {
        let variables = self.transform_yul_paths(&source.variables);
        let assignment = self.transform_yul_assignment_operator(&source.assignment);
        let expression = self.transform_yul_expression(&source.expression);

        Rc::new(output::YulVariableAssignmentStatementStruct {
            node_id: source.node_id,
            variables,
            assignment,
            expression,
        })
    }

    fn transform_yul_colon_and_equal(
        &mut self,
        source: &input::YulColonAndEqual,
    ) -> output::YulColonAndEqual {
        Rc::new(output::YulColonAndEqualStruct {
            node_id: source.node_id,
        })
    }

    fn transform_yul_stack_assignment_statement(
        &mut self,
        source: &input::YulStackAssignmentStatement,
    ) -> output::YulStackAssignmentStatement {
        let assignment = self.transform_yul_stack_assignment_operator(&source.assignment);
        let variable = Rc::clone(&source.variable);

        Rc::new(output::YulStackAssignmentStatementStruct {
            node_id: source.node_id,
            assignment,
            variable,
        })
    }

    fn transform_yul_equal_and_colon(
        &mut self,
        source: &input::YulEqualAndColon,
    ) -> output::YulEqualAndColon {
        Rc::new(output::YulEqualAndColonStruct {
            node_id: source.node_id,
        })
    }

    fn transform_yul_if_statement(
        &mut self,
        source: &input::YulIfStatement,
    ) -> output::YulIfStatement {
        let condition = self.transform_yul_expression(&source.condition);
        let body = self.transform_yul_block(&source.body);

        Rc::new(output::YulIfStatementStruct {
            node_id: source.node_id,
            condition,
            body,
        })
    }

    fn transform_yul_for_statement(
        &mut self,
        source: &input::YulForStatement,
    ) -> output::YulForStatement {
        let initialization = self.transform_yul_block(&source.initialization);
        let condition = self.transform_yul_expression(&source.condition);
        let iterator = self.transform_yul_block(&source.iterator);
        let body = self.transform_yul_block(&source.body);

        Rc::new(output::YulForStatementStruct {
            node_id: source.node_id,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    fn transform_yul_switch_statement(
        &mut self,
        source: &input::YulSwitchStatement,
    ) -> output::YulSwitchStatement {
        let expression = self.transform_yul_expression(&source.expression);
        let cases = self.transform_yul_switch_cases(&source.cases);

        Rc::new(output::YulSwitchStatementStruct {
            node_id: source.node_id,
            expression,
            cases,
        })
    }

    fn transform_yul_default_case(
        &mut self,
        source: &input::YulDefaultCase,
    ) -> output::YulDefaultCase {
        let body = self.transform_yul_block(&source.body);

        Rc::new(output::YulDefaultCaseStruct {
            node_id: source.node_id,
            body,
        })
    }

    fn transform_yul_value_case(&mut self, source: &input::YulValueCase) -> output::YulValueCase {
        let value = self.transform_yul_literal(&source.value);
        let body = self.transform_yul_block(&source.body);

        Rc::new(output::YulValueCaseStruct {
            node_id: source.node_id,
            value,
            body,
        })
    }

    fn transform_yul_leave_statement(
        &mut self,
        source: &input::YulLeaveStatement,
    ) -> output::YulLeaveStatement {
        Rc::new(output::YulLeaveStatementStruct {
            node_id: source.node_id,
        })
    }

    fn transform_yul_break_statement(
        &mut self,
        source: &input::YulBreakStatement,
    ) -> output::YulBreakStatement {
        Rc::new(output::YulBreakStatementStruct {
            node_id: source.node_id,
        })
    }

    fn transform_yul_continue_statement(
        &mut self,
        source: &input::YulContinueStatement,
    ) -> output::YulContinueStatement {
        Rc::new(output::YulContinueStatementStruct {
            node_id: source.node_id,
        })
    }

    fn transform_yul_label(&mut self, source: &input::YulLabel) -> output::YulLabel {
        let label = Rc::clone(&source.label);

        Rc::new(output::YulLabelStruct {
            node_id: source.node_id,
            label,
        })
    }

    fn transform_yul_function_call_expression(
        &mut self,
        source: &input::YulFunctionCallExpression,
    ) -> output::YulFunctionCallExpression {
        let operand = self.transform_yul_expression(&source.operand);
        let arguments = self.transform_yul_arguments(&source.arguments);

        Rc::new(output::YulFunctionCallExpressionStruct {
            node_id: source.node_id,
            operand,
            arguments,
        })
    }

    //
    // Choices:
    //

    fn default_transform_source_unit_member(
        &mut self,
        source: &input::SourceUnitMember,
    ) -> output::SourceUnitMember {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::SourceUnitMember::PragmaDirective(ref pragma_directive) => {
                output::SourceUnitMember::PragmaDirective(
                    self.transform_pragma_directive(pragma_directive),
                )
            }
            input::SourceUnitMember::ImportDirective(ref import_directive) => {
                output::SourceUnitMember::ImportDirective(
                    self.transform_import_directive(import_directive),
                )
            }
            input::SourceUnitMember::ContractDefinition(ref contract_definition) => {
                output::SourceUnitMember::ContractDefinition(
                    self.transform_contract_definition(contract_definition),
                )
            }
            input::SourceUnitMember::InterfaceDefinition(ref interface_definition) => {
                output::SourceUnitMember::InterfaceDefinition(
                    self.transform_interface_definition(interface_definition),
                )
            }
            input::SourceUnitMember::LibraryDefinition(ref library_definition) => {
                output::SourceUnitMember::LibraryDefinition(
                    self.transform_library_definition(library_definition),
                )
            }
            input::SourceUnitMember::StructDefinition(ref struct_definition) => {
                output::SourceUnitMember::StructDefinition(
                    self.transform_struct_definition(struct_definition),
                )
            }
            input::SourceUnitMember::EnumDefinition(ref enum_definition) => {
                output::SourceUnitMember::EnumDefinition(
                    self.transform_enum_definition(enum_definition),
                )
            }
            input::SourceUnitMember::FunctionDefinition(ref function_definition) => {
                output::SourceUnitMember::FunctionDefinition(
                    self.transform_function_definition(function_definition),
                )
            }
            input::SourceUnitMember::ErrorDefinition(ref error_definition) => {
                output::SourceUnitMember::ErrorDefinition(
                    self.transform_error_definition(error_definition),
                )
            }
            input::SourceUnitMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => output::SourceUnitMember::UserDefinedValueTypeDefinition(
                self.transform_user_defined_value_type_definition(
                    user_defined_value_type_definition,
                ),
            ),
            input::SourceUnitMember::UsingDirective(ref using_directive) => {
                output::SourceUnitMember::UsingDirective(
                    self.transform_using_directive(using_directive),
                )
            }
            input::SourceUnitMember::EventDefinition(ref event_definition) => {
                output::SourceUnitMember::EventDefinition(
                    self.transform_event_definition(event_definition),
                )
            }
            input::SourceUnitMember::ConstantDefinition(ref constant_definition) => {
                output::SourceUnitMember::ConstantDefinition(
                    self.transform_constant_definition(constant_definition),
                )
            }
        }
    }
    fn transform_source_unit_member(
        &mut self,
        source: &input::SourceUnitMember,
    ) -> output::SourceUnitMember {
        self.default_transform_source_unit_member(source)
    }

    fn default_transform_pragma(&mut self, source: &input::Pragma) -> output::Pragma {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::Pragma::VersionPragma(ref version_pragma) => {
                output::Pragma::VersionPragma(self.transform_version_pragma(version_pragma))
            }
            input::Pragma::AbicoderPragma(ref abicoder_pragma) => {
                output::Pragma::AbicoderPragma(self.transform_abicoder_pragma(abicoder_pragma))
            }
            input::Pragma::ExperimentalPragma(ref experimental_pragma) => {
                output::Pragma::ExperimentalPragma(
                    self.transform_experimental_pragma(experimental_pragma),
                )
            }
        }
    }
    fn transform_pragma(&mut self, source: &input::Pragma) -> output::Pragma {
        self.default_transform_pragma(source)
    }

    fn default_transform_abicoder_version(
        &mut self,
        source: &input::AbicoderVersion,
    ) -> output::AbicoderVersion {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::AbicoderVersion::AbicoderV1Keyword => output::AbicoderVersion::AbicoderV1Keyword,
            input::AbicoderVersion::AbicoderV2Keyword => output::AbicoderVersion::AbicoderV2Keyword,
        }
    }
    fn transform_abicoder_version(
        &mut self,
        source: &input::AbicoderVersion,
    ) -> output::AbicoderVersion {
        self.default_transform_abicoder_version(source)
    }

    fn default_transform_experimental_feature(
        &mut self,
        source: &input::ExperimentalFeature,
    ) -> output::ExperimentalFeature {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::ExperimentalFeature::StringLiteral(ref string_literal) => {
                output::ExperimentalFeature::StringLiteral(
                    self.transform_string_literal(string_literal),
                )
            }
            input::ExperimentalFeature::ABIEncoderV2Keyword => {
                output::ExperimentalFeature::ABIEncoderV2Keyword
            }
            input::ExperimentalFeature::SMTCheckerKeyword => {
                output::ExperimentalFeature::SMTCheckerKeyword
            }
        }
    }
    fn transform_experimental_feature(
        &mut self,
        source: &input::ExperimentalFeature,
    ) -> output::ExperimentalFeature {
        self.default_transform_experimental_feature(source)
    }

    fn default_transform_version_expression(
        &mut self,
        source: &input::VersionExpression,
    ) -> output::VersionExpression {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::VersionExpression::VersionRange(ref version_range) => {
                output::VersionExpression::VersionRange(self.transform_version_range(version_range))
            }
            input::VersionExpression::VersionTerm(ref version_term) => {
                output::VersionExpression::VersionTerm(self.transform_version_term(version_term))
            }
        }
    }
    fn transform_version_expression(
        &mut self,
        source: &input::VersionExpression,
    ) -> output::VersionExpression {
        self.default_transform_version_expression(source)
    }

    fn default_transform_version_operator(
        &mut self,
        source: &input::VersionOperator,
    ) -> output::VersionOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::VersionOperator::Caret => output::VersionOperator::Caret,
            input::VersionOperator::Tilde => output::VersionOperator::Tilde,
            input::VersionOperator::Equal => output::VersionOperator::Equal,
            input::VersionOperator::LessThan => output::VersionOperator::LessThan,
            input::VersionOperator::GreaterThan => output::VersionOperator::GreaterThan,
            input::VersionOperator::LessThanEqual => output::VersionOperator::LessThanEqual,
            input::VersionOperator::GreaterThanEqual => output::VersionOperator::GreaterThanEqual,
        }
    }
    fn transform_version_operator(
        &mut self,
        source: &input::VersionOperator,
    ) -> output::VersionOperator {
        self.default_transform_version_operator(source)
    }

    fn default_transform_version_literal(
        &mut self,
        source: &input::VersionLiteral,
    ) -> output::VersionLiteral {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::VersionLiteral::SimpleVersionLiteral(ref simple_version_literal) => {
                output::VersionLiteral::SimpleVersionLiteral(
                    self.transform_simple_version_literal(simple_version_literal),
                )
            }
            input::VersionLiteral::SingleQuotedVersionLiteral(node) => {
                output::VersionLiteral::SingleQuotedVersionLiteral(Rc::clone(node))
            }
            input::VersionLiteral::DoubleQuotedVersionLiteral(node) => {
                output::VersionLiteral::DoubleQuotedVersionLiteral(Rc::clone(node))
            }
        }
    }
    fn transform_version_literal(
        &mut self,
        source: &input::VersionLiteral,
    ) -> output::VersionLiteral {
        self.default_transform_version_literal(source)
    }

    fn default_transform_import_clause(
        &mut self,
        source: &input::ImportClause,
    ) -> output::ImportClause {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::ImportClause::PathImport(ref path_import) => {
                output::ImportClause::PathImport(self.transform_path_import(path_import))
            }
            input::ImportClause::NamedImport(ref named_import) => {
                output::ImportClause::NamedImport(self.transform_named_import(named_import))
            }
            input::ImportClause::ImportDeconstruction(ref import_deconstruction) => {
                output::ImportClause::ImportDeconstruction(
                    self.transform_import_deconstruction(import_deconstruction),
                )
            }
        }
    }
    fn transform_import_clause(&mut self, source: &input::ImportClause) -> output::ImportClause {
        self.default_transform_import_clause(source)
    }

    fn default_transform_using_clause(
        &mut self,
        source: &input::UsingClause,
    ) -> output::UsingClause {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::UsingClause::IdentifierPath(ref identifier_path) => {
                output::UsingClause::IdentifierPath(self.transform_identifier_path(identifier_path))
            }
            input::UsingClause::UsingDeconstruction(ref using_deconstruction) => {
                output::UsingClause::UsingDeconstruction(
                    self.transform_using_deconstruction(using_deconstruction),
                )
            }
        }
    }
    fn transform_using_clause(&mut self, source: &input::UsingClause) -> output::UsingClause {
        self.default_transform_using_clause(source)
    }

    fn default_transform_using_operator(
        &mut self,
        source: &input::UsingOperator,
    ) -> output::UsingOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::UsingOperator::Ampersand => output::UsingOperator::Ampersand,
            input::UsingOperator::Asterisk => output::UsingOperator::Asterisk,
            input::UsingOperator::BangEqual => output::UsingOperator::BangEqual,
            input::UsingOperator::Bar => output::UsingOperator::Bar,
            input::UsingOperator::Caret => output::UsingOperator::Caret,
            input::UsingOperator::EqualEqual => output::UsingOperator::EqualEqual,
            input::UsingOperator::GreaterThan => output::UsingOperator::GreaterThan,
            input::UsingOperator::GreaterThanEqual => output::UsingOperator::GreaterThanEqual,
            input::UsingOperator::LessThan => output::UsingOperator::LessThan,
            input::UsingOperator::LessThanEqual => output::UsingOperator::LessThanEqual,
            input::UsingOperator::Minus => output::UsingOperator::Minus,
            input::UsingOperator::Percent => output::UsingOperator::Percent,
            input::UsingOperator::Plus => output::UsingOperator::Plus,
            input::UsingOperator::Slash => output::UsingOperator::Slash,
            input::UsingOperator::Tilde => output::UsingOperator::Tilde,
        }
    }
    fn transform_using_operator(&mut self, source: &input::UsingOperator) -> output::UsingOperator {
        self.default_transform_using_operator(source)
    }

    fn default_transform_using_target(
        &mut self,
        source: &input::UsingTarget,
    ) -> output::UsingTarget {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::UsingTarget::TypeName(ref type_name) => {
                output::UsingTarget::TypeName(self.transform_type_name(type_name))
            }
            input::UsingTarget::Asterisk => output::UsingTarget::Asterisk,
        }
    }
    fn transform_using_target(&mut self, source: &input::UsingTarget) -> output::UsingTarget {
        self.default_transform_using_target(source)
    }

    fn default_transform_contract_specifier(
        &mut self,
        source: &input::ContractSpecifier,
    ) -> output::ContractSpecifier {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::ContractSpecifier::InheritanceSpecifier(ref inheritance_specifier) => {
                output::ContractSpecifier::InheritanceSpecifier(
                    self.transform_inheritance_specifier(inheritance_specifier),
                )
            }
            input::ContractSpecifier::StorageLayoutSpecifier(ref storage_layout_specifier) => {
                output::ContractSpecifier::StorageLayoutSpecifier(
                    self.transform_storage_layout_specifier(storage_layout_specifier),
                )
            }
        }
    }
    fn transform_contract_specifier(
        &mut self,
        source: &input::ContractSpecifier,
    ) -> output::ContractSpecifier {
        self.default_transform_contract_specifier(source)
    }

    fn default_transform_contract_member(
        &mut self,
        source: &input::ContractMember,
    ) -> output::ContractMember {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::ContractMember::UsingDirective(ref using_directive) => {
                output::ContractMember::UsingDirective(
                    self.transform_using_directive(using_directive),
                )
            }
            input::ContractMember::FunctionDefinition(ref function_definition) => {
                output::ContractMember::FunctionDefinition(
                    self.transform_function_definition(function_definition),
                )
            }
            input::ContractMember::StructDefinition(ref struct_definition) => {
                output::ContractMember::StructDefinition(
                    self.transform_struct_definition(struct_definition),
                )
            }
            input::ContractMember::EnumDefinition(ref enum_definition) => {
                output::ContractMember::EnumDefinition(
                    self.transform_enum_definition(enum_definition),
                )
            }
            input::ContractMember::EventDefinition(ref event_definition) => {
                output::ContractMember::EventDefinition(
                    self.transform_event_definition(event_definition),
                )
            }
            input::ContractMember::ErrorDefinition(ref error_definition) => {
                output::ContractMember::ErrorDefinition(
                    self.transform_error_definition(error_definition),
                )
            }
            input::ContractMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => output::ContractMember::UserDefinedValueTypeDefinition(
                self.transform_user_defined_value_type_definition(
                    user_defined_value_type_definition,
                ),
            ),
            input::ContractMember::StateVariableDefinition(ref state_variable_definition) => {
                output::ContractMember::StateVariableDefinition(
                    self.transform_state_variable_definition(state_variable_definition),
                )
            }
            _ => panic!("Unexpected variant {source:?}"),
        }
    }
    fn transform_contract_member(
        &mut self,
        source: &input::ContractMember,
    ) -> output::ContractMember {
        self.default_transform_contract_member(source)
    }

    fn default_transform_state_variable_attribute(
        &mut self,
        source: &input::StateVariableAttribute,
    ) -> output::StateVariableAttribute {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::StateVariableAttribute::OverrideSpecifier(ref override_specifier) => {
                output::StateVariableAttribute::OverrideSpecifier(
                    self.transform_override_specifier(override_specifier),
                )
            }
            input::StateVariableAttribute::ConstantKeyword => {
                output::StateVariableAttribute::ConstantKeyword
            }
            input::StateVariableAttribute::InternalKeyword => {
                output::StateVariableAttribute::InternalKeyword
            }
            input::StateVariableAttribute::PrivateKeyword => {
                output::StateVariableAttribute::PrivateKeyword
            }
            input::StateVariableAttribute::PublicKeyword => {
                output::StateVariableAttribute::PublicKeyword
            }
            input::StateVariableAttribute::ImmutableKeyword => {
                output::StateVariableAttribute::ImmutableKeyword
            }
            input::StateVariableAttribute::TransientKeyword => {
                output::StateVariableAttribute::TransientKeyword
            }
        }
    }
    fn transform_state_variable_attribute(
        &mut self,
        source: &input::StateVariableAttribute,
    ) -> output::StateVariableAttribute {
        self.default_transform_state_variable_attribute(source)
    }

    fn default_transform_function_attribute(
        &mut self,
        source: &input::FunctionAttribute,
    ) -> output::FunctionAttribute {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::FunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                output::FunctionAttribute::ModifierInvocation(
                    self.transform_modifier_invocation(modifier_invocation),
                )
            }
            input::FunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                output::FunctionAttribute::OverrideSpecifier(
                    self.transform_override_specifier(override_specifier),
                )
            }
            input::FunctionAttribute::ConstantKeyword => output::FunctionAttribute::ConstantKeyword,
            input::FunctionAttribute::ExternalKeyword => output::FunctionAttribute::ExternalKeyword,
            input::FunctionAttribute::InternalKeyword => output::FunctionAttribute::InternalKeyword,
            input::FunctionAttribute::PayableKeyword => output::FunctionAttribute::PayableKeyword,
            input::FunctionAttribute::PrivateKeyword => output::FunctionAttribute::PrivateKeyword,
            input::FunctionAttribute::PublicKeyword => output::FunctionAttribute::PublicKeyword,
            input::FunctionAttribute::PureKeyword => output::FunctionAttribute::PureKeyword,
            input::FunctionAttribute::ViewKeyword => output::FunctionAttribute::ViewKeyword,
            input::FunctionAttribute::VirtualKeyword => output::FunctionAttribute::VirtualKeyword,
        }
    }
    fn transform_function_attribute(
        &mut self,
        source: &input::FunctionAttribute,
    ) -> output::FunctionAttribute {
        self.default_transform_function_attribute(source)
    }

    fn default_transform_type_name(&mut self, source: &input::TypeName) -> output::TypeName {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::TypeName::ArrayTypeName(ref array_type_name) => {
                output::TypeName::ArrayTypeName(self.transform_array_type_name(array_type_name))
            }
            input::TypeName::FunctionType(ref function_type) => {
                output::TypeName::FunctionType(self.transform_function_type(function_type))
            }
            input::TypeName::MappingType(ref mapping_type) => {
                output::TypeName::MappingType(self.transform_mapping_type(mapping_type))
            }
            input::TypeName::ElementaryType(ref elementary_type) => {
                output::TypeName::ElementaryType(self.transform_elementary_type(elementary_type))
            }
            input::TypeName::IdentifierPath(ref identifier_path) => {
                output::TypeName::IdentifierPath(self.transform_identifier_path(identifier_path))
            }
        }
    }
    fn transform_type_name(&mut self, source: &input::TypeName) -> output::TypeName {
        self.default_transform_type_name(source)
    }

    fn default_transform_function_type_attribute(
        &mut self,
        source: &input::FunctionTypeAttribute,
    ) -> output::FunctionTypeAttribute {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::FunctionTypeAttribute::InternalKeyword => {
                output::FunctionTypeAttribute::InternalKeyword
            }
            input::FunctionTypeAttribute::ExternalKeyword => {
                output::FunctionTypeAttribute::ExternalKeyword
            }
            input::FunctionTypeAttribute::PrivateKeyword => {
                output::FunctionTypeAttribute::PrivateKeyword
            }
            input::FunctionTypeAttribute::PublicKeyword => {
                output::FunctionTypeAttribute::PublicKeyword
            }
            input::FunctionTypeAttribute::ConstantKeyword => {
                output::FunctionTypeAttribute::ConstantKeyword
            }
            input::FunctionTypeAttribute::PureKeyword => output::FunctionTypeAttribute::PureKeyword,
            input::FunctionTypeAttribute::ViewKeyword => output::FunctionTypeAttribute::ViewKeyword,
            input::FunctionTypeAttribute::PayableKeyword => {
                output::FunctionTypeAttribute::PayableKeyword
            }
        }
    }
    fn transform_function_type_attribute(
        &mut self,
        source: &input::FunctionTypeAttribute,
    ) -> output::FunctionTypeAttribute {
        self.default_transform_function_type_attribute(source)
    }

    fn default_transform_mapping_key_type(
        &mut self,
        source: &input::MappingKeyType,
    ) -> output::MappingKeyType {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::MappingKeyType::ElementaryType(ref elementary_type) => {
                output::MappingKeyType::ElementaryType(
                    self.transform_elementary_type(elementary_type),
                )
            }
            input::MappingKeyType::IdentifierPath(ref identifier_path) => {
                output::MappingKeyType::IdentifierPath(
                    self.transform_identifier_path(identifier_path),
                )
            }
        }
    }
    fn transform_mapping_key_type(
        &mut self,
        source: &input::MappingKeyType,
    ) -> output::MappingKeyType {
        self.default_transform_mapping_key_type(source)
    }

    fn default_transform_elementary_type(
        &mut self,
        source: &input::ElementaryType,
    ) -> output::ElementaryType {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::ElementaryType::AddressType(ref address_type) => {
                output::ElementaryType::AddressType(self.transform_address_type(address_type))
            }
            input::ElementaryType::BytesKeyword(node) => {
                output::ElementaryType::BytesKeyword(Rc::clone(node))
            }
            input::ElementaryType::IntKeyword(node) => {
                output::ElementaryType::IntKeyword(Rc::clone(node))
            }
            input::ElementaryType::UintKeyword(node) => {
                output::ElementaryType::UintKeyword(Rc::clone(node))
            }
            input::ElementaryType::FixedKeyword(node) => {
                output::ElementaryType::FixedKeyword(Rc::clone(node))
            }
            input::ElementaryType::UfixedKeyword(node) => {
                output::ElementaryType::UfixedKeyword(Rc::clone(node))
            }
            input::ElementaryType::BoolKeyword => output::ElementaryType::BoolKeyword,
            input::ElementaryType::ByteKeyword => output::ElementaryType::ByteKeyword,
            input::ElementaryType::StringKeyword => output::ElementaryType::StringKeyword,
        }
    }
    fn transform_elementary_type(
        &mut self,
        source: &input::ElementaryType,
    ) -> output::ElementaryType {
        self.default_transform_elementary_type(source)
    }

    fn default_transform_statement(&mut self, source: &input::Statement) -> output::Statement {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::Statement::IfStatement(ref if_statement) => {
                output::Statement::IfStatement(self.transform_if_statement(if_statement))
            }
            input::Statement::ForStatement(ref for_statement) => {
                output::Statement::ForStatement(self.transform_for_statement(for_statement))
            }
            input::Statement::WhileStatement(ref while_statement) => {
                output::Statement::WhileStatement(self.transform_while_statement(while_statement))
            }
            input::Statement::DoWhileStatement(ref do_while_statement) => {
                output::Statement::DoWhileStatement(
                    self.transform_do_while_statement(do_while_statement),
                )
            }
            input::Statement::ContinueStatement(ref continue_statement) => {
                output::Statement::ContinueStatement(
                    self.transform_continue_statement(continue_statement),
                )
            }
            input::Statement::BreakStatement(ref break_statement) => {
                output::Statement::BreakStatement(self.transform_break_statement(break_statement))
            }
            input::Statement::ReturnStatement(ref return_statement) => {
                output::Statement::ReturnStatement(
                    self.transform_return_statement(return_statement),
                )
            }
            input::Statement::ThrowStatement(ref throw_statement) => {
                output::Statement::ThrowStatement(self.transform_throw_statement(throw_statement))
            }
            input::Statement::EmitStatement(ref emit_statement) => {
                output::Statement::EmitStatement(self.transform_emit_statement(emit_statement))
            }
            input::Statement::TryStatement(ref try_statement) => {
                output::Statement::TryStatement(self.transform_try_statement(try_statement))
            }
            input::Statement::RevertStatement(ref revert_statement) => {
                output::Statement::RevertStatement(
                    self.transform_revert_statement(revert_statement),
                )
            }
            input::Statement::AssemblyStatement(ref assembly_statement) => {
                output::Statement::AssemblyStatement(
                    self.transform_assembly_statement(assembly_statement),
                )
            }
            input::Statement::Block(ref block) => {
                output::Statement::Block(self.transform_block(block))
            }
            input::Statement::UncheckedBlock(ref unchecked_block) => {
                output::Statement::UncheckedBlock(self.transform_unchecked_block(unchecked_block))
            }
            input::Statement::TupleDeconstructionStatement(ref tuple_deconstruction_statement) => {
                output::Statement::TupleDeconstructionStatement(
                    self.transform_tuple_deconstruction_statement(tuple_deconstruction_statement),
                )
            }
            input::Statement::VariableDeclarationStatement(ref variable_declaration_statement) => {
                output::Statement::VariableDeclarationStatement(
                    self.transform_variable_declaration_statement(variable_declaration_statement),
                )
            }
            input::Statement::ExpressionStatement(ref expression_statement) => {
                output::Statement::ExpressionStatement(
                    self.transform_expression_statement(expression_statement),
                )
            }
        }
    }
    fn transform_statement(&mut self, source: &input::Statement) -> output::Statement {
        self.default_transform_statement(source)
    }

    fn default_transform_tuple_member(
        &mut self,
        source: &input::TupleMember,
    ) -> output::TupleMember {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::TupleMember::TypedTupleMember(ref typed_tuple_member) => {
                output::TupleMember::TypedTupleMember(
                    self.transform_typed_tuple_member(typed_tuple_member),
                )
            }
            input::TupleMember::UntypedTupleMember(ref untyped_tuple_member) => {
                output::TupleMember::UntypedTupleMember(
                    self.transform_untyped_tuple_member(untyped_tuple_member),
                )
            }
        }
    }
    fn transform_tuple_member(&mut self, source: &input::TupleMember) -> output::TupleMember {
        self.default_transform_tuple_member(source)
    }

    fn default_transform_variable_declaration_type(
        &mut self,
        source: &input::VariableDeclarationType,
    ) -> output::VariableDeclarationType {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::VariableDeclarationType::TypeName(ref type_name) => {
                output::VariableDeclarationType::TypeName(self.transform_type_name(type_name))
            }
            input::VariableDeclarationType::VarKeyword => {
                output::VariableDeclarationType::VarKeyword
            }
        }
    }
    fn transform_variable_declaration_type(
        &mut self,
        source: &input::VariableDeclarationType,
    ) -> output::VariableDeclarationType {
        self.default_transform_variable_declaration_type(source)
    }

    fn default_transform_storage_location(
        &mut self,
        source: &input::StorageLocation,
    ) -> output::StorageLocation {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::StorageLocation::MemoryKeyword => output::StorageLocation::MemoryKeyword,
            input::StorageLocation::StorageKeyword => output::StorageLocation::StorageKeyword,
            input::StorageLocation::CallDataKeyword => output::StorageLocation::CallDataKeyword,
        }
    }
    fn transform_storage_location(
        &mut self,
        source: &input::StorageLocation,
    ) -> output::StorageLocation {
        self.default_transform_storage_location(source)
    }

    fn default_transform_for_statement_initialization(
        &mut self,
        source: &input::ForStatementInitialization,
    ) -> output::ForStatementInitialization {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::ForStatementInitialization::TupleDeconstructionStatement(
                ref tuple_deconstruction_statement,
            ) => output::ForStatementInitialization::TupleDeconstructionStatement(
                self.transform_tuple_deconstruction_statement(tuple_deconstruction_statement),
            ),
            input::ForStatementInitialization::VariableDeclarationStatement(
                ref variable_declaration_statement,
            ) => output::ForStatementInitialization::VariableDeclarationStatement(
                self.transform_variable_declaration_statement(variable_declaration_statement),
            ),
            input::ForStatementInitialization::ExpressionStatement(ref expression_statement) => {
                output::ForStatementInitialization::ExpressionStatement(
                    self.transform_expression_statement(expression_statement),
                )
            }
            input::ForStatementInitialization::Semicolon => {
                output::ForStatementInitialization::Semicolon
            }
        }
    }
    fn transform_for_statement_initialization(
        &mut self,
        source: &input::ForStatementInitialization,
    ) -> output::ForStatementInitialization {
        self.default_transform_for_statement_initialization(source)
    }

    fn default_transform_for_statement_condition(
        &mut self,
        source: &input::ForStatementCondition,
    ) -> output::ForStatementCondition {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::ForStatementCondition::ExpressionStatement(ref expression_statement) => {
                output::ForStatementCondition::ExpressionStatement(
                    self.transform_expression_statement(expression_statement),
                )
            }
            input::ForStatementCondition::Semicolon => output::ForStatementCondition::Semicolon,
        }
    }
    fn transform_for_statement_condition(
        &mut self,
        source: &input::ForStatementCondition,
    ) -> output::ForStatementCondition {
        self.default_transform_for_statement_condition(source)
    }

    fn default_transform_expression(&mut self, source: &input::Expression) -> output::Expression {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::Expression::AssignmentExpression(ref assignment_expression) => {
                output::Expression::AssignmentExpression(
                    self.transform_assignment_expression(assignment_expression),
                )
            }
            input::Expression::ConditionalExpression(ref conditional_expression) => {
                output::Expression::ConditionalExpression(
                    self.transform_conditional_expression(conditional_expression),
                )
            }
            input::Expression::OrExpression(ref or_expression) => {
                output::Expression::OrExpression(self.transform_or_expression(or_expression))
            }
            input::Expression::AndExpression(ref and_expression) => {
                output::Expression::AndExpression(self.transform_and_expression(and_expression))
            }
            input::Expression::EqualityExpression(ref equality_expression) => {
                output::Expression::EqualityExpression(
                    self.transform_equality_expression(equality_expression),
                )
            }
            input::Expression::InequalityExpression(ref inequality_expression) => {
                output::Expression::InequalityExpression(
                    self.transform_inequality_expression(inequality_expression),
                )
            }
            input::Expression::BitwiseOrExpression(ref bitwise_or_expression) => {
                output::Expression::BitwiseOrExpression(
                    self.transform_bitwise_or_expression(bitwise_or_expression),
                )
            }
            input::Expression::BitwiseXorExpression(ref bitwise_xor_expression) => {
                output::Expression::BitwiseXorExpression(
                    self.transform_bitwise_xor_expression(bitwise_xor_expression),
                )
            }
            input::Expression::BitwiseAndExpression(ref bitwise_and_expression) => {
                output::Expression::BitwiseAndExpression(
                    self.transform_bitwise_and_expression(bitwise_and_expression),
                )
            }
            input::Expression::ShiftExpression(ref shift_expression) => {
                output::Expression::ShiftExpression(
                    self.transform_shift_expression(shift_expression),
                )
            }
            input::Expression::AdditiveExpression(ref additive_expression) => {
                output::Expression::AdditiveExpression(
                    self.transform_additive_expression(additive_expression),
                )
            }
            input::Expression::MultiplicativeExpression(ref multiplicative_expression) => {
                output::Expression::MultiplicativeExpression(
                    self.transform_multiplicative_expression(multiplicative_expression),
                )
            }
            input::Expression::ExponentiationExpression(ref exponentiation_expression) => {
                output::Expression::ExponentiationExpression(
                    self.transform_exponentiation_expression(exponentiation_expression),
                )
            }
            input::Expression::PostfixExpression(ref postfix_expression) => {
                output::Expression::PostfixExpression(
                    self.transform_postfix_expression(postfix_expression),
                )
            }
            input::Expression::PrefixExpression(ref prefix_expression) => {
                output::Expression::PrefixExpression(
                    self.transform_prefix_expression(prefix_expression),
                )
            }
            input::Expression::FunctionCallExpression(ref function_call_expression) => {
                output::Expression::FunctionCallExpression(
                    self.transform_function_call_expression(function_call_expression),
                )
            }
            input::Expression::CallOptionsExpression(ref call_options_expression) => {
                output::Expression::CallOptionsExpression(
                    self.transform_call_options_expression(call_options_expression),
                )
            }
            input::Expression::MemberAccessExpression(ref member_access_expression) => {
                output::Expression::MemberAccessExpression(
                    self.transform_member_access_expression(member_access_expression),
                )
            }
            input::Expression::IndexAccessExpression(ref index_access_expression) => {
                output::Expression::IndexAccessExpression(
                    self.transform_index_access_expression(index_access_expression),
                )
            }
            input::Expression::NewExpression(ref new_expression) => {
                output::Expression::NewExpression(self.transform_new_expression(new_expression))
            }
            input::Expression::TupleExpression(ref tuple_expression) => {
                output::Expression::TupleExpression(
                    self.transform_tuple_expression(tuple_expression),
                )
            }
            input::Expression::TypeExpression(ref type_expression) => {
                output::Expression::TypeExpression(self.transform_type_expression(type_expression))
            }
            input::Expression::ArrayExpression(ref array_expression) => {
                output::Expression::ArrayExpression(
                    self.transform_array_expression(array_expression),
                )
            }
            input::Expression::HexNumberExpression(ref hex_number_expression) => {
                output::Expression::HexNumberExpression(
                    self.transform_hex_number_expression(hex_number_expression),
                )
            }
            input::Expression::DecimalNumberExpression(ref decimal_number_expression) => {
                output::Expression::DecimalNumberExpression(
                    self.transform_decimal_number_expression(decimal_number_expression),
                )
            }
            input::Expression::StringExpression(ref string_expression) => {
                output::Expression::StringExpression(
                    self.transform_string_expression(string_expression),
                )
            }
            input::Expression::ElementaryType(ref elementary_type) => {
                output::Expression::ElementaryType(self.transform_elementary_type(elementary_type))
            }
            input::Expression::Identifier(node) => output::Expression::Identifier(Rc::clone(node)),
            input::Expression::PayableKeyword => output::Expression::PayableKeyword,
            input::Expression::ThisKeyword => output::Expression::ThisKeyword,
            input::Expression::SuperKeyword => output::Expression::SuperKeyword,
            input::Expression::TrueKeyword => output::Expression::TrueKeyword,
            input::Expression::FalseKeyword => output::Expression::FalseKeyword,
        }
    }
    fn transform_expression(&mut self, source: &input::Expression) -> output::Expression {
        self.default_transform_expression(source)
    }

    fn default_transform_arguments_declaration(
        &mut self,
        source: &input::ArgumentsDeclaration,
    ) -> output::ArgumentsDeclaration {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::ArgumentsDeclaration::PositionalArgumentsDeclaration(
                ref positional_arguments_declaration,
            ) => output::ArgumentsDeclaration::PositionalArgumentsDeclaration(
                self.transform_positional_arguments_declaration(positional_arguments_declaration),
            ),
            input::ArgumentsDeclaration::NamedArgumentsDeclaration(
                ref named_arguments_declaration,
            ) => output::ArgumentsDeclaration::NamedArgumentsDeclaration(
                self.transform_named_arguments_declaration(named_arguments_declaration),
            ),
        }
    }
    fn transform_arguments_declaration(
        &mut self,
        source: &input::ArgumentsDeclaration,
    ) -> output::ArgumentsDeclaration {
        self.default_transform_arguments_declaration(source)
    }

    fn default_transform_number_unit(&mut self, source: &input::NumberUnit) -> output::NumberUnit {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::NumberUnit::WeiKeyword => output::NumberUnit::WeiKeyword,
            input::NumberUnit::GweiKeyword => output::NumberUnit::GweiKeyword,
            input::NumberUnit::SzaboKeyword => output::NumberUnit::SzaboKeyword,
            input::NumberUnit::FinneyKeyword => output::NumberUnit::FinneyKeyword,
            input::NumberUnit::EtherKeyword => output::NumberUnit::EtherKeyword,
            input::NumberUnit::SecondsKeyword => output::NumberUnit::SecondsKeyword,
            input::NumberUnit::MinutesKeyword => output::NumberUnit::MinutesKeyword,
            input::NumberUnit::HoursKeyword => output::NumberUnit::HoursKeyword,
            input::NumberUnit::DaysKeyword => output::NumberUnit::DaysKeyword,
            input::NumberUnit::WeeksKeyword => output::NumberUnit::WeeksKeyword,
            input::NumberUnit::YearsKeyword => output::NumberUnit::YearsKeyword,
        }
    }
    fn transform_number_unit(&mut self, source: &input::NumberUnit) -> output::NumberUnit {
        self.default_transform_number_unit(source)
    }

    fn default_transform_string_expression(
        &mut self,
        source: &input::StringExpression,
    ) -> output::StringExpression {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::StringExpression::StringLiteral(ref string_literal) => {
                output::StringExpression::StringLiteral(
                    self.transform_string_literal(string_literal),
                )
            }
            input::StringExpression::StringLiterals(ref string_literals) => {
                output::StringExpression::StringLiterals(
                    self.transform_string_literals(string_literals),
                )
            }
            input::StringExpression::HexStringLiteral(ref hex_string_literal) => {
                output::StringExpression::HexStringLiteral(
                    self.transform_hex_string_literal(hex_string_literal),
                )
            }
            input::StringExpression::HexStringLiterals(ref hex_string_literals) => {
                output::StringExpression::HexStringLiterals(
                    self.transform_hex_string_literals(hex_string_literals),
                )
            }
            input::StringExpression::UnicodeStringLiterals(ref unicode_string_literals) => {
                output::StringExpression::UnicodeStringLiterals(
                    self.transform_unicode_string_literals(unicode_string_literals),
                )
            }
        }
    }
    fn transform_string_expression(
        &mut self,
        source: &input::StringExpression,
    ) -> output::StringExpression {
        self.default_transform_string_expression(source)
    }

    fn default_transform_string_literal(
        &mut self,
        source: &input::StringLiteral,
    ) -> output::StringLiteral {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::StringLiteral::SingleQuotedStringLiteral(node) => {
                output::StringLiteral::SingleQuotedStringLiteral(Rc::clone(node))
            }
            input::StringLiteral::DoubleQuotedStringLiteral(node) => {
                output::StringLiteral::DoubleQuotedStringLiteral(Rc::clone(node))
            }
        }
    }
    fn transform_string_literal(&mut self, source: &input::StringLiteral) -> output::StringLiteral {
        self.default_transform_string_literal(source)
    }

    fn default_transform_hex_string_literal(
        &mut self,
        source: &input::HexStringLiteral,
    ) -> output::HexStringLiteral {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::HexStringLiteral::SingleQuotedHexStringLiteral(node) => {
                output::HexStringLiteral::SingleQuotedHexStringLiteral(Rc::clone(node))
            }
            input::HexStringLiteral::DoubleQuotedHexStringLiteral(node) => {
                output::HexStringLiteral::DoubleQuotedHexStringLiteral(Rc::clone(node))
            }
        }
    }
    fn transform_hex_string_literal(
        &mut self,
        source: &input::HexStringLiteral,
    ) -> output::HexStringLiteral {
        self.default_transform_hex_string_literal(source)
    }

    fn default_transform_unicode_string_literal(
        &mut self,
        source: &input::UnicodeStringLiteral,
    ) -> output::UnicodeStringLiteral {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(node) => {
                output::UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(Rc::clone(node))
            }
            input::UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(node) => {
                output::UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(Rc::clone(node))
            }
        }
    }
    fn transform_unicode_string_literal(
        &mut self,
        source: &input::UnicodeStringLiteral,
    ) -> output::UnicodeStringLiteral {
        self.default_transform_unicode_string_literal(source)
    }

    fn default_transform_yul_statement(
        &mut self,
        source: &input::YulStatement,
    ) -> output::YulStatement {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::YulStatement::YulBlock(ref yul_block) => {
                output::YulStatement::YulBlock(self.transform_yul_block(yul_block))
            }
            input::YulStatement::YulFunctionDefinition(ref yul_function_definition) => {
                output::YulStatement::YulFunctionDefinition(
                    self.transform_yul_function_definition(yul_function_definition),
                )
            }
            input::YulStatement::YulStackAssignmentStatement(
                ref yul_stack_assignment_statement,
            ) => output::YulStatement::YulStackAssignmentStatement(
                self.transform_yul_stack_assignment_statement(yul_stack_assignment_statement),
            ),
            input::YulStatement::YulIfStatement(ref yul_if_statement) => {
                output::YulStatement::YulIfStatement(
                    self.transform_yul_if_statement(yul_if_statement),
                )
            }
            input::YulStatement::YulForStatement(ref yul_for_statement) => {
                output::YulStatement::YulForStatement(
                    self.transform_yul_for_statement(yul_for_statement),
                )
            }
            input::YulStatement::YulSwitchStatement(ref yul_switch_statement) => {
                output::YulStatement::YulSwitchStatement(
                    self.transform_yul_switch_statement(yul_switch_statement),
                )
            }
            input::YulStatement::YulLeaveStatement(ref yul_leave_statement) => {
                output::YulStatement::YulLeaveStatement(
                    self.transform_yul_leave_statement(yul_leave_statement),
                )
            }
            input::YulStatement::YulBreakStatement(ref yul_break_statement) => {
                output::YulStatement::YulBreakStatement(
                    self.transform_yul_break_statement(yul_break_statement),
                )
            }
            input::YulStatement::YulContinueStatement(ref yul_continue_statement) => {
                output::YulStatement::YulContinueStatement(
                    self.transform_yul_continue_statement(yul_continue_statement),
                )
            }
            input::YulStatement::YulVariableAssignmentStatement(
                ref yul_variable_assignment_statement,
            ) => output::YulStatement::YulVariableAssignmentStatement(
                self.transform_yul_variable_assignment_statement(yul_variable_assignment_statement),
            ),
            input::YulStatement::YulLabel(ref yul_label) => {
                output::YulStatement::YulLabel(self.transform_yul_label(yul_label))
            }
            input::YulStatement::YulVariableDeclarationStatement(
                ref yul_variable_declaration_statement,
            ) => output::YulStatement::YulVariableDeclarationStatement(
                self.transform_yul_variable_declaration_statement(
                    yul_variable_declaration_statement,
                ),
            ),
            input::YulStatement::YulExpression(ref yul_expression) => {
                output::YulStatement::YulExpression(self.transform_yul_expression(yul_expression))
            }
        }
    }
    fn transform_yul_statement(&mut self, source: &input::YulStatement) -> output::YulStatement {
        self.default_transform_yul_statement(source)
    }

    fn default_transform_yul_assignment_operator(
        &mut self,
        source: &input::YulAssignmentOperator,
    ) -> output::YulAssignmentOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::YulAssignmentOperator::YulColonAndEqual(ref yul_colon_and_equal) => {
                output::YulAssignmentOperator::YulColonAndEqual(
                    self.transform_yul_colon_and_equal(yul_colon_and_equal),
                )
            }
            input::YulAssignmentOperator::ColonEqual => output::YulAssignmentOperator::ColonEqual,
        }
    }
    fn transform_yul_assignment_operator(
        &mut self,
        source: &input::YulAssignmentOperator,
    ) -> output::YulAssignmentOperator {
        self.default_transform_yul_assignment_operator(source)
    }

    fn default_transform_yul_stack_assignment_operator(
        &mut self,
        source: &input::YulStackAssignmentOperator,
    ) -> output::YulStackAssignmentOperator {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::YulStackAssignmentOperator::YulEqualAndColon(ref yul_equal_and_colon) => {
                output::YulStackAssignmentOperator::YulEqualAndColon(
                    self.transform_yul_equal_and_colon(yul_equal_and_colon),
                )
            }
            input::YulStackAssignmentOperator::EqualColon => {
                output::YulStackAssignmentOperator::EqualColon
            }
        }
    }
    fn transform_yul_stack_assignment_operator(
        &mut self,
        source: &input::YulStackAssignmentOperator,
    ) -> output::YulStackAssignmentOperator {
        self.default_transform_yul_stack_assignment_operator(source)
    }

    fn default_transform_yul_switch_case(
        &mut self,
        source: &input::YulSwitchCase,
    ) -> output::YulSwitchCase {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::YulSwitchCase::YulDefaultCase(ref yul_default_case) => {
                output::YulSwitchCase::YulDefaultCase(
                    self.transform_yul_default_case(yul_default_case),
                )
            }
            input::YulSwitchCase::YulValueCase(ref yul_value_case) => {
                output::YulSwitchCase::YulValueCase(self.transform_yul_value_case(yul_value_case))
            }
        }
    }
    fn transform_yul_switch_case(
        &mut self,
        source: &input::YulSwitchCase,
    ) -> output::YulSwitchCase {
        self.default_transform_yul_switch_case(source)
    }

    fn default_transform_yul_expression(
        &mut self,
        source: &input::YulExpression,
    ) -> output::YulExpression {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::YulExpression::YulFunctionCallExpression(ref yul_function_call_expression) => {
                output::YulExpression::YulFunctionCallExpression(
                    self.transform_yul_function_call_expression(yul_function_call_expression),
                )
            }
            input::YulExpression::YulLiteral(ref yul_literal) => {
                output::YulExpression::YulLiteral(self.transform_yul_literal(yul_literal))
            }
            input::YulExpression::YulPath(ref yul_path) => {
                output::YulExpression::YulPath(self.transform_yul_path(yul_path))
            }
        }
    }
    fn transform_yul_expression(&mut self, source: &input::YulExpression) -> output::YulExpression {
        self.default_transform_yul_expression(source)
    }

    fn default_transform_yul_literal(&mut self, source: &input::YulLiteral) -> output::YulLiteral {
        #[allow(clippy::match_wildcard_for_single_variants)]
        match source {
            input::YulLiteral::HexStringLiteral(ref hex_string_literal) => {
                output::YulLiteral::HexStringLiteral(
                    self.transform_hex_string_literal(hex_string_literal),
                )
            }
            input::YulLiteral::StringLiteral(ref string_literal) => {
                output::YulLiteral::StringLiteral(self.transform_string_literal(string_literal))
            }
            input::YulLiteral::YulDecimalLiteral(node) => {
                output::YulLiteral::YulDecimalLiteral(Rc::clone(node))
            }
            input::YulLiteral::YulHexLiteral(node) => {
                output::YulLiteral::YulHexLiteral(Rc::clone(node))
            }
            input::YulLiteral::YulTrueKeyword => output::YulLiteral::YulTrueKeyword,
            input::YulLiteral::YulFalseKeyword => output::YulLiteral::YulFalseKeyword,
        }
    }
    fn transform_yul_literal(&mut self, source: &input::YulLiteral) -> output::YulLiteral {
        self.default_transform_yul_literal(source)
    }

    //
    // Repeated & Separated
    //

    fn transform_source_unit_members(
        &mut self,
        source: &input::SourceUnitMembers,
    ) -> output::SourceUnitMembers {
        source
            .iter()
            .map(|item| self.transform_source_unit_member(item))
            .collect()
    }

    fn transform_version_expression_sets(
        &mut self,
        source: &input::VersionExpressionSets,
    ) -> output::VersionExpressionSets {
        source
            .iter()
            .map(|item| self.transform_version_expression_set(item))
            .collect()
    }

    fn transform_version_expression_set(
        &mut self,
        source: &input::VersionExpressionSet,
    ) -> output::VersionExpressionSet {
        source
            .iter()
            .map(|item| self.transform_version_expression(item))
            .collect()
    }

    fn transform_simple_version_literal(
        &mut self,
        source: &input::SimpleVersionLiteral,
    ) -> output::SimpleVersionLiteral {
        source.iter().map(Rc::clone).collect()
    }

    fn transform_import_deconstruction_symbols(
        &mut self,
        source: &input::ImportDeconstructionSymbols,
    ) -> output::ImportDeconstructionSymbols {
        source
            .iter()
            .map(|item| self.transform_import_deconstruction_symbol(item))
            .collect()
    }

    fn transform_using_deconstruction_symbols(
        &mut self,
        source: &input::UsingDeconstructionSymbols,
    ) -> output::UsingDeconstructionSymbols {
        source
            .iter()
            .map(|item| self.transform_using_deconstruction_symbol(item))
            .collect()
    }

    fn transform_inheritance_types(
        &mut self,
        source: &input::InheritanceTypes,
    ) -> output::InheritanceTypes {
        source
            .iter()
            .map(|item| self.transform_inheritance_type(item))
            .collect()
    }

    fn transform_contract_members(
        &mut self,
        source: &input::ContractMembers,
    ) -> output::ContractMembers {
        source
            .iter()
            .map(|item| self.transform_contract_member(item))
            .collect()
    }

    fn transform_interface_members(
        &mut self,
        source: &input::InterfaceMembers,
    ) -> output::InterfaceMembers {
        source
            .iter()
            .map(|item| self.transform_contract_member(item))
            .collect()
    }

    fn transform_library_members(
        &mut self,
        source: &input::LibraryMembers,
    ) -> output::LibraryMembers {
        source
            .iter()
            .map(|item| self.transform_contract_member(item))
            .collect()
    }

    fn transform_struct_members(&mut self, source: &input::StructMembers) -> output::StructMembers {
        source
            .iter()
            .map(|item| self.transform_struct_member(item))
            .collect()
    }

    fn transform_enum_members(&mut self, source: &input::EnumMembers) -> output::EnumMembers {
        source.iter().map(Rc::clone).collect()
    }

    fn transform_state_variable_attributes(
        &mut self,
        source: &input::StateVariableAttributes,
    ) -> output::StateVariableAttributes {
        source
            .iter()
            .map(|item| self.transform_state_variable_attribute(item))
            .collect()
    }

    fn transform_parameters(&mut self, source: &input::Parameters) -> output::Parameters {
        source
            .iter()
            .map(|item| self.transform_parameter(item))
            .collect()
    }

    fn transform_function_attributes(
        &mut self,
        source: &input::FunctionAttributes,
    ) -> output::FunctionAttributes {
        source
            .iter()
            .map(|item| self.transform_function_attribute(item))
            .collect()
    }

    fn transform_override_paths(&mut self, source: &input::OverridePaths) -> output::OverridePaths {
        source
            .iter()
            .map(|item| self.transform_identifier_path(item))
            .collect()
    }

    fn transform_event_parameters(
        &mut self,
        source: &input::EventParameters,
    ) -> output::EventParameters {
        source
            .iter()
            .map(|item| self.transform_event_parameter(item))
            .collect()
    }

    fn transform_error_parameters(
        &mut self,
        source: &input::ErrorParameters,
    ) -> output::ErrorParameters {
        source
            .iter()
            .map(|item| self.transform_error_parameter(item))
            .collect()
    }

    fn transform_function_type_attributes(
        &mut self,
        source: &input::FunctionTypeAttributes,
    ) -> output::FunctionTypeAttributes {
        source
            .iter()
            .map(|item| self.transform_function_type_attribute(item))
            .collect()
    }

    fn transform_statements(&mut self, source: &input::Statements) -> output::Statements {
        source
            .iter()
            .map(|item| self.transform_statement(item))
            .collect()
    }

    fn transform_assembly_flags(&mut self, source: &input::AssemblyFlags) -> output::AssemblyFlags {
        source
            .iter()
            .map(|item| self.transform_string_literal(item))
            .collect()
    }

    fn transform_tuple_deconstruction_elements(
        &mut self,
        source: &input::TupleDeconstructionElements,
    ) -> output::TupleDeconstructionElements {
        source
            .iter()
            .map(|item| self.transform_tuple_deconstruction_element(item))
            .collect()
    }

    fn transform_catch_clauses(&mut self, source: &input::CatchClauses) -> output::CatchClauses {
        source
            .iter()
            .map(|item| self.transform_catch_clause(item))
            .collect()
    }

    fn transform_positional_arguments(
        &mut self,
        source: &input::PositionalArguments,
    ) -> output::PositionalArguments {
        source
            .iter()
            .map(|item| self.transform_expression(item))
            .collect()
    }

    fn transform_named_arguments(
        &mut self,
        source: &input::NamedArguments,
    ) -> output::NamedArguments {
        source
            .iter()
            .map(|item| self.transform_named_argument(item))
            .collect()
    }

    fn transform_call_options(&mut self, source: &input::CallOptions) -> output::CallOptions {
        source
            .iter()
            .map(|item| self.transform_named_argument(item))
            .collect()
    }

    fn transform_tuple_values(&mut self, source: &input::TupleValues) -> output::TupleValues {
        source
            .iter()
            .map(|item| self.transform_tuple_value(item))
            .collect()
    }

    fn transform_array_values(&mut self, source: &input::ArrayValues) -> output::ArrayValues {
        source
            .iter()
            .map(|item| self.transform_expression(item))
            .collect()
    }

    fn transform_string_literals(
        &mut self,
        source: &input::StringLiterals,
    ) -> output::StringLiterals {
        source
            .iter()
            .map(|item| self.transform_string_literal(item))
            .collect()
    }

    fn transform_hex_string_literals(
        &mut self,
        source: &input::HexStringLiterals,
    ) -> output::HexStringLiterals {
        source
            .iter()
            .map(|item| self.transform_hex_string_literal(item))
            .collect()
    }

    fn transform_unicode_string_literals(
        &mut self,
        source: &input::UnicodeStringLiterals,
    ) -> output::UnicodeStringLiterals {
        source
            .iter()
            .map(|item| self.transform_unicode_string_literal(item))
            .collect()
    }

    fn transform_identifier_path(
        &mut self,
        source: &input::IdentifierPath,
    ) -> output::IdentifierPath {
        source.iter().map(Rc::clone).collect()
    }

    fn transform_yul_statements(&mut self, source: &input::YulStatements) -> output::YulStatements {
        source
            .iter()
            .map(|item| self.transform_yul_statement(item))
            .collect()
    }

    fn transform_yul_parameters(&mut self, source: &input::YulParameters) -> output::YulParameters {
        source.iter().map(Rc::clone).collect()
    }

    fn transform_yul_variable_names(
        &mut self,
        source: &input::YulVariableNames,
    ) -> output::YulVariableNames {
        source.iter().map(Rc::clone).collect()
    }

    fn transform_yul_switch_cases(
        &mut self,
        source: &input::YulSwitchCases,
    ) -> output::YulSwitchCases {
        source
            .iter()
            .map(|item| self.transform_yul_switch_case(item))
            .collect()
    }

    fn transform_yul_arguments(&mut self, source: &input::YulArguments) -> output::YulArguments {
        source
            .iter()
            .map(|item| self.transform_yul_expression(item))
            .collect()
    }

    fn transform_yul_paths(&mut self, source: &input::YulPaths) -> output::YulPaths {
        source
            .iter()
            .map(|item| self.transform_yul_path(item))
            .collect()
    }

    fn transform_yul_path(&mut self, source: &input::YulPath) -> output::YulPath {
        source.iter().map(Rc::clone).collect()
    }
}
