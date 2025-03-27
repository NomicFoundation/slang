// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;

pub trait Mutator {
    //
    // Sequences:
    //

    fn mutate_source_unit(&mut self, source: &SourceUnit) -> SourceUnit {
        let members = self.mutate_source_unit_members(&source.members);

        Rc::new(SourceUnitStruct {
            node_id: source.node_id,
            members,
        })
    }

    fn mutate_pragma_directive(&mut self, source: &PragmaDirective) -> PragmaDirective {
        let pragma = self.mutate_pragma(&source.pragma);

        Rc::new(PragmaDirectiveStruct {
            node_id: source.node_id,
            pragma,
        })
    }

    fn mutate_abicoder_pragma(&mut self, source: &AbicoderPragma) -> AbicoderPragma {
        let version = Rc::clone(&source.version);

        Rc::new(AbicoderPragmaStruct {
            node_id: source.node_id,
            version,
        })
    }

    fn mutate_experimental_pragma(&mut self, source: &ExperimentalPragma) -> ExperimentalPragma {
        let feature = self.mutate_experimental_feature(&source.feature);

        Rc::new(ExperimentalPragmaStruct {
            node_id: source.node_id,
            feature,
        })
    }

    fn mutate_version_pragma(&mut self, source: &VersionPragma) -> VersionPragma {
        let sets = self.mutate_version_expression_sets(&source.sets);

        Rc::new(VersionPragmaStruct {
            node_id: source.node_id,
            sets,
        })
    }

    fn mutate_version_range(&mut self, source: &VersionRange) -> VersionRange {
        let start = self.mutate_version_literal(&source.start);
        let end = self.mutate_version_literal(&source.end);

        Rc::new(VersionRangeStruct {
            node_id: source.node_id,
            start,
            end,
        })
    }

    fn mutate_version_term(&mut self, source: &VersionTerm) -> VersionTerm {
        let operator = source
            .operator
            .as_ref()
            .map(|value| self.mutate_version_operator(value));
        let literal = self.mutate_version_literal(&source.literal);

        Rc::new(VersionTermStruct {
            node_id: source.node_id,
            operator,
            literal,
        })
    }

    fn mutate_import_directive(&mut self, source: &ImportDirective) -> ImportDirective {
        let clause = self.mutate_import_clause(&source.clause);

        Rc::new(ImportDirectiveStruct {
            node_id: source.node_id,
            clause,
        })
    }

    fn mutate_path_import(&mut self, source: &PathImport) -> PathImport {
        let path = self.mutate_string_literal(&source.path);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.mutate_import_alias(value));

        Rc::new(PathImportStruct {
            node_id: source.node_id,
            path,
            alias,
        })
    }

    fn mutate_named_import(&mut self, source: &NamedImport) -> NamedImport {
        let alias = self.mutate_import_alias(&source.alias);
        let path = self.mutate_string_literal(&source.path);

        Rc::new(NamedImportStruct {
            node_id: source.node_id,
            alias,
            path,
        })
    }

    fn mutate_import_deconstruction(
        &mut self,
        source: &ImportDeconstruction,
    ) -> ImportDeconstruction {
        let symbols = self.mutate_import_deconstruction_symbols(&source.symbols);
        let path = self.mutate_string_literal(&source.path);

        Rc::new(ImportDeconstructionStruct {
            node_id: source.node_id,
            symbols,
            path,
        })
    }

    fn mutate_import_deconstruction_symbol(
        &mut self,
        source: &ImportDeconstructionSymbol,
    ) -> ImportDeconstructionSymbol {
        let name = Rc::clone(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.mutate_import_alias(value));

        Rc::new(ImportDeconstructionSymbolStruct {
            node_id: source.node_id,
            name,
            alias,
        })
    }

    fn mutate_import_alias(&mut self, source: &ImportAlias) -> ImportAlias {
        let identifier = Rc::clone(&source.identifier);

        Rc::new(ImportAliasStruct {
            node_id: source.node_id,
            identifier,
        })
    }

    fn mutate_using_directive(&mut self, source: &UsingDirective) -> UsingDirective {
        let clause = self.mutate_using_clause(&source.clause);
        let target = self.mutate_using_target(&source.target);
        let global_keyword = source.global_keyword.as_ref().map(Rc::clone);

        Rc::new(UsingDirectiveStruct {
            node_id: source.node_id,
            clause,
            target,
            global_keyword,
        })
    }

    fn mutate_using_deconstruction(&mut self, source: &UsingDeconstruction) -> UsingDeconstruction {
        let symbols = self.mutate_using_deconstruction_symbols(&source.symbols);

        Rc::new(UsingDeconstructionStruct {
            node_id: source.node_id,
            symbols,
        })
    }

    fn mutate_using_deconstruction_symbol(
        &mut self,
        source: &UsingDeconstructionSymbol,
    ) -> UsingDeconstructionSymbol {
        let name = self.mutate_identifier_path(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.mutate_using_alias(value));

        Rc::new(UsingDeconstructionSymbolStruct {
            node_id: source.node_id,
            name,
            alias,
        })
    }

    fn mutate_using_alias(&mut self, source: &UsingAlias) -> UsingAlias {
        let operator = self.mutate_using_operator(&source.operator);

        Rc::new(UsingAliasStruct {
            node_id: source.node_id,
            operator,
        })
    }

    fn mutate_contract_definition(&mut self, source: &ContractDefinition) -> ContractDefinition {
        let abstract_keyword = source.abstract_keyword.as_ref().map(Rc::clone);
        let name = Rc::clone(&source.name);
        let members = self.mutate_contract_members(&source.members);
        let inheritance_types = self.mutate_inheritance_types(&source.inheritance_types);

        Rc::new(ContractDefinitionStruct {
            node_id: source.node_id,
            abstract_keyword,
            name,
            members,
            inheritance_types,
        })
    }

    fn mutate_inheritance_specifier(
        &mut self,
        source: &InheritanceSpecifier,
    ) -> InheritanceSpecifier {
        let types = self.mutate_inheritance_types(&source.types);

        Rc::new(InheritanceSpecifierStruct {
            node_id: source.node_id,
            types,
        })
    }

    fn mutate_inheritance_type(&mut self, source: &InheritanceType) -> InheritanceType {
        let type_name = self.mutate_identifier_path(&source.type_name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.mutate_arguments_declaration(value));

        Rc::new(InheritanceTypeStruct {
            node_id: source.node_id,
            type_name,
            arguments,
        })
    }

    fn mutate_interface_definition(&mut self, source: &InterfaceDefinition) -> InterfaceDefinition {
        let name = Rc::clone(&source.name);
        let inheritance = source
            .inheritance
            .as_ref()
            .map(|value| self.mutate_inheritance_specifier(value));
        let members = self.mutate_interface_members(&source.members);

        Rc::new(InterfaceDefinitionStruct {
            node_id: source.node_id,
            name,
            inheritance,
            members,
        })
    }

    fn mutate_library_definition(&mut self, source: &LibraryDefinition) -> LibraryDefinition {
        let name = Rc::clone(&source.name);
        let members = self.mutate_library_members(&source.members);

        Rc::new(LibraryDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn mutate_struct_definition(&mut self, source: &StructDefinition) -> StructDefinition {
        let name = Rc::clone(&source.name);
        let members = self.mutate_struct_members(&source.members);

        Rc::new(StructDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn mutate_struct_member(&mut self, source: &StructMember) -> StructMember {
        let type_name = self.mutate_type_name(&source.type_name);
        let name = Rc::clone(&source.name);

        Rc::new(StructMemberStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn mutate_enum_definition(&mut self, source: &EnumDefinition) -> EnumDefinition {
        let name = Rc::clone(&source.name);
        let members = self.mutate_enum_members(&source.members);

        Rc::new(EnumDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn mutate_constant_definition(&mut self, source: &ConstantDefinition) -> ConstantDefinition {
        let type_name = self.mutate_type_name(&source.type_name);
        let name = Rc::clone(&source.name);
        let value = self.mutate_expression(&source.value);

        Rc::new(ConstantDefinitionStruct {
            node_id: source.node_id,
            type_name,
            name,
            value,
        })
    }

    fn mutate_state_variable_definition(
        &mut self,
        source: &StateVariableDefinition,
    ) -> StateVariableDefinition {
        let type_name = self.mutate_type_name(&source.type_name);
        let attributes = self.mutate_state_variable_attributes(&source.attributes);
        let name = Rc::clone(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.mutate_state_variable_definition_value(value));

        Rc::new(StateVariableDefinitionStruct {
            node_id: source.node_id,
            type_name,
            attributes,
            name,
            value,
        })
    }

    fn mutate_state_variable_definition_value(
        &mut self,
        source: &StateVariableDefinitionValue,
    ) -> StateVariableDefinitionValue {
        let value = self.mutate_expression(&source.value);

        Rc::new(StateVariableDefinitionValueStruct {
            node_id: source.node_id,
            value,
        })
    }

    fn mutate_function_definition(&mut self, source: &FunctionDefinition) -> FunctionDefinition {
        let name = self.mutate_function_name(&source.name);
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_function_attributes(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_returns_declaration(value));
        let body = self.mutate_function_body(&source.body);

        Rc::new(FunctionDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            attributes,
            returns,
            body,
        })
    }

    fn mutate_parameters_declaration(
        &mut self,
        source: &ParametersDeclaration,
    ) -> ParametersDeclaration {
        let parameters = self.mutate_parameters(&source.parameters);

        Rc::new(ParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn mutate_parameter(&mut self, source: &Parameter) -> Parameter {
        let type_name = self.mutate_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.mutate_storage_location(value));
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(ParameterStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
        })
    }

    fn mutate_override_specifier(&mut self, source: &OverrideSpecifier) -> OverrideSpecifier {
        let overridden = source
            .overridden
            .as_ref()
            .map(|value| self.mutate_override_paths_declaration(value));

        Rc::new(OverrideSpecifierStruct {
            node_id: source.node_id,
            overridden,
        })
    }

    fn mutate_override_paths_declaration(
        &mut self,
        source: &OverridePathsDeclaration,
    ) -> OverridePathsDeclaration {
        let paths = self.mutate_override_paths(&source.paths);

        Rc::new(OverridePathsDeclarationStruct {
            node_id: source.node_id,
            paths,
        })
    }

    fn mutate_returns_declaration(&mut self, source: &ReturnsDeclaration) -> ReturnsDeclaration {
        let variables = self.mutate_parameters_declaration(&source.variables);

        Rc::new(ReturnsDeclarationStruct {
            node_id: source.node_id,
            variables,
        })
    }

    fn mutate_constructor_definition(
        &mut self,
        source: &ConstructorDefinition,
    ) -> ConstructorDefinition {
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_constructor_attributes(&source.attributes);
        let body = self.mutate_block(&source.body);

        Rc::new(ConstructorDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            body,
        })
    }

    fn mutate_fallback_function_definition(
        &mut self,
        source: &FallbackFunctionDefinition,
    ) -> FallbackFunctionDefinition {
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_fallback_function_attributes(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_returns_declaration(value));
        let body = self.mutate_function_body(&source.body);

        Rc::new(FallbackFunctionDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            returns,
            body,
        })
    }

    fn mutate_receive_function_definition(
        &mut self,
        source: &ReceiveFunctionDefinition,
    ) -> ReceiveFunctionDefinition {
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_receive_function_attributes(&source.attributes);
        let body = self.mutate_function_body(&source.body);

        Rc::new(ReceiveFunctionDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            body,
        })
    }

    fn mutate_modifier_definition(&mut self, source: &ModifierDefinition) -> ModifierDefinition {
        let name = Rc::clone(&source.name);
        let parameters = source
            .parameters
            .as_ref()
            .map(|value| self.mutate_parameters_declaration(value));
        let attributes = self.mutate_modifier_attributes(&source.attributes);
        let body = self.mutate_function_body(&source.body);

        Rc::new(ModifierDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            attributes,
            body,
        })
    }

    fn mutate_modifier_invocation(&mut self, source: &ModifierInvocation) -> ModifierInvocation {
        let name = self.mutate_identifier_path(&source.name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.mutate_arguments_declaration(value));

        Rc::new(ModifierInvocationStruct {
            node_id: source.node_id,
            name,
            arguments,
        })
    }

    fn mutate_event_definition(&mut self, source: &EventDefinition) -> EventDefinition {
        let name = Rc::clone(&source.name);
        let parameters = self.mutate_event_parameters_declaration(&source.parameters);
        let anonymous_keyword = source.anonymous_keyword.as_ref().map(Rc::clone);

        Rc::new(EventDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            anonymous_keyword,
        })
    }

    fn mutate_event_parameters_declaration(
        &mut self,
        source: &EventParametersDeclaration,
    ) -> EventParametersDeclaration {
        let parameters = self.mutate_event_parameters(&source.parameters);

        Rc::new(EventParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn mutate_event_parameter(&mut self, source: &EventParameter) -> EventParameter {
        let type_name = self.mutate_type_name(&source.type_name);
        let indexed_keyword = source.indexed_keyword.as_ref().map(Rc::clone);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(EventParameterStruct {
            node_id: source.node_id,
            type_name,
            indexed_keyword,
            name,
        })
    }

    fn mutate_user_defined_value_type_definition(
        &mut self,
        source: &UserDefinedValueTypeDefinition,
    ) -> UserDefinedValueTypeDefinition {
        let name = Rc::clone(&source.name);
        let value_type = self.mutate_elementary_type(&source.value_type);

        Rc::new(UserDefinedValueTypeDefinitionStruct {
            node_id: source.node_id,
            name,
            value_type,
        })
    }

    fn mutate_error_definition(&mut self, source: &ErrorDefinition) -> ErrorDefinition {
        let name = Rc::clone(&source.name);
        let members = self.mutate_error_parameters_declaration(&source.members);

        Rc::new(ErrorDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn mutate_error_parameters_declaration(
        &mut self,
        source: &ErrorParametersDeclaration,
    ) -> ErrorParametersDeclaration {
        let parameters = self.mutate_error_parameters(&source.parameters);

        Rc::new(ErrorParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn mutate_error_parameter(&mut self, source: &ErrorParameter) -> ErrorParameter {
        let type_name = self.mutate_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(ErrorParameterStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn mutate_array_type_name(&mut self, source: &ArrayTypeName) -> ArrayTypeName {
        let operand = self.mutate_type_name(&source.operand);
        let index = source
            .index
            .as_ref()
            .map(|value| self.mutate_expression(value));

        Rc::new(ArrayTypeNameStruct {
            node_id: source.node_id,
            operand,
            index,
        })
    }

    fn mutate_function_type(&mut self, source: &FunctionType) -> FunctionType {
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_function_type_attributes(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_returns_declaration(value));

        Rc::new(FunctionTypeStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            returns,
        })
    }

    fn mutate_mapping_type(&mut self, source: &MappingType) -> MappingType {
        let key_type = self.mutate_mapping_key(&source.key_type);
        let value_type = self.mutate_mapping_value(&source.value_type);

        Rc::new(MappingTypeStruct {
            node_id: source.node_id,
            key_type,
            value_type,
        })
    }

    fn mutate_mapping_key(&mut self, source: &MappingKey) -> MappingKey {
        let key_type = self.mutate_mapping_key_type(&source.key_type);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(MappingKeyStruct {
            node_id: source.node_id,
            key_type,
            name,
        })
    }

    fn mutate_mapping_value(&mut self, source: &MappingValue) -> MappingValue {
        let type_name = self.mutate_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(MappingValueStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn mutate_address_type(&mut self, source: &AddressType) -> AddressType {
        let payable_keyword = source.payable_keyword.as_ref().map(Rc::clone);

        Rc::new(AddressTypeStruct {
            node_id: source.node_id,
            payable_keyword,
        })
    }

    fn mutate_block(&mut self, source: &Block) -> Block {
        let statements = self.mutate_statements(&source.statements);

        Rc::new(BlockStruct {
            node_id: source.node_id,
            statements,
        })
    }

    fn mutate_unchecked_block(&mut self, source: &UncheckedBlock) -> UncheckedBlock {
        let block = self.mutate_block(&source.block);

        Rc::new(UncheckedBlockStruct {
            node_id: source.node_id,
            block,
        })
    }

    fn mutate_expression_statement(&mut self, source: &ExpressionStatement) -> ExpressionStatement {
        let expression = self.mutate_expression(&source.expression);

        Rc::new(ExpressionStatementStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn mutate_assembly_statement(&mut self, source: &AssemblyStatement) -> AssemblyStatement {
        let label = source
            .label
            .as_ref()
            .map(|value| self.mutate_string_literal(value));
        let flags = source
            .flags
            .as_ref()
            .map(|value| self.mutate_assembly_flags_declaration(value));
        let body = self.mutate_yul_block(&source.body);

        Rc::new(AssemblyStatementStruct {
            node_id: source.node_id,
            label,
            flags,
            body,
        })
    }

    fn mutate_assembly_flags_declaration(
        &mut self,
        source: &AssemblyFlagsDeclaration,
    ) -> AssemblyFlagsDeclaration {
        let flags = self.mutate_assembly_flags(&source.flags);

        Rc::new(AssemblyFlagsDeclarationStruct {
            node_id: source.node_id,
            flags,
        })
    }

    fn mutate_tuple_deconstruction_statement(
        &mut self,
        source: &TupleDeconstructionStatement,
    ) -> TupleDeconstructionStatement {
        let var_keyword = source.var_keyword.as_ref().map(Rc::clone);
        let elements = self.mutate_tuple_deconstruction_elements(&source.elements);
        let expression = self.mutate_expression(&source.expression);

        Rc::new(TupleDeconstructionStatementStruct {
            node_id: source.node_id,
            var_keyword,
            elements,
            expression,
        })
    }

    fn mutate_tuple_deconstruction_element(
        &mut self,
        source: &TupleDeconstructionElement,
    ) -> TupleDeconstructionElement {
        let member = source
            .member
            .as_ref()
            .map(|value| self.mutate_tuple_member(value));

        Rc::new(TupleDeconstructionElementStruct {
            node_id: source.node_id,
            member,
        })
    }

    fn mutate_typed_tuple_member(&mut self, source: &TypedTupleMember) -> TypedTupleMember {
        let type_name = self.mutate_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.mutate_storage_location(value));
        let name = Rc::clone(&source.name);

        Rc::new(TypedTupleMemberStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
        })
    }

    fn mutate_untyped_tuple_member(&mut self, source: &UntypedTupleMember) -> UntypedTupleMember {
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.mutate_storage_location(value));
        let name = Rc::clone(&source.name);

        Rc::new(UntypedTupleMemberStruct {
            node_id: source.node_id,
            storage_location,
            name,
        })
    }

    fn mutate_variable_declaration_statement(
        &mut self,
        source: &VariableDeclarationStatement,
    ) -> VariableDeclarationStatement {
        let variable_type = self.mutate_variable_declaration_type(&source.variable_type);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.mutate_storage_location(value));
        let name = Rc::clone(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.mutate_variable_declaration_value(value));

        Rc::new(VariableDeclarationStatementStruct {
            node_id: source.node_id,
            variable_type,
            storage_location,
            name,
            value,
        })
    }

    fn mutate_variable_declaration_value(
        &mut self,
        source: &VariableDeclarationValue,
    ) -> VariableDeclarationValue {
        let expression = self.mutate_expression(&source.expression);

        Rc::new(VariableDeclarationValueStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn mutate_if_statement(&mut self, source: &IfStatement) -> IfStatement {
        let condition = self.mutate_expression(&source.condition);
        let body = self.mutate_statement(&source.body);
        let else_branch = source
            .else_branch
            .as_ref()
            .map(|value| self.mutate_else_branch(value));

        Rc::new(IfStatementStruct {
            node_id: source.node_id,
            condition,
            body,
            else_branch,
        })
    }

    fn mutate_else_branch(&mut self, source: &ElseBranch) -> ElseBranch {
        let body = self.mutate_statement(&source.body);

        Rc::new(ElseBranchStruct {
            node_id: source.node_id,
            body,
        })
    }

    fn mutate_for_statement(&mut self, source: &ForStatement) -> ForStatement {
        let initialization = self.mutate_for_statement_initialization(&source.initialization);
        let condition = self.mutate_for_statement_condition(&source.condition);
        let iterator = source
            .iterator
            .as_ref()
            .map(|value| self.mutate_expression(value));
        let body = self.mutate_statement(&source.body);

        Rc::new(ForStatementStruct {
            node_id: source.node_id,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    fn mutate_while_statement(&mut self, source: &WhileStatement) -> WhileStatement {
        let condition = self.mutate_expression(&source.condition);
        let body = self.mutate_statement(&source.body);

        Rc::new(WhileStatementStruct {
            node_id: source.node_id,
            condition,
            body,
        })
    }

    fn mutate_do_while_statement(&mut self, source: &DoWhileStatement) -> DoWhileStatement {
        let body = self.mutate_statement(&source.body);
        let condition = self.mutate_expression(&source.condition);

        Rc::new(DoWhileStatementStruct {
            node_id: source.node_id,
            body,
            condition,
        })
    }

    fn mutate_continue_statement(&mut self, source: &ContinueStatement) -> ContinueStatement {
        Rc::new(ContinueStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_break_statement(&mut self, source: &BreakStatement) -> BreakStatement {
        Rc::new(BreakStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_return_statement(&mut self, source: &ReturnStatement) -> ReturnStatement {
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.mutate_expression(value));

        Rc::new(ReturnStatementStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn mutate_emit_statement(&mut self, source: &EmitStatement) -> EmitStatement {
        let event = self.mutate_identifier_path(&source.event);
        let arguments = self.mutate_arguments_declaration(&source.arguments);

        Rc::new(EmitStatementStruct {
            node_id: source.node_id,
            event,
            arguments,
        })
    }

    fn mutate_try_statement(&mut self, source: &TryStatement) -> TryStatement {
        let expression = self.mutate_expression(&source.expression);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_returns_declaration(value));
        let body = self.mutate_block(&source.body);
        let catch_clauses = self.mutate_catch_clauses(&source.catch_clauses);

        Rc::new(TryStatementStruct {
            node_id: source.node_id,
            expression,
            returns,
            body,
            catch_clauses,
        })
    }

    fn mutate_catch_clause(&mut self, source: &CatchClause) -> CatchClause {
        let error = source
            .error
            .as_ref()
            .map(|value| self.mutate_catch_clause_error(value));
        let body = self.mutate_block(&source.body);

        Rc::new(CatchClauseStruct {
            node_id: source.node_id,
            error,
            body,
        })
    }

    fn mutate_catch_clause_error(&mut self, source: &CatchClauseError) -> CatchClauseError {
        let name = source.name.as_ref().map(Rc::clone);
        let parameters = self.mutate_parameters_declaration(&source.parameters);

        Rc::new(CatchClauseErrorStruct {
            node_id: source.node_id,
            name,
            parameters,
        })
    }

    fn mutate_revert_statement(&mut self, source: &RevertStatement) -> RevertStatement {
        let error = source
            .error
            .as_ref()
            .map(|value| self.mutate_identifier_path(value));
        let arguments = self.mutate_arguments_declaration(&source.arguments);

        Rc::new(RevertStatementStruct {
            node_id: source.node_id,
            error,
            arguments,
        })
    }

    fn mutate_throw_statement(&mut self, source: &ThrowStatement) -> ThrowStatement {
        Rc::new(ThrowStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_assignment_expression(
        &mut self,
        source: &AssignmentExpression,
    ) -> AssignmentExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(AssignmentExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_conditional_expression(
        &mut self,
        source: &ConditionalExpression,
    ) -> ConditionalExpression {
        let operand = self.mutate_expression(&source.operand);
        let true_expression = self.mutate_expression(&source.true_expression);
        let false_expression = self.mutate_expression(&source.false_expression);

        Rc::new(ConditionalExpressionStruct {
            node_id: source.node_id,
            operand,
            true_expression,
            false_expression,
        })
    }

    fn mutate_or_expression(&mut self, source: &OrExpression) -> OrExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(OrExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_and_expression(&mut self, source: &AndExpression) -> AndExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(AndExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_equality_expression(&mut self, source: &EqualityExpression) -> EqualityExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(EqualityExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_inequality_expression(
        &mut self,
        source: &InequalityExpression,
    ) -> InequalityExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(InequalityExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_bitwise_or_expression(
        &mut self,
        source: &BitwiseOrExpression,
    ) -> BitwiseOrExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(BitwiseOrExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_bitwise_xor_expression(
        &mut self,
        source: &BitwiseXorExpression,
    ) -> BitwiseXorExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(BitwiseXorExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_bitwise_and_expression(
        &mut self,
        source: &BitwiseAndExpression,
    ) -> BitwiseAndExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(BitwiseAndExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_shift_expression(&mut self, source: &ShiftExpression) -> ShiftExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(ShiftExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_additive_expression(&mut self, source: &AdditiveExpression) -> AdditiveExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(AdditiveExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_multiplicative_expression(
        &mut self,
        source: &MultiplicativeExpression,
    ) -> MultiplicativeExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(MultiplicativeExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_exponentiation_expression(
        &mut self,
        source: &ExponentiationExpression,
    ) -> ExponentiationExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(ExponentiationExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_postfix_expression(&mut self, source: &PostfixExpression) -> PostfixExpression {
        let operand = self.mutate_expression(&source.operand);

        Rc::new(PostfixExpressionStruct {
            node_id: source.node_id,
            operand,
        })
    }

    fn mutate_prefix_expression(&mut self, source: &PrefixExpression) -> PrefixExpression {
        let operand = self.mutate_expression(&source.operand);

        Rc::new(PrefixExpressionStruct {
            node_id: source.node_id,
            operand,
        })
    }

    fn mutate_function_call_expression(
        &mut self,
        source: &FunctionCallExpression,
    ) -> FunctionCallExpression {
        let operand = self.mutate_expression(&source.operand);
        let arguments = self.mutate_arguments_declaration(&source.arguments);

        Rc::new(FunctionCallExpressionStruct {
            node_id: source.node_id,
            operand,
            arguments,
        })
    }

    fn mutate_call_options_expression(
        &mut self,
        source: &CallOptionsExpression,
    ) -> CallOptionsExpression {
        let operand = self.mutate_expression(&source.operand);
        let options = self.mutate_call_options(&source.options);

        Rc::new(CallOptionsExpressionStruct {
            node_id: source.node_id,
            operand,
            options,
        })
    }

    fn mutate_member_access_expression(
        &mut self,
        source: &MemberAccessExpression,
    ) -> MemberAccessExpression {
        let operand = self.mutate_expression(&source.operand);
        let member = Rc::clone(&source.member);

        Rc::new(MemberAccessExpressionStruct {
            node_id: source.node_id,
            operand,
            member,
        })
    }

    fn mutate_index_access_expression(
        &mut self,
        source: &IndexAccessExpression,
    ) -> IndexAccessExpression {
        let operand = self.mutate_expression(&source.operand);
        let start = source
            .start
            .as_ref()
            .map(|value| self.mutate_expression(value));
        let end = source
            .end
            .as_ref()
            .map(|value| self.mutate_index_access_end(value));

        Rc::new(IndexAccessExpressionStruct {
            node_id: source.node_id,
            operand,
            start,
            end,
        })
    }

    fn mutate_index_access_end(&mut self, source: &IndexAccessEnd) -> IndexAccessEnd {
        let end = source
            .end
            .as_ref()
            .map(|value| self.mutate_expression(value));

        Rc::new(IndexAccessEndStruct {
            node_id: source.node_id,
            end,
        })
    }

    fn mutate_positional_arguments_declaration(
        &mut self,
        source: &PositionalArgumentsDeclaration,
    ) -> PositionalArgumentsDeclaration {
        let arguments = self.mutate_positional_arguments(&source.arguments);

        Rc::new(PositionalArgumentsDeclarationStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn mutate_named_arguments_declaration(
        &mut self,
        source: &NamedArgumentsDeclaration,
    ) -> NamedArgumentsDeclaration {
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.mutate_named_argument_group(value));

        Rc::new(NamedArgumentsDeclarationStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn mutate_named_argument_group(&mut self, source: &NamedArgumentGroup) -> NamedArgumentGroup {
        let arguments = self.mutate_named_arguments(&source.arguments);

        Rc::new(NamedArgumentGroupStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn mutate_named_argument(&mut self, source: &NamedArgument) -> NamedArgument {
        let name = Rc::clone(&source.name);
        let value = self.mutate_expression(&source.value);

        Rc::new(NamedArgumentStruct {
            node_id: source.node_id,
            name,
            value,
        })
    }

    fn mutate_type_expression(&mut self, source: &TypeExpression) -> TypeExpression {
        let type_name = self.mutate_type_name(&source.type_name);

        Rc::new(TypeExpressionStruct {
            node_id: source.node_id,
            type_name,
        })
    }

    fn mutate_new_expression(&mut self, source: &NewExpression) -> NewExpression {
        let type_name = self.mutate_type_name(&source.type_name);

        Rc::new(NewExpressionStruct {
            node_id: source.node_id,
            type_name,
        })
    }

    fn mutate_tuple_expression(&mut self, source: &TupleExpression) -> TupleExpression {
        let items = self.mutate_tuple_values(&source.items);

        Rc::new(TupleExpressionStruct {
            node_id: source.node_id,
            items,
        })
    }

    fn mutate_tuple_value(&mut self, source: &TupleValue) -> TupleValue {
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.mutate_expression(value));

        Rc::new(TupleValueStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn mutate_array_expression(&mut self, source: &ArrayExpression) -> ArrayExpression {
        let items = self.mutate_array_values(&source.items);

        Rc::new(ArrayExpressionStruct {
            node_id: source.node_id,
            items,
        })
    }

    fn mutate_hex_number_expression(
        &mut self,
        source: &HexNumberExpression,
    ) -> HexNumberExpression {
        let literal = Rc::clone(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.mutate_number_unit(value));

        Rc::new(HexNumberExpressionStruct {
            node_id: source.node_id,
            literal,
            unit,
        })
    }

    fn mutate_decimal_number_expression(
        &mut self,
        source: &DecimalNumberExpression,
    ) -> DecimalNumberExpression {
        let literal = Rc::clone(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.mutate_number_unit(value));

        Rc::new(DecimalNumberExpressionStruct {
            node_id: source.node_id,
            literal,
            unit,
        })
    }

    fn mutate_yul_block(&mut self, source: &YulBlock) -> YulBlock {
        let statements = self.mutate_yul_statements(&source.statements);

        Rc::new(YulBlockStruct {
            node_id: source.node_id,
            statements,
        })
    }

    fn mutate_yul_function_definition(
        &mut self,
        source: &YulFunctionDefinition,
    ) -> YulFunctionDefinition {
        let name = Rc::clone(&source.name);
        let parameters = self.mutate_yul_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_yul_returns_declaration(value));
        let body = self.mutate_yul_block(&source.body);

        Rc::new(YulFunctionDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            returns,
            body,
        })
    }

    fn mutate_yul_parameters_declaration(
        &mut self,
        source: &YulParametersDeclaration,
    ) -> YulParametersDeclaration {
        let parameters = self.mutate_yul_parameters(&source.parameters);

        Rc::new(YulParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn mutate_yul_returns_declaration(
        &mut self,
        source: &YulReturnsDeclaration,
    ) -> YulReturnsDeclaration {
        let variables = self.mutate_yul_variable_names(&source.variables);

        Rc::new(YulReturnsDeclarationStruct {
            node_id: source.node_id,
            variables,
        })
    }

    fn mutate_yul_variable_declaration_statement(
        &mut self,
        source: &YulVariableDeclarationStatement,
    ) -> YulVariableDeclarationStatement {
        let variables = self.mutate_yul_variable_names(&source.variables);
        let value = source
            .value
            .as_ref()
            .map(|value| self.mutate_yul_variable_declaration_value(value));

        Rc::new(YulVariableDeclarationStatementStruct {
            node_id: source.node_id,
            variables,
            value,
        })
    }

    fn mutate_yul_variable_declaration_value(
        &mut self,
        source: &YulVariableDeclarationValue,
    ) -> YulVariableDeclarationValue {
        let assignment = self.mutate_yul_assignment_operator(&source.assignment);
        let expression = self.mutate_yul_expression(&source.expression);

        Rc::new(YulVariableDeclarationValueStruct {
            node_id: source.node_id,
            assignment,
            expression,
        })
    }

    fn mutate_yul_variable_assignment_statement(
        &mut self,
        source: &YulVariableAssignmentStatement,
    ) -> YulVariableAssignmentStatement {
        let variables = self.mutate_yul_paths(&source.variables);
        let assignment = self.mutate_yul_assignment_operator(&source.assignment);
        let expression = self.mutate_yul_expression(&source.expression);

        Rc::new(YulVariableAssignmentStatementStruct {
            node_id: source.node_id,
            variables,
            assignment,
            expression,
        })
    }

    fn mutate_yul_colon_and_equal(&mut self, source: &YulColonAndEqual) -> YulColonAndEqual {
        Rc::new(YulColonAndEqualStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_stack_assignment_statement(
        &mut self,
        source: &YulStackAssignmentStatement,
    ) -> YulStackAssignmentStatement {
        let assignment = self.mutate_yul_stack_assignment_operator(&source.assignment);
        let variable = Rc::clone(&source.variable);

        Rc::new(YulStackAssignmentStatementStruct {
            node_id: source.node_id,
            assignment,
            variable,
        })
    }

    fn mutate_yul_equal_and_colon(&mut self, source: &YulEqualAndColon) -> YulEqualAndColon {
        Rc::new(YulEqualAndColonStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_if_statement(&mut self, source: &YulIfStatement) -> YulIfStatement {
        let condition = self.mutate_yul_expression(&source.condition);
        let body = self.mutate_yul_block(&source.body);

        Rc::new(YulIfStatementStruct {
            node_id: source.node_id,
            condition,
            body,
        })
    }

    fn mutate_yul_for_statement(&mut self, source: &YulForStatement) -> YulForStatement {
        let initialization = self.mutate_yul_block(&source.initialization);
        let condition = self.mutate_yul_expression(&source.condition);
        let iterator = self.mutate_yul_block(&source.iterator);
        let body = self.mutate_yul_block(&source.body);

        Rc::new(YulForStatementStruct {
            node_id: source.node_id,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    fn mutate_yul_switch_statement(&mut self, source: &YulSwitchStatement) -> YulSwitchStatement {
        let expression = self.mutate_yul_expression(&source.expression);
        let cases = self.mutate_yul_switch_cases(&source.cases);

        Rc::new(YulSwitchStatementStruct {
            node_id: source.node_id,
            expression,
            cases,
        })
    }

    fn mutate_yul_default_case(&mut self, source: &YulDefaultCase) -> YulDefaultCase {
        let body = self.mutate_yul_block(&source.body);

        Rc::new(YulDefaultCaseStruct {
            node_id: source.node_id,
            body,
        })
    }

    fn mutate_yul_value_case(&mut self, source: &YulValueCase) -> YulValueCase {
        let value = self.mutate_yul_literal(&source.value);
        let body = self.mutate_yul_block(&source.body);

        Rc::new(YulValueCaseStruct {
            node_id: source.node_id,
            value,
            body,
        })
    }

    fn mutate_yul_leave_statement(&mut self, source: &YulLeaveStatement) -> YulLeaveStatement {
        Rc::new(YulLeaveStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_break_statement(&mut self, source: &YulBreakStatement) -> YulBreakStatement {
        Rc::new(YulBreakStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_continue_statement(
        &mut self,
        source: &YulContinueStatement,
    ) -> YulContinueStatement {
        Rc::new(YulContinueStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_label(&mut self, source: &YulLabel) -> YulLabel {
        let label = Rc::clone(&source.label);

        Rc::new(YulLabelStruct {
            node_id: source.node_id,
            label,
        })
    }

    fn mutate_yul_function_call_expression(
        &mut self,
        source: &YulFunctionCallExpression,
    ) -> YulFunctionCallExpression {
        let operand = self.mutate_yul_expression(&source.operand);
        let arguments = self.mutate_yul_arguments(&source.arguments);

        Rc::new(YulFunctionCallExpressionStruct {
            node_id: source.node_id,
            operand,
            arguments,
        })
    }

    //
    // Choices:
    //

    fn default_mutate_source_unit_member(&mut self, source: &SourceUnitMember) -> SourceUnitMember {
        match source {
            SourceUnitMember::PragmaDirective(ref pragma_directive) => {
                SourceUnitMember::PragmaDirective(self.mutate_pragma_directive(pragma_directive))
            }
            SourceUnitMember::ImportDirective(ref import_directive) => {
                SourceUnitMember::ImportDirective(self.mutate_import_directive(import_directive))
            }
            SourceUnitMember::ContractDefinition(ref contract_definition) => {
                SourceUnitMember::ContractDefinition(
                    self.mutate_contract_definition(contract_definition),
                )
            }
            SourceUnitMember::InterfaceDefinition(ref interface_definition) => {
                SourceUnitMember::InterfaceDefinition(
                    self.mutate_interface_definition(interface_definition),
                )
            }
            SourceUnitMember::LibraryDefinition(ref library_definition) => {
                SourceUnitMember::LibraryDefinition(
                    self.mutate_library_definition(library_definition),
                )
            }
            SourceUnitMember::StructDefinition(ref struct_definition) => {
                SourceUnitMember::StructDefinition(self.mutate_struct_definition(struct_definition))
            }
            SourceUnitMember::EnumDefinition(ref enum_definition) => {
                SourceUnitMember::EnumDefinition(self.mutate_enum_definition(enum_definition))
            }
            SourceUnitMember::FunctionDefinition(ref function_definition) => {
                SourceUnitMember::FunctionDefinition(
                    self.mutate_function_definition(function_definition),
                )
            }
            SourceUnitMember::ErrorDefinition(ref error_definition) => {
                SourceUnitMember::ErrorDefinition(self.mutate_error_definition(error_definition))
            }
            SourceUnitMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => SourceUnitMember::UserDefinedValueTypeDefinition(
                self.mutate_user_defined_value_type_definition(user_defined_value_type_definition),
            ),
            SourceUnitMember::UsingDirective(ref using_directive) => {
                SourceUnitMember::UsingDirective(self.mutate_using_directive(using_directive))
            }
            SourceUnitMember::EventDefinition(ref event_definition) => {
                SourceUnitMember::EventDefinition(self.mutate_event_definition(event_definition))
            }
            SourceUnitMember::ConstantDefinition(ref constant_definition) => {
                SourceUnitMember::ConstantDefinition(
                    self.mutate_constant_definition(constant_definition),
                )
            }
        }
    }
    fn mutate_source_unit_member(&mut self, source: &SourceUnitMember) -> SourceUnitMember {
        self.default_mutate_source_unit_member(source)
    }

    fn default_mutate_pragma(&mut self, source: &Pragma) -> Pragma {
        match source {
            Pragma::AbicoderPragma(ref abicoder_pragma) => {
                Pragma::AbicoderPragma(self.mutate_abicoder_pragma(abicoder_pragma))
            }
            Pragma::ExperimentalPragma(ref experimental_pragma) => {
                Pragma::ExperimentalPragma(self.mutate_experimental_pragma(experimental_pragma))
            }
            Pragma::VersionPragma(ref version_pragma) => {
                Pragma::VersionPragma(self.mutate_version_pragma(version_pragma))
            }
        }
    }
    fn mutate_pragma(&mut self, source: &Pragma) -> Pragma {
        self.default_mutate_pragma(source)
    }

    fn default_mutate_experimental_feature(
        &mut self,
        source: &ExperimentalFeature,
    ) -> ExperimentalFeature {
        match source {
            ExperimentalFeature::StringLiteral(ref string_literal) => {
                ExperimentalFeature::StringLiteral(self.mutate_string_literal(string_literal))
            }
            ExperimentalFeature::Identifier(node) => {
                ExperimentalFeature::Identifier(Rc::clone(node))
            }
        }
    }
    fn mutate_experimental_feature(&mut self, source: &ExperimentalFeature) -> ExperimentalFeature {
        self.default_mutate_experimental_feature(source)
    }

    fn default_mutate_version_expression(
        &mut self,
        source: &VersionExpression,
    ) -> VersionExpression {
        match source {
            VersionExpression::VersionRange(ref version_range) => {
                VersionExpression::VersionRange(self.mutate_version_range(version_range))
            }
            VersionExpression::VersionTerm(ref version_term) => {
                VersionExpression::VersionTerm(self.mutate_version_term(version_term))
            }
        }
    }
    fn mutate_version_expression(&mut self, source: &VersionExpression) -> VersionExpression {
        self.default_mutate_version_expression(source)
    }

    fn default_mutate_version_operator(&mut self, source: &VersionOperator) -> VersionOperator {
        match source {
            VersionOperator::Caret => VersionOperator::Caret,
            VersionOperator::Tilde => VersionOperator::Tilde,
            VersionOperator::Equal => VersionOperator::Equal,
            VersionOperator::LessThan => VersionOperator::LessThan,
            VersionOperator::GreaterThan => VersionOperator::GreaterThan,
            VersionOperator::LessThanEqual => VersionOperator::LessThanEqual,
            VersionOperator::GreaterThanEqual => VersionOperator::GreaterThanEqual,
        }
    }
    fn mutate_version_operator(&mut self, source: &VersionOperator) -> VersionOperator {
        self.default_mutate_version_operator(source)
    }

    fn default_mutate_version_literal(&mut self, source: &VersionLiteral) -> VersionLiteral {
        match source {
            VersionLiteral::SimpleVersionLiteral(ref simple_version_literal) => {
                VersionLiteral::SimpleVersionLiteral(
                    self.mutate_simple_version_literal(simple_version_literal),
                )
            }
            VersionLiteral::SingleQuotedVersionLiteral(node) => {
                VersionLiteral::SingleQuotedVersionLiteral(Rc::clone(node))
            }
            VersionLiteral::DoubleQuotedVersionLiteral(node) => {
                VersionLiteral::DoubleQuotedVersionLiteral(Rc::clone(node))
            }
        }
    }
    fn mutate_version_literal(&mut self, source: &VersionLiteral) -> VersionLiteral {
        self.default_mutate_version_literal(source)
    }

    fn default_mutate_import_clause(&mut self, source: &ImportClause) -> ImportClause {
        match source {
            ImportClause::PathImport(ref path_import) => {
                ImportClause::PathImport(self.mutate_path_import(path_import))
            }
            ImportClause::NamedImport(ref named_import) => {
                ImportClause::NamedImport(self.mutate_named_import(named_import))
            }
            ImportClause::ImportDeconstruction(ref import_deconstruction) => {
                ImportClause::ImportDeconstruction(
                    self.mutate_import_deconstruction(import_deconstruction),
                )
            }
        }
    }
    fn mutate_import_clause(&mut self, source: &ImportClause) -> ImportClause {
        self.default_mutate_import_clause(source)
    }

    fn default_mutate_using_clause(&mut self, source: &UsingClause) -> UsingClause {
        match source {
            UsingClause::IdentifierPath(ref identifier_path) => {
                UsingClause::IdentifierPath(self.mutate_identifier_path(identifier_path))
            }
            UsingClause::UsingDeconstruction(ref using_deconstruction) => {
                UsingClause::UsingDeconstruction(
                    self.mutate_using_deconstruction(using_deconstruction),
                )
            }
        }
    }
    fn mutate_using_clause(&mut self, source: &UsingClause) -> UsingClause {
        self.default_mutate_using_clause(source)
    }

    fn default_mutate_using_operator(&mut self, source: &UsingOperator) -> UsingOperator {
        match source {
            UsingOperator::Ampersand => UsingOperator::Ampersand,
            UsingOperator::Asterisk => UsingOperator::Asterisk,
            UsingOperator::BangEqual => UsingOperator::BangEqual,
            UsingOperator::Bar => UsingOperator::Bar,
            UsingOperator::Caret => UsingOperator::Caret,
            UsingOperator::EqualEqual => UsingOperator::EqualEqual,
            UsingOperator::GreaterThan => UsingOperator::GreaterThan,
            UsingOperator::GreaterThanEqual => UsingOperator::GreaterThanEqual,
            UsingOperator::LessThan => UsingOperator::LessThan,
            UsingOperator::LessThanEqual => UsingOperator::LessThanEqual,
            UsingOperator::Minus => UsingOperator::Minus,
            UsingOperator::Percent => UsingOperator::Percent,
            UsingOperator::Plus => UsingOperator::Plus,
            UsingOperator::Slash => UsingOperator::Slash,
            UsingOperator::Tilde => UsingOperator::Tilde,
        }
    }
    fn mutate_using_operator(&mut self, source: &UsingOperator) -> UsingOperator {
        self.default_mutate_using_operator(source)
    }

    fn default_mutate_using_target(&mut self, source: &UsingTarget) -> UsingTarget {
        match source {
            UsingTarget::TypeName(ref type_name) => {
                UsingTarget::TypeName(self.mutate_type_name(type_name))
            }
            UsingTarget::Asterisk => UsingTarget::Asterisk,
        }
    }
    fn mutate_using_target(&mut self, source: &UsingTarget) -> UsingTarget {
        self.default_mutate_using_target(source)
    }

    fn default_mutate_contract_member(&mut self, source: &ContractMember) -> ContractMember {
        match source {
            ContractMember::UsingDirective(ref using_directive) => {
                ContractMember::UsingDirective(self.mutate_using_directive(using_directive))
            }
            ContractMember::FunctionDefinition(ref function_definition) => {
                ContractMember::FunctionDefinition(
                    self.mutate_function_definition(function_definition),
                )
            }
            ContractMember::ConstructorDefinition(ref constructor_definition) => {
                ContractMember::ConstructorDefinition(
                    self.mutate_constructor_definition(constructor_definition),
                )
            }
            ContractMember::ReceiveFunctionDefinition(ref receive_function_definition) => {
                ContractMember::ReceiveFunctionDefinition(
                    self.mutate_receive_function_definition(receive_function_definition),
                )
            }
            ContractMember::FallbackFunctionDefinition(ref fallback_function_definition) => {
                ContractMember::FallbackFunctionDefinition(
                    self.mutate_fallback_function_definition(fallback_function_definition),
                )
            }
            ContractMember::ModifierDefinition(ref modifier_definition) => {
                ContractMember::ModifierDefinition(
                    self.mutate_modifier_definition(modifier_definition),
                )
            }
            ContractMember::StructDefinition(ref struct_definition) => {
                ContractMember::StructDefinition(self.mutate_struct_definition(struct_definition))
            }
            ContractMember::EnumDefinition(ref enum_definition) => {
                ContractMember::EnumDefinition(self.mutate_enum_definition(enum_definition))
            }
            ContractMember::EventDefinition(ref event_definition) => {
                ContractMember::EventDefinition(self.mutate_event_definition(event_definition))
            }
            ContractMember::ErrorDefinition(ref error_definition) => {
                ContractMember::ErrorDefinition(self.mutate_error_definition(error_definition))
            }
            ContractMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => ContractMember::UserDefinedValueTypeDefinition(
                self.mutate_user_defined_value_type_definition(user_defined_value_type_definition),
            ),
            ContractMember::StateVariableDefinition(ref state_variable_definition) => {
                ContractMember::StateVariableDefinition(
                    self.mutate_state_variable_definition(state_variable_definition),
                )
            }
        }
    }
    fn mutate_contract_member(&mut self, source: &ContractMember) -> ContractMember {
        self.default_mutate_contract_member(source)
    }

    fn default_mutate_state_variable_attribute(
        &mut self,
        source: &StateVariableAttribute,
    ) -> StateVariableAttribute {
        match source {
            StateVariableAttribute::OverrideSpecifier(ref override_specifier) => {
                StateVariableAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
                )
            }
            StateVariableAttribute::ConstantKeyword => StateVariableAttribute::ConstantKeyword,
            StateVariableAttribute::InternalKeyword => StateVariableAttribute::InternalKeyword,
            StateVariableAttribute::PrivateKeyword => StateVariableAttribute::PrivateKeyword,
            StateVariableAttribute::PublicKeyword => StateVariableAttribute::PublicKeyword,
            StateVariableAttribute::ImmutableKeyword => StateVariableAttribute::ImmutableKeyword,
            StateVariableAttribute::TransientKeyword => StateVariableAttribute::TransientKeyword,
        }
    }
    fn mutate_state_variable_attribute(
        &mut self,
        source: &StateVariableAttribute,
    ) -> StateVariableAttribute {
        self.default_mutate_state_variable_attribute(source)
    }

    fn default_mutate_function_name(&mut self, source: &FunctionName) -> FunctionName {
        match source {
            FunctionName::Identifier(node) => FunctionName::Identifier(Rc::clone(node)),
            FunctionName::FallbackKeyword => FunctionName::FallbackKeyword,
            FunctionName::ReceiveKeyword => FunctionName::ReceiveKeyword,
        }
    }
    fn mutate_function_name(&mut self, source: &FunctionName) -> FunctionName {
        self.default_mutate_function_name(source)
    }

    fn default_mutate_function_attribute(
        &mut self,
        source: &FunctionAttribute,
    ) -> FunctionAttribute {
        match source {
            FunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                FunctionAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            FunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                FunctionAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
                )
            }
            FunctionAttribute::ConstantKeyword => FunctionAttribute::ConstantKeyword,
            FunctionAttribute::ExternalKeyword => FunctionAttribute::ExternalKeyword,
            FunctionAttribute::InternalKeyword => FunctionAttribute::InternalKeyword,
            FunctionAttribute::PayableKeyword => FunctionAttribute::PayableKeyword,
            FunctionAttribute::PrivateKeyword => FunctionAttribute::PrivateKeyword,
            FunctionAttribute::PublicKeyword => FunctionAttribute::PublicKeyword,
            FunctionAttribute::PureKeyword => FunctionAttribute::PureKeyword,
            FunctionAttribute::ViewKeyword => FunctionAttribute::ViewKeyword,
            FunctionAttribute::VirtualKeyword => FunctionAttribute::VirtualKeyword,
        }
    }
    fn mutate_function_attribute(&mut self, source: &FunctionAttribute) -> FunctionAttribute {
        self.default_mutate_function_attribute(source)
    }

    fn default_mutate_function_body(&mut self, source: &FunctionBody) -> FunctionBody {
        match source {
            FunctionBody::Block(ref block) => FunctionBody::Block(self.mutate_block(block)),
            FunctionBody::Semicolon => FunctionBody::Semicolon,
        }
    }
    fn mutate_function_body(&mut self, source: &FunctionBody) -> FunctionBody {
        self.default_mutate_function_body(source)
    }

    fn default_mutate_constructor_attribute(
        &mut self,
        source: &ConstructorAttribute,
    ) -> ConstructorAttribute {
        match source {
            ConstructorAttribute::ModifierInvocation(ref modifier_invocation) => {
                ConstructorAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            ConstructorAttribute::InternalKeyword => ConstructorAttribute::InternalKeyword,
            ConstructorAttribute::OverrideKeyword => ConstructorAttribute::OverrideKeyword,
            ConstructorAttribute::PayableKeyword => ConstructorAttribute::PayableKeyword,
            ConstructorAttribute::PublicKeyword => ConstructorAttribute::PublicKeyword,
            ConstructorAttribute::VirtualKeyword => ConstructorAttribute::VirtualKeyword,
        }
    }
    fn mutate_constructor_attribute(
        &mut self,
        source: &ConstructorAttribute,
    ) -> ConstructorAttribute {
        self.default_mutate_constructor_attribute(source)
    }

    fn default_mutate_unnamed_function_attribute(
        &mut self,
        source: &UnnamedFunctionAttribute,
    ) -> UnnamedFunctionAttribute {
        match source {
            UnnamedFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                UnnamedFunctionAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            UnnamedFunctionAttribute::ConstantKeyword => UnnamedFunctionAttribute::ConstantKeyword,
            UnnamedFunctionAttribute::ExternalKeyword => UnnamedFunctionAttribute::ExternalKeyword,
            UnnamedFunctionAttribute::InternalKeyword => UnnamedFunctionAttribute::InternalKeyword,
            UnnamedFunctionAttribute::PayableKeyword => UnnamedFunctionAttribute::PayableKeyword,
            UnnamedFunctionAttribute::PrivateKeyword => UnnamedFunctionAttribute::PrivateKeyword,
            UnnamedFunctionAttribute::PublicKeyword => UnnamedFunctionAttribute::PublicKeyword,
            UnnamedFunctionAttribute::PureKeyword => UnnamedFunctionAttribute::PureKeyword,
            UnnamedFunctionAttribute::ViewKeyword => UnnamedFunctionAttribute::ViewKeyword,
        }
    }
    fn mutate_unnamed_function_attribute(
        &mut self,
        source: &UnnamedFunctionAttribute,
    ) -> UnnamedFunctionAttribute {
        self.default_mutate_unnamed_function_attribute(source)
    }

    fn default_mutate_fallback_function_attribute(
        &mut self,
        source: &FallbackFunctionAttribute,
    ) -> FallbackFunctionAttribute {
        match source {
            FallbackFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                FallbackFunctionAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            FallbackFunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                FallbackFunctionAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
                )
            }
            FallbackFunctionAttribute::ExternalKeyword => {
                FallbackFunctionAttribute::ExternalKeyword
            }
            FallbackFunctionAttribute::PayableKeyword => FallbackFunctionAttribute::PayableKeyword,
            FallbackFunctionAttribute::PureKeyword => FallbackFunctionAttribute::PureKeyword,
            FallbackFunctionAttribute::ViewKeyword => FallbackFunctionAttribute::ViewKeyword,
            FallbackFunctionAttribute::VirtualKeyword => FallbackFunctionAttribute::VirtualKeyword,
        }
    }
    fn mutate_fallback_function_attribute(
        &mut self,
        source: &FallbackFunctionAttribute,
    ) -> FallbackFunctionAttribute {
        self.default_mutate_fallback_function_attribute(source)
    }

    fn default_mutate_receive_function_attribute(
        &mut self,
        source: &ReceiveFunctionAttribute,
    ) -> ReceiveFunctionAttribute {
        match source {
            ReceiveFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                ReceiveFunctionAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            ReceiveFunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                ReceiveFunctionAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
                )
            }
            ReceiveFunctionAttribute::ExternalKeyword => ReceiveFunctionAttribute::ExternalKeyword,
            ReceiveFunctionAttribute::PayableKeyword => ReceiveFunctionAttribute::PayableKeyword,
            ReceiveFunctionAttribute::VirtualKeyword => ReceiveFunctionAttribute::VirtualKeyword,
        }
    }
    fn mutate_receive_function_attribute(
        &mut self,
        source: &ReceiveFunctionAttribute,
    ) -> ReceiveFunctionAttribute {
        self.default_mutate_receive_function_attribute(source)
    }

    fn default_mutate_modifier_attribute(
        &mut self,
        source: &ModifierAttribute,
    ) -> ModifierAttribute {
        match source {
            ModifierAttribute::OverrideSpecifier(ref override_specifier) => {
                ModifierAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
                )
            }
            ModifierAttribute::VirtualKeyword => ModifierAttribute::VirtualKeyword,
        }
    }
    fn mutate_modifier_attribute(&mut self, source: &ModifierAttribute) -> ModifierAttribute {
        self.default_mutate_modifier_attribute(source)
    }

    fn default_mutate_type_name(&mut self, source: &TypeName) -> TypeName {
        match source {
            TypeName::ArrayTypeName(ref array_type_name) => {
                TypeName::ArrayTypeName(self.mutate_array_type_name(array_type_name))
            }
            TypeName::FunctionType(ref function_type) => {
                TypeName::FunctionType(self.mutate_function_type(function_type))
            }
            TypeName::MappingType(ref mapping_type) => {
                TypeName::MappingType(self.mutate_mapping_type(mapping_type))
            }
            TypeName::ElementaryType(ref elementary_type) => {
                TypeName::ElementaryType(self.mutate_elementary_type(elementary_type))
            }
            TypeName::IdentifierPath(ref identifier_path) => {
                TypeName::IdentifierPath(self.mutate_identifier_path(identifier_path))
            }
        }
    }
    fn mutate_type_name(&mut self, source: &TypeName) -> TypeName {
        self.default_mutate_type_name(source)
    }

    fn default_mutate_function_type_attribute(
        &mut self,
        source: &FunctionTypeAttribute,
    ) -> FunctionTypeAttribute {
        match source {
            FunctionTypeAttribute::InternalKeyword => FunctionTypeAttribute::InternalKeyword,
            FunctionTypeAttribute::ExternalKeyword => FunctionTypeAttribute::ExternalKeyword,
            FunctionTypeAttribute::PrivateKeyword => FunctionTypeAttribute::PrivateKeyword,
            FunctionTypeAttribute::PublicKeyword => FunctionTypeAttribute::PublicKeyword,
            FunctionTypeAttribute::ConstantKeyword => FunctionTypeAttribute::ConstantKeyword,
            FunctionTypeAttribute::PureKeyword => FunctionTypeAttribute::PureKeyword,
            FunctionTypeAttribute::ViewKeyword => FunctionTypeAttribute::ViewKeyword,
            FunctionTypeAttribute::PayableKeyword => FunctionTypeAttribute::PayableKeyword,
        }
    }
    fn mutate_function_type_attribute(
        &mut self,
        source: &FunctionTypeAttribute,
    ) -> FunctionTypeAttribute {
        self.default_mutate_function_type_attribute(source)
    }

    fn default_mutate_mapping_key_type(&mut self, source: &MappingKeyType) -> MappingKeyType {
        match source {
            MappingKeyType::ElementaryType(ref elementary_type) => {
                MappingKeyType::ElementaryType(self.mutate_elementary_type(elementary_type))
            }
            MappingKeyType::IdentifierPath(ref identifier_path) => {
                MappingKeyType::IdentifierPath(self.mutate_identifier_path(identifier_path))
            }
        }
    }
    fn mutate_mapping_key_type(&mut self, source: &MappingKeyType) -> MappingKeyType {
        self.default_mutate_mapping_key_type(source)
    }

    fn default_mutate_elementary_type(&mut self, source: &ElementaryType) -> ElementaryType {
        match source {
            ElementaryType::AddressType(ref address_type) => {
                ElementaryType::AddressType(self.mutate_address_type(address_type))
            }
            ElementaryType::BoolKeyword => ElementaryType::BoolKeyword,
            ElementaryType::ByteKeyword => ElementaryType::ByteKeyword,
            ElementaryType::StringKeyword => ElementaryType::StringKeyword,
            ElementaryType::BytesKeyword(node) => ElementaryType::BytesKeyword(Rc::clone(node)),
            ElementaryType::IntKeyword(node) => ElementaryType::IntKeyword(Rc::clone(node)),
            ElementaryType::UintKeyword(node) => ElementaryType::UintKeyword(Rc::clone(node)),
            ElementaryType::FixedKeyword(node) => ElementaryType::FixedKeyword(Rc::clone(node)),
            ElementaryType::UfixedKeyword(node) => ElementaryType::UfixedKeyword(Rc::clone(node)),
        }
    }
    fn mutate_elementary_type(&mut self, source: &ElementaryType) -> ElementaryType {
        self.default_mutate_elementary_type(source)
    }

    fn default_mutate_statement(&mut self, source: &Statement) -> Statement {
        match source {
            Statement::IfStatement(ref if_statement) => {
                Statement::IfStatement(self.mutate_if_statement(if_statement))
            }
            Statement::ForStatement(ref for_statement) => {
                Statement::ForStatement(self.mutate_for_statement(for_statement))
            }
            Statement::WhileStatement(ref while_statement) => {
                Statement::WhileStatement(self.mutate_while_statement(while_statement))
            }
            Statement::DoWhileStatement(ref do_while_statement) => {
                Statement::DoWhileStatement(self.mutate_do_while_statement(do_while_statement))
            }
            Statement::ContinueStatement(ref continue_statement) => {
                Statement::ContinueStatement(self.mutate_continue_statement(continue_statement))
            }
            Statement::BreakStatement(ref break_statement) => {
                Statement::BreakStatement(self.mutate_break_statement(break_statement))
            }
            Statement::ReturnStatement(ref return_statement) => {
                Statement::ReturnStatement(self.mutate_return_statement(return_statement))
            }
            Statement::ThrowStatement(ref throw_statement) => {
                Statement::ThrowStatement(self.mutate_throw_statement(throw_statement))
            }
            Statement::EmitStatement(ref emit_statement) => {
                Statement::EmitStatement(self.mutate_emit_statement(emit_statement))
            }
            Statement::TryStatement(ref try_statement) => {
                Statement::TryStatement(self.mutate_try_statement(try_statement))
            }
            Statement::RevertStatement(ref revert_statement) => {
                Statement::RevertStatement(self.mutate_revert_statement(revert_statement))
            }
            Statement::AssemblyStatement(ref assembly_statement) => {
                Statement::AssemblyStatement(self.mutate_assembly_statement(assembly_statement))
            }
            Statement::Block(ref block) => Statement::Block(self.mutate_block(block)),
            Statement::UncheckedBlock(ref unchecked_block) => {
                Statement::UncheckedBlock(self.mutate_unchecked_block(unchecked_block))
            }
            Statement::TupleDeconstructionStatement(ref tuple_deconstruction_statement) => {
                Statement::TupleDeconstructionStatement(
                    self.mutate_tuple_deconstruction_statement(tuple_deconstruction_statement),
                )
            }
            Statement::VariableDeclarationStatement(ref variable_declaration_statement) => {
                Statement::VariableDeclarationStatement(
                    self.mutate_variable_declaration_statement(variable_declaration_statement),
                )
            }
            Statement::ExpressionStatement(ref expression_statement) => {
                Statement::ExpressionStatement(
                    self.mutate_expression_statement(expression_statement),
                )
            }
        }
    }
    fn mutate_statement(&mut self, source: &Statement) -> Statement {
        self.default_mutate_statement(source)
    }

    fn default_mutate_tuple_member(&mut self, source: &TupleMember) -> TupleMember {
        match source {
            TupleMember::TypedTupleMember(ref typed_tuple_member) => {
                TupleMember::TypedTupleMember(self.mutate_typed_tuple_member(typed_tuple_member))
            }
            TupleMember::UntypedTupleMember(ref untyped_tuple_member) => {
                TupleMember::UntypedTupleMember(
                    self.mutate_untyped_tuple_member(untyped_tuple_member),
                )
            }
        }
    }
    fn mutate_tuple_member(&mut self, source: &TupleMember) -> TupleMember {
        self.default_mutate_tuple_member(source)
    }

    fn default_mutate_variable_declaration_type(
        &mut self,
        source: &VariableDeclarationType,
    ) -> VariableDeclarationType {
        match source {
            VariableDeclarationType::TypeName(ref type_name) => {
                VariableDeclarationType::TypeName(self.mutate_type_name(type_name))
            }
            VariableDeclarationType::VarKeyword => VariableDeclarationType::VarKeyword,
        }
    }
    fn mutate_variable_declaration_type(
        &mut self,
        source: &VariableDeclarationType,
    ) -> VariableDeclarationType {
        self.default_mutate_variable_declaration_type(source)
    }

    fn default_mutate_storage_location(&mut self, source: &StorageLocation) -> StorageLocation {
        match source {
            StorageLocation::MemoryKeyword => StorageLocation::MemoryKeyword,
            StorageLocation::StorageKeyword => StorageLocation::StorageKeyword,
            StorageLocation::CallDataKeyword => StorageLocation::CallDataKeyword,
        }
    }
    fn mutate_storage_location(&mut self, source: &StorageLocation) -> StorageLocation {
        self.default_mutate_storage_location(source)
    }

    fn default_mutate_for_statement_initialization(
        &mut self,
        source: &ForStatementInitialization,
    ) -> ForStatementInitialization {
        match source {
            ForStatementInitialization::TupleDeconstructionStatement(
                ref tuple_deconstruction_statement,
            ) => ForStatementInitialization::TupleDeconstructionStatement(
                self.mutate_tuple_deconstruction_statement(tuple_deconstruction_statement),
            ),
            ForStatementInitialization::VariableDeclarationStatement(
                ref variable_declaration_statement,
            ) => ForStatementInitialization::VariableDeclarationStatement(
                self.mutate_variable_declaration_statement(variable_declaration_statement),
            ),
            ForStatementInitialization::ExpressionStatement(ref expression_statement) => {
                ForStatementInitialization::ExpressionStatement(
                    self.mutate_expression_statement(expression_statement),
                )
            }
            ForStatementInitialization::Semicolon => ForStatementInitialization::Semicolon,
        }
    }
    fn mutate_for_statement_initialization(
        &mut self,
        source: &ForStatementInitialization,
    ) -> ForStatementInitialization {
        self.default_mutate_for_statement_initialization(source)
    }

    fn default_mutate_for_statement_condition(
        &mut self,
        source: &ForStatementCondition,
    ) -> ForStatementCondition {
        match source {
            ForStatementCondition::ExpressionStatement(ref expression_statement) => {
                ForStatementCondition::ExpressionStatement(
                    self.mutate_expression_statement(expression_statement),
                )
            }
            ForStatementCondition::Semicolon => ForStatementCondition::Semicolon,
        }
    }
    fn mutate_for_statement_condition(
        &mut self,
        source: &ForStatementCondition,
    ) -> ForStatementCondition {
        self.default_mutate_for_statement_condition(source)
    }

    fn default_mutate_expression(&mut self, source: &Expression) -> Expression {
        match source {
            Expression::AssignmentExpression(ref assignment_expression) => {
                Expression::AssignmentExpression(
                    self.mutate_assignment_expression(assignment_expression),
                )
            }
            Expression::ConditionalExpression(ref conditional_expression) => {
                Expression::ConditionalExpression(
                    self.mutate_conditional_expression(conditional_expression),
                )
            }
            Expression::OrExpression(ref or_expression) => {
                Expression::OrExpression(self.mutate_or_expression(or_expression))
            }
            Expression::AndExpression(ref and_expression) => {
                Expression::AndExpression(self.mutate_and_expression(and_expression))
            }
            Expression::EqualityExpression(ref equality_expression) => {
                Expression::EqualityExpression(self.mutate_equality_expression(equality_expression))
            }
            Expression::InequalityExpression(ref inequality_expression) => {
                Expression::InequalityExpression(
                    self.mutate_inequality_expression(inequality_expression),
                )
            }
            Expression::BitwiseOrExpression(ref bitwise_or_expression) => {
                Expression::BitwiseOrExpression(
                    self.mutate_bitwise_or_expression(bitwise_or_expression),
                )
            }
            Expression::BitwiseXorExpression(ref bitwise_xor_expression) => {
                Expression::BitwiseXorExpression(
                    self.mutate_bitwise_xor_expression(bitwise_xor_expression),
                )
            }
            Expression::BitwiseAndExpression(ref bitwise_and_expression) => {
                Expression::BitwiseAndExpression(
                    self.mutate_bitwise_and_expression(bitwise_and_expression),
                )
            }
            Expression::ShiftExpression(ref shift_expression) => {
                Expression::ShiftExpression(self.mutate_shift_expression(shift_expression))
            }
            Expression::AdditiveExpression(ref additive_expression) => {
                Expression::AdditiveExpression(self.mutate_additive_expression(additive_expression))
            }
            Expression::MultiplicativeExpression(ref multiplicative_expression) => {
                Expression::MultiplicativeExpression(
                    self.mutate_multiplicative_expression(multiplicative_expression),
                )
            }
            Expression::ExponentiationExpression(ref exponentiation_expression) => {
                Expression::ExponentiationExpression(
                    self.mutate_exponentiation_expression(exponentiation_expression),
                )
            }
            Expression::PostfixExpression(ref postfix_expression) => {
                Expression::PostfixExpression(self.mutate_postfix_expression(postfix_expression))
            }
            Expression::PrefixExpression(ref prefix_expression) => {
                Expression::PrefixExpression(self.mutate_prefix_expression(prefix_expression))
            }
            Expression::FunctionCallExpression(ref function_call_expression) => {
                Expression::FunctionCallExpression(
                    self.mutate_function_call_expression(function_call_expression),
                )
            }
            Expression::CallOptionsExpression(ref call_options_expression) => {
                Expression::CallOptionsExpression(
                    self.mutate_call_options_expression(call_options_expression),
                )
            }
            Expression::MemberAccessExpression(ref member_access_expression) => {
                Expression::MemberAccessExpression(
                    self.mutate_member_access_expression(member_access_expression),
                )
            }
            Expression::IndexAccessExpression(ref index_access_expression) => {
                Expression::IndexAccessExpression(
                    self.mutate_index_access_expression(index_access_expression),
                )
            }
            Expression::NewExpression(ref new_expression) => {
                Expression::NewExpression(self.mutate_new_expression(new_expression))
            }
            Expression::TupleExpression(ref tuple_expression) => {
                Expression::TupleExpression(self.mutate_tuple_expression(tuple_expression))
            }
            Expression::TypeExpression(ref type_expression) => {
                Expression::TypeExpression(self.mutate_type_expression(type_expression))
            }
            Expression::ArrayExpression(ref array_expression) => {
                Expression::ArrayExpression(self.mutate_array_expression(array_expression))
            }
            Expression::HexNumberExpression(ref hex_number_expression) => {
                Expression::HexNumberExpression(
                    self.mutate_hex_number_expression(hex_number_expression),
                )
            }
            Expression::DecimalNumberExpression(ref decimal_number_expression) => {
                Expression::DecimalNumberExpression(
                    self.mutate_decimal_number_expression(decimal_number_expression),
                )
            }
            Expression::StringExpression(ref string_expression) => {
                Expression::StringExpression(self.mutate_string_expression(string_expression))
            }
            Expression::ElementaryType(ref elementary_type) => {
                Expression::ElementaryType(self.mutate_elementary_type(elementary_type))
            }
            Expression::PayableKeyword => Expression::PayableKeyword,
            Expression::ThisKeyword => Expression::ThisKeyword,
            Expression::SuperKeyword => Expression::SuperKeyword,
            Expression::TrueKeyword => Expression::TrueKeyword,
            Expression::FalseKeyword => Expression::FalseKeyword,
            Expression::Identifier(node) => Expression::Identifier(Rc::clone(node)),
        }
    }
    fn mutate_expression(&mut self, source: &Expression) -> Expression {
        self.default_mutate_expression(source)
    }

    fn default_mutate_arguments_declaration(
        &mut self,
        source: &ArgumentsDeclaration,
    ) -> ArgumentsDeclaration {
        match source {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(
                ref positional_arguments_declaration,
            ) => ArgumentsDeclaration::PositionalArgumentsDeclaration(
                self.mutate_positional_arguments_declaration(positional_arguments_declaration),
            ),
            ArgumentsDeclaration::NamedArgumentsDeclaration(ref named_arguments_declaration) => {
                ArgumentsDeclaration::NamedArgumentsDeclaration(
                    self.mutate_named_arguments_declaration(named_arguments_declaration),
                )
            }
        }
    }
    fn mutate_arguments_declaration(
        &mut self,
        source: &ArgumentsDeclaration,
    ) -> ArgumentsDeclaration {
        self.default_mutate_arguments_declaration(source)
    }

    fn default_mutate_number_unit(&mut self, source: &NumberUnit) -> NumberUnit {
        match source {
            NumberUnit::WeiKeyword => NumberUnit::WeiKeyword,
            NumberUnit::GweiKeyword => NumberUnit::GweiKeyword,
            NumberUnit::SzaboKeyword => NumberUnit::SzaboKeyword,
            NumberUnit::FinneyKeyword => NumberUnit::FinneyKeyword,
            NumberUnit::EtherKeyword => NumberUnit::EtherKeyword,
            NumberUnit::SecondsKeyword => NumberUnit::SecondsKeyword,
            NumberUnit::MinutesKeyword => NumberUnit::MinutesKeyword,
            NumberUnit::HoursKeyword => NumberUnit::HoursKeyword,
            NumberUnit::DaysKeyword => NumberUnit::DaysKeyword,
            NumberUnit::WeeksKeyword => NumberUnit::WeeksKeyword,
            NumberUnit::YearsKeyword => NumberUnit::YearsKeyword,
        }
    }
    fn mutate_number_unit(&mut self, source: &NumberUnit) -> NumberUnit {
        self.default_mutate_number_unit(source)
    }

    fn default_mutate_string_expression(&mut self, source: &StringExpression) -> StringExpression {
        match source {
            StringExpression::StringLiteral(ref string_literal) => {
                StringExpression::StringLiteral(self.mutate_string_literal(string_literal))
            }
            StringExpression::StringLiterals(ref string_literals) => {
                StringExpression::StringLiterals(self.mutate_string_literals(string_literals))
            }
            StringExpression::HexStringLiteral(ref hex_string_literal) => {
                StringExpression::HexStringLiteral(
                    self.mutate_hex_string_literal(hex_string_literal),
                )
            }
            StringExpression::HexStringLiterals(ref hex_string_literals) => {
                StringExpression::HexStringLiterals(
                    self.mutate_hex_string_literals(hex_string_literals),
                )
            }
            StringExpression::UnicodeStringLiterals(ref unicode_string_literals) => {
                StringExpression::UnicodeStringLiterals(
                    self.mutate_unicode_string_literals(unicode_string_literals),
                )
            }
        }
    }
    fn mutate_string_expression(&mut self, source: &StringExpression) -> StringExpression {
        self.default_mutate_string_expression(source)
    }

    fn default_mutate_string_literal(&mut self, source: &StringLiteral) -> StringLiteral {
        match source {
            StringLiteral::SingleQuotedStringLiteral(node) => {
                StringLiteral::SingleQuotedStringLiteral(Rc::clone(node))
            }
            StringLiteral::DoubleQuotedStringLiteral(node) => {
                StringLiteral::DoubleQuotedStringLiteral(Rc::clone(node))
            }
        }
    }
    fn mutate_string_literal(&mut self, source: &StringLiteral) -> StringLiteral {
        self.default_mutate_string_literal(source)
    }

    fn default_mutate_hex_string_literal(&mut self, source: &HexStringLiteral) -> HexStringLiteral {
        match source {
            HexStringLiteral::SingleQuotedHexStringLiteral(node) => {
                HexStringLiteral::SingleQuotedHexStringLiteral(Rc::clone(node))
            }
            HexStringLiteral::DoubleQuotedHexStringLiteral(node) => {
                HexStringLiteral::DoubleQuotedHexStringLiteral(Rc::clone(node))
            }
        }
    }
    fn mutate_hex_string_literal(&mut self, source: &HexStringLiteral) -> HexStringLiteral {
        self.default_mutate_hex_string_literal(source)
    }

    fn default_mutate_unicode_string_literal(
        &mut self,
        source: &UnicodeStringLiteral,
    ) -> UnicodeStringLiteral {
        match source {
            UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(node) => {
                UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(Rc::clone(node))
            }
            UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(node) => {
                UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(Rc::clone(node))
            }
        }
    }
    fn mutate_unicode_string_literal(
        &mut self,
        source: &UnicodeStringLiteral,
    ) -> UnicodeStringLiteral {
        self.default_mutate_unicode_string_literal(source)
    }

    fn default_mutate_yul_statement(&mut self, source: &YulStatement) -> YulStatement {
        match source {
            YulStatement::YulBlock(ref yul_block) => {
                YulStatement::YulBlock(self.mutate_yul_block(yul_block))
            }
            YulStatement::YulFunctionDefinition(ref yul_function_definition) => {
                YulStatement::YulFunctionDefinition(
                    self.mutate_yul_function_definition(yul_function_definition),
                )
            }
            YulStatement::YulStackAssignmentStatement(ref yul_stack_assignment_statement) => {
                YulStatement::YulStackAssignmentStatement(
                    self.mutate_yul_stack_assignment_statement(yul_stack_assignment_statement),
                )
            }
            YulStatement::YulIfStatement(ref yul_if_statement) => {
                YulStatement::YulIfStatement(self.mutate_yul_if_statement(yul_if_statement))
            }
            YulStatement::YulForStatement(ref yul_for_statement) => {
                YulStatement::YulForStatement(self.mutate_yul_for_statement(yul_for_statement))
            }
            YulStatement::YulSwitchStatement(ref yul_switch_statement) => {
                YulStatement::YulSwitchStatement(
                    self.mutate_yul_switch_statement(yul_switch_statement),
                )
            }
            YulStatement::YulLeaveStatement(ref yul_leave_statement) => {
                YulStatement::YulLeaveStatement(
                    self.mutate_yul_leave_statement(yul_leave_statement),
                )
            }
            YulStatement::YulBreakStatement(ref yul_break_statement) => {
                YulStatement::YulBreakStatement(
                    self.mutate_yul_break_statement(yul_break_statement),
                )
            }
            YulStatement::YulContinueStatement(ref yul_continue_statement) => {
                YulStatement::YulContinueStatement(
                    self.mutate_yul_continue_statement(yul_continue_statement),
                )
            }
            YulStatement::YulVariableAssignmentStatement(ref yul_variable_assignment_statement) => {
                YulStatement::YulVariableAssignmentStatement(
                    self.mutate_yul_variable_assignment_statement(
                        yul_variable_assignment_statement,
                    ),
                )
            }
            YulStatement::YulLabel(ref yul_label) => {
                YulStatement::YulLabel(self.mutate_yul_label(yul_label))
            }
            YulStatement::YulVariableDeclarationStatement(
                ref yul_variable_declaration_statement,
            ) => YulStatement::YulVariableDeclarationStatement(
                self.mutate_yul_variable_declaration_statement(yul_variable_declaration_statement),
            ),
            YulStatement::YulExpression(ref yul_expression) => {
                YulStatement::YulExpression(self.mutate_yul_expression(yul_expression))
            }
        }
    }
    fn mutate_yul_statement(&mut self, source: &YulStatement) -> YulStatement {
        self.default_mutate_yul_statement(source)
    }

    fn default_mutate_yul_assignment_operator(
        &mut self,
        source: &YulAssignmentOperator,
    ) -> YulAssignmentOperator {
        match source {
            YulAssignmentOperator::YulColonAndEqual(ref yul_colon_and_equal) => {
                YulAssignmentOperator::YulColonAndEqual(
                    self.mutate_yul_colon_and_equal(yul_colon_and_equal),
                )
            }
            YulAssignmentOperator::ColonEqual => YulAssignmentOperator::ColonEqual,
        }
    }
    fn mutate_yul_assignment_operator(
        &mut self,
        source: &YulAssignmentOperator,
    ) -> YulAssignmentOperator {
        self.default_mutate_yul_assignment_operator(source)
    }

    fn default_mutate_yul_stack_assignment_operator(
        &mut self,
        source: &YulStackAssignmentOperator,
    ) -> YulStackAssignmentOperator {
        match source {
            YulStackAssignmentOperator::YulEqualAndColon(ref yul_equal_and_colon) => {
                YulStackAssignmentOperator::YulEqualAndColon(
                    self.mutate_yul_equal_and_colon(yul_equal_and_colon),
                )
            }
            YulStackAssignmentOperator::EqualColon => YulStackAssignmentOperator::EqualColon,
        }
    }
    fn mutate_yul_stack_assignment_operator(
        &mut self,
        source: &YulStackAssignmentOperator,
    ) -> YulStackAssignmentOperator {
        self.default_mutate_yul_stack_assignment_operator(source)
    }

    fn default_mutate_yul_switch_case(&mut self, source: &YulSwitchCase) -> YulSwitchCase {
        match source {
            YulSwitchCase::YulDefaultCase(ref yul_default_case) => {
                YulSwitchCase::YulDefaultCase(self.mutate_yul_default_case(yul_default_case))
            }
            YulSwitchCase::YulValueCase(ref yul_value_case) => {
                YulSwitchCase::YulValueCase(self.mutate_yul_value_case(yul_value_case))
            }
        }
    }
    fn mutate_yul_switch_case(&mut self, source: &YulSwitchCase) -> YulSwitchCase {
        self.default_mutate_yul_switch_case(source)
    }

    fn default_mutate_yul_expression(&mut self, source: &YulExpression) -> YulExpression {
        match source {
            YulExpression::YulFunctionCallExpression(ref yul_function_call_expression) => {
                YulExpression::YulFunctionCallExpression(
                    self.mutate_yul_function_call_expression(yul_function_call_expression),
                )
            }
            YulExpression::YulLiteral(ref yul_literal) => {
                YulExpression::YulLiteral(self.mutate_yul_literal(yul_literal))
            }
            YulExpression::YulPath(ref yul_path) => {
                YulExpression::YulPath(self.mutate_yul_path(yul_path))
            }
        }
    }
    fn mutate_yul_expression(&mut self, source: &YulExpression) -> YulExpression {
        self.default_mutate_yul_expression(source)
    }

    fn default_mutate_yul_literal(&mut self, source: &YulLiteral) -> YulLiteral {
        match source {
            YulLiteral::HexStringLiteral(ref hex_string_literal) => {
                YulLiteral::HexStringLiteral(self.mutate_hex_string_literal(hex_string_literal))
            }
            YulLiteral::StringLiteral(ref string_literal) => {
                YulLiteral::StringLiteral(self.mutate_string_literal(string_literal))
            }
            YulLiteral::YulTrueKeyword => YulLiteral::YulTrueKeyword,
            YulLiteral::YulFalseKeyword => YulLiteral::YulFalseKeyword,
            YulLiteral::YulDecimalLiteral(node) => YulLiteral::YulDecimalLiteral(Rc::clone(node)),
            YulLiteral::YulHexLiteral(node) => YulLiteral::YulHexLiteral(Rc::clone(node)),
        }
    }
    fn mutate_yul_literal(&mut self, source: &YulLiteral) -> YulLiteral {
        self.default_mutate_yul_literal(source)
    }

    //
    // Repeated:
    //

    fn mutate_source_unit_members(&mut self, source: &SourceUnitMembers) -> SourceUnitMembers {
        source
            .iter()
            .map(|item| self.mutate_source_unit_member(item))
            .collect()
    }

    fn mutate_version_expression_set(
        &mut self,
        source: &VersionExpressionSet,
    ) -> VersionExpressionSet {
        source
            .iter()
            .map(|item| self.mutate_version_expression(item))
            .collect()
    }

    fn mutate_contract_members(&mut self, source: &ContractMembers) -> ContractMembers {
        source
            .iter()
            .map(|item| self.mutate_contract_member(item))
            .collect()
    }

    fn mutate_interface_members(&mut self, source: &InterfaceMembers) -> InterfaceMembers {
        source
            .iter()
            .map(|item| self.mutate_contract_member(item))
            .collect()
    }

    fn mutate_library_members(&mut self, source: &LibraryMembers) -> LibraryMembers {
        source
            .iter()
            .map(|item| self.mutate_contract_member(item))
            .collect()
    }

    fn mutate_struct_members(&mut self, source: &StructMembers) -> StructMembers {
        source
            .iter()
            .map(|item| self.mutate_struct_member(item))
            .collect()
    }

    fn mutate_state_variable_attributes(
        &mut self,
        source: &StateVariableAttributes,
    ) -> StateVariableAttributes {
        source
            .iter()
            .map(|item| self.mutate_state_variable_attribute(item))
            .collect()
    }

    fn mutate_function_attributes(&mut self, source: &FunctionAttributes) -> FunctionAttributes {
        source
            .iter()
            .map(|item| self.mutate_function_attribute(item))
            .collect()
    }

    fn mutate_constructor_attributes(
        &mut self,
        source: &ConstructorAttributes,
    ) -> ConstructorAttributes {
        source
            .iter()
            .map(|item| self.mutate_constructor_attribute(item))
            .collect()
    }

    fn mutate_fallback_function_attributes(
        &mut self,
        source: &FallbackFunctionAttributes,
    ) -> FallbackFunctionAttributes {
        source
            .iter()
            .map(|item| self.mutate_fallback_function_attribute(item))
            .collect()
    }

    fn mutate_receive_function_attributes(
        &mut self,
        source: &ReceiveFunctionAttributes,
    ) -> ReceiveFunctionAttributes {
        source
            .iter()
            .map(|item| self.mutate_receive_function_attribute(item))
            .collect()
    }

    fn mutate_modifier_attributes(&mut self, source: &ModifierAttributes) -> ModifierAttributes {
        source
            .iter()
            .map(|item| self.mutate_modifier_attribute(item))
            .collect()
    }

    fn mutate_function_type_attributes(
        &mut self,
        source: &FunctionTypeAttributes,
    ) -> FunctionTypeAttributes {
        source
            .iter()
            .map(|item| self.mutate_function_type_attribute(item))
            .collect()
    }

    fn mutate_statements(&mut self, source: &Statements) -> Statements {
        source
            .iter()
            .map(|item| self.mutate_statement(item))
            .collect()
    }

    fn mutate_catch_clauses(&mut self, source: &CatchClauses) -> CatchClauses {
        source
            .iter()
            .map(|item| self.mutate_catch_clause(item))
            .collect()
    }

    fn mutate_string_literals(&mut self, source: &StringLiterals) -> StringLiterals {
        source
            .iter()
            .map(|item| self.mutate_string_literal(item))
            .collect()
    }

    fn mutate_hex_string_literals(&mut self, source: &HexStringLiterals) -> HexStringLiterals {
        source
            .iter()
            .map(|item| self.mutate_hex_string_literal(item))
            .collect()
    }

    fn mutate_unicode_string_literals(
        &mut self,
        source: &UnicodeStringLiterals,
    ) -> UnicodeStringLiterals {
        source
            .iter()
            .map(|item| self.mutate_unicode_string_literal(item))
            .collect()
    }

    fn mutate_yul_statements(&mut self, source: &YulStatements) -> YulStatements {
        source
            .iter()
            .map(|item| self.mutate_yul_statement(item))
            .collect()
    }

    fn mutate_yul_switch_cases(&mut self, source: &YulSwitchCases) -> YulSwitchCases {
        source
            .iter()
            .map(|item| self.mutate_yul_switch_case(item))
            .collect()
    }

    //
    // Separated:
    //

    fn mutate_version_expression_sets(
        &mut self,
        source: &VersionExpressionSets,
    ) -> VersionExpressionSets {
        source
            .iter()
            .map(|item| self.mutate_version_expression_set(item))
            .collect()
    }

    fn mutate_simple_version_literal(
        &mut self,
        source: &SimpleVersionLiteral,
    ) -> SimpleVersionLiteral {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_import_deconstruction_symbols(
        &mut self,
        source: &ImportDeconstructionSymbols,
    ) -> ImportDeconstructionSymbols {
        source
            .iter()
            .map(|item| self.mutate_import_deconstruction_symbol(item))
            .collect()
    }

    fn mutate_using_deconstruction_symbols(
        &mut self,
        source: &UsingDeconstructionSymbols,
    ) -> UsingDeconstructionSymbols {
        source
            .iter()
            .map(|item| self.mutate_using_deconstruction_symbol(item))
            .collect()
    }

    fn mutate_inheritance_types(&mut self, source: &InheritanceTypes) -> InheritanceTypes {
        source
            .iter()
            .map(|item| self.mutate_inheritance_type(item))
            .collect()
    }

    fn mutate_enum_members(&mut self, source: &EnumMembers) -> EnumMembers {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_parameters(&mut self, source: &Parameters) -> Parameters {
        source
            .iter()
            .map(|item| self.mutate_parameter(item))
            .collect()
    }

    fn mutate_override_paths(&mut self, source: &OverridePaths) -> OverridePaths {
        source
            .iter()
            .map(|item| self.mutate_identifier_path(item))
            .collect()
    }

    fn mutate_event_parameters(&mut self, source: &EventParameters) -> EventParameters {
        source
            .iter()
            .map(|item| self.mutate_event_parameter(item))
            .collect()
    }

    fn mutate_error_parameters(&mut self, source: &ErrorParameters) -> ErrorParameters {
        source
            .iter()
            .map(|item| self.mutate_error_parameter(item))
            .collect()
    }

    fn mutate_assembly_flags(&mut self, source: &AssemblyFlags) -> AssemblyFlags {
        source
            .iter()
            .map(|item| self.mutate_string_literal(item))
            .collect()
    }

    fn mutate_tuple_deconstruction_elements(
        &mut self,
        source: &TupleDeconstructionElements,
    ) -> TupleDeconstructionElements {
        source
            .iter()
            .map(|item| self.mutate_tuple_deconstruction_element(item))
            .collect()
    }

    fn mutate_positional_arguments(&mut self, source: &PositionalArguments) -> PositionalArguments {
        source
            .iter()
            .map(|item| self.mutate_expression(item))
            .collect()
    }

    fn mutate_named_arguments(&mut self, source: &NamedArguments) -> NamedArguments {
        source
            .iter()
            .map(|item| self.mutate_named_argument(item))
            .collect()
    }

    fn mutate_call_options(&mut self, source: &CallOptions) -> CallOptions {
        source
            .iter()
            .map(|item| self.mutate_named_argument(item))
            .collect()
    }

    fn mutate_tuple_values(&mut self, source: &TupleValues) -> TupleValues {
        source
            .iter()
            .map(|item| self.mutate_tuple_value(item))
            .collect()
    }

    fn mutate_array_values(&mut self, source: &ArrayValues) -> ArrayValues {
        source
            .iter()
            .map(|item| self.mutate_expression(item))
            .collect()
    }

    fn mutate_identifier_path(&mut self, source: &IdentifierPath) -> IdentifierPath {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_yul_parameters(&mut self, source: &YulParameters) -> YulParameters {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_yul_variable_names(&mut self, source: &YulVariableNames) -> YulVariableNames {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_yul_arguments(&mut self, source: &YulArguments) -> YulArguments {
        source
            .iter()
            .map(|item| self.mutate_yul_expression(item))
            .collect()
    }

    fn mutate_yul_paths(&mut self, source: &YulPaths) -> YulPaths {
        source
            .iter()
            .map(|item| self.mutate_yul_path(item))
            .collect()
    }

    fn mutate_yul_path(&mut self, source: &YulPath) -> YulPath {
        source.iter().map(Rc::clone).collect()
    }
}
