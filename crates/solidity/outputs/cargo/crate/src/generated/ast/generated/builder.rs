// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::{Cursor, EdgeLabel, NodeKind, NonterminalKind, TerminalNode};

//
// Sequences:
//

pub fn build_source_unit(cursor: Cursor) -> Result<SourceUnit> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SourceUnit)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let members = build_source_unit_members(helper.accept_label(EdgeLabel::Members)?)?;
    helper.finalize()?;

    Ok(Rc::new(SourceUnitStruct { cursor, members }))
}

pub fn build_pragma_directive(cursor: Cursor) -> Result<PragmaDirective> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PragmaDirective)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let pragma_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::PragmaKeyword)?)?;
    let pragma = build_pragma(helper.accept_label(EdgeLabel::Pragma)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(PragmaDirectiveStruct {
        cursor,
        pragma_keyword,
        pragma,
        semicolon,
    }))
}

pub fn build_abicoder_pragma(cursor: Cursor) -> Result<AbicoderPragma> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AbicoderPragma)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let abicoder_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::AbicoderKeyword)?)?;
    let version = fetch_terminal_node(&helper.accept_label(EdgeLabel::Version)?)?;
    helper.finalize()?;

    Ok(Rc::new(AbicoderPragmaStruct {
        cursor,
        abicoder_keyword,
        version,
    }))
}

pub fn build_experimental_pragma(cursor: Cursor) -> Result<ExperimentalPragma> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ExperimentalPragma)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let experimental_keyword =
        fetch_terminal_node(&helper.accept_label(EdgeLabel::ExperimentalKeyword)?)?;
    let feature = build_experimental_feature(helper.accept_label(EdgeLabel::Feature)?)?;
    helper.finalize()?;

    Ok(Rc::new(ExperimentalPragmaStruct {
        cursor,
        experimental_keyword,
        feature,
    }))
}

pub fn build_version_pragma(cursor: Cursor) -> Result<VersionPragma> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionPragma)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let solidity_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::SolidityKeyword)?)?;
    let sets = build_version_expression_sets(helper.accept_label(EdgeLabel::Sets)?)?;
    helper.finalize()?;

    Ok(Rc::new(VersionPragmaStruct {
        cursor,
        solidity_keyword,
        sets,
    }))
}

pub fn build_version_range(cursor: Cursor) -> Result<VersionRange> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionRange)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let start = build_version_literal(helper.accept_label(EdgeLabel::Start)?)?;
    let minus = fetch_terminal_node(&helper.accept_label(EdgeLabel::Minus)?)?;
    let end = build_version_literal(helper.accept_label(EdgeLabel::End)?)?;
    helper.finalize()?;

    Ok(Rc::new(VersionRangeStruct {
        cursor,
        start,
        minus,
        end,
    }))
}

pub fn build_version_term(cursor: Cursor) -> Result<VersionTerm> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionTerm)?;
    let mut helper = SequenceHelper::new(cursor.clone());
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
        cursor,
        operator,
        literal,
    }))
}

pub fn build_import_directive(cursor: Cursor) -> Result<ImportDirective> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportDirective)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let import_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ImportKeyword)?)?;
    let clause = build_import_clause(helper.accept_label(EdgeLabel::Clause)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ImportDirectiveStruct {
        cursor,
        import_keyword,
        clause,
        semicolon,
    }))
}

pub fn build_path_import(cursor: Cursor) -> Result<PathImport> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PathImport)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let path = build_string_literal(helper.accept_label(EdgeLabel::Path)?)?;
    let alias = if helper.at_label(EdgeLabel::Alias) {
        Some(build_import_alias(helper.accept_label(EdgeLabel::Alias)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(PathImportStruct {
        cursor,
        path,
        alias,
    }))
}

pub fn build_named_import(cursor: Cursor) -> Result<NamedImport> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NamedImport)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let asterisk = fetch_terminal_node(&helper.accept_label(EdgeLabel::Asterisk)?)?;
    let alias = build_import_alias(helper.accept_label(EdgeLabel::Alias)?)?;
    let from_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::FromKeyword)?)?;
    let path = build_string_literal(helper.accept_label(EdgeLabel::Path)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedImportStruct {
        cursor,
        asterisk,
        alias,
        from_keyword,
        path,
    }))
}

pub fn build_import_deconstruction(cursor: Cursor) -> Result<ImportDeconstruction> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportDeconstruction)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let symbols = build_import_deconstruction_symbols(helper.accept_label(EdgeLabel::Symbols)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    let from_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::FromKeyword)?)?;
    let path = build_string_literal(helper.accept_label(EdgeLabel::Path)?)?;
    helper.finalize()?;

    Ok(Rc::new(ImportDeconstructionStruct {
        cursor,
        open_brace,
        symbols,
        close_brace,
        from_keyword,
        path,
    }))
}

pub fn build_import_deconstruction_symbol(cursor: Cursor) -> Result<ImportDeconstructionSymbol> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportDeconstructionSymbol)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let alias = if helper.at_label(EdgeLabel::Alias) {
        Some(build_import_alias(helper.accept_label(EdgeLabel::Alias)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ImportDeconstructionSymbolStruct {
        cursor,
        name,
        alias,
    }))
}

pub fn build_import_alias(cursor: Cursor) -> Result<ImportAlias> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ImportAlias)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let as_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::AsKeyword)?)?;
    let identifier = fetch_terminal_node(&helper.accept_label(EdgeLabel::Identifier)?)?;
    helper.finalize()?;

    Ok(Rc::new(ImportAliasStruct {
        cursor,
        as_keyword,
        identifier,
    }))
}

pub fn build_using_directive(cursor: Cursor) -> Result<UsingDirective> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingDirective)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let using_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::UsingKeyword)?)?;
    let clause = build_using_clause(helper.accept_label(EdgeLabel::Clause)?)?;
    let for_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ForKeyword)?)?;
    let target = build_using_target(helper.accept_label(EdgeLabel::Target)?)?;
    let global_keyword = if helper.at_label(EdgeLabel::GlobalKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::GlobalKeyword)?,
        )?)
    } else {
        None
    };
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(UsingDirectiveStruct {
        cursor,
        using_keyword,
        clause,
        for_keyword,
        target,
        global_keyword,
        semicolon,
    }))
}

pub fn build_using_deconstruction(cursor: Cursor) -> Result<UsingDeconstruction> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingDeconstruction)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let symbols = build_using_deconstruction_symbols(helper.accept_label(EdgeLabel::Symbols)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(UsingDeconstructionStruct {
        cursor,
        open_brace,
        symbols,
        close_brace,
    }))
}

pub fn build_using_deconstruction_symbol(cursor: Cursor) -> Result<UsingDeconstructionSymbol> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingDeconstructionSymbol)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let name = build_identifier_path(helper.accept_label(EdgeLabel::Name)?)?;
    let alias = if helper.at_label(EdgeLabel::Alias) {
        Some(build_using_alias(helper.accept_label(EdgeLabel::Alias)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(UsingDeconstructionSymbolStruct {
        cursor,
        name,
        alias,
    }))
}

pub fn build_using_alias(cursor: Cursor) -> Result<UsingAlias> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingAlias)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let as_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::AsKeyword)?)?;
    let operator = build_using_operator(helper.accept_label(EdgeLabel::Operator)?)?;
    helper.finalize()?;

    Ok(Rc::new(UsingAliasStruct {
        cursor,
        as_keyword,
        operator,
    }))
}

pub fn build_contract_definition(cursor: Cursor) -> Result<ContractDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ContractDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let abstract_keyword = if helper.at_label(EdgeLabel::AbstractKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::AbstractKeyword)?,
        )?)
    } else {
        None
    };
    let contract_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ContractKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let inheritance = if helper.at_label(EdgeLabel::Inheritance) {
        Some(build_inheritance_specifier(
            helper.accept_label(EdgeLabel::Inheritance)?,
        )?)
    } else {
        None
    };
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_contract_members(helper.accept_label(EdgeLabel::Members)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(ContractDefinitionStruct {
        cursor,
        abstract_keyword,
        contract_keyword,
        name,
        inheritance,
        open_brace,
        members,
        close_brace,
    }))
}

pub fn build_inheritance_specifier(cursor: Cursor) -> Result<InheritanceSpecifier> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InheritanceSpecifier)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let is_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::IsKeyword)?)?;
    let types = build_inheritance_types(helper.accept_label(EdgeLabel::Types)?)?;
    helper.finalize()?;

    Ok(Rc::new(InheritanceSpecifierStruct {
        cursor,
        is_keyword,
        types,
    }))
}

pub fn build_inheritance_type(cursor: Cursor) -> Result<InheritanceType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InheritanceType)?;
    let mut helper = SequenceHelper::new(cursor.clone());
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
        cursor,
        type_name,
        arguments,
    }))
}

pub fn build_interface_definition(cursor: Cursor) -> Result<InterfaceDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InterfaceDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let interface_keyword =
        fetch_terminal_node(&helper.accept_label(EdgeLabel::InterfaceKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let inheritance = if helper.at_label(EdgeLabel::Inheritance) {
        Some(build_inheritance_specifier(
            helper.accept_label(EdgeLabel::Inheritance)?,
        )?)
    } else {
        None
    };
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_interface_members(helper.accept_label(EdgeLabel::Members)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(InterfaceDefinitionStruct {
        cursor,
        interface_keyword,
        name,
        inheritance,
        open_brace,
        members,
        close_brace,
    }))
}

pub fn build_library_definition(cursor: Cursor) -> Result<LibraryDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::LibraryDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let library_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::LibraryKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_library_members(helper.accept_label(EdgeLabel::Members)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(LibraryDefinitionStruct {
        cursor,
        library_keyword,
        name,
        open_brace,
        members,
        close_brace,
    }))
}

pub fn build_struct_definition(cursor: Cursor) -> Result<StructDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StructDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let struct_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::StructKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_struct_members(helper.accept_label(EdgeLabel::Members)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(StructDefinitionStruct {
        cursor,
        struct_keyword,
        name,
        open_brace,
        members,
        close_brace,
    }))
}

pub fn build_struct_member(cursor: Cursor) -> Result<StructMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StructMember)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(StructMemberStruct {
        cursor,
        type_name,
        name,
        semicolon,
    }))
}

pub fn build_enum_definition(cursor: Cursor) -> Result<EnumDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EnumDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let enum_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::EnumKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let members = build_enum_members(helper.accept_label(EdgeLabel::Members)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(EnumDefinitionStruct {
        cursor,
        enum_keyword,
        name,
        open_brace,
        members,
        close_brace,
    }))
}

pub fn build_constant_definition(cursor: Cursor) -> Result<ConstantDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ConstantDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let constant_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ConstantKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let equal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    let value = build_expression(helper.accept_label(EdgeLabel::Value)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ConstantDefinitionStruct {
        cursor,
        type_name,
        constant_keyword,
        name,
        equal,
        value,
        semicolon,
    }))
}

pub fn build_state_variable_definition(cursor: Cursor) -> Result<StateVariableDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StateVariableDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
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
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(StateVariableDefinitionStruct {
        cursor,
        type_name,
        attributes,
        name,
        value,
        semicolon,
    }))
}

pub fn build_state_variable_definition_value(
    cursor: Cursor,
) -> Result<StateVariableDefinitionValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StateVariableDefinitionValue)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let equal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    let value = build_expression(helper.accept_label(EdgeLabel::Value)?)?;
    helper.finalize()?;

    Ok(Rc::new(StateVariableDefinitionValueStruct {
        cursor,
        equal,
        value,
    }))
}

pub fn build_function_definition(cursor: Cursor) -> Result<FunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let function_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::FunctionKeyword)?)?;
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
        cursor,
        function_keyword,
        name,
        parameters,
        attributes,
        returns,
        body,
    }))
}

pub fn build_parameters_declaration(cursor: Cursor) -> Result<ParametersDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ParametersDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let parameters = build_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(ParametersDeclarationStruct {
        cursor,
        open_paren,
        parameters,
        close_paren,
    }))
}

pub fn build_parameter(cursor: Cursor) -> Result<Parameter> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Parameter)?;
    let mut helper = SequenceHelper::new(cursor.clone());
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
        cursor,
        type_name,
        storage_location,
        name,
    }))
}

pub fn build_override_specifier(cursor: Cursor) -> Result<OverrideSpecifier> {
    expect_nonterminal_kind(&cursor, NonterminalKind::OverrideSpecifier)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let override_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::OverrideKeyword)?)?;
    let overridden = if helper.at_label(EdgeLabel::Overridden) {
        Some(build_override_paths_declaration(
            helper.accept_label(EdgeLabel::Overridden)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(OverrideSpecifierStruct {
        cursor,
        override_keyword,
        overridden,
    }))
}

pub fn build_override_paths_declaration(cursor: Cursor) -> Result<OverridePathsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::OverridePathsDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let paths = build_override_paths(helper.accept_label(EdgeLabel::Paths)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(OverridePathsDeclarationStruct {
        cursor,
        open_paren,
        paths,
        close_paren,
    }))
}

pub fn build_returns_declaration(cursor: Cursor) -> Result<ReturnsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ReturnsDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let returns_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ReturnsKeyword)?)?;
    let variables = build_parameters_declaration(helper.accept_label(EdgeLabel::Variables)?)?;
    helper.finalize()?;

    Ok(Rc::new(ReturnsDeclarationStruct {
        cursor,
        returns_keyword,
        variables,
    }))
}

pub fn build_constructor_definition(cursor: Cursor) -> Result<ConstructorDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ConstructorDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let constructor_keyword =
        fetch_terminal_node(&helper.accept_label(EdgeLabel::ConstructorKeyword)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes = build_constructor_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(ConstructorDefinitionStruct {
        cursor,
        constructor_keyword,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_unnamed_function_definition(cursor: Cursor) -> Result<UnnamedFunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UnnamedFunctionDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let function_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::FunctionKeyword)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes =
        build_unnamed_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(UnnamedFunctionDefinitionStruct {
        cursor,
        function_keyword,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_fallback_function_definition(cursor: Cursor) -> Result<FallbackFunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FallbackFunctionDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let fallback_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::FallbackKeyword)?)?;
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
        cursor,
        fallback_keyword,
        parameters,
        attributes,
        returns,
        body,
    }))
}

pub fn build_receive_function_definition(cursor: Cursor) -> Result<ReceiveFunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ReceiveFunctionDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let receive_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ReceiveKeyword)?)?;
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    let attributes =
        build_receive_function_attributes(helper.accept_label(EdgeLabel::Attributes)?)?;
    let body = build_function_body(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(ReceiveFunctionDefinitionStruct {
        cursor,
        receive_keyword,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_modifier_definition(cursor: Cursor) -> Result<ModifierDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ModifierDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let modifier_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ModifierKeyword)?)?;
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
        cursor,
        modifier_keyword,
        name,
        parameters,
        attributes,
        body,
    }))
}

pub fn build_modifier_invocation(cursor: Cursor) -> Result<ModifierInvocation> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ModifierInvocation)?;
    let mut helper = SequenceHelper::new(cursor.clone());
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
        cursor,
        name,
        arguments,
    }))
}

pub fn build_event_definition(cursor: Cursor) -> Result<EventDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EventDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let event_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::EventKeyword)?)?;
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
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(EventDefinitionStruct {
        cursor,
        event_keyword,
        name,
        parameters,
        anonymous_keyword,
        semicolon,
    }))
}

pub fn build_event_parameters_declaration(cursor: Cursor) -> Result<EventParametersDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EventParametersDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let parameters = build_event_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(EventParametersDeclarationStruct {
        cursor,
        open_paren,
        parameters,
        close_paren,
    }))
}

pub fn build_event_parameter(cursor: Cursor) -> Result<EventParameter> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EventParameter)?;
    let mut helper = SequenceHelper::new(cursor.clone());
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
        cursor,
        type_name,
        indexed_keyword,
        name,
    }))
}

pub fn build_user_defined_value_type_definition(
    cursor: Cursor,
) -> Result<UserDefinedValueTypeDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UserDefinedValueTypeDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let type_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::TypeKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let is_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::IsKeyword)?)?;
    let value_type = build_elementary_type(helper.accept_label(EdgeLabel::ValueType)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(UserDefinedValueTypeDefinitionStruct {
        cursor,
        type_keyword,
        name,
        is_keyword,
        value_type,
        semicolon,
    }))
}

pub fn build_error_definition(cursor: Cursor) -> Result<ErrorDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ErrorDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let error_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ErrorKeyword)?)?;
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let members = build_error_parameters_declaration(helper.accept_label(EdgeLabel::Members)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ErrorDefinitionStruct {
        cursor,
        error_keyword,
        name,
        members,
        semicolon,
    }))
}

pub fn build_error_parameters_declaration(cursor: Cursor) -> Result<ErrorParametersDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ErrorParametersDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let parameters = build_error_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(ErrorParametersDeclarationStruct {
        cursor,
        open_paren,
        parameters,
        close_paren,
    }))
}

pub fn build_error_parameter(cursor: Cursor) -> Result<ErrorParameter> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ErrorParameter)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(ErrorParameterStruct {
        cursor,
        type_name,
        name,
    }))
}

pub fn build_array_type_name(cursor: Cursor) -> Result<ArrayTypeName> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ArrayTypeName)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operand = build_type_name(helper.accept_label(EdgeLabel::Operand)?)?;
    let open_bracket = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBracket)?)?;
    let index = if helper.at_label(EdgeLabel::Index) {
        Some(build_expression(helper.accept_label(EdgeLabel::Index)?)?)
    } else {
        None
    };
    let close_bracket = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBracket)?)?;
    helper.finalize()?;

    Ok(Rc::new(ArrayTypeNameStruct {
        cursor,
        operand,
        open_bracket,
        index,
        close_bracket,
    }))
}

pub fn build_function_type(cursor: Cursor) -> Result<FunctionType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionType)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let function_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::FunctionKeyword)?)?;
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
        cursor,
        function_keyword,
        parameters,
        attributes,
        returns,
    }))
}

pub fn build_mapping_type(cursor: Cursor) -> Result<MappingType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MappingType)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let mapping_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::MappingKeyword)?)?;
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let key_type = build_mapping_key(helper.accept_label(EdgeLabel::KeyType)?)?;
    let equal_greater_than =
        fetch_terminal_node(&helper.accept_label(EdgeLabel::EqualGreaterThan)?)?;
    let value_type = build_mapping_value(helper.accept_label(EdgeLabel::ValueType)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(MappingTypeStruct {
        cursor,
        mapping_keyword,
        open_paren,
        key_type,
        equal_greater_than,
        value_type,
        close_paren,
    }))
}

pub fn build_mapping_key(cursor: Cursor) -> Result<MappingKey> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MappingKey)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let key_type = build_mapping_key_type(helper.accept_label(EdgeLabel::KeyType)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(MappingKeyStruct {
        cursor,
        key_type,
        name,
    }))
}

pub fn build_mapping_value(cursor: Cursor) -> Result<MappingValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MappingValue)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(MappingValueStruct {
        cursor,
        type_name,
        name,
    }))
}

pub fn build_address_type(cursor: Cursor) -> Result<AddressType> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AddressType)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let address_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::AddressKeyword)?)?;
    let payable_keyword = if helper.at_label(EdgeLabel::PayableKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::PayableKeyword)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(AddressTypeStruct {
        cursor,
        address_keyword,
        payable_keyword,
    }))
}

pub fn build_block(cursor: Cursor) -> Result<Block> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Block)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let statements = build_statements(helper.accept_label(EdgeLabel::Statements)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(BlockStruct {
        cursor,
        open_brace,
        statements,
        close_brace,
    }))
}

pub fn build_unchecked_block(cursor: Cursor) -> Result<UncheckedBlock> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UncheckedBlock)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let unchecked_keyword =
        fetch_terminal_node(&helper.accept_label(EdgeLabel::UncheckedKeyword)?)?;
    let block = build_block(helper.accept_label(EdgeLabel::Block)?)?;
    helper.finalize()?;

    Ok(Rc::new(UncheckedBlockStruct {
        cursor,
        unchecked_keyword,
        block,
    }))
}

pub fn build_expression_statement(cursor: Cursor) -> Result<ExpressionStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ExpressionStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ExpressionStatementStruct {
        cursor,
        expression,
        semicolon,
    }))
}

pub fn build_assembly_statement(cursor: Cursor) -> Result<AssemblyStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AssemblyStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let assembly_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::AssemblyKeyword)?)?;
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
        cursor,
        assembly_keyword,
        label,
        flags,
        body,
    }))
}

pub fn build_assembly_flags_declaration(cursor: Cursor) -> Result<AssemblyFlagsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AssemblyFlagsDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let flags = build_assembly_flags(helper.accept_label(EdgeLabel::Flags)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(AssemblyFlagsDeclarationStruct {
        cursor,
        open_paren,
        flags,
        close_paren,
    }))
}

pub fn build_tuple_deconstruction_statement(
    cursor: Cursor,
) -> Result<TupleDeconstructionStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleDeconstructionStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let var_keyword = if helper.at_label(EdgeLabel::VarKeyword) {
        Some(fetch_terminal_node(
            &helper.accept_label(EdgeLabel::VarKeyword)?,
        )?)
    } else {
        None
    };
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let elements = build_tuple_deconstruction_elements(helper.accept_label(EdgeLabel::Elements)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    let equal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(TupleDeconstructionStatementStruct {
        cursor,
        var_keyword,
        open_paren,
        elements,
        close_paren,
        equal,
        expression,
        semicolon,
    }))
}

pub fn build_tuple_deconstruction_element(cursor: Cursor) -> Result<TupleDeconstructionElement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleDeconstructionElement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let member = if helper.at_label(EdgeLabel::Member) {
        Some(build_tuple_member(helper.accept_label(EdgeLabel::Member)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(TupleDeconstructionElementStruct { cursor, member }))
}

pub fn build_typed_tuple_member(cursor: Cursor) -> Result<TypedTupleMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TypedTupleMember)?;
    let mut helper = SequenceHelper::new(cursor.clone());
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
        cursor,
        type_name,
        storage_location,
        name,
    }))
}

pub fn build_untyped_tuple_member(cursor: Cursor) -> Result<UntypedTupleMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UntypedTupleMember)?;
    let mut helper = SequenceHelper::new(cursor.clone());
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
        cursor,
        storage_location,
        name,
    }))
}

pub fn build_variable_declaration_statement(
    cursor: Cursor,
) -> Result<VariableDeclarationStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VariableDeclarationStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
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
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(VariableDeclarationStatementStruct {
        cursor,
        variable_type,
        storage_location,
        name,
        value,
        semicolon,
    }))
}

pub fn build_variable_declaration_value(cursor: Cursor) -> Result<VariableDeclarationValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VariableDeclarationValue)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let equal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    let expression = build_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    helper.finalize()?;

    Ok(Rc::new(VariableDeclarationValueStruct {
        cursor,
        equal,
        expression,
    }))
}

pub fn build_if_statement(cursor: Cursor) -> Result<IfStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::IfStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let if_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::IfKeyword)?)?;
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let condition = build_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
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
        cursor,
        if_keyword,
        open_paren,
        condition,
        close_paren,
        body,
        else_branch,
    }))
}

pub fn build_else_branch(cursor: Cursor) -> Result<ElseBranch> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ElseBranch)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let else_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ElseKeyword)?)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(ElseBranchStruct {
        cursor,
        else_keyword,
        body,
    }))
}

pub fn build_for_statement(cursor: Cursor) -> Result<ForStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ForStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let for_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ForKeyword)?)?;
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let initialization =
        build_for_statement_initialization(helper.accept_label(EdgeLabel::Initialization)?)?;
    let condition = build_for_statement_condition(helper.accept_label(EdgeLabel::Condition)?)?;
    let iterator = if helper.at_label(EdgeLabel::Iterator) {
        Some(build_expression(helper.accept_label(EdgeLabel::Iterator)?)?)
    } else {
        None
    };
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(ForStatementStruct {
        cursor,
        for_keyword,
        open_paren,
        initialization,
        condition,
        iterator,
        close_paren,
        body,
    }))
}

pub fn build_while_statement(cursor: Cursor) -> Result<WhileStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::WhileStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let while_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::WhileKeyword)?)?;
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let condition = build_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(WhileStatementStruct {
        cursor,
        while_keyword,
        open_paren,
        condition,
        close_paren,
        body,
    }))
}

pub fn build_do_while_statement(cursor: Cursor) -> Result<DoWhileStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::DoWhileStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let do_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::DoKeyword)?)?;
    let body = build_statement(helper.accept_label(EdgeLabel::Body)?)?;
    let while_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::WhileKeyword)?)?;
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let condition = build_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(DoWhileStatementStruct {
        cursor,
        do_keyword,
        body,
        while_keyword,
        open_paren,
        condition,
        close_paren,
        semicolon,
    }))
}

pub fn build_continue_statement(cursor: Cursor) -> Result<ContinueStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ContinueStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let continue_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ContinueKeyword)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ContinueStatementStruct {
        cursor,
        continue_keyword,
        semicolon,
    }))
}

pub fn build_break_statement(cursor: Cursor) -> Result<BreakStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::BreakStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let break_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::BreakKeyword)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(BreakStatementStruct {
        cursor,
        break_keyword,
        semicolon,
    }))
}

pub fn build_return_statement(cursor: Cursor) -> Result<ReturnStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ReturnStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let return_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ReturnKeyword)?)?;
    let expression = if helper.at_label(EdgeLabel::Expression) {
        Some(build_expression(
            helper.accept_label(EdgeLabel::Expression)?,
        )?)
    } else {
        None
    };
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ReturnStatementStruct {
        cursor,
        return_keyword,
        expression,
        semicolon,
    }))
}

pub fn build_emit_statement(cursor: Cursor) -> Result<EmitStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EmitStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let emit_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::EmitKeyword)?)?;
    let event = build_identifier_path(helper.accept_label(EdgeLabel::Event)?)?;
    let arguments = build_arguments_declaration(helper.accept_label(EdgeLabel::Arguments)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(EmitStatementStruct {
        cursor,
        emit_keyword,
        event,
        arguments,
        semicolon,
    }))
}

pub fn build_try_statement(cursor: Cursor) -> Result<TryStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TryStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let try_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::TryKeyword)?)?;
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
        cursor,
        try_keyword,
        expression,
        returns,
        body,
        catch_clauses,
    }))
}

pub fn build_catch_clause(cursor: Cursor) -> Result<CatchClause> {
    expect_nonterminal_kind(&cursor, NonterminalKind::CatchClause)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let catch_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::CatchKeyword)?)?;
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
        cursor,
        catch_keyword,
        error,
        body,
    }))
}

pub fn build_catch_clause_error(cursor: Cursor) -> Result<CatchClauseError> {
    expect_nonterminal_kind(&cursor, NonterminalKind::CatchClauseError)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    let parameters = build_parameters_declaration(helper.accept_label(EdgeLabel::Parameters)?)?;
    helper.finalize()?;

    Ok(Rc::new(CatchClauseErrorStruct {
        cursor,
        name,
        parameters,
    }))
}

pub fn build_revert_statement(cursor: Cursor) -> Result<RevertStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::RevertStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let revert_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::RevertKeyword)?)?;
    let error = if helper.at_label(EdgeLabel::Error) {
        Some(build_identifier_path(
            helper.accept_label(EdgeLabel::Error)?,
        )?)
    } else {
        None
    };
    let arguments = build_arguments_declaration(helper.accept_label(EdgeLabel::Arguments)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(RevertStatementStruct {
        cursor,
        revert_keyword,
        error,
        arguments,
        semicolon,
    }))
}

pub fn build_throw_statement(cursor: Cursor) -> Result<ThrowStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ThrowStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let throw_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ThrowKeyword)?)?;
    let semicolon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(ThrowStatementStruct {
        cursor,
        throw_keyword,
        semicolon,
    }))
}

pub fn build_assignment_expression(cursor: Cursor) -> Result<AssignmentExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AssignmentExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(AssignmentExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_conditional_expression(cursor: Cursor) -> Result<ConditionalExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ConditionalExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let question_mark = fetch_terminal_node(&helper.accept_label(EdgeLabel::QuestionMark)?)?;
    let true_expression = build_expression(helper.accept_label(EdgeLabel::TrueExpression)?)?;
    let colon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    let false_expression = build_expression(helper.accept_label(EdgeLabel::FalseExpression)?)?;
    helper.finalize()?;

    Ok(Rc::new(ConditionalExpressionStruct {
        cursor,
        operand,
        question_mark,
        true_expression,
        colon,
        false_expression,
    }))
}

pub fn build_or_expression(cursor: Cursor) -> Result<OrExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::OrExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(OrExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_and_expression(cursor: Cursor) -> Result<AndExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AndExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(AndExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_equality_expression(cursor: Cursor) -> Result<EqualityExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::EqualityExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(EqualityExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_inequality_expression(cursor: Cursor) -> Result<InequalityExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::InequalityExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(InequalityExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_bitwise_or_expression(cursor: Cursor) -> Result<BitwiseOrExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::BitwiseOrExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(BitwiseOrExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_bitwise_xor_expression(cursor: Cursor) -> Result<BitwiseXorExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::BitwiseXorExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(BitwiseXorExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_bitwise_and_expression(cursor: Cursor) -> Result<BitwiseAndExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::BitwiseAndExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(BitwiseAndExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_shift_expression(cursor: Cursor) -> Result<ShiftExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ShiftExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(ShiftExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_additive_expression(cursor: Cursor) -> Result<AdditiveExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AdditiveExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(AdditiveExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_multiplicative_expression(cursor: Cursor) -> Result<MultiplicativeExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MultiplicativeExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(MultiplicativeExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_exponentiation_expression(cursor: Cursor) -> Result<ExponentiationExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ExponentiationExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(ExponentiationExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_postfix_expression(cursor: Cursor) -> Result<PostfixExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PostfixExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    helper.finalize()?;

    Ok(Rc::new(PostfixExpressionStruct {
        cursor,
        operand,
        operator,
    }))
}

pub fn build_prefix_expression(cursor: Cursor) -> Result<PrefixExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PrefixExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operator = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    helper.finalize()?;

    Ok(Rc::new(PrefixExpressionStruct {
        cursor,
        operator,
        operand,
    }))
}

pub fn build_function_call_expression(cursor: Cursor) -> Result<FunctionCallExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionCallExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let arguments = build_arguments_declaration(helper.accept_label(EdgeLabel::Arguments)?)?;
    helper.finalize()?;

    Ok(Rc::new(FunctionCallExpressionStruct {
        cursor,
        operand,
        arguments,
    }))
}

pub fn build_call_options_expression(cursor: Cursor) -> Result<CallOptionsExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::CallOptionsExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let options = build_call_options(helper.accept_label(EdgeLabel::Options)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(CallOptionsExpressionStruct {
        cursor,
        operand,
        open_brace,
        options,
        close_brace,
    }))
}

pub fn build_member_access_expression(cursor: Cursor) -> Result<MemberAccessExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MemberAccessExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let period = fetch_terminal_node(&helper.accept_label(EdgeLabel::Period)?)?;
    let member = fetch_terminal_node(&helper.accept_label(EdgeLabel::Member)?)?;
    helper.finalize()?;

    Ok(Rc::new(MemberAccessExpressionStruct {
        cursor,
        operand,
        period,
        member,
    }))
}

pub fn build_index_access_expression(cursor: Cursor) -> Result<IndexAccessExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::IndexAccessExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let open_bracket = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBracket)?)?;
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
    let close_bracket = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBracket)?)?;
    helper.finalize()?;

    Ok(Rc::new(IndexAccessExpressionStruct {
        cursor,
        operand,
        open_bracket,
        start,
        end,
        close_bracket,
    }))
}

pub fn build_index_access_end(cursor: Cursor) -> Result<IndexAccessEnd> {
    expect_nonterminal_kind(&cursor, NonterminalKind::IndexAccessEnd)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let colon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    let end = if helper.at_label(EdgeLabel::End) {
        Some(build_expression(helper.accept_label(EdgeLabel::End)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(IndexAccessEndStruct { cursor, colon, end }))
}

pub fn build_positional_arguments_declaration(
    cursor: Cursor,
) -> Result<PositionalArgumentsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::PositionalArgumentsDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let arguments = build_positional_arguments(helper.accept_label(EdgeLabel::Arguments)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(PositionalArgumentsDeclarationStruct {
        cursor,
        open_paren,
        arguments,
        close_paren,
    }))
}

pub fn build_named_arguments_declaration(cursor: Cursor) -> Result<NamedArgumentsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NamedArgumentsDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let arguments = if helper.at_label(EdgeLabel::Arguments) {
        Some(build_named_argument_group(
            helper.accept_label(EdgeLabel::Arguments)?,
        )?)
    } else {
        None
    };
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedArgumentsDeclarationStruct {
        cursor,
        open_paren,
        arguments,
        close_paren,
    }))
}

pub fn build_named_argument_group(cursor: Cursor) -> Result<NamedArgumentGroup> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NamedArgumentGroup)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let arguments = build_named_arguments(helper.accept_label(EdgeLabel::Arguments)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedArgumentGroupStruct {
        cursor,
        open_brace,
        arguments,
        close_brace,
    }))
}

pub fn build_named_argument(cursor: Cursor) -> Result<NamedArgument> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NamedArgument)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let name = fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?;
    let colon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    let value = build_expression(helper.accept_label(EdgeLabel::Value)?)?;
    helper.finalize()?;

    Ok(Rc::new(NamedArgumentStruct {
        cursor,
        name,
        colon,
        value,
    }))
}

pub fn build_type_expression(cursor: Cursor) -> Result<TypeExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TypeExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let type_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::TypeKeyword)?)?;
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(TypeExpressionStruct {
        cursor,
        type_keyword,
        open_paren,
        type_name,
        close_paren,
    }))
}

pub fn build_new_expression(cursor: Cursor) -> Result<NewExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NewExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let new_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::NewKeyword)?)?;
    let type_name = build_type_name(helper.accept_label(EdgeLabel::TypeName)?)?;
    helper.finalize()?;

    Ok(Rc::new(NewExpressionStruct {
        cursor,
        new_keyword,
        type_name,
    }))
}

pub fn build_tuple_expression(cursor: Cursor) -> Result<TupleExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let items = build_tuple_values(helper.accept_label(EdgeLabel::Items)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(TupleExpressionStruct {
        cursor,
        open_paren,
        items,
        close_paren,
    }))
}

pub fn build_tuple_value(cursor: Cursor) -> Result<TupleValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TupleValue)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let expression = if helper.at_label(EdgeLabel::Expression) {
        Some(build_expression(
            helper.accept_label(EdgeLabel::Expression)?,
        )?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(TupleValueStruct { cursor, expression }))
}

pub fn build_array_expression(cursor: Cursor) -> Result<ArrayExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::ArrayExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_bracket = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBracket)?)?;
    let items = build_array_values(helper.accept_label(EdgeLabel::Items)?)?;
    let close_bracket = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBracket)?)?;
    helper.finalize()?;

    Ok(Rc::new(ArrayExpressionStruct {
        cursor,
        open_bracket,
        items,
        close_bracket,
    }))
}

pub fn build_hex_number_expression(cursor: Cursor) -> Result<HexNumberExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::HexNumberExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let literal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Literal)?)?;
    let unit = if helper.at_label(EdgeLabel::Unit) {
        Some(build_number_unit(helper.accept_label(EdgeLabel::Unit)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(HexNumberExpressionStruct {
        cursor,
        literal,
        unit,
    }))
}

pub fn build_decimal_number_expression(cursor: Cursor) -> Result<DecimalNumberExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::DecimalNumberExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let literal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Literal)?)?;
    let unit = if helper.at_label(EdgeLabel::Unit) {
        Some(build_number_unit(helper.accept_label(EdgeLabel::Unit)?)?)
    } else {
        None
    };
    helper.finalize()?;

    Ok(Rc::new(DecimalNumberExpressionStruct {
        cursor,
        literal,
        unit,
    }))
}

pub fn build_yul_block(cursor: Cursor) -> Result<YulBlock> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulBlock)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBrace)?)?;
    let statements = build_yul_statements(helper.accept_label(EdgeLabel::Statements)?)?;
    let close_brace = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBrace)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulBlockStruct {
        cursor,
        open_brace,
        statements,
        close_brace,
    }))
}

pub fn build_yul_function_definition(cursor: Cursor) -> Result<YulFunctionDefinition> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulFunctionDefinition)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let function_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::FunctionKeyword)?)?;
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
        cursor,
        function_keyword,
        name,
        parameters,
        returns,
        body,
    }))
}

pub fn build_yul_parameters_declaration(cursor: Cursor) -> Result<YulParametersDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulParametersDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let parameters = build_yul_parameters(helper.accept_label(EdgeLabel::Parameters)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulParametersDeclarationStruct {
        cursor,
        open_paren,
        parameters,
        close_paren,
    }))
}

pub fn build_yul_returns_declaration(cursor: Cursor) -> Result<YulReturnsDeclaration> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulReturnsDeclaration)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let minus_greater_than =
        fetch_terminal_node(&helper.accept_label(EdgeLabel::MinusGreaterThan)?)?;
    let variables = build_yul_variable_names(helper.accept_label(EdgeLabel::Variables)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulReturnsDeclarationStruct {
        cursor,
        minus_greater_than,
        variables,
    }))
}

pub fn build_yul_variable_declaration_statement(
    cursor: Cursor,
) -> Result<YulVariableDeclarationStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulVariableDeclarationStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let let_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::LetKeyword)?)?;
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
        cursor,
        let_keyword,
        variables,
        value,
    }))
}

pub fn build_yul_variable_declaration_value(cursor: Cursor) -> Result<YulVariableDeclarationValue> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulVariableDeclarationValue)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let assignment = build_yul_assignment_operator(helper.accept_label(EdgeLabel::Assignment)?)?;
    let expression = build_yul_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulVariableDeclarationValueStruct {
        cursor,
        assignment,
        expression,
    }))
}

pub fn build_yul_variable_assignment_statement(
    cursor: Cursor,
) -> Result<YulVariableAssignmentStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulVariableAssignmentStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let variables = build_yul_paths(helper.accept_label(EdgeLabel::Variables)?)?;
    let assignment = build_yul_assignment_operator(helper.accept_label(EdgeLabel::Assignment)?)?;
    let expression = build_yul_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulVariableAssignmentStatementStruct {
        cursor,
        variables,
        assignment,
        expression,
    }))
}

pub fn build_yul_colon_and_equal(cursor: Cursor) -> Result<YulColonAndEqual> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulColonAndEqual)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let colon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    let equal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulColonAndEqualStruct {
        cursor,
        colon,
        equal,
    }))
}

pub fn build_yul_stack_assignment_statement(cursor: Cursor) -> Result<YulStackAssignmentStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulStackAssignmentStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let assignment =
        build_yul_stack_assignment_operator(helper.accept_label(EdgeLabel::Assignment)?)?;
    let variable = fetch_terminal_node(&helper.accept_label(EdgeLabel::Variable)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulStackAssignmentStatementStruct {
        cursor,
        assignment,
        variable,
    }))
}

pub fn build_yul_equal_and_colon(cursor: Cursor) -> Result<YulEqualAndColon> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulEqualAndColon)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let equal = fetch_terminal_node(&helper.accept_label(EdgeLabel::Equal)?)?;
    let colon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulEqualAndColonStruct {
        cursor,
        equal,
        colon,
    }))
}

pub fn build_yul_if_statement(cursor: Cursor) -> Result<YulIfStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulIfStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let if_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::IfKeyword)?)?;
    let condition = build_yul_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulIfStatementStruct {
        cursor,
        if_keyword,
        condition,
        body,
    }))
}

pub fn build_yul_for_statement(cursor: Cursor) -> Result<YulForStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulForStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let for_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ForKeyword)?)?;
    let initialization = build_yul_block(helper.accept_label(EdgeLabel::Initialization)?)?;
    let condition = build_yul_expression(helper.accept_label(EdgeLabel::Condition)?)?;
    let iterator = build_yul_block(helper.accept_label(EdgeLabel::Iterator)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulForStatementStruct {
        cursor,
        for_keyword,
        initialization,
        condition,
        iterator,
        body,
    }))
}

pub fn build_yul_switch_statement(cursor: Cursor) -> Result<YulSwitchStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulSwitchStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let switch_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::SwitchKeyword)?)?;
    let expression = build_yul_expression(helper.accept_label(EdgeLabel::Expression)?)?;
    let cases = build_yul_switch_cases(helper.accept_label(EdgeLabel::Cases)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulSwitchStatementStruct {
        cursor,
        switch_keyword,
        expression,
        cases,
    }))
}

pub fn build_yul_default_case(cursor: Cursor) -> Result<YulDefaultCase> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulDefaultCase)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let default_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::DefaultKeyword)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulDefaultCaseStruct {
        cursor,
        default_keyword,
        body,
    }))
}

pub fn build_yul_value_case(cursor: Cursor) -> Result<YulValueCase> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulValueCase)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let case_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::CaseKeyword)?)?;
    let value = build_yul_literal(helper.accept_label(EdgeLabel::Value)?)?;
    let body = build_yul_block(helper.accept_label(EdgeLabel::Body)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulValueCaseStruct {
        cursor,
        case_keyword,
        value,
        body,
    }))
}

pub fn build_yul_leave_statement(cursor: Cursor) -> Result<YulLeaveStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulLeaveStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let leave_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::LeaveKeyword)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulLeaveStatementStruct {
        cursor,
        leave_keyword,
    }))
}

pub fn build_yul_break_statement(cursor: Cursor) -> Result<YulBreakStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulBreakStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let break_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::BreakKeyword)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulBreakStatementStruct {
        cursor,
        break_keyword,
    }))
}

pub fn build_yul_continue_statement(cursor: Cursor) -> Result<YulContinueStatement> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulContinueStatement)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let continue_keyword = fetch_terminal_node(&helper.accept_label(EdgeLabel::ContinueKeyword)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulContinueStatementStruct {
        cursor,
        continue_keyword,
    }))
}

pub fn build_yul_label(cursor: Cursor) -> Result<YulLabel> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulLabel)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let label = fetch_terminal_node(&helper.accept_label(EdgeLabel::Label)?)?;
    let colon = fetch_terminal_node(&helper.accept_label(EdgeLabel::Colon)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulLabelStruct {
        cursor,
        label,
        colon,
    }))
}

pub fn build_yul_function_call_expression(cursor: Cursor) -> Result<YulFunctionCallExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::YulFunctionCallExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operand = build_yul_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    let open_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenParen)?)?;
    let arguments = build_yul_arguments(helper.accept_label(EdgeLabel::Arguments)?)?;
    let close_paren = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseParen)?)?;
    helper.finalize()?;

    Ok(Rc::new(YulFunctionCallExpressionStruct {
        cursor,
        operand,
        open_paren,
        arguments,
        close_paren,
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
        NodeKind::Terminal(_) => ExperimentalFeature::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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

pub fn build_version_operator(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::VersionOperator)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
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
        NodeKind::Terminal(_) => VersionLiteral::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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

pub fn build_using_operator(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UsingOperator)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
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
        NodeKind::Terminal(_) => UsingTarget::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => {
            StateVariableAttribute::TerminalNode(fetch_terminal_node(&cursor)?)
        }
        NodeKind::Nonterminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_function_name(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionName)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
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
        NodeKind::Terminal(_) => FunctionAttribute::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => FunctionBody::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => ConstructorAttribute::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => {
            UnnamedFunctionAttribute::TerminalNode(fetch_terminal_node(&cursor)?)
        }
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => {
            FallbackFunctionAttribute::TerminalNode(fetch_terminal_node(&cursor)?)
        }
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => {
            ReceiveFunctionAttribute::TerminalNode(fetch_terminal_node(&cursor)?)
        }
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => ModifierAttribute::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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

pub fn build_function_type_attribute(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::FunctionTypeAttribute)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
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
        NodeKind::Terminal(_) => ElementaryType::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => {
            VariableDeclarationType::TerminalNode(fetch_terminal_node(&cursor)?)
        }
        NodeKind::Nonterminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_storage_location(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StorageLocation)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
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
        NodeKind::Terminal(_) => {
            ForStatementInitialization::TerminalNode(fetch_terminal_node(&cursor)?)
        }
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => ForStatementCondition::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => Expression::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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

pub fn build_number_unit(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NumberUnit)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
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

pub fn build_string_literal(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::StringLiteral)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_hex_string_literal(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::HexStringLiteral)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_unicode_string_literal(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::UnicodeStringLiteral)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
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
        NodeKind::Terminal(_) => YulAssignmentOperator::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => {
            YulStackAssignmentOperator::TerminalNode(fetch_terminal_node(&cursor)?)
        }
        NodeKind::Nonterminal(_) => {
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
        NodeKind::Terminal(_) => YulLiteral::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
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
