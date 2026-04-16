use std::ops::Range;
use std::rc::Rc;

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::parser::parser_helpers;
use slang_solidity_v2_parser::{Parser, ParserError};

use crate::ir::nodes as output;

mod intermediate_types;
use intermediate_types as intermediate;

pub struct IrConsumer<'source> {
    source: &'source str,
}

impl<'source> IrConsumer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self { source }
    }

    fn text(&self, range: Range<usize>) -> String {
        self.source[range].to_owned()
    }
}

/// Parse Solidity source and produce an IR directly, bypassing the CST.
pub fn parse(
    input: &str,
    version: LanguageVersion,
) -> Result<output::SourceUnit, ParserError> {
    let consumer = IrConsumer::new(input);
    Parser::parse_with_consumer(input, version, &consumer)
}

// ============================================
// === Hand-Written Builder Methods ===
// ============================================
// The generated `ParserConsumer` impl delegates complex factory methods to
// the `build_*_impl` methods defined here.

impl IrConsumer<'_> {
    // ----------------------------------------
    // Collapsed choice: IdentifierPathElement → Identifier
    // Synthesize an `Identifier` from an `AddressKeyword` range.
    // ----------------------------------------
    fn build_identifier_path_element_address_keyword_impl(
        &self,
        range: Range<usize>,
    ) -> output::Identifier {
        Rc::new(output::IdentifierStruct {
            range: range.clone(),
            text: self.text(range),
        })
    }

    // ----------------------------------------
    // Removed choices: FunctionName → (FunctionKind, Option<Identifier>)
    // ----------------------------------------
    fn build_function_name_identifier_impl(
        &self,
        element: output::Identifier,
    ) -> (output::FunctionKind, Option<output::Identifier>) {
        (output::FunctionKind::Regular, Some(element))
    }

    fn build_function_name_fallback_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> (output::FunctionKind, Option<output::Identifier>) {
        (output::FunctionKind::Fallback, None)
    }

    fn build_function_name_receive_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> (output::FunctionKind, Option<output::Identifier>) {
        (output::FunctionKind::Receive, None)
    }

    // ----------------------------------------
    // Removed choices: FunctionBody → Option<Block>
    // ----------------------------------------
    fn build_function_body_block_impl(&self, block: output::Block) -> Option<output::Block> {
        Some(block)
    }

    fn build_function_body_semicolon_impl(
        &self,
        _semicolon: Range<usize>,
    ) -> Option<output::Block> {
        None
    }

    // ----------------------------------------
    // Removed choices: MappingKeyType → TypeName
    // ----------------------------------------
    fn build_mapping_key_type_elementary_type_impl(
        &self,
        element: output::ElementaryType,
    ) -> output::TypeName {
        output::TypeName::ElementaryType(element)
    }

    fn build_mapping_key_type_identifier_path_impl(
        &self,
        element: output::IdentifierPath,
    ) -> output::TypeName {
        output::TypeName::IdentifierPath(element)
    }

    // ----------------------------------------
    // Removed choices: ContractSpecifier → IrContractSpecifier
    // ----------------------------------------
    fn build_contract_specifier_inheritance_specifier_impl(
        &self,
        element: output::InheritanceTypes,
    ) -> intermediate::IrContractSpecifier {
        intermediate::IrContractSpecifier::Inheritance(element)
    }

    fn build_contract_specifier_storage_layout_specifier_impl(
        &self,
        element: output::Expression,
    ) -> intermediate::IrContractSpecifier {
        intermediate::IrContractSpecifier::StorageLayout(element)
    }

    // ----------------------------------------
    // Attribute variant factories: FunctionAttribute
    // ----------------------------------------
    fn build_function_attribute_external_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Visibility(output::FunctionVisibility::External)
    }

    fn build_function_attribute_internal_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Visibility(output::FunctionVisibility::Internal)
    }

    fn build_function_attribute_private_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Visibility(output::FunctionVisibility::Private)
    }

    fn build_function_attribute_public_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Visibility(output::FunctionVisibility::Public)
    }

    fn build_function_attribute_pure_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Mutability(output::FunctionMutability::Pure)
    }

    fn build_function_attribute_view_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Mutability(output::FunctionMutability::View)
    }

    fn build_function_attribute_payable_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Mutability(output::FunctionMutability::Payable)
    }

    fn build_function_attribute_virtual_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Virtual
    }

    fn build_function_attribute_override_specifier_impl(
        &self,
        paths: output::OverridePaths,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Override(paths)
    }

    fn build_function_attribute_modifier_invocation_impl(
        &self,
        invocation: output::ModifierInvocation,
    ) -> intermediate::IrFunctionAttribute {
        intermediate::IrFunctionAttribute::Modifier(invocation)
    }

    // ----------------------------------------
    // Attribute variant factories: ConstructorAttribute
    // ----------------------------------------
    fn build_constructor_attribute_internal_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrConstructorAttribute {
        intermediate::IrConstructorAttribute::Visibility(output::FunctionVisibility::Internal)
    }

    fn build_constructor_attribute_public_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrConstructorAttribute {
        intermediate::IrConstructorAttribute::Visibility(output::FunctionVisibility::Public)
    }

    fn build_constructor_attribute_payable_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrConstructorAttribute {
        intermediate::IrConstructorAttribute::Mutability(output::FunctionMutability::Payable)
    }

    fn build_constructor_attribute_modifier_invocation_impl(
        &self,
        invocation: output::ModifierInvocation,
    ) -> intermediate::IrConstructorAttribute {
        intermediate::IrConstructorAttribute::Modifier(invocation)
    }

    // ----------------------------------------
    // Attribute variant factories: FallbackFunctionAttribute
    // ----------------------------------------
    fn build_fallback_function_attribute_external_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFallbackFunctionAttribute {
        intermediate::IrFallbackFunctionAttribute::Visibility(output::FunctionVisibility::External)
    }

    fn build_fallback_function_attribute_payable_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFallbackFunctionAttribute {
        intermediate::IrFallbackFunctionAttribute::Mutability(output::FunctionMutability::Payable)
    }

    fn build_fallback_function_attribute_pure_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFallbackFunctionAttribute {
        intermediate::IrFallbackFunctionAttribute::Mutability(output::FunctionMutability::Pure)
    }

    fn build_fallback_function_attribute_view_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFallbackFunctionAttribute {
        intermediate::IrFallbackFunctionAttribute::Mutability(output::FunctionMutability::View)
    }

    fn build_fallback_function_attribute_virtual_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFallbackFunctionAttribute {
        intermediate::IrFallbackFunctionAttribute::Virtual
    }

    fn build_fallback_function_attribute_override_specifier_impl(
        &self,
        paths: output::OverridePaths,
    ) -> intermediate::IrFallbackFunctionAttribute {
        intermediate::IrFallbackFunctionAttribute::Override(paths)
    }

    fn build_fallback_function_attribute_modifier_invocation_impl(
        &self,
        invocation: output::ModifierInvocation,
    ) -> intermediate::IrFallbackFunctionAttribute {
        intermediate::IrFallbackFunctionAttribute::Modifier(invocation)
    }

    // ----------------------------------------
    // Attribute variant factories: ReceiveFunctionAttribute
    // ----------------------------------------
    fn build_receive_function_attribute_external_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrReceiveFunctionAttribute {
        intermediate::IrReceiveFunctionAttribute::Visibility(output::FunctionVisibility::External)
    }

    fn build_receive_function_attribute_payable_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrReceiveFunctionAttribute {
        intermediate::IrReceiveFunctionAttribute::Mutability(output::FunctionMutability::Payable)
    }

    fn build_receive_function_attribute_virtual_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrReceiveFunctionAttribute {
        intermediate::IrReceiveFunctionAttribute::Virtual
    }

    fn build_receive_function_attribute_override_specifier_impl(
        &self,
        paths: output::OverridePaths,
    ) -> intermediate::IrReceiveFunctionAttribute {
        intermediate::IrReceiveFunctionAttribute::Override(paths)
    }

    fn build_receive_function_attribute_modifier_invocation_impl(
        &self,
        invocation: output::ModifierInvocation,
    ) -> intermediate::IrReceiveFunctionAttribute {
        intermediate::IrReceiveFunctionAttribute::Modifier(invocation)
    }

    // ----------------------------------------
    // Attribute variant factories: ModifierAttribute
    // ----------------------------------------
    fn build_modifier_attribute_virtual_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrModifierAttribute {
        intermediate::IrModifierAttribute::Virtual
    }

    fn build_modifier_attribute_override_specifier_impl(
        &self,
        paths: output::OverridePaths,
    ) -> intermediate::IrModifierAttribute {
        intermediate::IrModifierAttribute::Override(paths)
    }

    // ----------------------------------------
    // Attribute variant factories: StateVariableAttribute
    // ----------------------------------------
    fn build_state_variable_attribute_internal_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrStateVariableAttribute {
        intermediate::IrStateVariableAttribute::Visibility(output::StateVariableVisibility::Internal)
    }

    fn build_state_variable_attribute_private_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrStateVariableAttribute {
        intermediate::IrStateVariableAttribute::Visibility(output::StateVariableVisibility::Private)
    }

    fn build_state_variable_attribute_public_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrStateVariableAttribute {
        intermediate::IrStateVariableAttribute::Visibility(output::StateVariableVisibility::Public)
    }

    fn build_state_variable_attribute_constant_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrStateVariableAttribute {
        intermediate::IrStateVariableAttribute::Mutability(output::StateVariableMutability::Constant)
    }

    fn build_state_variable_attribute_immutable_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrStateVariableAttribute {
        intermediate::IrStateVariableAttribute::Mutability(output::StateVariableMutability::Immutable)
    }

    fn build_state_variable_attribute_transient_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrStateVariableAttribute {
        intermediate::IrStateVariableAttribute::Mutability(output::StateVariableMutability::Transient)
    }

    fn build_state_variable_attribute_override_specifier_impl(
        &self,
        paths: output::OverridePaths,
    ) -> intermediate::IrStateVariableAttribute {
        intermediate::IrStateVariableAttribute::Override(paths)
    }

    // ----------------------------------------
    // Attribute variant factories: FunctionTypeAttribute
    // ----------------------------------------
    fn build_function_type_attribute_external_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionTypeAttribute {
        intermediate::IrFunctionTypeAttribute::Visibility(output::FunctionVisibility::External)
    }

    fn build_function_type_attribute_internal_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionTypeAttribute {
        intermediate::IrFunctionTypeAttribute::Visibility(output::FunctionVisibility::Internal)
    }

    fn build_function_type_attribute_private_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionTypeAttribute {
        intermediate::IrFunctionTypeAttribute::Visibility(output::FunctionVisibility::Private)
    }

    fn build_function_type_attribute_public_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionTypeAttribute {
        intermediate::IrFunctionTypeAttribute::Visibility(output::FunctionVisibility::Public)
    }

    fn build_function_type_attribute_pure_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionTypeAttribute {
        intermediate::IrFunctionTypeAttribute::Mutability(output::FunctionMutability::Pure)
    }

    fn build_function_type_attribute_view_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionTypeAttribute {
        intermediate::IrFunctionTypeAttribute::Mutability(output::FunctionMutability::View)
    }

    fn build_function_type_attribute_payable_keyword_impl(
        &self,
        _range: Range<usize>,
    ) -> intermediate::IrFunctionTypeAttribute {
        intermediate::IrFunctionTypeAttribute::Mutability(output::FunctionMutability::Payable)
    }

    // ----------------------------------------
    // Choice dispatch: ContractMember (removed variants)
    // ----------------------------------------
    fn build_contract_member_constructor_definition_impl(
        &self,
        element: output::FunctionDefinition,
    ) -> output::ContractMember {
        output::ContractMember::FunctionDefinition(element)
    }

    fn build_contract_member_fallback_function_definition_impl(
        &self,
        element: output::FunctionDefinition,
    ) -> output::ContractMember {
        output::ContractMember::FunctionDefinition(element)
    }

    fn build_contract_member_receive_function_definition_impl(
        &self,
        element: output::FunctionDefinition,
    ) -> output::ContractMember {
        output::ContractMember::FunctionDefinition(element)
    }

    fn build_contract_member_modifier_definition_impl(
        &self,
        element: output::FunctionDefinition,
    ) -> output::ContractMember {
        output::ContractMember::FunctionDefinition(element)
    }

    fn build_contract_member_state_variable_definition_impl(
        &self,
        element: output::StateVariableDefinition,
    ) -> output::ContractMember {
        // Constant state variables that are not public become ConstantDefinition.
        // Public constants keep their StateVariableDefinition form because they
        // still generate a getter function.
        if matches!(element.mutability, output::StateVariableMutability::Constant)
            && !matches!(element.visibility, output::StateVariableVisibility::Public)
        {
            let svd = Rc::into_inner(element)
                .expect("freshly built StateVariableDefinition should not yet be shared");
            output::ContractMember::ConstantDefinition(Rc::new(
                output::ConstantDefinitionStruct {
                    type_name: svd.type_name,
                    name: svd.name,
                    visibility: Some(svd.visibility),
                    value: svd.value,
                },
            ))
        } else {
            output::ContractMember::StateVariableDefinition(element)
        }
    }

    // ----------------------------------------
    // Choice dispatch: ImportClause (NamedImport → PathImport)
    // ----------------------------------------
    fn build_import_clause_named_import_impl(
        &self,
        element: output::PathImport,
    ) -> output::ImportClause {
        output::ImportClause::PathImport(element)
    }

    // ----------------------------------------
    // Choice dispatch: ArgumentsDeclaration
    // ----------------------------------------
    fn build_arguments_declaration_positional_arguments_declaration_impl(
        &self,
        element: output::PositionalArguments,
    ) -> output::ArgumentsDeclaration {
        output::ArgumentsDeclaration::PositionalArguments(element)
    }

    fn build_arguments_declaration_named_arguments_declaration_impl(
        &self,
        element: output::NamedArguments,
    ) -> output::ArgumentsDeclaration {
        output::ArgumentsDeclaration::NamedArguments(element)
    }

    // ----------------------------------------
    // Removed sequences: simple passthroughs
    // ----------------------------------------
    fn build_event_parameters_declaration_impl(
        &self,
        _open_paren: Range<usize>,
        parameters: output::Parameters,
        _close_paren: Range<usize>,
    ) -> output::Parameters {
        parameters
    }

    fn build_error_parameters_declaration_impl(
        &self,
        _open_paren: Range<usize>,
        parameters: output::Parameters,
        _close_paren: Range<usize>,
    ) -> output::Parameters {
        parameters
    }

    fn build_positional_arguments_declaration_impl(
        &self,
        _open_paren: Range<usize>,
        arguments: output::PositionalArguments,
        _close_paren: Range<usize>,
    ) -> output::PositionalArguments {
        arguments
    }

    fn build_named_arguments_declaration_impl(
        &self,
        _open_paren: Range<usize>,
        arguments: output::NamedArguments,
        _close_paren: Range<usize>,
    ) -> output::NamedArguments {
        arguments
    }

    fn build_index_access_end_impl(
        &self,
        _colon: Range<usize>,
        end: Option<output::Expression>,
    ) -> Option<output::Expression> {
        end
    }

    fn build_override_specifier_impl(
        &self,
        _override_keyword: Range<usize>,
        overridden: Option<output::OverridePaths>,
    ) -> output::OverridePaths {
        overridden.unwrap_or_default()
    }

    // ----------------------------------------
    // Removed sequences: parameter unification
    // ----------------------------------------
    fn build_event_parameter_impl(
        &self,
        type_name: output::TypeName,
        indexed_keyword: Option<Range<usize>>,
        name: Option<output::Identifier>,
    ) -> output::Parameter {
        Rc::new(output::ParameterStruct {
            type_name,
            storage_location: None,
            name,
            indexed: indexed_keyword.is_some(),
        })
    }

    fn build_error_parameter_impl(
        &self,
        type_name: output::TypeName,
        name: Option<output::Identifier>,
    ) -> output::Parameter {
        Rc::new(output::ParameterStruct {
            type_name,
            storage_location: None,
            name,
            indexed: false,
        })
    }

    fn build_mapping_key_impl(
        &self,
        key_type: output::TypeName,
        name: Option<output::Identifier>,
    ) -> output::Parameter {
        Rc::new(output::ParameterStruct {
            type_name: key_type,
            storage_location: None,
            name,
            indexed: false,
        })
    }

    fn build_mapping_value_impl(
        &self,
        type_name: output::TypeName,
        name: Option<output::Identifier>,
    ) -> output::Parameter {
        Rc::new(output::ParameterStruct {
            type_name,
            storage_location: None,
            name,
            indexed: false,
        })
    }

    // ----------------------------------------
    // Removed sequence: NamedImport → PathImport
    // ----------------------------------------
    fn build_named_import_impl(
        &self,
        _asterisk: Range<usize>,
        alias: output::Identifier,
        _from_keyword: Range<usize>,
        path: output::StringLiteral,
    ) -> output::PathImport {
        Rc::new(output::PathImportStruct {
            path,
            alias: Some(alias),
        })
    }

    // ----------------------------------------
    // Complex sequence: Parameter (adds indexed=false)
    // ----------------------------------------
    fn build_parameter_impl(
        &self,
        type_name: output::TypeName,
        storage_location: Option<output::StorageLocation>,
        name: Option<output::Identifier>,
    ) -> output::Parameter {
        Rc::new(output::ParameterStruct {
            type_name,
            storage_location,
            name,
            indexed: false,
        })
    }

    // ----------------------------------------
    // Complex sequence: ConstantDefinition
    // ----------------------------------------
    fn build_constant_definition_impl(
        &self,
        type_name: output::TypeName,
        _constant_keyword: Range<usize>,
        name: output::Identifier,
        _equal: Range<usize>,
        value: output::Expression,
        _semicolon: Range<usize>,
    ) -> output::ConstantDefinition {
        Rc::new(output::ConstantDefinitionStruct {
            type_name,
            name,
            visibility: None,
            value: Some(value),
        })
    }

    // ----------------------------------------
    // Complex sequence: ContractDefinition
    // ----------------------------------------
    fn build_contract_definition_impl(
        &self,
        abstract_keyword: Option<Range<usize>>,
        _contract_keyword: Range<usize>,
        name: output::Identifier,
        specifiers: Vec<intermediate::IrContractSpecifier>,
        _open_brace: Range<usize>,
        members: output::ContractMembers,
        _close_brace: Range<usize>,
    ) -> output::ContractDefinition {
        let mut inheritance_types: output::InheritanceTypes = Vec::new();
        let mut storage_layout: Option<output::Expression> = None;
        for specifier in specifiers {
            match specifier {
                intermediate::IrContractSpecifier::Inheritance(types) => {
                    inheritance_types = types;
                }
                intermediate::IrContractSpecifier::StorageLayout(expr) => {
                    storage_layout = Some(expr);
                }
            }
        }
        Rc::new(output::ContractDefinitionStruct {
            abstract_keyword: abstract_keyword.is_some(),
            name,
            inheritance_types,
            storage_layout,
            members,
        })
    }

    // ----------------------------------------
    // Complex sequence: FunctionDefinition
    // ----------------------------------------
    fn build_function_definition_impl(
        &self,
        _function_keyword: Range<usize>,
        name: (output::FunctionKind, Option<output::Identifier>),
        parameters: output::Parameters,
        attributes: Vec<intermediate::IrFunctionAttribute>,
        returns: Option<output::Parameters>,
        body: Option<output::Block>,
    ) -> output::FunctionDefinition {
        let (kind, name) = name;
        let (visibility, mutability, virtual_keyword, override_specifier, modifier_invocations) =
            fold_function_attributes(attributes);
        Rc::new(output::FunctionDefinitionStruct {
            kind,
            name,
            parameters,
            visibility,
            mutability,
            virtual_keyword,
            override_specifier,
            modifier_invocations,
            returns,
            body,
        })
    }

    // ----------------------------------------
    // Complex sequence: ConstructorDefinition → FunctionDefinition
    // ----------------------------------------
    fn build_constructor_definition_impl(
        &self,
        _constructor_keyword: Range<usize>,
        parameters: output::Parameters,
        attributes: Vec<intermediate::IrConstructorAttribute>,
        body: output::Block,
    ) -> output::FunctionDefinition {
        let mut visibility = output::FunctionVisibility::Public;
        let mut mutability = output::FunctionMutability::NonPayable;
        let mut modifier_invocations: output::ModifierInvocations = Vec::new();
        for attribute in attributes {
            match attribute {
                intermediate::IrConstructorAttribute::Visibility(v) => visibility = v,
                intermediate::IrConstructorAttribute::Mutability(m) => mutability = m,
                intermediate::IrConstructorAttribute::Modifier(m) => modifier_invocations.push(m),
            }
        }
        Rc::new(output::FunctionDefinitionStruct {
            kind: output::FunctionKind::Constructor,
            name: None,
            parameters,
            visibility,
            mutability,
            virtual_keyword: false,
            override_specifier: None,
            modifier_invocations,
            returns: None,
            body: Some(body),
        })
    }

    // ----------------------------------------
    // Complex sequence: FallbackFunctionDefinition → FunctionDefinition
    // ----------------------------------------
    fn build_fallback_function_definition_impl(
        &self,
        _fallback_keyword: Range<usize>,
        parameters: output::Parameters,
        attributes: Vec<intermediate::IrFallbackFunctionAttribute>,
        returns: Option<output::Parameters>,
        body: Option<output::Block>,
    ) -> output::FunctionDefinition {
        // TODO(validation): fallback functions must have external visibility
        let mut visibility = output::FunctionVisibility::External;
        let mut mutability = output::FunctionMutability::NonPayable;
        let mut virtual_keyword = false;
        let mut override_specifier: Option<output::OverridePaths> = None;
        let mut modifier_invocations: output::ModifierInvocations = Vec::new();
        for attribute in attributes {
            match attribute {
                intermediate::IrFallbackFunctionAttribute::Visibility(v) => visibility = v,
                intermediate::IrFallbackFunctionAttribute::Mutability(m) => mutability = m,
                intermediate::IrFallbackFunctionAttribute::Virtual => virtual_keyword = true,
                intermediate::IrFallbackFunctionAttribute::Override(paths) => {
                    override_specifier = Some(paths);
                }
                intermediate::IrFallbackFunctionAttribute::Modifier(m) => modifier_invocations.push(m),
            }
        }
        Rc::new(output::FunctionDefinitionStruct {
            kind: output::FunctionKind::Fallback,
            name: None,
            parameters,
            visibility,
            mutability,
            virtual_keyword,
            override_specifier,
            modifier_invocations,
            returns,
            body,
        })
    }

    // ----------------------------------------
    // Complex sequence: ReceiveFunctionDefinition → FunctionDefinition
    // ----------------------------------------
    fn build_receive_function_definition_impl(
        &self,
        _receive_keyword: Range<usize>,
        parameters: output::Parameters,
        attributes: Vec<intermediate::IrReceiveFunctionAttribute>,
        body: Option<output::Block>,
    ) -> output::FunctionDefinition {
        // TODO(validation): receive functions must be external + payable
        let mut visibility = output::FunctionVisibility::External;
        let mut mutability = output::FunctionMutability::Payable;
        let mut virtual_keyword = false;
        let mut override_specifier: Option<output::OverridePaths> = None;
        let mut modifier_invocations: output::ModifierInvocations = Vec::new();
        for attribute in attributes {
            match attribute {
                intermediate::IrReceiveFunctionAttribute::Visibility(v) => visibility = v,
                intermediate::IrReceiveFunctionAttribute::Mutability(m) => mutability = m,
                intermediate::IrReceiveFunctionAttribute::Virtual => virtual_keyword = true,
                intermediate::IrReceiveFunctionAttribute::Override(paths) => {
                    override_specifier = Some(paths);
                }
                intermediate::IrReceiveFunctionAttribute::Modifier(m) => modifier_invocations.push(m),
            }
        }
        Rc::new(output::FunctionDefinitionStruct {
            kind: output::FunctionKind::Receive,
            name: None,
            parameters,
            visibility,
            mutability,
            virtual_keyword,
            override_specifier,
            modifier_invocations,
            returns: None,
            body,
        })
    }

    // ----------------------------------------
    // Complex sequence: ModifierDefinition → FunctionDefinition
    // ----------------------------------------
    fn build_modifier_definition_impl(
        &self,
        _modifier_keyword: Range<usize>,
        name: output::Identifier,
        parameters: Option<output::Parameters>,
        attributes: Vec<intermediate::IrModifierAttribute>,
        body: Option<output::Block>,
    ) -> output::FunctionDefinition {
        let mut virtual_keyword = false;
        let mut override_specifier: Option<output::OverridePaths> = None;
        for attribute in attributes {
            match attribute {
                intermediate::IrModifierAttribute::Virtual => virtual_keyword = true,
                intermediate::IrModifierAttribute::Override(paths) => {
                    override_specifier = Some(paths);
                }
            }
        }
        Rc::new(output::FunctionDefinitionStruct {
            kind: output::FunctionKind::Modifier,
            name: Some(name),
            parameters: parameters.unwrap_or_default(),
            visibility: output::FunctionVisibility::Internal,
            mutability: output::FunctionMutability::NonPayable,
            virtual_keyword,
            override_specifier,
            modifier_invocations: Vec::new(),
            returns: None,
            body,
        })
    }

    // ----------------------------------------
    // Complex sequence: EventDefinition
    // ----------------------------------------
    fn build_event_definition_impl(
        &self,
        _event_keyword: Range<usize>,
        name: output::Identifier,
        parameters: output::Parameters,
        anonymous_keyword: Option<Range<usize>>,
        _semicolon: Range<usize>,
    ) -> output::EventDefinition {
        Rc::new(output::EventDefinitionStruct {
            name,
            anonymous_keyword: anonymous_keyword.is_some(),
            parameters,
        })
    }

    // ----------------------------------------
    // Complex sequence: ErrorDefinition
    // ----------------------------------------
    fn build_error_definition_impl(
        &self,
        _error_keyword: Range<usize>,
        name: output::Identifier,
        parameters: output::Parameters,
        _semicolon: Range<usize>,
    ) -> output::ErrorDefinition {
        Rc::new(output::ErrorDefinitionStruct { name, parameters })
    }

    // ----------------------------------------
    // Complex sequence: StateVariableDefinition (with attribute folding)
    // ----------------------------------------
    fn build_state_variable_definition_impl(
        &self,
        type_name: output::TypeName,
        attributes: Vec<intermediate::IrStateVariableAttribute>,
        name: output::Identifier,
        value: Option<output::Expression>,
        _semicolon: Range<usize>,
    ) -> output::StateVariableDefinition {
        let (visibility, mutability, override_specifier) = fold_state_variable_attributes(attributes);
        Rc::new(output::StateVariableDefinitionStruct {
            type_name,
            name,
            value,
            visibility,
            mutability,
            override_specifier,
        })
    }

    // ----------------------------------------
    // Complex sequence: MappingType
    // ----------------------------------------
    fn build_mapping_type_impl(
        &self,
        _mapping_keyword: Range<usize>,
        _open_paren: Range<usize>,
        key_type: output::Parameter,
        _equal_greater_than: Range<usize>,
        value_type: output::Parameter,
        _close_paren: Range<usize>,
    ) -> output::MappingType {
        Rc::new(output::MappingTypeStruct {
            key_type,
            value_type,
        })
    }

    // ----------------------------------------
    // Complex sequence: IndexAccessExpression (flatten IndexAccessEnd)
    // ----------------------------------------
    fn build_index_access_expression_impl(
        &self,
        operand: output::Expression,
        _open_bracket: Range<usize>,
        start: Option<output::Expression>,
        end: Option<Option<output::Expression>>,
        _close_bracket: Range<usize>,
    ) -> output::IndexAccessExpression {
        Rc::new(output::IndexAccessExpressionStruct {
            operand,
            start,
            end: end.flatten(),
        })
    }

    // ----------------------------------------
    // Complex sequence: FunctionType → IrFunctionType wrapper
    // ----------------------------------------
    fn build_function_type_impl(
        &self,
        _function_keyword: Range<usize>,
        parameters: output::Parameters,
        attributes: Vec<intermediate::IrFunctionTypeAttribute>,
        returns: Option<output::Parameters>,
    ) -> intermediate::IrFunctionType {
        intermediate::IrFunctionType {
            parameters,
            attributes,
            returns,
        }
    }

    // ----------------------------------------
    // Special helpers
    // ----------------------------------------
    fn build_type_name_from_index_access_path_impl(
        &self,
        index_access_path: parser_helpers::IndexAccessPath<Self>,
    ) -> output::TypeName {
        let parser_helpers::IndexAccessPath { path, indices } = index_access_path;
        let mut type_name = match path {
            parser_helpers::Path::IdentifierPath(ip) => output::TypeName::IdentifierPath(ip),
            parser_helpers::Path::ElementaryType(et) => output::TypeName::ElementaryType(et),
        };
        for index in indices {
            assert!(
                index.end.is_none(),
                "Slicing is not supported in type names"
            );
            let array = Rc::new(output::ArrayTypeNameStruct {
                operand: type_name,
                index: index.start,
            });
            type_name = output::TypeName::ArrayTypeName(array);
        }
        type_name
    }

    fn build_expression_from_index_access_path_impl(
        &self,
        index_access_path: parser_helpers::IndexAccessPath<Self>,
    ) -> output::Expression {
        let parser_helpers::IndexAccessPath { path, indices } = index_access_path;
        let mut expression = match path {
            parser_helpers::Path::IdentifierPath(ip) => {
                self.build_expression_from_identifier_path_impl(ip)
            }
            parser_helpers::Path::ElementaryType(et) => output::Expression::ElementaryType(et),
        };
        for index in indices {
            let array = Rc::new(output::IndexAccessExpressionStruct {
                operand: expression,
                start: index.start,
                end: index.end.flatten(),
            });
            expression = output::Expression::IndexAccessExpression(array);
        }
        expression
    }

    fn build_expression_from_identifier_path_impl(
        &self,
        identifier_path: output::IdentifierPath,
    ) -> output::Expression {
        identifier_path
            .into_iter()
            .fold(None, |acc, id| match acc {
                None => Some(output::Expression::Identifier(id)),
                Some(acc) => Some(output::Expression::MemberAccessExpression(Rc::new(
                    output::MemberAccessExpressionStruct {
                        operand: acc,
                        member: id,
                    },
                ))),
            })
            .expect("IdentifierPath should have at least one element")
    }

    fn extract_extra_attributes_impl(
        &self,
        fun_type: intermediate::IrFunctionType,
    ) -> (
        intermediate::IrFunctionType,
        Vec<intermediate::IrStateVariableAttribute>,
    ) {
        let intermediate::IrFunctionType {
            parameters,
            attributes,
            returns,
        } = fun_type;

        let mut kept: Vec<intermediate::IrFunctionTypeAttribute> =
            Vec::with_capacity(attributes.len());
        let mut extracted: Vec<intermediate::IrStateVariableAttribute> = Vec::new();

        let mut seen_internal = false;
        let mut seen_private = false;
        let mut seen_public = false;
        let mut duplicate_found = false;

        for attr in attributes {
            if duplicate_found {
                if let Some(sv_attr) = function_type_attr_to_state_variable(&attr) {
                    extracted.push(sv_attr);
                } else {
                    kept.push(attr);
                }
                continue;
            }

            let seen_flag = match &attr {
                intermediate::IrFunctionTypeAttribute::Visibility(v) => match v {
                    output::FunctionVisibility::Internal => Some(&mut seen_internal),
                    output::FunctionVisibility::Private => Some(&mut seen_private),
                    output::FunctionVisibility::Public => Some(&mut seen_public),
                    _ => None,
                },
                _ => None,
            };

            match seen_flag {
                Some(flag) => {
                    if *flag {
                        duplicate_found = true;
                        if let Some(sv_attr) = function_type_attr_to_state_variable(&attr) {
                            extracted.push(sv_attr);
                        } else {
                            kept.push(attr);
                        }
                    } else {
                        *flag = true;
                        kept.push(attr);
                    }
                }
                None => kept.push(attr),
            }
        }

        let new_fun_type = intermediate::IrFunctionType {
            parameters,
            attributes: kept,
            returns,
        };
        (new_fun_type, extracted)
    }
}

// ============================================
// === Private Fold Helpers ===
// ============================================

fn fold_function_attributes(
    attributes: Vec<intermediate::IrFunctionAttribute>,
) -> (
    output::FunctionVisibility,
    output::FunctionMutability,
    bool,
    Option<output::OverridePaths>,
    output::ModifierInvocations,
) {
    // TODO(validation): duplicate visibility/mutability keywords
    let mut visibility = output::FunctionVisibility::Internal;
    let mut mutability = output::FunctionMutability::NonPayable;
    let mut virtual_keyword = false;
    let mut override_specifier: Option<output::OverridePaths> = None;
    let mut modifier_invocations: output::ModifierInvocations = Vec::new();
    for attribute in attributes {
        match attribute {
            intermediate::IrFunctionAttribute::Visibility(v) => visibility = v,
            intermediate::IrFunctionAttribute::Mutability(m) => mutability = m,
            intermediate::IrFunctionAttribute::Virtual => virtual_keyword = true,
            intermediate::IrFunctionAttribute::Override(paths) => override_specifier = Some(paths),
            intermediate::IrFunctionAttribute::Modifier(m) => modifier_invocations.push(m),
        }
    }
    (
        visibility,
        mutability,
        virtual_keyword,
        override_specifier,
        modifier_invocations,
    )
}

fn fold_state_variable_attributes(
    attributes: Vec<intermediate::IrStateVariableAttribute>,
) -> (
    output::StateVariableVisibility,
    output::StateVariableMutability,
    Option<output::OverridePaths>,
) {
    // TODO(validation): duplicate visibility/mutability keywords
    let mut visibility = output::StateVariableVisibility::Internal;
    let mut mutability = output::StateVariableMutability::Mutable;
    let mut override_specifier: Option<output::OverridePaths> = None;
    for attribute in attributes {
        match attribute {
            intermediate::IrStateVariableAttribute::Visibility(v) => visibility = v,
            intermediate::IrStateVariableAttribute::Mutability(m) => mutability = m,
            intermediate::IrStateVariableAttribute::Override(paths) => override_specifier = Some(paths),
        }
    }
    (visibility, mutability, override_specifier)
}

fn function_type_attr_to_state_variable(
    attr: &intermediate::IrFunctionTypeAttribute,
) -> Option<intermediate::IrStateVariableAttribute> {
    match attr {
        intermediate::IrFunctionTypeAttribute::Visibility(v) => match v {
            output::FunctionVisibility::Internal => Some(
                intermediate::IrStateVariableAttribute::Visibility(
                    output::StateVariableVisibility::Internal,
                ),
            ),
            output::FunctionVisibility::Private => Some(
                intermediate::IrStateVariableAttribute::Visibility(
                    output::StateVariableVisibility::Private,
                ),
            ),
            output::FunctionVisibility::Public => Some(
                intermediate::IrStateVariableAttribute::Visibility(
                    output::StateVariableVisibility::Public,
                ),
            ),
            _ => None,
        },
        _ => None,
    }
}

// The generated `impl ParserConsumer for IrConsumer` lives in this file.
// The template delegates complex factory methods to the `build_*_impl`
// methods defined above.
#[path = "consumer_impl.generated.rs"]
mod consumer_impl;
