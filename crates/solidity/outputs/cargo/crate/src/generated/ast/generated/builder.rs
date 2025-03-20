// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::{Cursor, EdgeLabel, NodeKind, NonterminalKind, TerminalKind, TerminalNode};

//
// Sequences:
//

pub fn build_source_unit(cursor: Cursor) -> Result<SourceUnit> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SourceUnit)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let members = build_source_unit_members(helper.accept_label(EdgeLabel::Members)?)?;
    helper.finalize()?;

    Ok(Rc::new(SourceUnitStruct { node_id, members }))
}

pub fn build_pragma_directive(cursor: Cursor) -> Result<PragmaDirective> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PragmaDirective)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::PragmaKeyword)?)?;
    let pragma = build_pragma(helper.accept_label(EdgeLabel::Pragma)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(PragmaDirectiveStruct { node_id, pragma }))
}

pub fn build_abicoder_pragma(cursor: Cursor) -> Result<AbicoderPragma> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AbicoderPragma)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::AbicoderKeyword)?)?;
    let version = fetch_terminal_node(&helper.accept_label(EdgeLabel::Version)?)?;
    helper.finalize()?;

    Ok(Rc::new(AbicoderPragmaStruct { node_id, version }))
}

pub fn build_experimental_pragma(cursor: Cursor) -> Result<ExperimentalPragma> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ExperimentalPragma)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ExperimentalKeyword)?)?;
    let feature = build_experimental_feature(helper.accept_label(EdgeLabel::Feature)?)?;
    helper.finalize()?;

    Ok(Rc::new(ExperimentalPragmaStruct { node_id, feature }))
}

pub fn build_version_pragma(cursor: Cursor) -> Result<VersionPragma> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionPragma)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::SolidityKeyword)?)?;
    let sets = build_version_expression_sets(helper.accept_label(EdgeLabel::Sets)?)?;
    helper.finalize()?;

    Ok(Rc::new(VersionPragmaStruct { node_id, sets }))
}

pub fn build_version_range(cursor: Cursor) -> Result<VersionRange> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionRange)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let start = build_version_literal(helper.accept_label(EdgeLabel::Start)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Minus)?)?;
    let end = build_version_literal(helper.accept_label(EdgeLabel::End)?)?;
    helper.finalize()?;

    Ok(Rc::new(VersionRangeStruct {
        node_id,
        start,
        end,
    }))
}

pub fn build_version_term(cursor: Cursor) -> Result<VersionTerm> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionTerm)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let operator = if helper.at_label(EdgeLabel::Operator) {
        Some(build_version_operator(
            helper.accept_label(EdgeLabel::Operator)?,
        )?)
    } else {
        None
    };
    let literal = build_version_literal(helper.accept_label(EdgeLabel::Literal)?)?;
    helper.finalize()?;

    Ok(Rc::new(VersionTermStruct {
        node_id,
        operator,
        literal,
    }))
}

pub fn build_import_directive(cursor: Cursor) -> Result<ImportDirective> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportDirective)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ImportKeyword)?)?;
    let clause = build_import_clause(helper.accept_label(EdgeLabel::Clause)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ImportDirectiveStruct { node_id, clause }))
}

pub fn build_path_import(cursor: Cursor) -> Result<PathImport> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PathImport)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let path = build_string_literal(helper.accept_label(EdgeLabel::Path)?)?;
    let alias = if helper.at_label(EdgeLabel::Alias) {
        Some(build_import_alias(helper.accept_label(EdgeLabel::Alias)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(PathImportStruct {
        node_id,
        path,
        alias,
    }))
}

pub fn build_named_import(cursor: Cursor) -> Result<NamedImport> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NamedImport)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Asterisk)?)?;
    let alias = build_import_alias(helper.accept_label(EdgeLabel::Alias)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::FromKeyword)?)?;
    let path = build_string_literal(helper.accept_label(EdgeLabel::Path)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedImportStruct {
        node_id,
        alias,
        path,
    }))
}

pub fn build_import_deconstruction(cursor: Cursor) -> Result<ImportDeconstruction> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportDeconstruction)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let symbols = build_import_deconstruction_symbols(helper.accept_label(EdgeLabel::Symbols)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::FromKeyword)?)?;
    let path = build_string_literal(helper.accept_label(EdgeLabel::Path)?)?;
    helper.finalize()?;

    Ok(Rc::new(ImportDeconstructionStruct {
        node_id,
        symbols,
        path,
    }))
}

pub fn build_import_deconstruction_symbol(cursor: Cursor) -> Result<ImportDeconstructionSymbol> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportDeconstructionSymbol)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let alias = if helper.at_label(EdgeLabel::Alias) {
        Some(build_import_alias(helper.accept_label(EdgeLabel::Alias)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ImportDeconstructionSymbolStruct {
        node_id,
        name,
        alias,
    }))
}

pub fn build_import_alias(cursor: Cursor) -> Result<ImportAlias> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportAlias)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::AsKeyword)?)?;
    let identifier = fetch_terminal_node(&helper.accept_label(EdgeLabel::Identifier)?)?;
    helper.finalize()?;

    Ok(Rc::new(ImportAliasStruct {
        node_id,
        identifier,
    }))
}

pub fn build_using_directive(cursor: Cursor) -> Result<UsingDirective> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingDirective)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::UsingKeyword)?)?;
    let clause = build_using_clause(helper.accept_label(EdgeLabel::Clause)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ForKeyword)?)?;
    let target = build_using_target(helper.accept_label(EdgeLabel::Target)?)?;
    let global_keyword = if helper.at_label(EdgeLabel::GlobalKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::GlobalKeyword)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(UsingDirectiveStruct {
        node_id,
        clause,
        target,
        global_keyword,
    }))
}

pub fn build_using_deconstruction(cursor: Cursor) -> Result<UsingDeconstruction> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingDeconstruction)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let symbols = build_using_deconstruction_symbols(helper.accept_label(EdgeLabel::Symbols)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(UsingDeconstructionStruct { node_id, symbols }))
}

pub fn build_using_deconstruction_symbol(cursor: Cursor) -> Result<UsingDeconstructionSymbol> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingDeconstructionSymbol)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let name = build_identifier_path(helper.accept_label(EdgeLabel::Name)?)?;
    let alias = if helper.at_label(EdgeLabel::Alias) {
        Some(build_using_alias(helper.accept_label(EdgeLabel::Alias)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(UsingDeconstructionSymbolStruct {
        node_id,
        name,
        alias,
    }))
}

pub fn build_using_alias(cursor: Cursor) -> Result<UsingAlias> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingAlias)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::AsKeyword)?)?;
    let operator = build_using_operator(helper.accept_label(EdgeLabel::Operator)?)?;
    helper.finalize()?;

    Ok(Rc::new(UsingAliasStruct { node_id, operator }))
}

pub fn build_contract_definition(cursor: Cursor) -> Result<ContractDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ContractDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let abstract_keyword = if helper.at_label(EdgeLabel::AbstractKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::AbstractKeyword)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ContractKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let inheritance = if helper.at_label(EdgeLabel::Inheritance) {
        Some(build_inheritance_specifier(
            helper.accept_label(EdgeLabel::Inheritance)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_contract_members(helper.accept_label(EdgeLabel::Members)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(ContractDefinitionStruct {
        node_id,
        abstract_keyword,
        name,
        inheritance,
        members,
    }))
}

pub fn build_inheritance_specifier(cursor: Cursor) -> Result<InheritanceSpecifier> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InheritanceSpecifier)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::IsKeyword)?)?;
    let types = build_inheritance_types(helper.accept_label(EdgeLabel::Types)?)?;
    helper.finalize()?;

    Ok(Rc::new(InheritanceSpecifierStruct { node_id, types }))
}

pub fn build_inheritance_type(cursor: Cursor) -> Result<InheritanceType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InheritanceType)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let type_name = build_identifier_path(helper.accept_label(EdgeLabel::TypeName)?)?;
    let arguments = if helper.at_label(EdgeLabel::Arguments) {
        Some(build_arguments_declaration(
            helper.accept_label(EdgeLabel::Arguments)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(InheritanceTypeStruct {
        node_id,
        type_name,
        arguments,
    }))
}

pub fn build_interface_definition(cursor: Cursor) -> Result<InterfaceDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InterfaceDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::InterfaceKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let inheritance = if helper.at_label(EdgeLabel::Inheritance) {
        Some(build_inheritance_specifier(
            helper.accept_label(EdgeLabel::Inheritance)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_interface_members(helper.accept_label(EdgeLabel::Members)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(InterfaceDefinitionStruct {
        node_id,
        name,
        inheritance,
        members,
    }))
}

pub fn build_library_definition(cursor: Cursor) -> Result<LibraryDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::LibraryDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::LibraryKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_library_members(helper.accept_label(EdgeLabel::Members)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(LibraryDefinitionStruct {
        node_id,
        name,
        members,
    }))
}

pub fn build_struct_definition(cursor: Cursor) -> Result<StructDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StructDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::StructKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_struct_members(helper.accept_label(EdgeLabel::Members)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(StructDefinitionStruct {
        node_id,
        name,
        members,
    }))
}

pub fn build_struct_member(cursor: Cursor) -> Result<StructMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StructMember)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(StructMemberStruct {
        node_id,
        type_name,
        name,
    }))
}

pub fn build_enum_definition(cursor: Cursor) -> Result<EnumDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EnumDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::EnumKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_enum_members(helper.accept_label(EdgeLabel::Members)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(EnumDefinitionStruct {
        node_id,
        name,
        members,
    }))
}

pub fn build_constant_definition(cursor: Cursor) -> Result<ConstantDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ConstantDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ConstantKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    let value = build_expression(helper.accept_label(EdgeLabel::Value)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ConstantDefinitionStruct {
        node_id,
        type_name,
        name,
        value,
    }))
}

pub fn build_state_variable_definition(cursor: Cursor) -> Result<StateVariableDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StateVariableDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let attributes = build_state_variable_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let value = if helper.at_label(EdgeLabel::Value) {
        Some(build_state_variable_definition_value(
            helper.accept_label(EdgeLabel::Value)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(StateVariableDefinitionStruct {
        node_id,
        type_name,
        attributes,
        name,
        value,
    }))
}

pub fn build_state_variable_definition_value(
    cursor: Cursor,
) -> Result<StateVariableDefinitionValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StateVariableDefinitionValue)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    let value = build_expression(helper.accept_label(EdgeLabel::Value)?)?;
    helper.finalize()?;

    Ok(Rc::new(StateVariableDefinitionValueStruct {
        node_id,
        value,
    }))
}

pub fn build_function_definition(cursor: Cursor) -> Result<FunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::FunctionKeyword)?)?;
    let name = build_function_name(helper.accept_label(EdgeLabel::Name)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes = build_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_returns_declaration(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)
    } else {
        None
    };
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(FunctionDefinitionStruct {
        node_id,
        name,
        parameters,
        attributes,
        returns,
        body,
    }))
}

pub fn build_parameters_declaration(cursor: Cursor) -> Result<ParametersDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ParametersDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let parameters = build_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(ParametersDeclarationStruct {
        node_id,
        parameters,
    }))
}

pub fn build_parameter(cursor: Cursor) -> Result<Parameter> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Parameter)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let storage_location = if helper.at_label(EdgeLabel::StorageLocation) {
        Some(build_storage_location(
            helper.accept_label(EdgeLabel::StorageLocation)?,
        )?)
    } else {
        None
    };
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ParameterStruct {
        node_id,
        type_name,
        storage_location,
        name,
    }))
}

pub fn build_override_specifier(cursor: Cursor) -> Result<OverrideSpecifier> {
    expect_nonterminal_kind(&cursor, NonterminalKind::OverrideSpecifier)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OverrideKeyword)?)?;
    let overridden = if helper.at_label(EdgeLabel::Overridden) {
        Some(build_override_paths_declaration(
            helper.accept_label(EdgeLabel::Overridden)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(OverrideSpecifierStruct {
        node_id,
        overridden,
    }))
}

pub fn build_override_paths_declaration(cursor: Cursor) -> Result<OverridePathsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::OverridePathsDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let paths = build_override_paths(helper.accept_label(EdgeLabel::Paths)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(OverridePathsDeclarationStruct { node_id, paths }))
}

pub fn build_returns_declaration(cursor: Cursor) -> Result<ReturnsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ReturnsDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ReturnsKeyword)?)?;
    let variables = build_parameters_declaration(helper.accept_label(EdgeLabel::Variables)?)?;
    helper.finalize()?;

    Ok(Rc::new(ReturnsDeclarationStruct { node_id, variables }))
}

pub fn build_constructor_definition(cursor: Cursor) -> Result<ConstructorDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ConstructorDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ConstructorKeyword)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes = build_constructor_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(ConstructorDefinitionStruct {
        node_id,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_unnamed_function_definition(cursor: Cursor) -> Result<UnnamedFunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UnnamedFunctionDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::FunctionKeyword)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes =
        build_unnamed_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(UnnamedFunctionDefinitionStruct {
        node_id,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_fallback_function_definition(cursor: Cursor) -> Result<FallbackFunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FallbackFunctionDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::FallbackKeyword)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes =
        build_fallback_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_returns_declaration(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)
    } else {
        None
    };
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(FallbackFunctionDefinitionStruct {
        node_id,
        parameters,
        attributes,
        returns,
        body,
    }))
}

pub fn build_receive_function_definition(cursor: Cursor) -> Result<ReceiveFunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ReceiveFunctionDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ReceiveKeyword)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes =
        build_receive_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(ReceiveFunctionDefinitionStruct {
        node_id,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_modifier_definition(cursor: Cursor) -> Result<ModifierDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ModifierDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ModifierKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let parameters = if helper.at_label(EdgeLabel::Parameters) {
        Some(build_parameters_declaration(
            helper.accept_label(EdgeLabel::Parameters)?,
        )?)
    } else {
        None
    };
    let attributes = build_modifier_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(ModifierDefinitionStruct {
        node_id,
        name,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_modifier_invocation(cursor: Cursor) -> Result<ModifierInvocation> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ModifierInvocation)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let name = build_identifier_path(helper.accept_label(EdgeLabel::Name)?)?;
    let arguments = if helper.at_label(EdgeLabel::Arguments) {
        Some(build_arguments_declaration(
            helper.accept_label(EdgeLabel::Arguments)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ModifierInvocationStruct {
        node_id,
        name,
        arguments,
    }))
}

pub fn build_event_definition(cursor: Cursor) -> Result<EventDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EventDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::EventKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let parameters =
        build_event_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let anonymous_keyword = if helper.at_label(EdgeLabel::AnonymousKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::AnonymousKeyword)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(EventDefinitionStruct {
        node_id,
        name,
        parameters,
        anonymous_keyword,
    }))
}

pub fn build_event_parameters_declaration(cursor: Cursor) -> Result<EventParametersDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EventParametersDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let parameters = build_event_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(EventParametersDeclarationStruct {
        node_id,
        parameters,
    }))
}

pub fn build_event_parameter(cursor: Cursor) -> Result<EventParameter> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EventParameter)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let indexed_keyword = if helper.at_label(EdgeLabel::IndexedKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::IndexedKeyword)?,
        )?)
    } else {
        None
    };
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(EventParameterStruct {
        node_id,
        type_name,
        indexed_keyword,
        name,
    }))
}

pub fn build_user_defined_value_type_definition(
    cursor: Cursor,
) -> Result<UserDefinedValueTypeDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UserDefinedValueTypeDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::TypeKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::IsKeyword)?)?;
    let value_type = build_elementary_type(helper.accept_label(EdgeLabel::ValueType)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(UserDefinedValueTypeDefinitionStruct {
        node_id,
        name,
        value_type,
    }))
}

pub fn build_error_definition(cursor: Cursor) -> Result<ErrorDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ErrorDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ErrorKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let members = build_error_parameters_declaration(helper.accept_label(EdgeLabel::Members)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ErrorDefinitionStruct {
        node_id,
        name,
        members,
    }))
}

pub fn build_error_parameters_declaration(cursor: Cursor) -> Result<ErrorParametersDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ErrorParametersDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let parameters = build_error_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(ErrorParametersDeclarationStruct {
        node_id,
        parameters,
    }))
}

pub fn build_error_parameter(cursor: Cursor) -> Result<ErrorParameter> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ErrorParameter)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ErrorParameterStruct {
        node_id,
        type_name,
        name,
    }))
}

pub fn build_array_type_name(cursor: Cursor) -> Result<ArrayTypeName> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ArrayTypeName)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let operand = build_type_name(helper.accept_label(EdgeLabel::Operand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBracket)?)?;
    let index = if helper.at_label(EdgeLabel::Index) {
        Some(build_expression(helper.accept_label(EdgeLabel::Index)?)?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBracket)?)?;
    helper.finalize()?;

    Ok(Rc::new(ArrayTypeNameStruct {
        node_id,
        operand,
        index,
    }))
}

pub fn build_function_type(cursor: Cursor) -> Result<FunctionType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionType)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::FunctionKeyword)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes = build_function_type_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_returns_declaration(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(FunctionTypeStruct {
        node_id,
        parameters,
        attributes,
        returns,
    }))
}

pub fn build_mapping_type(cursor: Cursor) -> Result<MappingType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MappingType)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::MappingKeyword)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let key_type = build_mapping_key(helper.accept_label(EdgeLabel::KeyType)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::EqualGreaterThan)?)?;
    let value_type = build_mapping_value(helper.accept_label(EdgeLabel::ValueType)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(MappingTypeStruct {
        node_id,
        key_type,
        value_type,
    }))
}

pub fn build_mapping_key(cursor: Cursor) -> Result<MappingKey> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MappingKey)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let key_type = build_mapping_key_type(helper.accept_label(EdgeLabel::KeyType)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(MappingKeyStruct {
        node_id,
        key_type,
        name,
    }))
}

pub fn build_mapping_value(cursor: Cursor) -> Result<MappingValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MappingValue)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(MappingValueStruct {
        node_id,
        type_name,
        name,
    }))
}

pub fn build_address_type(cursor: Cursor) -> Result<AddressType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AddressType)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::AddressKeyword)?)?;
    let payable_keyword = if helper.at_label(EdgeLabel::PayableKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::PayableKeyword)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(AddressTypeStruct {
        node_id,
        payable_keyword,
    }))
}

pub fn build_block(cursor: Cursor) -> Result<Block> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Block)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let statements = build_statements(helper.accept_label(EdgeLabel::Statements)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(BlockStruct {
        node_id,
        statements,
    }))
}

pub fn build_unchecked_block(cursor: Cursor) -> Result<UncheckedBlock> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UncheckedBlock)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::UncheckedKeyword)?)?;
    let block = build_block(helper.accept_label(EdgeLabel::Block)?)?;
    helper.finalize()?;

    Ok(Rc::new(UncheckedBlockStruct { node_id, block }))
}

pub fn build_expression_statement(cursor: Cursor) -> Result<ExpressionStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ExpressionStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ExpressionStatementStruct {
        node_id,
        expression,
    }))
}

pub fn build_assembly_statement(cursor: Cursor) -> Result<AssemblyStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AssemblyStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::AssemblyKeyword)?)?;
    let label = if helper.at_label(EdgeLabel::Label) {
        Some(build_string_literal(
            helper.accept_label(EdgeLabel::Label)?,
        )?)
    } else {
        None
    };
    let flags = if helper.at_label(EdgeLabel::Flags) {
        Some(build_assembly_flags_declaration(
            helper.accept_label(EdgeLabel::Flags)?,
        )?)
    } else {
        None
    };
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(AssemblyStatementStruct {
        node_id,
        label,
        flags,
        body,
    }))
}

pub fn build_assembly_flags_declaration(cursor: Cursor) -> Result<AssemblyFlagsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AssemblyFlagsDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let flags = build_assembly_flags(helper.accept_label(EdgeLabel::Flags)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(AssemblyFlagsDeclarationStruct { node_id, flags }))
}

pub fn build_tuple_deconstruction_statement(
    cursor: Cursor,
) -> Result<TupleDeconstructionStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleDeconstructionStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let var_keyword = if helper.at_label(EdgeLabel::VarKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::VarKeyword)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let elements = build_tuple_deconstruction_elements(helper.accept_label(EdgeLabel::Elements)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(TupleDeconstructionStatementStruct {
        node_id,
        var_keyword,
        elements,
        expression,
    }))
}

pub fn build_tuple_deconstruction_element(cursor: Cursor) -> Result<TupleDeconstructionElement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleDeconstructionElement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let member = if helper.at_label(EdgeLabel::Member) {
        Some(build_tuple_member(helper.accept_label(EdgeLabel::Member)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(TupleDeconstructionElementStruct {
        node_id,
        member,
    }))
}

pub fn build_typed_tuple_member(cursor: Cursor) -> Result<TypedTupleMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TypedTupleMember)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let storage_location = if helper.at_label(EdgeLabel::StorageLocation) {
        Some(build_storage_location(
            helper.accept_label(EdgeLabel::StorageLocation)?,
        )?)
    } else {
        None
    };
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    helper.finalize()?;

    Ok(Rc::new(TypedTupleMemberStruct {
        node_id,
        type_name,
        storage_location,
        name,
    }))
}

pub fn build_untyped_tuple_member(cursor: Cursor) -> Result<UntypedTupleMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UntypedTupleMember)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let storage_location = if helper.at_label(EdgeLabel::StorageLocation) {
        Some(build_storage_location(
            helper.accept_label(EdgeLabel::StorageLocation)?,
        )?)
    } else {
        None
    };
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    helper.finalize()?;

    Ok(Rc::new(UntypedTupleMemberStruct {
        node_id,
        storage_location,
        name,
    }))
}

pub fn build_variable_declaration_statement(
    cursor: Cursor,
) -> Result<VariableDeclarationStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VariableDeclarationStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let variable_type =
        build_variable_declaration_type(helper.accept_label(EdgeLabel::VariableType)?)?;
    let storage_location = if helper.at_label(EdgeLabel::StorageLocation) {
        Some(build_storage_location(
            helper.accept_label(EdgeLabel::StorageLocation)?,
        )?)
    } else {
        None
    };
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let value = if helper.at_label(EdgeLabel::Value) {
        Some(build_variable_declaration_value(
            helper.accept_label(EdgeLabel::Value)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(VariableDeclarationStatementStruct {
        node_id,
        variable_type,
        storage_location,
        name,
        value,
    }))
}

pub fn build_variable_declaration_value(cursor: Cursor) -> Result<VariableDeclarationValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VariableDeclarationValue)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    helper.finalize()?;

    Ok(Rc::new(VariableDeclarationValueStruct {
        node_id,
        expression,
    }))
}

pub fn build_if_statement(cursor: Cursor) -> Result<IfStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::IfStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::IfKeyword)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let condition = build_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    let else_branch = if helper.at_label(EdgeLabel::ElseBranch) {
        Some(build_else_branch(
            helper.accept_label(EdgeLabel::ElseBranch)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(IfStatementStruct {
        node_id,
        condition,
        body,
        else_branch,
    }))
}

pub fn build_else_branch(cursor: Cursor) -> Result<ElseBranch> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ElseBranch)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ElseKeyword)?)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(ElseBranchStruct { node_id, body }))
}

pub fn build_for_statement(cursor: Cursor) -> Result<ForStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ForStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ForKeyword)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let initialization =
        build_for_statement_initialization(helper.accept_label(EdgeLabel::Initialization)?)?;
    let condition = build_for_statement_condition(helper.accept_label(EdgeLabel::Condition)?)?;
    let iterator = if helper.at_label(EdgeLabel::Iterator) {
        Some(build_expression(helper.accept_label(EdgeLabel::Iterator)?)?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(ForStatementStruct {
        node_id,
        initialization,
        condition,
        iterator,
        body,
    }))
}

pub fn build_while_statement(cursor: Cursor) -> Result<WhileStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::WhileStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::WhileKeyword)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let condition = build_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(WhileStatementStruct {
        node_id,
        condition,
        body,
    }))
}

pub fn build_do_while_statement(cursor: Cursor) -> Result<DoWhileStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::DoWhileStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::DoKeyword)?)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::WhileKeyword)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let condition = build_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(DoWhileStatementStruct {
        node_id,
        body,
        condition,
    }))
}

pub fn build_continue_statement(cursor: Cursor) -> Result<ContinueStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ContinueStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ContinueKeyword)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ContinueStatementStruct { node_id }))
}

pub fn build_break_statement(cursor: Cursor) -> Result<BreakStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::BreakStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::BreakKeyword)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(BreakStatementStruct { node_id }))
}

pub fn build_return_statement(cursor: Cursor) -> Result<ReturnStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ReturnStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ReturnKeyword)?)?;
    let expression = if helper.at_label(EdgeLabel::Expression) {
        Some(build_expression(
            helper.accept_label(EdgeLabel::Expression)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ReturnStatementStruct {
        node_id,
        expression,
    }))
}

pub fn build_emit_statement(cursor: Cursor) -> Result<EmitStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EmitStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::EmitKeyword)?)?;
    let event = build_identifier_path(helper.accept_label(EdgeLabel::Event)?)?;
    let arguments = build_arguments_declaration(helper.accept_label(EdgeLabel::Arguments)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(EmitStatementStruct {
        node_id,
        event,
        arguments,
    }))
}

pub fn build_try_statement(cursor: Cursor) -> Result<TryStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TryStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::TryKeyword)?)?;
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_returns_declaration(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)
    } else {
        None
    };
    let body = build_block(helper.accept_label(EdgeLabel::Body)?)?;
    let catch_clauses = build_catch_clauses(helper.accept_label(EdgeLabel::CatchClauses)?)?;
    helper.finalize()?;

    Ok(Rc::new(TryStatementStruct {
        node_id,
        expression,
        returns,
        body,
        catch_clauses,
    }))
}

pub fn build_catch_clause(cursor: Cursor) -> Result<CatchClause> {
    expect_nonterminal_kind(&cursor, NonterminalKind::CatchClause)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CatchKeyword)?)?;
    let error = if helper.at_label(EdgeLabel::Error) {
        Some(build_catch_clause_error(
            helper.accept_label(EdgeLabel::Error)?,
        )?)
    } else {
        None
    };
    let body = build_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(CatchClauseStruct {
        node_id,
        error,
        body,
    }))
}

pub fn build_catch_clause_error(cursor: Cursor) -> Result<CatchClauseError> {
    expect_nonterminal_kind(&cursor, NonterminalKind::CatchClauseError)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    helper.finalize()?;

    Ok(Rc::new(CatchClauseErrorStruct {
        node_id,
        name,
        parameters,
    }))
}

pub fn build_revert_statement(cursor: Cursor) -> Result<RevertStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::RevertStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::RevertKeyword)?)?;
    let error = if helper.at_label(EdgeLabel::Error) {
        Some(build_identifier_path(
            helper.accept_label(EdgeLabel::Error)?,
        )?)
    } else {
        None
    };
    let arguments = build_arguments_declaration(helper.accept_label(EdgeLabel::Arguments)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(RevertStatementStruct {
        node_id,
        error,
        arguments,
    }))
}

pub fn build_throw_statement(cursor: Cursor) -> Result<ThrowStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ThrowStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ThrowKeyword)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ThrowStatementStruct { node_id }))
}

pub fn build_assignment_expression(cursor: Cursor) -> Result<AssignmentExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AssignmentExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(AssignmentExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_conditional_expression(cursor: Cursor) -> Result<ConditionalExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ConditionalExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::QuestionMark)?)?;
    let true_expression = build_expression(helper.accept_label(EdgeLabel::TrueExpression)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    let false_expression = build_expression(helper.accept_label(EdgeLabel::FalseExpression)?)?;
    helper.finalize()?;

    Ok(Rc::new(ConditionalExpressionStruct {
        node_id,
        operand,
        true_expression,
        false_expression,
    }))
}

pub fn build_or_expression(cursor: Cursor) -> Result<OrExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::OrExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(OrExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_and_expression(cursor: Cursor) -> Result<AndExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AndExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(AndExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_equality_expression(cursor: Cursor) -> Result<EqualityExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EqualityExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(EqualityExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_inequality_expression(cursor: Cursor) -> Result<InequalityExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InequalityExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(InequalityExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_bitwise_or_expression(cursor: Cursor) -> Result<BitwiseOrExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::BitwiseOrExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(BitwiseOrExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_bitwise_xor_expression(cursor: Cursor) -> Result<BitwiseXorExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::BitwiseXorExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(BitwiseXorExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_bitwise_and_expression(cursor: Cursor) -> Result<BitwiseAndExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::BitwiseAndExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(BitwiseAndExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_shift_expression(cursor: Cursor) -> Result<ShiftExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ShiftExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(ShiftExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_additive_expression(cursor: Cursor) -> Result<AdditiveExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AdditiveExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(AdditiveExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_multiplicative_expression(cursor: Cursor) -> Result<MultiplicativeExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MultiplicativeExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(MultiplicativeExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_exponentiation_expression(cursor: Cursor) -> Result<ExponentiationExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ExponentiationExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(ExponentiationExpressionStruct {
        node_id,
        left_operand,
        right_operand,
    }))
}

pub fn build_postfix_expression(cursor: Cursor) -> Result<PostfixExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PostfixExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    helper.finalize()?;

    Ok(Rc::new(PostfixExpressionStruct { node_id, operand }))
}

pub fn build_prefix_expression(cursor: Cursor) -> Result<PrefixExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PrefixExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    helper.finalize()?;

    Ok(Rc::new(PrefixExpressionStruct { node_id, operand }))
}

pub fn build_function_call_expression(cursor: Cursor) -> Result<FunctionCallExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionCallExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let arguments = build_arguments_declaration(helper.accept_label(EdgeLabel::Arguments)?)?;
    helper.finalize()?;

    Ok(Rc::new(FunctionCallExpressionStruct {
        node_id,
        operand,
        arguments,
    }))
}

pub fn build_call_options_expression(cursor: Cursor) -> Result<CallOptionsExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::CallOptionsExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let options = build_call_options(helper.accept_label(EdgeLabel::Options)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(CallOptionsExpressionStruct {
        node_id,
        operand,
        options,
    }))
}

pub fn build_member_access_expression(cursor: Cursor) -> Result<MemberAccessExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MemberAccessExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Period)?)?;
    let member = fetch_terminal_node(&helper.accept_label(EdgeLabel::Member)?)?;
    helper.finalize()?;

    Ok(Rc::new(MemberAccessExpressionStruct {
        node_id,
        operand,
        member,
    }))
}

pub fn build_index_access_expression(cursor: Cursor) -> Result<IndexAccessExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::IndexAccessExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBracket)?)?;
    let start = if helper.at_label(EdgeLabel::Start) {
        Some(build_expression(helper.accept_label(EdgeLabel::Start)?)?)
    } else {
        None
    };
    let end = if helper.at_label(EdgeLabel::End) {
        Some(build_index_access_end(
            helper.accept_label(EdgeLabel::End)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBracket)?)?;
    helper.finalize()?;

    Ok(Rc::new(IndexAccessExpressionStruct {
        node_id,
        operand,
        start,
        end,
    }))
}

pub fn build_index_access_end(cursor: Cursor) -> Result<IndexAccessEnd> {
    expect_nonterminal_kind(&cursor, NonterminalKind::IndexAccessEnd)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    let end = if helper.at_label(EdgeLabel::End) {
        Some(build_expression(helper.accept_label(EdgeLabel::End)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(IndexAccessEndStruct { node_id, end }))
}

pub fn build_positional_arguments_declaration(
    cursor: Cursor,
) -> Result<PositionalArgumentsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PositionalArgumentsDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let arguments = build_positional_arguments(helper.accept_label(EdgeLabel::Arguments)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(PositionalArgumentsDeclarationStruct {
        node_id,
        arguments,
    }))
}

pub fn build_named_arguments_declaration(cursor: Cursor) -> Result<NamedArgumentsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NamedArgumentsDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let arguments = if helper.at_label(EdgeLabel::Arguments) {
        Some(build_named_argument_group(
            helper.accept_label(EdgeLabel::Arguments)?,
        )?)
    } else {
        None
    };
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedArgumentsDeclarationStruct {
        node_id,
        arguments,
    }))
}

pub fn build_named_argument_group(cursor: Cursor) -> Result<NamedArgumentGroup> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NamedArgumentGroup)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let arguments = build_named_arguments(helper.accept_label(EdgeLabel::Arguments)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedArgumentGroupStruct { node_id, arguments }))
}

pub fn build_named_argument(cursor: Cursor) -> Result<NamedArgument> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NamedArgument)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    let value = build_expression(helper.accept_label(EdgeLabel::Value)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedArgumentStruct {
        node_id,
        name,
        value,
    }))
}

pub fn build_type_expression(cursor: Cursor) -> Result<TypeExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TypeExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::TypeKeyword)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(TypeExpressionStruct { node_id, type_name }))
}

pub fn build_new_expression(cursor: Cursor) -> Result<NewExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NewExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::NewKeyword)?)?;
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    helper.finalize()?;

    Ok(Rc::new(NewExpressionStruct { node_id, type_name }))
}

pub fn build_tuple_expression(cursor: Cursor) -> Result<TupleExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let items = build_tuple_values(helper.accept_label(EdgeLabel::Items)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(TupleExpressionStruct { node_id, items }))
}

pub fn build_tuple_value(cursor: Cursor) -> Result<TupleValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleValue)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let expression = if helper.at_label(EdgeLabel::Expression) {
        Some(build_expression(
            helper.accept_label(EdgeLabel::Expression)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(TupleValueStruct {
        node_id,
        expression,
    }))
}

pub fn build_array_expression(cursor: Cursor) -> Result<ArrayExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ArrayExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBracket)?)?;
    let items = build_array_values(helper.accept_label(EdgeLabel::Items)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBracket)?)?;
    helper.finalize()?;

    Ok(Rc::new(ArrayExpressionStruct { node_id, items }))
}

pub fn build_hex_number_expression(cursor: Cursor) -> Result<HexNumberExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::HexNumberExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let literal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Literal)?)?;
    let unit = if helper.at_label(EdgeLabel::Unit) {
        Some(build_number_unit(helper.accept_label(EdgeLabel::Unit)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(HexNumberExpressionStruct {
        node_id,
        literal,
        unit,
    }))
}

pub fn build_decimal_number_expression(cursor: Cursor) -> Result<DecimalNumberExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::DecimalNumberExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let literal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Literal)?)?;
    let unit = if helper.at_label(EdgeLabel::Unit) {
        Some(build_number_unit(helper.accept_label(EdgeLabel::Unit)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(DecimalNumberExpressionStruct {
        node_id,
        literal,
        unit,
    }))
}

pub fn build_yul_block(cursor: Cursor) -> Result<YulBlock> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulBlock)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let statements = build_yul_statements(helper.accept_label(EdgeLabel::Statements)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulBlockStruct {
        node_id,
        statements,
    }))
}

pub fn build_yul_function_definition(cursor: Cursor) -> Result<YulFunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulFunctionDefinition)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::FunctionKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let parameters = build_yul_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let returns = if helper.at_label(EdgeLabel::Returns) {
        Some(build_yul_returns_declaration(
            helper.accept_label(EdgeLabel::Returns)?,
        )?)
    } else {
        None
    };
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulFunctionDefinitionStruct {
        node_id,
        name,
        parameters,
        returns,
        body,
    }))
}

pub fn build_yul_parameters_declaration(cursor: Cursor) -> Result<YulParametersDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulParametersDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let parameters = build_yul_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulParametersDeclarationStruct {
        node_id,
        parameters,
    }))
}

pub fn build_yul_returns_declaration(cursor: Cursor) -> Result<YulReturnsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulReturnsDeclaration)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::MinusGreaterThan)?)?;
    let variables = build_yul_variable_names(helper.accept_label(EdgeLabel::Variables)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulReturnsDeclarationStruct { node_id, variables }))
}

pub fn build_yul_variable_declaration_statement(
    cursor: Cursor,
) -> Result<YulVariableDeclarationStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulVariableDeclarationStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::LetKeyword)?)?;
    let variables = build_yul_variable_names(helper.accept_label(EdgeLabel::Variables)?)?;
    let value = if helper.at_label(EdgeLabel::Value) {
        Some(build_yul_variable_declaration_value(
            helper.accept_label(EdgeLabel::Value)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(YulVariableDeclarationStatementStruct {
        node_id,
        variables,
        value,
    }))
}

pub fn build_yul_variable_declaration_value(cursor: Cursor) -> Result<YulVariableDeclarationValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulVariableDeclarationValue)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let assignment = build_yul_assignment_operator(helper.accept_label(EdgeLabel::Assignment)?)?;
    let expression = build_yul_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulVariableDeclarationValueStruct {
        node_id,
        assignment,
        expression,
    }))
}

pub fn build_yul_variable_assignment_statement(
    cursor: Cursor,
) -> Result<YulVariableAssignmentStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulVariableAssignmentStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let variables = build_yul_paths(helper.accept_label(EdgeLabel::Variables)?)?;
    let assignment = build_yul_assignment_operator(helper.accept_label(EdgeLabel::Assignment)?)?;
    let expression = build_yul_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulVariableAssignmentStatementStruct {
        node_id,
        variables,
        assignment,
        expression,
    }))
}

pub fn build_yul_colon_and_equal(cursor: Cursor) -> Result<YulColonAndEqual> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulColonAndEqual)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulColonAndEqualStruct { node_id }))
}

pub fn build_yul_stack_assignment_statement(cursor: Cursor) -> Result<YulStackAssignmentStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulStackAssignmentStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let assignment =
        build_yul_stack_assignment_operator(helper.accept_label(EdgeLabel::Assignment)?)?;
    let variable = fetch_terminal_node(&helper.accept_label(EdgeLabel::Variable)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulStackAssignmentStatementStruct {
        node_id,
        assignment,
        variable,
    }))
}

pub fn build_yul_equal_and_colon(cursor: Cursor) -> Result<YulEqualAndColon> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulEqualAndColon)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulEqualAndColonStruct { node_id }))
}

pub fn build_yul_if_statement(cursor: Cursor) -> Result<YulIfStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulIfStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::IfKeyword)?)?;
    let condition = build_yul_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulIfStatementStruct {
        node_id,
        condition,
        body,
    }))
}

pub fn build_yul_for_statement(cursor: Cursor) -> Result<YulForStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulForStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ForKeyword)?)?;
    let initialization = build_yul_block(helper.accept_label(EdgeLabel::Initialization)?)?;
    let condition = build_yul_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    let iterator = build_yul_block(helper.accept_label(EdgeLabel::Iterator)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulForStatementStruct {
        node_id,
        initialization,
        condition,
        iterator,
        body,
    }))
}

pub fn build_yul_switch_statement(cursor: Cursor) -> Result<YulSwitchStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulSwitchStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::SwitchKeyword)?)?;
    let expression = build_yul_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    let cases = build_yul_switch_cases(helper.accept_label(EdgeLabel::Cases)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulSwitchStatementStruct {
        node_id,
        expression,
        cases,
    }))
}

pub fn build_yul_default_case(cursor: Cursor) -> Result<YulDefaultCase> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulDefaultCase)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::DefaultKeyword)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulDefaultCaseStruct { node_id, body }))
}

pub fn build_yul_value_case(cursor: Cursor) -> Result<YulValueCase> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulValueCase)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CaseKeyword)?)?;
    let value = build_yul_literal(helper.accept_label(EdgeLabel::Value)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulValueCaseStruct {
        node_id,
        value,
        body,
    }))
}

pub fn build_yul_leave_statement(cursor: Cursor) -> Result<YulLeaveStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulLeaveStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::LeaveKeyword)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulLeaveStatementStruct { node_id }))
}

pub fn build_yul_break_statement(cursor: Cursor) -> Result<YulBreakStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulBreakStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::BreakKeyword)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulBreakStatementStruct { node_id }))
}

pub fn build_yul_continue_statement(cursor: Cursor) -> Result<YulContinueStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulContinueStatement)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::ContinueKeyword)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulContinueStatementStruct { node_id }))
}

pub fn build_yul_label(cursor: Cursor) -> Result<YulLabel> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulLabel)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let label = fetch_terminal_node(&helper.accept_label(EdgeLabel::Label)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulLabelStruct { node_id, label }))
}

pub fn build_yul_function_call_expression(cursor: Cursor) -> Result<YulFunctionCallExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulFunctionCallExpression)?;
    let node_id = cursor.node().id();
    let mut helper = SequenceHelper::new(cursor);
    let operand = build_yul_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let arguments = build_yul_arguments(helper.accept_label(EdgeLabel::Arguments)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulFunctionCallExpressionStruct {
        node_id,
        operand,
        arguments,
    }))
}

//
// Choices:
//

pub fn build_source_unit_member(mut cursor: Cursor) -> Result<SourceUnitMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SourceUnitMember)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::PragmaDirective) => {
            SourceUnitMember::PragmaDirective(build_pragma_directive(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ImportDirective) => {
            SourceUnitMember::ImportDirective(build_import_directive(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ContractDefinition) => {
            SourceUnitMember::ContractDefinition(build_contract_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::InterfaceDefinition) => {
            SourceUnitMember::InterfaceDefinition(build_interface_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::LibraryDefinition) => {
            SourceUnitMember::LibraryDefinition(build_library_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::StructDefinition) => {
            SourceUnitMember::StructDefinition(build_struct_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::EnumDefinition) => {
            SourceUnitMember::EnumDefinition(build_enum_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionDefinition) => {
            SourceUnitMember::FunctionDefinition(build_function_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ErrorDefinition) => {
            SourceUnitMember::ErrorDefinition(build_error_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::UserDefinedValueTypeDefinition) => {
            SourceUnitMember::UserDefinedValueTypeDefinition(
                build_user_defined_value_type_definition(cursor.clone())?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::UsingDirective) => {
            SourceUnitMember::UsingDirective(build_using_directive(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::EventDefinition) => {
            SourceUnitMember::EventDefinition(build_event_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ConstantDefinition) => {
            SourceUnitMember::ConstantDefinition(build_constant_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_pragma(mut cursor: Cursor) -> Result<Pragma> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Pragma)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::AbicoderPragma) => {
            Pragma::AbicoderPragma(build_abicoder_pragma(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ExperimentalPragma) => {
            Pragma::ExperimentalPragma(build_experimental_pragma(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::VersionPragma) => {
            Pragma::VersionPragma(build_version_pragma(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_experimental_feature(mut cursor: Cursor) -> Result<ExperimentalFeature> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ExperimentalFeature)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::StringLiteral) => {
            ExperimentalFeature::StringLiteral(build_string_literal(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::Identifier) => {
            let node = fetch_terminal_node(&cursor)?;
            ExperimentalFeature::Identifier(node)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_version_expression(mut cursor: Cursor) -> Result<VersionExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionExpression)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::VersionRange) => {
            VersionExpression::VersionRange(build_version_range(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::VersionTerm) => {
            VersionExpression::VersionTerm(build_version_term(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_version_operator(mut cursor: Cursor) -> Result<VersionOperator> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionOperator)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Terminal(TerminalKind::Caret) => {
            _ = fetch_terminal_node(&cursor)?;
            VersionOperator::Caret
        }
        NodeKind::Terminal(TerminalKind::Tilde) => {
            _ = fetch_terminal_node(&cursor)?;
            VersionOperator::Tilde
        }
        NodeKind::Terminal(TerminalKind::Equal) => {
            _ = fetch_terminal_node(&cursor)?;
            VersionOperator::Equal
        }
        NodeKind::Terminal(TerminalKind::LessThan) => {
            _ = fetch_terminal_node(&cursor)?;
            VersionOperator::LessThan
        }
        NodeKind::Terminal(TerminalKind::GreaterThan) => {
            _ = fetch_terminal_node(&cursor)?;
            VersionOperator::GreaterThan
        }
        NodeKind::Terminal(TerminalKind::LessThanEqual) => {
            _ = fetch_terminal_node(&cursor)?;
            VersionOperator::LessThanEqual
        }
        NodeKind::Terminal(TerminalKind::GreaterThanEqual) => {
            _ = fetch_terminal_node(&cursor)?;
            VersionOperator::GreaterThanEqual
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_version_literal(mut cursor: Cursor) -> Result<VersionLiteral> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionLiteral)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::SimpleVersionLiteral) => {
            VersionLiteral::SimpleVersionLiteral(build_simple_version_literal(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::SingleQuotedVersionLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            VersionLiteral::SingleQuotedVersionLiteral(node)
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedVersionLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            VersionLiteral::DoubleQuotedVersionLiteral(node)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_import_clause(mut cursor: Cursor) -> Result<ImportClause> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportClause)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::PathImport) => {
            ImportClause::PathImport(build_path_import(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::NamedImport) => {
            ImportClause::NamedImport(build_named_import(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ImportDeconstruction) => {
            ImportClause::ImportDeconstruction(build_import_deconstruction(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_using_clause(mut cursor: Cursor) -> Result<UsingClause> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingClause)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::IdentifierPath) => {
            UsingClause::IdentifierPath(build_identifier_path(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::UsingDeconstruction) => {
            UsingClause::UsingDeconstruction(build_using_deconstruction(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_using_operator(mut cursor: Cursor) -> Result<UsingOperator> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingOperator)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Terminal(TerminalKind::Ampersand) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::Ampersand
        }
        NodeKind::Terminal(TerminalKind::Asterisk) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::Asterisk
        }
        NodeKind::Terminal(TerminalKind::BangEqual) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::BangEqual
        }
        NodeKind::Terminal(TerminalKind::Bar) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::Bar
        }
        NodeKind::Terminal(TerminalKind::Caret) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::Caret
        }
        NodeKind::Terminal(TerminalKind::EqualEqual) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::EqualEqual
        }
        NodeKind::Terminal(TerminalKind::GreaterThan) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::GreaterThan
        }
        NodeKind::Terminal(TerminalKind::GreaterThanEqual) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::GreaterThanEqual
        }
        NodeKind::Terminal(TerminalKind::LessThan) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::LessThan
        }
        NodeKind::Terminal(TerminalKind::LessThanEqual) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::LessThanEqual
        }
        NodeKind::Terminal(TerminalKind::Minus) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::Minus
        }
        NodeKind::Terminal(TerminalKind::Percent) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::Percent
        }
        NodeKind::Terminal(TerminalKind::Plus) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::Plus
        }
        NodeKind::Terminal(TerminalKind::Slash) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::Slash
        }
        NodeKind::Terminal(TerminalKind::Tilde) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingOperator::Tilde
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_using_target(mut cursor: Cursor) -> Result<UsingTarget> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingTarget)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::TypeName) => {
            UsingTarget::TypeName(build_type_name(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::Asterisk) => {
            _ = fetch_terminal_node(&cursor)?;
            UsingTarget::Asterisk
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_contract_member(mut cursor: Cursor) -> Result<ContractMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ContractMember)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::UsingDirective) => {
            ContractMember::UsingDirective(build_using_directive(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionDefinition) => {
            ContractMember::FunctionDefinition(build_function_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ConstructorDefinition) => {
            ContractMember::ConstructorDefinition(build_constructor_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ReceiveFunctionDefinition) => {
            ContractMember::ReceiveFunctionDefinition(build_receive_function_definition(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::FallbackFunctionDefinition) => {
            ContractMember::FallbackFunctionDefinition(build_fallback_function_definition(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::UnnamedFunctionDefinition) => {
            ContractMember::UnnamedFunctionDefinition(build_unnamed_function_definition(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::ModifierDefinition) => {
            ContractMember::ModifierDefinition(build_modifier_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::StructDefinition) => {
            ContractMember::StructDefinition(build_struct_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::EnumDefinition) => {
            ContractMember::EnumDefinition(build_enum_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::EventDefinition) => {
            ContractMember::EventDefinition(build_event_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ErrorDefinition) => {
            ContractMember::ErrorDefinition(build_error_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::UserDefinedValueTypeDefinition) => {
            ContractMember::UserDefinedValueTypeDefinition(
                build_user_defined_value_type_definition(cursor.clone())?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::StateVariableDefinition) => {
            ContractMember::StateVariableDefinition(build_state_variable_definition(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_state_variable_attribute(mut cursor: Cursor) -> Result<StateVariableAttribute> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StateVariableAttribute)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            StateVariableAttribute::OverrideSpecifier(build_override_specifier(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::ConstantKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            StateVariableAttribute::ConstantKeyword
        }
        NodeKind::Terminal(TerminalKind::InternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            StateVariableAttribute::InternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PrivateKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            StateVariableAttribute::PrivateKeyword
        }
        NodeKind::Terminal(TerminalKind::PublicKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            StateVariableAttribute::PublicKeyword
        }
        NodeKind::Terminal(TerminalKind::ImmutableKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            StateVariableAttribute::ImmutableKeyword
        }
        NodeKind::Terminal(TerminalKind::TransientKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            StateVariableAttribute::TransientKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_function_name(mut cursor: Cursor) -> Result<FunctionName> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionName)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Terminal(TerminalKind::Identifier) => {
            let node = fetch_terminal_node(&cursor)?;
            FunctionName::Identifier(node)
        }
        NodeKind::Terminal(TerminalKind::FallbackKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionName::FallbackKeyword
        }
        NodeKind::Terminal(TerminalKind::ReceiveKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionName::ReceiveKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_function_attribute(mut cursor: Cursor) -> Result<FunctionAttribute> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionAttribute)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            FunctionAttribute::ModifierInvocation(build_modifier_invocation(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            FunctionAttribute::OverrideSpecifier(build_override_specifier(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::ConstantKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionAttribute::ConstantKeyword
        }
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionAttribute::ExternalKeyword
        }
        NodeKind::Terminal(TerminalKind::InternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionAttribute::InternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionAttribute::PayableKeyword
        }
        NodeKind::Terminal(TerminalKind::PrivateKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionAttribute::PrivateKeyword
        }
        NodeKind::Terminal(TerminalKind::PublicKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionAttribute::PublicKeyword
        }
        NodeKind::Terminal(TerminalKind::PureKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionAttribute::PureKeyword
        }
        NodeKind::Terminal(TerminalKind::ViewKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionAttribute::ViewKeyword
        }
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionAttribute::VirtualKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_function_body(mut cursor: Cursor) -> Result<FunctionBody> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionBody)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::Block) => {
            FunctionBody::Block(build_block(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::Semicolon) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionBody::Semicolon
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_constructor_attribute(mut cursor: Cursor) -> Result<ConstructorAttribute> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ConstructorAttribute)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            ConstructorAttribute::ModifierInvocation(build_modifier_invocation(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::InternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ConstructorAttribute::InternalKeyword
        }
        NodeKind::Terminal(TerminalKind::OverrideKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ConstructorAttribute::OverrideKeyword
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ConstructorAttribute::PayableKeyword
        }
        NodeKind::Terminal(TerminalKind::PublicKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ConstructorAttribute::PublicKeyword
        }
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ConstructorAttribute::VirtualKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_unnamed_function_attribute(mut cursor: Cursor) -> Result<UnnamedFunctionAttribute> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UnnamedFunctionAttribute)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            UnnamedFunctionAttribute::ModifierInvocation(build_modifier_invocation(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::ConstantKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            UnnamedFunctionAttribute::ConstantKeyword
        }
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            UnnamedFunctionAttribute::ExternalKeyword
        }
        NodeKind::Terminal(TerminalKind::InternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            UnnamedFunctionAttribute::InternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            UnnamedFunctionAttribute::PayableKeyword
        }
        NodeKind::Terminal(TerminalKind::PrivateKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            UnnamedFunctionAttribute::PrivateKeyword
        }
        NodeKind::Terminal(TerminalKind::PublicKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            UnnamedFunctionAttribute::PublicKeyword
        }
        NodeKind::Terminal(TerminalKind::PureKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            UnnamedFunctionAttribute::PureKeyword
        }
        NodeKind::Terminal(TerminalKind::ViewKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            UnnamedFunctionAttribute::ViewKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_fallback_function_attribute(mut cursor: Cursor) -> Result<FallbackFunctionAttribute> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FallbackFunctionAttribute)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            FallbackFunctionAttribute::ModifierInvocation(build_modifier_invocation(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            FallbackFunctionAttribute::OverrideSpecifier(build_override_specifier(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FallbackFunctionAttribute::ExternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FallbackFunctionAttribute::PayableKeyword
        }
        NodeKind::Terminal(TerminalKind::PureKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FallbackFunctionAttribute::PureKeyword
        }
        NodeKind::Terminal(TerminalKind::ViewKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FallbackFunctionAttribute::ViewKeyword
        }
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FallbackFunctionAttribute::VirtualKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_receive_function_attribute(mut cursor: Cursor) -> Result<ReceiveFunctionAttribute> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ReceiveFunctionAttribute)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) => {
            ReceiveFunctionAttribute::ModifierInvocation(build_modifier_invocation(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            ReceiveFunctionAttribute::OverrideSpecifier(build_override_specifier(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ReceiveFunctionAttribute::ExternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ReceiveFunctionAttribute::PayableKeyword
        }
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ReceiveFunctionAttribute::VirtualKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_modifier_attribute(mut cursor: Cursor) -> Result<ModifierAttribute> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ModifierAttribute)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) => {
            ModifierAttribute::OverrideSpecifier(build_override_specifier(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::VirtualKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ModifierAttribute::VirtualKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_type_name(mut cursor: Cursor) -> Result<TypeName> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TypeName)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::ArrayTypeName) => {
            TypeName::ArrayTypeName(build_array_type_name(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionType) => {
            TypeName::FunctionType(build_function_type(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::MappingType) => {
            TypeName::MappingType(build_mapping_type(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ElementaryType) => {
            TypeName::ElementaryType(build_elementary_type(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::IdentifierPath) => {
            TypeName::IdentifierPath(build_identifier_path(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_function_type_attribute(mut cursor: Cursor) -> Result<FunctionTypeAttribute> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionTypeAttribute)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Terminal(TerminalKind::InternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionTypeAttribute::InternalKeyword
        }
        NodeKind::Terminal(TerminalKind::ExternalKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionTypeAttribute::ExternalKeyword
        }
        NodeKind::Terminal(TerminalKind::PrivateKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionTypeAttribute::PrivateKeyword
        }
        NodeKind::Terminal(TerminalKind::PublicKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionTypeAttribute::PublicKeyword
        }
        NodeKind::Terminal(TerminalKind::ConstantKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionTypeAttribute::ConstantKeyword
        }
        NodeKind::Terminal(TerminalKind::PureKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionTypeAttribute::PureKeyword
        }
        NodeKind::Terminal(TerminalKind::ViewKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionTypeAttribute::ViewKeyword
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            FunctionTypeAttribute::PayableKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_mapping_key_type(mut cursor: Cursor) -> Result<MappingKeyType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MappingKeyType)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::ElementaryType) => {
            MappingKeyType::ElementaryType(build_elementary_type(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::IdentifierPath) => {
            MappingKeyType::IdentifierPath(build_identifier_path(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_elementary_type(mut cursor: Cursor) -> Result<ElementaryType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ElementaryType)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::AddressType) => {
            ElementaryType::AddressType(build_address_type(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::BoolKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ElementaryType::BoolKeyword
        }
        NodeKind::Terminal(TerminalKind::ByteKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ElementaryType::ByteKeyword
        }
        NodeKind::Terminal(TerminalKind::StringKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            ElementaryType::StringKeyword
        }
        NodeKind::Terminal(TerminalKind::BytesKeyword) => {
            let node = fetch_terminal_node(&cursor)?;
            ElementaryType::BytesKeyword(node)
        }
        NodeKind::Terminal(TerminalKind::IntKeyword) => {
            let node = fetch_terminal_node(&cursor)?;
            ElementaryType::IntKeyword(node)
        }
        NodeKind::Terminal(TerminalKind::UintKeyword) => {
            let node = fetch_terminal_node(&cursor)?;
            ElementaryType::UintKeyword(node)
        }
        NodeKind::Terminal(TerminalKind::FixedKeyword) => {
            let node = fetch_terminal_node(&cursor)?;
            ElementaryType::FixedKeyword(node)
        }
        NodeKind::Terminal(TerminalKind::UfixedKeyword) => {
            let node = fetch_terminal_node(&cursor)?;
            ElementaryType::UfixedKeyword(node)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_statement(mut cursor: Cursor) -> Result<Statement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Statement)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::IfStatement) => {
            Statement::IfStatement(build_if_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ForStatement) => {
            Statement::ForStatement(build_for_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::WhileStatement) => {
            Statement::WhileStatement(build_while_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::DoWhileStatement) => {
            Statement::DoWhileStatement(build_do_while_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ContinueStatement) => {
            Statement::ContinueStatement(build_continue_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::BreakStatement) => {
            Statement::BreakStatement(build_break_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ReturnStatement) => {
            Statement::ReturnStatement(build_return_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ThrowStatement) => {
            Statement::ThrowStatement(build_throw_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::EmitStatement) => {
            Statement::EmitStatement(build_emit_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::TryStatement) => {
            Statement::TryStatement(build_try_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::RevertStatement) => {
            Statement::RevertStatement(build_revert_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::AssemblyStatement) => {
            Statement::AssemblyStatement(build_assembly_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::Block) => {
            Statement::Block(build_block(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::UncheckedBlock) => {
            Statement::UncheckedBlock(build_unchecked_block(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement) => {
            Statement::TupleDeconstructionStatement(build_tuple_deconstruction_statement(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement) => {
            Statement::VariableDeclarationStatement(build_variable_declaration_statement(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) => {
            Statement::ExpressionStatement(build_expression_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_tuple_member(mut cursor: Cursor) -> Result<TupleMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleMember)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::TypedTupleMember) => {
            TupleMember::TypedTupleMember(build_typed_tuple_member(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::UntypedTupleMember) => {
            TupleMember::UntypedTupleMember(build_untyped_tuple_member(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_variable_declaration_type(mut cursor: Cursor) -> Result<VariableDeclarationType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VariableDeclarationType)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::TypeName) => {
            VariableDeclarationType::TypeName(build_type_name(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::VarKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            VariableDeclarationType::VarKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_storage_location(mut cursor: Cursor) -> Result<StorageLocation> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StorageLocation)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Terminal(TerminalKind::MemoryKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            StorageLocation::MemoryKeyword
        }
        NodeKind::Terminal(TerminalKind::StorageKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            StorageLocation::StorageKeyword
        }
        NodeKind::Terminal(TerminalKind::CallDataKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            StorageLocation::CallDataKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_for_statement_initialization(
    mut cursor: Cursor,
) -> Result<ForStatementInitialization> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ForStatementInitialization)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement) => {
            ForStatementInitialization::TupleDeconstructionStatement(
                build_tuple_deconstruction_statement(cursor.clone())?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement) => {
            ForStatementInitialization::VariableDeclarationStatement(
                build_variable_declaration_statement(cursor.clone())?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) => {
            ForStatementInitialization::ExpressionStatement(build_expression_statement(
                cursor.clone(),
            )?)
        }
        NodeKind::Terminal(TerminalKind::Semicolon) => {
            _ = fetch_terminal_node(&cursor)?;
            ForStatementInitialization::Semicolon
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_for_statement_condition(mut cursor: Cursor) -> Result<ForStatementCondition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ForStatementCondition)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) => {
            ForStatementCondition::ExpressionStatement(build_expression_statement(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::Semicolon) => {
            _ = fetch_terminal_node(&cursor)?;
            ForStatementCondition::Semicolon
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_expression(mut cursor: Cursor) -> Result<Expression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Expression)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::AssignmentExpression) => {
            Expression::AssignmentExpression(build_assignment_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ConditionalExpression) => {
            Expression::ConditionalExpression(build_conditional_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::OrExpression) => {
            Expression::OrExpression(build_or_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::AndExpression) => {
            Expression::AndExpression(build_and_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::EqualityExpression) => {
            Expression::EqualityExpression(build_equality_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::InequalityExpression) => {
            Expression::InequalityExpression(build_inequality_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::BitwiseOrExpression) => {
            Expression::BitwiseOrExpression(build_bitwise_or_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::BitwiseXorExpression) => {
            Expression::BitwiseXorExpression(build_bitwise_xor_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::BitwiseAndExpression) => {
            Expression::BitwiseAndExpression(build_bitwise_and_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ShiftExpression) => {
            Expression::ShiftExpression(build_shift_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::AdditiveExpression) => {
            Expression::AdditiveExpression(build_additive_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::MultiplicativeExpression) => {
            Expression::MultiplicativeExpression(build_multiplicative_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ExponentiationExpression) => {
            Expression::ExponentiationExpression(build_exponentiation_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::PostfixExpression) => {
            Expression::PostfixExpression(build_postfix_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::PrefixExpression) => {
            Expression::PrefixExpression(build_prefix_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::FunctionCallExpression) => {
            Expression::FunctionCallExpression(build_function_call_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::CallOptionsExpression) => {
            Expression::CallOptionsExpression(build_call_options_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::MemberAccessExpression) => {
            Expression::MemberAccessExpression(build_member_access_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::IndexAccessExpression) => {
            Expression::IndexAccessExpression(build_index_access_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::NewExpression) => {
            Expression::NewExpression(build_new_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::TupleExpression) => {
            Expression::TupleExpression(build_tuple_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::TypeExpression) => {
            Expression::TypeExpression(build_type_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ArrayExpression) => {
            Expression::ArrayExpression(build_array_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::HexNumberExpression) => {
            Expression::HexNumberExpression(build_hex_number_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::DecimalNumberExpression) => {
            Expression::DecimalNumberExpression(build_decimal_number_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::StringExpression) => {
            Expression::StringExpression(build_string_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::ElementaryType) => {
            Expression::ElementaryType(build_elementary_type(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::PayableKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            Expression::PayableKeyword
        }
        NodeKind::Terminal(TerminalKind::ThisKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            Expression::ThisKeyword
        }
        NodeKind::Terminal(TerminalKind::SuperKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            Expression::SuperKeyword
        }
        NodeKind::Terminal(TerminalKind::TrueKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            Expression::TrueKeyword
        }
        NodeKind::Terminal(TerminalKind::FalseKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            Expression::FalseKeyword
        }
        NodeKind::Terminal(TerminalKind::Identifier) => {
            let node = fetch_terminal_node(&cursor)?;
            Expression::Identifier(node)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_arguments_declaration(mut cursor: Cursor) -> Result<ArgumentsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ArgumentsDeclaration)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::PositionalArgumentsDeclaration) => {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(
                build_positional_arguments_declaration(cursor.clone())?,
            )
        }
        NodeKind::Nonterminal(NonterminalKind::NamedArgumentsDeclaration) => {
            ArgumentsDeclaration::NamedArgumentsDeclaration(build_named_arguments_declaration(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_number_unit(mut cursor: Cursor) -> Result<NumberUnit> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NumberUnit)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Terminal(TerminalKind::WeiKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::WeiKeyword
        }
        NodeKind::Terminal(TerminalKind::GweiKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::GweiKeyword
        }
        NodeKind::Terminal(TerminalKind::SzaboKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::SzaboKeyword
        }
        NodeKind::Terminal(TerminalKind::FinneyKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::FinneyKeyword
        }
        NodeKind::Terminal(TerminalKind::EtherKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::EtherKeyword
        }
        NodeKind::Terminal(TerminalKind::SecondsKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::SecondsKeyword
        }
        NodeKind::Terminal(TerminalKind::MinutesKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::MinutesKeyword
        }
        NodeKind::Terminal(TerminalKind::HoursKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::HoursKeyword
        }
        NodeKind::Terminal(TerminalKind::DaysKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::DaysKeyword
        }
        NodeKind::Terminal(TerminalKind::WeeksKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::WeeksKeyword
        }
        NodeKind::Terminal(TerminalKind::YearsKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            NumberUnit::YearsKeyword
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_string_expression(mut cursor: Cursor) -> Result<StringExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StringExpression)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::StringLiteral) => {
            StringExpression::StringLiteral(build_string_literal(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::StringLiterals) => {
            StringExpression::StringLiterals(build_string_literals(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::HexStringLiteral) => {
            StringExpression::HexStringLiteral(build_hex_string_literal(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::HexStringLiterals) => {
            StringExpression::HexStringLiterals(build_hex_string_literals(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::UnicodeStringLiterals) => {
            StringExpression::UnicodeStringLiterals(build_unicode_string_literals(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_string_literal(mut cursor: Cursor) -> Result<StringLiteral> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StringLiteral)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Terminal(TerminalKind::SingleQuotedStringLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            StringLiteral::SingleQuotedStringLiteral(node)
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedStringLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            StringLiteral::DoubleQuotedStringLiteral(node)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_hex_string_literal(mut cursor: Cursor) -> Result<HexStringLiteral> {
    expect_nonterminal_kind(&cursor, NonterminalKind::HexStringLiteral)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Terminal(TerminalKind::SingleQuotedHexStringLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            HexStringLiteral::SingleQuotedHexStringLiteral(node)
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedHexStringLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            HexStringLiteral::DoubleQuotedHexStringLiteral(node)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_unicode_string_literal(mut cursor: Cursor) -> Result<UnicodeStringLiteral> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UnicodeStringLiteral)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Terminal(TerminalKind::SingleQuotedUnicodeStringLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(node)
        }
        NodeKind::Terminal(TerminalKind::DoubleQuotedUnicodeStringLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(node)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_yul_statement(mut cursor: Cursor) -> Result<YulStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulStatement)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::YulBlock) => {
            YulStatement::YulBlock(build_yul_block(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulFunctionDefinition) => {
            YulStatement::YulFunctionDefinition(build_yul_function_definition(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulStackAssignmentStatement) => {
            YulStatement::YulStackAssignmentStatement(build_yul_stack_assignment_statement(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulIfStatement) => {
            YulStatement::YulIfStatement(build_yul_if_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulForStatement) => {
            YulStatement::YulForStatement(build_yul_for_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulSwitchStatement) => {
            YulStatement::YulSwitchStatement(build_yul_switch_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulLeaveStatement) => {
            YulStatement::YulLeaveStatement(build_yul_leave_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulBreakStatement) => {
            YulStatement::YulBreakStatement(build_yul_break_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulContinueStatement) => {
            YulStatement::YulContinueStatement(build_yul_continue_statement(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulVariableAssignmentStatement) => {
            YulStatement::YulVariableAssignmentStatement(build_yul_variable_assignment_statement(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulLabel) => {
            YulStatement::YulLabel(build_yul_label(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulVariableDeclarationStatement) => {
            YulStatement::YulVariableDeclarationStatement(build_yul_variable_declaration_statement(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulExpression) => {
            YulStatement::YulExpression(build_yul_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_yul_assignment_operator(mut cursor: Cursor) -> Result<YulAssignmentOperator> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulAssignmentOperator)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::YulColonAndEqual) => {
            YulAssignmentOperator::YulColonAndEqual(build_yul_colon_and_equal(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::ColonEqual) => {
            _ = fetch_terminal_node(&cursor)?;
            YulAssignmentOperator::ColonEqual
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_yul_stack_assignment_operator(
    mut cursor: Cursor,
) -> Result<YulStackAssignmentOperator> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulStackAssignmentOperator)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::YulEqualAndColon) => {
            YulStackAssignmentOperator::YulEqualAndColon(build_yul_equal_and_colon(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::EqualColon) => {
            _ = fetch_terminal_node(&cursor)?;
            YulStackAssignmentOperator::EqualColon
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_yul_switch_case(mut cursor: Cursor) -> Result<YulSwitchCase> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulSwitchCase)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::YulDefaultCase) => {
            YulSwitchCase::YulDefaultCase(build_yul_default_case(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulValueCase) => {
            YulSwitchCase::YulValueCase(build_yul_value_case(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_yul_expression(mut cursor: Cursor) -> Result<YulExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulExpression)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::YulFunctionCallExpression) => {
            YulExpression::YulFunctionCallExpression(build_yul_function_call_expression(
                cursor.clone(),
            )?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulLiteral) => {
            YulExpression::YulLiteral(build_yul_literal(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::YulPath) => {
            YulExpression::YulPath(build_yul_path(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_yul_literal(mut cursor: Cursor) -> Result<YulLiteral> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulLiteral)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::HexStringLiteral) => {
            YulLiteral::HexStringLiteral(build_hex_string_literal(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::StringLiteral) => {
            YulLiteral::StringLiteral(build_string_literal(cursor.clone())?)
        }
        NodeKind::Terminal(TerminalKind::YulTrueKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            YulLiteral::YulTrueKeyword
        }
        NodeKind::Terminal(TerminalKind::YulFalseKeyword) => {
            _ = fetch_terminal_node(&cursor)?;
            YulLiteral::YulFalseKeyword
        }
        NodeKind::Terminal(TerminalKind::YulDecimalLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            YulLiteral::YulDecimalLiteral(node)
        }
        NodeKind::Terminal(TerminalKind::YulHexLiteral) => {
            let node = fetch_terminal_node(&cursor)?;
            YulLiteral::YulHexLiteral(node)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

//
// Repeated:
//

pub fn build_source_unit_members(mut cursor: Cursor) -> Result<SourceUnitMembers> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SourceUnitMembers)?;
    let mut items = SourceUnitMembers::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_source_unit_member(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_version_expression_set(mut cursor: Cursor) -> Result<VersionExpressionSet> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionExpressionSet)?;
    let mut items = VersionExpressionSet::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_version_expression(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_contract_members(mut cursor: Cursor) -> Result<ContractMembers> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ContractMembers)?;
    let mut items = ContractMembers::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_contract_member(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_interface_members(mut cursor: Cursor) -> Result<InterfaceMembers> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InterfaceMembers)?;
    let mut items = InterfaceMembers::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_contract_member(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_library_members(mut cursor: Cursor) -> Result<LibraryMembers> {
    expect_nonterminal_kind(&cursor, NonterminalKind::LibraryMembers)?;
    let mut items = LibraryMembers::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_contract_member(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_struct_members(mut cursor: Cursor) -> Result<StructMembers> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StructMembers)?;
    let mut items = StructMembers::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_struct_member(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_state_variable_attributes(mut cursor: Cursor) -> Result<StateVariableAttributes> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StateVariableAttributes)?;
    let mut items = StateVariableAttributes::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_state_variable_attribute(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_function_attributes(mut cursor: Cursor) -> Result<FunctionAttributes> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionAttributes)?;
    let mut items = FunctionAttributes::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_function_attribute(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_constructor_attributes(mut cursor: Cursor) -> Result<ConstructorAttributes> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ConstructorAttributes)?;
    let mut items = ConstructorAttributes::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_constructor_attribute(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_unnamed_function_attributes(mut cursor: Cursor) -> Result<UnnamedFunctionAttributes> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UnnamedFunctionAttributes)?;
    let mut items = UnnamedFunctionAttributes::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_unnamed_function_attribute(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_fallback_function_attributes(
    mut cursor: Cursor,
) -> Result<FallbackFunctionAttributes> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FallbackFunctionAttributes)?;
    let mut items = FallbackFunctionAttributes::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_fallback_function_attribute(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_receive_function_attributes(mut cursor: Cursor) -> Result<ReceiveFunctionAttributes> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ReceiveFunctionAttributes)?;
    let mut items = ReceiveFunctionAttributes::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_receive_function_attribute(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_modifier_attributes(mut cursor: Cursor) -> Result<ModifierAttributes> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ModifierAttributes)?;
    let mut items = ModifierAttributes::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_modifier_attribute(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_function_type_attributes(mut cursor: Cursor) -> Result<FunctionTypeAttributes> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionTypeAttributes)?;
    let mut items = FunctionTypeAttributes::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_function_type_attribute(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_statements(mut cursor: Cursor) -> Result<Statements> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Statements)?;
    let mut items = Statements::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_statement(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_catch_clauses(mut cursor: Cursor) -> Result<CatchClauses> {
    expect_nonterminal_kind(&cursor, NonterminalKind::CatchClauses)?;
    let mut items = CatchClauses::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_catch_clause(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_string_literals(mut cursor: Cursor) -> Result<StringLiterals> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StringLiterals)?;
    let mut items = StringLiterals::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_string_literal(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_hex_string_literals(mut cursor: Cursor) -> Result<HexStringLiterals> {
    expect_nonterminal_kind(&cursor, NonterminalKind::HexStringLiterals)?;
    let mut items = HexStringLiterals::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_hex_string_literal(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_unicode_string_literals(mut cursor: Cursor) -> Result<UnicodeStringLiterals> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UnicodeStringLiterals)?;
    let mut items = UnicodeStringLiterals::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_unicode_string_literal(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_yul_statements(mut cursor: Cursor) -> Result<YulStatements> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulStatements)?;
    let mut items = YulStatements::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_yul_statement(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_yul_switch_cases(mut cursor: Cursor) -> Result<YulSwitchCases> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulSwitchCases)?;
    let mut items = YulSwitchCases::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_yul_switch_case(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

//
// Separated:
//

pub fn build_version_expression_sets(mut cursor: Cursor) -> Result<VersionExpressionSets> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionExpressionSets)?;
    let mut items = VersionExpressionSets::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_version_expression_set(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_simple_version_literal(mut cursor: Cursor) -> Result<SimpleVersionLiteral> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SimpleVersionLiteral)?;
    let mut items = SimpleVersionLiteral::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = fetch_terminal_node(&cursor)?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_import_deconstruction_symbols(
    mut cursor: Cursor,
) -> Result<ImportDeconstructionSymbols> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportDeconstructionSymbols)?;
    let mut items = ImportDeconstructionSymbols::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_import_deconstruction_symbol(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_using_deconstruction_symbols(
    mut cursor: Cursor,
) -> Result<UsingDeconstructionSymbols> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingDeconstructionSymbols)?;
    let mut items = UsingDeconstructionSymbols::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_using_deconstruction_symbol(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_inheritance_types(mut cursor: Cursor) -> Result<InheritanceTypes> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InheritanceTypes)?;
    let mut items = InheritanceTypes::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_inheritance_type(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_enum_members(mut cursor: Cursor) -> Result<EnumMembers> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EnumMembers)?;
    let mut items = EnumMembers::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = fetch_terminal_node(&cursor)?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_parameters(mut cursor: Cursor) -> Result<Parameters> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Parameters)?;
    let mut items = Parameters::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_parameter(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_override_paths(mut cursor: Cursor) -> Result<OverridePaths> {
    expect_nonterminal_kind(&cursor, NonterminalKind::OverridePaths)?;
    let mut items = OverridePaths::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_identifier_path(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_event_parameters(mut cursor: Cursor) -> Result<EventParameters> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EventParameters)?;
    let mut items = EventParameters::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_event_parameter(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_error_parameters(mut cursor: Cursor) -> Result<ErrorParameters> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ErrorParameters)?;
    let mut items = ErrorParameters::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_error_parameter(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_assembly_flags(mut cursor: Cursor) -> Result<AssemblyFlags> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AssemblyFlags)?;
    let mut items = AssemblyFlags::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_string_literal(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_tuple_deconstruction_elements(
    mut cursor: Cursor,
) -> Result<TupleDeconstructionElements> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleDeconstructionElements)?;
    let mut items = TupleDeconstructionElements::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_tuple_deconstruction_element(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_positional_arguments(mut cursor: Cursor) -> Result<PositionalArguments> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PositionalArguments)?;
    let mut items = PositionalArguments::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_expression(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_named_arguments(mut cursor: Cursor) -> Result<NamedArguments> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NamedArguments)?;
    let mut items = NamedArguments::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_named_argument(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_call_options(mut cursor: Cursor) -> Result<CallOptions> {
    expect_nonterminal_kind(&cursor, NonterminalKind::CallOptions)?;
    let mut items = CallOptions::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_named_argument(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_tuple_values(mut cursor: Cursor) -> Result<TupleValues> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleValues)?;
    let mut items = TupleValues::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_tuple_value(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_array_values(mut cursor: Cursor) -> Result<ArrayValues> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ArrayValues)?;
    let mut items = ArrayValues::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_expression(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_identifier_path(mut cursor: Cursor) -> Result<IdentifierPath> {
    expect_nonterminal_kind(&cursor, NonterminalKind::IdentifierPath)?;
    let mut items = IdentifierPath::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = fetch_terminal_node(&cursor)?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_yul_parameters(mut cursor: Cursor) -> Result<YulParameters> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulParameters)?;
    let mut items = YulParameters::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = fetch_terminal_node(&cursor)?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_yul_variable_names(mut cursor: Cursor) -> Result<YulVariableNames> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulVariableNames)?;
    let mut items = YulVariableNames::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = fetch_terminal_node(&cursor)?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_yul_arguments(mut cursor: Cursor) -> Result<YulArguments> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulArguments)?;
    let mut items = YulArguments::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_yul_expression(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_yul_paths(mut cursor: Cursor) -> Result<YulPaths> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulPaths)?;
    let mut items = YulPaths::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_yul_path(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_yul_path(mut cursor: Cursor) -> Result<YulPath> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulPath)?;
    let mut items = YulPath::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = fetch_terminal_node(&cursor)?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

//
// Common:
//

type Result<T> = std::result::Result<T, String>;

#[allow(dead_code)]
#[inline]
fn fetch_terminal_node(cursor: &Cursor) -> Result<Rc<TerminalNode>> {
    cursor.node().into_terminal().ok_or(format!(
        "Expected terminal node, got {:?} instead",
        cursor.node().kind()
    ))
}

#[allow(dead_code)]
#[inline]
fn expect_label(cursor: &Cursor, label: EdgeLabel) -> Result<()> {
    if cursor.label() == label {
        Ok(())
    } else {
        Err(format!(
            "Expected label {label:?}, but got {:?} instead",
            cursor.label()
        ))
    }
}

#[allow(dead_code)]
#[inline]
fn expect_nonterminal_kind(cursor: &Cursor, kind: NonterminalKind) -> Result<()> {
    if cursor.node().is_nonterminal_with_kind(kind) {
        Ok(())
    } else {
        Err(format!(
            "Expected {kind:?} node, but got {:?} instead",
            cursor.node().kind()
        ))
    }
}

#[allow(dead_code)]
#[inline]
fn skip_trivia(cursor: &mut Cursor) -> Result<()> {
    while cursor.node().is_trivia() {
        if !cursor.go_to_next_sibling() {
            return Err("Expected choice node to have at least non trivia child".into());
        }
    }
    Ok(())
}

#[allow(dead_code)]
#[inline]
fn consume_remaining_trivia(mut cursor: Cursor) -> Result<()> {
    while cursor.go_to_next_sibling() {
        if !cursor.node().is_trivia() {
            return Err("Unexpected non-trivia node".into());
        }
    }
    Ok(())
}

struct SequenceHelper {
    cursor: Cursor,
    finished: bool,
}

impl SequenceHelper {
    fn new(mut cursor: Cursor) -> Self {
        let mut finished = false;
        if cursor.go_to_first_child() {
            // skip initial trivia
            while cursor.node().is_trivia() {
                if !cursor.go_to_next_sibling() {
                    finished = true;
                    break;
                }
            }
        }
        Self { cursor, finished }
    }

    fn at_label(&self, label: EdgeLabel) -> bool {
        !self.finished && self.cursor.label() == label
    }

    fn accept_label(&mut self, label: EdgeLabel) -> Result<Cursor> {
        if self.finished {
            return Err(format!(
                "Expected more sibling nodes, looking for label {label:?}"
            ));
        }
        if self.cursor.label() == label {
            let cursor = self.cursor.clone();
            loop {
                if !self.cursor.go_to_next_sibling() {
                    self.finished = true;
                    break;
                }
                if !self.cursor.node().is_trivia() {
                    break;
                }
            }
            Ok(cursor)
        } else {
            Err(format!(
                "Expected node with label {label:?}, got {:?}",
                self.cursor.label()
            ))
        }
    }

    fn finalize(self) -> Result<()> {
        consume_remaining_trivia(self.cursor)
    }
}
