// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;

pub trait Rewriter {
    //
    // Sequences:
    //

    fn rewrite_source_unit(&mut self, source: &SourceUnit) -> SourceUnit {
        let members = self.rewrite_source_unit_members(&source.members);

        Rc::new(SourceUnitStruct {
            node_id: source.node_id,
            members,
        })
    }

    fn rewrite_pragma_directive(&mut self, source: &PragmaDirective) -> PragmaDirective {
        let pragma = self.rewrite_pragma(&source.pragma);

        Rc::new(PragmaDirectiveStruct {
            node_id: source.node_id,
            pragma,
        })
    }

    fn rewrite_abicoder_pragma(&mut self, source: &AbicoderPragma) -> AbicoderPragma {
        let version = self.rewrite_abicoder_version(&source.version);

        Rc::new(AbicoderPragmaStruct {
            node_id: source.node_id,
            version,
        })
    }

    fn rewrite_experimental_pragma(&mut self, source: &ExperimentalPragma) -> ExperimentalPragma {
        let feature = self.rewrite_experimental_feature(&source.feature);

        Rc::new(ExperimentalPragmaStruct {
            node_id: source.node_id,
            feature,
        })
    }

    fn rewrite_version_pragma(&mut self, source: &VersionPragma) -> VersionPragma {
        let sets = self.rewrite_version_expression_sets(&source.sets);

        Rc::new(VersionPragmaStruct {
            node_id: source.node_id,
            sets,
        })
    }

    fn rewrite_version_range(&mut self, source: &VersionRange) -> VersionRange {
        let start = self.rewrite_version_literal(&source.start);
        let end = self.rewrite_version_literal(&source.end);

        Rc::new(VersionRangeStruct {
            node_id: source.node_id,
            start,
            end,
        })
    }

    fn rewrite_version_term(&mut self, source: &VersionTerm) -> VersionTerm {
        let operator = source
            .operator
            .as_ref()
            .map(|value| self.rewrite_version_operator(value));
        let literal = self.rewrite_version_literal(&source.literal);

        Rc::new(VersionTermStruct {
            node_id: source.node_id,
            operator,
            literal,
        })
    }

    fn rewrite_import_directive(&mut self, source: &ImportDirective) -> ImportDirective {
        let clause = self.rewrite_import_clause(&source.clause);

        Rc::new(ImportDirectiveStruct {
            node_id: source.node_id,
            clause,
        })
    }

    fn rewrite_path_import(&mut self, source: &PathImport) -> PathImport {
        let path = self.rewrite_string_literal(&source.path);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.rewrite_import_alias(value));

        Rc::new(PathImportStruct {
            node_id: source.node_id,
            path,
            alias,
        })
    }

    fn rewrite_named_import(&mut self, source: &NamedImport) -> NamedImport {
        let alias = self.rewrite_import_alias(&source.alias);
        let path = self.rewrite_string_literal(&source.path);

        Rc::new(NamedImportStruct {
            node_id: source.node_id,
            alias,
            path,
        })
    }

    fn rewrite_import_deconstruction(
        &mut self,
        source: &ImportDeconstruction,
    ) -> ImportDeconstruction {
        let symbols = self.rewrite_import_deconstruction_symbols(&source.symbols);
        let path = self.rewrite_string_literal(&source.path);

        Rc::new(ImportDeconstructionStruct {
            node_id: source.node_id,
            symbols,
            path,
        })
    }

    fn rewrite_import_deconstruction_symbol(
        &mut self,
        source: &ImportDeconstructionSymbol,
    ) -> ImportDeconstructionSymbol {
        let name = Rc::clone(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.rewrite_import_alias(value));

        Rc::new(ImportDeconstructionSymbolStruct {
            node_id: source.node_id,
            name,
            alias,
        })
    }

    fn rewrite_import_alias(&mut self, source: &ImportAlias) -> ImportAlias {
        let identifier = Rc::clone(&source.identifier);

        Rc::new(ImportAliasStruct {
            node_id: source.node_id,
            identifier,
        })
    }

    fn rewrite_using_directive(&mut self, source: &UsingDirective) -> UsingDirective {
        let clause = self.rewrite_using_clause(&source.clause);
        let target = self.rewrite_using_target(&source.target);
        let global_keyword = source.global_keyword;

        Rc::new(UsingDirectiveStruct {
            node_id: source.node_id,
            clause,
            target,
            global_keyword,
        })
    }

    fn rewrite_using_deconstruction(
        &mut self,
        source: &UsingDeconstruction,
    ) -> UsingDeconstruction {
        let symbols = self.rewrite_using_deconstruction_symbols(&source.symbols);

        Rc::new(UsingDeconstructionStruct {
            node_id: source.node_id,
            symbols,
        })
    }

    fn rewrite_using_deconstruction_symbol(
        &mut self,
        source: &UsingDeconstructionSymbol,
    ) -> UsingDeconstructionSymbol {
        let name = self.rewrite_identifier_path(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.rewrite_using_alias(value));

        Rc::new(UsingDeconstructionSymbolStruct {
            node_id: source.node_id,
            name,
            alias,
        })
    }

    fn rewrite_using_alias(&mut self, source: &UsingAlias) -> UsingAlias {
        let operator = self.rewrite_using_operator(&source.operator);

        Rc::new(UsingAliasStruct {
            node_id: source.node_id,
            operator,
        })
    }

    fn rewrite_contract_definition(&mut self, source: &ContractDefinition) -> ContractDefinition {
        let abstract_keyword = source.abstract_keyword;
        let name = Rc::clone(&source.name);
        let specifiers = self.rewrite_contract_specifiers(&source.specifiers);
        let members = self.rewrite_contract_members(&source.members);

        Rc::new(ContractDefinitionStruct {
            node_id: source.node_id,
            abstract_keyword,
            name,
            specifiers,
            members,
        })
    }

    fn rewrite_inheritance_specifier(
        &mut self,
        source: &InheritanceSpecifier,
    ) -> InheritanceSpecifier {
        let types = self.rewrite_inheritance_types(&source.types);

        Rc::new(InheritanceSpecifierStruct {
            node_id: source.node_id,
            types,
        })
    }

    fn rewrite_inheritance_type(&mut self, source: &InheritanceType) -> InheritanceType {
        let type_name = self.rewrite_identifier_path(&source.type_name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.rewrite_arguments_declaration(value));

        Rc::new(InheritanceTypeStruct {
            node_id: source.node_id,
            type_name,
            arguments,
        })
    }

    fn rewrite_storage_layout_specifier(
        &mut self,
        source: &StorageLayoutSpecifier,
    ) -> StorageLayoutSpecifier {
        let expression = self.rewrite_expression(&source.expression);

        Rc::new(StorageLayoutSpecifierStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn rewrite_interface_definition(
        &mut self,
        source: &InterfaceDefinition,
    ) -> InterfaceDefinition {
        let name = Rc::clone(&source.name);
        let inheritance = source
            .inheritance
            .as_ref()
            .map(|value| self.rewrite_inheritance_specifier(value));
        let members = self.rewrite_interface_members(&source.members);

        Rc::new(InterfaceDefinitionStruct {
            node_id: source.node_id,
            name,
            inheritance,
            members,
        })
    }

    fn rewrite_library_definition(&mut self, source: &LibraryDefinition) -> LibraryDefinition {
        let name = Rc::clone(&source.name);
        let members = self.rewrite_library_members(&source.members);

        Rc::new(LibraryDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn rewrite_struct_definition(&mut self, source: &StructDefinition) -> StructDefinition {
        let name = Rc::clone(&source.name);
        let members = self.rewrite_struct_members(&source.members);

        Rc::new(StructDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn rewrite_struct_member(&mut self, source: &StructMember) -> StructMember {
        let type_name = self.rewrite_type_name(&source.type_name);
        let name = Rc::clone(&source.name);

        Rc::new(StructMemberStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn rewrite_enum_definition(&mut self, source: &EnumDefinition) -> EnumDefinition {
        let name = Rc::clone(&source.name);
        let members = self.rewrite_enum_members(&source.members);

        Rc::new(EnumDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn rewrite_constant_definition(&mut self, source: &ConstantDefinition) -> ConstantDefinition {
        let type_name = self.rewrite_type_name(&source.type_name);
        let name = Rc::clone(&source.name);
        let value = self.rewrite_expression(&source.value);

        Rc::new(ConstantDefinitionStruct {
            node_id: source.node_id,
            type_name,
            name,
            value,
        })
    }

    fn rewrite_state_variable_definition(
        &mut self,
        source: &StateVariableDefinition,
    ) -> StateVariableDefinition {
        let type_name = self.rewrite_type_name(&source.type_name);
        let attributes = self.rewrite_state_variable_attributes(&source.attributes);
        let name = Rc::clone(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.rewrite_state_variable_definition_value(value));

        Rc::new(StateVariableDefinitionStruct {
            node_id: source.node_id,
            type_name,
            attributes,
            name,
            value,
        })
    }

    fn rewrite_state_variable_definition_value(
        &mut self,
        source: &StateVariableDefinitionValue,
    ) -> StateVariableDefinitionValue {
        let value = self.rewrite_expression(&source.value);

        Rc::new(StateVariableDefinitionValueStruct {
            node_id: source.node_id,
            value,
        })
    }

    fn rewrite_function_definition(&mut self, source: &FunctionDefinition) -> FunctionDefinition {
        let name = self.rewrite_function_name(&source.name);
        let parameters = self.rewrite_parameters_declaration(&source.parameters);
        let attributes = self.rewrite_function_attributes(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.rewrite_returns_declaration(value));
        let body = self.rewrite_function_body(&source.body);

        Rc::new(FunctionDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            attributes,
            returns,
            body,
        })
    }

    fn rewrite_parameters_declaration(
        &mut self,
        source: &ParametersDeclaration,
    ) -> ParametersDeclaration {
        let parameters = self.rewrite_parameters(&source.parameters);

        Rc::new(ParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn rewrite_parameter(&mut self, source: &Parameter) -> Parameter {
        let type_name = self.rewrite_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.rewrite_storage_location(value));
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(ParameterStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
        })
    }

    fn rewrite_override_specifier(&mut self, source: &OverrideSpecifier) -> OverrideSpecifier {
        let overridden = source
            .overridden
            .as_ref()
            .map(|value| self.rewrite_override_paths_declaration(value));

        Rc::new(OverrideSpecifierStruct {
            node_id: source.node_id,
            overridden,
        })
    }

    fn rewrite_override_paths_declaration(
        &mut self,
        source: &OverridePathsDeclaration,
    ) -> OverridePathsDeclaration {
        let paths = self.rewrite_override_paths(&source.paths);

        Rc::new(OverridePathsDeclarationStruct {
            node_id: source.node_id,
            paths,
        })
    }

    fn rewrite_returns_declaration(&mut self, source: &ReturnsDeclaration) -> ReturnsDeclaration {
        let variables = self.rewrite_parameters_declaration(&source.variables);

        Rc::new(ReturnsDeclarationStruct {
            node_id: source.node_id,
            variables,
        })
    }

    fn rewrite_constructor_definition(
        &mut self,
        source: &ConstructorDefinition,
    ) -> ConstructorDefinition {
        let parameters = self.rewrite_parameters_declaration(&source.parameters);
        let attributes = self.rewrite_constructor_attributes(&source.attributes);
        let body = self.rewrite_block(&source.body);

        Rc::new(ConstructorDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            body,
        })
    }

    fn rewrite_unnamed_function_definition(
        &mut self,
        source: &UnnamedFunctionDefinition,
    ) -> UnnamedFunctionDefinition {
        let parameters = self.rewrite_parameters_declaration(&source.parameters);
        let attributes = self.rewrite_unnamed_function_attributes(&source.attributes);
        let body = self.rewrite_function_body(&source.body);

        Rc::new(UnnamedFunctionDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            body,
        })
    }

    fn rewrite_fallback_function_definition(
        &mut self,
        source: &FallbackFunctionDefinition,
    ) -> FallbackFunctionDefinition {
        let parameters = self.rewrite_parameters_declaration(&source.parameters);
        let attributes = self.rewrite_fallback_function_attributes(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.rewrite_returns_declaration(value));
        let body = self.rewrite_function_body(&source.body);

        Rc::new(FallbackFunctionDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            returns,
            body,
        })
    }

    fn rewrite_receive_function_definition(
        &mut self,
        source: &ReceiveFunctionDefinition,
    ) -> ReceiveFunctionDefinition {
        let parameters = self.rewrite_parameters_declaration(&source.parameters);
        let attributes = self.rewrite_receive_function_attributes(&source.attributes);
        let body = self.rewrite_function_body(&source.body);

        Rc::new(ReceiveFunctionDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            body,
        })
    }

    fn rewrite_modifier_definition(&mut self, source: &ModifierDefinition) -> ModifierDefinition {
        let name = Rc::clone(&source.name);
        let parameters = source
            .parameters
            .as_ref()
            .map(|value| self.rewrite_parameters_declaration(value));
        let attributes = self.rewrite_modifier_attributes(&source.attributes);
        let body = self.rewrite_function_body(&source.body);

        Rc::new(ModifierDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            attributes,
            body,
        })
    }

    fn rewrite_modifier_invocation(&mut self, source: &ModifierInvocation) -> ModifierInvocation {
        let name = self.rewrite_identifier_path(&source.name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.rewrite_arguments_declaration(value));

        Rc::new(ModifierInvocationStruct {
            node_id: source.node_id,
            name,
            arguments,
        })
    }

    fn rewrite_event_definition(&mut self, source: &EventDefinition) -> EventDefinition {
        let name = Rc::clone(&source.name);
        let parameters = self.rewrite_event_parameters_declaration(&source.parameters);
        let anonymous_keyword = source.anonymous_keyword;

        Rc::new(EventDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            anonymous_keyword,
        })
    }

    fn rewrite_event_parameters_declaration(
        &mut self,
        source: &EventParametersDeclaration,
    ) -> EventParametersDeclaration {
        let parameters = self.rewrite_event_parameters(&source.parameters);

        Rc::new(EventParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn rewrite_event_parameter(&mut self, source: &EventParameter) -> EventParameter {
        let type_name = self.rewrite_type_name(&source.type_name);
        let indexed_keyword = source.indexed_keyword;
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(EventParameterStruct {
            node_id: source.node_id,
            type_name,
            indexed_keyword,
            name,
        })
    }

    fn rewrite_user_defined_value_type_definition(
        &mut self,
        source: &UserDefinedValueTypeDefinition,
    ) -> UserDefinedValueTypeDefinition {
        let name = Rc::clone(&source.name);
        let value_type = self.rewrite_elementary_type(&source.value_type);

        Rc::new(UserDefinedValueTypeDefinitionStruct {
            node_id: source.node_id,
            name,
            value_type,
        })
    }

    fn rewrite_error_definition(&mut self, source: &ErrorDefinition) -> ErrorDefinition {
        let name = Rc::clone(&source.name);
        let members = self.rewrite_error_parameters_declaration(&source.members);

        Rc::new(ErrorDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn rewrite_error_parameters_declaration(
        &mut self,
        source: &ErrorParametersDeclaration,
    ) -> ErrorParametersDeclaration {
        let parameters = self.rewrite_error_parameters(&source.parameters);

        Rc::new(ErrorParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn rewrite_error_parameter(&mut self, source: &ErrorParameter) -> ErrorParameter {
        let type_name = self.rewrite_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(ErrorParameterStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn rewrite_array_type_name(&mut self, source: &ArrayTypeName) -> ArrayTypeName {
        let operand = self.rewrite_type_name(&source.operand);
        let index = source
            .index
            .as_ref()
            .map(|value| self.rewrite_expression(value));

        Rc::new(ArrayTypeNameStruct {
            node_id: source.node_id,
            operand,
            index,
        })
    }

    fn rewrite_function_type(&mut self, source: &FunctionType) -> FunctionType {
        let parameters = self.rewrite_parameters_declaration(&source.parameters);
        let attributes = self.rewrite_function_type_attributes(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.rewrite_returns_declaration(value));

        Rc::new(FunctionTypeStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            returns,
        })
    }

    fn rewrite_mapping_type(&mut self, source: &MappingType) -> MappingType {
        let key_type = self.rewrite_mapping_key(&source.key_type);
        let value_type = self.rewrite_mapping_value(&source.value_type);

        Rc::new(MappingTypeStruct {
            node_id: source.node_id,
            key_type,
            value_type,
        })
    }

    fn rewrite_mapping_key(&mut self, source: &MappingKey) -> MappingKey {
        let key_type = self.rewrite_mapping_key_type(&source.key_type);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(MappingKeyStruct {
            node_id: source.node_id,
            key_type,
            name,
        })
    }

    fn rewrite_mapping_value(&mut self, source: &MappingValue) -> MappingValue {
        let type_name = self.rewrite_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(MappingValueStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn rewrite_address_type(&mut self, source: &AddressType) -> AddressType {
        let payable_keyword = source.payable_keyword;

        Rc::new(AddressTypeStruct {
            node_id: source.node_id,
            payable_keyword,
        })
    }

    fn rewrite_block(&mut self, source: &Block) -> Block {
        let statements = self.rewrite_statements(&source.statements);

        Rc::new(BlockStruct {
            node_id: source.node_id,
            statements,
        })
    }

    fn rewrite_unchecked_block(&mut self, source: &UncheckedBlock) -> UncheckedBlock {
        let block = self.rewrite_block(&source.block);

        Rc::new(UncheckedBlockStruct {
            node_id: source.node_id,
            block,
        })
    }

    fn rewrite_expression_statement(
        &mut self,
        source: &ExpressionStatement,
    ) -> ExpressionStatement {
        let expression = self.rewrite_expression(&source.expression);

        Rc::new(ExpressionStatementStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn rewrite_assembly_statement(&mut self, source: &AssemblyStatement) -> AssemblyStatement {
        let label = source
            .label
            .as_ref()
            .map(|value| self.rewrite_string_literal(value));
        let flags = source
            .flags
            .as_ref()
            .map(|value| self.rewrite_assembly_flags_declaration(value));
        let body = self.rewrite_yul_block(&source.body);

        Rc::new(AssemblyStatementStruct {
            node_id: source.node_id,
            label,
            flags,
            body,
        })
    }

    fn rewrite_assembly_flags_declaration(
        &mut self,
        source: &AssemblyFlagsDeclaration,
    ) -> AssemblyFlagsDeclaration {
        let flags = self.rewrite_assembly_flags(&source.flags);

        Rc::new(AssemblyFlagsDeclarationStruct {
            node_id: source.node_id,
            flags,
        })
    }

    fn rewrite_tuple_deconstruction_statement(
        &mut self,
        source: &TupleDeconstructionStatement,
    ) -> TupleDeconstructionStatement {
        let target = self.rewrite_tuple_deconstruction_target(&source.target);
        let expression = self.rewrite_expression(&source.expression);

        Rc::new(TupleDeconstructionStatementStruct {
            node_id: source.node_id,
            target,
            expression,
        })
    }

    fn rewrite_var_tuple_deconstruction_target(
        &mut self,
        source: &VarTupleDeconstructionTarget,
    ) -> VarTupleDeconstructionTarget {
        let elements = self.rewrite_untyped_tuple_deconstruction_elements(&source.elements);

        Rc::new(VarTupleDeconstructionTargetStruct {
            node_id: source.node_id,
            elements,
        })
    }

    fn rewrite_untyped_tuple_deconstruction_element(
        &mut self,
        source: &UntypedTupleDeconstructionElement,
    ) -> UntypedTupleDeconstructionElement {
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(UntypedTupleDeconstructionElementStruct {
            node_id: source.node_id,
            name,
        })
    }

    fn rewrite_typed_tuple_deconstruction_target(
        &mut self,
        source: &TypedTupleDeconstructionTarget,
    ) -> TypedTupleDeconstructionTarget {
        let elements = self.rewrite_typed_tuple_deconstruction_elements(&source.elements);

        Rc::new(TypedTupleDeconstructionTargetStruct {
            node_id: source.node_id,
            elements,
        })
    }

    fn rewrite_typed_tuple_deconstruction_element(
        &mut self,
        source: &TypedTupleDeconstructionElement,
    ) -> TypedTupleDeconstructionElement {
        let member = source
            .member
            .as_ref()
            .map(|value| self.rewrite_typed_tuple_deconstruction_member(value));

        Rc::new(TypedTupleDeconstructionElementStruct {
            node_id: source.node_id,
            member,
        })
    }

    fn rewrite_typed_tuple_deconstruction_member(
        &mut self,
        source: &TypedTupleDeconstructionMember,
    ) -> TypedTupleDeconstructionMember {
        let type_name = self.rewrite_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.rewrite_storage_location(value));
        let name = Rc::clone(&source.name);

        Rc::new(TypedTupleDeconstructionMemberStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
        })
    }

    fn rewrite_variable_declaration_statement(
        &mut self,
        source: &VariableDeclarationStatement,
    ) -> VariableDeclarationStatement {
        let variable_type = self.rewrite_variable_declaration_type(&source.variable_type);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.rewrite_storage_location(value));
        let name = Rc::clone(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.rewrite_variable_declaration_value(value));

        Rc::new(VariableDeclarationStatementStruct {
            node_id: source.node_id,
            variable_type,
            storage_location,
            name,
            value,
        })
    }

    fn rewrite_variable_declaration_value(
        &mut self,
        source: &VariableDeclarationValue,
    ) -> VariableDeclarationValue {
        let expression = self.rewrite_expression(&source.expression);

        Rc::new(VariableDeclarationValueStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn rewrite_if_statement(&mut self, source: &IfStatement) -> IfStatement {
        let condition = self.rewrite_expression(&source.condition);
        let body = self.rewrite_statement(&source.body);
        let else_branch = source
            .else_branch
            .as_ref()
            .map(|value| self.rewrite_else_branch(value));

        Rc::new(IfStatementStruct {
            node_id: source.node_id,
            condition,
            body,
            else_branch,
        })
    }

    fn rewrite_else_branch(&mut self, source: &ElseBranch) -> ElseBranch {
        let body = self.rewrite_statement(&source.body);

        Rc::new(ElseBranchStruct {
            node_id: source.node_id,
            body,
        })
    }

    fn rewrite_for_statement(&mut self, source: &ForStatement) -> ForStatement {
        let initialization = self.rewrite_for_statement_initialization(&source.initialization);
        let condition = self.rewrite_for_statement_condition(&source.condition);
        let iterator = source
            .iterator
            .as_ref()
            .map(|value| self.rewrite_expression(value));
        let body = self.rewrite_statement(&source.body);

        Rc::new(ForStatementStruct {
            node_id: source.node_id,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    fn rewrite_while_statement(&mut self, source: &WhileStatement) -> WhileStatement {
        let condition = self.rewrite_expression(&source.condition);
        let body = self.rewrite_statement(&source.body);

        Rc::new(WhileStatementStruct {
            node_id: source.node_id,
            condition,
            body,
        })
    }

    fn rewrite_do_while_statement(&mut self, source: &DoWhileStatement) -> DoWhileStatement {
        let body = self.rewrite_statement(&source.body);
        let condition = self.rewrite_expression(&source.condition);

        Rc::new(DoWhileStatementStruct {
            node_id: source.node_id,
            body,
            condition,
        })
    }

    fn rewrite_continue_statement(&mut self, source: &ContinueStatement) -> ContinueStatement {
        Rc::new(ContinueStatementStruct {
            node_id: source.node_id,
        })
    }

    fn rewrite_break_statement(&mut self, source: &BreakStatement) -> BreakStatement {
        Rc::new(BreakStatementStruct {
            node_id: source.node_id,
        })
    }

    fn rewrite_return_statement(&mut self, source: &ReturnStatement) -> ReturnStatement {
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.rewrite_expression(value));

        Rc::new(ReturnStatementStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn rewrite_emit_statement(&mut self, source: &EmitStatement) -> EmitStatement {
        let event = self.rewrite_identifier_path(&source.event);
        let arguments = self.rewrite_arguments_declaration(&source.arguments);

        Rc::new(EmitStatementStruct {
            node_id: source.node_id,
            event,
            arguments,
        })
    }

    fn rewrite_try_statement(&mut self, source: &TryStatement) -> TryStatement {
        let expression = self.rewrite_expression(&source.expression);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.rewrite_returns_declaration(value));
        let body = self.rewrite_block(&source.body);
        let catch_clauses = self.rewrite_catch_clauses(&source.catch_clauses);

        Rc::new(TryStatementStruct {
            node_id: source.node_id,
            expression,
            returns,
            body,
            catch_clauses,
        })
    }

    fn rewrite_catch_clause(&mut self, source: &CatchClause) -> CatchClause {
        let error = source
            .error
            .as_ref()
            .map(|value| self.rewrite_catch_clause_error(value));
        let body = self.rewrite_block(&source.body);

        Rc::new(CatchClauseStruct {
            node_id: source.node_id,
            error,
            body,
        })
    }

    fn rewrite_catch_clause_error(&mut self, source: &CatchClauseError) -> CatchClauseError {
        let name = source.name.as_ref().map(Rc::clone);
        let parameters = self.rewrite_parameters_declaration(&source.parameters);

        Rc::new(CatchClauseErrorStruct {
            node_id: source.node_id,
            name,
            parameters,
        })
    }

    fn rewrite_revert_statement(&mut self, source: &RevertStatement) -> RevertStatement {
        let error = self.rewrite_identifier_path(&source.error);
        let arguments = self.rewrite_arguments_declaration(&source.arguments);

        Rc::new(RevertStatementStruct {
            node_id: source.node_id,
            error,
            arguments,
        })
    }

    fn rewrite_throw_statement(&mut self, source: &ThrowStatement) -> ThrowStatement {
        Rc::new(ThrowStatementStruct {
            node_id: source.node_id,
        })
    }

    fn rewrite_assignment_expression(
        &mut self,
        source: &AssignmentExpression,
    ) -> AssignmentExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(AssignmentExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn rewrite_conditional_expression(
        &mut self,
        source: &ConditionalExpression,
    ) -> ConditionalExpression {
        let operand = self.rewrite_expression(&source.operand);
        let true_expression = self.rewrite_expression(&source.true_expression);
        let false_expression = self.rewrite_expression(&source.false_expression);

        Rc::new(ConditionalExpressionStruct {
            node_id: source.node_id,
            operand,
            true_expression,
            false_expression,
        })
    }

    fn rewrite_or_expression(&mut self, source: &OrExpression) -> OrExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(OrExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn rewrite_and_expression(&mut self, source: &AndExpression) -> AndExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(AndExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn rewrite_equality_expression(&mut self, source: &EqualityExpression) -> EqualityExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(EqualityExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn rewrite_inequality_expression(
        &mut self,
        source: &InequalityExpression,
    ) -> InequalityExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(InequalityExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn rewrite_bitwise_or_expression(
        &mut self,
        source: &BitwiseOrExpression,
    ) -> BitwiseOrExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(BitwiseOrExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn rewrite_bitwise_xor_expression(
        &mut self,
        source: &BitwiseXorExpression,
    ) -> BitwiseXorExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(BitwiseXorExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn rewrite_bitwise_and_expression(
        &mut self,
        source: &BitwiseAndExpression,
    ) -> BitwiseAndExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(BitwiseAndExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn rewrite_shift_expression(&mut self, source: &ShiftExpression) -> ShiftExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(ShiftExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn rewrite_additive_expression(&mut self, source: &AdditiveExpression) -> AdditiveExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(AdditiveExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn rewrite_multiplicative_expression(
        &mut self,
        source: &MultiplicativeExpression,
    ) -> MultiplicativeExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(MultiplicativeExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn rewrite_exponentiation_expression(
        &mut self,
        source: &ExponentiationExpression,
    ) -> ExponentiationExpression {
        let left_operand = self.rewrite_expression(&source.left_operand);
        let operator = Rc::clone(&source.operator);
        let right_operand = self.rewrite_expression(&source.right_operand);

        Rc::new(ExponentiationExpressionStruct {
            node_id: source.node_id,
            left_operand,
            operator,
            right_operand,
        })
    }

    fn rewrite_postfix_expression(&mut self, source: &PostfixExpression) -> PostfixExpression {
        let operand = self.rewrite_expression(&source.operand);
        let operator = Rc::clone(&source.operator);

        Rc::new(PostfixExpressionStruct {
            node_id: source.node_id,
            operand,
            operator,
        })
    }

    fn rewrite_prefix_expression(&mut self, source: &PrefixExpression) -> PrefixExpression {
        let operator = Rc::clone(&source.operator);
        let operand = self.rewrite_expression(&source.operand);

        Rc::new(PrefixExpressionStruct {
            node_id: source.node_id,
            operator,
            operand,
        })
    }

    fn rewrite_function_call_expression(
        &mut self,
        source: &FunctionCallExpression,
    ) -> FunctionCallExpression {
        let operand = self.rewrite_expression(&source.operand);
        let arguments = self.rewrite_arguments_declaration(&source.arguments);

        Rc::new(FunctionCallExpressionStruct {
            node_id: source.node_id,
            operand,
            arguments,
        })
    }

    fn rewrite_call_options_expression(
        &mut self,
        source: &CallOptionsExpression,
    ) -> CallOptionsExpression {
        let operand = self.rewrite_expression(&source.operand);
        let options = self.rewrite_call_options(&source.options);

        Rc::new(CallOptionsExpressionStruct {
            node_id: source.node_id,
            operand,
            options,
        })
    }

    fn rewrite_member_access_expression(
        &mut self,
        source: &MemberAccessExpression,
    ) -> MemberAccessExpression {
        let operand = self.rewrite_expression(&source.operand);
        let member = Rc::clone(&source.member);

        Rc::new(MemberAccessExpressionStruct {
            node_id: source.node_id,
            operand,
            member,
        })
    }

    fn rewrite_index_access_expression(
        &mut self,
        source: &IndexAccessExpression,
    ) -> IndexAccessExpression {
        let operand = self.rewrite_expression(&source.operand);
        let start = source
            .start
            .as_ref()
            .map(|value| self.rewrite_expression(value));
        let end = source
            .end
            .as_ref()
            .map(|value| self.rewrite_index_access_end(value));

        Rc::new(IndexAccessExpressionStruct {
            node_id: source.node_id,
            operand,
            start,
            end,
        })
    }

    fn rewrite_index_access_end(&mut self, source: &IndexAccessEnd) -> IndexAccessEnd {
        let end = source
            .end
            .as_ref()
            .map(|value| self.rewrite_expression(value));

        Rc::new(IndexAccessEndStruct {
            node_id: source.node_id,
            end,
        })
    }

    fn rewrite_positional_arguments_declaration(
        &mut self,
        source: &PositionalArgumentsDeclaration,
    ) -> PositionalArgumentsDeclaration {
        let arguments = self.rewrite_positional_arguments(&source.arguments);

        Rc::new(PositionalArgumentsDeclarationStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn rewrite_named_arguments_declaration(
        &mut self,
        source: &NamedArgumentsDeclaration,
    ) -> NamedArgumentsDeclaration {
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.rewrite_named_argument_group(value));

        Rc::new(NamedArgumentsDeclarationStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn rewrite_named_argument_group(&mut self, source: &NamedArgumentGroup) -> NamedArgumentGroup {
        let arguments = self.rewrite_named_arguments(&source.arguments);

        Rc::new(NamedArgumentGroupStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn rewrite_named_argument(&mut self, source: &NamedArgument) -> NamedArgument {
        let name = Rc::clone(&source.name);
        let value = self.rewrite_expression(&source.value);

        Rc::new(NamedArgumentStruct {
            node_id: source.node_id,
            name,
            value,
        })
    }

    fn rewrite_type_expression(&mut self, source: &TypeExpression) -> TypeExpression {
        let type_name = self.rewrite_type_name(&source.type_name);

        Rc::new(TypeExpressionStruct {
            node_id: source.node_id,
            type_name,
        })
    }

    fn rewrite_new_expression(&mut self, source: &NewExpression) -> NewExpression {
        let type_name = self.rewrite_type_name(&source.type_name);

        Rc::new(NewExpressionStruct {
            node_id: source.node_id,
            type_name,
        })
    }

    fn rewrite_tuple_expression(&mut self, source: &TupleExpression) -> TupleExpression {
        let items = self.rewrite_tuple_values(&source.items);

        Rc::new(TupleExpressionStruct {
            node_id: source.node_id,
            items,
        })
    }

    fn rewrite_tuple_value(&mut self, source: &TupleValue) -> TupleValue {
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.rewrite_expression(value));

        Rc::new(TupleValueStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn rewrite_array_expression(&mut self, source: &ArrayExpression) -> ArrayExpression {
        let items = self.rewrite_array_values(&source.items);

        Rc::new(ArrayExpressionStruct {
            node_id: source.node_id,
            items,
        })
    }

    fn rewrite_hex_number_expression(
        &mut self,
        source: &HexNumberExpression,
    ) -> HexNumberExpression {
        let literal = Rc::clone(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.rewrite_number_unit(value));

        Rc::new(HexNumberExpressionStruct {
            node_id: source.node_id,
            literal,
            unit,
        })
    }

    fn rewrite_decimal_number_expression(
        &mut self,
        source: &DecimalNumberExpression,
    ) -> DecimalNumberExpression {
        let literal = Rc::clone(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.rewrite_number_unit(value));

        Rc::new(DecimalNumberExpressionStruct {
            node_id: source.node_id,
            literal,
            unit,
        })
    }

    fn rewrite_yul_block(&mut self, source: &YulBlock) -> YulBlock {
        let statements = self.rewrite_yul_statements(&source.statements);

        Rc::new(YulBlockStruct {
            node_id: source.node_id,
            statements,
        })
    }

    fn rewrite_yul_function_definition(
        &mut self,
        source: &YulFunctionDefinition,
    ) -> YulFunctionDefinition {
        let name = Rc::clone(&source.name);
        let parameters = self.rewrite_yul_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.rewrite_yul_returns_declaration(value));
        let body = self.rewrite_yul_block(&source.body);

        Rc::new(YulFunctionDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            returns,
            body,
        })
    }

    fn rewrite_yul_parameters_declaration(
        &mut self,
        source: &YulParametersDeclaration,
    ) -> YulParametersDeclaration {
        let parameters = self.rewrite_yul_parameters(&source.parameters);

        Rc::new(YulParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn rewrite_yul_returns_declaration(
        &mut self,
        source: &YulReturnsDeclaration,
    ) -> YulReturnsDeclaration {
        let variables = self.rewrite_yul_variable_names(&source.variables);

        Rc::new(YulReturnsDeclarationStruct {
            node_id: source.node_id,
            variables,
        })
    }

    fn rewrite_yul_variable_declaration_statement(
        &mut self,
        source: &YulVariableDeclarationStatement,
    ) -> YulVariableDeclarationStatement {
        let variables = self.rewrite_yul_variable_names(&source.variables);
        let value = source
            .value
            .as_ref()
            .map(|value| self.rewrite_yul_variable_declaration_value(value));

        Rc::new(YulVariableDeclarationStatementStruct {
            node_id: source.node_id,
            variables,
            value,
        })
    }

    fn rewrite_yul_variable_declaration_value(
        &mut self,
        source: &YulVariableDeclarationValue,
    ) -> YulVariableDeclarationValue {
        let assignment = self.rewrite_yul_assignment_operator(&source.assignment);
        let expression = self.rewrite_yul_expression(&source.expression);

        Rc::new(YulVariableDeclarationValueStruct {
            node_id: source.node_id,
            assignment,
            expression,
        })
    }

    fn rewrite_yul_variable_assignment_statement(
        &mut self,
        source: &YulVariableAssignmentStatement,
    ) -> YulVariableAssignmentStatement {
        let variables = self.rewrite_yul_paths(&source.variables);
        let assignment = self.rewrite_yul_assignment_operator(&source.assignment);
        let expression = self.rewrite_yul_expression(&source.expression);

        Rc::new(YulVariableAssignmentStatementStruct {
            node_id: source.node_id,
            variables,
            assignment,
            expression,
        })
    }

    fn rewrite_yul_colon_and_equal(&mut self, source: &YulColonAndEqual) -> YulColonAndEqual {
        Rc::new(YulColonAndEqualStruct {
            node_id: source.node_id,
        })
    }

    fn rewrite_yul_stack_assignment_statement(
        &mut self,
        source: &YulStackAssignmentStatement,
    ) -> YulStackAssignmentStatement {
        let assignment = self.rewrite_yul_stack_assignment_operator(&source.assignment);
        let variable = Rc::clone(&source.variable);

        Rc::new(YulStackAssignmentStatementStruct {
            node_id: source.node_id,
            assignment,
            variable,
        })
    }

    fn rewrite_yul_equal_and_colon(&mut self, source: &YulEqualAndColon) -> YulEqualAndColon {
        Rc::new(YulEqualAndColonStruct {
            node_id: source.node_id,
        })
    }

    fn rewrite_yul_if_statement(&mut self, source: &YulIfStatement) -> YulIfStatement {
        let condition = self.rewrite_yul_expression(&source.condition);
        let body = self.rewrite_yul_block(&source.body);

        Rc::new(YulIfStatementStruct {
            node_id: source.node_id,
            condition,
            body,
        })
    }

    fn rewrite_yul_for_statement(&mut self, source: &YulForStatement) -> YulForStatement {
        let initialization = self.rewrite_yul_block(&source.initialization);
        let condition = self.rewrite_yul_expression(&source.condition);
        let iterator = self.rewrite_yul_block(&source.iterator);
        let body = self.rewrite_yul_block(&source.body);

        Rc::new(YulForStatementStruct {
            node_id: source.node_id,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    fn rewrite_yul_switch_statement(&mut self, source: &YulSwitchStatement) -> YulSwitchStatement {
        let expression = self.rewrite_yul_expression(&source.expression);
        let cases = self.rewrite_yul_switch_cases(&source.cases);

        Rc::new(YulSwitchStatementStruct {
            node_id: source.node_id,
            expression,
            cases,
        })
    }

    fn rewrite_yul_default_case(&mut self, source: &YulDefaultCase) -> YulDefaultCase {
        let body = self.rewrite_yul_block(&source.body);

        Rc::new(YulDefaultCaseStruct {
            node_id: source.node_id,
            body,
        })
    }

    fn rewrite_yul_value_case(&mut self, source: &YulValueCase) -> YulValueCase {
        let value = self.rewrite_yul_literal(&source.value);
        let body = self.rewrite_yul_block(&source.body);

        Rc::new(YulValueCaseStruct {
            node_id: source.node_id,
            value,
            body,
        })
    }

    fn rewrite_yul_leave_statement(&mut self, source: &YulLeaveStatement) -> YulLeaveStatement {
        Rc::new(YulLeaveStatementStruct {
            node_id: source.node_id,
        })
    }

    fn rewrite_yul_break_statement(&mut self, source: &YulBreakStatement) -> YulBreakStatement {
        Rc::new(YulBreakStatementStruct {
            node_id: source.node_id,
        })
    }

    fn rewrite_yul_continue_statement(
        &mut self,
        source: &YulContinueStatement,
    ) -> YulContinueStatement {
        Rc::new(YulContinueStatementStruct {
            node_id: source.node_id,
        })
    }

    fn rewrite_yul_label(&mut self, source: &YulLabel) -> YulLabel {
        let label = Rc::clone(&source.label);

        Rc::new(YulLabelStruct {
            node_id: source.node_id,
            label,
        })
    }

    fn rewrite_yul_function_call_expression(
        &mut self,
        source: &YulFunctionCallExpression,
    ) -> YulFunctionCallExpression {
        let operand = self.rewrite_yul_expression(&source.operand);
        let arguments = self.rewrite_yul_arguments(&source.arguments);

        Rc::new(YulFunctionCallExpressionStruct {
            node_id: source.node_id,
            operand,
            arguments,
        })
    }

    //
    // Choices:
    //

    fn default_rewrite_source_unit_member(
        &mut self,
        source: &SourceUnitMember,
    ) -> SourceUnitMember {
        match source {
            SourceUnitMember::PragmaDirective(ref pragma_directive) => {
                SourceUnitMember::PragmaDirective(self.rewrite_pragma_directive(pragma_directive))
            }
            SourceUnitMember::ImportDirective(ref import_directive) => {
                SourceUnitMember::ImportDirective(self.rewrite_import_directive(import_directive))
            }
            SourceUnitMember::ContractDefinition(ref contract_definition) => {
                SourceUnitMember::ContractDefinition(
                    self.rewrite_contract_definition(contract_definition),
                )
            }
            SourceUnitMember::InterfaceDefinition(ref interface_definition) => {
                SourceUnitMember::InterfaceDefinition(
                    self.rewrite_interface_definition(interface_definition),
                )
            }
            SourceUnitMember::LibraryDefinition(ref library_definition) => {
                SourceUnitMember::LibraryDefinition(
                    self.rewrite_library_definition(library_definition),
                )
            }
            SourceUnitMember::StructDefinition(ref struct_definition) => {
                SourceUnitMember::StructDefinition(
                    self.rewrite_struct_definition(struct_definition),
                )
            }
            SourceUnitMember::EnumDefinition(ref enum_definition) => {
                SourceUnitMember::EnumDefinition(self.rewrite_enum_definition(enum_definition))
            }
            SourceUnitMember::FunctionDefinition(ref function_definition) => {
                SourceUnitMember::FunctionDefinition(
                    self.rewrite_function_definition(function_definition),
                )
            }
            SourceUnitMember::ErrorDefinition(ref error_definition) => {
                SourceUnitMember::ErrorDefinition(self.rewrite_error_definition(error_definition))
            }
            SourceUnitMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => SourceUnitMember::UserDefinedValueTypeDefinition(
                self.rewrite_user_defined_value_type_definition(user_defined_value_type_definition),
            ),
            SourceUnitMember::UsingDirective(ref using_directive) => {
                SourceUnitMember::UsingDirective(self.rewrite_using_directive(using_directive))
            }
            SourceUnitMember::EventDefinition(ref event_definition) => {
                SourceUnitMember::EventDefinition(self.rewrite_event_definition(event_definition))
            }
            SourceUnitMember::ConstantDefinition(ref constant_definition) => {
                SourceUnitMember::ConstantDefinition(
                    self.rewrite_constant_definition(constant_definition),
                )
            }
        }
    }
    fn rewrite_source_unit_member(&mut self, source: &SourceUnitMember) -> SourceUnitMember {
        self.default_rewrite_source_unit_member(source)
    }

    fn default_rewrite_pragma(&mut self, source: &Pragma) -> Pragma {
        match source {
            Pragma::VersionPragma(ref version_pragma) => {
                Pragma::VersionPragma(self.rewrite_version_pragma(version_pragma))
            }
            Pragma::AbicoderPragma(ref abicoder_pragma) => {
                Pragma::AbicoderPragma(self.rewrite_abicoder_pragma(abicoder_pragma))
            }
            Pragma::ExperimentalPragma(ref experimental_pragma) => {
                Pragma::ExperimentalPragma(self.rewrite_experimental_pragma(experimental_pragma))
            }
        }
    }
    fn rewrite_pragma(&mut self, source: &Pragma) -> Pragma {
        self.default_rewrite_pragma(source)
    }

    fn default_rewrite_abicoder_version(&mut self, source: &AbicoderVersion) -> AbicoderVersion {
        match source {
            AbicoderVersion::AbicoderV1Keyword => AbicoderVersion::AbicoderV1Keyword,
            AbicoderVersion::AbicoderV2Keyword => AbicoderVersion::AbicoderV2Keyword,
        }
    }
    fn rewrite_abicoder_version(&mut self, source: &AbicoderVersion) -> AbicoderVersion {
        self.default_rewrite_abicoder_version(source)
    }

    fn default_rewrite_experimental_feature(
        &mut self,
        source: &ExperimentalFeature,
    ) -> ExperimentalFeature {
        match source {
            ExperimentalFeature::StringLiteral(ref string_literal) => {
                ExperimentalFeature::StringLiteral(self.rewrite_string_literal(string_literal))
            }
            ExperimentalFeature::ABIEncoderV2Keyword => ExperimentalFeature::ABIEncoderV2Keyword,
            ExperimentalFeature::SMTCheckerKeyword => ExperimentalFeature::SMTCheckerKeyword,
        }
    }
    fn rewrite_experimental_feature(
        &mut self,
        source: &ExperimentalFeature,
    ) -> ExperimentalFeature {
        self.default_rewrite_experimental_feature(source)
    }

    fn default_rewrite_version_expression(
        &mut self,
        source: &VersionExpression,
    ) -> VersionExpression {
        match source {
            VersionExpression::VersionRange(ref version_range) => {
                VersionExpression::VersionRange(self.rewrite_version_range(version_range))
            }
            VersionExpression::VersionTerm(ref version_term) => {
                VersionExpression::VersionTerm(self.rewrite_version_term(version_term))
            }
        }
    }
    fn rewrite_version_expression(&mut self, source: &VersionExpression) -> VersionExpression {
        self.default_rewrite_version_expression(source)
    }

    fn default_rewrite_version_operator(&mut self, source: &VersionOperator) -> VersionOperator {
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
    fn rewrite_version_operator(&mut self, source: &VersionOperator) -> VersionOperator {
        self.default_rewrite_version_operator(source)
    }

    fn default_rewrite_version_literal(&mut self, source: &VersionLiteral) -> VersionLiteral {
        match source {
            VersionLiteral::SimpleVersionLiteral(ref simple_version_literal) => {
                VersionLiteral::SimpleVersionLiteral(
                    self.rewrite_simple_version_literal(simple_version_literal),
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
    fn rewrite_version_literal(&mut self, source: &VersionLiteral) -> VersionLiteral {
        self.default_rewrite_version_literal(source)
    }

    fn default_rewrite_import_clause(&mut self, source: &ImportClause) -> ImportClause {
        match source {
            ImportClause::PathImport(ref path_import) => {
                ImportClause::PathImport(self.rewrite_path_import(path_import))
            }
            ImportClause::NamedImport(ref named_import) => {
                ImportClause::NamedImport(self.rewrite_named_import(named_import))
            }
            ImportClause::ImportDeconstruction(ref import_deconstruction) => {
                ImportClause::ImportDeconstruction(
                    self.rewrite_import_deconstruction(import_deconstruction),
                )
            }
        }
    }
    fn rewrite_import_clause(&mut self, source: &ImportClause) -> ImportClause {
        self.default_rewrite_import_clause(source)
    }

    fn default_rewrite_using_clause(&mut self, source: &UsingClause) -> UsingClause {
        match source {
            UsingClause::IdentifierPath(ref identifier_path) => {
                UsingClause::IdentifierPath(self.rewrite_identifier_path(identifier_path))
            }
            UsingClause::UsingDeconstruction(ref using_deconstruction) => {
                UsingClause::UsingDeconstruction(
                    self.rewrite_using_deconstruction(using_deconstruction),
                )
            }
        }
    }
    fn rewrite_using_clause(&mut self, source: &UsingClause) -> UsingClause {
        self.default_rewrite_using_clause(source)
    }

    fn default_rewrite_using_operator(&mut self, source: &UsingOperator) -> UsingOperator {
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
    fn rewrite_using_operator(&mut self, source: &UsingOperator) -> UsingOperator {
        self.default_rewrite_using_operator(source)
    }

    fn default_rewrite_using_target(&mut self, source: &UsingTarget) -> UsingTarget {
        match source {
            UsingTarget::TypeName(ref type_name) => {
                UsingTarget::TypeName(self.rewrite_type_name(type_name))
            }
            UsingTarget::Asterisk => UsingTarget::Asterisk,
        }
    }
    fn rewrite_using_target(&mut self, source: &UsingTarget) -> UsingTarget {
        self.default_rewrite_using_target(source)
    }

    fn default_rewrite_contract_specifier(
        &mut self,
        source: &ContractSpecifier,
    ) -> ContractSpecifier {
        match source {
            ContractSpecifier::InheritanceSpecifier(ref inheritance_specifier) => {
                ContractSpecifier::InheritanceSpecifier(
                    self.rewrite_inheritance_specifier(inheritance_specifier),
                )
            }
            ContractSpecifier::StorageLayoutSpecifier(ref storage_layout_specifier) => {
                ContractSpecifier::StorageLayoutSpecifier(
                    self.rewrite_storage_layout_specifier(storage_layout_specifier),
                )
            }
        }
    }
    fn rewrite_contract_specifier(&mut self, source: &ContractSpecifier) -> ContractSpecifier {
        self.default_rewrite_contract_specifier(source)
    }

    fn default_rewrite_contract_member(&mut self, source: &ContractMember) -> ContractMember {
        match source {
            ContractMember::UsingDirective(ref using_directive) => {
                ContractMember::UsingDirective(self.rewrite_using_directive(using_directive))
            }
            ContractMember::FunctionDefinition(ref function_definition) => {
                ContractMember::FunctionDefinition(
                    self.rewrite_function_definition(function_definition),
                )
            }
            ContractMember::ConstructorDefinition(ref constructor_definition) => {
                ContractMember::ConstructorDefinition(
                    self.rewrite_constructor_definition(constructor_definition),
                )
            }
            ContractMember::ReceiveFunctionDefinition(ref receive_function_definition) => {
                ContractMember::ReceiveFunctionDefinition(
                    self.rewrite_receive_function_definition(receive_function_definition),
                )
            }
            ContractMember::FallbackFunctionDefinition(ref fallback_function_definition) => {
                ContractMember::FallbackFunctionDefinition(
                    self.rewrite_fallback_function_definition(fallback_function_definition),
                )
            }
            ContractMember::UnnamedFunctionDefinition(ref unnamed_function_definition) => {
                ContractMember::UnnamedFunctionDefinition(
                    self.rewrite_unnamed_function_definition(unnamed_function_definition),
                )
            }
            ContractMember::ModifierDefinition(ref modifier_definition) => {
                ContractMember::ModifierDefinition(
                    self.rewrite_modifier_definition(modifier_definition),
                )
            }
            ContractMember::StructDefinition(ref struct_definition) => {
                ContractMember::StructDefinition(self.rewrite_struct_definition(struct_definition))
            }
            ContractMember::EnumDefinition(ref enum_definition) => {
                ContractMember::EnumDefinition(self.rewrite_enum_definition(enum_definition))
            }
            ContractMember::EventDefinition(ref event_definition) => {
                ContractMember::EventDefinition(self.rewrite_event_definition(event_definition))
            }
            ContractMember::ErrorDefinition(ref error_definition) => {
                ContractMember::ErrorDefinition(self.rewrite_error_definition(error_definition))
            }
            ContractMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => ContractMember::UserDefinedValueTypeDefinition(
                self.rewrite_user_defined_value_type_definition(user_defined_value_type_definition),
            ),
            ContractMember::StateVariableDefinition(ref state_variable_definition) => {
                ContractMember::StateVariableDefinition(
                    self.rewrite_state_variable_definition(state_variable_definition),
                )
            }
        }
    }
    fn rewrite_contract_member(&mut self, source: &ContractMember) -> ContractMember {
        self.default_rewrite_contract_member(source)
    }

    fn default_rewrite_state_variable_attribute(
        &mut self,
        source: &StateVariableAttribute,
    ) -> StateVariableAttribute {
        match source {
            StateVariableAttribute::OverrideSpecifier(ref override_specifier) => {
                StateVariableAttribute::OverrideSpecifier(
                    self.rewrite_override_specifier(override_specifier),
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
    fn rewrite_state_variable_attribute(
        &mut self,
        source: &StateVariableAttribute,
    ) -> StateVariableAttribute {
        self.default_rewrite_state_variable_attribute(source)
    }

    fn default_rewrite_function_name(&mut self, source: &FunctionName) -> FunctionName {
        match source {
            FunctionName::Identifier(node) => FunctionName::Identifier(Rc::clone(node)),
            FunctionName::FallbackKeyword => FunctionName::FallbackKeyword,
            FunctionName::ReceiveKeyword => FunctionName::ReceiveKeyword,
        }
    }
    fn rewrite_function_name(&mut self, source: &FunctionName) -> FunctionName {
        self.default_rewrite_function_name(source)
    }

    fn default_rewrite_function_attribute(
        &mut self,
        source: &FunctionAttribute,
    ) -> FunctionAttribute {
        match source {
            FunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                FunctionAttribute::ModifierInvocation(
                    self.rewrite_modifier_invocation(modifier_invocation),
                )
            }
            FunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                FunctionAttribute::OverrideSpecifier(
                    self.rewrite_override_specifier(override_specifier),
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
    fn rewrite_function_attribute(&mut self, source: &FunctionAttribute) -> FunctionAttribute {
        self.default_rewrite_function_attribute(source)
    }

    fn default_rewrite_function_body(&mut self, source: &FunctionBody) -> FunctionBody {
        match source {
            FunctionBody::Block(ref block) => FunctionBody::Block(self.rewrite_block(block)),
            FunctionBody::Semicolon => FunctionBody::Semicolon,
        }
    }
    fn rewrite_function_body(&mut self, source: &FunctionBody) -> FunctionBody {
        self.default_rewrite_function_body(source)
    }

    fn default_rewrite_constructor_attribute(
        &mut self,
        source: &ConstructorAttribute,
    ) -> ConstructorAttribute {
        match source {
            ConstructorAttribute::ModifierInvocation(ref modifier_invocation) => {
                ConstructorAttribute::ModifierInvocation(
                    self.rewrite_modifier_invocation(modifier_invocation),
                )
            }
            ConstructorAttribute::InternalKeyword => ConstructorAttribute::InternalKeyword,
            ConstructorAttribute::OverrideKeyword => ConstructorAttribute::OverrideKeyword,
            ConstructorAttribute::PayableKeyword => ConstructorAttribute::PayableKeyword,
            ConstructorAttribute::PublicKeyword => ConstructorAttribute::PublicKeyword,
            ConstructorAttribute::VirtualKeyword => ConstructorAttribute::VirtualKeyword,
        }
    }
    fn rewrite_constructor_attribute(
        &mut self,
        source: &ConstructorAttribute,
    ) -> ConstructorAttribute {
        self.default_rewrite_constructor_attribute(source)
    }

    fn default_rewrite_unnamed_function_attribute(
        &mut self,
        source: &UnnamedFunctionAttribute,
    ) -> UnnamedFunctionAttribute {
        match source {
            UnnamedFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                UnnamedFunctionAttribute::ModifierInvocation(
                    self.rewrite_modifier_invocation(modifier_invocation),
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
    fn rewrite_unnamed_function_attribute(
        &mut self,
        source: &UnnamedFunctionAttribute,
    ) -> UnnamedFunctionAttribute {
        self.default_rewrite_unnamed_function_attribute(source)
    }

    fn default_rewrite_fallback_function_attribute(
        &mut self,
        source: &FallbackFunctionAttribute,
    ) -> FallbackFunctionAttribute {
        match source {
            FallbackFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                FallbackFunctionAttribute::ModifierInvocation(
                    self.rewrite_modifier_invocation(modifier_invocation),
                )
            }
            FallbackFunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                FallbackFunctionAttribute::OverrideSpecifier(
                    self.rewrite_override_specifier(override_specifier),
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
    fn rewrite_fallback_function_attribute(
        &mut self,
        source: &FallbackFunctionAttribute,
    ) -> FallbackFunctionAttribute {
        self.default_rewrite_fallback_function_attribute(source)
    }

    fn default_rewrite_receive_function_attribute(
        &mut self,
        source: &ReceiveFunctionAttribute,
    ) -> ReceiveFunctionAttribute {
        match source {
            ReceiveFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                ReceiveFunctionAttribute::ModifierInvocation(
                    self.rewrite_modifier_invocation(modifier_invocation),
                )
            }
            ReceiveFunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                ReceiveFunctionAttribute::OverrideSpecifier(
                    self.rewrite_override_specifier(override_specifier),
                )
            }
            ReceiveFunctionAttribute::ExternalKeyword => ReceiveFunctionAttribute::ExternalKeyword,
            ReceiveFunctionAttribute::PayableKeyword => ReceiveFunctionAttribute::PayableKeyword,
            ReceiveFunctionAttribute::VirtualKeyword => ReceiveFunctionAttribute::VirtualKeyword,
        }
    }
    fn rewrite_receive_function_attribute(
        &mut self,
        source: &ReceiveFunctionAttribute,
    ) -> ReceiveFunctionAttribute {
        self.default_rewrite_receive_function_attribute(source)
    }

    fn default_rewrite_modifier_attribute(
        &mut self,
        source: &ModifierAttribute,
    ) -> ModifierAttribute {
        match source {
            ModifierAttribute::OverrideSpecifier(ref override_specifier) => {
                ModifierAttribute::OverrideSpecifier(
                    self.rewrite_override_specifier(override_specifier),
                )
            }
            ModifierAttribute::VirtualKeyword => ModifierAttribute::VirtualKeyword,
        }
    }
    fn rewrite_modifier_attribute(&mut self, source: &ModifierAttribute) -> ModifierAttribute {
        self.default_rewrite_modifier_attribute(source)
    }

    fn default_rewrite_type_name(&mut self, source: &TypeName) -> TypeName {
        match source {
            TypeName::ArrayTypeName(ref array_type_name) => {
                TypeName::ArrayTypeName(self.rewrite_array_type_name(array_type_name))
            }
            TypeName::FunctionType(ref function_type) => {
                TypeName::FunctionType(self.rewrite_function_type(function_type))
            }
            TypeName::MappingType(ref mapping_type) => {
                TypeName::MappingType(self.rewrite_mapping_type(mapping_type))
            }
            TypeName::ElementaryType(ref elementary_type) => {
                TypeName::ElementaryType(self.rewrite_elementary_type(elementary_type))
            }
            TypeName::IdentifierPath(ref identifier_path) => {
                TypeName::IdentifierPath(self.rewrite_identifier_path(identifier_path))
            }
        }
    }
    fn rewrite_type_name(&mut self, source: &TypeName) -> TypeName {
        self.default_rewrite_type_name(source)
    }

    fn default_rewrite_function_type_attribute(
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
    fn rewrite_function_type_attribute(
        &mut self,
        source: &FunctionTypeAttribute,
    ) -> FunctionTypeAttribute {
        self.default_rewrite_function_type_attribute(source)
    }

    fn default_rewrite_mapping_key_type(&mut self, source: &MappingKeyType) -> MappingKeyType {
        match source {
            MappingKeyType::ElementaryType(ref elementary_type) => {
                MappingKeyType::ElementaryType(self.rewrite_elementary_type(elementary_type))
            }
            MappingKeyType::IdentifierPath(ref identifier_path) => {
                MappingKeyType::IdentifierPath(self.rewrite_identifier_path(identifier_path))
            }
        }
    }
    fn rewrite_mapping_key_type(&mut self, source: &MappingKeyType) -> MappingKeyType {
        self.default_rewrite_mapping_key_type(source)
    }

    fn default_rewrite_elementary_type(&mut self, source: &ElementaryType) -> ElementaryType {
        match source {
            ElementaryType::AddressType(ref address_type) => {
                ElementaryType::AddressType(self.rewrite_address_type(address_type))
            }
            ElementaryType::BytesKeyword(node) => ElementaryType::BytesKeyword(Rc::clone(node)),
            ElementaryType::IntKeyword(node) => ElementaryType::IntKeyword(Rc::clone(node)),
            ElementaryType::UintKeyword(node) => ElementaryType::UintKeyword(Rc::clone(node)),
            ElementaryType::FixedKeyword(node) => ElementaryType::FixedKeyword(Rc::clone(node)),
            ElementaryType::UfixedKeyword(node) => ElementaryType::UfixedKeyword(Rc::clone(node)),
            ElementaryType::BoolKeyword => ElementaryType::BoolKeyword,
            ElementaryType::ByteKeyword => ElementaryType::ByteKeyword,
            ElementaryType::StringKeyword => ElementaryType::StringKeyword,
        }
    }
    fn rewrite_elementary_type(&mut self, source: &ElementaryType) -> ElementaryType {
        self.default_rewrite_elementary_type(source)
    }

    fn default_rewrite_statement(&mut self, source: &Statement) -> Statement {
        match source {
            Statement::IfStatement(ref if_statement) => {
                Statement::IfStatement(self.rewrite_if_statement(if_statement))
            }
            Statement::ForStatement(ref for_statement) => {
                Statement::ForStatement(self.rewrite_for_statement(for_statement))
            }
            Statement::WhileStatement(ref while_statement) => {
                Statement::WhileStatement(self.rewrite_while_statement(while_statement))
            }
            Statement::DoWhileStatement(ref do_while_statement) => {
                Statement::DoWhileStatement(self.rewrite_do_while_statement(do_while_statement))
            }
            Statement::ContinueStatement(ref continue_statement) => {
                Statement::ContinueStatement(self.rewrite_continue_statement(continue_statement))
            }
            Statement::BreakStatement(ref break_statement) => {
                Statement::BreakStatement(self.rewrite_break_statement(break_statement))
            }
            Statement::ReturnStatement(ref return_statement) => {
                Statement::ReturnStatement(self.rewrite_return_statement(return_statement))
            }
            Statement::ThrowStatement(ref throw_statement) => {
                Statement::ThrowStatement(self.rewrite_throw_statement(throw_statement))
            }
            Statement::EmitStatement(ref emit_statement) => {
                Statement::EmitStatement(self.rewrite_emit_statement(emit_statement))
            }
            Statement::TryStatement(ref try_statement) => {
                Statement::TryStatement(self.rewrite_try_statement(try_statement))
            }
            Statement::RevertStatement(ref revert_statement) => {
                Statement::RevertStatement(self.rewrite_revert_statement(revert_statement))
            }
            Statement::AssemblyStatement(ref assembly_statement) => {
                Statement::AssemblyStatement(self.rewrite_assembly_statement(assembly_statement))
            }
            Statement::Block(ref block) => Statement::Block(self.rewrite_block(block)),
            Statement::UncheckedBlock(ref unchecked_block) => {
                Statement::UncheckedBlock(self.rewrite_unchecked_block(unchecked_block))
            }
            Statement::TupleDeconstructionStatement(ref tuple_deconstruction_statement) => {
                Statement::TupleDeconstructionStatement(
                    self.rewrite_tuple_deconstruction_statement(tuple_deconstruction_statement),
                )
            }
            Statement::VariableDeclarationStatement(ref variable_declaration_statement) => {
                Statement::VariableDeclarationStatement(
                    self.rewrite_variable_declaration_statement(variable_declaration_statement),
                )
            }
            Statement::ExpressionStatement(ref expression_statement) => {
                Statement::ExpressionStatement(
                    self.rewrite_expression_statement(expression_statement),
                )
            }
        }
    }
    fn rewrite_statement(&mut self, source: &Statement) -> Statement {
        self.default_rewrite_statement(source)
    }

    fn default_rewrite_tuple_deconstruction_target(
        &mut self,
        source: &TupleDeconstructionTarget,
    ) -> TupleDeconstructionTarget {
        match source {
            TupleDeconstructionTarget::VarTupleDeconstructionTarget(
                ref var_tuple_deconstruction_target,
            ) => TupleDeconstructionTarget::VarTupleDeconstructionTarget(
                self.rewrite_var_tuple_deconstruction_target(var_tuple_deconstruction_target),
            ),
            TupleDeconstructionTarget::TypedTupleDeconstructionTarget(
                ref typed_tuple_deconstruction_target,
            ) => TupleDeconstructionTarget::TypedTupleDeconstructionTarget(
                self.rewrite_typed_tuple_deconstruction_target(typed_tuple_deconstruction_target),
            ),
        }
    }
    fn rewrite_tuple_deconstruction_target(
        &mut self,
        source: &TupleDeconstructionTarget,
    ) -> TupleDeconstructionTarget {
        self.default_rewrite_tuple_deconstruction_target(source)
    }

    fn default_rewrite_variable_declaration_type(
        &mut self,
        source: &VariableDeclarationType,
    ) -> VariableDeclarationType {
        match source {
            VariableDeclarationType::TypeName(ref type_name) => {
                VariableDeclarationType::TypeName(self.rewrite_type_name(type_name))
            }
            VariableDeclarationType::VarKeyword => VariableDeclarationType::VarKeyword,
        }
    }
    fn rewrite_variable_declaration_type(
        &mut self,
        source: &VariableDeclarationType,
    ) -> VariableDeclarationType {
        self.default_rewrite_variable_declaration_type(source)
    }

    fn default_rewrite_storage_location(&mut self, source: &StorageLocation) -> StorageLocation {
        match source {
            StorageLocation::MemoryKeyword => StorageLocation::MemoryKeyword,
            StorageLocation::StorageKeyword => StorageLocation::StorageKeyword,
            StorageLocation::CallDataKeyword => StorageLocation::CallDataKeyword,
        }
    }
    fn rewrite_storage_location(&mut self, source: &StorageLocation) -> StorageLocation {
        self.default_rewrite_storage_location(source)
    }

    fn default_rewrite_for_statement_initialization(
        &mut self,
        source: &ForStatementInitialization,
    ) -> ForStatementInitialization {
        match source {
            ForStatementInitialization::TupleDeconstructionStatement(
                ref tuple_deconstruction_statement,
            ) => ForStatementInitialization::TupleDeconstructionStatement(
                self.rewrite_tuple_deconstruction_statement(tuple_deconstruction_statement),
            ),
            ForStatementInitialization::VariableDeclarationStatement(
                ref variable_declaration_statement,
            ) => ForStatementInitialization::VariableDeclarationStatement(
                self.rewrite_variable_declaration_statement(variable_declaration_statement),
            ),
            ForStatementInitialization::ExpressionStatement(ref expression_statement) => {
                ForStatementInitialization::ExpressionStatement(
                    self.rewrite_expression_statement(expression_statement),
                )
            }
            ForStatementInitialization::Semicolon => ForStatementInitialization::Semicolon,
        }
    }
    fn rewrite_for_statement_initialization(
        &mut self,
        source: &ForStatementInitialization,
    ) -> ForStatementInitialization {
        self.default_rewrite_for_statement_initialization(source)
    }

    fn default_rewrite_for_statement_condition(
        &mut self,
        source: &ForStatementCondition,
    ) -> ForStatementCondition {
        match source {
            ForStatementCondition::ExpressionStatement(ref expression_statement) => {
                ForStatementCondition::ExpressionStatement(
                    self.rewrite_expression_statement(expression_statement),
                )
            }
            ForStatementCondition::Semicolon => ForStatementCondition::Semicolon,
        }
    }
    fn rewrite_for_statement_condition(
        &mut self,
        source: &ForStatementCondition,
    ) -> ForStatementCondition {
        self.default_rewrite_for_statement_condition(source)
    }

    fn default_rewrite_expression(&mut self, source: &Expression) -> Expression {
        match source {
            Expression::AssignmentExpression(ref assignment_expression) => {
                Expression::AssignmentExpression(
                    self.rewrite_assignment_expression(assignment_expression),
                )
            }
            Expression::ConditionalExpression(ref conditional_expression) => {
                Expression::ConditionalExpression(
                    self.rewrite_conditional_expression(conditional_expression),
                )
            }
            Expression::OrExpression(ref or_expression) => {
                Expression::OrExpression(self.rewrite_or_expression(or_expression))
            }
            Expression::AndExpression(ref and_expression) => {
                Expression::AndExpression(self.rewrite_and_expression(and_expression))
            }
            Expression::EqualityExpression(ref equality_expression) => {
                Expression::EqualityExpression(
                    self.rewrite_equality_expression(equality_expression),
                )
            }
            Expression::InequalityExpression(ref inequality_expression) => {
                Expression::InequalityExpression(
                    self.rewrite_inequality_expression(inequality_expression),
                )
            }
            Expression::BitwiseOrExpression(ref bitwise_or_expression) => {
                Expression::BitwiseOrExpression(
                    self.rewrite_bitwise_or_expression(bitwise_or_expression),
                )
            }
            Expression::BitwiseXorExpression(ref bitwise_xor_expression) => {
                Expression::BitwiseXorExpression(
                    self.rewrite_bitwise_xor_expression(bitwise_xor_expression),
                )
            }
            Expression::BitwiseAndExpression(ref bitwise_and_expression) => {
                Expression::BitwiseAndExpression(
                    self.rewrite_bitwise_and_expression(bitwise_and_expression),
                )
            }
            Expression::ShiftExpression(ref shift_expression) => {
                Expression::ShiftExpression(self.rewrite_shift_expression(shift_expression))
            }
            Expression::AdditiveExpression(ref additive_expression) => {
                Expression::AdditiveExpression(
                    self.rewrite_additive_expression(additive_expression),
                )
            }
            Expression::MultiplicativeExpression(ref multiplicative_expression) => {
                Expression::MultiplicativeExpression(
                    self.rewrite_multiplicative_expression(multiplicative_expression),
                )
            }
            Expression::ExponentiationExpression(ref exponentiation_expression) => {
                Expression::ExponentiationExpression(
                    self.rewrite_exponentiation_expression(exponentiation_expression),
                )
            }
            Expression::PostfixExpression(ref postfix_expression) => {
                Expression::PostfixExpression(self.rewrite_postfix_expression(postfix_expression))
            }
            Expression::PrefixExpression(ref prefix_expression) => {
                Expression::PrefixExpression(self.rewrite_prefix_expression(prefix_expression))
            }
            Expression::FunctionCallExpression(ref function_call_expression) => {
                Expression::FunctionCallExpression(
                    self.rewrite_function_call_expression(function_call_expression),
                )
            }
            Expression::CallOptionsExpression(ref call_options_expression) => {
                Expression::CallOptionsExpression(
                    self.rewrite_call_options_expression(call_options_expression),
                )
            }
            Expression::MemberAccessExpression(ref member_access_expression) => {
                Expression::MemberAccessExpression(
                    self.rewrite_member_access_expression(member_access_expression),
                )
            }
            Expression::IndexAccessExpression(ref index_access_expression) => {
                Expression::IndexAccessExpression(
                    self.rewrite_index_access_expression(index_access_expression),
                )
            }
            Expression::NewExpression(ref new_expression) => {
                Expression::NewExpression(self.rewrite_new_expression(new_expression))
            }
            Expression::TupleExpression(ref tuple_expression) => {
                Expression::TupleExpression(self.rewrite_tuple_expression(tuple_expression))
            }
            Expression::TypeExpression(ref type_expression) => {
                Expression::TypeExpression(self.rewrite_type_expression(type_expression))
            }
            Expression::ArrayExpression(ref array_expression) => {
                Expression::ArrayExpression(self.rewrite_array_expression(array_expression))
            }
            Expression::HexNumberExpression(ref hex_number_expression) => {
                Expression::HexNumberExpression(
                    self.rewrite_hex_number_expression(hex_number_expression),
                )
            }
            Expression::DecimalNumberExpression(ref decimal_number_expression) => {
                Expression::DecimalNumberExpression(
                    self.rewrite_decimal_number_expression(decimal_number_expression),
                )
            }
            Expression::StringExpression(ref string_expression) => {
                Expression::StringExpression(self.rewrite_string_expression(string_expression))
            }
            Expression::ElementaryType(ref elementary_type) => {
                Expression::ElementaryType(self.rewrite_elementary_type(elementary_type))
            }
            Expression::Identifier(node) => Expression::Identifier(Rc::clone(node)),
            Expression::PayableKeyword => Expression::PayableKeyword,
            Expression::ThisKeyword => Expression::ThisKeyword,
            Expression::SuperKeyword => Expression::SuperKeyword,
            Expression::TrueKeyword => Expression::TrueKeyword,
            Expression::FalseKeyword => Expression::FalseKeyword,
        }
    }
    fn rewrite_expression(&mut self, source: &Expression) -> Expression {
        self.default_rewrite_expression(source)
    }

    fn default_rewrite_arguments_declaration(
        &mut self,
        source: &ArgumentsDeclaration,
    ) -> ArgumentsDeclaration {
        match source {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(
                ref positional_arguments_declaration,
            ) => ArgumentsDeclaration::PositionalArgumentsDeclaration(
                self.rewrite_positional_arguments_declaration(positional_arguments_declaration),
            ),
            ArgumentsDeclaration::NamedArgumentsDeclaration(ref named_arguments_declaration) => {
                ArgumentsDeclaration::NamedArgumentsDeclaration(
                    self.rewrite_named_arguments_declaration(named_arguments_declaration),
                )
            }
        }
    }
    fn rewrite_arguments_declaration(
        &mut self,
        source: &ArgumentsDeclaration,
    ) -> ArgumentsDeclaration {
        self.default_rewrite_arguments_declaration(source)
    }

    fn default_rewrite_number_unit(&mut self, source: &NumberUnit) -> NumberUnit {
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
    fn rewrite_number_unit(&mut self, source: &NumberUnit) -> NumberUnit {
        self.default_rewrite_number_unit(source)
    }

    fn default_rewrite_string_expression(&mut self, source: &StringExpression) -> StringExpression {
        match source {
            StringExpression::StringLiteral(ref string_literal) => {
                StringExpression::StringLiteral(self.rewrite_string_literal(string_literal))
            }
            StringExpression::StringLiterals(ref string_literals) => {
                StringExpression::StringLiterals(self.rewrite_string_literals(string_literals))
            }
            StringExpression::HexStringLiteral(ref hex_string_literal) => {
                StringExpression::HexStringLiteral(
                    self.rewrite_hex_string_literal(hex_string_literal),
                )
            }
            StringExpression::HexStringLiterals(ref hex_string_literals) => {
                StringExpression::HexStringLiterals(
                    self.rewrite_hex_string_literals(hex_string_literals),
                )
            }
            StringExpression::UnicodeStringLiterals(ref unicode_string_literals) => {
                StringExpression::UnicodeStringLiterals(
                    self.rewrite_unicode_string_literals(unicode_string_literals),
                )
            }
        }
    }
    fn rewrite_string_expression(&mut self, source: &StringExpression) -> StringExpression {
        self.default_rewrite_string_expression(source)
    }

    fn default_rewrite_string_literal(&mut self, source: &StringLiteral) -> StringLiteral {
        match source {
            StringLiteral::SingleQuotedStringLiteral(node) => {
                StringLiteral::SingleQuotedStringLiteral(Rc::clone(node))
            }
            StringLiteral::DoubleQuotedStringLiteral(node) => {
                StringLiteral::DoubleQuotedStringLiteral(Rc::clone(node))
            }
        }
    }
    fn rewrite_string_literal(&mut self, source: &StringLiteral) -> StringLiteral {
        self.default_rewrite_string_literal(source)
    }

    fn default_rewrite_hex_string_literal(
        &mut self,
        source: &HexStringLiteral,
    ) -> HexStringLiteral {
        match source {
            HexStringLiteral::SingleQuotedHexStringLiteral(node) => {
                HexStringLiteral::SingleQuotedHexStringLiteral(Rc::clone(node))
            }
            HexStringLiteral::DoubleQuotedHexStringLiteral(node) => {
                HexStringLiteral::DoubleQuotedHexStringLiteral(Rc::clone(node))
            }
        }
    }
    fn rewrite_hex_string_literal(&mut self, source: &HexStringLiteral) -> HexStringLiteral {
        self.default_rewrite_hex_string_literal(source)
    }

    fn default_rewrite_unicode_string_literal(
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
    fn rewrite_unicode_string_literal(
        &mut self,
        source: &UnicodeStringLiteral,
    ) -> UnicodeStringLiteral {
        self.default_rewrite_unicode_string_literal(source)
    }

    fn default_rewrite_yul_statement(&mut self, source: &YulStatement) -> YulStatement {
        match source {
            YulStatement::YulBlock(ref yul_block) => {
                YulStatement::YulBlock(self.rewrite_yul_block(yul_block))
            }
            YulStatement::YulFunctionDefinition(ref yul_function_definition) => {
                YulStatement::YulFunctionDefinition(
                    self.rewrite_yul_function_definition(yul_function_definition),
                )
            }
            YulStatement::YulStackAssignmentStatement(ref yul_stack_assignment_statement) => {
                YulStatement::YulStackAssignmentStatement(
                    self.rewrite_yul_stack_assignment_statement(yul_stack_assignment_statement),
                )
            }
            YulStatement::YulIfStatement(ref yul_if_statement) => {
                YulStatement::YulIfStatement(self.rewrite_yul_if_statement(yul_if_statement))
            }
            YulStatement::YulForStatement(ref yul_for_statement) => {
                YulStatement::YulForStatement(self.rewrite_yul_for_statement(yul_for_statement))
            }
            YulStatement::YulSwitchStatement(ref yul_switch_statement) => {
                YulStatement::YulSwitchStatement(
                    self.rewrite_yul_switch_statement(yul_switch_statement),
                )
            }
            YulStatement::YulLeaveStatement(ref yul_leave_statement) => {
                YulStatement::YulLeaveStatement(
                    self.rewrite_yul_leave_statement(yul_leave_statement),
                )
            }
            YulStatement::YulBreakStatement(ref yul_break_statement) => {
                YulStatement::YulBreakStatement(
                    self.rewrite_yul_break_statement(yul_break_statement),
                )
            }
            YulStatement::YulContinueStatement(ref yul_continue_statement) => {
                YulStatement::YulContinueStatement(
                    self.rewrite_yul_continue_statement(yul_continue_statement),
                )
            }
            YulStatement::YulVariableAssignmentStatement(ref yul_variable_assignment_statement) => {
                YulStatement::YulVariableAssignmentStatement(
                    self.rewrite_yul_variable_assignment_statement(
                        yul_variable_assignment_statement,
                    ),
                )
            }
            YulStatement::YulLabel(ref yul_label) => {
                YulStatement::YulLabel(self.rewrite_yul_label(yul_label))
            }
            YulStatement::YulVariableDeclarationStatement(
                ref yul_variable_declaration_statement,
            ) => YulStatement::YulVariableDeclarationStatement(
                self.rewrite_yul_variable_declaration_statement(yul_variable_declaration_statement),
            ),
            YulStatement::YulExpression(ref yul_expression) => {
                YulStatement::YulExpression(self.rewrite_yul_expression(yul_expression))
            }
        }
    }
    fn rewrite_yul_statement(&mut self, source: &YulStatement) -> YulStatement {
        self.default_rewrite_yul_statement(source)
    }

    fn default_rewrite_yul_assignment_operator(
        &mut self,
        source: &YulAssignmentOperator,
    ) -> YulAssignmentOperator {
        match source {
            YulAssignmentOperator::YulColonAndEqual(ref yul_colon_and_equal) => {
                YulAssignmentOperator::YulColonAndEqual(
                    self.rewrite_yul_colon_and_equal(yul_colon_and_equal),
                )
            }
            YulAssignmentOperator::ColonEqual => YulAssignmentOperator::ColonEqual,
        }
    }
    fn rewrite_yul_assignment_operator(
        &mut self,
        source: &YulAssignmentOperator,
    ) -> YulAssignmentOperator {
        self.default_rewrite_yul_assignment_operator(source)
    }

    fn default_rewrite_yul_stack_assignment_operator(
        &mut self,
        source: &YulStackAssignmentOperator,
    ) -> YulStackAssignmentOperator {
        match source {
            YulStackAssignmentOperator::YulEqualAndColon(ref yul_equal_and_colon) => {
                YulStackAssignmentOperator::YulEqualAndColon(
                    self.rewrite_yul_equal_and_colon(yul_equal_and_colon),
                )
            }
            YulStackAssignmentOperator::EqualColon => YulStackAssignmentOperator::EqualColon,
        }
    }
    fn rewrite_yul_stack_assignment_operator(
        &mut self,
        source: &YulStackAssignmentOperator,
    ) -> YulStackAssignmentOperator {
        self.default_rewrite_yul_stack_assignment_operator(source)
    }

    fn default_rewrite_yul_switch_case(&mut self, source: &YulSwitchCase) -> YulSwitchCase {
        match source {
            YulSwitchCase::YulDefaultCase(ref yul_default_case) => {
                YulSwitchCase::YulDefaultCase(self.rewrite_yul_default_case(yul_default_case))
            }
            YulSwitchCase::YulValueCase(ref yul_value_case) => {
                YulSwitchCase::YulValueCase(self.rewrite_yul_value_case(yul_value_case))
            }
        }
    }
    fn rewrite_yul_switch_case(&mut self, source: &YulSwitchCase) -> YulSwitchCase {
        self.default_rewrite_yul_switch_case(source)
    }

    fn default_rewrite_yul_expression(&mut self, source: &YulExpression) -> YulExpression {
        match source {
            YulExpression::YulFunctionCallExpression(ref yul_function_call_expression) => {
                YulExpression::YulFunctionCallExpression(
                    self.rewrite_yul_function_call_expression(yul_function_call_expression),
                )
            }
            YulExpression::YulLiteral(ref yul_literal) => {
                YulExpression::YulLiteral(self.rewrite_yul_literal(yul_literal))
            }
            YulExpression::YulPath(ref yul_path) => {
                YulExpression::YulPath(self.rewrite_yul_path(yul_path))
            }
        }
    }
    fn rewrite_yul_expression(&mut self, source: &YulExpression) -> YulExpression {
        self.default_rewrite_yul_expression(source)
    }

    fn default_rewrite_yul_literal(&mut self, source: &YulLiteral) -> YulLiteral {
        match source {
            YulLiteral::HexStringLiteral(ref hex_string_literal) => {
                YulLiteral::HexStringLiteral(self.rewrite_hex_string_literal(hex_string_literal))
            }
            YulLiteral::StringLiteral(ref string_literal) => {
                YulLiteral::StringLiteral(self.rewrite_string_literal(string_literal))
            }
            YulLiteral::YulDecimalLiteral(node) => YulLiteral::YulDecimalLiteral(Rc::clone(node)),
            YulLiteral::YulHexLiteral(node) => YulLiteral::YulHexLiteral(Rc::clone(node)),
            YulLiteral::YulTrueKeyword => YulLiteral::YulTrueKeyword,
            YulLiteral::YulFalseKeyword => YulLiteral::YulFalseKeyword,
        }
    }
    fn rewrite_yul_literal(&mut self, source: &YulLiteral) -> YulLiteral {
        self.default_rewrite_yul_literal(source)
    }

    //
    // Repeated & Separated
    //

    fn rewrite_source_unit_members(&mut self, source: &SourceUnitMembers) -> SourceUnitMembers {
        source
            .iter()
            .map(|item| self.rewrite_source_unit_member(item))
            .collect()
    }

    fn rewrite_version_expression_sets(
        &mut self,
        source: &VersionExpressionSets,
    ) -> VersionExpressionSets {
        source
            .iter()
            .map(|item| self.rewrite_version_expression_set(item))
            .collect()
    }

    fn rewrite_version_expression_set(
        &mut self,
        source: &VersionExpressionSet,
    ) -> VersionExpressionSet {
        source
            .iter()
            .map(|item| self.rewrite_version_expression(item))
            .collect()
    }

    fn rewrite_simple_version_literal(
        &mut self,
        source: &SimpleVersionLiteral,
    ) -> SimpleVersionLiteral {
        source.iter().map(Rc::clone).collect()
    }

    fn rewrite_import_deconstruction_symbols(
        &mut self,
        source: &ImportDeconstructionSymbols,
    ) -> ImportDeconstructionSymbols {
        source
            .iter()
            .map(|item| self.rewrite_import_deconstruction_symbol(item))
            .collect()
    }

    fn rewrite_using_deconstruction_symbols(
        &mut self,
        source: &UsingDeconstructionSymbols,
    ) -> UsingDeconstructionSymbols {
        source
            .iter()
            .map(|item| self.rewrite_using_deconstruction_symbol(item))
            .collect()
    }

    fn rewrite_contract_specifiers(&mut self, source: &ContractSpecifiers) -> ContractSpecifiers {
        source
            .iter()
            .map(|item| self.rewrite_contract_specifier(item))
            .collect()
    }

    fn rewrite_inheritance_types(&mut self, source: &InheritanceTypes) -> InheritanceTypes {
        source
            .iter()
            .map(|item| self.rewrite_inheritance_type(item))
            .collect()
    }

    fn rewrite_contract_members(&mut self, source: &ContractMembers) -> ContractMembers {
        source
            .iter()
            .map(|item| self.rewrite_contract_member(item))
            .collect()
    }

    fn rewrite_interface_members(&mut self, source: &InterfaceMembers) -> InterfaceMembers {
        source
            .iter()
            .map(|item| self.rewrite_contract_member(item))
            .collect()
    }

    fn rewrite_library_members(&mut self, source: &LibraryMembers) -> LibraryMembers {
        source
            .iter()
            .map(|item| self.rewrite_contract_member(item))
            .collect()
    }

    fn rewrite_struct_members(&mut self, source: &StructMembers) -> StructMembers {
        source
            .iter()
            .map(|item| self.rewrite_struct_member(item))
            .collect()
    }

    fn rewrite_enum_members(&mut self, source: &EnumMembers) -> EnumMembers {
        source.iter().map(Rc::clone).collect()
    }

    fn rewrite_state_variable_attributes(
        &mut self,
        source: &StateVariableAttributes,
    ) -> StateVariableAttributes {
        source
            .iter()
            .map(|item| self.rewrite_state_variable_attribute(item))
            .collect()
    }

    fn rewrite_parameters(&mut self, source: &Parameters) -> Parameters {
        source
            .iter()
            .map(|item| self.rewrite_parameter(item))
            .collect()
    }

    fn rewrite_function_attributes(&mut self, source: &FunctionAttributes) -> FunctionAttributes {
        source
            .iter()
            .map(|item| self.rewrite_function_attribute(item))
            .collect()
    }

    fn rewrite_override_paths(&mut self, source: &OverridePaths) -> OverridePaths {
        source
            .iter()
            .map(|item| self.rewrite_identifier_path(item))
            .collect()
    }

    fn rewrite_constructor_attributes(
        &mut self,
        source: &ConstructorAttributes,
    ) -> ConstructorAttributes {
        source
            .iter()
            .map(|item| self.rewrite_constructor_attribute(item))
            .collect()
    }

    fn rewrite_unnamed_function_attributes(
        &mut self,
        source: &UnnamedFunctionAttributes,
    ) -> UnnamedFunctionAttributes {
        source
            .iter()
            .map(|item| self.rewrite_unnamed_function_attribute(item))
            .collect()
    }

    fn rewrite_fallback_function_attributes(
        &mut self,
        source: &FallbackFunctionAttributes,
    ) -> FallbackFunctionAttributes {
        source
            .iter()
            .map(|item| self.rewrite_fallback_function_attribute(item))
            .collect()
    }

    fn rewrite_receive_function_attributes(
        &mut self,
        source: &ReceiveFunctionAttributes,
    ) -> ReceiveFunctionAttributes {
        source
            .iter()
            .map(|item| self.rewrite_receive_function_attribute(item))
            .collect()
    }

    fn rewrite_modifier_attributes(&mut self, source: &ModifierAttributes) -> ModifierAttributes {
        source
            .iter()
            .map(|item| self.rewrite_modifier_attribute(item))
            .collect()
    }

    fn rewrite_event_parameters(&mut self, source: &EventParameters) -> EventParameters {
        source
            .iter()
            .map(|item| self.rewrite_event_parameter(item))
            .collect()
    }

    fn rewrite_error_parameters(&mut self, source: &ErrorParameters) -> ErrorParameters {
        source
            .iter()
            .map(|item| self.rewrite_error_parameter(item))
            .collect()
    }

    fn rewrite_function_type_attributes(
        &mut self,
        source: &FunctionTypeAttributes,
    ) -> FunctionTypeAttributes {
        source
            .iter()
            .map(|item| self.rewrite_function_type_attribute(item))
            .collect()
    }

    fn rewrite_statements(&mut self, source: &Statements) -> Statements {
        source
            .iter()
            .map(|item| self.rewrite_statement(item))
            .collect()
    }

    fn rewrite_assembly_flags(&mut self, source: &AssemblyFlags) -> AssemblyFlags {
        source
            .iter()
            .map(|item| self.rewrite_string_literal(item))
            .collect()
    }

    fn rewrite_untyped_tuple_deconstruction_elements(
        &mut self,
        source: &UntypedTupleDeconstructionElements,
    ) -> UntypedTupleDeconstructionElements {
        source
            .iter()
            .map(|item| self.rewrite_untyped_tuple_deconstruction_element(item))
            .collect()
    }

    fn rewrite_typed_tuple_deconstruction_elements(
        &mut self,
        source: &TypedTupleDeconstructionElements,
    ) -> TypedTupleDeconstructionElements {
        source
            .iter()
            .map(|item| self.rewrite_typed_tuple_deconstruction_element(item))
            .collect()
    }

    fn rewrite_catch_clauses(&mut self, source: &CatchClauses) -> CatchClauses {
        source
            .iter()
            .map(|item| self.rewrite_catch_clause(item))
            .collect()
    }

    fn rewrite_positional_arguments(
        &mut self,
        source: &PositionalArguments,
    ) -> PositionalArguments {
        source
            .iter()
            .map(|item| self.rewrite_expression(item))
            .collect()
    }

    fn rewrite_named_arguments(&mut self, source: &NamedArguments) -> NamedArguments {
        source
            .iter()
            .map(|item| self.rewrite_named_argument(item))
            .collect()
    }

    fn rewrite_call_options(&mut self, source: &CallOptions) -> CallOptions {
        source
            .iter()
            .map(|item| self.rewrite_named_argument(item))
            .collect()
    }

    fn rewrite_tuple_values(&mut self, source: &TupleValues) -> TupleValues {
        source
            .iter()
            .map(|item| self.rewrite_tuple_value(item))
            .collect()
    }

    fn rewrite_array_values(&mut self, source: &ArrayValues) -> ArrayValues {
        source
            .iter()
            .map(|item| self.rewrite_expression(item))
            .collect()
    }

    fn rewrite_string_literals(&mut self, source: &StringLiterals) -> StringLiterals {
        source
            .iter()
            .map(|item| self.rewrite_string_literal(item))
            .collect()
    }

    fn rewrite_hex_string_literals(&mut self, source: &HexStringLiterals) -> HexStringLiterals {
        source
            .iter()
            .map(|item| self.rewrite_hex_string_literal(item))
            .collect()
    }

    fn rewrite_unicode_string_literals(
        &mut self,
        source: &UnicodeStringLiterals,
    ) -> UnicodeStringLiterals {
        source
            .iter()
            .map(|item| self.rewrite_unicode_string_literal(item))
            .collect()
    }

    fn rewrite_identifier_path(&mut self, source: &IdentifierPath) -> IdentifierPath {
        source.iter().map(Rc::clone).collect()
    }

    fn rewrite_yul_statements(&mut self, source: &YulStatements) -> YulStatements {
        source
            .iter()
            .map(|item| self.rewrite_yul_statement(item))
            .collect()
    }

    fn rewrite_yul_parameters(&mut self, source: &YulParameters) -> YulParameters {
        source.iter().map(Rc::clone).collect()
    }

    fn rewrite_yul_variable_names(&mut self, source: &YulVariableNames) -> YulVariableNames {
        source.iter().map(Rc::clone).collect()
    }

    fn rewrite_yul_switch_cases(&mut self, source: &YulSwitchCases) -> YulSwitchCases {
        source
            .iter()
            .map(|item| self.rewrite_yul_switch_case(item))
            .collect()
    }

    fn rewrite_yul_arguments(&mut self, source: &YulArguments) -> YulArguments {
        source
            .iter()
            .map(|item| self.rewrite_yul_expression(item))
            .collect()
    }

    fn rewrite_yul_paths(&mut self, source: &YulPaths) -> YulPaths {
        source
            .iter()
            .map(|item| self.rewrite_yul_path(item))
            .collect()
    }

    fn rewrite_yul_path(&mut self, source: &YulPath) -> YulPath {
        source.iter().map(Rc::clone).collect()
    }
}
