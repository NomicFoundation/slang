// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::{EdgeLabel, NodeKind, NonterminalKind, SyntaxNode, TerminalKind};

//
// Sequences:
//

pub fn build_source_unit(node: Rc<SyntaxNode>) -> Option<SourceUnit> {
    assert_nonterminal_kind(&node, NonterminalKind::SourceUnit);
    let mut helper = ChildrenHelper::new(&node);
    let members = build_source_unit_members(helper.accept_label(EdgeLabel::Members)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(SourceUnitStruct { node, members }))
}

pub fn build_pragma_directive(node: Rc<SyntaxNode>) -> Option<PragmaDirective> {
    assert_nonterminal_kind(&node, NonterminalKind::PragmaDirective);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::PragmaKeyword)?;
    let pragma = build_pragma(helper.accept_label(EdgeLabel::Pragma)?)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(PragmaDirectiveStruct { node, pragma }))
}

pub fn build_abicoder_pragma(node: Rc<SyntaxNode>) -> Option<AbicoderPragma> {
    assert_nonterminal_kind(&node, NonterminalKind::AbicoderPragma);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::AbicoderKeyword)?;
    let version = build_abicoder_version(helper.accept_label(EdgeLabel::Version)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(AbicoderPragmaStruct { node, version }))
}

pub fn build_experimental_pragma(node: Rc<SyntaxNode>) -> Option<ExperimentalPragma> {
    assert_nonterminal_kind(&node, NonterminalKind::ExperimentalPragma);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ExperimentalKeyword)?;
    let feature = build_experimental_feature(helper.accept_label(EdgeLabel::Feature)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ExperimentalPragmaStruct { node, feature }))
}

pub fn build_version_pragma(node: Rc<SyntaxNode>) -> Option<VersionPragma> {
    assert_nonterminal_kind(&node, NonterminalKind::VersionPragma);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::SolidityKeyword)?;
    let sets = build_version_expression_sets(helper.accept_label(EdgeLabel::Sets)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(VersionPragmaStruct { node, sets }))
}

pub fn build_version_range(node: Rc<SyntaxNode>) -> Option<VersionRange> {
    assert_nonterminal_kind(&node, NonterminalKind::VersionRange);
    let mut helper = ChildrenHelper::new(&node);
    let start = build_version_literal(helper.accept_label(EdgeLabel::Start)?)?;
    helper.skip_label(EdgeLabel::Minus)?;
    let end = build_version_literal(helper.accept_label(EdgeLabel::End)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(VersionRangeStruct { node, start, end }))
}

pub fn build_version_term(node: Rc<SyntaxNode>) -> Option<VersionTerm> {
    assert_nonterminal_kind(&node, NonterminalKind::VersionTerm);
    let mut helper = ChildrenHelper::new(&node);
    let operator = helper
        .accept_label(EdgeLabel::Operator)
        .and_then(build_version_operator);
    let literal = build_version_literal(helper.accept_label(EdgeLabel::Literal)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(VersionTermStruct {
        node,
        operator,
        literal,
    }))
}

pub fn build_import_directive(node: Rc<SyntaxNode>) -> Option<ImportDirective> {
    assert_nonterminal_kind(&node, NonterminalKind::ImportDirective);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ImportKeyword)?;
    let clause = build_import_clause(helper.accept_label(EdgeLabel::Clause)?)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ImportDirectiveStruct { node, clause }))
}

pub fn build_path_import(node: Rc<SyntaxNode>) -> Option<PathImport> {
    assert_nonterminal_kind(&node, NonterminalKind::PathImport);
    let mut helper = ChildrenHelper::new(&node);
    let path = build_string_literal(helper.accept_label(EdgeLabel::Path)?)?;
    let alias = helper
        .accept_label(EdgeLabel::Alias)
        .and_then(build_import_alias);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(PathImportStruct { node, path, alias }))
}

pub fn build_named_import(node: Rc<SyntaxNode>) -> Option<NamedImport> {
    assert_nonterminal_kind(&node, NonterminalKind::NamedImport);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::Asterisk)?;
    let alias = build_import_alias(helper.accept_label(EdgeLabel::Alias)?)?;
    helper.skip_label(EdgeLabel::FromKeyword)?;
    let path = build_string_literal(helper.accept_label(EdgeLabel::Path)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(NamedImportStruct { node, alias, path }))
}

pub fn build_import_deconstruction(node: Rc<SyntaxNode>) -> Option<ImportDeconstruction> {
    assert_nonterminal_kind(&node, NonterminalKind::ImportDeconstruction);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let symbols = build_import_deconstruction_symbols(helper.accept_label(EdgeLabel::Symbols)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    helper.skip_label(EdgeLabel::FromKeyword)?;
    let path = build_string_literal(helper.accept_label(EdgeLabel::Path)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ImportDeconstructionStruct {
        node,
        symbols,
        path,
    }))
}

pub fn build_import_deconstruction_symbol(
    node: Rc<SyntaxNode>,
) -> Option<ImportDeconstructionSymbol> {
    assert_nonterminal_kind(&node, NonterminalKind::ImportDeconstructionSymbol);
    let mut helper = ChildrenHelper::new(&node);
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    let alias = helper
        .accept_label(EdgeLabel::Alias)
        .and_then(build_import_alias);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ImportDeconstructionSymbolStruct {
        node,
        name,
        alias,
    }))
}

pub fn build_import_alias(node: Rc<SyntaxNode>) -> Option<ImportAlias> {
    assert_nonterminal_kind(&node, NonterminalKind::ImportAlias);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::AsKeyword)?;
    let identifier = terminal_node_cloned(helper.accept_label(EdgeLabel::Identifier)?);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ImportAliasStruct { node, identifier }))
}

pub fn build_using_directive(node: Rc<SyntaxNode>) -> Option<UsingDirective> {
    assert_nonterminal_kind(&node, NonterminalKind::UsingDirective);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::UsingKeyword)?;
    let clause = build_using_clause(helper.accept_label(EdgeLabel::Clause)?)?;
    helper.skip_label(EdgeLabel::ForKeyword)?;
    let target = build_using_target(helper.accept_label(EdgeLabel::Target)?)?;
    let global_keyword = helper.accept_label(EdgeLabel::GlobalKeyword).is_some();
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(UsingDirectiveStruct {
        node,
        clause,
        target,
        global_keyword,
    }))
}

pub fn build_using_deconstruction(node: Rc<SyntaxNode>) -> Option<UsingDeconstruction> {
    assert_nonterminal_kind(&node, NonterminalKind::UsingDeconstruction);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let symbols = build_using_deconstruction_symbols(helper.accept_label(EdgeLabel::Symbols)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(UsingDeconstructionStruct { node, symbols }))
}

pub fn build_using_deconstruction_symbol(
    node: Rc<SyntaxNode>,
) -> Option<UsingDeconstructionSymbol> {
    assert_nonterminal_kind(&node, NonterminalKind::UsingDeconstructionSymbol);
    let mut helper = ChildrenHelper::new(&node);
    let name = build_identifier_path(helper.accept_label(EdgeLabel::Name)?)?;
    let alias = helper
        .accept_label(EdgeLabel::Alias)
        .and_then(build_using_alias);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(UsingDeconstructionSymbolStruct {
        node,
        name,
        alias,
    }))
}

pub fn build_using_alias(node: Rc<SyntaxNode>) -> Option<UsingAlias> {
    assert_nonterminal_kind(&node, NonterminalKind::UsingAlias);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::AsKeyword)?;
    let operator = build_using_operator(helper.accept_label(EdgeLabel::Operator)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(UsingAliasStruct { node, operator }))
}

pub fn build_contract_definition(node: Rc<SyntaxNode>) -> Option<ContractDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::ContractDefinition);
    let mut helper = ChildrenHelper::new(&node);
    let abstract_keyword = helper.accept_label(EdgeLabel::AbstractKeyword).is_some();
    helper.skip_label(EdgeLabel::ContractKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    let specifiers = build_contract_specifiers(helper.accept_label(EdgeLabel::Specifiers)?)?;
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let members = build_contract_members(helper.accept_label(EdgeLabel::Members)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ContractDefinitionStruct {
        node,
        abstract_keyword,
        name,
        specifiers,
        members,
    }))
}

pub fn build_inheritance_specifier(node: Rc<SyntaxNode>) -> Option<InheritanceSpecifier> {
    assert_nonterminal_kind(&node, NonterminalKind::InheritanceSpecifier);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::IsKeyword)?;
    let types = build_inheritance_types(helper.accept_label(EdgeLabel::Types)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(InheritanceSpecifierStruct { node, types }))
}

pub fn build_inheritance_type(node: Rc<SyntaxNode>) -> Option<InheritanceType> {
    assert_nonterminal_kind(&node, NonterminalKind::InheritanceType);
    let mut helper = ChildrenHelper::new(&node);
    let type_name = build_identifier_path(helper.accept_label(EdgeLabel::TypeName)?)?;
    let arguments = helper
        .accept_label(EdgeLabel::Arguments)
        .and_then(build_arguments_declaration);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(InheritanceTypeStruct {
        node,
        type_name,
        arguments,
    }))
}

pub fn build_storage_layout_specifier(node: Rc<SyntaxNode>) -> Option<StorageLayoutSpecifier> {
    assert_nonterminal_kind(&node, NonterminalKind::StorageLayoutSpecifier);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::LayoutKeyword)?;
    helper.skip_label(EdgeLabel::AtKeyword)?;
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(StorageLayoutSpecifierStruct { node, expression }))
}

pub fn build_interface_definition(node: Rc<SyntaxNode>) -> Option<InterfaceDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::InterfaceDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::InterfaceKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    let inheritance = helper
        .accept_label(EdgeLabel::Inheritance)
        .and_then(build_inheritance_specifier);
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let members = build_interface_members(helper.accept_label(EdgeLabel::Members)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(InterfaceDefinitionStruct {
        node,
        name,
        inheritance,
        members,
    }))
}

pub fn build_library_definition(node: Rc<SyntaxNode>) -> Option<LibraryDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::LibraryDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::LibraryKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let members = build_library_members(helper.accept_label(EdgeLabel::Members)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(LibraryDefinitionStruct {
        node,
        name,
        members,
    }))
}

pub fn build_struct_definition(node: Rc<SyntaxNode>) -> Option<StructDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::StructDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::StructKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let members = build_struct_members(helper.accept_label(EdgeLabel::Members)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(StructDefinitionStruct {
        node,
        name,
        members,
    }))
}

pub fn build_struct_member(node: Rc<SyntaxNode>) -> Option<StructMember> {
    assert_nonterminal_kind(&node, NonterminalKind::StructMember);
    let mut helper = ChildrenHelper::new(&node);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(StructMemberStruct {
        node,
        type_name,
        name,
    }))
}

pub fn build_enum_definition(node: Rc<SyntaxNode>) -> Option<EnumDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::EnumDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::EnumKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let members = build_enum_members(helper.accept_label(EdgeLabel::Members)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(EnumDefinitionStruct {
        node,
        name,
        members,
    }))
}

pub fn build_constant_definition(node: Rc<SyntaxNode>) -> Option<ConstantDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::ConstantDefinition);
    let mut helper = ChildrenHelper::new(&node);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    helper.skip_label(EdgeLabel::ConstantKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    helper.skip_label(EdgeLabel::Equal)?;
    let value = build_expression(helper.accept_label(EdgeLabel::Value)?)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ConstantDefinitionStruct {
        node,
        type_name,
        name,
        value,
    }))
}

pub fn build_state_variable_definition(node: Rc<SyntaxNode>) -> Option<StateVariableDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::StateVariableDefinition);
    let mut helper = ChildrenHelper::new(&node);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let attributes = build_state_variable_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    let value = helper
        .accept_label(EdgeLabel::Value)
        .and_then(build_state_variable_definition_value);
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(StateVariableDefinitionStruct {
        node,
        type_name,
        attributes,
        name,
        value,
    }))
}

pub fn build_state_variable_definition_value(
    node: Rc<SyntaxNode>,
) -> Option<StateVariableDefinitionValue> {
    assert_nonterminal_kind(&node, NonterminalKind::StateVariableDefinitionValue);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::Equal)?;
    let value = build_expression(helper.accept_label(EdgeLabel::Value)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(StateVariableDefinitionValueStruct { node, value }))
}

pub fn build_function_definition(node: Rc<SyntaxNode>) -> Option<FunctionDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::FunctionDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::FunctionKeyword)?;
    let name = build_function_name(helper.accept_label(EdgeLabel::Name)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes = build_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let returns = helper
        .accept_label(EdgeLabel::Returns)
        .and_then(build_returns_declaration);
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(FunctionDefinitionStruct {
        node,
        name,
        parameters,
        attributes,
        returns,
        body,
    }))
}

pub fn build_parameters_declaration(node: Rc<SyntaxNode>) -> Option<ParametersDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::ParametersDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenParen)?;
    let parameters = build_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ParametersDeclarationStruct { node, parameters }))
}

pub fn build_parameter(node: Rc<SyntaxNode>) -> Option<Parameter> {
    assert_nonterminal_kind(&node, NonterminalKind::Parameter);
    let mut helper = ChildrenHelper::new(&node);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let storage_location = helper
        .accept_label(EdgeLabel::StorageLocation)
        .and_then(build_storage_location);
    let name = helper
        .accept_label(EdgeLabel::Name)
        .map(terminal_node_cloned);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ParameterStruct {
        node,
        type_name,
        storage_location,
        name,
    }))
}

pub fn build_override_specifier(node: Rc<SyntaxNode>) -> Option<OverrideSpecifier> {
    assert_nonterminal_kind(&node, NonterminalKind::OverrideSpecifier);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OverrideKeyword)?;
    let overridden = helper
        .accept_label(EdgeLabel::Overridden)
        .and_then(build_override_paths_declaration);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(OverrideSpecifierStruct { node, overridden }))
}

pub fn build_override_paths_declaration(node: Rc<SyntaxNode>) -> Option<OverridePathsDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::OverridePathsDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenParen)?;
    let paths = build_override_paths(helper.accept_label(EdgeLabel::Paths)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(OverridePathsDeclarationStruct { node, paths }))
}

pub fn build_returns_declaration(node: Rc<SyntaxNode>) -> Option<ReturnsDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::ReturnsDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ReturnsKeyword)?;
    let variables = build_parameters_declaration(helper.accept_label(EdgeLabel::Variables)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ReturnsDeclarationStruct { node, variables }))
}

pub fn build_constructor_definition(node: Rc<SyntaxNode>) -> Option<ConstructorDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::ConstructorDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ConstructorKeyword)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes = build_constructor_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_block(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ConstructorDefinitionStruct {
        node,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_unnamed_function_definition(
    node: Rc<SyntaxNode>,
) -> Option<UnnamedFunctionDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::UnnamedFunctionDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::FunctionKeyword)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes =
        build_unnamed_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(UnnamedFunctionDefinitionStruct {
        node,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_fallback_function_definition(
    node: Rc<SyntaxNode>,
) -> Option<FallbackFunctionDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::FallbackFunctionDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::FallbackKeyword)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes =
        build_fallback_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let returns = helper
        .accept_label(EdgeLabel::Returns)
        .and_then(build_returns_declaration);
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(FallbackFunctionDefinitionStruct {
        node,
        parameters,
        attributes,
        returns,
        body,
    }))
}

pub fn build_receive_function_definition(
    node: Rc<SyntaxNode>,
) -> Option<ReceiveFunctionDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::ReceiveFunctionDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ReceiveKeyword)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes =
        build_receive_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ReceiveFunctionDefinitionStruct {
        node,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_modifier_definition(node: Rc<SyntaxNode>) -> Option<ModifierDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::ModifierDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ModifierKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    let parameters = helper
        .accept_label(EdgeLabel::Parameters)
        .and_then(build_parameters_declaration);
    let attributes = build_modifier_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ModifierDefinitionStruct {
        node,
        name,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_modifier_invocation(node: Rc<SyntaxNode>) -> Option<ModifierInvocation> {
    assert_nonterminal_kind(&node, NonterminalKind::ModifierInvocation);
    let mut helper = ChildrenHelper::new(&node);
    let name = build_identifier_path(helper.accept_label(EdgeLabel::Name)?)?;
    let arguments = helper
        .accept_label(EdgeLabel::Arguments)
        .and_then(build_arguments_declaration);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ModifierInvocationStruct {
        node,
        name,
        arguments,
    }))
}

pub fn build_event_definition(node: Rc<SyntaxNode>) -> Option<EventDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::EventDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::EventKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    let parameters =
        build_event_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let anonymous_keyword = helper.accept_label(EdgeLabel::AnonymousKeyword).is_some();
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(EventDefinitionStruct {
        node,
        name,
        parameters,
        anonymous_keyword,
    }))
}

pub fn build_event_parameters_declaration(
    node: Rc<SyntaxNode>,
) -> Option<EventParametersDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::EventParametersDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenParen)?;
    let parameters = build_event_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(EventParametersDeclarationStruct {
        node,
        parameters,
    }))
}

pub fn build_event_parameter(node: Rc<SyntaxNode>) -> Option<EventParameter> {
    assert_nonterminal_kind(&node, NonterminalKind::EventParameter);
    let mut helper = ChildrenHelper::new(&node);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let indexed_keyword = helper.accept_label(EdgeLabel::IndexedKeyword).is_some();
    let name = helper
        .accept_label(EdgeLabel::Name)
        .map(terminal_node_cloned);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(EventParameterStruct {
        node,
        type_name,
        indexed_keyword,
        name,
    }))
}

pub fn build_user_defined_value_type_definition(
    node: Rc<SyntaxNode>,
) -> Option<UserDefinedValueTypeDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::UserDefinedValueTypeDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::TypeKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    helper.skip_label(EdgeLabel::IsKeyword)?;
    let value_type = build_elementary_type(helper.accept_label(EdgeLabel::ValueType)?)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(UserDefinedValueTypeDefinitionStruct {
        node,
        name,
        value_type,
    }))
}

pub fn build_error_definition(node: Rc<SyntaxNode>) -> Option<ErrorDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::ErrorDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ErrorKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    let members = build_error_parameters_declaration(helper.accept_label(EdgeLabel::Members)?)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ErrorDefinitionStruct {
        node,
        name,
        members,
    }))
}

pub fn build_error_parameters_declaration(
    node: Rc<SyntaxNode>,
) -> Option<ErrorParametersDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::ErrorParametersDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenParen)?;
    let parameters = build_error_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ErrorParametersDeclarationStruct {
        node,
        parameters,
    }))
}

pub fn build_error_parameter(node: Rc<SyntaxNode>) -> Option<ErrorParameter> {
    assert_nonterminal_kind(&node, NonterminalKind::ErrorParameter);
    let mut helper = ChildrenHelper::new(&node);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let name = helper
        .accept_label(EdgeLabel::Name)
        .map(terminal_node_cloned);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ErrorParameterStruct {
        node,
        type_name,
        name,
    }))
}

pub fn build_array_type_name(node: Rc<SyntaxNode>) -> Option<ArrayTypeName> {
    assert_nonterminal_kind(&node, NonterminalKind::ArrayTypeName);
    let mut helper = ChildrenHelper::new(&node);
    let operand = build_type_name(helper.accept_label(EdgeLabel::Operand)?)?;
    helper.skip_label(EdgeLabel::OpenBracket)?;
    let index = helper
        .accept_label(EdgeLabel::Index)
        .and_then(build_expression);
    helper.skip_label(EdgeLabel::CloseBracket)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ArrayTypeNameStruct {
        node,
        operand,
        index,
    }))
}

pub fn build_function_type(node: Rc<SyntaxNode>) -> Option<FunctionType> {
    assert_nonterminal_kind(&node, NonterminalKind::FunctionType);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::FunctionKeyword)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes = build_function_type_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let returns = helper
        .accept_label(EdgeLabel::Returns)
        .and_then(build_returns_declaration);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(FunctionTypeStruct {
        node,
        parameters,
        attributes,
        returns,
    }))
}

pub fn build_mapping_type(node: Rc<SyntaxNode>) -> Option<MappingType> {
    assert_nonterminal_kind(&node, NonterminalKind::MappingType);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::MappingKeyword)?;
    helper.skip_label(EdgeLabel::OpenParen)?;
    let key_type = build_mapping_key(helper.accept_label(EdgeLabel::KeyType)?)?;
    helper.skip_label(EdgeLabel::EqualGreaterThan)?;
    let value_type = build_mapping_value(helper.accept_label(EdgeLabel::ValueType)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(MappingTypeStruct {
        node,
        key_type,
        value_type,
    }))
}

pub fn build_mapping_key(node: Rc<SyntaxNode>) -> Option<MappingKey> {
    assert_nonterminal_kind(&node, NonterminalKind::MappingKey);
    let mut helper = ChildrenHelper::new(&node);
    let key_type = build_mapping_key_type(helper.accept_label(EdgeLabel::KeyType)?)?;
    let name = helper
        .accept_label(EdgeLabel::Name)
        .map(terminal_node_cloned);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(MappingKeyStruct {
        node,
        key_type,
        name,
    }))
}

pub fn build_mapping_value(node: Rc<SyntaxNode>) -> Option<MappingValue> {
    assert_nonterminal_kind(&node, NonterminalKind::MappingValue);
    let mut helper = ChildrenHelper::new(&node);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let name = helper
        .accept_label(EdgeLabel::Name)
        .map(terminal_node_cloned);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(MappingValueStruct {
        node,
        type_name,
        name,
    }))
}

pub fn build_address_type(node: Rc<SyntaxNode>) -> Option<AddressType> {
    assert_nonterminal_kind(&node, NonterminalKind::AddressType);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::AddressKeyword)?;
    let payable_keyword = helper.accept_label(EdgeLabel::PayableKeyword).is_some();
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(AddressTypeStruct {
        node,
        payable_keyword,
    }))
}

pub fn build_block(node: Rc<SyntaxNode>) -> Option<Block> {
    assert_nonterminal_kind(&node, NonterminalKind::Block);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let statements = build_statements(helper.accept_label(EdgeLabel::Statements)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(BlockStruct { node, statements }))
}

pub fn build_unchecked_block(node: Rc<SyntaxNode>) -> Option<UncheckedBlock> {
    assert_nonterminal_kind(&node, NonterminalKind::UncheckedBlock);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::UncheckedKeyword)?;
    let block = build_block(helper.accept_label(EdgeLabel::Block)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(UncheckedBlockStruct { node, block }))
}

pub fn build_expression_statement(node: Rc<SyntaxNode>) -> Option<ExpressionStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::ExpressionStatement);
    let mut helper = ChildrenHelper::new(&node);
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ExpressionStatementStruct { node, expression }))
}

pub fn build_assembly_statement(node: Rc<SyntaxNode>) -> Option<AssemblyStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::AssemblyStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::AssemblyKeyword)?;
    let label = helper
        .accept_label(EdgeLabel::Label)
        .and_then(build_string_literal);
    let flags = helper
        .accept_label(EdgeLabel::Flags)
        .and_then(build_assembly_flags_declaration);
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(AssemblyStatementStruct {
        node,
        label,
        flags,
        body,
    }))
}

pub fn build_assembly_flags_declaration(node: Rc<SyntaxNode>) -> Option<AssemblyFlagsDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::AssemblyFlagsDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenParen)?;
    let flags = build_assembly_flags(helper.accept_label(EdgeLabel::Flags)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(AssemblyFlagsDeclarationStruct { node, flags }))
}

pub fn build_tuple_deconstruction_statement(
    node: Rc<SyntaxNode>,
) -> Option<TupleDeconstructionStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::TupleDeconstructionStatement);
    let mut helper = ChildrenHelper::new(&node);
    let var_keyword = helper.accept_label(EdgeLabel::VarKeyword).is_some();
    helper.skip_label(EdgeLabel::OpenParen)?;
    let elements = build_tuple_deconstruction_elements(helper.accept_label(EdgeLabel::Elements)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    helper.skip_label(EdgeLabel::Equal)?;
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(TupleDeconstructionStatementStruct {
        node,
        var_keyword,
        elements,
        expression,
    }))
}

pub fn build_tuple_deconstruction_element(
    node: Rc<SyntaxNode>,
) -> Option<TupleDeconstructionElement> {
    assert_nonterminal_kind(&node, NonterminalKind::TupleDeconstructionElement);
    let mut helper = ChildrenHelper::new(&node);
    let member = helper
        .accept_label(EdgeLabel::Member)
        .and_then(build_tuple_member);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(TupleDeconstructionElementStruct { node, member }))
}

pub fn build_typed_tuple_member(node: Rc<SyntaxNode>) -> Option<TypedTupleMember> {
    assert_nonterminal_kind(&node, NonterminalKind::TypedTupleMember);
    let mut helper = ChildrenHelper::new(&node);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let storage_location = helper
        .accept_label(EdgeLabel::StorageLocation)
        .and_then(build_storage_location);
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(TypedTupleMemberStruct {
        node,
        type_name,
        storage_location,
        name,
    }))
}

pub fn build_untyped_tuple_member(node: Rc<SyntaxNode>) -> Option<UntypedTupleMember> {
    assert_nonterminal_kind(&node, NonterminalKind::UntypedTupleMember);
    let mut helper = ChildrenHelper::new(&node);
    let storage_location = helper
        .accept_label(EdgeLabel::StorageLocation)
        .and_then(build_storage_location);
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(UntypedTupleMemberStruct {
        node,
        storage_location,
        name,
    }))
}

pub fn build_variable_declaration_statement(
    node: Rc<SyntaxNode>,
) -> Option<VariableDeclarationStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::VariableDeclarationStatement);
    let mut helper = ChildrenHelper::new(&node);
    let variable_type =
        build_variable_declaration_type(helper.accept_label(EdgeLabel::VariableType)?)?;
    let storage_location = helper
        .accept_label(EdgeLabel::StorageLocation)
        .and_then(build_storage_location);
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    let value = helper
        .accept_label(EdgeLabel::Value)
        .and_then(build_variable_declaration_value);
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(VariableDeclarationStatementStruct {
        node,
        variable_type,
        storage_location,
        name,
        value,
    }))
}

pub fn build_variable_declaration_value(node: Rc<SyntaxNode>) -> Option<VariableDeclarationValue> {
    assert_nonterminal_kind(&node, NonterminalKind::VariableDeclarationValue);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::Equal)?;
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(VariableDeclarationValueStruct { node, expression }))
}

pub fn build_if_statement(node: Rc<SyntaxNode>) -> Option<IfStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::IfStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::IfKeyword)?;
    helper.skip_label(EdgeLabel::OpenParen)?;
    let condition = build_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    let else_branch = helper
        .accept_label(EdgeLabel::ElseBranch)
        .and_then(build_else_branch);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(IfStatementStruct {
        node,
        condition,
        body,
        else_branch,
    }))
}

pub fn build_else_branch(node: Rc<SyntaxNode>) -> Option<ElseBranch> {
    assert_nonterminal_kind(&node, NonterminalKind::ElseBranch);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ElseKeyword)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ElseBranchStruct { node, body }))
}

pub fn build_for_statement(node: Rc<SyntaxNode>) -> Option<ForStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::ForStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ForKeyword)?;
    helper.skip_label(EdgeLabel::OpenParen)?;
    let initialization =
        build_for_statement_initialization(helper.accept_label(EdgeLabel::Initialization)?)?;
    let condition = build_for_statement_condition(helper.accept_label(EdgeLabel::Condition)?)?;
    let iterator = helper
        .accept_label(EdgeLabel::Iterator)
        .and_then(build_expression);
    helper.skip_label(EdgeLabel::CloseParen)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ForStatementStruct {
        node,
        initialization,
        condition,
        iterator,
        body,
    }))
}

pub fn build_while_statement(node: Rc<SyntaxNode>) -> Option<WhileStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::WhileStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::WhileKeyword)?;
    helper.skip_label(EdgeLabel::OpenParen)?;
    let condition = build_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(WhileStatementStruct {
        node,
        condition,
        body,
    }))
}

pub fn build_do_while_statement(node: Rc<SyntaxNode>) -> Option<DoWhileStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::DoWhileStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::DoKeyword)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    helper.skip_label(EdgeLabel::WhileKeyword)?;
    helper.skip_label(EdgeLabel::OpenParen)?;
    let condition = build_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(DoWhileStatementStruct {
        node,
        body,
        condition,
    }))
}

pub fn build_continue_statement(node: Rc<SyntaxNode>) -> Option<ContinueStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::ContinueStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ContinueKeyword)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ContinueStatementStruct { node }))
}

pub fn build_break_statement(node: Rc<SyntaxNode>) -> Option<BreakStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::BreakStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::BreakKeyword)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(BreakStatementStruct { node }))
}

pub fn build_return_statement(node: Rc<SyntaxNode>) -> Option<ReturnStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::ReturnStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ReturnKeyword)?;
    let expression = helper
        .accept_label(EdgeLabel::Expression)
        .and_then(build_expression);
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ReturnStatementStruct { node, expression }))
}

pub fn build_emit_statement(node: Rc<SyntaxNode>) -> Option<EmitStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::EmitStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::EmitKeyword)?;
    let event = build_identifier_path(helper.accept_label(EdgeLabel::Event)?)?;
    let arguments = build_arguments_declaration(helper.accept_label(EdgeLabel::Arguments)?)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(EmitStatementStruct {
        node,
        event,
        arguments,
    }))
}

pub fn build_try_statement(node: Rc<SyntaxNode>) -> Option<TryStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::TryStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::TryKeyword)?;
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    let returns = helper
        .accept_label(EdgeLabel::Returns)
        .and_then(build_returns_declaration);
    let body = build_block(helper.accept_label(EdgeLabel::Body)?)?;
    let catch_clauses = build_catch_clauses(helper.accept_label(EdgeLabel::CatchClauses)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(TryStatementStruct {
        node,
        expression,
        returns,
        body,
        catch_clauses,
    }))
}

pub fn build_catch_clause(node: Rc<SyntaxNode>) -> Option<CatchClause> {
    assert_nonterminal_kind(&node, NonterminalKind::CatchClause);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::CatchKeyword)?;
    let error = helper
        .accept_label(EdgeLabel::Error)
        .and_then(build_catch_clause_error);
    let body = build_block(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(CatchClauseStruct { node, error, body }))
}

pub fn build_catch_clause_error(node: Rc<SyntaxNode>) -> Option<CatchClauseError> {
    assert_nonterminal_kind(&node, NonterminalKind::CatchClauseError);
    let mut helper = ChildrenHelper::new(&node);
    let name = helper
        .accept_label(EdgeLabel::Name)
        .map(terminal_node_cloned);
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(CatchClauseErrorStruct {
        node,
        name,
        parameters,
    }))
}

pub fn build_revert_statement(node: Rc<SyntaxNode>) -> Option<RevertStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::RevertStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::RevertKeyword)?;
    let error = helper
        .accept_label(EdgeLabel::Error)
        .and_then(build_identifier_path);
    let arguments = build_arguments_declaration(helper.accept_label(EdgeLabel::Arguments)?)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(RevertStatementStruct {
        node,
        error,
        arguments,
    }))
}

pub fn build_throw_statement(node: Rc<SyntaxNode>) -> Option<ThrowStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::ThrowStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ThrowKeyword)?;
    helper.skip_label(EdgeLabel::Semicolon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ThrowStatementStruct { node }))
}

pub fn build_assignment_expression(node: Rc<SyntaxNode>) -> Option<AssignmentExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::AssignmentExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?);
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(AssignmentExpressionStruct {
        node,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_conditional_expression(node: Rc<SyntaxNode>) -> Option<ConditionalExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::ConditionalExpression);
    let mut helper = ChildrenHelper::new(&node);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    helper.skip_label(EdgeLabel::QuestionMark)?;
    let true_expression = build_expression(helper.accept_label(EdgeLabel::TrueExpression)?)?;
    helper.skip_label(EdgeLabel::Colon)?;
    let false_expression = build_expression(helper.accept_label(EdgeLabel::FalseExpression)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ConditionalExpressionStruct {
        node,
        operand,
        true_expression,
        false_expression,
    }))
}

pub fn build_or_expression(node: Rc<SyntaxNode>) -> Option<OrExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::OrExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    helper.skip_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(OrExpressionStruct {
        node,
        left_operand,
        right_operand,
    }))
}

pub fn build_and_expression(node: Rc<SyntaxNode>) -> Option<AndExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::AndExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    helper.skip_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(AndExpressionStruct {
        node,
        left_operand,
        right_operand,
    }))
}

pub fn build_equality_expression(node: Rc<SyntaxNode>) -> Option<EqualityExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::EqualityExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?);
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(EqualityExpressionStruct {
        node,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_inequality_expression(node: Rc<SyntaxNode>) -> Option<InequalityExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::InequalityExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?);
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(InequalityExpressionStruct {
        node,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_bitwise_or_expression(node: Rc<SyntaxNode>) -> Option<BitwiseOrExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::BitwiseOrExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    helper.skip_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(BitwiseOrExpressionStruct {
        node,
        left_operand,
        right_operand,
    }))
}

pub fn build_bitwise_xor_expression(node: Rc<SyntaxNode>) -> Option<BitwiseXorExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::BitwiseXorExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    helper.skip_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(BitwiseXorExpressionStruct {
        node,
        left_operand,
        right_operand,
    }))
}

pub fn build_bitwise_and_expression(node: Rc<SyntaxNode>) -> Option<BitwiseAndExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::BitwiseAndExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    helper.skip_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(BitwiseAndExpressionStruct {
        node,
        left_operand,
        right_operand,
    }))
}

pub fn build_shift_expression(node: Rc<SyntaxNode>) -> Option<ShiftExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::ShiftExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?);
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ShiftExpressionStruct {
        node,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_additive_expression(node: Rc<SyntaxNode>) -> Option<AdditiveExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::AdditiveExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?);
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(AdditiveExpressionStruct {
        node,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_multiplicative_expression(node: Rc<SyntaxNode>) -> Option<MultiplicativeExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::MultiplicativeExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?);
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(MultiplicativeExpressionStruct {
        node,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_exponentiation_expression(node: Rc<SyntaxNode>) -> Option<ExponentiationExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::ExponentiationExpression);
    let mut helper = ChildrenHelper::new(&node);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?);
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ExponentiationExpressionStruct {
        node,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_postfix_expression(node: Rc<SyntaxNode>) -> Option<PostfixExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::PostfixExpression);
    let mut helper = ChildrenHelper::new(&node);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(PostfixExpressionStruct {
        node,
        operand,
        operator,
    }))
}

pub fn build_prefix_expression(node: Rc<SyntaxNode>) -> Option<PrefixExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::PrefixExpression);
    let mut helper = ChildrenHelper::new(&node);
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(PrefixExpressionStruct {
        node,
        operator,
        operand,
    }))
}

pub fn build_function_call_expression(node: Rc<SyntaxNode>) -> Option<FunctionCallExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::FunctionCallExpression);
    let mut helper = ChildrenHelper::new(&node);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let arguments = build_arguments_declaration(helper.accept_label(EdgeLabel::Arguments)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(FunctionCallExpressionStruct {
        node,
        operand,
        arguments,
    }))
}

pub fn build_call_options_expression(node: Rc<SyntaxNode>) -> Option<CallOptionsExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::CallOptionsExpression);
    let mut helper = ChildrenHelper::new(&node);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let options = build_call_options(helper.accept_label(EdgeLabel::Options)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(CallOptionsExpressionStruct {
        node,
        operand,
        options,
    }))
}

pub fn build_member_access_expression(node: Rc<SyntaxNode>) -> Option<MemberAccessExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::MemberAccessExpression);
    let mut helper = ChildrenHelper::new(&node);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    helper.skip_label(EdgeLabel::Period)?;
    let member = terminal_node_cloned(helper.accept_label(EdgeLabel::Member)?);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(MemberAccessExpressionStruct {
        node,
        operand,
        member,
    }))
}

pub fn build_index_access_expression(node: Rc<SyntaxNode>) -> Option<IndexAccessExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::IndexAccessExpression);
    let mut helper = ChildrenHelper::new(&node);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    helper.skip_label(EdgeLabel::OpenBracket)?;
    let start = helper
        .accept_label(EdgeLabel::Start)
        .and_then(build_expression);
    let end = helper
        .accept_label(EdgeLabel::End)
        .and_then(build_index_access_end);
    helper.skip_label(EdgeLabel::CloseBracket)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(IndexAccessExpressionStruct {
        node,
        operand,
        start,
        end,
    }))
}

pub fn build_index_access_end(node: Rc<SyntaxNode>) -> Option<IndexAccessEnd> {
    assert_nonterminal_kind(&node, NonterminalKind::IndexAccessEnd);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::Colon)?;
    let end = helper
        .accept_label(EdgeLabel::End)
        .and_then(build_expression);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(IndexAccessEndStruct { node, end }))
}

pub fn build_positional_arguments_declaration(
    node: Rc<SyntaxNode>,
) -> Option<PositionalArgumentsDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::PositionalArgumentsDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenParen)?;
    let arguments = build_positional_arguments(helper.accept_label(EdgeLabel::Arguments)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(PositionalArgumentsDeclarationStruct {
        node,
        arguments,
    }))
}

pub fn build_named_arguments_declaration(
    node: Rc<SyntaxNode>,
) -> Option<NamedArgumentsDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::NamedArgumentsDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenParen)?;
    let arguments = helper
        .accept_label(EdgeLabel::Arguments)
        .and_then(build_named_argument_group);
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(NamedArgumentsDeclarationStruct { node, arguments }))
}

pub fn build_named_argument_group(node: Rc<SyntaxNode>) -> Option<NamedArgumentGroup> {
    assert_nonterminal_kind(&node, NonterminalKind::NamedArgumentGroup);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let arguments = build_named_arguments(helper.accept_label(EdgeLabel::Arguments)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(NamedArgumentGroupStruct { node, arguments }))
}

pub fn build_named_argument(node: Rc<SyntaxNode>) -> Option<NamedArgument> {
    assert_nonterminal_kind(&node, NonterminalKind::NamedArgument);
    let mut helper = ChildrenHelper::new(&node);
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    helper.skip_label(EdgeLabel::Colon)?;
    let value = build_expression(helper.accept_label(EdgeLabel::Value)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(NamedArgumentStruct { node, name, value }))
}

pub fn build_type_expression(node: Rc<SyntaxNode>) -> Option<TypeExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::TypeExpression);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::TypeKeyword)?;
    helper.skip_label(EdgeLabel::OpenParen)?;
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(TypeExpressionStruct { node, type_name }))
}

pub fn build_new_expression(node: Rc<SyntaxNode>) -> Option<NewExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::NewExpression);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::NewKeyword)?;
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(NewExpressionStruct { node, type_name }))
}

pub fn build_tuple_expression(node: Rc<SyntaxNode>) -> Option<TupleExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::TupleExpression);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenParen)?;
    let items = build_tuple_values(helper.accept_label(EdgeLabel::Items)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(TupleExpressionStruct { node, items }))
}

pub fn build_tuple_value(node: Rc<SyntaxNode>) -> Option<TupleValue> {
    assert_nonterminal_kind(&node, NonterminalKind::TupleValue);
    let mut helper = ChildrenHelper::new(&node);
    let expression = helper
        .accept_label(EdgeLabel::Expression)
        .and_then(build_expression);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(TupleValueStruct { node, expression }))
}

pub fn build_array_expression(node: Rc<SyntaxNode>) -> Option<ArrayExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::ArrayExpression);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenBracket)?;
    let items = build_array_values(helper.accept_label(EdgeLabel::Items)?)?;
    helper.skip_label(EdgeLabel::CloseBracket)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(ArrayExpressionStruct { node, items }))
}

pub fn build_hex_number_expression(node: Rc<SyntaxNode>) -> Option<HexNumberExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::HexNumberExpression);
    let mut helper = ChildrenHelper::new(&node);
    let literal = terminal_node_cloned(helper.accept_label(EdgeLabel::Literal)?);
    let unit = helper
        .accept_label(EdgeLabel::Unit)
        .and_then(build_number_unit);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(HexNumberExpressionStruct {
        node,
        literal,
        unit,
    }))
}

pub fn build_decimal_number_expression(node: Rc<SyntaxNode>) -> Option<DecimalNumberExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::DecimalNumberExpression);
    let mut helper = ChildrenHelper::new(&node);
    let literal = terminal_node_cloned(helper.accept_label(EdgeLabel::Literal)?);
    let unit = helper
        .accept_label(EdgeLabel::Unit)
        .and_then(build_number_unit);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(DecimalNumberExpressionStruct {
        node,
        literal,
        unit,
    }))
}

pub fn build_yul_block(node: Rc<SyntaxNode>) -> Option<YulBlock> {
    assert_nonterminal_kind(&node, NonterminalKind::YulBlock);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenBrace)?;
    let statements = build_yul_statements(helper.accept_label(EdgeLabel::Statements)?)?;
    helper.skip_label(EdgeLabel::CloseBrace)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulBlockStruct { node, statements }))
}

pub fn build_yul_function_definition(node: Rc<SyntaxNode>) -> Option<YulFunctionDefinition> {
    assert_nonterminal_kind(&node, NonterminalKind::YulFunctionDefinition);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::FunctionKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?);
    let parameters = build_yul_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let returns = helper
        .accept_label(EdgeLabel::Returns)
        .and_then(build_yul_returns_declaration);
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulFunctionDefinitionStruct {
        node,
        name,
        parameters,
        returns,
        body,
    }))
}

pub fn build_yul_parameters_declaration(node: Rc<SyntaxNode>) -> Option<YulParametersDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::YulParametersDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::OpenParen)?;
    let parameters = build_yul_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulParametersDeclarationStruct { node, parameters }))
}

pub fn build_yul_returns_declaration(node: Rc<SyntaxNode>) -> Option<YulReturnsDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::YulReturnsDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::MinusGreaterThan)?;
    let variables = build_yul_variable_names(helper.accept_label(EdgeLabel::Variables)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulReturnsDeclarationStruct { node, variables }))
}

pub fn build_yul_variable_declaration_statement(
    node: Rc<SyntaxNode>,
) -> Option<YulVariableDeclarationStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulVariableDeclarationStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::LetKeyword)?;
    let variables = build_yul_variable_names(helper.accept_label(EdgeLabel::Variables)?)?;
    let value = helper
        .accept_label(EdgeLabel::Value)
        .and_then(build_yul_variable_declaration_value);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulVariableDeclarationStatementStruct {
        node,
        variables,
        value,
    }))
}

pub fn build_yul_variable_declaration_value(
    node: Rc<SyntaxNode>,
) -> Option<YulVariableDeclarationValue> {
    assert_nonterminal_kind(&node, NonterminalKind::YulVariableDeclarationValue);
    let mut helper = ChildrenHelper::new(&node);
    let assignment = build_yul_assignment_operator(helper.accept_label(EdgeLabel::Assignment)?)?;
    let expression = build_yul_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulVariableDeclarationValueStruct {
        node,
        assignment,
        expression,
    }))
}

pub fn build_yul_variable_assignment_statement(
    node: Rc<SyntaxNode>,
) -> Option<YulVariableAssignmentStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulVariableAssignmentStatement);
    let mut helper = ChildrenHelper::new(&node);
    let variables = build_yul_paths(helper.accept_label(EdgeLabel::Variables)?)?;
    let assignment = build_yul_assignment_operator(helper.accept_label(EdgeLabel::Assignment)?)?;
    let expression = build_yul_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulVariableAssignmentStatementStruct {
        node,
        variables,
        assignment,
        expression,
    }))
}

pub fn build_yul_colon_and_equal(node: Rc<SyntaxNode>) -> Option<YulColonAndEqual> {
    assert_nonterminal_kind(&node, NonterminalKind::YulColonAndEqual);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::Colon)?;
    helper.skip_label(EdgeLabel::Equal)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulColonAndEqualStruct { node }))
}

pub fn build_yul_stack_assignment_statement(
    node: Rc<SyntaxNode>,
) -> Option<YulStackAssignmentStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulStackAssignmentStatement);
    let mut helper = ChildrenHelper::new(&node);
    let assignment =
        build_yul_stack_assignment_operator(helper.accept_label(EdgeLabel::Assignment)?)?;
    let variable = terminal_node_cloned(helper.accept_label(EdgeLabel::Variable)?);
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulStackAssignmentStatementStruct {
        node,
        assignment,
        variable,
    }))
}

pub fn build_yul_equal_and_colon(node: Rc<SyntaxNode>) -> Option<YulEqualAndColon> {
    assert_nonterminal_kind(&node, NonterminalKind::YulEqualAndColon);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::Equal)?;
    helper.skip_label(EdgeLabel::Colon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulEqualAndColonStruct { node }))
}

pub fn build_yul_if_statement(node: Rc<SyntaxNode>) -> Option<YulIfStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulIfStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::IfKeyword)?;
    let condition = build_yul_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulIfStatementStruct {
        node,
        condition,
        body,
    }))
}

pub fn build_yul_for_statement(node: Rc<SyntaxNode>) -> Option<YulForStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulForStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ForKeyword)?;
    let initialization = build_yul_block(helper.accept_label(EdgeLabel::Initialization)?)?;
    let condition = build_yul_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    let iterator = build_yul_block(helper.accept_label(EdgeLabel::Iterator)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulForStatementStruct {
        node,
        initialization,
        condition,
        iterator,
        body,
    }))
}

pub fn build_yul_switch_statement(node: Rc<SyntaxNode>) -> Option<YulSwitchStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulSwitchStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::SwitchKeyword)?;
    let expression = build_yul_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    let cases = build_yul_switch_cases(helper.accept_label(EdgeLabel::Cases)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulSwitchStatementStruct {
        node,
        expression,
        cases,
    }))
}

pub fn build_yul_default_case(node: Rc<SyntaxNode>) -> Option<YulDefaultCase> {
    assert_nonterminal_kind(&node, NonterminalKind::YulDefaultCase);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::DefaultKeyword)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulDefaultCaseStruct { node, body }))
}

pub fn build_yul_value_case(node: Rc<SyntaxNode>) -> Option<YulValueCase> {
    assert_nonterminal_kind(&node, NonterminalKind::YulValueCase);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::CaseKeyword)?;
    let value = build_yul_literal(helper.accept_label(EdgeLabel::Value)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulValueCaseStruct { node, value, body }))
}

pub fn build_yul_leave_statement(node: Rc<SyntaxNode>) -> Option<YulLeaveStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulLeaveStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::LeaveKeyword)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulLeaveStatementStruct { node }))
}

pub fn build_yul_break_statement(node: Rc<SyntaxNode>) -> Option<YulBreakStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulBreakStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::BreakKeyword)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulBreakStatementStruct { node }))
}

pub fn build_yul_continue_statement(node: Rc<SyntaxNode>) -> Option<YulContinueStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulContinueStatement);
    let mut helper = ChildrenHelper::new(&node);
    helper.skip_label(EdgeLabel::ContinueKeyword)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulContinueStatementStruct { node }))
}

pub fn build_yul_label(node: Rc<SyntaxNode>) -> Option<YulLabel> {
    assert_nonterminal_kind(&node, NonterminalKind::YulLabel);
    let mut helper = ChildrenHelper::new(&node);
    let label = terminal_node_cloned(helper.accept_label(EdgeLabel::Label)?);
    helper.skip_label(EdgeLabel::Colon)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulLabelStruct { node, label }))
}

pub fn build_yul_function_call_expression(
    node: Rc<SyntaxNode>,
) -> Option<YulFunctionCallExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::YulFunctionCallExpression);
    let mut helper = ChildrenHelper::new(&node);
    let operand = build_yul_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    helper.skip_label(EdgeLabel::OpenParen)?;
    let arguments = build_yul_arguments(helper.accept_label(EdgeLabel::Arguments)?)?;
    helper.skip_label(EdgeLabel::CloseParen)?;
    if !helper.finalize() {
        return None;
    }

    Some(Rc::new(YulFunctionCallExpressionStruct {
        node,
        operand,
        arguments,
    }))
}

//
// Choices:
//

#[allow(clippy::needless_pass_by_value)]
pub fn build_source_unit_member(node: Rc<SyntaxNode>) -> Option<SourceUnitMember> {
    assert_nonterminal_kind(&node, NonterminalKind::SourceUnitMember);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::PragmaDirective) => {
            SourceUnitMember::PragmaDirective(build_pragma_directive(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ImportDirective) => {
            SourceUnitMember::ImportDirective(build_import_directive(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ContractDefinition) => {
            SourceUnitMember::ContractDefinition(build_contract_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::InterfaceDefinition) => {
            SourceUnitMember::InterfaceDefinition(build_interface_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::LibraryDefinition) => {
            SourceUnitMember::LibraryDefinition(build_library_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StructDefinition) => {
            SourceUnitMember::StructDefinition(build_struct_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EnumDefinition) => {
            SourceUnitMember::EnumDefinition(build_enum_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionDefinition) => {
            SourceUnitMember::FunctionDefinition(build_function_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ErrorDefinition) => {
            SourceUnitMember::ErrorDefinition(build_error_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UserDefinedValueTypeDefinition) => {
            SourceUnitMember::UserDefinedValueTypeDefinition(
                build_user_defined_value_type_definition(variant)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::UsingDirective) => {
            SourceUnitMember::UsingDirective(build_using_directive(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EventDefinition) => {
            SourceUnitMember::EventDefinition(build_event_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ConstantDefinition) => {
            SourceUnitMember::ConstantDefinition(build_constant_definition(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_pragma(node: Rc<SyntaxNode>) -> Option<Pragma> {
    assert_nonterminal_kind(&node, NonterminalKind::Pragma);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::VersionPragma) => {
            Pragma::VersionPragma(build_version_pragma(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::AbicoderPragma) => {
            Pragma::AbicoderPragma(build_abicoder_pragma(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ExperimentalPragma) => {
            Pragma::ExperimentalPragma(build_experimental_pragma(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_abicoder_version(node: Rc<SyntaxNode>) -> Option<AbicoderVersion> {
    assert_nonterminal_kind(&node, NonterminalKind::AbicoderVersion);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::AbicoderV1Keyword) => AbicoderVersion::AbicoderV1Keyword,
        NodeKind::Terminal(TerminalKind::AbicoderV2Keyword) => AbicoderVersion::AbicoderV2Keyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_experimental_feature(node: Rc<SyntaxNode>) -> Option<ExperimentalFeature> {
    assert_nonterminal_kind(&node, NonterminalKind::ExperimentalFeature);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::StringLiteral) => {
            ExperimentalFeature::StringLiteral(build_string_literal(variant)?)
        }
        NodeKind::Terminal(TerminalKind::ABIEncoderV2Keyword) => {
            ExperimentalFeature::ABIEncoderV2Keyword
        }
        NodeKind::Terminal(TerminalKind::SMTCheckerKeyword) => {
            ExperimentalFeature::SMTCheckerKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_version_expression(node: Rc<SyntaxNode>) -> Option<VersionExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::VersionExpression);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::VersionRange) => {
            VersionExpression::VersionRange(build_version_range(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::VersionTerm) => {
            VersionExpression::VersionTerm(build_version_term(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_version_operator(node: Rc<SyntaxNode>) -> Option<VersionOperator> {
    assert_nonterminal_kind(&node, NonterminalKind::VersionOperator);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::Caret) => VersionOperator::Caret,
        NodeKind::Terminal(TerminalKind::Tilde) => VersionOperator::Tilde,
        NodeKind::Terminal(TerminalKind::Equal) => VersionOperator::Equal,
        NodeKind::Terminal(TerminalKind::LessThan) => VersionOperator::LessThan,
        NodeKind::Terminal(TerminalKind::GreaterThan) => VersionOperator::GreaterThan,
        NodeKind::Terminal(TerminalKind::LessThanEqual) => VersionOperator::LessThanEqual,
        NodeKind::Terminal(TerminalKind::GreaterThanEqual) => VersionOperator::GreaterThanEqual,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_version_literal(node: Rc<SyntaxNode>) -> Option<VersionLiteral> {
    assert_nonterminal_kind(&node, NonterminalKind::VersionLiteral);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::SimpleVersionLiteral) => {
            VersionLiteral::SimpleVersionLiteral(build_simple_version_literal(variant)?)
        }
        NodeKind::Terminal(TerminalKind::SingleQuotedVersionLiteral) => {
            VersionLiteral::SingleQuotedVersionLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedVersionLiteral) => {
            VersionLiteral::DoubleQuotedVersionLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_import_clause(node: Rc<SyntaxNode>) -> Option<ImportClause> {
    assert_nonterminal_kind(&node, NonterminalKind::ImportClause);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::PathImport) => {
            ImportClause::PathImport(build_path_import(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::NamedImport) => {
            ImportClause::NamedImport(build_named_import(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ImportDeconstruction) => {
            ImportClause::ImportDeconstruction(build_import_deconstruction(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_using_clause(node: Rc<SyntaxNode>) -> Option<UsingClause> {
    assert_nonterminal_kind(&node, NonterminalKind::UsingClause);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::IdentifierPath) => {
            UsingClause::IdentifierPath(build_identifier_path(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UsingDeconstruction) => {
            UsingClause::UsingDeconstruction(build_using_deconstruction(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_using_operator(node: Rc<SyntaxNode>) -> Option<UsingOperator> {
    assert_nonterminal_kind(&node, NonterminalKind::UsingOperator);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::Ampersand) => UsingOperator::Ampersand,
        NodeKind::Terminal(TerminalKind::Asterisk) => UsingOperator::Asterisk,
        NodeKind::Terminal(TerminalKind::BangEqual) => UsingOperator::BangEqual,
        NodeKind::Terminal(TerminalKind::Bar) => UsingOperator::Bar,
        NodeKind::Terminal(TerminalKind::Caret) => UsingOperator::Caret,
        NodeKind::Terminal(TerminalKind::EqualEqual) => UsingOperator::EqualEqual,
        NodeKind::Terminal(TerminalKind::GreaterThan) => UsingOperator::GreaterThan,
        NodeKind::Terminal(TerminalKind::GreaterThanEqual) => UsingOperator::GreaterThanEqual,
        NodeKind::Terminal(TerminalKind::LessThan) => UsingOperator::LessThan,
        NodeKind::Terminal(TerminalKind::LessThanEqual) => UsingOperator::LessThanEqual,
        NodeKind::Terminal(TerminalKind::Minus) => UsingOperator::Minus,
        NodeKind::Terminal(TerminalKind::Percent) => UsingOperator::Percent,
        NodeKind::Terminal(TerminalKind::Plus) => UsingOperator::Plus,
        NodeKind::Terminal(TerminalKind::Slash) => UsingOperator::Slash,
        NodeKind::Terminal(TerminalKind::Tilde) => UsingOperator::Tilde,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_using_target(node: Rc<SyntaxNode>) -> Option<UsingTarget> {
    assert_nonterminal_kind(&node, NonterminalKind::UsingTarget);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::TypeName) => {
            UsingTarget::TypeName(build_type_name(variant)?)
        }
        NodeKind::Terminal(TerminalKind::Asterisk) => UsingTarget::Asterisk,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_contract_specifier(node: Rc<SyntaxNode>) -> Option<ContractSpecifier> {
    assert_nonterminal_kind(&node, NonterminalKind::ContractSpecifier);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::InheritanceSpecifier) => {
            ContractSpecifier::InheritanceSpecifier(build_inheritance_specifier(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StorageLayoutSpecifier) => {
            ContractSpecifier::StorageLayoutSpecifier(build_storage_layout_specifier(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_contract_member(node: Rc<SyntaxNode>) -> Option<ContractMember> {
    assert_nonterminal_kind(&node, NonterminalKind::ContractMember);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::UsingDirective) => {
            ContractMember::UsingDirective(build_using_directive(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionDefinition) => {
            ContractMember::FunctionDefinition(build_function_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ConstructorDefinition) => {
            ContractMember::ConstructorDefinition(build_constructor_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ReceiveFunctionDefinition) => {
            ContractMember::ReceiveFunctionDefinition(build_receive_function_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::FallbackFunctionDefinition) => {
            ContractMember::FallbackFunctionDefinition(build_fallback_function_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UnnamedFunctionDefinition) => {
            ContractMember::UnnamedFunctionDefinition(build_unnamed_function_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ModifierDefinition) => {
            ContractMember::ModifierDefinition(build_modifier_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StructDefinition) => {
            ContractMember::StructDefinition(build_struct_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EnumDefinition) => {
            ContractMember::EnumDefinition(build_enum_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EventDefinition) => {
            ContractMember::EventDefinition(build_event_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ErrorDefinition) => {
            ContractMember::ErrorDefinition(build_error_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UserDefinedValueTypeDefinition) => {
            ContractMember::UserDefinedValueTypeDefinition(
                build_user_defined_value_type_definition(variant)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::StateVariableDefinition) => {
            ContractMember::StateVariableDefinition(build_state_variable_definition(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_state_variable_attribute(node: Rc<SyntaxNode>) -> Option<StateVariableAttribute> {
    assert_nonterminal_kind(&node, NonterminalKind::StateVariableAttribute);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            StateVariableAttribute::OverrideSpecifier(build_override_specifier(variant)?)
        }
        NodeKind::Terminal(TerminalKind::ConstantKeyword) => {
            StateVariableAttribute::ConstantKeyword
        }
        NodeKind::Terminal(TerminalKind::InternalKeyword) => {
            StateVariableAttribute::InternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PrivateKeyword) => StateVariableAttribute::PrivateKeyword,
        NodeKind::Terminal(TerminalKind::PublicKeyword) => StateVariableAttribute::PublicKeyword,
        NodeKind::Terminal(TerminalKind::ImmutableKeyword) => {
            StateVariableAttribute::ImmutableKeyword
        }
        NodeKind::Terminal(TerminalKind::TransientKeyword) => {
            StateVariableAttribute::TransientKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_function_name(node: Rc<SyntaxNode>) -> Option<FunctionName> {
    assert_nonterminal_kind(&node, NonterminalKind::FunctionName);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::Identifier) => {
            FunctionName::Identifier(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::FallbackKeyword) => FunctionName::FallbackKeyword,
        NodeKind::Terminal(TerminalKind::ReceiveKeyword) => FunctionName::ReceiveKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_function_attribute(node: Rc<SyntaxNode>) -> Option<FunctionAttribute> {
    assert_nonterminal_kind(&node, NonterminalKind::FunctionAttribute);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            FunctionAttribute::ModifierInvocation(build_modifier_invocation(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            FunctionAttribute::OverrideSpecifier(build_override_specifier(variant)?)
        }
        NodeKind::Terminal(TerminalKind::ConstantKeyword) => FunctionAttribute::ConstantKeyword,
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => FunctionAttribute::ExternalKeyword,
        NodeKind::Terminal(TerminalKind::InternalKeyword) => FunctionAttribute::InternalKeyword,
        NodeKind::Terminal(TerminalKind::PayableKeyword) => FunctionAttribute::PayableKeyword,
        NodeKind::Terminal(TerminalKind::PrivateKeyword) => FunctionAttribute::PrivateKeyword,
        NodeKind::Terminal(TerminalKind::PublicKeyword) => FunctionAttribute::PublicKeyword,
        NodeKind::Terminal(TerminalKind::PureKeyword) => FunctionAttribute::PureKeyword,
        NodeKind::Terminal(TerminalKind::ViewKeyword) => FunctionAttribute::ViewKeyword,
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => FunctionAttribute::VirtualKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_function_body(node: Rc<SyntaxNode>) -> Option<FunctionBody> {
    assert_nonterminal_kind(&node, NonterminalKind::FunctionBody);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::Block) => FunctionBody::Block(build_block(variant)?),
        NodeKind::Terminal(TerminalKind::Semicolon) => FunctionBody::Semicolon,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_constructor_attribute(node: Rc<SyntaxNode>) -> Option<ConstructorAttribute> {
    assert_nonterminal_kind(&node, NonterminalKind::ConstructorAttribute);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            ConstructorAttribute::ModifierInvocation(build_modifier_invocation(variant)?)
        }
        NodeKind::Terminal(TerminalKind::InternalKeyword) => ConstructorAttribute::InternalKeyword,
        NodeKind::Terminal(TerminalKind::OverrideKeyword) => ConstructorAttribute::OverrideKeyword,
        NodeKind::Terminal(TerminalKind::PayableKeyword) => ConstructorAttribute::PayableKeyword,
        NodeKind::Terminal(TerminalKind::PublicKeyword) => ConstructorAttribute::PublicKeyword,
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => ConstructorAttribute::VirtualKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_unnamed_function_attribute(node: Rc<SyntaxNode>) -> Option<UnnamedFunctionAttribute> {
    assert_nonterminal_kind(&node, NonterminalKind::UnnamedFunctionAttribute);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            UnnamedFunctionAttribute::ModifierInvocation(build_modifier_invocation(variant)?)
        }
        NodeKind::Terminal(TerminalKind::ConstantKeyword) => {
            UnnamedFunctionAttribute::ConstantKeyword
        }
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => {
            UnnamedFunctionAttribute::ExternalKeyword
        }
        NodeKind::Terminal(TerminalKind::InternalKeyword) => {
            UnnamedFunctionAttribute::InternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            UnnamedFunctionAttribute::PayableKeyword
        }
        NodeKind::Terminal(TerminalKind::PrivateKeyword) => {
            UnnamedFunctionAttribute::PrivateKeyword
        }
        NodeKind::Terminal(TerminalKind::PublicKeyword) => UnnamedFunctionAttribute::PublicKeyword,
        NodeKind::Terminal(TerminalKind::PureKeyword) => UnnamedFunctionAttribute::PureKeyword,
        NodeKind::Terminal(TerminalKind::ViewKeyword) => UnnamedFunctionAttribute::ViewKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_fallback_function_attribute(
    node: Rc<SyntaxNode>,
) -> Option<FallbackFunctionAttribute> {
    assert_nonterminal_kind(&node, NonterminalKind::FallbackFunctionAttribute);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            FallbackFunctionAttribute::ModifierInvocation(build_modifier_invocation(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            FallbackFunctionAttribute::OverrideSpecifier(build_override_specifier(variant)?)
        }
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => {
            FallbackFunctionAttribute::ExternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            FallbackFunctionAttribute::PayableKeyword
        }
        NodeKind::Terminal(TerminalKind::PureKeyword) => FallbackFunctionAttribute::PureKeyword,
        NodeKind::Terminal(TerminalKind::ViewKeyword) => FallbackFunctionAttribute::ViewKeyword,
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => {
            FallbackFunctionAttribute::VirtualKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_receive_function_attribute(node: Rc<SyntaxNode>) -> Option<ReceiveFunctionAttribute> {
    assert_nonterminal_kind(&node, NonterminalKind::ReceiveFunctionAttribute);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            ReceiveFunctionAttribute::ModifierInvocation(build_modifier_invocation(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            ReceiveFunctionAttribute::OverrideSpecifier(build_override_specifier(variant)?)
        }
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => {
            ReceiveFunctionAttribute::ExternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            ReceiveFunctionAttribute::PayableKeyword
        }
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => {
            ReceiveFunctionAttribute::VirtualKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_modifier_attribute(node: Rc<SyntaxNode>) -> Option<ModifierAttribute> {
    assert_nonterminal_kind(&node, NonterminalKind::ModifierAttribute);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            ModifierAttribute::OverrideSpecifier(build_override_specifier(variant)?)
        }
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => ModifierAttribute::VirtualKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_type_name(node: Rc<SyntaxNode>) -> Option<TypeName> {
    assert_nonterminal_kind(&node, NonterminalKind::TypeName);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ArrayTypeName) => {
            TypeName::ArrayTypeName(build_array_type_name(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionType) => {
            TypeName::FunctionType(build_function_type(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::MappingType) => {
            TypeName::MappingType(build_mapping_type(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ElementaryType) => {
            TypeName::ElementaryType(build_elementary_type(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::IdentifierPath) => {
            TypeName::IdentifierPath(build_identifier_path(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_function_type_attribute(node: Rc<SyntaxNode>) -> Option<FunctionTypeAttribute> {
    assert_nonterminal_kind(&node, NonterminalKind::FunctionTypeAttribute);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::InternalKeyword) => FunctionTypeAttribute::InternalKeyword,
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => FunctionTypeAttribute::ExternalKeyword,
        NodeKind::Terminal(TerminalKind::PrivateKeyword) => FunctionTypeAttribute::PrivateKeyword,
        NodeKind::Terminal(TerminalKind::PublicKeyword) => FunctionTypeAttribute::PublicKeyword,
        NodeKind::Terminal(TerminalKind::ConstantKeyword) => FunctionTypeAttribute::ConstantKeyword,
        NodeKind::Terminal(TerminalKind::PureKeyword) => FunctionTypeAttribute::PureKeyword,
        NodeKind::Terminal(TerminalKind::ViewKeyword) => FunctionTypeAttribute::ViewKeyword,
        NodeKind::Terminal(TerminalKind::PayableKeyword) => FunctionTypeAttribute::PayableKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_mapping_key_type(node: Rc<SyntaxNode>) -> Option<MappingKeyType> {
    assert_nonterminal_kind(&node, NonterminalKind::MappingKeyType);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ElementaryType) => {
            MappingKeyType::ElementaryType(build_elementary_type(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::IdentifierPath) => {
            MappingKeyType::IdentifierPath(build_identifier_path(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_elementary_type(node: Rc<SyntaxNode>) -> Option<ElementaryType> {
    assert_nonterminal_kind(&node, NonterminalKind::ElementaryType);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::AddressType) => {
            ElementaryType::AddressType(build_address_type(variant)?)
        }
        NodeKind::Terminal(TerminalKind::BytesKeyword) => {
            ElementaryType::BytesKeyword(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::IntKeyword) => {
            ElementaryType::IntKeyword(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::UintKeyword) => {
            ElementaryType::UintKeyword(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::FixedKeyword) => {
            ElementaryType::FixedKeyword(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::UfixedKeyword) => {
            ElementaryType::UfixedKeyword(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::BoolKeyword) => ElementaryType::BoolKeyword,
        NodeKind::Terminal(TerminalKind::ByteKeyword) => ElementaryType::ByteKeyword,
        NodeKind::Terminal(TerminalKind::StringKeyword) => ElementaryType::StringKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_statement(node: Rc<SyntaxNode>) -> Option<Statement> {
    assert_nonterminal_kind(&node, NonterminalKind::Statement);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::IfStatement) => {
            Statement::IfStatement(build_if_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ForStatement) => {
            Statement::ForStatement(build_for_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::WhileStatement) => {
            Statement::WhileStatement(build_while_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::DoWhileStatement) => {
            Statement::DoWhileStatement(build_do_while_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ContinueStatement) => {
            Statement::ContinueStatement(build_continue_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::BreakStatement) => {
            Statement::BreakStatement(build_break_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ReturnStatement) => {
            Statement::ReturnStatement(build_return_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ThrowStatement) => {
            Statement::ThrowStatement(build_throw_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EmitStatement) => {
            Statement::EmitStatement(build_emit_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::TryStatement) => {
            Statement::TryStatement(build_try_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::RevertStatement) => {
            Statement::RevertStatement(build_revert_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::AssemblyStatement) => {
            Statement::AssemblyStatement(build_assembly_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::Block) => Statement::Block(build_block(variant)?),
        NodeKind::Nonterminal(NonterminalKind::UncheckedBlock) => {
            Statement::UncheckedBlock(build_unchecked_block(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement) => {
            Statement::TupleDeconstructionStatement(build_tuple_deconstruction_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement) => {
            Statement::VariableDeclarationStatement(build_variable_declaration_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) => {
            Statement::ExpressionStatement(build_expression_statement(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_tuple_member(node: Rc<SyntaxNode>) -> Option<TupleMember> {
    assert_nonterminal_kind(&node, NonterminalKind::TupleMember);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::TypedTupleMember) => {
            TupleMember::TypedTupleMember(build_typed_tuple_member(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UntypedTupleMember) => {
            TupleMember::UntypedTupleMember(build_untyped_tuple_member(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_variable_declaration_type(node: Rc<SyntaxNode>) -> Option<VariableDeclarationType> {
    assert_nonterminal_kind(&node, NonterminalKind::VariableDeclarationType);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::TypeName) => {
            VariableDeclarationType::TypeName(build_type_name(variant)?)
        }
        NodeKind::Terminal(TerminalKind::VarKeyword) => VariableDeclarationType::VarKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_storage_location(node: Rc<SyntaxNode>) -> Option<StorageLocation> {
    assert_nonterminal_kind(&node, NonterminalKind::StorageLocation);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::MemoryKeyword) => StorageLocation::MemoryKeyword,
        NodeKind::Terminal(TerminalKind::StorageKeyword) => StorageLocation::StorageKeyword,
        NodeKind::Terminal(TerminalKind::CallDataKeyword) => StorageLocation::CallDataKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_for_statement_initialization(
    node: Rc<SyntaxNode>,
) -> Option<ForStatementInitialization> {
    assert_nonterminal_kind(&node, NonterminalKind::ForStatementInitialization);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement) => {
            ForStatementInitialization::TupleDeconstructionStatement(
                build_tuple_deconstruction_statement(variant)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement) => {
            ForStatementInitialization::VariableDeclarationStatement(
                build_variable_declaration_statement(variant)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) => {
            ForStatementInitialization::ExpressionStatement(build_expression_statement(variant)?)
        }
        NodeKind::Terminal(TerminalKind::Semicolon) => ForStatementInitialization::Semicolon,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_for_statement_condition(node: Rc<SyntaxNode>) -> Option<ForStatementCondition> {
    assert_nonterminal_kind(&node, NonterminalKind::ForStatementCondition);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) => {
            ForStatementCondition::ExpressionStatement(build_expression_statement(variant)?)
        }
        NodeKind::Terminal(TerminalKind::Semicolon) => ForStatementCondition::Semicolon,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_expression(node: Rc<SyntaxNode>) -> Option<Expression> {
    assert_nonterminal_kind(&node, NonterminalKind::Expression);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::AssignmentExpression) => {
            Expression::AssignmentExpression(build_assignment_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ConditionalExpression) => {
            Expression::ConditionalExpression(build_conditional_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::OrExpression) => {
            Expression::OrExpression(build_or_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::AndExpression) => {
            Expression::AndExpression(build_and_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EqualityExpression) => {
            Expression::EqualityExpression(build_equality_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::InequalityExpression) => {
            Expression::InequalityExpression(build_inequality_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::BitwiseOrExpression) => {
            Expression::BitwiseOrExpression(build_bitwise_or_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::BitwiseXorExpression) => {
            Expression::BitwiseXorExpression(build_bitwise_xor_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::BitwiseAndExpression) => {
            Expression::BitwiseAndExpression(build_bitwise_and_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ShiftExpression) => {
            Expression::ShiftExpression(build_shift_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::AdditiveExpression) => {
            Expression::AdditiveExpression(build_additive_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::MultiplicativeExpression) => {
            Expression::MultiplicativeExpression(build_multiplicative_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ExponentiationExpression) => {
            Expression::ExponentiationExpression(build_exponentiation_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::PostfixExpression) => {
            Expression::PostfixExpression(build_postfix_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::PrefixExpression) => {
            Expression::PrefixExpression(build_prefix_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionCallExpression) => {
            Expression::FunctionCallExpression(build_function_call_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::CallOptionsExpression) => {
            Expression::CallOptionsExpression(build_call_options_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::MemberAccessExpression) => {
            Expression::MemberAccessExpression(build_member_access_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::IndexAccessExpression) => {
            Expression::IndexAccessExpression(build_index_access_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::NewExpression) => {
            Expression::NewExpression(build_new_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::TupleExpression) => {
            Expression::TupleExpression(build_tuple_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::TypeExpression) => {
            Expression::TypeExpression(build_type_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ArrayExpression) => {
            Expression::ArrayExpression(build_array_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::HexNumberExpression) => {
            Expression::HexNumberExpression(build_hex_number_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::DecimalNumberExpression) => {
            Expression::DecimalNumberExpression(build_decimal_number_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StringExpression) => {
            Expression::StringExpression(build_string_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ElementaryType) => {
            Expression::ElementaryType(build_elementary_type(variant)?)
        }
        NodeKind::Terminal(TerminalKind::Identifier) => {
            Expression::Identifier(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => Expression::PayableKeyword,
        NodeKind::Terminal(TerminalKind::ThisKeyword) => Expression::ThisKeyword,
        NodeKind::Terminal(TerminalKind::SuperKeyword) => Expression::SuperKeyword,
        NodeKind::Terminal(TerminalKind::TrueKeyword) => Expression::TrueKeyword,
        NodeKind::Terminal(TerminalKind::FalseKeyword) => Expression::FalseKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_arguments_declaration(node: Rc<SyntaxNode>) -> Option<ArgumentsDeclaration> {
    assert_nonterminal_kind(&node, NonterminalKind::ArgumentsDeclaration);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::PositionalArgumentsDeclaration) => {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(
                build_positional_arguments_declaration(variant)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::NamedArgumentsDeclaration) => {
            ArgumentsDeclaration::NamedArgumentsDeclaration(build_named_arguments_declaration(
                variant,
            )?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_number_unit(node: Rc<SyntaxNode>) -> Option<NumberUnit> {
    assert_nonterminal_kind(&node, NonterminalKind::NumberUnit);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::WeiKeyword) => NumberUnit::WeiKeyword,
        NodeKind::Terminal(TerminalKind::GweiKeyword) => NumberUnit::GweiKeyword,
        NodeKind::Terminal(TerminalKind::SzaboKeyword) => NumberUnit::SzaboKeyword,
        NodeKind::Terminal(TerminalKind::FinneyKeyword) => NumberUnit::FinneyKeyword,
        NodeKind::Terminal(TerminalKind::EtherKeyword) => NumberUnit::EtherKeyword,
        NodeKind::Terminal(TerminalKind::SecondsKeyword) => NumberUnit::SecondsKeyword,
        NodeKind::Terminal(TerminalKind::MinutesKeyword) => NumberUnit::MinutesKeyword,
        NodeKind::Terminal(TerminalKind::HoursKeyword) => NumberUnit::HoursKeyword,
        NodeKind::Terminal(TerminalKind::DaysKeyword) => NumberUnit::DaysKeyword,
        NodeKind::Terminal(TerminalKind::WeeksKeyword) => NumberUnit::WeeksKeyword,
        NodeKind::Terminal(TerminalKind::YearsKeyword) => NumberUnit::YearsKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_string_expression(node: Rc<SyntaxNode>) -> Option<StringExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::StringExpression);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::StringLiteral) => {
            StringExpression::StringLiteral(build_string_literal(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StringLiterals) => {
            StringExpression::StringLiterals(build_string_literals(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::HexStringLiteral) => {
            StringExpression::HexStringLiteral(build_hex_string_literal(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::HexStringLiterals) => {
            StringExpression::HexStringLiterals(build_hex_string_literals(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UnicodeStringLiterals) => {
            StringExpression::UnicodeStringLiterals(build_unicode_string_literals(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_string_literal(node: Rc<SyntaxNode>) -> Option<StringLiteral> {
    assert_nonterminal_kind(&node, NonterminalKind::StringLiteral);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::SingleQuotedStringLiteral) => {
            StringLiteral::SingleQuotedStringLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedStringLiteral) => {
            StringLiteral::DoubleQuotedStringLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_hex_string_literal(node: Rc<SyntaxNode>) -> Option<HexStringLiteral> {
    assert_nonterminal_kind(&node, NonterminalKind::HexStringLiteral);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::SingleQuotedHexStringLiteral) => {
            HexStringLiteral::SingleQuotedHexStringLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedHexStringLiteral) => {
            HexStringLiteral::DoubleQuotedHexStringLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_unicode_string_literal(node: Rc<SyntaxNode>) -> Option<UnicodeStringLiteral> {
    assert_nonterminal_kind(&node, NonterminalKind::UnicodeStringLiteral);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::SingleQuotedUnicodeStringLiteral) => {
            UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedUnicodeStringLiteral) => {
            UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_statement(node: Rc<SyntaxNode>) -> Option<YulStatement> {
    assert_nonterminal_kind(&node, NonterminalKind::YulStatement);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulBlock) => {
            YulStatement::YulBlock(build_yul_block(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulFunctionDefinition) => {
            YulStatement::YulFunctionDefinition(build_yul_function_definition(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulStackAssignmentStatement) => {
            YulStatement::YulStackAssignmentStatement(build_yul_stack_assignment_statement(
                variant,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulIfStatement) => {
            YulStatement::YulIfStatement(build_yul_if_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulForStatement) => {
            YulStatement::YulForStatement(build_yul_for_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulSwitchStatement) => {
            YulStatement::YulSwitchStatement(build_yul_switch_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulLeaveStatement) => {
            YulStatement::YulLeaveStatement(build_yul_leave_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulBreakStatement) => {
            YulStatement::YulBreakStatement(build_yul_break_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulContinueStatement) => {
            YulStatement::YulContinueStatement(build_yul_continue_statement(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulVariableAssignmentStatement) => {
            YulStatement::YulVariableAssignmentStatement(build_yul_variable_assignment_statement(
                variant,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulLabel) => {
            YulStatement::YulLabel(build_yul_label(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulVariableDeclarationStatement) => {
            YulStatement::YulVariableDeclarationStatement(build_yul_variable_declaration_statement(
                variant,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulExpression) => {
            YulStatement::YulExpression(build_yul_expression(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_assignment_operator(node: Rc<SyntaxNode>) -> Option<YulAssignmentOperator> {
    assert_nonterminal_kind(&node, NonterminalKind::YulAssignmentOperator);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulColonAndEqual) => {
            YulAssignmentOperator::YulColonAndEqual(build_yul_colon_and_equal(variant)?)
        }
        NodeKind::Terminal(TerminalKind::ColonEqual) => YulAssignmentOperator::ColonEqual,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_stack_assignment_operator(
    node: Rc<SyntaxNode>,
) -> Option<YulStackAssignmentOperator> {
    assert_nonterminal_kind(&node, NonterminalKind::YulStackAssignmentOperator);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulEqualAndColon) => {
            YulStackAssignmentOperator::YulEqualAndColon(build_yul_equal_and_colon(variant)?)
        }
        NodeKind::Terminal(TerminalKind::EqualColon) => YulStackAssignmentOperator::EqualColon,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_switch_case(node: Rc<SyntaxNode>) -> Option<YulSwitchCase> {
    assert_nonterminal_kind(&node, NonterminalKind::YulSwitchCase);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulDefaultCase) => {
            YulSwitchCase::YulDefaultCase(build_yul_default_case(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulValueCase) => {
            YulSwitchCase::YulValueCase(build_yul_value_case(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_expression(node: Rc<SyntaxNode>) -> Option<YulExpression> {
    assert_nonterminal_kind(&node, NonterminalKind::YulExpression);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulFunctionCallExpression) => {
            YulExpression::YulFunctionCallExpression(build_yul_function_call_expression(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulLiteral) => {
            YulExpression::YulLiteral(build_yul_literal(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulPath) => {
            YulExpression::YulPath(build_yul_path(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_literal(node: Rc<SyntaxNode>) -> Option<YulLiteral> {
    assert_nonterminal_kind(&node, NonterminalKind::YulLiteral);
    let mut helper = ChildrenHelper::new(&node);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::HexStringLiteral) => {
            YulLiteral::HexStringLiteral(build_hex_string_literal(variant)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StringLiteral) => {
            YulLiteral::StringLiteral(build_string_literal(variant)?)
        }
        NodeKind::Terminal(TerminalKind::YulDecimalLiteral) => {
            YulLiteral::YulDecimalLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::YulHexLiteral) => {
            YulLiteral::YulHexLiteral(terminal_node_cloned(variant))
        }
        NodeKind::Terminal(TerminalKind::YulTrueKeyword) => YulLiteral::YulTrueKeyword,
        NodeKind::Terminal(TerminalKind::YulFalseKeyword) => YulLiteral::YulFalseKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            unreachable!(
                "unexpected variant node of kind {kind}",
                kind = variant.kind()
            );
        }
    };
    if !helper.finalize() {
        return None;
    }
    Some(item)
}

//
// Repeated & Separated
//

#[allow(clippy::needless_pass_by_value)]
pub fn build_source_unit_members(node: Rc<SyntaxNode>) -> Option<SourceUnitMembers> {
    assert_nonterminal_kind(&node, NonterminalKind::SourceUnitMembers);
    let mut items = SourceUnitMembers::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_source_unit_member(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_version_expression_sets(node: Rc<SyntaxNode>) -> Option<VersionExpressionSets> {
    assert_nonterminal_kind(&node, NonterminalKind::VersionExpressionSets);
    let mut items = VersionExpressionSets::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_version_expression_set(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_version_expression_set(node: Rc<SyntaxNode>) -> Option<VersionExpressionSet> {
    assert_nonterminal_kind(&node, NonterminalKind::VersionExpressionSet);
    let mut items = VersionExpressionSet::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_version_expression(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_simple_version_literal(node: Rc<SyntaxNode>) -> Option<SimpleVersionLiteral> {
    assert_nonterminal_kind(&node, NonterminalKind::SimpleVersionLiteral);
    let mut items = SimpleVersionLiteral::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        items.push(terminal_node_cloned(child));
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_import_deconstruction_symbols(
    node: Rc<SyntaxNode>,
) -> Option<ImportDeconstructionSymbols> {
    assert_nonterminal_kind(&node, NonterminalKind::ImportDeconstructionSymbols);
    let mut items = ImportDeconstructionSymbols::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_import_deconstruction_symbol(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_using_deconstruction_symbols(
    node: Rc<SyntaxNode>,
) -> Option<UsingDeconstructionSymbols> {
    assert_nonterminal_kind(&node, NonterminalKind::UsingDeconstructionSymbols);
    let mut items = UsingDeconstructionSymbols::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_using_deconstruction_symbol(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_contract_specifiers(node: Rc<SyntaxNode>) -> Option<ContractSpecifiers> {
    assert_nonterminal_kind(&node, NonterminalKind::ContractSpecifiers);
    let mut items = ContractSpecifiers::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_contract_specifier(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_inheritance_types(node: Rc<SyntaxNode>) -> Option<InheritanceTypes> {
    assert_nonterminal_kind(&node, NonterminalKind::InheritanceTypes);
    let mut items = InheritanceTypes::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_inheritance_type(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_contract_members(node: Rc<SyntaxNode>) -> Option<ContractMembers> {
    assert_nonterminal_kind(&node, NonterminalKind::ContractMembers);
    let mut items = ContractMembers::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_contract_member(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_interface_members(node: Rc<SyntaxNode>) -> Option<InterfaceMembers> {
    assert_nonterminal_kind(&node, NonterminalKind::InterfaceMembers);
    let mut items = InterfaceMembers::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_contract_member(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_library_members(node: Rc<SyntaxNode>) -> Option<LibraryMembers> {
    assert_nonterminal_kind(&node, NonterminalKind::LibraryMembers);
    let mut items = LibraryMembers::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_contract_member(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_struct_members(node: Rc<SyntaxNode>) -> Option<StructMembers> {
    assert_nonterminal_kind(&node, NonterminalKind::StructMembers);
    let mut items = StructMembers::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_struct_member(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_enum_members(node: Rc<SyntaxNode>) -> Option<EnumMembers> {
    assert_nonterminal_kind(&node, NonterminalKind::EnumMembers);
    let mut items = EnumMembers::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        items.push(terminal_node_cloned(child));
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_state_variable_attributes(node: Rc<SyntaxNode>) -> Option<StateVariableAttributes> {
    assert_nonterminal_kind(&node, NonterminalKind::StateVariableAttributes);
    let mut items = StateVariableAttributes::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_state_variable_attribute(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_parameters(node: Rc<SyntaxNode>) -> Option<Parameters> {
    assert_nonterminal_kind(&node, NonterminalKind::Parameters);
    let mut items = Parameters::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_parameter(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_function_attributes(node: Rc<SyntaxNode>) -> Option<FunctionAttributes> {
    assert_nonterminal_kind(&node, NonterminalKind::FunctionAttributes);
    let mut items = FunctionAttributes::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_function_attribute(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_override_paths(node: Rc<SyntaxNode>) -> Option<OverridePaths> {
    assert_nonterminal_kind(&node, NonterminalKind::OverridePaths);
    let mut items = OverridePaths::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_identifier_path(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_constructor_attributes(node: Rc<SyntaxNode>) -> Option<ConstructorAttributes> {
    assert_nonterminal_kind(&node, NonterminalKind::ConstructorAttributes);
    let mut items = ConstructorAttributes::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_constructor_attribute(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_unnamed_function_attributes(
    node: Rc<SyntaxNode>,
) -> Option<UnnamedFunctionAttributes> {
    assert_nonterminal_kind(&node, NonterminalKind::UnnamedFunctionAttributes);
    let mut items = UnnamedFunctionAttributes::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_unnamed_function_attribute(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_fallback_function_attributes(
    node: Rc<SyntaxNode>,
) -> Option<FallbackFunctionAttributes> {
    assert_nonterminal_kind(&node, NonterminalKind::FallbackFunctionAttributes);
    let mut items = FallbackFunctionAttributes::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_fallback_function_attribute(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_receive_function_attributes(
    node: Rc<SyntaxNode>,
) -> Option<ReceiveFunctionAttributes> {
    assert_nonterminal_kind(&node, NonterminalKind::ReceiveFunctionAttributes);
    let mut items = ReceiveFunctionAttributes::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_receive_function_attribute(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_modifier_attributes(node: Rc<SyntaxNode>) -> Option<ModifierAttributes> {
    assert_nonterminal_kind(&node, NonterminalKind::ModifierAttributes);
    let mut items = ModifierAttributes::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_modifier_attribute(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_event_parameters(node: Rc<SyntaxNode>) -> Option<EventParameters> {
    assert_nonterminal_kind(&node, NonterminalKind::EventParameters);
    let mut items = EventParameters::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_event_parameter(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_error_parameters(node: Rc<SyntaxNode>) -> Option<ErrorParameters> {
    assert_nonterminal_kind(&node, NonterminalKind::ErrorParameters);
    let mut items = ErrorParameters::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_error_parameter(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_function_type_attributes(node: Rc<SyntaxNode>) -> Option<FunctionTypeAttributes> {
    assert_nonterminal_kind(&node, NonterminalKind::FunctionTypeAttributes);
    let mut items = FunctionTypeAttributes::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_function_type_attribute(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_statements(node: Rc<SyntaxNode>) -> Option<Statements> {
    assert_nonterminal_kind(&node, NonterminalKind::Statements);
    let mut items = Statements::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_statement(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_assembly_flags(node: Rc<SyntaxNode>) -> Option<AssemblyFlags> {
    assert_nonterminal_kind(&node, NonterminalKind::AssemblyFlags);
    let mut items = AssemblyFlags::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_string_literal(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_tuple_deconstruction_elements(
    node: Rc<SyntaxNode>,
) -> Option<TupleDeconstructionElements> {
    assert_nonterminal_kind(&node, NonterminalKind::TupleDeconstructionElements);
    let mut items = TupleDeconstructionElements::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_tuple_deconstruction_element(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_catch_clauses(node: Rc<SyntaxNode>) -> Option<CatchClauses> {
    assert_nonterminal_kind(&node, NonterminalKind::CatchClauses);
    let mut items = CatchClauses::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_catch_clause(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_positional_arguments(node: Rc<SyntaxNode>) -> Option<PositionalArguments> {
    assert_nonterminal_kind(&node, NonterminalKind::PositionalArguments);
    let mut items = PositionalArguments::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_expression(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_named_arguments(node: Rc<SyntaxNode>) -> Option<NamedArguments> {
    assert_nonterminal_kind(&node, NonterminalKind::NamedArguments);
    let mut items = NamedArguments::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_named_argument(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_call_options(node: Rc<SyntaxNode>) -> Option<CallOptions> {
    assert_nonterminal_kind(&node, NonterminalKind::CallOptions);
    let mut items = CallOptions::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_named_argument(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_tuple_values(node: Rc<SyntaxNode>) -> Option<TupleValues> {
    assert_nonterminal_kind(&node, NonterminalKind::TupleValues);
    let mut items = TupleValues::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_tuple_value(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_array_values(node: Rc<SyntaxNode>) -> Option<ArrayValues> {
    assert_nonterminal_kind(&node, NonterminalKind::ArrayValues);
    let mut items = ArrayValues::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_expression(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_string_literals(node: Rc<SyntaxNode>) -> Option<StringLiterals> {
    assert_nonterminal_kind(&node, NonterminalKind::StringLiterals);
    let mut items = StringLiterals::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_string_literal(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_hex_string_literals(node: Rc<SyntaxNode>) -> Option<HexStringLiterals> {
    assert_nonterminal_kind(&node, NonterminalKind::HexStringLiterals);
    let mut items = HexStringLiterals::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_hex_string_literal(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_unicode_string_literals(node: Rc<SyntaxNode>) -> Option<UnicodeStringLiterals> {
    assert_nonterminal_kind(&node, NonterminalKind::UnicodeStringLiterals);
    let mut items = UnicodeStringLiterals::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_unicode_string_literal(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_identifier_path(node: Rc<SyntaxNode>) -> Option<IdentifierPath> {
    assert_nonterminal_kind(&node, NonterminalKind::IdentifierPath);
    let mut items = IdentifierPath::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        items.push(terminal_node_cloned(child));
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_statements(node: Rc<SyntaxNode>) -> Option<YulStatements> {
    assert_nonterminal_kind(&node, NonterminalKind::YulStatements);
    let mut items = YulStatements::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_yul_statement(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_parameters(node: Rc<SyntaxNode>) -> Option<YulParameters> {
    assert_nonterminal_kind(&node, NonterminalKind::YulParameters);
    let mut items = YulParameters::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        items.push(terminal_node_cloned(child));
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_variable_names(node: Rc<SyntaxNode>) -> Option<YulVariableNames> {
    assert_nonterminal_kind(&node, NonterminalKind::YulVariableNames);
    let mut items = YulVariableNames::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        items.push(terminal_node_cloned(child));
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_switch_cases(node: Rc<SyntaxNode>) -> Option<YulSwitchCases> {
    assert_nonterminal_kind(&node, NonterminalKind::YulSwitchCases);
    let mut items = YulSwitchCases::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_yul_switch_case(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_arguments(node: Rc<SyntaxNode>) -> Option<YulArguments> {
    assert_nonterminal_kind(&node, NonterminalKind::YulArguments);
    let mut items = YulArguments::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_yul_expression(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_paths(node: Rc<SyntaxNode>) -> Option<YulPaths> {
    assert_nonterminal_kind(&node, NonterminalKind::YulPaths);
    let mut items = YulPaths::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        if let Some(item) = build_yul_path(child) {
            items.push(item);
        }
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_yul_path(node: Rc<SyntaxNode>) -> Option<YulPath> {
    assert_nonterminal_kind(&node, NonterminalKind::YulPath);
    let mut items = YulPath::new();
    let mut helper = ChildrenHelper::new(&node);
    while let Some(child) = helper.accept_label(EdgeLabel::Item) {
        items.push(terminal_node_cloned(child));
        helper.skip_label(EdgeLabel::Separator);
    }
    if !helper.finalize() {
        return None;
    }
    Some(items)
}

//
// Common:
//

#[allow(dead_code)]
#[inline]
fn assert_nonterminal_kind(node: &Rc<SyntaxNode>, kind: NonterminalKind) {
    assert_eq!(
        node.kind(),
        NodeKind::Nonterminal(kind),
        "expected non-terminal of kind {kind}, got {node:?}"
    );
}

#[allow(dead_code)]
#[inline]
fn terminal_node_cloned(node: Rc<SyntaxNode>) -> Rc<SyntaxNode> {
    assert!(node.is_terminal(), "expected terminal node");
    node
}

struct ChildrenHelper<'a> {
    children: &'a Rc<SyntaxNode>,
    index: usize,
}

impl<'a> ChildrenHelper<'a> {
    fn new(children: &'a Rc<SyntaxNode>) -> Self {
        let mut index = 0;
        while index < children.children_count() {
            if children.child_is_valid_and_not_trivia(index) {
                break;
            }
            index += 1;
        }
        Self { children, index }
    }

    fn skip_label(&mut self, label: EdgeLabel) -> Option<usize> {
        if self.index >= self.children.children_count()
            || self.children.child_label(self.index) != label
        {
            return None;
        }

        let result = self.index;
        loop {
            self.index += 1;
            if self.index >= self.children.children_count()
                || self.children.child_is_valid_and_not_trivia(self.index)
            {
                break;
            }
        }
        Some(result)
    }

    fn accept_label(&mut self, label: EdgeLabel) -> Option<Rc<SyntaxNode>> {
        let index = self.skip_label(label)?;
        Some(self.children.nth_child(index))
    }

    fn finalize(mut self) -> bool {
        // skip over trailing trivia and unrecognized nodes
        while self.index < self.children.children_count() {
            if self.children.child_is_valid_and_not_trivia(self.index) {
                return false;
            }
            self.index += 1;
        }
        true
    }
}
