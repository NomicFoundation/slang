// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::{
    Edge, EdgeLabel, Node, NodeKind, NonterminalKind, NonterminalNode, TerminalKind, TerminalNode,
};

//
// Sequences:
//

pub fn build_source_unit(node: &Rc<NonterminalNode>) -> Result<SourceUnit> {
    expect_nonterminal_kind(node, NonterminalKind::SourceUnit)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let members =
        build_source_unit_members(nonterminal_node(helper.accept_label(EdgeLabel::Members)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(SourceUnitStruct {
        node_id: node.id(),
        members,
    }))
}

pub fn build_pragma_directive(node: &Rc<NonterminalNode>) -> Result<PragmaDirective> {
    expect_nonterminal_kind(node, NonterminalKind::PragmaDirective)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::PragmaKeyword)?;
    let pragma = build_pragma(nonterminal_node(helper.accept_label(EdgeLabel::Pragma)?)?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(PragmaDirectiveStruct {
        node_id: node.id(),
        pragma,
    }))
}

pub fn build_abicoder_pragma(node: &Rc<NonterminalNode>) -> Result<AbicoderPragma> {
    expect_nonterminal_kind(node, NonterminalKind::AbicoderPragma)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::AbicoderKeyword)?;
    let version = terminal_node_cloned(helper.accept_label(EdgeLabel::Version)?)?;
    helper.finalize()?;

    Ok(Rc::new(AbicoderPragmaStruct {
        node_id: node.id(),
        version,
    }))
}

pub fn build_experimental_pragma(node: &Rc<NonterminalNode>) -> Result<ExperimentalPragma> {
    expect_nonterminal_kind(node, NonterminalKind::ExperimentalPragma)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ExperimentalKeyword)?;
    let feature =
        build_experimental_feature(nonterminal_node(helper.accept_label(EdgeLabel::Feature)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(ExperimentalPragmaStruct {
        node_id: node.id(),
        feature,
    }))
}

pub fn build_version_pragma(node: &Rc<NonterminalNode>) -> Result<VersionPragma> {
    expect_nonterminal_kind(node, NonterminalKind::VersionPragma)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::SolidityKeyword)?;
    let sets =
        build_version_expression_sets(nonterminal_node(helper.accept_label(EdgeLabel::Sets)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(VersionPragmaStruct {
        node_id: node.id(),
        sets,
    }))
}

pub fn build_version_range(node: &Rc<NonterminalNode>) -> Result<VersionRange> {
    expect_nonterminal_kind(node, NonterminalKind::VersionRange)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let start = build_version_literal(nonterminal_node(helper.accept_label(EdgeLabel::Start)?)?)?;
    _ = helper.accept_label(EdgeLabel::Minus)?;
    let end = build_version_literal(nonterminal_node(helper.accept_label(EdgeLabel::End)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(VersionRangeStruct {
        node_id: node.id(),
        start,
        end,
    }))
}

pub fn build_version_term(node: &Rc<NonterminalNode>) -> Result<VersionTerm> {
    expect_nonterminal_kind(node, NonterminalKind::VersionTerm)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operator = if helper.at_label(EdgeLabel::Operator) {
        Some(build_version_operator(nonterminal_node(
            helper.accept_label(EdgeLabel::Operator)?,
        )?)?)
    } else {
        None
    };
    let literal =
        build_version_literal(nonterminal_node(helper.accept_label(EdgeLabel::Literal)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(VersionTermStruct {
        node_id: node.id(),
        operator,
        literal,
    }))
}

pub fn build_import_directive(node: &Rc<NonterminalNode>) -> Result<ImportDirective> {
    expect_nonterminal_kind(node, NonterminalKind::ImportDirective)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ImportKeyword)?;
    let clause = build_import_clause(nonterminal_node(helper.accept_label(EdgeLabel::Clause)?)?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(ImportDirectiveStruct {
        node_id: node.id(),
        clause,
    }))
}

pub fn build_path_import(node: &Rc<NonterminalNode>) -> Result<PathImport> {
    expect_nonterminal_kind(node, NonterminalKind::PathImport)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let path = build_string_literal(nonterminal_node(helper.accept_label(EdgeLabel::Path)?)?)?;
    let alias = if helper.at_label(EdgeLabel::Alias) {
        Some(build_import_alias(nonterminal_node(
            helper.accept_label(EdgeLabel::Alias)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(PathImportStruct {
        node_id: node.id(),
        path,
        alias,
    }))
}

pub fn build_named_import(node: &Rc<NonterminalNode>) -> Result<NamedImport> {
    expect_nonterminal_kind(node, NonterminalKind::NamedImport)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::Asterisk)?;
    let alias = build_import_alias(nonterminal_node(helper.accept_label(EdgeLabel::Alias)?)?)?;
    _ = helper.accept_label(EdgeLabel::FromKeyword)?;
    let path = build_string_literal(nonterminal_node(helper.accept_label(EdgeLabel::Path)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedImportStruct {
        node_id: node.id(),
        alias,
        path,
    }))
}

pub fn build_import_deconstruction(node: &Rc<NonterminalNode>) -> Result<ImportDeconstruction> {
    expect_nonterminal_kind(node, NonterminalKind::ImportDeconstruction)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let symbols = build_import_deconstruction_symbols(nonterminal_node(
        helper.accept_label(EdgeLabel::Symbols)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    _ = helper.accept_label(EdgeLabel::FromKeyword)?;
    let path = build_string_literal(nonterminal_node(helper.accept_label(EdgeLabel::Path)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(ImportDeconstructionStruct {
        node_id: node.id(),
        symbols,
        path,
    }))
}

pub fn build_import_deconstruction_symbol(
    node: &Rc<NonterminalNode>,
) -> Result<ImportDeconstructionSymbol> {
    expect_nonterminal_kind(node, NonterminalKind::ImportDeconstructionSymbol)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    let alias = if helper.at_label(EdgeLabel::Alias) {
        Some(build_import_alias(nonterminal_node(
            helper.accept_label(EdgeLabel::Alias)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ImportDeconstructionSymbolStruct {
        node_id: node.id(),
        name,
        alias,
    }))
}

pub fn build_import_alias(node: &Rc<NonterminalNode>) -> Result<ImportAlias> {
    expect_nonterminal_kind(node, NonterminalKind::ImportAlias)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::AsKeyword)?;
    let identifier = terminal_node_cloned(helper.accept_label(EdgeLabel::Identifier)?)?;
    helper.finalize()?;

    Ok(Rc::new(ImportAliasStruct {
        node_id: node.id(),
        identifier,
    }))
}

pub fn build_using_directive(node: &Rc<NonterminalNode>) -> Result<UsingDirective> {
    expect_nonterminal_kind(node, NonterminalKind::UsingDirective)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::UsingKeyword)?;
    let clause = build_using_clause(nonterminal_node(helper.accept_label(EdgeLabel::Clause)?)?)?;
    _ = helper.accept_label(EdgeLabel::ForKeyword)?;
    let target = build_using_target(nonterminal_node(helper.accept_label(EdgeLabel::Target)?)?)?;
    let global_keyword = if helper.at_label(EdgeLabel::GlobalKeyword) {
        Some(terminal_node_cloned(
            helper.accept_label(EdgeLabel::GlobalKeyword)?,
        )?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(UsingDirectiveStruct {
        node_id: node.id(),
        clause,
        target,
        global_keyword,
    }))
}

pub fn build_using_deconstruction(node: &Rc<NonterminalNode>) -> Result<UsingDeconstruction> {
    expect_nonterminal_kind(node, NonterminalKind::UsingDeconstruction)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let symbols = build_using_deconstruction_symbols(nonterminal_node(
        helper.accept_label(EdgeLabel::Symbols)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(UsingDeconstructionStruct {
        node_id: node.id(),
        symbols,
    }))
}

pub fn build_using_deconstruction_symbol(
    node: &Rc<NonterminalNode>,
) -> Result<UsingDeconstructionSymbol> {
    expect_nonterminal_kind(node, NonterminalKind::UsingDeconstructionSymbol)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let name = build_identifier_path(nonterminal_node(helper.accept_label(EdgeLabel::Name)?)?)?;
    let alias = if helper.at_label(EdgeLabel::Alias) {
        Some(build_using_alias(nonterminal_node(
            helper.accept_label(EdgeLabel::Alias)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(UsingDeconstructionSymbolStruct {
        node_id: node.id(),
        name,
        alias,
    }))
}

pub fn build_using_alias(node: &Rc<NonterminalNode>) -> Result<UsingAlias> {
    expect_nonterminal_kind(node, NonterminalKind::UsingAlias)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::AsKeyword)?;
    let operator =
        build_using_operator(nonterminal_node(helper.accept_label(EdgeLabel::Operator)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(UsingAliasStruct {
        node_id: node.id(),
        operator,
    }))
}

pub fn build_contract_definition(node: &Rc<NonterminalNode>) -> Result<ContractDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::ContractDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let abstract_keyword = if helper.at_label(EdgeLabel::AbstractKeyword) {
        Some(terminal_node_cloned(
            helper.accept_label(EdgeLabel::AbstractKeyword)?,
        )?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::ContractKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    let specifiers = build_contract_specifiers(nonterminal_node(
        helper.accept_label(EdgeLabel::Specifiers)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let members =
        build_contract_members(nonterminal_node(helper.accept_label(EdgeLabel::Members)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(ContractDefinitionStruct {
        node_id: node.id(),
        abstract_keyword,
        name,
        specifiers,
        members,
    }))
}

pub fn build_inheritance_specifier(node: &Rc<NonterminalNode>) -> Result<InheritanceSpecifier> {
    expect_nonterminal_kind(node, NonterminalKind::InheritanceSpecifier)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::IsKeyword)?;
    let types = build_inheritance_types(nonterminal_node(helper.accept_label(EdgeLabel::Types)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(InheritanceSpecifierStruct {
        node_id: node.id(),
        types,
    }))
}

pub fn build_inheritance_type(node: &Rc<NonterminalNode>) -> Result<InheritanceType> {
    expect_nonterminal_kind(node, NonterminalKind::InheritanceType)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let type_name =
        build_identifier_path(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    let arguments = if helper.at_label(EdgeLabel::Arguments) {
        Some(build_arguments_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Arguments)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(InheritanceTypeStruct {
        node_id: node.id(),
        type_name,
        arguments,
    }))
}

pub fn build_storage_layout_specifier(
    node: &Rc<NonterminalNode>,
) -> Result<StorageLayoutSpecifier> {
    expect_nonterminal_kind(node, NonterminalKind::StorageLayoutSpecifier)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::LayoutKeyword)?;
    _ = helper.accept_label(EdgeLabel::AtKeyword)?;
    let expression = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Expression)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(StorageLayoutSpecifierStruct {
        node_id: node.id(),
        expression,
    }))
}

pub fn build_interface_definition(node: &Rc<NonterminalNode>) -> Result<InterfaceDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::InterfaceDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::InterfaceKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    let inheritance = if helper.at_label(EdgeLabel::Inheritance) {
        Some(build_inheritance_specifier(nonterminal_node(
            helper.accept_label(EdgeLabel::Inheritance)?,
        )?)?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let members =
        build_interface_members(nonterminal_node(helper.accept_label(EdgeLabel::Members)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(InterfaceDefinitionStruct {
        node_id: node.id(),
        name,
        inheritance,
        members,
    }))
}

pub fn build_library_definition(node: &Rc<NonterminalNode>) -> Result<LibraryDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::LibraryDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::LibraryKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let members =
        build_library_members(nonterminal_node(helper.accept_label(EdgeLabel::Members)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(LibraryDefinitionStruct {
        node_id: node.id(),
        name,
        members,
    }))
}

pub fn build_struct_definition(node: &Rc<NonterminalNode>) -> Result<StructDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::StructDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::StructKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let members =
        build_struct_members(nonterminal_node(helper.accept_label(EdgeLabel::Members)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(StructDefinitionStruct {
        node_id: node.id(),
        name,
        members,
    }))
}

pub fn build_struct_member(node: &Rc<NonterminalNode>) -> Result<StructMember> {
    expect_nonterminal_kind(node, NonterminalKind::StructMember)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(StructMemberStruct {
        node_id: node.id(),
        type_name,
        name,
    }))
}

pub fn build_enum_definition(node: &Rc<NonterminalNode>) -> Result<EnumDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::EnumDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::EnumKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let members = build_enum_members(nonterminal_node(helper.accept_label(EdgeLabel::Members)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(EnumDefinitionStruct {
        node_id: node.id(),
        name,
        members,
    }))
}

pub fn build_constant_definition(node: &Rc<NonterminalNode>) -> Result<ConstantDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::ConstantDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    _ = helper.accept_label(EdgeLabel::ConstantKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    _ = helper.accept_label(EdgeLabel::Equal)?;
    let value = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Value)?)?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(ConstantDefinitionStruct {
        node_id: node.id(),
        type_name,
        name,
        value,
    }))
}

pub fn build_state_variable_definition(
    node: &Rc<NonterminalNode>,
) -> Result<StateVariableDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::StateVariableDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    let attributes = build_state_variable_attributes(nonterminal_node(
        helper.accept_label(EdgeLabel::Attributes)?,
    )?)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    let value = if helper.at_label(EdgeLabel::Value) {
        Some(build_state_variable_definition_value(nonterminal_node(
            helper.accept_label(EdgeLabel::Value)?,
        )?)?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(StateVariableDefinitionStruct {
        node_id: node.id(),
        type_name,
        attributes,
        name,
        value,
    }))
}

pub fn build_state_variable_definition_value(
    node: &Rc<NonterminalNode>,
) -> Result<StateVariableDefinitionValue> {
    expect_nonterminal_kind(node, NonterminalKind::StateVariableDefinitionValue)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::Equal)?;
    let value = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Value)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(StateVariableDefinitionValueStruct {
        node_id: node.id(),
        value,
    }))
}

pub fn build_function_definition(node: &Rc<NonterminalNode>) -> Result<FunctionDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::FunctionDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::FunctionKeyword)?;
    let name = build_function_name(nonterminal_node(helper.accept_label(EdgeLabel::Name)?)?)?;
    let parameters = build_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    let attributes = build_function_attributes(nonterminal_node(
        helper.accept_label(EdgeLabel::Attributes)?,
    )?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_returns_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)?)
    } else {
        None
    };
    let body = build_function_body(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(FunctionDefinitionStruct {
        node_id: node.id(),
        name,
        parameters,
        attributes,
        returns,
        body,
    }))
}

pub fn build_parameters_declaration(node: &Rc<NonterminalNode>) -> Result<ParametersDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::ParametersDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let parameters = build_parameters(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(ParametersDeclarationStruct {
        node_id: node.id(),
        parameters,
    }))
}

pub fn build_parameter(node: &Rc<NonterminalNode>) -> Result<Parameter> {
    expect_nonterminal_kind(node, NonterminalKind::Parameter)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    let storage_location = if helper.at_label(EdgeLabel::StorageLocation) {
        Some(build_storage_location(nonterminal_node(
            helper.accept_label(EdgeLabel::StorageLocation)?,
        )?)?)
    } else {
        None
    };
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ParameterStruct {
        node_id: node.id(),
        type_name,
        storage_location,
        name,
    }))
}

pub fn build_override_specifier(node: &Rc<NonterminalNode>) -> Result<OverrideSpecifier> {
    expect_nonterminal_kind(node, NonterminalKind::OverrideSpecifier)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OverrideKeyword)?;
    let overridden = if helper.at_label(EdgeLabel::Overridden) {
        Some(build_override_paths_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Overridden)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(OverrideSpecifierStruct {
        node_id: node.id(),
        overridden,
    }))
}

pub fn build_override_paths_declaration(
    node: &Rc<NonterminalNode>,
) -> Result<OverridePathsDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::OverridePathsDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let paths = build_override_paths(nonterminal_node(helper.accept_label(EdgeLabel::Paths)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(OverridePathsDeclarationStruct {
        node_id: node.id(),
        paths,
    }))
}

pub fn build_returns_declaration(node: &Rc<NonterminalNode>) -> Result<ReturnsDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::ReturnsDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ReturnsKeyword)?;
    let variables = build_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Variables)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(ReturnsDeclarationStruct {
        node_id: node.id(),
        variables,
    }))
}

pub fn build_constructor_definition(node: &Rc<NonterminalNode>) -> Result<ConstructorDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::ConstructorDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ConstructorKeyword)?;
    let parameters = build_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    let attributes = build_constructor_attributes(nonterminal_node(
        helper.accept_label(EdgeLabel::Attributes)?,
    )?)?;
    let body = build_block(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(ConstructorDefinitionStruct {
        node_id: node.id(),
        parameters,
        attributes,
        body,
    }))
}

pub fn build_unnamed_function_definition(
    node: &Rc<NonterminalNode>,
) -> Result<UnnamedFunctionDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::UnnamedFunctionDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::FunctionKeyword)?;
    let parameters = build_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    let attributes = build_unnamed_function_attributes(nonterminal_node(
        helper.accept_label(EdgeLabel::Attributes)?,
    )?)?;
    let body = build_function_body(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(UnnamedFunctionDefinitionStruct {
        node_id: node.id(),
        parameters,
        attributes,
        body,
    }))
}

pub fn build_fallback_function_definition(
    node: &Rc<NonterminalNode>,
) -> Result<FallbackFunctionDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::FallbackFunctionDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::FallbackKeyword)?;
    let parameters = build_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    let attributes = build_fallback_function_attributes(nonterminal_node(
        helper.accept_label(EdgeLabel::Attributes)?,
    )?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_returns_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)?)
    } else {
        None
    };
    let body = build_function_body(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(FallbackFunctionDefinitionStruct {
        node_id: node.id(),
        parameters,
        attributes,
        returns,
        body,
    }))
}

pub fn build_receive_function_definition(
    node: &Rc<NonterminalNode>,
) -> Result<ReceiveFunctionDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::ReceiveFunctionDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ReceiveKeyword)?;
    let parameters = build_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    let attributes = build_receive_function_attributes(nonterminal_node(
        helper.accept_label(EdgeLabel::Attributes)?,
    )?)?;
    let body = build_function_body(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(ReceiveFunctionDefinitionStruct {
        node_id: node.id(),
        parameters,
        attributes,
        body,
    }))
}

pub fn build_modifier_definition(node: &Rc<NonterminalNode>) -> Result<ModifierDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::ModifierDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ModifierKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    let parameters = if helper.at_label(EdgeLabel::Parameters) {
        Some(build_parameters_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Parameters)?,
        )?)?)
    } else {
        None
    };
    let attributes = build_modifier_attributes(nonterminal_node(
        helper.accept_label(EdgeLabel::Attributes)?,
    )?)?;
    let body = build_function_body(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(ModifierDefinitionStruct {
        node_id: node.id(),
        name,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_modifier_invocation(node: &Rc<NonterminalNode>) -> Result<ModifierInvocation> {
    expect_nonterminal_kind(node, NonterminalKind::ModifierInvocation)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let name = build_identifier_path(nonterminal_node(helper.accept_label(EdgeLabel::Name)?)?)?;
    let arguments = if helper.at_label(EdgeLabel::Arguments) {
        Some(build_arguments_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Arguments)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ModifierInvocationStruct {
        node_id: node.id(),
        name,
        arguments,
    }))
}

pub fn build_event_definition(node: &Rc<NonterminalNode>) -> Result<EventDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::EventDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::EventKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    let parameters = build_event_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    let anonymous_keyword = if helper.at_label(EdgeLabel::AnonymousKeyword) {
        Some(terminal_node_cloned(
            helper.accept_label(EdgeLabel::AnonymousKeyword)?,
        )?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(EventDefinitionStruct {
        node_id: node.id(),
        name,
        parameters,
        anonymous_keyword,
    }))
}

pub fn build_event_parameters_declaration(
    node: &Rc<NonterminalNode>,
) -> Result<EventParametersDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::EventParametersDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let parameters = build_event_parameters(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(EventParametersDeclarationStruct {
        node_id: node.id(),
        parameters,
    }))
}

pub fn build_event_parameter(node: &Rc<NonterminalNode>) -> Result<EventParameter> {
    expect_nonterminal_kind(node, NonterminalKind::EventParameter)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    let indexed_keyword = if helper.at_label(EdgeLabel::IndexedKeyword) {
        Some(terminal_node_cloned(
            helper.accept_label(EdgeLabel::IndexedKeyword)?,
        )?)
    } else {
        None
    };
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(EventParameterStruct {
        node_id: node.id(),
        type_name,
        indexed_keyword,
        name,
    }))
}

pub fn build_user_defined_value_type_definition(
    node: &Rc<NonterminalNode>,
) -> Result<UserDefinedValueTypeDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::UserDefinedValueTypeDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::TypeKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    _ = helper.accept_label(EdgeLabel::IsKeyword)?;
    let value_type = build_elementary_type(nonterminal_node(
        helper.accept_label(EdgeLabel::ValueType)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(UserDefinedValueTypeDefinitionStruct {
        node_id: node.id(),
        name,
        value_type,
    }))
}

pub fn build_error_definition(node: &Rc<NonterminalNode>) -> Result<ErrorDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::ErrorDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ErrorKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    let members = build_error_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Members)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(ErrorDefinitionStruct {
        node_id: node.id(),
        name,
        members,
    }))
}

pub fn build_error_parameters_declaration(
    node: &Rc<NonterminalNode>,
) -> Result<ErrorParametersDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::ErrorParametersDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let parameters = build_error_parameters(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(ErrorParametersDeclarationStruct {
        node_id: node.id(),
        parameters,
    }))
}

pub fn build_error_parameter(node: &Rc<NonterminalNode>) -> Result<ErrorParameter> {
    expect_nonterminal_kind(node, NonterminalKind::ErrorParameter)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ErrorParameterStruct {
        node_id: node.id(),
        type_name,
        name,
    }))
}

pub fn build_array_type_name(node: &Rc<NonterminalNode>) -> Result<ArrayTypeName> {
    expect_nonterminal_kind(node, NonterminalKind::ArrayTypeName)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operand = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::Operand)?)?)?;
    _ = helper.accept_label(EdgeLabel::OpenBracket)?;
    let index = if helper.at_label(EdgeLabel::Index) {
        Some(build_expression(nonterminal_node(
            helper.accept_label(EdgeLabel::Index)?,
        )?)?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::CloseBracket)?;
    helper.finalize()?;

    Ok(Rc::new(ArrayTypeNameStruct {
        node_id: node.id(),
        operand,
        index,
    }))
}

pub fn build_function_type(node: &Rc<NonterminalNode>) -> Result<FunctionType> {
    expect_nonterminal_kind(node, NonterminalKind::FunctionType)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::FunctionKeyword)?;
    let parameters = build_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    let attributes = build_function_type_attributes(nonterminal_node(
        helper.accept_label(EdgeLabel::Attributes)?,
    )?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_returns_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(FunctionTypeStruct {
        node_id: node.id(),
        parameters,
        attributes,
        returns,
    }))
}

pub fn build_mapping_type(node: &Rc<NonterminalNode>) -> Result<MappingType> {
    expect_nonterminal_kind(node, NonterminalKind::MappingType)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::MappingKeyword)?;
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let key_type = build_mapping_key(nonterminal_node(helper.accept_label(EdgeLabel::KeyType)?)?)?;
    _ = helper.accept_label(EdgeLabel::EqualGreaterThan)?;
    let value_type = build_mapping_value(nonterminal_node(
        helper.accept_label(EdgeLabel::ValueType)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(MappingTypeStruct {
        node_id: node.id(),
        key_type,
        value_type,
    }))
}

pub fn build_mapping_key(node: &Rc<NonterminalNode>) -> Result<MappingKey> {
    expect_nonterminal_kind(node, NonterminalKind::MappingKey)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let key_type =
        build_mapping_key_type(nonterminal_node(helper.accept_label(EdgeLabel::KeyType)?)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(MappingKeyStruct {
        node_id: node.id(),
        key_type,
        name,
    }))
}

pub fn build_mapping_value(node: &Rc<NonterminalNode>) -> Result<MappingValue> {
    expect_nonterminal_kind(node, NonterminalKind::MappingValue)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(MappingValueStruct {
        node_id: node.id(),
        type_name,
        name,
    }))
}

pub fn build_address_type(node: &Rc<NonterminalNode>) -> Result<AddressType> {
    expect_nonterminal_kind(node, NonterminalKind::AddressType)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::AddressKeyword)?;
    let payable_keyword = if helper.at_label(EdgeLabel::PayableKeyword) {
        Some(terminal_node_cloned(
            helper.accept_label(EdgeLabel::PayableKeyword)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(AddressTypeStruct {
        node_id: node.id(),
        payable_keyword,
    }))
}

pub fn build_block(node: &Rc<NonterminalNode>) -> Result<Block> {
    expect_nonterminal_kind(node, NonterminalKind::Block)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let statements = build_statements(nonterminal_node(
        helper.accept_label(EdgeLabel::Statements)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(BlockStruct {
        node_id: node.id(),
        statements,
    }))
}

pub fn build_unchecked_block(node: &Rc<NonterminalNode>) -> Result<UncheckedBlock> {
    expect_nonterminal_kind(node, NonterminalKind::UncheckedBlock)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::UncheckedKeyword)?;
    let block = build_block(nonterminal_node(helper.accept_label(EdgeLabel::Block)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(UncheckedBlockStruct {
        node_id: node.id(),
        block,
    }))
}

pub fn build_expression_statement(node: &Rc<NonterminalNode>) -> Result<ExpressionStatement> {
    expect_nonterminal_kind(node, NonterminalKind::ExpressionStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let expression = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Expression)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(ExpressionStatementStruct {
        node_id: node.id(),
        expression,
    }))
}

pub fn build_assembly_statement(node: &Rc<NonterminalNode>) -> Result<AssemblyStatement> {
    expect_nonterminal_kind(node, NonterminalKind::AssemblyStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::AssemblyKeyword)?;
    let label = if helper.at_label(EdgeLabel::Label) {
        Some(build_string_literal(nonterminal_node(
            helper.accept_label(EdgeLabel::Label)?,
        )?)?)
    } else {
        None
    };
    let flags = if helper.at_label(EdgeLabel::Flags) {
        Some(build_assembly_flags_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Flags)?,
        )?)?)
    } else {
        None
    };
    let body = build_yul_block(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(AssemblyStatementStruct {
        node_id: node.id(),
        label,
        flags,
        body,
    }))
}

pub fn build_assembly_flags_declaration(
    node: &Rc<NonterminalNode>,
) -> Result<AssemblyFlagsDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::AssemblyFlagsDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let flags = build_assembly_flags(nonterminal_node(helper.accept_label(EdgeLabel::Flags)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(AssemblyFlagsDeclarationStruct {
        node_id: node.id(),
        flags,
    }))
}

pub fn build_tuple_deconstruction_statement(
    node: &Rc<NonterminalNode>,
) -> Result<TupleDeconstructionStatement> {
    expect_nonterminal_kind(node, NonterminalKind::TupleDeconstructionStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let var_keyword = if helper.at_label(EdgeLabel::VarKeyword) {
        Some(terminal_node_cloned(
            helper.accept_label(EdgeLabel::VarKeyword)?,
        )?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let elements = build_tuple_deconstruction_elements(nonterminal_node(
        helper.accept_label(EdgeLabel::Elements)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    _ = helper.accept_label(EdgeLabel::Equal)?;
    let expression = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Expression)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(TupleDeconstructionStatementStruct {
        node_id: node.id(),
        var_keyword,
        elements,
        expression,
    }))
}

pub fn build_tuple_deconstruction_element(
    node: &Rc<NonterminalNode>,
) -> Result<TupleDeconstructionElement> {
    expect_nonterminal_kind(node, NonterminalKind::TupleDeconstructionElement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let member = if helper.at_label(EdgeLabel::Member) {
        Some(build_tuple_member(nonterminal_node(
            helper.accept_label(EdgeLabel::Member)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(TupleDeconstructionElementStruct {
        node_id: node.id(),
        member,
    }))
}

pub fn build_typed_tuple_member(node: &Rc<NonterminalNode>) -> Result<TypedTupleMember> {
    expect_nonterminal_kind(node, NonterminalKind::TypedTupleMember)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    let storage_location = if helper.at_label(EdgeLabel::StorageLocation) {
        Some(build_storage_location(nonterminal_node(
            helper.accept_label(EdgeLabel::StorageLocation)?,
        )?)?)
    } else {
        None
    };
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    helper.finalize()?;

    Ok(Rc::new(TypedTupleMemberStruct {
        node_id: node.id(),
        type_name,
        storage_location,
        name,
    }))
}

pub fn build_untyped_tuple_member(node: &Rc<NonterminalNode>) -> Result<UntypedTupleMember> {
    expect_nonterminal_kind(node, NonterminalKind::UntypedTupleMember)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let storage_location = if helper.at_label(EdgeLabel::StorageLocation) {
        Some(build_storage_location(nonterminal_node(
            helper.accept_label(EdgeLabel::StorageLocation)?,
        )?)?)
    } else {
        None
    };
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    helper.finalize()?;

    Ok(Rc::new(UntypedTupleMemberStruct {
        node_id: node.id(),
        storage_location,
        name,
    }))
}

pub fn build_variable_declaration_statement(
    node: &Rc<NonterminalNode>,
) -> Result<VariableDeclarationStatement> {
    expect_nonterminal_kind(node, NonterminalKind::VariableDeclarationStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variable_type = build_variable_declaration_type(nonterminal_node(
        helper.accept_label(EdgeLabel::VariableType)?,
    )?)?;
    let storage_location = if helper.at_label(EdgeLabel::StorageLocation) {
        Some(build_storage_location(nonterminal_node(
            helper.accept_label(EdgeLabel::StorageLocation)?,
        )?)?)
    } else {
        None
    };
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    let value = if helper.at_label(EdgeLabel::Value) {
        Some(build_variable_declaration_value(nonterminal_node(
            helper.accept_label(EdgeLabel::Value)?,
        )?)?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(VariableDeclarationStatementStruct {
        node_id: node.id(),
        variable_type,
        storage_location,
        name,
        value,
    }))
}

pub fn build_variable_declaration_value(
    node: &Rc<NonterminalNode>,
) -> Result<VariableDeclarationValue> {
    expect_nonterminal_kind(node, NonterminalKind::VariableDeclarationValue)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::Equal)?;
    let expression = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Expression)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(VariableDeclarationValueStruct {
        node_id: node.id(),
        expression,
    }))
}

pub fn build_if_statement(node: &Rc<NonterminalNode>) -> Result<IfStatement> {
    expect_nonterminal_kind(node, NonterminalKind::IfStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::IfKeyword)?;
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let condition = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Condition)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    let body = build_statement(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    let else_branch = if helper.at_label(EdgeLabel::ElseBranch) {
        Some(build_else_branch(nonterminal_node(
            helper.accept_label(EdgeLabel::ElseBranch)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(IfStatementStruct {
        node_id: node.id(),
        condition,
        body,
        else_branch,
    }))
}

pub fn build_else_branch(node: &Rc<NonterminalNode>) -> Result<ElseBranch> {
    expect_nonterminal_kind(node, NonterminalKind::ElseBranch)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ElseKeyword)?;
    let body = build_statement(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(ElseBranchStruct {
        node_id: node.id(),
        body,
    }))
}

pub fn build_for_statement(node: &Rc<NonterminalNode>) -> Result<ForStatement> {
    expect_nonterminal_kind(node, NonterminalKind::ForStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ForKeyword)?;
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let initialization = build_for_statement_initialization(nonterminal_node(
        helper.accept_label(EdgeLabel::Initialization)?,
    )?)?;
    let condition = build_for_statement_condition(nonterminal_node(
        helper.accept_label(EdgeLabel::Condition)?,
    )?)?;
    let iterator = if helper.at_label(EdgeLabel::Iterator) {
        Some(build_expression(nonterminal_node(
            helper.accept_label(EdgeLabel::Iterator)?,
        )?)?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    let body = build_statement(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(ForStatementStruct {
        node_id: node.id(),
        initialization,
        condition,
        iterator,
        body,
    }))
}

pub fn build_while_statement(node: &Rc<NonterminalNode>) -> Result<WhileStatement> {
    expect_nonterminal_kind(node, NonterminalKind::WhileStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::WhileKeyword)?;
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let condition = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Condition)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    let body = build_statement(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(WhileStatementStruct {
        node_id: node.id(),
        condition,
        body,
    }))
}

pub fn build_do_while_statement(node: &Rc<NonterminalNode>) -> Result<DoWhileStatement> {
    expect_nonterminal_kind(node, NonterminalKind::DoWhileStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::DoKeyword)?;
    let body = build_statement(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    _ = helper.accept_label(EdgeLabel::WhileKeyword)?;
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let condition = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Condition)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(DoWhileStatementStruct {
        node_id: node.id(),
        body,
        condition,
    }))
}

pub fn build_continue_statement(node: &Rc<NonterminalNode>) -> Result<ContinueStatement> {
    expect_nonterminal_kind(node, NonterminalKind::ContinueStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ContinueKeyword)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(ContinueStatementStruct { node_id: node.id() }))
}

pub fn build_break_statement(node: &Rc<NonterminalNode>) -> Result<BreakStatement> {
    expect_nonterminal_kind(node, NonterminalKind::BreakStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::BreakKeyword)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(BreakStatementStruct { node_id: node.id() }))
}

pub fn build_return_statement(node: &Rc<NonterminalNode>) -> Result<ReturnStatement> {
    expect_nonterminal_kind(node, NonterminalKind::ReturnStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ReturnKeyword)?;
    let expression = if helper.at_label(EdgeLabel::Expression) {
        Some(build_expression(nonterminal_node(
            helper.accept_label(EdgeLabel::Expression)?,
        )?)?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(ReturnStatementStruct {
        node_id: node.id(),
        expression,
    }))
}

pub fn build_emit_statement(node: &Rc<NonterminalNode>) -> Result<EmitStatement> {
    expect_nonterminal_kind(node, NonterminalKind::EmitStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::EmitKeyword)?;
    let event = build_identifier_path(nonterminal_node(helper.accept_label(EdgeLabel::Event)?)?)?;
    let arguments = build_arguments_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Arguments)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(EmitStatementStruct {
        node_id: node.id(),
        event,
        arguments,
    }))
}

pub fn build_try_statement(node: &Rc<NonterminalNode>) -> Result<TryStatement> {
    expect_nonterminal_kind(node, NonterminalKind::TryStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::TryKeyword)?;
    let expression = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Expression)?,
    )?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_returns_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)?)
    } else {
        None
    };
    let body = build_block(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    let catch_clauses = build_catch_clauses(nonterminal_node(
        helper.accept_label(EdgeLabel::CatchClauses)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(TryStatementStruct {
        node_id: node.id(),
        expression,
        returns,
        body,
        catch_clauses,
    }))
}

pub fn build_catch_clause(node: &Rc<NonterminalNode>) -> Result<CatchClause> {
    expect_nonterminal_kind(node, NonterminalKind::CatchClause)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::CatchKeyword)?;
    let error = if helper.at_label(EdgeLabel::Error) {
        Some(build_catch_clause_error(nonterminal_node(
            helper.accept_label(EdgeLabel::Error)?,
        )?)?)
    } else {
        None
    };
    let body = build_block(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(CatchClauseStruct {
        node_id: node.id(),
        error,
        body,
    }))
}

pub fn build_catch_clause_error(node: &Rc<NonterminalNode>) -> Result<CatchClauseError> {
    expect_nonterminal_kind(node, NonterminalKind::CatchClauseError)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    let parameters = build_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(CatchClauseErrorStruct {
        node_id: node.id(),
        name,
        parameters,
    }))
}

pub fn build_revert_statement(node: &Rc<NonterminalNode>) -> Result<RevertStatement> {
    expect_nonterminal_kind(node, NonterminalKind::RevertStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::RevertKeyword)?;
    let error = if helper.at_label(EdgeLabel::Error) {
        Some(build_identifier_path(nonterminal_node(
            helper.accept_label(EdgeLabel::Error)?,
        )?)?)
    } else {
        None
    };
    let arguments = build_arguments_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Arguments)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(RevertStatementStruct {
        node_id: node.id(),
        error,
        arguments,
    }))
}

pub fn build_throw_statement(node: &Rc<NonterminalNode>) -> Result<ThrowStatement> {
    expect_nonterminal_kind(node, NonterminalKind::ThrowStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ThrowKeyword)?;
    _ = helper.accept_label(EdgeLabel::Semicolon)?;
    helper.finalize()?;

    Ok(Rc::new(ThrowStatementStruct { node_id: node.id() }))
}

pub fn build_assignment_expression(node: &Rc<NonterminalNode>) -> Result<AssignmentExpression> {
    expect_nonterminal_kind(node, NonterminalKind::AssignmentExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(AssignmentExpressionStruct {
        node_id: node.id(),
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_conditional_expression(node: &Rc<NonterminalNode>) -> Result<ConditionalExpression> {
    expect_nonterminal_kind(node, NonterminalKind::ConditionalExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operand = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Operand)?)?)?;
    _ = helper.accept_label(EdgeLabel::QuestionMark)?;
    let true_expression = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::TrueExpression)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Colon)?;
    let false_expression = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::FalseExpression)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(ConditionalExpressionStruct {
        node_id: node.id(),
        operand,
        true_expression,
        false_expression,
    }))
}

pub fn build_or_expression(node: &Rc<NonterminalNode>) -> Result<OrExpression> {
    expect_nonterminal_kind(node, NonterminalKind::OrExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(OrExpressionStruct {
        node_id: node.id(),
        left_operand,
        right_operand,
    }))
}

pub fn build_and_expression(node: &Rc<NonterminalNode>) -> Result<AndExpression> {
    expect_nonterminal_kind(node, NonterminalKind::AndExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(AndExpressionStruct {
        node_id: node.id(),
        left_operand,
        right_operand,
    }))
}

pub fn build_equality_expression(node: &Rc<NonterminalNode>) -> Result<EqualityExpression> {
    expect_nonterminal_kind(node, NonterminalKind::EqualityExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(EqualityExpressionStruct {
        node_id: node.id(),
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_inequality_expression(node: &Rc<NonterminalNode>) -> Result<InequalityExpression> {
    expect_nonterminal_kind(node, NonterminalKind::InequalityExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(InequalityExpressionStruct {
        node_id: node.id(),
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_bitwise_or_expression(node: &Rc<NonterminalNode>) -> Result<BitwiseOrExpression> {
    expect_nonterminal_kind(node, NonterminalKind::BitwiseOrExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(BitwiseOrExpressionStruct {
        node_id: node.id(),
        left_operand,
        right_operand,
    }))
}

pub fn build_bitwise_xor_expression(node: &Rc<NonterminalNode>) -> Result<BitwiseXorExpression> {
    expect_nonterminal_kind(node, NonterminalKind::BitwiseXorExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(BitwiseXorExpressionStruct {
        node_id: node.id(),
        left_operand,
        right_operand,
    }))
}

pub fn build_bitwise_and_expression(node: &Rc<NonterminalNode>) -> Result<BitwiseAndExpression> {
    expect_nonterminal_kind(node, NonterminalKind::BitwiseAndExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::Operator)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(BitwiseAndExpressionStruct {
        node_id: node.id(),
        left_operand,
        right_operand,
    }))
}

pub fn build_shift_expression(node: &Rc<NonterminalNode>) -> Result<ShiftExpression> {
    expect_nonterminal_kind(node, NonterminalKind::ShiftExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(ShiftExpressionStruct {
        node_id: node.id(),
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_additive_expression(node: &Rc<NonterminalNode>) -> Result<AdditiveExpression> {
    expect_nonterminal_kind(node, NonterminalKind::AdditiveExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(AdditiveExpressionStruct {
        node_id: node.id(),
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_multiplicative_expression(
    node: &Rc<NonterminalNode>,
) -> Result<MultiplicativeExpression> {
    expect_nonterminal_kind(node, NonterminalKind::MultiplicativeExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(MultiplicativeExpressionStruct {
        node_id: node.id(),
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_exponentiation_expression(
    node: &Rc<NonterminalNode>,
) -> Result<ExponentiationExpression> {
    expect_nonterminal_kind(node, NonterminalKind::ExponentiationExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let left_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::LeftOperand)?,
    )?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::RightOperand)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(ExponentiationExpressionStruct {
        node_id: node.id(),
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_postfix_expression(node: &Rc<NonterminalNode>) -> Result<PostfixExpression> {
    expect_nonterminal_kind(node, NonterminalKind::PostfixExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operand = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Operand)?)?)?;
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?)?;
    helper.finalize()?;

    Ok(Rc::new(PostfixExpressionStruct {
        node_id: node.id(),
        operand,
        operator,
    }))
}

pub fn build_prefix_expression(node: &Rc<NonterminalNode>) -> Result<PrefixExpression> {
    expect_nonterminal_kind(node, NonterminalKind::PrefixExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operator = terminal_node_cloned(helper.accept_label(EdgeLabel::Operator)?)?;
    let operand = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Operand)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(PrefixExpressionStruct {
        node_id: node.id(),
        operator,
        operand,
    }))
}

pub fn build_function_call_expression(
    node: &Rc<NonterminalNode>,
) -> Result<FunctionCallExpression> {
    expect_nonterminal_kind(node, NonterminalKind::FunctionCallExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operand = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Operand)?)?)?;
    let arguments = build_arguments_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Arguments)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(FunctionCallExpressionStruct {
        node_id: node.id(),
        operand,
        arguments,
    }))
}

pub fn build_call_options_expression(node: &Rc<NonterminalNode>) -> Result<CallOptionsExpression> {
    expect_nonterminal_kind(node, NonterminalKind::CallOptionsExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operand = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Operand)?)?)?;
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let options = build_call_options(nonterminal_node(helper.accept_label(EdgeLabel::Options)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(CallOptionsExpressionStruct {
        node_id: node.id(),
        operand,
        options,
    }))
}

pub fn build_member_access_expression(
    node: &Rc<NonterminalNode>,
) -> Result<MemberAccessExpression> {
    expect_nonterminal_kind(node, NonterminalKind::MemberAccessExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operand = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Operand)?)?)?;
    _ = helper.accept_label(EdgeLabel::Period)?;
    let member = terminal_node_cloned(helper.accept_label(EdgeLabel::Member)?)?;
    helper.finalize()?;

    Ok(Rc::new(MemberAccessExpressionStruct {
        node_id: node.id(),
        operand,
        member,
    }))
}

pub fn build_index_access_expression(node: &Rc<NonterminalNode>) -> Result<IndexAccessExpression> {
    expect_nonterminal_kind(node, NonterminalKind::IndexAccessExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operand = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Operand)?)?)?;
    _ = helper.accept_label(EdgeLabel::OpenBracket)?;
    let start = if helper.at_label(EdgeLabel::Start) {
        Some(build_expression(nonterminal_node(
            helper.accept_label(EdgeLabel::Start)?,
        )?)?)
    } else {
        None
    };
    let end = if helper.at_label(EdgeLabel::End) {
        Some(build_index_access_end(nonterminal_node(
            helper.accept_label(EdgeLabel::End)?,
        )?)?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::CloseBracket)?;
    helper.finalize()?;

    Ok(Rc::new(IndexAccessExpressionStruct {
        node_id: node.id(),
        operand,
        start,
        end,
    }))
}

pub fn build_index_access_end(node: &Rc<NonterminalNode>) -> Result<IndexAccessEnd> {
    expect_nonterminal_kind(node, NonterminalKind::IndexAccessEnd)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::Colon)?;
    let end = if helper.at_label(EdgeLabel::End) {
        Some(build_expression(nonterminal_node(
            helper.accept_label(EdgeLabel::End)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(IndexAccessEndStruct {
        node_id: node.id(),
        end,
    }))
}

pub fn build_positional_arguments_declaration(
    node: &Rc<NonterminalNode>,
) -> Result<PositionalArgumentsDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::PositionalArgumentsDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let arguments = build_positional_arguments(nonterminal_node(
        helper.accept_label(EdgeLabel::Arguments)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(PositionalArgumentsDeclarationStruct {
        node_id: node.id(),
        arguments,
    }))
}

pub fn build_named_arguments_declaration(
    node: &Rc<NonterminalNode>,
) -> Result<NamedArgumentsDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::NamedArgumentsDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let arguments = if helper.at_label(EdgeLabel::Arguments) {
        Some(build_named_argument_group(nonterminal_node(
            helper.accept_label(EdgeLabel::Arguments)?,
        )?)?)
    } else {
        None
    };
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(NamedArgumentsDeclarationStruct {
        node_id: node.id(),
        arguments,
    }))
}

pub fn build_named_argument_group(node: &Rc<NonterminalNode>) -> Result<NamedArgumentGroup> {
    expect_nonterminal_kind(node, NonterminalKind::NamedArgumentGroup)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let arguments = build_named_arguments(nonterminal_node(
        helper.accept_label(EdgeLabel::Arguments)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(NamedArgumentGroupStruct {
        node_id: node.id(),
        arguments,
    }))
}

pub fn build_named_argument(node: &Rc<NonterminalNode>) -> Result<NamedArgument> {
    expect_nonterminal_kind(node, NonterminalKind::NamedArgument)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    _ = helper.accept_label(EdgeLabel::Colon)?;
    let value = build_expression(nonterminal_node(helper.accept_label(EdgeLabel::Value)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedArgumentStruct {
        node_id: node.id(),
        name,
        value,
    }))
}

pub fn build_type_expression(node: &Rc<NonterminalNode>) -> Result<TypeExpression> {
    expect_nonterminal_kind(node, NonterminalKind::TypeExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::TypeKeyword)?;
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(TypeExpressionStruct {
        node_id: node.id(),
        type_name,
    }))
}

pub fn build_new_expression(node: &Rc<NonterminalNode>) -> Result<NewExpression> {
    expect_nonterminal_kind(node, NonterminalKind::NewExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::NewKeyword)?;
    let type_name = build_type_name(nonterminal_node(helper.accept_label(EdgeLabel::TypeName)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(NewExpressionStruct {
        node_id: node.id(),
        type_name,
    }))
}

pub fn build_tuple_expression(node: &Rc<NonterminalNode>) -> Result<TupleExpression> {
    expect_nonterminal_kind(node, NonterminalKind::TupleExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let items = build_tuple_values(nonterminal_node(helper.accept_label(EdgeLabel::Items)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(TupleExpressionStruct {
        node_id: node.id(),
        items,
    }))
}

pub fn build_tuple_value(node: &Rc<NonterminalNode>) -> Result<TupleValue> {
    expect_nonterminal_kind(node, NonterminalKind::TupleValue)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let expression = if helper.at_label(EdgeLabel::Expression) {
        Some(build_expression(nonterminal_node(
            helper.accept_label(EdgeLabel::Expression)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(TupleValueStruct {
        node_id: node.id(),
        expression,
    }))
}

pub fn build_array_expression(node: &Rc<NonterminalNode>) -> Result<ArrayExpression> {
    expect_nonterminal_kind(node, NonterminalKind::ArrayExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenBracket)?;
    let items = build_array_values(nonterminal_node(helper.accept_label(EdgeLabel::Items)?)?)?;
    _ = helper.accept_label(EdgeLabel::CloseBracket)?;
    helper.finalize()?;

    Ok(Rc::new(ArrayExpressionStruct {
        node_id: node.id(),
        items,
    }))
}

pub fn build_hex_number_expression(node: &Rc<NonterminalNode>) -> Result<HexNumberExpression> {
    expect_nonterminal_kind(node, NonterminalKind::HexNumberExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let literal = terminal_node_cloned(helper.accept_label(EdgeLabel::Literal)?)?;
    let unit = if helper.at_label(EdgeLabel::Unit) {
        Some(build_number_unit(nonterminal_node(
            helper.accept_label(EdgeLabel::Unit)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(HexNumberExpressionStruct {
        node_id: node.id(),
        literal,
        unit,
    }))
}

pub fn build_decimal_number_expression(
    node: &Rc<NonterminalNode>,
) -> Result<DecimalNumberExpression> {
    expect_nonterminal_kind(node, NonterminalKind::DecimalNumberExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let literal = terminal_node_cloned(helper.accept_label(EdgeLabel::Literal)?)?;
    let unit = if helper.at_label(EdgeLabel::Unit) {
        Some(build_number_unit(nonterminal_node(
            helper.accept_label(EdgeLabel::Unit)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(DecimalNumberExpressionStruct {
        node_id: node.id(),
        literal,
        unit,
    }))
}

pub fn build_yul_block(node: &Rc<NonterminalNode>) -> Result<YulBlock> {
    expect_nonterminal_kind(node, NonterminalKind::YulBlock)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenBrace)?;
    let statements = build_yul_statements(nonterminal_node(
        helper.accept_label(EdgeLabel::Statements)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseBrace)?;
    helper.finalize()?;

    Ok(Rc::new(YulBlockStruct {
        node_id: node.id(),
        statements,
    }))
}

pub fn build_yul_function_definition(node: &Rc<NonterminalNode>) -> Result<YulFunctionDefinition> {
    expect_nonterminal_kind(node, NonterminalKind::YulFunctionDefinition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::FunctionKeyword)?;
    let name = terminal_node_cloned(helper.accept_label(EdgeLabel::Name)?)?;
    let parameters = build_yul_parameters_declaration(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_yul_returns_declaration(nonterminal_node(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)?)
    } else {
        None
    };
    let body = build_yul_block(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulFunctionDefinitionStruct {
        node_id: node.id(),
        name,
        parameters,
        returns,
        body,
    }))
}

pub fn build_yul_parameters_declaration(
    node: &Rc<NonterminalNode>,
) -> Result<YulParametersDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::YulParametersDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let parameters = build_yul_parameters(nonterminal_node(
        helper.accept_label(EdgeLabel::Parameters)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(YulParametersDeclarationStruct {
        node_id: node.id(),
        parameters,
    }))
}

pub fn build_yul_returns_declaration(node: &Rc<NonterminalNode>) -> Result<YulReturnsDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::YulReturnsDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::MinusGreaterThan)?;
    let variables = build_yul_variable_names(nonterminal_node(
        helper.accept_label(EdgeLabel::Variables)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(YulReturnsDeclarationStruct {
        node_id: node.id(),
        variables,
    }))
}

pub fn build_yul_variable_declaration_statement(
    node: &Rc<NonterminalNode>,
) -> Result<YulVariableDeclarationStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulVariableDeclarationStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::LetKeyword)?;
    let variables = build_yul_variable_names(nonterminal_node(
        helper.accept_label(EdgeLabel::Variables)?,
    )?)?;
    let value = if helper.at_label(EdgeLabel::Value) {
        Some(build_yul_variable_declaration_value(nonterminal_node(
            helper.accept_label(EdgeLabel::Value)?,
        )?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(YulVariableDeclarationStatementStruct {
        node_id: node.id(),
        variables,
        value,
    }))
}

pub fn build_yul_variable_declaration_value(
    node: &Rc<NonterminalNode>,
) -> Result<YulVariableDeclarationValue> {
    expect_nonterminal_kind(node, NonterminalKind::YulVariableDeclarationValue)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let assignment = build_yul_assignment_operator(nonterminal_node(
        helper.accept_label(EdgeLabel::Assignment)?,
    )?)?;
    let expression = build_yul_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Expression)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(YulVariableDeclarationValueStruct {
        node_id: node.id(),
        assignment,
        expression,
    }))
}

pub fn build_yul_variable_assignment_statement(
    node: &Rc<NonterminalNode>,
) -> Result<YulVariableAssignmentStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulVariableAssignmentStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variables = build_yul_paths(nonterminal_node(
        helper.accept_label(EdgeLabel::Variables)?,
    )?)?;
    let assignment = build_yul_assignment_operator(nonterminal_node(
        helper.accept_label(EdgeLabel::Assignment)?,
    )?)?;
    let expression = build_yul_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Expression)?,
    )?)?;
    helper.finalize()?;

    Ok(Rc::new(YulVariableAssignmentStatementStruct {
        node_id: node.id(),
        variables,
        assignment,
        expression,
    }))
}

pub fn build_yul_colon_and_equal(node: &Rc<NonterminalNode>) -> Result<YulColonAndEqual> {
    expect_nonterminal_kind(node, NonterminalKind::YulColonAndEqual)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::Colon)?;
    _ = helper.accept_label(EdgeLabel::Equal)?;
    helper.finalize()?;

    Ok(Rc::new(YulColonAndEqualStruct { node_id: node.id() }))
}

pub fn build_yul_stack_assignment_statement(
    node: &Rc<NonterminalNode>,
) -> Result<YulStackAssignmentStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulStackAssignmentStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let assignment = build_yul_stack_assignment_operator(nonterminal_node(
        helper.accept_label(EdgeLabel::Assignment)?,
    )?)?;
    let variable = terminal_node_cloned(helper.accept_label(EdgeLabel::Variable)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulStackAssignmentStatementStruct {
        node_id: node.id(),
        assignment,
        variable,
    }))
}

pub fn build_yul_equal_and_colon(node: &Rc<NonterminalNode>) -> Result<YulEqualAndColon> {
    expect_nonterminal_kind(node, NonterminalKind::YulEqualAndColon)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::Equal)?;
    _ = helper.accept_label(EdgeLabel::Colon)?;
    helper.finalize()?;

    Ok(Rc::new(YulEqualAndColonStruct { node_id: node.id() }))
}

pub fn build_yul_if_statement(node: &Rc<NonterminalNode>) -> Result<YulIfStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulIfStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::IfKeyword)?;
    let condition = build_yul_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Condition)?,
    )?)?;
    let body = build_yul_block(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulIfStatementStruct {
        node_id: node.id(),
        condition,
        body,
    }))
}

pub fn build_yul_for_statement(node: &Rc<NonterminalNode>) -> Result<YulForStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulForStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ForKeyword)?;
    let initialization = build_yul_block(nonterminal_node(
        helper.accept_label(EdgeLabel::Initialization)?,
    )?)?;
    let condition = build_yul_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Condition)?,
    )?)?;
    let iterator = build_yul_block(nonterminal_node(helper.accept_label(EdgeLabel::Iterator)?)?)?;
    let body = build_yul_block(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulForStatementStruct {
        node_id: node.id(),
        initialization,
        condition,
        iterator,
        body,
    }))
}

pub fn build_yul_switch_statement(node: &Rc<NonterminalNode>) -> Result<YulSwitchStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulSwitchStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::SwitchKeyword)?;
    let expression = build_yul_expression(nonterminal_node(
        helper.accept_label(EdgeLabel::Expression)?,
    )?)?;
    let cases = build_yul_switch_cases(nonterminal_node(helper.accept_label(EdgeLabel::Cases)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulSwitchStatementStruct {
        node_id: node.id(),
        expression,
        cases,
    }))
}

pub fn build_yul_default_case(node: &Rc<NonterminalNode>) -> Result<YulDefaultCase> {
    expect_nonterminal_kind(node, NonterminalKind::YulDefaultCase)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::DefaultKeyword)?;
    let body = build_yul_block(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulDefaultCaseStruct {
        node_id: node.id(),
        body,
    }))
}

pub fn build_yul_value_case(node: &Rc<NonterminalNode>) -> Result<YulValueCase> {
    expect_nonterminal_kind(node, NonterminalKind::YulValueCase)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::CaseKeyword)?;
    let value = build_yul_literal(nonterminal_node(helper.accept_label(EdgeLabel::Value)?)?)?;
    let body = build_yul_block(nonterminal_node(helper.accept_label(EdgeLabel::Body)?)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulValueCaseStruct {
        node_id: node.id(),
        value,
        body,
    }))
}

pub fn build_yul_leave_statement(node: &Rc<NonterminalNode>) -> Result<YulLeaveStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulLeaveStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::LeaveKeyword)?;
    helper.finalize()?;

    Ok(Rc::new(YulLeaveStatementStruct { node_id: node.id() }))
}

pub fn build_yul_break_statement(node: &Rc<NonterminalNode>) -> Result<YulBreakStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulBreakStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::BreakKeyword)?;
    helper.finalize()?;

    Ok(Rc::new(YulBreakStatementStruct { node_id: node.id() }))
}

pub fn build_yul_continue_statement(node: &Rc<NonterminalNode>) -> Result<YulContinueStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulContinueStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    _ = helper.accept_label(EdgeLabel::ContinueKeyword)?;
    helper.finalize()?;

    Ok(Rc::new(YulContinueStatementStruct { node_id: node.id() }))
}

pub fn build_yul_label(node: &Rc<NonterminalNode>) -> Result<YulLabel> {
    expect_nonterminal_kind(node, NonterminalKind::YulLabel)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let label = terminal_node_cloned(helper.accept_label(EdgeLabel::Label)?)?;
    _ = helper.accept_label(EdgeLabel::Colon)?;
    helper.finalize()?;

    Ok(Rc::new(YulLabelStruct {
        node_id: node.id(),
        label,
    }))
}

pub fn build_yul_function_call_expression(
    node: &Rc<NonterminalNode>,
) -> Result<YulFunctionCallExpression> {
    expect_nonterminal_kind(node, NonterminalKind::YulFunctionCallExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let operand =
        build_yul_expression(nonterminal_node(helper.accept_label(EdgeLabel::Operand)?)?)?;
    _ = helper.accept_label(EdgeLabel::OpenParen)?;
    let arguments = build_yul_arguments(nonterminal_node(
        helper.accept_label(EdgeLabel::Arguments)?,
    )?)?;
    _ = helper.accept_label(EdgeLabel::CloseParen)?;
    helper.finalize()?;

    Ok(Rc::new(YulFunctionCallExpressionStruct {
        node_id: node.id(),
        operand,
        arguments,
    }))
}

//
// Choices:
//

pub fn build_source_unit_member(node: &Rc<NonterminalNode>) -> Result<SourceUnitMember> {
    expect_nonterminal_kind(node, NonterminalKind::SourceUnitMember)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::PragmaDirective) => {
            SourceUnitMember::PragmaDirective(build_pragma_directive(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ImportDirective) => {
            SourceUnitMember::ImportDirective(build_import_directive(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ContractDefinition) => {
            SourceUnitMember::ContractDefinition(build_contract_definition(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::InterfaceDefinition) => {
            SourceUnitMember::InterfaceDefinition(build_interface_definition(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::LibraryDefinition) => {
            SourceUnitMember::LibraryDefinition(build_library_definition(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StructDefinition) => {
            SourceUnitMember::StructDefinition(build_struct_definition(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EnumDefinition) => {
            SourceUnitMember::EnumDefinition(build_enum_definition(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionDefinition) => {
            SourceUnitMember::FunctionDefinition(build_function_definition(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ErrorDefinition) => {
            SourceUnitMember::ErrorDefinition(build_error_definition(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UserDefinedValueTypeDefinition) => {
            SourceUnitMember::UserDefinedValueTypeDefinition(
                build_user_defined_value_type_definition(nonterminal_node(variant)?)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::UsingDirective) => {
            SourceUnitMember::UsingDirective(build_using_directive(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EventDefinition) => {
            SourceUnitMember::EventDefinition(build_event_definition(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ConstantDefinition) => {
            SourceUnitMember::ConstantDefinition(build_constant_definition(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_pragma(node: &Rc<NonterminalNode>) -> Result<Pragma> {
    expect_nonterminal_kind(node, NonterminalKind::Pragma)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::AbicoderPragma) => {
            Pragma::AbicoderPragma(build_abicoder_pragma(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ExperimentalPragma) => {
            Pragma::ExperimentalPragma(build_experimental_pragma(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::VersionPragma) => {
            Pragma::VersionPragma(build_version_pragma(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_experimental_feature(node: &Rc<NonterminalNode>) -> Result<ExperimentalFeature> {
    expect_nonterminal_kind(node, NonterminalKind::ExperimentalFeature)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::StringLiteral) => {
            ExperimentalFeature::StringLiteral(build_string_literal(nonterminal_node(variant)?)?)
        }
        NodeKind::Terminal(TerminalKind::Identifier) => {
            ExperimentalFeature::Identifier(terminal_node_cloned(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_version_expression(node: &Rc<NonterminalNode>) -> Result<VersionExpression> {
    expect_nonterminal_kind(node, NonterminalKind::VersionExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::VersionRange) => {
            VersionExpression::VersionRange(build_version_range(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::VersionTerm) => {
            VersionExpression::VersionTerm(build_version_term(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_version_operator(node: &Rc<NonterminalNode>) -> Result<VersionOperator> {
    expect_nonterminal_kind(node, NonterminalKind::VersionOperator)?;
    let mut helper = ChildrenHelper::new(&node.children);
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
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_version_literal(node: &Rc<NonterminalNode>) -> Result<VersionLiteral> {
    expect_nonterminal_kind(node, NonterminalKind::VersionLiteral)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::SimpleVersionLiteral) => {
            VersionLiteral::SimpleVersionLiteral(build_simple_version_literal(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Terminal(TerminalKind::SingleQuotedVersionLiteral) => {
            VersionLiteral::SingleQuotedVersionLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedVersionLiteral) => {
            VersionLiteral::DoubleQuotedVersionLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_import_clause(node: &Rc<NonterminalNode>) -> Result<ImportClause> {
    expect_nonterminal_kind(node, NonterminalKind::ImportClause)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::PathImport) => {
            ImportClause::PathImport(build_path_import(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::NamedImport) => {
            ImportClause::NamedImport(build_named_import(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ImportDeconstruction) => {
            ImportClause::ImportDeconstruction(build_import_deconstruction(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_using_clause(node: &Rc<NonterminalNode>) -> Result<UsingClause> {
    expect_nonterminal_kind(node, NonterminalKind::UsingClause)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::IdentifierPath) => {
            UsingClause::IdentifierPath(build_identifier_path(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UsingDeconstruction) => {
            UsingClause::UsingDeconstruction(build_using_deconstruction(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_using_operator(node: &Rc<NonterminalNode>) -> Result<UsingOperator> {
    expect_nonterminal_kind(node, NonterminalKind::UsingOperator)?;
    let mut helper = ChildrenHelper::new(&node.children);
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
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_using_target(node: &Rc<NonterminalNode>) -> Result<UsingTarget> {
    expect_nonterminal_kind(node, NonterminalKind::UsingTarget)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::TypeName) => {
            UsingTarget::TypeName(build_type_name(nonterminal_node(variant)?)?)
        }
        NodeKind::Terminal(TerminalKind::Asterisk) => UsingTarget::Asterisk,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_contract_specifier(node: &Rc<NonterminalNode>) -> Result<ContractSpecifier> {
    expect_nonterminal_kind(node, NonterminalKind::ContractSpecifier)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::InheritanceSpecifier) => {
            ContractSpecifier::InheritanceSpecifier(build_inheritance_specifier(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StorageLayoutSpecifier) => {
            ContractSpecifier::StorageLayoutSpecifier(build_storage_layout_specifier(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_contract_member(node: &Rc<NonterminalNode>) -> Result<ContractMember> {
    expect_nonterminal_kind(node, NonterminalKind::ContractMember)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::UsingDirective) => {
            ContractMember::UsingDirective(build_using_directive(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionDefinition) => {
            ContractMember::FunctionDefinition(build_function_definition(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ConstructorDefinition) => {
            ContractMember::ConstructorDefinition(build_constructor_definition(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ReceiveFunctionDefinition) => {
            ContractMember::ReceiveFunctionDefinition(build_receive_function_definition(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::FallbackFunctionDefinition) => {
            ContractMember::FallbackFunctionDefinition(build_fallback_function_definition(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::UnnamedFunctionDefinition) => {
            ContractMember::UnnamedFunctionDefinition(build_unnamed_function_definition(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::ModifierDefinition) => {
            ContractMember::ModifierDefinition(build_modifier_definition(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StructDefinition) => {
            ContractMember::StructDefinition(build_struct_definition(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EnumDefinition) => {
            ContractMember::EnumDefinition(build_enum_definition(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EventDefinition) => {
            ContractMember::EventDefinition(build_event_definition(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ErrorDefinition) => {
            ContractMember::ErrorDefinition(build_error_definition(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UserDefinedValueTypeDefinition) => {
            ContractMember::UserDefinedValueTypeDefinition(
                build_user_defined_value_type_definition(nonterminal_node(variant)?)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::StateVariableDefinition) => {
            ContractMember::StateVariableDefinition(build_state_variable_definition(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_state_variable_attribute(
    node: &Rc<NonterminalNode>,
) -> Result<StateVariableAttribute> {
    expect_nonterminal_kind(node, NonterminalKind::StateVariableAttribute)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            StateVariableAttribute::OverrideSpecifier(build_override_specifier(nonterminal_node(
                variant,
            )?)?)
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
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_function_name(node: &Rc<NonterminalNode>) -> Result<FunctionName> {
    expect_nonterminal_kind(node, NonterminalKind::FunctionName)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::Identifier) => {
            FunctionName::Identifier(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::FallbackKeyword) => FunctionName::FallbackKeyword,
        NodeKind::Terminal(TerminalKind::ReceiveKeyword) => FunctionName::ReceiveKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_function_attribute(node: &Rc<NonterminalNode>) -> Result<FunctionAttribute> {
    expect_nonterminal_kind(node, NonterminalKind::FunctionAttribute)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            FunctionAttribute::ModifierInvocation(build_modifier_invocation(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            FunctionAttribute::OverrideSpecifier(build_override_specifier(nonterminal_node(
                variant,
            )?)?)
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
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_function_body(node: &Rc<NonterminalNode>) -> Result<FunctionBody> {
    expect_nonterminal_kind(node, NonterminalKind::FunctionBody)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::Block) => {
            FunctionBody::Block(build_block(nonterminal_node(variant)?)?)
        }
        NodeKind::Terminal(TerminalKind::Semicolon) => FunctionBody::Semicolon,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_constructor_attribute(node: &Rc<NonterminalNode>) -> Result<ConstructorAttribute> {
    expect_nonterminal_kind(node, NonterminalKind::ConstructorAttribute)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            ConstructorAttribute::ModifierInvocation(build_modifier_invocation(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Terminal(TerminalKind::InternalKeyword) => ConstructorAttribute::InternalKeyword,
        NodeKind::Terminal(TerminalKind::OverrideKeyword) => ConstructorAttribute::OverrideKeyword,
        NodeKind::Terminal(TerminalKind::PayableKeyword) => ConstructorAttribute::PayableKeyword,
        NodeKind::Terminal(TerminalKind::PublicKeyword) => ConstructorAttribute::PublicKeyword,
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => ConstructorAttribute::VirtualKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_unnamed_function_attribute(
    node: &Rc<NonterminalNode>,
) -> Result<UnnamedFunctionAttribute> {
    expect_nonterminal_kind(node, NonterminalKind::UnnamedFunctionAttribute)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            UnnamedFunctionAttribute::ModifierInvocation(build_modifier_invocation(
                nonterminal_node(variant)?,
            )?)
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
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_fallback_function_attribute(
    node: &Rc<NonterminalNode>,
) -> Result<FallbackFunctionAttribute> {
    expect_nonterminal_kind(node, NonterminalKind::FallbackFunctionAttribute)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            FallbackFunctionAttribute::ModifierInvocation(build_modifier_invocation(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            FallbackFunctionAttribute::OverrideSpecifier(build_override_specifier(
                nonterminal_node(variant)?,
            )?)
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
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_receive_function_attribute(
    node: &Rc<NonterminalNode>,
) -> Result<ReceiveFunctionAttribute> {
    expect_nonterminal_kind(node, NonterminalKind::ReceiveFunctionAttribute)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            ReceiveFunctionAttribute::ModifierInvocation(build_modifier_invocation(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            ReceiveFunctionAttribute::OverrideSpecifier(build_override_specifier(
                nonterminal_node(variant)?,
            )?)
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
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_modifier_attribute(node: &Rc<NonterminalNode>) -> Result<ModifierAttribute> {
    expect_nonterminal_kind(node, NonterminalKind::ModifierAttribute)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            ModifierAttribute::OverrideSpecifier(build_override_specifier(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => ModifierAttribute::VirtualKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_type_name(node: &Rc<NonterminalNode>) -> Result<TypeName> {
    expect_nonterminal_kind(node, NonterminalKind::TypeName)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ArrayTypeName) => {
            TypeName::ArrayTypeName(build_array_type_name(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionType) => {
            TypeName::FunctionType(build_function_type(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::MappingType) => {
            TypeName::MappingType(build_mapping_type(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ElementaryType) => {
            TypeName::ElementaryType(build_elementary_type(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::IdentifierPath) => {
            TypeName::IdentifierPath(build_identifier_path(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_function_type_attribute(node: &Rc<NonterminalNode>) -> Result<FunctionTypeAttribute> {
    expect_nonterminal_kind(node, NonterminalKind::FunctionTypeAttribute)?;
    let mut helper = ChildrenHelper::new(&node.children);
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
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_mapping_key_type(node: &Rc<NonterminalNode>) -> Result<MappingKeyType> {
    expect_nonterminal_kind(node, NonterminalKind::MappingKeyType)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ElementaryType) => {
            MappingKeyType::ElementaryType(build_elementary_type(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::IdentifierPath) => {
            MappingKeyType::IdentifierPath(build_identifier_path(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_elementary_type(node: &Rc<NonterminalNode>) -> Result<ElementaryType> {
    expect_nonterminal_kind(node, NonterminalKind::ElementaryType)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::AddressType) => {
            ElementaryType::AddressType(build_address_type(nonterminal_node(variant)?)?)
        }
        NodeKind::Terminal(TerminalKind::BytesKeyword) => {
            ElementaryType::BytesKeyword(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::IntKeyword) => {
            ElementaryType::IntKeyword(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::UintKeyword) => {
            ElementaryType::UintKeyword(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::FixedKeyword) => {
            ElementaryType::FixedKeyword(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::UfixedKeyword) => {
            ElementaryType::UfixedKeyword(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::BoolKeyword) => ElementaryType::BoolKeyword,
        NodeKind::Terminal(TerminalKind::ByteKeyword) => ElementaryType::ByteKeyword,
        NodeKind::Terminal(TerminalKind::StringKeyword) => ElementaryType::StringKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_statement(node: &Rc<NonterminalNode>) -> Result<Statement> {
    expect_nonterminal_kind(node, NonterminalKind::Statement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::IfStatement) => {
            Statement::IfStatement(build_if_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ForStatement) => {
            Statement::ForStatement(build_for_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::WhileStatement) => {
            Statement::WhileStatement(build_while_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::DoWhileStatement) => {
            Statement::DoWhileStatement(build_do_while_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ContinueStatement) => {
            Statement::ContinueStatement(build_continue_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::BreakStatement) => {
            Statement::BreakStatement(build_break_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ReturnStatement) => {
            Statement::ReturnStatement(build_return_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ThrowStatement) => {
            Statement::ThrowStatement(build_throw_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EmitStatement) => {
            Statement::EmitStatement(build_emit_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::TryStatement) => {
            Statement::TryStatement(build_try_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::RevertStatement) => {
            Statement::RevertStatement(build_revert_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::AssemblyStatement) => {
            Statement::AssemblyStatement(build_assembly_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::Block) => {
            Statement::Block(build_block(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UncheckedBlock) => {
            Statement::UncheckedBlock(build_unchecked_block(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement) => {
            Statement::TupleDeconstructionStatement(build_tuple_deconstruction_statement(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement) => {
            Statement::VariableDeclarationStatement(build_variable_declaration_statement(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) => {
            Statement::ExpressionStatement(build_expression_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_tuple_member(node: &Rc<NonterminalNode>) -> Result<TupleMember> {
    expect_nonterminal_kind(node, NonterminalKind::TupleMember)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::TypedTupleMember) => {
            TupleMember::TypedTupleMember(build_typed_tuple_member(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UntypedTupleMember) => {
            TupleMember::UntypedTupleMember(build_untyped_tuple_member(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_variable_declaration_type(
    node: &Rc<NonterminalNode>,
) -> Result<VariableDeclarationType> {
    expect_nonterminal_kind(node, NonterminalKind::VariableDeclarationType)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::TypeName) => {
            VariableDeclarationType::TypeName(build_type_name(nonterminal_node(variant)?)?)
        }
        NodeKind::Terminal(TerminalKind::VarKeyword) => VariableDeclarationType::VarKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_storage_location(node: &Rc<NonterminalNode>) -> Result<StorageLocation> {
    expect_nonterminal_kind(node, NonterminalKind::StorageLocation)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::MemoryKeyword) => StorageLocation::MemoryKeyword,
        NodeKind::Terminal(TerminalKind::StorageKeyword) => StorageLocation::StorageKeyword,
        NodeKind::Terminal(TerminalKind::CallDataKeyword) => StorageLocation::CallDataKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_for_statement_initialization(
    node: &Rc<NonterminalNode>,
) -> Result<ForStatementInitialization> {
    expect_nonterminal_kind(node, NonterminalKind::ForStatementInitialization)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement) => {
            ForStatementInitialization::TupleDeconstructionStatement(
                build_tuple_deconstruction_statement(nonterminal_node(variant)?)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement) => {
            ForStatementInitialization::VariableDeclarationStatement(
                build_variable_declaration_statement(nonterminal_node(variant)?)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) => {
            ForStatementInitialization::ExpressionStatement(build_expression_statement(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Terminal(TerminalKind::Semicolon) => ForStatementInitialization::Semicolon,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_for_statement_condition(node: &Rc<NonterminalNode>) -> Result<ForStatementCondition> {
    expect_nonterminal_kind(node, NonterminalKind::ForStatementCondition)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) => {
            ForStatementCondition::ExpressionStatement(build_expression_statement(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Terminal(TerminalKind::Semicolon) => ForStatementCondition::Semicolon,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_expression(node: &Rc<NonterminalNode>) -> Result<Expression> {
    expect_nonterminal_kind(node, NonterminalKind::Expression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::AssignmentExpression) => {
            Expression::AssignmentExpression(build_assignment_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ConditionalExpression) => {
            Expression::ConditionalExpression(build_conditional_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::OrExpression) => {
            Expression::OrExpression(build_or_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::AndExpression) => {
            Expression::AndExpression(build_and_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::EqualityExpression) => {
            Expression::EqualityExpression(build_equality_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::InequalityExpression) => {
            Expression::InequalityExpression(build_inequality_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::BitwiseOrExpression) => {
            Expression::BitwiseOrExpression(build_bitwise_or_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::BitwiseXorExpression) => {
            Expression::BitwiseXorExpression(build_bitwise_xor_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::BitwiseAndExpression) => {
            Expression::BitwiseAndExpression(build_bitwise_and_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ShiftExpression) => {
            Expression::ShiftExpression(build_shift_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::AdditiveExpression) => {
            Expression::AdditiveExpression(build_additive_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::MultiplicativeExpression) => {
            Expression::MultiplicativeExpression(build_multiplicative_expression(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::ExponentiationExpression) => {
            Expression::ExponentiationExpression(build_exponentiation_expression(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::PostfixExpression) => {
            Expression::PostfixExpression(build_postfix_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::PrefixExpression) => {
            Expression::PrefixExpression(build_prefix_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionCallExpression) => {
            Expression::FunctionCallExpression(build_function_call_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::CallOptionsExpression) => {
            Expression::CallOptionsExpression(build_call_options_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::MemberAccessExpression) => {
            Expression::MemberAccessExpression(build_member_access_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::IndexAccessExpression) => {
            Expression::IndexAccessExpression(build_index_access_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::NewExpression) => {
            Expression::NewExpression(build_new_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::TupleExpression) => {
            Expression::TupleExpression(build_tuple_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::TypeExpression) => {
            Expression::TypeExpression(build_type_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ArrayExpression) => {
            Expression::ArrayExpression(build_array_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::HexNumberExpression) => {
            Expression::HexNumberExpression(build_hex_number_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::DecimalNumberExpression) => {
            Expression::DecimalNumberExpression(build_decimal_number_expression(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StringExpression) => {
            Expression::StringExpression(build_string_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::ElementaryType) => {
            Expression::ElementaryType(build_elementary_type(nonterminal_node(variant)?)?)
        }
        NodeKind::Terminal(TerminalKind::Identifier) => {
            Expression::Identifier(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => Expression::PayableKeyword,
        NodeKind::Terminal(TerminalKind::ThisKeyword) => Expression::ThisKeyword,
        NodeKind::Terminal(TerminalKind::SuperKeyword) => Expression::SuperKeyword,
        NodeKind::Terminal(TerminalKind::TrueKeyword) => Expression::TrueKeyword,
        NodeKind::Terminal(TerminalKind::FalseKeyword) => Expression::FalseKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_arguments_declaration(node: &Rc<NonterminalNode>) -> Result<ArgumentsDeclaration> {
    expect_nonterminal_kind(node, NonterminalKind::ArgumentsDeclaration)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::PositionalArgumentsDeclaration) => {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(
                build_positional_arguments_declaration(nonterminal_node(variant)?)?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::NamedArgumentsDeclaration) => {
            ArgumentsDeclaration::NamedArgumentsDeclaration(build_named_arguments_declaration(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_number_unit(node: &Rc<NonterminalNode>) -> Result<NumberUnit> {
    expect_nonterminal_kind(node, NonterminalKind::NumberUnit)?;
    let mut helper = ChildrenHelper::new(&node.children);
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
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_string_expression(node: &Rc<NonterminalNode>) -> Result<StringExpression> {
    expect_nonterminal_kind(node, NonterminalKind::StringExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::StringLiteral) => {
            StringExpression::StringLiteral(build_string_literal(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StringLiterals) => {
            StringExpression::StringLiterals(build_string_literals(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::HexStringLiteral) => {
            StringExpression::HexStringLiteral(build_hex_string_literal(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::HexStringLiterals) => {
            StringExpression::HexStringLiterals(build_hex_string_literals(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::UnicodeStringLiterals) => {
            StringExpression::UnicodeStringLiterals(build_unicode_string_literals(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_string_literal(node: &Rc<NonterminalNode>) -> Result<StringLiteral> {
    expect_nonterminal_kind(node, NonterminalKind::StringLiteral)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::SingleQuotedStringLiteral) => {
            StringLiteral::SingleQuotedStringLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedStringLiteral) => {
            StringLiteral::DoubleQuotedStringLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_hex_string_literal(node: &Rc<NonterminalNode>) -> Result<HexStringLiteral> {
    expect_nonterminal_kind(node, NonterminalKind::HexStringLiteral)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::SingleQuotedHexStringLiteral) => {
            HexStringLiteral::SingleQuotedHexStringLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedHexStringLiteral) => {
            HexStringLiteral::DoubleQuotedHexStringLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_unicode_string_literal(node: &Rc<NonterminalNode>) -> Result<UnicodeStringLiteral> {
    expect_nonterminal_kind(node, NonterminalKind::UnicodeStringLiteral)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Terminal(TerminalKind::SingleQuotedUnicodeStringLiteral) => {
            UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedUnicodeStringLiteral) => {
            UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_yul_statement(node: &Rc<NonterminalNode>) -> Result<YulStatement> {
    expect_nonterminal_kind(node, NonterminalKind::YulStatement)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulBlock) => {
            YulStatement::YulBlock(build_yul_block(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulFunctionDefinition) => {
            YulStatement::YulFunctionDefinition(build_yul_function_definition(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulStackAssignmentStatement) => {
            YulStatement::YulStackAssignmentStatement(build_yul_stack_assignment_statement(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulIfStatement) => {
            YulStatement::YulIfStatement(build_yul_if_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulForStatement) => {
            YulStatement::YulForStatement(build_yul_for_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulSwitchStatement) => {
            YulStatement::YulSwitchStatement(build_yul_switch_statement(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulLeaveStatement) => {
            YulStatement::YulLeaveStatement(build_yul_leave_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulBreakStatement) => {
            YulStatement::YulBreakStatement(build_yul_break_statement(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulContinueStatement) => {
            YulStatement::YulContinueStatement(build_yul_continue_statement(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulVariableAssignmentStatement) => {
            YulStatement::YulVariableAssignmentStatement(build_yul_variable_assignment_statement(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulLabel) => {
            YulStatement::YulLabel(build_yul_label(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulVariableDeclarationStatement) => {
            YulStatement::YulVariableDeclarationStatement(build_yul_variable_declaration_statement(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulExpression) => {
            YulStatement::YulExpression(build_yul_expression(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_yul_assignment_operator(node: &Rc<NonterminalNode>) -> Result<YulAssignmentOperator> {
    expect_nonterminal_kind(node, NonterminalKind::YulAssignmentOperator)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulColonAndEqual) => {
            YulAssignmentOperator::YulColonAndEqual(build_yul_colon_and_equal(nonterminal_node(
                variant,
            )?)?)
        }
        NodeKind::Terminal(TerminalKind::ColonEqual) => YulAssignmentOperator::ColonEqual,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_yul_stack_assignment_operator(
    node: &Rc<NonterminalNode>,
) -> Result<YulStackAssignmentOperator> {
    expect_nonterminal_kind(node, NonterminalKind::YulStackAssignmentOperator)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulEqualAndColon) => {
            YulStackAssignmentOperator::YulEqualAndColon(build_yul_equal_and_colon(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Terminal(TerminalKind::EqualColon) => YulStackAssignmentOperator::EqualColon,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_yul_switch_case(node: &Rc<NonterminalNode>) -> Result<YulSwitchCase> {
    expect_nonterminal_kind(node, NonterminalKind::YulSwitchCase)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulDefaultCase) => {
            YulSwitchCase::YulDefaultCase(build_yul_default_case(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulValueCase) => {
            YulSwitchCase::YulValueCase(build_yul_value_case(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_yul_expression(node: &Rc<NonterminalNode>) -> Result<YulExpression> {
    expect_nonterminal_kind(node, NonterminalKind::YulExpression)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::YulFunctionCallExpression) => {
            YulExpression::YulFunctionCallExpression(build_yul_function_call_expression(
                nonterminal_node(variant)?,
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulLiteral) => {
            YulExpression::YulLiteral(build_yul_literal(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulPath) => {
            YulExpression::YulPath(build_yul_path(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

pub fn build_yul_literal(node: &Rc<NonterminalNode>) -> Result<YulLiteral> {
    expect_nonterminal_kind(node, NonterminalKind::YulLiteral)?;
    let mut helper = ChildrenHelper::new(&node.children);
    let variant = helper.accept_label(EdgeLabel::Variant)?;
    let item = match variant.kind() {
        NodeKind::Nonterminal(NonterminalKind::HexStringLiteral) => {
            YulLiteral::HexStringLiteral(build_hex_string_literal(nonterminal_node(variant)?)?)
        }
        NodeKind::Nonterminal(NonterminalKind::StringLiteral) => {
            YulLiteral::StringLiteral(build_string_literal(nonterminal_node(variant)?)?)
        }
        NodeKind::Terminal(TerminalKind::YulDecimalLiteral) => {
            YulLiteral::YulDecimalLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::YulHexLiteral) => {
            YulLiteral::YulHexLiteral(terminal_node_cloned(variant)?)
        }
        NodeKind::Terminal(TerminalKind::YulTrueKeyword) => YulLiteral::YulTrueKeyword,
        NodeKind::Terminal(TerminalKind::YulFalseKeyword) => YulLiteral::YulFalseKeyword,
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                variant.kind()
            ));
        }
    };
    helper.finalize()?;
    Ok(item)
}

//
// Repeated & Separated
//

pub fn build_source_unit_members(node: &Rc<NonterminalNode>) -> Result<SourceUnitMembers> {
    expect_nonterminal_kind(node, NonterminalKind::SourceUnitMembers)?;
    let mut items = SourceUnitMembers::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_source_unit_member(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_version_expression_sets(node: &Rc<NonterminalNode>) -> Result<VersionExpressionSets> {
    expect_nonterminal_kind(node, NonterminalKind::VersionExpressionSets)?;
    let mut items = VersionExpressionSets::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_version_expression_set(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_version_expression_set(node: &Rc<NonterminalNode>) -> Result<VersionExpressionSet> {
    expect_nonterminal_kind(node, NonterminalKind::VersionExpressionSet)?;
    let mut items = VersionExpressionSet::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_version_expression(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_simple_version_literal(node: &Rc<NonterminalNode>) -> Result<SimpleVersionLiteral> {
    expect_nonterminal_kind(node, NonterminalKind::SimpleVersionLiteral)?;
    let mut items = SimpleVersionLiteral::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = terminal_node_cloned(child)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_import_deconstruction_symbols(
    node: &Rc<NonterminalNode>,
) -> Result<ImportDeconstructionSymbols> {
    expect_nonterminal_kind(node, NonterminalKind::ImportDeconstructionSymbols)?;
    let mut items = ImportDeconstructionSymbols::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_import_deconstruction_symbol(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_using_deconstruction_symbols(
    node: &Rc<NonterminalNode>,
) -> Result<UsingDeconstructionSymbols> {
    expect_nonterminal_kind(node, NonterminalKind::UsingDeconstructionSymbols)?;
    let mut items = UsingDeconstructionSymbols::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_using_deconstruction_symbol(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_contract_specifiers(node: &Rc<NonterminalNode>) -> Result<ContractSpecifiers> {
    expect_nonterminal_kind(node, NonterminalKind::ContractSpecifiers)?;
    let mut items = ContractSpecifiers::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_contract_specifier(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_inheritance_types(node: &Rc<NonterminalNode>) -> Result<InheritanceTypes> {
    expect_nonterminal_kind(node, NonterminalKind::InheritanceTypes)?;
    let mut items = InheritanceTypes::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_inheritance_type(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_contract_members(node: &Rc<NonterminalNode>) -> Result<ContractMembers> {
    expect_nonterminal_kind(node, NonterminalKind::ContractMembers)?;
    let mut items = ContractMembers::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_contract_member(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_interface_members(node: &Rc<NonterminalNode>) -> Result<InterfaceMembers> {
    expect_nonterminal_kind(node, NonterminalKind::InterfaceMembers)?;
    let mut items = InterfaceMembers::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_contract_member(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_library_members(node: &Rc<NonterminalNode>) -> Result<LibraryMembers> {
    expect_nonterminal_kind(node, NonterminalKind::LibraryMembers)?;
    let mut items = LibraryMembers::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_contract_member(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_struct_members(node: &Rc<NonterminalNode>) -> Result<StructMembers> {
    expect_nonterminal_kind(node, NonterminalKind::StructMembers)?;
    let mut items = StructMembers::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_struct_member(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_enum_members(node: &Rc<NonterminalNode>) -> Result<EnumMembers> {
    expect_nonterminal_kind(node, NonterminalKind::EnumMembers)?;
    let mut items = EnumMembers::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = terminal_node_cloned(child)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_state_variable_attributes(
    node: &Rc<NonterminalNode>,
) -> Result<StateVariableAttributes> {
    expect_nonterminal_kind(node, NonterminalKind::StateVariableAttributes)?;
    let mut items = StateVariableAttributes::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_state_variable_attribute(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_parameters(node: &Rc<NonterminalNode>) -> Result<Parameters> {
    expect_nonterminal_kind(node, NonterminalKind::Parameters)?;
    let mut items = Parameters::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_parameter(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_function_attributes(node: &Rc<NonterminalNode>) -> Result<FunctionAttributes> {
    expect_nonterminal_kind(node, NonterminalKind::FunctionAttributes)?;
    let mut items = FunctionAttributes::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_function_attribute(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_override_paths(node: &Rc<NonterminalNode>) -> Result<OverridePaths> {
    expect_nonterminal_kind(node, NonterminalKind::OverridePaths)?;
    let mut items = OverridePaths::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_identifier_path(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_constructor_attributes(node: &Rc<NonterminalNode>) -> Result<ConstructorAttributes> {
    expect_nonterminal_kind(node, NonterminalKind::ConstructorAttributes)?;
    let mut items = ConstructorAttributes::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_constructor_attribute(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_unnamed_function_attributes(
    node: &Rc<NonterminalNode>,
) -> Result<UnnamedFunctionAttributes> {
    expect_nonterminal_kind(node, NonterminalKind::UnnamedFunctionAttributes)?;
    let mut items = UnnamedFunctionAttributes::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_unnamed_function_attribute(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_fallback_function_attributes(
    node: &Rc<NonterminalNode>,
) -> Result<FallbackFunctionAttributes> {
    expect_nonterminal_kind(node, NonterminalKind::FallbackFunctionAttributes)?;
    let mut items = FallbackFunctionAttributes::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_fallback_function_attribute(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_receive_function_attributes(
    node: &Rc<NonterminalNode>,
) -> Result<ReceiveFunctionAttributes> {
    expect_nonterminal_kind(node, NonterminalKind::ReceiveFunctionAttributes)?;
    let mut items = ReceiveFunctionAttributes::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_receive_function_attribute(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_modifier_attributes(node: &Rc<NonterminalNode>) -> Result<ModifierAttributes> {
    expect_nonterminal_kind(node, NonterminalKind::ModifierAttributes)?;
    let mut items = ModifierAttributes::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_modifier_attribute(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_event_parameters(node: &Rc<NonterminalNode>) -> Result<EventParameters> {
    expect_nonterminal_kind(node, NonterminalKind::EventParameters)?;
    let mut items = EventParameters::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_event_parameter(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_error_parameters(node: &Rc<NonterminalNode>) -> Result<ErrorParameters> {
    expect_nonterminal_kind(node, NonterminalKind::ErrorParameters)?;
    let mut items = ErrorParameters::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_error_parameter(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_function_type_attributes(
    node: &Rc<NonterminalNode>,
) -> Result<FunctionTypeAttributes> {
    expect_nonterminal_kind(node, NonterminalKind::FunctionTypeAttributes)?;
    let mut items = FunctionTypeAttributes::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_function_type_attribute(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_statements(node: &Rc<NonterminalNode>) -> Result<Statements> {
    expect_nonterminal_kind(node, NonterminalKind::Statements)?;
    let mut items = Statements::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_statement(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_assembly_flags(node: &Rc<NonterminalNode>) -> Result<AssemblyFlags> {
    expect_nonterminal_kind(node, NonterminalKind::AssemblyFlags)?;
    let mut items = AssemblyFlags::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_string_literal(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_tuple_deconstruction_elements(
    node: &Rc<NonterminalNode>,
) -> Result<TupleDeconstructionElements> {
    expect_nonterminal_kind(node, NonterminalKind::TupleDeconstructionElements)?;
    let mut items = TupleDeconstructionElements::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_tuple_deconstruction_element(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_catch_clauses(node: &Rc<NonterminalNode>) -> Result<CatchClauses> {
    expect_nonterminal_kind(node, NonterminalKind::CatchClauses)?;
    let mut items = CatchClauses::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_catch_clause(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_positional_arguments(node: &Rc<NonterminalNode>) -> Result<PositionalArguments> {
    expect_nonterminal_kind(node, NonterminalKind::PositionalArguments)?;
    let mut items = PositionalArguments::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_expression(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_named_arguments(node: &Rc<NonterminalNode>) -> Result<NamedArguments> {
    expect_nonterminal_kind(node, NonterminalKind::NamedArguments)?;
    let mut items = NamedArguments::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_named_argument(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_call_options(node: &Rc<NonterminalNode>) -> Result<CallOptions> {
    expect_nonterminal_kind(node, NonterminalKind::CallOptions)?;
    let mut items = CallOptions::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_named_argument(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_tuple_values(node: &Rc<NonterminalNode>) -> Result<TupleValues> {
    expect_nonterminal_kind(node, NonterminalKind::TupleValues)?;
    let mut items = TupleValues::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_tuple_value(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_array_values(node: &Rc<NonterminalNode>) -> Result<ArrayValues> {
    expect_nonterminal_kind(node, NonterminalKind::ArrayValues)?;
    let mut items = ArrayValues::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_expression(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_string_literals(node: &Rc<NonterminalNode>) -> Result<StringLiterals> {
    expect_nonterminal_kind(node, NonterminalKind::StringLiterals)?;
    let mut items = StringLiterals::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_string_literal(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_hex_string_literals(node: &Rc<NonterminalNode>) -> Result<HexStringLiterals> {
    expect_nonterminal_kind(node, NonterminalKind::HexStringLiterals)?;
    let mut items = HexStringLiterals::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_hex_string_literal(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_unicode_string_literals(node: &Rc<NonterminalNode>) -> Result<UnicodeStringLiterals> {
    expect_nonterminal_kind(node, NonterminalKind::UnicodeStringLiterals)?;
    let mut items = UnicodeStringLiterals::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_unicode_string_literal(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_identifier_path(node: &Rc<NonterminalNode>) -> Result<IdentifierPath> {
    expect_nonterminal_kind(node, NonterminalKind::IdentifierPath)?;
    let mut items = IdentifierPath::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = terminal_node_cloned(child)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_yul_statements(node: &Rc<NonterminalNode>) -> Result<YulStatements> {
    expect_nonterminal_kind(node, NonterminalKind::YulStatements)?;
    let mut items = YulStatements::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_yul_statement(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_yul_parameters(node: &Rc<NonterminalNode>) -> Result<YulParameters> {
    expect_nonterminal_kind(node, NonterminalKind::YulParameters)?;
    let mut items = YulParameters::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = terminal_node_cloned(child)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_yul_variable_names(node: &Rc<NonterminalNode>) -> Result<YulVariableNames> {
    expect_nonterminal_kind(node, NonterminalKind::YulVariableNames)?;
    let mut items = YulVariableNames::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = terminal_node_cloned(child)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_yul_switch_cases(node: &Rc<NonterminalNode>) -> Result<YulSwitchCases> {
    expect_nonterminal_kind(node, NonterminalKind::YulSwitchCases)?;
    let mut items = YulSwitchCases::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_yul_switch_case(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_yul_arguments(node: &Rc<NonterminalNode>) -> Result<YulArguments> {
    expect_nonterminal_kind(node, NonterminalKind::YulArguments)?;
    let mut items = YulArguments::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_yul_expression(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_yul_paths(node: &Rc<NonterminalNode>) -> Result<YulPaths> {
    expect_nonterminal_kind(node, NonterminalKind::YulPaths)?;
    let mut items = YulPaths::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = build_yul_path(nonterminal_node(child)?)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

pub fn build_yul_path(node: &Rc<NonterminalNode>) -> Result<YulPath> {
    expect_nonterminal_kind(node, NonterminalKind::YulPath)?;
    let mut items = YulPath::new();
    let mut helper = ChildrenHelper::new(&node.children);
    while helper.at_label(EdgeLabel::Item) {
        let child = helper.accept_label(EdgeLabel::Item)?;
        let item = terminal_node_cloned(child)?;
        items.push(item);
        if helper.at_label(EdgeLabel::Separator) {
            _ = helper.accept_label(EdgeLabel::Separator)?;
        }
    }
    helper.finalize()?;
    Ok(items)
}

//
// Common:
//

type Result<T> = std::result::Result<T, String>;

#[allow(dead_code)]
#[inline]
fn expect_nonterminal_kind(node: &Rc<NonterminalNode>, kind: NonterminalKind) -> Result<()> {
    if node.kind == kind {
        Ok(())
    } else {
        Err(format!(
            "Expected {kind:?} node, but got {:?} instead",
            node.kind
        ))
    }
}

#[allow(dead_code)]
#[inline]
fn terminal_node_cloned(node: &Node) -> Result<Rc<TerminalNode>> {
    node.as_terminal().map(Rc::clone).ok_or(format!(
        "Expected terminal node, got {:?} instead",
        node.kind()
    ))
}

#[allow(dead_code)]
#[inline]
fn nonterminal_node(node: &Node) -> Result<&Rc<NonterminalNode>> {
    node.as_nonterminal().ok_or(format!(
        "Expected non-terminal node, got {:?} instead",
        node.kind()
    ))
}

struct ChildrenHelper<'a> {
    children: &'a Vec<Edge>,
    index: usize,
}

impl<'a> ChildrenHelper<'a> {
    fn new(children: &'a Vec<Edge>) -> Self {
        let mut index = 0;
        while index < children.len() {
            if !children[index].is_trivia() {
                break;
            }
            index += 1;
        }
        Self { children, index }
    }

    fn at_label(&self, label: EdgeLabel) -> bool {
        self.index < self.children.len() && self.children[self.index].label == label
    }

    fn accept_label(&mut self, label: EdgeLabel) -> Result<&Node> {
        if self.index >= self.children.len() {
            return Err(format!(
                "Expected more sibling nodes, looking for label {label:?}"
            ));
        }
        if self.children[self.index].label == label {
            let node = &self.children[self.index].node;
            loop {
                self.index += 1;
                if self.index >= self.children.len() || !self.children[self.index].is_trivia() {
                    break;
                }
            }
            Ok(node)
        } else {
            Err(format!(
                "Expected node with label {label:?}, got {:?}",
                self.children[self.index].label
            ))
        }
    }

    fn finalize(mut self) -> Result<()> {
        while self.index < self.children.len() {
            if !self.children[self.index].is_trivia() {
                return Err("Unexpected non-trivia node".into());
            }
            self.index += 1;
        }
        Ok(())
    }
}
