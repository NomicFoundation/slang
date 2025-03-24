// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// Transformer from previous language implementation `ast`
#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use super::{input, nodes as output};

pub trait Transformer {
    //
    // Sequences:
    //

    fn mutate_source_unit(&mut self, source: &input::SourceUnit) -> output::SourceUnit {
        let members = self.mutate_source_unit_members(&source.members);

        Rc::new(output::SourceUnitStruct {
            node_id: source.node_id,
            members,
        })
    }

    fn mutate_pragma_directive(
        &mut self,
        source: &input::PragmaDirective,
    ) -> output::PragmaDirective {
        let pragma = self.mutate_pragma(&source.pragma);

        Rc::new(output::PragmaDirectiveStruct {
            node_id: source.node_id,
            pragma,
        })
    }

    fn mutate_abicoder_pragma(&mut self, source: &input::AbicoderPragma) -> output::AbicoderPragma {
        let version = Rc::clone(&source.version);

        Rc::new(output::AbicoderPragmaStruct {
            node_id: source.node_id,
            version,
        })
    }

    fn mutate_experimental_pragma(
        &mut self,
        source: &input::ExperimentalPragma,
    ) -> output::ExperimentalPragma {
        let feature = self.mutate_experimental_feature(&source.feature);

        Rc::new(output::ExperimentalPragmaStruct {
            node_id: source.node_id,
            feature,
        })
    }

    fn mutate_version_pragma(&mut self, source: &input::VersionPragma) -> output::VersionPragma {
        let sets = self.mutate_version_expression_sets(&source.sets);

        Rc::new(output::VersionPragmaStruct {
            node_id: source.node_id,
            sets,
        })
    }

    fn mutate_version_range(&mut self, source: &input::VersionRange) -> output::VersionRange {
        let start = self.mutate_version_literal(&source.start);
        let end = self.mutate_version_literal(&source.end);

        Rc::new(output::VersionRangeStruct {
            node_id: source.node_id,
            start,
            end,
        })
    }

    fn mutate_version_term(&mut self, source: &input::VersionTerm) -> output::VersionTerm {
        let operator = source
            .operator
            .as_ref()
            .map(|value| self.mutate_version_operator(value));
        let literal = self.mutate_version_literal(&source.literal);

        Rc::new(output::VersionTermStruct {
            node_id: source.node_id,
            operator,
            literal,
        })
    }

    fn mutate_import_directive(
        &mut self,
        source: &input::ImportDirective,
    ) -> output::ImportDirective {
        let clause = self.mutate_import_clause(&source.clause);

        Rc::new(output::ImportDirectiveStruct {
            node_id: source.node_id,
            clause,
        })
    }

    fn mutate_path_import(&mut self, source: &input::PathImport) -> output::PathImport {
        let path = self.mutate_string_literal(&source.path);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.mutate_import_alias(value));

        Rc::new(output::PathImportStruct {
            node_id: source.node_id,
            path,
            alias,
        })
    }

    fn mutate_named_import(&mut self, source: &input::NamedImport) -> output::NamedImport {
        let alias = self.mutate_import_alias(&source.alias);
        let path = self.mutate_string_literal(&source.path);

        Rc::new(output::NamedImportStruct {
            node_id: source.node_id,
            alias,
            path,
        })
    }

    fn mutate_import_deconstruction(
        &mut self,
        source: &input::ImportDeconstruction,
    ) -> output::ImportDeconstruction {
        let symbols = self.mutate_import_deconstruction_symbols(&source.symbols);
        let path = self.mutate_string_literal(&source.path);

        Rc::new(output::ImportDeconstructionStruct {
            node_id: source.node_id,
            symbols,
            path,
        })
    }

    fn mutate_import_deconstruction_symbol(
        &mut self,
        source: &input::ImportDeconstructionSymbol,
    ) -> output::ImportDeconstructionSymbol {
        let name = Rc::clone(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.mutate_import_alias(value));

        Rc::new(output::ImportDeconstructionSymbolStruct {
            node_id: source.node_id,
            name,
            alias,
        })
    }

    fn mutate_import_alias(&mut self, source: &input::ImportAlias) -> output::ImportAlias {
        let identifier = Rc::clone(&source.identifier);

        Rc::new(output::ImportAliasStruct {
            node_id: source.node_id,
            identifier,
        })
    }

    fn mutate_using_directive(&mut self, source: &input::UsingDirective) -> output::UsingDirective {
        let clause = self.mutate_using_clause(&source.clause);
        let target = self.mutate_using_target(&source.target);
        let global_keyword = source.global_keyword.as_ref().map(Rc::clone);

        Rc::new(output::UsingDirectiveStruct {
            node_id: source.node_id,
            clause,
            target,
            global_keyword,
        })
    }

    fn mutate_using_deconstruction(
        &mut self,
        source: &input::UsingDeconstruction,
    ) -> output::UsingDeconstruction {
        let symbols = self.mutate_using_deconstruction_symbols(&source.symbols);

        Rc::new(output::UsingDeconstructionStruct {
            node_id: source.node_id,
            symbols,
        })
    }

    fn mutate_using_deconstruction_symbol(
        &mut self,
        source: &input::UsingDeconstructionSymbol,
    ) -> output::UsingDeconstructionSymbol {
        let name = self.mutate_identifier_path(&source.name);
        let alias = source
            .alias
            .as_ref()
            .map(|value| self.mutate_using_alias(value));

        Rc::new(output::UsingDeconstructionSymbolStruct {
            node_id: source.node_id,
            name,
            alias,
        })
    }

    fn mutate_using_alias(&mut self, source: &input::UsingAlias) -> output::UsingAlias {
        let operator = self.mutate_using_operator(&source.operator);

        Rc::new(output::UsingAliasStruct {
            node_id: source.node_id,
            operator,
        })
    }

    fn mutate_contract_definition(
        &mut self,
        source: &input::ContractDefinition,
    ) -> output::ContractDefinition {
        let abstract_keyword = source.abstract_keyword.as_ref().map(Rc::clone);
        let name = Rc::clone(&source.name);
        let inheritance = source
            .inheritance
            .as_ref()
            .map(|value| self.mutate_inheritance_specifier(value));
        let members = self.mutate_contract_members(&source.members);

        Rc::new(output::ContractDefinitionStruct {
            node_id: source.node_id,
            abstract_keyword,
            name,
            inheritance,
            members,
        })
    }

    fn mutate_inheritance_specifier(
        &mut self,
        source: &input::InheritanceSpecifier,
    ) -> output::InheritanceSpecifier {
        let types = self.mutate_inheritance_types(&source.types);

        Rc::new(output::InheritanceSpecifierStruct {
            node_id: source.node_id,
            types,
        })
    }

    fn mutate_inheritance_type(
        &mut self,
        source: &input::InheritanceType,
    ) -> output::InheritanceType {
        let type_name = self.mutate_identifier_path(&source.type_name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.mutate_arguments_declaration(value));

        Rc::new(output::InheritanceTypeStruct {
            node_id: source.node_id,
            type_name,
            arguments,
        })
    }

    fn mutate_interface_definition(
        &mut self,
        source: &input::InterfaceDefinition,
    ) -> output::InterfaceDefinition {
        let name = Rc::clone(&source.name);
        let inheritance = source
            .inheritance
            .as_ref()
            .map(|value| self.mutate_inheritance_specifier(value));
        let members = self.mutate_interface_members(&source.members);

        Rc::new(output::InterfaceDefinitionStruct {
            node_id: source.node_id,
            name,
            inheritance,
            members,
        })
    }

    fn mutate_library_definition(
        &mut self,
        source: &input::LibraryDefinition,
    ) -> output::LibraryDefinition {
        let name = Rc::clone(&source.name);
        let members = self.mutate_library_members(&source.members);

        Rc::new(output::LibraryDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn mutate_struct_definition(
        &mut self,
        source: &input::StructDefinition,
    ) -> output::StructDefinition {
        let name = Rc::clone(&source.name);
        let members = self.mutate_struct_members(&source.members);

        Rc::new(output::StructDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn mutate_struct_member(&mut self, source: &input::StructMember) -> output::StructMember {
        let type_name = self.mutate_type_name(&source.type_name);
        let name = Rc::clone(&source.name);

        Rc::new(output::StructMemberStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn mutate_enum_definition(&mut self, source: &input::EnumDefinition) -> output::EnumDefinition {
        let name = Rc::clone(&source.name);
        let members = self.mutate_enum_members(&source.members);

        Rc::new(output::EnumDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn mutate_constant_definition(
        &mut self,
        source: &input::ConstantDefinition,
    ) -> output::ConstantDefinition {
        let type_name = self.mutate_type_name(&source.type_name);
        let name = Rc::clone(&source.name);
        let value = self.mutate_expression(&source.value);

        Rc::new(output::ConstantDefinitionStruct {
            node_id: source.node_id,
            type_name,
            name,
            value,
        })
    }

    fn mutate_state_variable_definition(
        &mut self,
        source: &input::StateVariableDefinition,
    ) -> output::StateVariableDefinition {
        let type_name = self.mutate_type_name(&source.type_name);
        let attributes = self.mutate_state_variable_attributes(&source.attributes);
        let name = Rc::clone(&source.name);
        let value = source
            .value
            .as_ref()
            .map(|value| self.mutate_state_variable_definition_value(value));

        Rc::new(output::StateVariableDefinitionStruct {
            node_id: source.node_id,
            type_name,
            attributes,
            name,
            value,
        })
    }

    fn mutate_state_variable_definition_value(
        &mut self,
        source: &input::StateVariableDefinitionValue,
    ) -> output::StateVariableDefinitionValue {
        let value = self.mutate_expression(&source.value);

        Rc::new(output::StateVariableDefinitionValueStruct {
            node_id: source.node_id,
            value,
        })
    }

    fn mutate_function_definition(
        &mut self,
        source: &input::FunctionDefinition,
    ) -> output::FunctionDefinition {
        let name = self.mutate_function_name(&source.name);
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_function_attributes(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_returns_declaration(value));
        let body = self.mutate_function_body(&source.body);

        Rc::new(output::FunctionDefinitionStruct {
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
        source: &input::ParametersDeclaration,
    ) -> output::ParametersDeclaration {
        let parameters = self.mutate_parameters(&source.parameters);

        Rc::new(output::ParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn mutate_parameter(&mut self, source: &input::Parameter) -> output::Parameter {
        let type_name = self.mutate_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.mutate_storage_location(value));
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::ParameterStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
        })
    }

    fn mutate_override_specifier(
        &mut self,
        source: &input::OverrideSpecifier,
    ) -> output::OverrideSpecifier {
        let overridden = source
            .overridden
            .as_ref()
            .map(|value| self.mutate_override_paths_declaration(value));

        Rc::new(output::OverrideSpecifierStruct {
            node_id: source.node_id,
            overridden,
        })
    }

    fn mutate_override_paths_declaration(
        &mut self,
        source: &input::OverridePathsDeclaration,
    ) -> output::OverridePathsDeclaration {
        let paths = self.mutate_override_paths(&source.paths);

        Rc::new(output::OverridePathsDeclarationStruct {
            node_id: source.node_id,
            paths,
        })
    }

    fn mutate_returns_declaration(
        &mut self,
        source: &input::ReturnsDeclaration,
    ) -> output::ReturnsDeclaration {
        let variables = self.mutate_parameters_declaration(&source.variables);

        Rc::new(output::ReturnsDeclarationStruct {
            node_id: source.node_id,
            variables,
        })
    }

    fn mutate_constructor_definition(
        &mut self,
        source: &input::ConstructorDefinition,
    ) -> output::ConstructorDefinition {
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_constructor_attributes(&source.attributes);
        let body = self.mutate_block(&source.body);

        Rc::new(output::ConstructorDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            body,
        })
    }

    fn mutate_unnamed_function_definition(
        &mut self,
        source: &input::UnnamedFunctionDefinition,
    ) -> output::UnnamedFunctionDefinition {
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_unnamed_function_attributes(&source.attributes);
        let body = self.mutate_function_body(&source.body);

        Rc::new(output::UnnamedFunctionDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            body,
        })
    }

    fn mutate_fallback_function_definition(
        &mut self,
        source: &input::FallbackFunctionDefinition,
    ) -> output::FallbackFunctionDefinition {
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_fallback_function_attributes(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_returns_declaration(value));
        let body = self.mutate_function_body(&source.body);

        Rc::new(output::FallbackFunctionDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            returns,
            body,
        })
    }

    fn mutate_receive_function_definition(
        &mut self,
        source: &input::ReceiveFunctionDefinition,
    ) -> output::ReceiveFunctionDefinition {
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_receive_function_attributes(&source.attributes);
        let body = self.mutate_function_body(&source.body);

        Rc::new(output::ReceiveFunctionDefinitionStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            body,
        })
    }

    fn mutate_modifier_definition(
        &mut self,
        source: &input::ModifierDefinition,
    ) -> output::ModifierDefinition {
        let name = Rc::clone(&source.name);
        let parameters = source
            .parameters
            .as_ref()
            .map(|value| self.mutate_parameters_declaration(value));
        let attributes = self.mutate_modifier_attributes(&source.attributes);
        let body = self.mutate_function_body(&source.body);

        Rc::new(output::ModifierDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            attributes,
            body,
        })
    }

    fn mutate_modifier_invocation(
        &mut self,
        source: &input::ModifierInvocation,
    ) -> output::ModifierInvocation {
        let name = self.mutate_identifier_path(&source.name);
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.mutate_arguments_declaration(value));

        Rc::new(output::ModifierInvocationStruct {
            node_id: source.node_id,
            name,
            arguments,
        })
    }

    fn mutate_event_definition(
        &mut self,
        source: &input::EventDefinition,
    ) -> output::EventDefinition {
        let name = Rc::clone(&source.name);
        let parameters = self.mutate_event_parameters_declaration(&source.parameters);
        let anonymous_keyword = source.anonymous_keyword.as_ref().map(Rc::clone);

        Rc::new(output::EventDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            anonymous_keyword,
        })
    }

    fn mutate_event_parameters_declaration(
        &mut self,
        source: &input::EventParametersDeclaration,
    ) -> output::EventParametersDeclaration {
        let parameters = self.mutate_event_parameters(&source.parameters);

        Rc::new(output::EventParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn mutate_event_parameter(&mut self, source: &input::EventParameter) -> output::EventParameter {
        let type_name = self.mutate_type_name(&source.type_name);
        let indexed_keyword = source.indexed_keyword.as_ref().map(Rc::clone);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::EventParameterStruct {
            node_id: source.node_id,
            type_name,
            indexed_keyword,
            name,
        })
    }

    fn mutate_user_defined_value_type_definition(
        &mut self,
        source: &input::UserDefinedValueTypeDefinition,
    ) -> output::UserDefinedValueTypeDefinition {
        let name = Rc::clone(&source.name);
        let value_type = self.mutate_elementary_type(&source.value_type);

        Rc::new(output::UserDefinedValueTypeDefinitionStruct {
            node_id: source.node_id,
            name,
            value_type,
        })
    }

    fn mutate_error_definition(
        &mut self,
        source: &input::ErrorDefinition,
    ) -> output::ErrorDefinition {
        let name = Rc::clone(&source.name);
        let members = self.mutate_error_parameters_declaration(&source.members);

        Rc::new(output::ErrorDefinitionStruct {
            node_id: source.node_id,
            name,
            members,
        })
    }

    fn mutate_error_parameters_declaration(
        &mut self,
        source: &input::ErrorParametersDeclaration,
    ) -> output::ErrorParametersDeclaration {
        let parameters = self.mutate_error_parameters(&source.parameters);

        Rc::new(output::ErrorParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn mutate_error_parameter(&mut self, source: &input::ErrorParameter) -> output::ErrorParameter {
        let type_name = self.mutate_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::ErrorParameterStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn mutate_array_type_name(&mut self, source: &input::ArrayTypeName) -> output::ArrayTypeName {
        let operand = self.mutate_type_name(&source.operand);
        let index = source
            .index
            .as_ref()
            .map(|value| self.mutate_expression(value));

        Rc::new(output::ArrayTypeNameStruct {
            node_id: source.node_id,
            operand,
            index,
        })
    }

    fn mutate_function_type(&mut self, source: &input::FunctionType) -> output::FunctionType {
        let parameters = self.mutate_parameters_declaration(&source.parameters);
        let attributes = self.mutate_function_type_attributes(&source.attributes);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_returns_declaration(value));

        Rc::new(output::FunctionTypeStruct {
            node_id: source.node_id,
            parameters,
            attributes,
            returns,
        })
    }

    fn mutate_mapping_type(&mut self, source: &input::MappingType) -> output::MappingType {
        let key_type = self.mutate_mapping_key(&source.key_type);
        let value_type = self.mutate_mapping_value(&source.value_type);

        Rc::new(output::MappingTypeStruct {
            node_id: source.node_id,
            key_type,
            value_type,
        })
    }

    fn mutate_mapping_key(&mut self, source: &input::MappingKey) -> output::MappingKey {
        let key_type = self.mutate_mapping_key_type(&source.key_type);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::MappingKeyStruct {
            node_id: source.node_id,
            key_type,
            name,
        })
    }

    fn mutate_mapping_value(&mut self, source: &input::MappingValue) -> output::MappingValue {
        let type_name = self.mutate_type_name(&source.type_name);
        let name = source.name.as_ref().map(Rc::clone);

        Rc::new(output::MappingValueStruct {
            node_id: source.node_id,
            type_name,
            name,
        })
    }

    fn mutate_address_type(&mut self, source: &input::AddressType) -> output::AddressType {
        let payable_keyword = source.payable_keyword.as_ref().map(Rc::clone);

        Rc::new(output::AddressTypeStruct {
            node_id: source.node_id,
            payable_keyword,
        })
    }

    fn mutate_block(&mut self, source: &input::Block) -> output::Block {
        let statements = self.mutate_statements(&source.statements);

        Rc::new(output::BlockStruct {
            node_id: source.node_id,
            statements,
        })
    }

    fn mutate_unchecked_block(&mut self, source: &input::UncheckedBlock) -> output::UncheckedBlock {
        let block = self.mutate_block(&source.block);

        Rc::new(output::UncheckedBlockStruct {
            node_id: source.node_id,
            block,
        })
    }

    fn mutate_expression_statement(
        &mut self,
        source: &input::ExpressionStatement,
    ) -> output::ExpressionStatement {
        let expression = self.mutate_expression(&source.expression);

        Rc::new(output::ExpressionStatementStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn mutate_assembly_statement(
        &mut self,
        source: &input::AssemblyStatement,
    ) -> output::AssemblyStatement {
        let label = source
            .label
            .as_ref()
            .map(|value| self.mutate_string_literal(value));
        let flags = source
            .flags
            .as_ref()
            .map(|value| self.mutate_assembly_flags_declaration(value));
        let body = self.mutate_yul_block(&source.body);

        Rc::new(output::AssemblyStatementStruct {
            node_id: source.node_id,
            label,
            flags,
            body,
        })
    }

    fn mutate_assembly_flags_declaration(
        &mut self,
        source: &input::AssemblyFlagsDeclaration,
    ) -> output::AssemblyFlagsDeclaration {
        let flags = self.mutate_assembly_flags(&source.flags);

        Rc::new(output::AssemblyFlagsDeclarationStruct {
            node_id: source.node_id,
            flags,
        })
    }

    fn mutate_tuple_deconstruction_statement(
        &mut self,
        source: &input::TupleDeconstructionStatement,
    ) -> output::TupleDeconstructionStatement {
        let var_keyword = source.var_keyword.as_ref().map(Rc::clone);
        let elements = self.mutate_tuple_deconstruction_elements(&source.elements);
        let expression = self.mutate_expression(&source.expression);

        Rc::new(output::TupleDeconstructionStatementStruct {
            node_id: source.node_id,
            var_keyword,
            elements,
            expression,
        })
    }

    fn mutate_tuple_deconstruction_element(
        &mut self,
        source: &input::TupleDeconstructionElement,
    ) -> output::TupleDeconstructionElement {
        let member = source
            .member
            .as_ref()
            .map(|value| self.mutate_tuple_member(value));

        Rc::new(output::TupleDeconstructionElementStruct {
            node_id: source.node_id,
            member,
        })
    }

    fn mutate_typed_tuple_member(
        &mut self,
        source: &input::TypedTupleMember,
    ) -> output::TypedTupleMember {
        let type_name = self.mutate_type_name(&source.type_name);
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.mutate_storage_location(value));
        let name = Rc::clone(&source.name);

        Rc::new(output::TypedTupleMemberStruct {
            node_id: source.node_id,
            type_name,
            storage_location,
            name,
        })
    }

    fn mutate_untyped_tuple_member(
        &mut self,
        source: &input::UntypedTupleMember,
    ) -> output::UntypedTupleMember {
        let storage_location = source
            .storage_location
            .as_ref()
            .map(|value| self.mutate_storage_location(value));
        let name = Rc::clone(&source.name);

        Rc::new(output::UntypedTupleMemberStruct {
            node_id: source.node_id,
            storage_location,
            name,
        })
    }

    fn mutate_variable_declaration_statement(
        &mut self,
        source: &input::VariableDeclarationStatement,
    ) -> output::VariableDeclarationStatement {
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

        Rc::new(output::VariableDeclarationStatementStruct {
            node_id: source.node_id,
            variable_type,
            storage_location,
            name,
            value,
        })
    }

    fn mutate_variable_declaration_value(
        &mut self,
        source: &input::VariableDeclarationValue,
    ) -> output::VariableDeclarationValue {
        let expression = self.mutate_expression(&source.expression);

        Rc::new(output::VariableDeclarationValueStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn mutate_if_statement(&mut self, source: &input::IfStatement) -> output::IfStatement {
        let condition = self.mutate_expression(&source.condition);
        let body = self.mutate_statement(&source.body);
        let else_branch = source
            .else_branch
            .as_ref()
            .map(|value| self.mutate_else_branch(value));

        Rc::new(output::IfStatementStruct {
            node_id: source.node_id,
            condition,
            body,
            else_branch,
        })
    }

    fn mutate_else_branch(&mut self, source: &input::ElseBranch) -> output::ElseBranch {
        let body = self.mutate_statement(&source.body);

        Rc::new(output::ElseBranchStruct {
            node_id: source.node_id,
            body,
        })
    }

    fn mutate_for_statement(&mut self, source: &input::ForStatement) -> output::ForStatement {
        let initialization = self.mutate_for_statement_initialization(&source.initialization);
        let condition = self.mutate_for_statement_condition(&source.condition);
        let iterator = source
            .iterator
            .as_ref()
            .map(|value| self.mutate_expression(value));
        let body = self.mutate_statement(&source.body);

        Rc::new(output::ForStatementStruct {
            node_id: source.node_id,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    fn mutate_while_statement(&mut self, source: &input::WhileStatement) -> output::WhileStatement {
        let condition = self.mutate_expression(&source.condition);
        let body = self.mutate_statement(&source.body);

        Rc::new(output::WhileStatementStruct {
            node_id: source.node_id,
            condition,
            body,
        })
    }

    fn mutate_do_while_statement(
        &mut self,
        source: &input::DoWhileStatement,
    ) -> output::DoWhileStatement {
        let body = self.mutate_statement(&source.body);
        let condition = self.mutate_expression(&source.condition);

        Rc::new(output::DoWhileStatementStruct {
            node_id: source.node_id,
            body,
            condition,
        })
    }

    fn mutate_continue_statement(
        &mut self,
        source: &input::ContinueStatement,
    ) -> output::ContinueStatement {
        Rc::new(output::ContinueStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_break_statement(&mut self, source: &input::BreakStatement) -> output::BreakStatement {
        Rc::new(output::BreakStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_return_statement(
        &mut self,
        source: &input::ReturnStatement,
    ) -> output::ReturnStatement {
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.mutate_expression(value));

        Rc::new(output::ReturnStatementStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn mutate_emit_statement(&mut self, source: &input::EmitStatement) -> output::EmitStatement {
        let event = self.mutate_identifier_path(&source.event);
        let arguments = self.mutate_arguments_declaration(&source.arguments);

        Rc::new(output::EmitStatementStruct {
            node_id: source.node_id,
            event,
            arguments,
        })
    }

    fn mutate_try_statement(&mut self, source: &input::TryStatement) -> output::TryStatement {
        let expression = self.mutate_expression(&source.expression);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_returns_declaration(value));
        let body = self.mutate_block(&source.body);
        let catch_clauses = self.mutate_catch_clauses(&source.catch_clauses);

        Rc::new(output::TryStatementStruct {
            node_id: source.node_id,
            expression,
            returns,
            body,
            catch_clauses,
        })
    }

    fn mutate_catch_clause(&mut self, source: &input::CatchClause) -> output::CatchClause {
        let error = source
            .error
            .as_ref()
            .map(|value| self.mutate_catch_clause_error(value));
        let body = self.mutate_block(&source.body);

        Rc::new(output::CatchClauseStruct {
            node_id: source.node_id,
            error,
            body,
        })
    }

    fn mutate_catch_clause_error(
        &mut self,
        source: &input::CatchClauseError,
    ) -> output::CatchClauseError {
        let name = source.name.as_ref().map(Rc::clone);
        let parameters = self.mutate_parameters_declaration(&source.parameters);

        Rc::new(output::CatchClauseErrorStruct {
            node_id: source.node_id,
            name,
            parameters,
        })
    }

    fn mutate_revert_statement(
        &mut self,
        source: &input::RevertStatement,
    ) -> output::RevertStatement {
        let error = source
            .error
            .as_ref()
            .map(|value| self.mutate_identifier_path(value));
        let arguments = self.mutate_arguments_declaration(&source.arguments);

        Rc::new(output::RevertStatementStruct {
            node_id: source.node_id,
            error,
            arguments,
        })
    }

    fn mutate_throw_statement(&mut self, source: &input::ThrowStatement) -> output::ThrowStatement {
        Rc::new(output::ThrowStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_assignment_expression(
        &mut self,
        source: &input::AssignmentExpression,
    ) -> output::AssignmentExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::AssignmentExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_conditional_expression(
        &mut self,
        source: &input::ConditionalExpression,
    ) -> output::ConditionalExpression {
        let operand = self.mutate_expression(&source.operand);
        let true_expression = self.mutate_expression(&source.true_expression);
        let false_expression = self.mutate_expression(&source.false_expression);

        Rc::new(output::ConditionalExpressionStruct {
            node_id: source.node_id,
            operand,
            true_expression,
            false_expression,
        })
    }

    fn mutate_or_expression(&mut self, source: &input::OrExpression) -> output::OrExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::OrExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_and_expression(&mut self, source: &input::AndExpression) -> output::AndExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::AndExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_equality_expression(
        &mut self,
        source: &input::EqualityExpression,
    ) -> output::EqualityExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::EqualityExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_inequality_expression(
        &mut self,
        source: &input::InequalityExpression,
    ) -> output::InequalityExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::InequalityExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_bitwise_or_expression(
        &mut self,
        source: &input::BitwiseOrExpression,
    ) -> output::BitwiseOrExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::BitwiseOrExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_bitwise_xor_expression(
        &mut self,
        source: &input::BitwiseXorExpression,
    ) -> output::BitwiseXorExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::BitwiseXorExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_bitwise_and_expression(
        &mut self,
        source: &input::BitwiseAndExpression,
    ) -> output::BitwiseAndExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::BitwiseAndExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_shift_expression(
        &mut self,
        source: &input::ShiftExpression,
    ) -> output::ShiftExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::ShiftExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_additive_expression(
        &mut self,
        source: &input::AdditiveExpression,
    ) -> output::AdditiveExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::AdditiveExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_multiplicative_expression(
        &mut self,
        source: &input::MultiplicativeExpression,
    ) -> output::MultiplicativeExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::MultiplicativeExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_exponentiation_expression(
        &mut self,
        source: &input::ExponentiationExpression,
    ) -> output::ExponentiationExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(output::ExponentiationExpressionStruct {
            node_id: source.node_id,
            left_operand,
            right_operand,
        })
    }

    fn mutate_postfix_expression(
        &mut self,
        source: &input::PostfixExpression,
    ) -> output::PostfixExpression {
        let operand = self.mutate_expression(&source.operand);

        Rc::new(output::PostfixExpressionStruct {
            node_id: source.node_id,
            operand,
        })
    }

    fn mutate_prefix_expression(
        &mut self,
        source: &input::PrefixExpression,
    ) -> output::PrefixExpression {
        let operand = self.mutate_expression(&source.operand);

        Rc::new(output::PrefixExpressionStruct {
            node_id: source.node_id,
            operand,
        })
    }

    fn mutate_function_call_expression(
        &mut self,
        source: &input::FunctionCallExpression,
    ) -> output::FunctionCallExpression {
        let operand = self.mutate_expression(&source.operand);
        let arguments = self.mutate_arguments_declaration(&source.arguments);

        Rc::new(output::FunctionCallExpressionStruct {
            node_id: source.node_id,
            operand,
            arguments,
        })
    }

    fn mutate_call_options_expression(
        &mut self,
        source: &input::CallOptionsExpression,
    ) -> output::CallOptionsExpression {
        let operand = self.mutate_expression(&source.operand);
        let options = self.mutate_call_options(&source.options);

        Rc::new(output::CallOptionsExpressionStruct {
            node_id: source.node_id,
            operand,
            options,
        })
    }

    fn mutate_member_access_expression(
        &mut self,
        source: &input::MemberAccessExpression,
    ) -> output::MemberAccessExpression {
        let operand = self.mutate_expression(&source.operand);
        let member = Rc::clone(&source.member);

        Rc::new(output::MemberAccessExpressionStruct {
            node_id: source.node_id,
            operand,
            member,
        })
    }

    fn mutate_index_access_expression(
        &mut self,
        source: &input::IndexAccessExpression,
    ) -> output::IndexAccessExpression {
        let operand = self.mutate_expression(&source.operand);
        let start = source
            .start
            .as_ref()
            .map(|value| self.mutate_expression(value));
        let end = source
            .end
            .as_ref()
            .map(|value| self.mutate_index_access_end(value));

        Rc::new(output::IndexAccessExpressionStruct {
            node_id: source.node_id,
            operand,
            start,
            end,
        })
    }

    fn mutate_index_access_end(
        &mut self,
        source: &input::IndexAccessEnd,
    ) -> output::IndexAccessEnd {
        let end = source
            .end
            .as_ref()
            .map(|value| self.mutate_expression(value));

        Rc::new(output::IndexAccessEndStruct {
            node_id: source.node_id,
            end,
        })
    }

    fn mutate_positional_arguments_declaration(
        &mut self,
        source: &input::PositionalArgumentsDeclaration,
    ) -> output::PositionalArgumentsDeclaration {
        let arguments = self.mutate_positional_arguments(&source.arguments);

        Rc::new(output::PositionalArgumentsDeclarationStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn mutate_named_arguments_declaration(
        &mut self,
        source: &input::NamedArgumentsDeclaration,
    ) -> output::NamedArgumentsDeclaration {
        let arguments = source
            .arguments
            .as_ref()
            .map(|value| self.mutate_named_argument_group(value));

        Rc::new(output::NamedArgumentsDeclarationStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn mutate_named_argument_group(
        &mut self,
        source: &input::NamedArgumentGroup,
    ) -> output::NamedArgumentGroup {
        let arguments = self.mutate_named_arguments(&source.arguments);

        Rc::new(output::NamedArgumentGroupStruct {
            node_id: source.node_id,
            arguments,
        })
    }

    fn mutate_named_argument(&mut self, source: &input::NamedArgument) -> output::NamedArgument {
        let name = Rc::clone(&source.name);
        let value = self.mutate_expression(&source.value);

        Rc::new(output::NamedArgumentStruct {
            node_id: source.node_id,
            name,
            value,
        })
    }

    fn mutate_type_expression(&mut self, source: &input::TypeExpression) -> output::TypeExpression {
        let type_name = self.mutate_type_name(&source.type_name);

        Rc::new(output::TypeExpressionStruct {
            node_id: source.node_id,
            type_name,
        })
    }

    fn mutate_new_expression(&mut self, source: &input::NewExpression) -> output::NewExpression {
        let type_name = self.mutate_type_name(&source.type_name);

        Rc::new(output::NewExpressionStruct {
            node_id: source.node_id,
            type_name,
        })
    }

    fn mutate_tuple_expression(
        &mut self,
        source: &input::TupleExpression,
    ) -> output::TupleExpression {
        let items = self.mutate_tuple_values(&source.items);

        Rc::new(output::TupleExpressionStruct {
            node_id: source.node_id,
            items,
        })
    }

    fn mutate_tuple_value(&mut self, source: &input::TupleValue) -> output::TupleValue {
        let expression = source
            .expression
            .as_ref()
            .map(|value| self.mutate_expression(value));

        Rc::new(output::TupleValueStruct {
            node_id: source.node_id,
            expression,
        })
    }

    fn mutate_array_expression(
        &mut self,
        source: &input::ArrayExpression,
    ) -> output::ArrayExpression {
        let items = self.mutate_array_values(&source.items);

        Rc::new(output::ArrayExpressionStruct {
            node_id: source.node_id,
            items,
        })
    }

    fn mutate_hex_number_expression(
        &mut self,
        source: &input::HexNumberExpression,
    ) -> output::HexNumberExpression {
        let literal = Rc::clone(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.mutate_number_unit(value));

        Rc::new(output::HexNumberExpressionStruct {
            node_id: source.node_id,
            literal,
            unit,
        })
    }

    fn mutate_decimal_number_expression(
        &mut self,
        source: &input::DecimalNumberExpression,
    ) -> output::DecimalNumberExpression {
        let literal = Rc::clone(&source.literal);
        let unit = source
            .unit
            .as_ref()
            .map(|value| self.mutate_number_unit(value));

        Rc::new(output::DecimalNumberExpressionStruct {
            node_id: source.node_id,
            literal,
            unit,
        })
    }

    fn mutate_yul_block(&mut self, source: &input::YulBlock) -> output::YulBlock {
        let statements = self.mutate_yul_statements(&source.statements);

        Rc::new(output::YulBlockStruct {
            node_id: source.node_id,
            statements,
        })
    }

    fn mutate_yul_function_definition(
        &mut self,
        source: &input::YulFunctionDefinition,
    ) -> output::YulFunctionDefinition {
        let name = Rc::clone(&source.name);
        let parameters = self.mutate_yul_parameters_declaration(&source.parameters);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.mutate_yul_returns_declaration(value));
        let body = self.mutate_yul_block(&source.body);

        Rc::new(output::YulFunctionDefinitionStruct {
            node_id: source.node_id,
            name,
            parameters,
            returns,
            body,
        })
    }

    fn mutate_yul_parameters_declaration(
        &mut self,
        source: &input::YulParametersDeclaration,
    ) -> output::YulParametersDeclaration {
        let parameters = self.mutate_yul_parameters(&source.parameters);

        Rc::new(output::YulParametersDeclarationStruct {
            node_id: source.node_id,
            parameters,
        })
    }

    fn mutate_yul_returns_declaration(
        &mut self,
        source: &input::YulReturnsDeclaration,
    ) -> output::YulReturnsDeclaration {
        let variables = self.mutate_yul_variable_names(&source.variables);

        Rc::new(output::YulReturnsDeclarationStruct {
            node_id: source.node_id,
            variables,
        })
    }

    fn mutate_yul_variable_declaration_statement(
        &mut self,
        source: &input::YulVariableDeclarationStatement,
    ) -> output::YulVariableDeclarationStatement {
        let variables = self.mutate_yul_variable_names(&source.variables);
        let value = source
            .value
            .as_ref()
            .map(|value| self.mutate_yul_variable_declaration_value(value));

        Rc::new(output::YulVariableDeclarationStatementStruct {
            node_id: source.node_id,
            variables,
            value,
        })
    }

    fn mutate_yul_variable_declaration_value(
        &mut self,
        source: &input::YulVariableDeclarationValue,
    ) -> output::YulVariableDeclarationValue {
        let assignment = self.mutate_yul_assignment_operator(&source.assignment);
        let expression = self.mutate_yul_expression(&source.expression);

        Rc::new(output::YulVariableDeclarationValueStruct {
            node_id: source.node_id,
            assignment,
            expression,
        })
    }

    fn mutate_yul_variable_assignment_statement(
        &mut self,
        source: &input::YulVariableAssignmentStatement,
    ) -> output::YulVariableAssignmentStatement {
        let variables = self.mutate_yul_paths(&source.variables);
        let assignment = self.mutate_yul_assignment_operator(&source.assignment);
        let expression = self.mutate_yul_expression(&source.expression);

        Rc::new(output::YulVariableAssignmentStatementStruct {
            node_id: source.node_id,
            variables,
            assignment,
            expression,
        })
    }

    fn mutate_yul_colon_and_equal(
        &mut self,
        source: &input::YulColonAndEqual,
    ) -> output::YulColonAndEqual {
        Rc::new(output::YulColonAndEqualStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_stack_assignment_statement(
        &mut self,
        source: &input::YulStackAssignmentStatement,
    ) -> output::YulStackAssignmentStatement {
        let assignment = self.mutate_yul_stack_assignment_operator(&source.assignment);
        let variable = Rc::clone(&source.variable);

        Rc::new(output::YulStackAssignmentStatementStruct {
            node_id: source.node_id,
            assignment,
            variable,
        })
    }

    fn mutate_yul_equal_and_colon(
        &mut self,
        source: &input::YulEqualAndColon,
    ) -> output::YulEqualAndColon {
        Rc::new(output::YulEqualAndColonStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_if_statement(
        &mut self,
        source: &input::YulIfStatement,
    ) -> output::YulIfStatement {
        let condition = self.mutate_yul_expression(&source.condition);
        let body = self.mutate_yul_block(&source.body);

        Rc::new(output::YulIfStatementStruct {
            node_id: source.node_id,
            condition,
            body,
        })
    }

    fn mutate_yul_for_statement(
        &mut self,
        source: &input::YulForStatement,
    ) -> output::YulForStatement {
        let initialization = self.mutate_yul_block(&source.initialization);
        let condition = self.mutate_yul_expression(&source.condition);
        let iterator = self.mutate_yul_block(&source.iterator);
        let body = self.mutate_yul_block(&source.body);

        Rc::new(output::YulForStatementStruct {
            node_id: source.node_id,
            initialization,
            condition,
            iterator,
            body,
        })
    }

    fn mutate_yul_switch_statement(
        &mut self,
        source: &input::YulSwitchStatement,
    ) -> output::YulSwitchStatement {
        let expression = self.mutate_yul_expression(&source.expression);
        let cases = self.mutate_yul_switch_cases(&source.cases);

        Rc::new(output::YulSwitchStatementStruct {
            node_id: source.node_id,
            expression,
            cases,
        })
    }

    fn mutate_yul_default_case(
        &mut self,
        source: &input::YulDefaultCase,
    ) -> output::YulDefaultCase {
        let body = self.mutate_yul_block(&source.body);

        Rc::new(output::YulDefaultCaseStruct {
            node_id: source.node_id,
            body,
        })
    }

    fn mutate_yul_value_case(&mut self, source: &input::YulValueCase) -> output::YulValueCase {
        let value = self.mutate_yul_literal(&source.value);
        let body = self.mutate_yul_block(&source.body);

        Rc::new(output::YulValueCaseStruct {
            node_id: source.node_id,
            value,
            body,
        })
    }

    fn mutate_yul_leave_statement(
        &mut self,
        source: &input::YulLeaveStatement,
    ) -> output::YulLeaveStatement {
        Rc::new(output::YulLeaveStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_break_statement(
        &mut self,
        source: &input::YulBreakStatement,
    ) -> output::YulBreakStatement {
        Rc::new(output::YulBreakStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_continue_statement(
        &mut self,
        source: &input::YulContinueStatement,
    ) -> output::YulContinueStatement {
        Rc::new(output::YulContinueStatementStruct {
            node_id: source.node_id,
        })
    }

    fn mutate_yul_label(&mut self, source: &input::YulLabel) -> output::YulLabel {
        let label = Rc::clone(&source.label);

        Rc::new(output::YulLabelStruct {
            node_id: source.node_id,
            label,
        })
    }

    fn mutate_yul_function_call_expression(
        &mut self,
        source: &input::YulFunctionCallExpression,
    ) -> output::YulFunctionCallExpression {
        let operand = self.mutate_yul_expression(&source.operand);
        let arguments = self.mutate_yul_arguments(&source.arguments);

        Rc::new(output::YulFunctionCallExpressionStruct {
            node_id: source.node_id,
            operand,
            arguments,
        })
    }

    //
    // Choices:
    //

    fn default_mutate_source_unit_member(
        &mut self,
        source: &input::SourceUnitMember,
    ) -> output::SourceUnitMember {
        match source {
            input::SourceUnitMember::PragmaDirective(ref pragma_directive) => {
                output::SourceUnitMember::PragmaDirective(
                    self.mutate_pragma_directive(pragma_directive),
                )
            }
            input::SourceUnitMember::ImportDirective(ref import_directive) => {
                output::SourceUnitMember::ImportDirective(
                    self.mutate_import_directive(import_directive),
                )
            }
            input::SourceUnitMember::ContractDefinition(ref contract_definition) => {
                output::SourceUnitMember::ContractDefinition(
                    self.mutate_contract_definition(contract_definition),
                )
            }
            input::SourceUnitMember::InterfaceDefinition(ref interface_definition) => {
                output::SourceUnitMember::InterfaceDefinition(
                    self.mutate_interface_definition(interface_definition),
                )
            }
            input::SourceUnitMember::LibraryDefinition(ref library_definition) => {
                output::SourceUnitMember::LibraryDefinition(
                    self.mutate_library_definition(library_definition),
                )
            }
            input::SourceUnitMember::StructDefinition(ref struct_definition) => {
                output::SourceUnitMember::StructDefinition(
                    self.mutate_struct_definition(struct_definition),
                )
            }
            input::SourceUnitMember::EnumDefinition(ref enum_definition) => {
                output::SourceUnitMember::EnumDefinition(
                    self.mutate_enum_definition(enum_definition),
                )
            }
            input::SourceUnitMember::FunctionDefinition(ref function_definition) => {
                output::SourceUnitMember::FunctionDefinition(
                    self.mutate_function_definition(function_definition),
                )
            }
            input::SourceUnitMember::ErrorDefinition(ref error_definition) => {
                output::SourceUnitMember::ErrorDefinition(
                    self.mutate_error_definition(error_definition),
                )
            }
            input::SourceUnitMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => output::SourceUnitMember::UserDefinedValueTypeDefinition(
                self.mutate_user_defined_value_type_definition(user_defined_value_type_definition),
            ),
            input::SourceUnitMember::UsingDirective(ref using_directive) => {
                output::SourceUnitMember::UsingDirective(
                    self.mutate_using_directive(using_directive),
                )
            }
            input::SourceUnitMember::EventDefinition(ref event_definition) => {
                output::SourceUnitMember::EventDefinition(
                    self.mutate_event_definition(event_definition),
                )
            }
            input::SourceUnitMember::ConstantDefinition(ref constant_definition) => {
                output::SourceUnitMember::ConstantDefinition(
                    self.mutate_constant_definition(constant_definition),
                )
            }
        }
    }
    fn mutate_source_unit_member(
        &mut self,
        source: &input::SourceUnitMember,
    ) -> output::SourceUnitMember {
        self.default_mutate_source_unit_member(source)
    }

    fn default_mutate_pragma(&mut self, source: &input::Pragma) -> output::Pragma {
        match source {
            input::Pragma::AbicoderPragma(ref abicoder_pragma) => {
                output::Pragma::AbicoderPragma(self.mutate_abicoder_pragma(abicoder_pragma))
            }
            input::Pragma::ExperimentalPragma(ref experimental_pragma) => {
                output::Pragma::ExperimentalPragma(
                    self.mutate_experimental_pragma(experimental_pragma),
                )
            }
            input::Pragma::VersionPragma(ref version_pragma) => {
                output::Pragma::VersionPragma(self.mutate_version_pragma(version_pragma))
            }
        }
    }
    fn mutate_pragma(&mut self, source: &input::Pragma) -> output::Pragma {
        self.default_mutate_pragma(source)
    }

    fn default_mutate_experimental_feature(
        &mut self,
        source: &input::ExperimentalFeature,
    ) -> output::ExperimentalFeature {
        match source {
            input::ExperimentalFeature::StringLiteral(ref string_literal) => {
                output::ExperimentalFeature::StringLiteral(
                    self.mutate_string_literal(string_literal),
                )
            }
            input::ExperimentalFeature::Identifier(node) => {
                output::ExperimentalFeature::Identifier(Rc::clone(node))
            }
        }
    }
    fn mutate_experimental_feature(
        &mut self,
        source: &input::ExperimentalFeature,
    ) -> output::ExperimentalFeature {
        self.default_mutate_experimental_feature(source)
    }

    fn default_mutate_version_expression(
        &mut self,
        source: &input::VersionExpression,
    ) -> output::VersionExpression {
        match source {
            input::VersionExpression::VersionRange(ref version_range) => {
                output::VersionExpression::VersionRange(self.mutate_version_range(version_range))
            }
            input::VersionExpression::VersionTerm(ref version_term) => {
                output::VersionExpression::VersionTerm(self.mutate_version_term(version_term))
            }
        }
    }
    fn mutate_version_expression(
        &mut self,
        source: &input::VersionExpression,
    ) -> output::VersionExpression {
        self.default_mutate_version_expression(source)
    }

    fn default_mutate_version_operator(
        &mut self,
        source: &input::VersionOperator,
    ) -> output::VersionOperator {
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
    fn mutate_version_operator(
        &mut self,
        source: &input::VersionOperator,
    ) -> output::VersionOperator {
        self.default_mutate_version_operator(source)
    }

    fn default_mutate_version_literal(
        &mut self,
        source: &input::VersionLiteral,
    ) -> output::VersionLiteral {
        match source {
            input::VersionLiteral::SimpleVersionLiteral(ref simple_version_literal) => {
                output::VersionLiteral::SimpleVersionLiteral(
                    self.mutate_simple_version_literal(simple_version_literal),
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
    fn mutate_version_literal(&mut self, source: &input::VersionLiteral) -> output::VersionLiteral {
        self.default_mutate_version_literal(source)
    }

    fn default_mutate_import_clause(
        &mut self,
        source: &input::ImportClause,
    ) -> output::ImportClause {
        match source {
            input::ImportClause::PathImport(ref path_import) => {
                output::ImportClause::PathImport(self.mutate_path_import(path_import))
            }
            input::ImportClause::NamedImport(ref named_import) => {
                output::ImportClause::NamedImport(self.mutate_named_import(named_import))
            }
            input::ImportClause::ImportDeconstruction(ref import_deconstruction) => {
                output::ImportClause::ImportDeconstruction(
                    self.mutate_import_deconstruction(import_deconstruction),
                )
            }
        }
    }
    fn mutate_import_clause(&mut self, source: &input::ImportClause) -> output::ImportClause {
        self.default_mutate_import_clause(source)
    }

    fn default_mutate_using_clause(&mut self, source: &input::UsingClause) -> output::UsingClause {
        match source {
            input::UsingClause::IdentifierPath(ref identifier_path) => {
                output::UsingClause::IdentifierPath(self.mutate_identifier_path(identifier_path))
            }
            input::UsingClause::UsingDeconstruction(ref using_deconstruction) => {
                output::UsingClause::UsingDeconstruction(
                    self.mutate_using_deconstruction(using_deconstruction),
                )
            }
        }
    }
    fn mutate_using_clause(&mut self, source: &input::UsingClause) -> output::UsingClause {
        self.default_mutate_using_clause(source)
    }

    fn default_mutate_using_operator(
        &mut self,
        source: &input::UsingOperator,
    ) -> output::UsingOperator {
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
    fn mutate_using_operator(&mut self, source: &input::UsingOperator) -> output::UsingOperator {
        self.default_mutate_using_operator(source)
    }

    fn default_mutate_using_target(&mut self, source: &input::UsingTarget) -> output::UsingTarget {
        match source {
            input::UsingTarget::TypeName(ref type_name) => {
                output::UsingTarget::TypeName(self.mutate_type_name(type_name))
            }
            input::UsingTarget::Asterisk => output::UsingTarget::Asterisk,
        }
    }
    fn mutate_using_target(&mut self, source: &input::UsingTarget) -> output::UsingTarget {
        self.default_mutate_using_target(source)
    }

    fn default_mutate_contract_member(
        &mut self,
        source: &input::ContractMember,
    ) -> output::ContractMember {
        match source {
            input::ContractMember::UsingDirective(ref using_directive) => {
                output::ContractMember::UsingDirective(self.mutate_using_directive(using_directive))
            }
            input::ContractMember::FunctionDefinition(ref function_definition) => {
                output::ContractMember::FunctionDefinition(
                    self.mutate_function_definition(function_definition),
                )
            }
            input::ContractMember::ConstructorDefinition(ref constructor_definition) => {
                output::ContractMember::ConstructorDefinition(
                    self.mutate_constructor_definition(constructor_definition),
                )
            }
            input::ContractMember::ReceiveFunctionDefinition(ref receive_function_definition) => {
                output::ContractMember::ReceiveFunctionDefinition(
                    self.mutate_receive_function_definition(receive_function_definition),
                )
            }
            input::ContractMember::FallbackFunctionDefinition(ref fallback_function_definition) => {
                output::ContractMember::FallbackFunctionDefinition(
                    self.mutate_fallback_function_definition(fallback_function_definition),
                )
            }
            input::ContractMember::UnnamedFunctionDefinition(ref unnamed_function_definition) => {
                output::ContractMember::UnnamedFunctionDefinition(
                    self.mutate_unnamed_function_definition(unnamed_function_definition),
                )
            }
            input::ContractMember::ModifierDefinition(ref modifier_definition) => {
                output::ContractMember::ModifierDefinition(
                    self.mutate_modifier_definition(modifier_definition),
                )
            }
            input::ContractMember::StructDefinition(ref struct_definition) => {
                output::ContractMember::StructDefinition(
                    self.mutate_struct_definition(struct_definition),
                )
            }
            input::ContractMember::EnumDefinition(ref enum_definition) => {
                output::ContractMember::EnumDefinition(self.mutate_enum_definition(enum_definition))
            }
            input::ContractMember::EventDefinition(ref event_definition) => {
                output::ContractMember::EventDefinition(
                    self.mutate_event_definition(event_definition),
                )
            }
            input::ContractMember::ErrorDefinition(ref error_definition) => {
                output::ContractMember::ErrorDefinition(
                    self.mutate_error_definition(error_definition),
                )
            }
            input::ContractMember::UserDefinedValueTypeDefinition(
                ref user_defined_value_type_definition,
            ) => output::ContractMember::UserDefinedValueTypeDefinition(
                self.mutate_user_defined_value_type_definition(user_defined_value_type_definition),
            ),
            input::ContractMember::StateVariableDefinition(ref state_variable_definition) => {
                output::ContractMember::StateVariableDefinition(
                    self.mutate_state_variable_definition(state_variable_definition),
                )
            }
        }
    }
    fn mutate_contract_member(&mut self, source: &input::ContractMember) -> output::ContractMember {
        self.default_mutate_contract_member(source)
    }

    fn default_mutate_state_variable_attribute(
        &mut self,
        source: &input::StateVariableAttribute,
    ) -> output::StateVariableAttribute {
        match source {
            input::StateVariableAttribute::OverrideSpecifier(ref override_specifier) => {
                output::StateVariableAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
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
    fn mutate_state_variable_attribute(
        &mut self,
        source: &input::StateVariableAttribute,
    ) -> output::StateVariableAttribute {
        self.default_mutate_state_variable_attribute(source)
    }

    fn default_mutate_function_name(
        &mut self,
        source: &input::FunctionName,
    ) -> output::FunctionName {
        match source {
            input::FunctionName::Identifier(node) => {
                output::FunctionName::Identifier(Rc::clone(node))
            }
            input::FunctionName::FallbackKeyword => output::FunctionName::FallbackKeyword,
            input::FunctionName::ReceiveKeyword => output::FunctionName::ReceiveKeyword,
        }
    }
    fn mutate_function_name(&mut self, source: &input::FunctionName) -> output::FunctionName {
        self.default_mutate_function_name(source)
    }

    fn default_mutate_function_attribute(
        &mut self,
        source: &input::FunctionAttribute,
    ) -> output::FunctionAttribute {
        match source {
            input::FunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                output::FunctionAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            input::FunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                output::FunctionAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
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
    fn mutate_function_attribute(
        &mut self,
        source: &input::FunctionAttribute,
    ) -> output::FunctionAttribute {
        self.default_mutate_function_attribute(source)
    }

    fn default_mutate_function_body(
        &mut self,
        source: &input::FunctionBody,
    ) -> output::FunctionBody {
        match source {
            input::FunctionBody::Block(ref block) => {
                output::FunctionBody::Block(self.mutate_block(block))
            }
            input::FunctionBody::Semicolon => output::FunctionBody::Semicolon,
        }
    }
    fn mutate_function_body(&mut self, source: &input::FunctionBody) -> output::FunctionBody {
        self.default_mutate_function_body(source)
    }

    fn default_mutate_constructor_attribute(
        &mut self,
        source: &input::ConstructorAttribute,
    ) -> output::ConstructorAttribute {
        match source {
            input::ConstructorAttribute::ModifierInvocation(ref modifier_invocation) => {
                output::ConstructorAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            input::ConstructorAttribute::InternalKeyword => {
                output::ConstructorAttribute::InternalKeyword
            }
            input::ConstructorAttribute::OverrideKeyword => {
                output::ConstructorAttribute::OverrideKeyword
            }
            input::ConstructorAttribute::PayableKeyword => {
                output::ConstructorAttribute::PayableKeyword
            }
            input::ConstructorAttribute::PublicKeyword => {
                output::ConstructorAttribute::PublicKeyword
            }
            input::ConstructorAttribute::VirtualKeyword => {
                output::ConstructorAttribute::VirtualKeyword
            }
        }
    }
    fn mutate_constructor_attribute(
        &mut self,
        source: &input::ConstructorAttribute,
    ) -> output::ConstructorAttribute {
        self.default_mutate_constructor_attribute(source)
    }

    fn default_mutate_unnamed_function_attribute(
        &mut self,
        source: &input::UnnamedFunctionAttribute,
    ) -> output::UnnamedFunctionAttribute {
        match source {
            input::UnnamedFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                output::UnnamedFunctionAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            input::UnnamedFunctionAttribute::ConstantKeyword => {
                output::UnnamedFunctionAttribute::ConstantKeyword
            }
            input::UnnamedFunctionAttribute::ExternalKeyword => {
                output::UnnamedFunctionAttribute::ExternalKeyword
            }
            input::UnnamedFunctionAttribute::InternalKeyword => {
                output::UnnamedFunctionAttribute::InternalKeyword
            }
            input::UnnamedFunctionAttribute::PayableKeyword => {
                output::UnnamedFunctionAttribute::PayableKeyword
            }
            input::UnnamedFunctionAttribute::PrivateKeyword => {
                output::UnnamedFunctionAttribute::PrivateKeyword
            }
            input::UnnamedFunctionAttribute::PublicKeyword => {
                output::UnnamedFunctionAttribute::PublicKeyword
            }
            input::UnnamedFunctionAttribute::PureKeyword => {
                output::UnnamedFunctionAttribute::PureKeyword
            }
            input::UnnamedFunctionAttribute::ViewKeyword => {
                output::UnnamedFunctionAttribute::ViewKeyword
            }
        }
    }
    fn mutate_unnamed_function_attribute(
        &mut self,
        source: &input::UnnamedFunctionAttribute,
    ) -> output::UnnamedFunctionAttribute {
        self.default_mutate_unnamed_function_attribute(source)
    }

    fn default_mutate_fallback_function_attribute(
        &mut self,
        source: &input::FallbackFunctionAttribute,
    ) -> output::FallbackFunctionAttribute {
        match source {
            input::FallbackFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                output::FallbackFunctionAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            input::FallbackFunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                output::FallbackFunctionAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
                )
            }
            input::FallbackFunctionAttribute::ExternalKeyword => {
                output::FallbackFunctionAttribute::ExternalKeyword
            }
            input::FallbackFunctionAttribute::PayableKeyword => {
                output::FallbackFunctionAttribute::PayableKeyword
            }
            input::FallbackFunctionAttribute::PureKeyword => {
                output::FallbackFunctionAttribute::PureKeyword
            }
            input::FallbackFunctionAttribute::ViewKeyword => {
                output::FallbackFunctionAttribute::ViewKeyword
            }
            input::FallbackFunctionAttribute::VirtualKeyword => {
                output::FallbackFunctionAttribute::VirtualKeyword
            }
        }
    }
    fn mutate_fallback_function_attribute(
        &mut self,
        source: &input::FallbackFunctionAttribute,
    ) -> output::FallbackFunctionAttribute {
        self.default_mutate_fallback_function_attribute(source)
    }

    fn default_mutate_receive_function_attribute(
        &mut self,
        source: &input::ReceiveFunctionAttribute,
    ) -> output::ReceiveFunctionAttribute {
        match source {
            input::ReceiveFunctionAttribute::ModifierInvocation(ref modifier_invocation) => {
                output::ReceiveFunctionAttribute::ModifierInvocation(
                    self.mutate_modifier_invocation(modifier_invocation),
                )
            }
            input::ReceiveFunctionAttribute::OverrideSpecifier(ref override_specifier) => {
                output::ReceiveFunctionAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
                )
            }
            input::ReceiveFunctionAttribute::ExternalKeyword => {
                output::ReceiveFunctionAttribute::ExternalKeyword
            }
            input::ReceiveFunctionAttribute::PayableKeyword => {
                output::ReceiveFunctionAttribute::PayableKeyword
            }
            input::ReceiveFunctionAttribute::VirtualKeyword => {
                output::ReceiveFunctionAttribute::VirtualKeyword
            }
        }
    }
    fn mutate_receive_function_attribute(
        &mut self,
        source: &input::ReceiveFunctionAttribute,
    ) -> output::ReceiveFunctionAttribute {
        self.default_mutate_receive_function_attribute(source)
    }

    fn default_mutate_modifier_attribute(
        &mut self,
        source: &input::ModifierAttribute,
    ) -> output::ModifierAttribute {
        match source {
            input::ModifierAttribute::OverrideSpecifier(ref override_specifier) => {
                output::ModifierAttribute::OverrideSpecifier(
                    self.mutate_override_specifier(override_specifier),
                )
            }
            input::ModifierAttribute::VirtualKeyword => output::ModifierAttribute::VirtualKeyword,
        }
    }
    fn mutate_modifier_attribute(
        &mut self,
        source: &input::ModifierAttribute,
    ) -> output::ModifierAttribute {
        self.default_mutate_modifier_attribute(source)
    }

    fn default_mutate_type_name(&mut self, source: &input::TypeName) -> output::TypeName {
        match source {
            input::TypeName::ArrayTypeName(ref array_type_name) => {
                output::TypeName::ArrayTypeName(self.mutate_array_type_name(array_type_name))
            }
            input::TypeName::FunctionType(ref function_type) => {
                output::TypeName::FunctionType(self.mutate_function_type(function_type))
            }
            input::TypeName::MappingType(ref mapping_type) => {
                output::TypeName::MappingType(self.mutate_mapping_type(mapping_type))
            }
            input::TypeName::ElementaryType(ref elementary_type) => {
                output::TypeName::ElementaryType(self.mutate_elementary_type(elementary_type))
            }
            input::TypeName::IdentifierPath(ref identifier_path) => {
                output::TypeName::IdentifierPath(self.mutate_identifier_path(identifier_path))
            }
        }
    }
    fn mutate_type_name(&mut self, source: &input::TypeName) -> output::TypeName {
        self.default_mutate_type_name(source)
    }

    fn default_mutate_function_type_attribute(
        &mut self,
        source: &input::FunctionTypeAttribute,
    ) -> output::FunctionTypeAttribute {
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
    fn mutate_function_type_attribute(
        &mut self,
        source: &input::FunctionTypeAttribute,
    ) -> output::FunctionTypeAttribute {
        self.default_mutate_function_type_attribute(source)
    }

    fn default_mutate_mapping_key_type(
        &mut self,
        source: &input::MappingKeyType,
    ) -> output::MappingKeyType {
        match source {
            input::MappingKeyType::ElementaryType(ref elementary_type) => {
                output::MappingKeyType::ElementaryType(self.mutate_elementary_type(elementary_type))
            }
            input::MappingKeyType::IdentifierPath(ref identifier_path) => {
                output::MappingKeyType::IdentifierPath(self.mutate_identifier_path(identifier_path))
            }
        }
    }
    fn mutate_mapping_key_type(
        &mut self,
        source: &input::MappingKeyType,
    ) -> output::MappingKeyType {
        self.default_mutate_mapping_key_type(source)
    }

    fn default_mutate_elementary_type(
        &mut self,
        source: &input::ElementaryType,
    ) -> output::ElementaryType {
        match source {
            input::ElementaryType::AddressType(ref address_type) => {
                output::ElementaryType::AddressType(self.mutate_address_type(address_type))
            }
            input::ElementaryType::BoolKeyword => output::ElementaryType::BoolKeyword,
            input::ElementaryType::ByteKeyword => output::ElementaryType::ByteKeyword,
            input::ElementaryType::StringKeyword => output::ElementaryType::StringKeyword,
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
        }
    }
    fn mutate_elementary_type(&mut self, source: &input::ElementaryType) -> output::ElementaryType {
        self.default_mutate_elementary_type(source)
    }

    fn default_mutate_statement(&mut self, source: &input::Statement) -> output::Statement {
        match source {
            input::Statement::IfStatement(ref if_statement) => {
                output::Statement::IfStatement(self.mutate_if_statement(if_statement))
            }
            input::Statement::ForStatement(ref for_statement) => {
                output::Statement::ForStatement(self.mutate_for_statement(for_statement))
            }
            input::Statement::WhileStatement(ref while_statement) => {
                output::Statement::WhileStatement(self.mutate_while_statement(while_statement))
            }
            input::Statement::DoWhileStatement(ref do_while_statement) => {
                output::Statement::DoWhileStatement(
                    self.mutate_do_while_statement(do_while_statement),
                )
            }
            input::Statement::ContinueStatement(ref continue_statement) => {
                output::Statement::ContinueStatement(
                    self.mutate_continue_statement(continue_statement),
                )
            }
            input::Statement::BreakStatement(ref break_statement) => {
                output::Statement::BreakStatement(self.mutate_break_statement(break_statement))
            }
            input::Statement::ReturnStatement(ref return_statement) => {
                output::Statement::ReturnStatement(self.mutate_return_statement(return_statement))
            }
            input::Statement::ThrowStatement(ref throw_statement) => {
                output::Statement::ThrowStatement(self.mutate_throw_statement(throw_statement))
            }
            input::Statement::EmitStatement(ref emit_statement) => {
                output::Statement::EmitStatement(self.mutate_emit_statement(emit_statement))
            }
            input::Statement::TryStatement(ref try_statement) => {
                output::Statement::TryStatement(self.mutate_try_statement(try_statement))
            }
            input::Statement::RevertStatement(ref revert_statement) => {
                output::Statement::RevertStatement(self.mutate_revert_statement(revert_statement))
            }
            input::Statement::AssemblyStatement(ref assembly_statement) => {
                output::Statement::AssemblyStatement(
                    self.mutate_assembly_statement(assembly_statement),
                )
            }
            input::Statement::Block(ref block) => {
                output::Statement::Block(self.mutate_block(block))
            }
            input::Statement::UncheckedBlock(ref unchecked_block) => {
                output::Statement::UncheckedBlock(self.mutate_unchecked_block(unchecked_block))
            }
            input::Statement::TupleDeconstructionStatement(ref tuple_deconstruction_statement) => {
                output::Statement::TupleDeconstructionStatement(
                    self.mutate_tuple_deconstruction_statement(tuple_deconstruction_statement),
                )
            }
            input::Statement::VariableDeclarationStatement(ref variable_declaration_statement) => {
                output::Statement::VariableDeclarationStatement(
                    self.mutate_variable_declaration_statement(variable_declaration_statement),
                )
            }
            input::Statement::ExpressionStatement(ref expression_statement) => {
                output::Statement::ExpressionStatement(
                    self.mutate_expression_statement(expression_statement),
                )
            }
        }
    }
    fn mutate_statement(&mut self, source: &input::Statement) -> output::Statement {
        self.default_mutate_statement(source)
    }

    fn default_mutate_tuple_member(&mut self, source: &input::TupleMember) -> output::TupleMember {
        match source {
            input::TupleMember::TypedTupleMember(ref typed_tuple_member) => {
                output::TupleMember::TypedTupleMember(
                    self.mutate_typed_tuple_member(typed_tuple_member),
                )
            }
            input::TupleMember::UntypedTupleMember(ref untyped_tuple_member) => {
                output::TupleMember::UntypedTupleMember(
                    self.mutate_untyped_tuple_member(untyped_tuple_member),
                )
            }
        }
    }
    fn mutate_tuple_member(&mut self, source: &input::TupleMember) -> output::TupleMember {
        self.default_mutate_tuple_member(source)
    }

    fn default_mutate_variable_declaration_type(
        &mut self,
        source: &input::VariableDeclarationType,
    ) -> output::VariableDeclarationType {
        match source {
            input::VariableDeclarationType::TypeName(ref type_name) => {
                output::VariableDeclarationType::TypeName(self.mutate_type_name(type_name))
            }
            input::VariableDeclarationType::VarKeyword => {
                output::VariableDeclarationType::VarKeyword
            }
        }
    }
    fn mutate_variable_declaration_type(
        &mut self,
        source: &input::VariableDeclarationType,
    ) -> output::VariableDeclarationType {
        self.default_mutate_variable_declaration_type(source)
    }

    fn default_mutate_storage_location(
        &mut self,
        source: &input::StorageLocation,
    ) -> output::StorageLocation {
        match source {
            input::StorageLocation::MemoryKeyword => output::StorageLocation::MemoryKeyword,
            input::StorageLocation::StorageKeyword => output::StorageLocation::StorageKeyword,
            input::StorageLocation::CallDataKeyword => output::StorageLocation::CallDataKeyword,
        }
    }
    fn mutate_storage_location(
        &mut self,
        source: &input::StorageLocation,
    ) -> output::StorageLocation {
        self.default_mutate_storage_location(source)
    }

    fn default_mutate_for_statement_initialization(
        &mut self,
        source: &input::ForStatementInitialization,
    ) -> output::ForStatementInitialization {
        match source {
            input::ForStatementInitialization::TupleDeconstructionStatement(
                ref tuple_deconstruction_statement,
            ) => output::ForStatementInitialization::TupleDeconstructionStatement(
                self.mutate_tuple_deconstruction_statement(tuple_deconstruction_statement),
            ),
            input::ForStatementInitialization::VariableDeclarationStatement(
                ref variable_declaration_statement,
            ) => output::ForStatementInitialization::VariableDeclarationStatement(
                self.mutate_variable_declaration_statement(variable_declaration_statement),
            ),
            input::ForStatementInitialization::ExpressionStatement(ref expression_statement) => {
                output::ForStatementInitialization::ExpressionStatement(
                    self.mutate_expression_statement(expression_statement),
                )
            }
            input::ForStatementInitialization::Semicolon => {
                output::ForStatementInitialization::Semicolon
            }
        }
    }
    fn mutate_for_statement_initialization(
        &mut self,
        source: &input::ForStatementInitialization,
    ) -> output::ForStatementInitialization {
        self.default_mutate_for_statement_initialization(source)
    }

    fn default_mutate_for_statement_condition(
        &mut self,
        source: &input::ForStatementCondition,
    ) -> output::ForStatementCondition {
        match source {
            input::ForStatementCondition::ExpressionStatement(ref expression_statement) => {
                output::ForStatementCondition::ExpressionStatement(
                    self.mutate_expression_statement(expression_statement),
                )
            }
            input::ForStatementCondition::Semicolon => output::ForStatementCondition::Semicolon,
        }
    }
    fn mutate_for_statement_condition(
        &mut self,
        source: &input::ForStatementCondition,
    ) -> output::ForStatementCondition {
        self.default_mutate_for_statement_condition(source)
    }

    fn default_mutate_expression(&mut self, source: &input::Expression) -> output::Expression {
        match source {
            input::Expression::AssignmentExpression(ref assignment_expression) => {
                output::Expression::AssignmentExpression(
                    self.mutate_assignment_expression(assignment_expression),
                )
            }
            input::Expression::ConditionalExpression(ref conditional_expression) => {
                output::Expression::ConditionalExpression(
                    self.mutate_conditional_expression(conditional_expression),
                )
            }
            input::Expression::OrExpression(ref or_expression) => {
                output::Expression::OrExpression(self.mutate_or_expression(or_expression))
            }
            input::Expression::AndExpression(ref and_expression) => {
                output::Expression::AndExpression(self.mutate_and_expression(and_expression))
            }
            input::Expression::EqualityExpression(ref equality_expression) => {
                output::Expression::EqualityExpression(
                    self.mutate_equality_expression(equality_expression),
                )
            }
            input::Expression::InequalityExpression(ref inequality_expression) => {
                output::Expression::InequalityExpression(
                    self.mutate_inequality_expression(inequality_expression),
                )
            }
            input::Expression::BitwiseOrExpression(ref bitwise_or_expression) => {
                output::Expression::BitwiseOrExpression(
                    self.mutate_bitwise_or_expression(bitwise_or_expression),
                )
            }
            input::Expression::BitwiseXorExpression(ref bitwise_xor_expression) => {
                output::Expression::BitwiseXorExpression(
                    self.mutate_bitwise_xor_expression(bitwise_xor_expression),
                )
            }
            input::Expression::BitwiseAndExpression(ref bitwise_and_expression) => {
                output::Expression::BitwiseAndExpression(
                    self.mutate_bitwise_and_expression(bitwise_and_expression),
                )
            }
            input::Expression::ShiftExpression(ref shift_expression) => {
                output::Expression::ShiftExpression(self.mutate_shift_expression(shift_expression))
            }
            input::Expression::AdditiveExpression(ref additive_expression) => {
                output::Expression::AdditiveExpression(
                    self.mutate_additive_expression(additive_expression),
                )
            }
            input::Expression::MultiplicativeExpression(ref multiplicative_expression) => {
                output::Expression::MultiplicativeExpression(
                    self.mutate_multiplicative_expression(multiplicative_expression),
                )
            }
            input::Expression::ExponentiationExpression(ref exponentiation_expression) => {
                output::Expression::ExponentiationExpression(
                    self.mutate_exponentiation_expression(exponentiation_expression),
                )
            }
            input::Expression::PostfixExpression(ref postfix_expression) => {
                output::Expression::PostfixExpression(
                    self.mutate_postfix_expression(postfix_expression),
                )
            }
            input::Expression::PrefixExpression(ref prefix_expression) => {
                output::Expression::PrefixExpression(
                    self.mutate_prefix_expression(prefix_expression),
                )
            }
            input::Expression::FunctionCallExpression(ref function_call_expression) => {
                output::Expression::FunctionCallExpression(
                    self.mutate_function_call_expression(function_call_expression),
                )
            }
            input::Expression::CallOptionsExpression(ref call_options_expression) => {
                output::Expression::CallOptionsExpression(
                    self.mutate_call_options_expression(call_options_expression),
                )
            }
            input::Expression::MemberAccessExpression(ref member_access_expression) => {
                output::Expression::MemberAccessExpression(
                    self.mutate_member_access_expression(member_access_expression),
                )
            }
            input::Expression::IndexAccessExpression(ref index_access_expression) => {
                output::Expression::IndexAccessExpression(
                    self.mutate_index_access_expression(index_access_expression),
                )
            }
            input::Expression::NewExpression(ref new_expression) => {
                output::Expression::NewExpression(self.mutate_new_expression(new_expression))
            }
            input::Expression::TupleExpression(ref tuple_expression) => {
                output::Expression::TupleExpression(self.mutate_tuple_expression(tuple_expression))
            }
            input::Expression::TypeExpression(ref type_expression) => {
                output::Expression::TypeExpression(self.mutate_type_expression(type_expression))
            }
            input::Expression::ArrayExpression(ref array_expression) => {
                output::Expression::ArrayExpression(self.mutate_array_expression(array_expression))
            }
            input::Expression::HexNumberExpression(ref hex_number_expression) => {
                output::Expression::HexNumberExpression(
                    self.mutate_hex_number_expression(hex_number_expression),
                )
            }
            input::Expression::DecimalNumberExpression(ref decimal_number_expression) => {
                output::Expression::DecimalNumberExpression(
                    self.mutate_decimal_number_expression(decimal_number_expression),
                )
            }
            input::Expression::StringExpression(ref string_expression) => {
                output::Expression::StringExpression(
                    self.mutate_string_expression(string_expression),
                )
            }
            input::Expression::ElementaryType(ref elementary_type) => {
                output::Expression::ElementaryType(self.mutate_elementary_type(elementary_type))
            }
            input::Expression::PayableKeyword => output::Expression::PayableKeyword,
            input::Expression::ThisKeyword => output::Expression::ThisKeyword,
            input::Expression::SuperKeyword => output::Expression::SuperKeyword,
            input::Expression::TrueKeyword => output::Expression::TrueKeyword,
            input::Expression::FalseKeyword => output::Expression::FalseKeyword,
            input::Expression::Identifier(node) => output::Expression::Identifier(Rc::clone(node)),
        }
    }
    fn mutate_expression(&mut self, source: &input::Expression) -> output::Expression {
        self.default_mutate_expression(source)
    }

    fn default_mutate_arguments_declaration(
        &mut self,
        source: &input::ArgumentsDeclaration,
    ) -> output::ArgumentsDeclaration {
        match source {
            input::ArgumentsDeclaration::PositionalArgumentsDeclaration(
                ref positional_arguments_declaration,
            ) => output::ArgumentsDeclaration::PositionalArgumentsDeclaration(
                self.mutate_positional_arguments_declaration(positional_arguments_declaration),
            ),
            input::ArgumentsDeclaration::NamedArgumentsDeclaration(
                ref named_arguments_declaration,
            ) => output::ArgumentsDeclaration::NamedArgumentsDeclaration(
                self.mutate_named_arguments_declaration(named_arguments_declaration),
            ),
        }
    }
    fn mutate_arguments_declaration(
        &mut self,
        source: &input::ArgumentsDeclaration,
    ) -> output::ArgumentsDeclaration {
        self.default_mutate_arguments_declaration(source)
    }

    fn default_mutate_number_unit(&mut self, source: &input::NumberUnit) -> output::NumberUnit {
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
    fn mutate_number_unit(&mut self, source: &input::NumberUnit) -> output::NumberUnit {
        self.default_mutate_number_unit(source)
    }

    fn default_mutate_string_expression(
        &mut self,
        source: &input::StringExpression,
    ) -> output::StringExpression {
        match source {
            input::StringExpression::StringLiteral(ref string_literal) => {
                output::StringExpression::StringLiteral(self.mutate_string_literal(string_literal))
            }
            input::StringExpression::StringLiterals(ref string_literals) => {
                output::StringExpression::StringLiterals(
                    self.mutate_string_literals(string_literals),
                )
            }
            input::StringExpression::HexStringLiteral(ref hex_string_literal) => {
                output::StringExpression::HexStringLiteral(
                    self.mutate_hex_string_literal(hex_string_literal),
                )
            }
            input::StringExpression::HexStringLiterals(ref hex_string_literals) => {
                output::StringExpression::HexStringLiterals(
                    self.mutate_hex_string_literals(hex_string_literals),
                )
            }
            input::StringExpression::UnicodeStringLiterals(ref unicode_string_literals) => {
                output::StringExpression::UnicodeStringLiterals(
                    self.mutate_unicode_string_literals(unicode_string_literals),
                )
            }
        }
    }
    fn mutate_string_expression(
        &mut self,
        source: &input::StringExpression,
    ) -> output::StringExpression {
        self.default_mutate_string_expression(source)
    }

    fn default_mutate_string_literal(
        &mut self,
        source: &input::StringLiteral,
    ) -> output::StringLiteral {
        match source {
            input::StringLiteral::SingleQuotedStringLiteral(node) => {
                output::StringLiteral::SingleQuotedStringLiteral(Rc::clone(node))
            }
            input::StringLiteral::DoubleQuotedStringLiteral(node) => {
                output::StringLiteral::DoubleQuotedStringLiteral(Rc::clone(node))
            }
        }
    }
    fn mutate_string_literal(&mut self, source: &input::StringLiteral) -> output::StringLiteral {
        self.default_mutate_string_literal(source)
    }

    fn default_mutate_hex_string_literal(
        &mut self,
        source: &input::HexStringLiteral,
    ) -> output::HexStringLiteral {
        match source {
            input::HexStringLiteral::SingleQuotedHexStringLiteral(node) => {
                output::HexStringLiteral::SingleQuotedHexStringLiteral(Rc::clone(node))
            }
            input::HexStringLiteral::DoubleQuotedHexStringLiteral(node) => {
                output::HexStringLiteral::DoubleQuotedHexStringLiteral(Rc::clone(node))
            }
        }
    }
    fn mutate_hex_string_literal(
        &mut self,
        source: &input::HexStringLiteral,
    ) -> output::HexStringLiteral {
        self.default_mutate_hex_string_literal(source)
    }

    fn default_mutate_unicode_string_literal(
        &mut self,
        source: &input::UnicodeStringLiteral,
    ) -> output::UnicodeStringLiteral {
        match source {
            input::UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(node) => {
                output::UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(Rc::clone(node))
            }
            input::UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(node) => {
                output::UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(Rc::clone(node))
            }
        }
    }
    fn mutate_unicode_string_literal(
        &mut self,
        source: &input::UnicodeStringLiteral,
    ) -> output::UnicodeStringLiteral {
        self.default_mutate_unicode_string_literal(source)
    }

    fn default_mutate_yul_statement(
        &mut self,
        source: &input::YulStatement,
    ) -> output::YulStatement {
        match source {
            input::YulStatement::YulBlock(ref yul_block) => {
                output::YulStatement::YulBlock(self.mutate_yul_block(yul_block))
            }
            input::YulStatement::YulFunctionDefinition(ref yul_function_definition) => {
                output::YulStatement::YulFunctionDefinition(
                    self.mutate_yul_function_definition(yul_function_definition),
                )
            }
            input::YulStatement::YulStackAssignmentStatement(
                ref yul_stack_assignment_statement,
            ) => output::YulStatement::YulStackAssignmentStatement(
                self.mutate_yul_stack_assignment_statement(yul_stack_assignment_statement),
            ),
            input::YulStatement::YulIfStatement(ref yul_if_statement) => {
                output::YulStatement::YulIfStatement(self.mutate_yul_if_statement(yul_if_statement))
            }
            input::YulStatement::YulForStatement(ref yul_for_statement) => {
                output::YulStatement::YulForStatement(
                    self.mutate_yul_for_statement(yul_for_statement),
                )
            }
            input::YulStatement::YulSwitchStatement(ref yul_switch_statement) => {
                output::YulStatement::YulSwitchStatement(
                    self.mutate_yul_switch_statement(yul_switch_statement),
                )
            }
            input::YulStatement::YulLeaveStatement(ref yul_leave_statement) => {
                output::YulStatement::YulLeaveStatement(
                    self.mutate_yul_leave_statement(yul_leave_statement),
                )
            }
            input::YulStatement::YulBreakStatement(ref yul_break_statement) => {
                output::YulStatement::YulBreakStatement(
                    self.mutate_yul_break_statement(yul_break_statement),
                )
            }
            input::YulStatement::YulContinueStatement(ref yul_continue_statement) => {
                output::YulStatement::YulContinueStatement(
                    self.mutate_yul_continue_statement(yul_continue_statement),
                )
            }
            input::YulStatement::YulVariableAssignmentStatement(
                ref yul_variable_assignment_statement,
            ) => output::YulStatement::YulVariableAssignmentStatement(
                self.mutate_yul_variable_assignment_statement(yul_variable_assignment_statement),
            ),
            input::YulStatement::YulLabel(ref yul_label) => {
                output::YulStatement::YulLabel(self.mutate_yul_label(yul_label))
            }
            input::YulStatement::YulVariableDeclarationStatement(
                ref yul_variable_declaration_statement,
            ) => output::YulStatement::YulVariableDeclarationStatement(
                self.mutate_yul_variable_declaration_statement(yul_variable_declaration_statement),
            ),
            input::YulStatement::YulExpression(ref yul_expression) => {
                output::YulStatement::YulExpression(self.mutate_yul_expression(yul_expression))
            }
        }
    }
    fn mutate_yul_statement(&mut self, source: &input::YulStatement) -> output::YulStatement {
        self.default_mutate_yul_statement(source)
    }

    fn default_mutate_yul_assignment_operator(
        &mut self,
        source: &input::YulAssignmentOperator,
    ) -> output::YulAssignmentOperator {
        match source {
            input::YulAssignmentOperator::YulColonAndEqual(ref yul_colon_and_equal) => {
                output::YulAssignmentOperator::YulColonAndEqual(
                    self.mutate_yul_colon_and_equal(yul_colon_and_equal),
                )
            }
            input::YulAssignmentOperator::ColonEqual => output::YulAssignmentOperator::ColonEqual,
        }
    }
    fn mutate_yul_assignment_operator(
        &mut self,
        source: &input::YulAssignmentOperator,
    ) -> output::YulAssignmentOperator {
        self.default_mutate_yul_assignment_operator(source)
    }

    fn default_mutate_yul_stack_assignment_operator(
        &mut self,
        source: &input::YulStackAssignmentOperator,
    ) -> output::YulStackAssignmentOperator {
        match source {
            input::YulStackAssignmentOperator::YulEqualAndColon(ref yul_equal_and_colon) => {
                output::YulStackAssignmentOperator::YulEqualAndColon(
                    self.mutate_yul_equal_and_colon(yul_equal_and_colon),
                )
            }
            input::YulStackAssignmentOperator::EqualColon => {
                output::YulStackAssignmentOperator::EqualColon
            }
        }
    }
    fn mutate_yul_stack_assignment_operator(
        &mut self,
        source: &input::YulStackAssignmentOperator,
    ) -> output::YulStackAssignmentOperator {
        self.default_mutate_yul_stack_assignment_operator(source)
    }

    fn default_mutate_yul_switch_case(
        &mut self,
        source: &input::YulSwitchCase,
    ) -> output::YulSwitchCase {
        match source {
            input::YulSwitchCase::YulDefaultCase(ref yul_default_case) => {
                output::YulSwitchCase::YulDefaultCase(
                    self.mutate_yul_default_case(yul_default_case),
                )
            }
            input::YulSwitchCase::YulValueCase(ref yul_value_case) => {
                output::YulSwitchCase::YulValueCase(self.mutate_yul_value_case(yul_value_case))
            }
        }
    }
    fn mutate_yul_switch_case(&mut self, source: &input::YulSwitchCase) -> output::YulSwitchCase {
        self.default_mutate_yul_switch_case(source)
    }

    fn default_mutate_yul_expression(
        &mut self,
        source: &input::YulExpression,
    ) -> output::YulExpression {
        match source {
            input::YulExpression::YulFunctionCallExpression(ref yul_function_call_expression) => {
                output::YulExpression::YulFunctionCallExpression(
                    self.mutate_yul_function_call_expression(yul_function_call_expression),
                )
            }
            input::YulExpression::YulLiteral(ref yul_literal) => {
                output::YulExpression::YulLiteral(self.mutate_yul_literal(yul_literal))
            }
            input::YulExpression::YulPath(ref yul_path) => {
                output::YulExpression::YulPath(self.mutate_yul_path(yul_path))
            }
        }
    }
    fn mutate_yul_expression(&mut self, source: &input::YulExpression) -> output::YulExpression {
        self.default_mutate_yul_expression(source)
    }

    fn default_mutate_yul_literal(&mut self, source: &input::YulLiteral) -> output::YulLiteral {
        match source {
            input::YulLiteral::HexStringLiteral(ref hex_string_literal) => {
                output::YulLiteral::HexStringLiteral(
                    self.mutate_hex_string_literal(hex_string_literal),
                )
            }
            input::YulLiteral::StringLiteral(ref string_literal) => {
                output::YulLiteral::StringLiteral(self.mutate_string_literal(string_literal))
            }
            input::YulLiteral::YulTrueKeyword => output::YulLiteral::YulTrueKeyword,
            input::YulLiteral::YulFalseKeyword => output::YulLiteral::YulFalseKeyword,
            input::YulLiteral::YulDecimalLiteral(node) => {
                output::YulLiteral::YulDecimalLiteral(Rc::clone(node))
            }
            input::YulLiteral::YulHexLiteral(node) => {
                output::YulLiteral::YulHexLiteral(Rc::clone(node))
            }
        }
    }
    fn mutate_yul_literal(&mut self, source: &input::YulLiteral) -> output::YulLiteral {
        self.default_mutate_yul_literal(source)
    }

    //
    // Repeated:
    //

    fn mutate_source_unit_members(
        &mut self,
        source: &input::SourceUnitMembers,
    ) -> output::SourceUnitMembers {
        source
            .iter()
            .map(|item| self.mutate_source_unit_member(item))
            .collect()
    }

    fn mutate_version_expression_set(
        &mut self,
        source: &input::VersionExpressionSet,
    ) -> output::VersionExpressionSet {
        source
            .iter()
            .map(|item| self.mutate_version_expression(item))
            .collect()
    }

    fn mutate_contract_members(
        &mut self,
        source: &input::ContractMembers,
    ) -> output::ContractMembers {
        source
            .iter()
            .map(|item| self.mutate_contract_member(item))
            .collect()
    }

    fn mutate_interface_members(
        &mut self,
        source: &input::InterfaceMembers,
    ) -> output::InterfaceMembers {
        source
            .iter()
            .map(|item| self.mutate_contract_member(item))
            .collect()
    }

    fn mutate_library_members(&mut self, source: &input::LibraryMembers) -> output::LibraryMembers {
        source
            .iter()
            .map(|item| self.mutate_contract_member(item))
            .collect()
    }

    fn mutate_struct_members(&mut self, source: &input::StructMembers) -> output::StructMembers {
        source
            .iter()
            .map(|item| self.mutate_struct_member(item))
            .collect()
    }

    fn mutate_state_variable_attributes(
        &mut self,
        source: &input::StateVariableAttributes,
    ) -> output::StateVariableAttributes {
        source
            .iter()
            .map(|item| self.mutate_state_variable_attribute(item))
            .collect()
    }

    fn mutate_function_attributes(
        &mut self,
        source: &input::FunctionAttributes,
    ) -> output::FunctionAttributes {
        source
            .iter()
            .map(|item| self.mutate_function_attribute(item))
            .collect()
    }

    fn mutate_constructor_attributes(
        &mut self,
        source: &input::ConstructorAttributes,
    ) -> output::ConstructorAttributes {
        source
            .iter()
            .map(|item| self.mutate_constructor_attribute(item))
            .collect()
    }

    fn mutate_unnamed_function_attributes(
        &mut self,
        source: &input::UnnamedFunctionAttributes,
    ) -> output::UnnamedFunctionAttributes {
        source
            .iter()
            .map(|item| self.mutate_unnamed_function_attribute(item))
            .collect()
    }

    fn mutate_fallback_function_attributes(
        &mut self,
        source: &input::FallbackFunctionAttributes,
    ) -> output::FallbackFunctionAttributes {
        source
            .iter()
            .map(|item| self.mutate_fallback_function_attribute(item))
            .collect()
    }

    fn mutate_receive_function_attributes(
        &mut self,
        source: &input::ReceiveFunctionAttributes,
    ) -> output::ReceiveFunctionAttributes {
        source
            .iter()
            .map(|item| self.mutate_receive_function_attribute(item))
            .collect()
    }

    fn mutate_modifier_attributes(
        &mut self,
        source: &input::ModifierAttributes,
    ) -> output::ModifierAttributes {
        source
            .iter()
            .map(|item| self.mutate_modifier_attribute(item))
            .collect()
    }

    fn mutate_function_type_attributes(
        &mut self,
        source: &input::FunctionTypeAttributes,
    ) -> output::FunctionTypeAttributes {
        source
            .iter()
            .map(|item| self.mutate_function_type_attribute(item))
            .collect()
    }

    fn mutate_statements(&mut self, source: &input::Statements) -> output::Statements {
        source
            .iter()
            .map(|item| self.mutate_statement(item))
            .collect()
    }

    fn mutate_catch_clauses(&mut self, source: &input::CatchClauses) -> output::CatchClauses {
        source
            .iter()
            .map(|item| self.mutate_catch_clause(item))
            .collect()
    }

    fn mutate_string_literals(&mut self, source: &input::StringLiterals) -> output::StringLiterals {
        source
            .iter()
            .map(|item| self.mutate_string_literal(item))
            .collect()
    }

    fn mutate_hex_string_literals(
        &mut self,
        source: &input::HexStringLiterals,
    ) -> output::HexStringLiterals {
        source
            .iter()
            .map(|item| self.mutate_hex_string_literal(item))
            .collect()
    }

    fn mutate_unicode_string_literals(
        &mut self,
        source: &input::UnicodeStringLiterals,
    ) -> output::UnicodeStringLiterals {
        source
            .iter()
            .map(|item| self.mutate_unicode_string_literal(item))
            .collect()
    }

    fn mutate_yul_statements(&mut self, source: &input::YulStatements) -> output::YulStatements {
        source
            .iter()
            .map(|item| self.mutate_yul_statement(item))
            .collect()
    }

    fn mutate_yul_switch_cases(
        &mut self,
        source: &input::YulSwitchCases,
    ) -> output::YulSwitchCases {
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
        source: &input::VersionExpressionSets,
    ) -> output::VersionExpressionSets {
        source
            .iter()
            .map(|item| self.mutate_version_expression_set(item))
            .collect()
    }

    fn mutate_simple_version_literal(
        &mut self,
        source: &input::SimpleVersionLiteral,
    ) -> output::SimpleVersionLiteral {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_import_deconstruction_symbols(
        &mut self,
        source: &input::ImportDeconstructionSymbols,
    ) -> output::ImportDeconstructionSymbols {
        source
            .iter()
            .map(|item| self.mutate_import_deconstruction_symbol(item))
            .collect()
    }

    fn mutate_using_deconstruction_symbols(
        &mut self,
        source: &input::UsingDeconstructionSymbols,
    ) -> output::UsingDeconstructionSymbols {
        source
            .iter()
            .map(|item| self.mutate_using_deconstruction_symbol(item))
            .collect()
    }

    fn mutate_inheritance_types(
        &mut self,
        source: &input::InheritanceTypes,
    ) -> output::InheritanceTypes {
        source
            .iter()
            .map(|item| self.mutate_inheritance_type(item))
            .collect()
    }

    fn mutate_enum_members(&mut self, source: &input::EnumMembers) -> output::EnumMembers {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_parameters(&mut self, source: &input::Parameters) -> output::Parameters {
        source
            .iter()
            .map(|item| self.mutate_parameter(item))
            .collect()
    }

    fn mutate_override_paths(&mut self, source: &input::OverridePaths) -> output::OverridePaths {
        source
            .iter()
            .map(|item| self.mutate_identifier_path(item))
            .collect()
    }

    fn mutate_event_parameters(
        &mut self,
        source: &input::EventParameters,
    ) -> output::EventParameters {
        source
            .iter()
            .map(|item| self.mutate_event_parameter(item))
            .collect()
    }

    fn mutate_error_parameters(
        &mut self,
        source: &input::ErrorParameters,
    ) -> output::ErrorParameters {
        source
            .iter()
            .map(|item| self.mutate_error_parameter(item))
            .collect()
    }

    fn mutate_assembly_flags(&mut self, source: &input::AssemblyFlags) -> output::AssemblyFlags {
        source
            .iter()
            .map(|item| self.mutate_string_literal(item))
            .collect()
    }

    fn mutate_tuple_deconstruction_elements(
        &mut self,
        source: &input::TupleDeconstructionElements,
    ) -> output::TupleDeconstructionElements {
        source
            .iter()
            .map(|item| self.mutate_tuple_deconstruction_element(item))
            .collect()
    }

    fn mutate_positional_arguments(
        &mut self,
        source: &input::PositionalArguments,
    ) -> output::PositionalArguments {
        source
            .iter()
            .map(|item| self.mutate_expression(item))
            .collect()
    }

    fn mutate_named_arguments(&mut self, source: &input::NamedArguments) -> output::NamedArguments {
        source
            .iter()
            .map(|item| self.mutate_named_argument(item))
            .collect()
    }

    fn mutate_call_options(&mut self, source: &input::CallOptions) -> output::CallOptions {
        source
            .iter()
            .map(|item| self.mutate_named_argument(item))
            .collect()
    }

    fn mutate_tuple_values(&mut self, source: &input::TupleValues) -> output::TupleValues {
        source
            .iter()
            .map(|item| self.mutate_tuple_value(item))
            .collect()
    }

    fn mutate_array_values(&mut self, source: &input::ArrayValues) -> output::ArrayValues {
        source
            .iter()
            .map(|item| self.mutate_expression(item))
            .collect()
    }

    fn mutate_identifier_path(&mut self, source: &input::IdentifierPath) -> output::IdentifierPath {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_yul_parameters(&mut self, source: &input::YulParameters) -> output::YulParameters {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_yul_variable_names(
        &mut self,
        source: &input::YulVariableNames,
    ) -> output::YulVariableNames {
        source.iter().map(Rc::clone).collect()
    }

    fn mutate_yul_arguments(&mut self, source: &input::YulArguments) -> output::YulArguments {
        source
            .iter()
            .map(|item| self.mutate_yul_expression(item))
            .collect()
    }

    fn mutate_yul_paths(&mut self, source: &input::YulPaths) -> output::YulPaths {
        source
            .iter()
            .map(|item| self.mutate_yul_path(item))
            .collect()
    }

    fn mutate_yul_path(&mut self, source: &input::YulPath) -> output::YulPath {
        source.iter().map(Rc::clone).collect()
    }
}
