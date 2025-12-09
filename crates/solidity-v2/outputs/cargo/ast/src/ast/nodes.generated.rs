// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::rc::Rc;

use crate::ast::lexemes::LexemeKind;

// TODO:
// - (perf) don't use terminals that are not needed

// Special cases

#[derive(Debug)]
pub struct ProtoTuple {
    // Do we care about range in source code?
    pub _open_paren: OpenParen,
    pub _elements: Vec<ProtoTupleElement>,
    pub _close_paren: CloseParen,
}

pub fn new_proto_tuple(
    _open_paren: OpenParen,
    _elements: Vec<ProtoTupleElement>,
    _close_paren: CloseParen,
) -> ProtoTuple {
    ProtoTuple {
        _open_paren,
        _elements,
        _close_paren,
    }
}

pub fn new_tuple_expression_from_proto_tuple(proto_tuple: ProtoTuple) -> TupleExpression {
    let elements: Vec<TupleValue> = proto_tuple
        ._elements
        .into_iter()
        .map(|element| match element {
            ProtoTupleElement::Expression(name) => new_tuple_value(Some(name)),
            ProtoTupleElement::Declaration(_) => panic!("Tuples can't have declarations"),
            ProtoTupleElement::StorageLocation(_, _) => {
                panic!("Tuples can't have storage locations")
            }
            ProtoTupleElement::Empty => new_tuple_value(None),
        })
        .collect();
    new_tuple_expression(
        proto_tuple._open_paren,
        new_tuple_values(elements),
        proto_tuple._close_paren,
    )
}

pub fn new_tuple_deconstruction_statement_from_proto_tuple(
    proto_tuple: ProtoTuple,
    equal: Equal,
    expression: Expression,
    semicolon: Semicolon,
) -> TupleDeconstructionStatement {
    let elements: Vec<TupleDeconstructionElement> = proto_tuple
        ._elements
        .into_iter()
        .map(|element| match element {
            ProtoTupleElement::Expression(name) => match name {
                Expression::Identifier(name) => new_tuple_deconstruction_element(Some(
                    new_tuple_member_untyped_tuple_member(new_untyped_tuple_member(None, name)),
                )),
                _ => panic!("Tuples deconstruction can only be used with identifiers"),
            },
            ProtoTupleElement::Declaration(decl) => {
                let tuple_member = match &decl.variable_type {
                    VariableDeclarationType::TypeName(type_name) => {
                        new_tuple_member_typed_tuple_member(new_typed_tuple_member(
                            type_name.clone(),
                            decl.storage_location.clone(),
                            decl.name.clone(),
                        ))
                    }
                    VariableDeclarationType::VarKeyword(_) => {
                        new_tuple_member_untyped_tuple_member(new_untyped_tuple_member(
                            decl.storage_location.clone(),
                            decl.name.clone(),
                        ))
                    }
                };
                new_tuple_deconstruction_element(Some(tuple_member))
            }
            ProtoTupleElement::StorageLocation(storage_location, name) => {
                new_tuple_deconstruction_element(Some(new_tuple_member_untyped_tuple_member(
                    new_untyped_tuple_member(Some(storage_location), name),
                )))
            }
            ProtoTupleElement::Empty => new_tuple_deconstruction_element(None),
        })
        .collect();
    new_tuple_deconstruction_statement(
        None,
        proto_tuple._open_paren,
        new_tuple_deconstruction_elements(elements),
        proto_tuple._close_paren,
        equal,
        expression,
        semicolon,
    )
}

#[derive(Debug)]
pub enum ProtoTupleElement {
    Expression(Expression),
    Declaration(VariableDeclaration),
    StorageLocation(StorageLocation, Identifier),
    Empty,
}

pub fn new_proto_tuple_element_expression(name: Expression) -> ProtoTupleElement {
    ProtoTupleElement::Expression(name)
}

pub fn new_proto_tuple_element_declaration(decl: VariableDeclaration) -> ProtoTupleElement {
    ProtoTupleElement::Declaration(decl)
}

pub fn new_proto_tuple_element_storage_location(
    storage_location: StorageLocation,
    name: Identifier,
) -> ProtoTupleElement {
    ProtoTupleElement::StorageLocation(storage_location, name)
}

pub fn new_proto_tuple_element_empty() -> ProtoTupleElement {
    ProtoTupleElement::Empty
}

pub fn new_expression_identifier_path(identifier_path: IdentifierPath) -> Expression {
    // TODO: This is just a placeholder, we need to fix this
    let base: Expression = new_expression_identifier(identifier_path.elements[0].clone());
    identifier_path.elements[1..].iter().fold(base, |acc, id| {
        new_expression_member_access_expression(new_member_access_expression(
            acc,
            new_empty_terminal(),
            id.clone(),
        ))
    })
}

/// We use this function to share attributes between a state variable that has a function type.
/// We find and split the attributes from the function type as needed
/// TODO(v2) fail gracefully if a wrong attribute is found
pub fn extract_extra_attributes(
    mut fun_type: FunctionType,
) -> (FunctionType, Vec<StateVariableAttribute>) {
    // Move all matching attributes to extra_attributes if duplicate_found, else only the first occurrence
    let mut seen_constant = false;
    let mut seen_internal = false;
    let mut seen_private = false;
    let mut seen_public = false;
    let mut duplicate_found = false;

    let mut extra_attributes: Vec<StateVariableAttribute> = Vec::new();
    fn add_to_extra(
        attr: &FunctionTypeAttribute,
        extra_attributes: &mut Vec<StateVariableAttribute>,
    ) {
        match attr {
      FunctionTypeAttribute::ConstantKeyword(terminal) => {
        extra_attributes.push(StateVariableAttribute::ConstantKeyword(terminal.clone()));
      }
      FunctionTypeAttribute::InternalKeyword(terminal) => {
        extra_attributes.push(StateVariableAttribute::InternalKeyword(terminal.clone()));
      }
      FunctionTypeAttribute::PrivateKeyword(terminal) => {
        extra_attributes.push(StateVariableAttribute::PrivateKeyword(terminal.clone()));
      }
      FunctionTypeAttribute::PublicKeyword(terminal) => {
        extra_attributes.push(StateVariableAttribute::PublicKeyword(terminal.clone()));
      }
      _ => panic!("This is wrong, I don't really know what to do for now, but it should fail gracefully (like a parser error)")
    }
    }

    Rc::get_mut(&mut fun_type)
        .unwrap()
        .attributes
        .elements
        .retain(|attr| {
            if duplicate_found {
                add_to_extra(attr, &mut extra_attributes);
                return false;
            }
            let &mut seen = match attr {
                FunctionTypeAttribute::ConstantKeyword(_) => &mut seen_constant,
                FunctionTypeAttribute::InternalKeyword(_) => &mut seen_internal,
                FunctionTypeAttribute::PrivateKeyword(_) => &mut seen_private,
                FunctionTypeAttribute::PublicKeyword(_) => &mut seen_public,
                _ => return true,
            };

            if seen {
                duplicate_found = true;
                add_to_extra(attr, &mut extra_attributes);
                false
            } else {
                seen_constant = true;
                true
            }
        });

    (fun_type, extra_attributes)
}

//
// Sequences:
//

pub type SourceUnit = Rc<SourceUnitStruct>;

#[derive(Debug)]
pub struct SourceUnitStruct {
    // Do we care about range in source code?
    pub members: SourceUnitMembers,
}

pub fn new_source_unit(members: SourceUnitMembers) -> SourceUnit {
    Rc::new(SourceUnitStruct { members })
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

#[derive(Debug)]
pub struct PragmaDirectiveStruct {
    // Do we care about range in source code?
    pub pragma_keyword: PragmaKeyword,
    pub pragma: Pragma,
    pub semicolon: Semicolon,
}

pub fn new_pragma_directive(
    pragma_keyword: PragmaKeyword,
    pragma: Pragma,
    semicolon: Semicolon,
) -> PragmaDirective {
    Rc::new(PragmaDirectiveStruct {
        pragma_keyword,
        pragma,
        semicolon,
    })
}

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

#[derive(Debug)]
pub struct AbicoderPragmaStruct {
    // Do we care about range in source code?
    pub abicoder_keyword: AbicoderKeyword,
    pub version: AbicoderVersion,
}

pub fn new_abicoder_pragma(
    abicoder_keyword: AbicoderKeyword,
    version: AbicoderVersion,
) -> AbicoderPragma {
    Rc::new(AbicoderPragmaStruct {
        abicoder_keyword,
        version,
    })
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

#[derive(Debug)]
pub struct ExperimentalPragmaStruct {
    // Do we care about range in source code?
    pub experimental_keyword: ExperimentalKeyword,
    pub feature: ExperimentalFeature,
}

pub fn new_experimental_pragma(
    experimental_keyword: ExperimentalKeyword,
    feature: ExperimentalFeature,
) -> ExperimentalPragma {
    Rc::new(ExperimentalPragmaStruct {
        experimental_keyword,
        feature,
    })
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

#[derive(Debug)]
pub struct VersionPragmaStruct {
    // Do we care about range in source code?
    pub solidity_keyword: SolidityKeyword,
    pub sets: VersionExpressionSets,
}

pub fn new_version_pragma(
    solidity_keyword: SolidityKeyword,
    sets: VersionExpressionSets,
) -> VersionPragma {
    Rc::new(VersionPragmaStruct {
        solidity_keyword,
        sets,
    })
}

pub type VersionRange = Rc<VersionRangeStruct>;

#[derive(Debug)]
pub struct VersionRangeStruct {
    // Do we care about range in source code?
    pub start: VersionLiteral,
    pub minus: Minus,
    pub end: VersionLiteral,
}

pub fn new_version_range(start: VersionLiteral, minus: Minus, end: VersionLiteral) -> VersionRange {
    Rc::new(VersionRangeStruct { start, minus, end })
}

pub type VersionTerm = Rc<VersionTermStruct>;

#[derive(Debug)]
pub struct VersionTermStruct {
    // Do we care about range in source code?
    pub operator: Option<VersionOperator>,
    pub literal: VersionLiteral,
}

pub fn new_version_term(operator: Option<VersionOperator>, literal: VersionLiteral) -> VersionTerm {
    Rc::new(VersionTermStruct { operator, literal })
}

pub type ImportDirective = Rc<ImportDirectiveStruct>;

#[derive(Debug)]
pub struct ImportDirectiveStruct {
    // Do we care about range in source code?
    pub import_keyword: ImportKeyword,
    pub clause: ImportClause,
    pub semicolon: Semicolon,
}

pub fn new_import_directive(
    import_keyword: ImportKeyword,
    clause: ImportClause,
    semicolon: Semicolon,
) -> ImportDirective {
    Rc::new(ImportDirectiveStruct {
        import_keyword,
        clause,
        semicolon,
    })
}

pub type PathImport = Rc<PathImportStruct>;

#[derive(Debug)]
pub struct PathImportStruct {
    // Do we care about range in source code?
    pub path: StringLiteral,
    pub alias: Option<ImportAlias>,
}

pub fn new_path_import(path: StringLiteral, alias: Option<ImportAlias>) -> PathImport {
    Rc::new(PathImportStruct { path, alias })
}

pub type NamedImport = Rc<NamedImportStruct>;

#[derive(Debug)]
pub struct NamedImportStruct {
    // Do we care about range in source code?
    pub asterisk: Asterisk,
    pub alias: ImportAlias,
    pub from_keyword: FromKeyword,
    pub path: StringLiteral,
}

pub fn new_named_import(
    asterisk: Asterisk,
    alias: ImportAlias,
    from_keyword: FromKeyword,
    path: StringLiteral,
) -> NamedImport {
    Rc::new(NamedImportStruct {
        asterisk,
        alias,
        from_keyword,
        path,
    })
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionStruct {
    // Do we care about range in source code?
    pub open_brace: OpenBrace,
    pub symbols: ImportDeconstructionSymbols,
    pub close_brace: CloseBrace,
    pub from_keyword: FromKeyword,
    pub path: StringLiteral,
}

pub fn new_import_deconstruction(
    open_brace: OpenBrace,
    symbols: ImportDeconstructionSymbols,
    close_brace: CloseBrace,
    from_keyword: FromKeyword,
    path: StringLiteral,
) -> ImportDeconstruction {
    Rc::new(ImportDeconstructionStruct {
        open_brace,
        symbols,
        close_brace,
        from_keyword,
        path,
    })
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionSymbolStruct {
    // Do we care about range in source code?
    pub name: Identifier,
    pub alias: Option<ImportAlias>,
}

pub fn new_import_deconstruction_symbol(
    name: Identifier,
    alias: Option<ImportAlias>,
) -> ImportDeconstructionSymbol {
    Rc::new(ImportDeconstructionSymbolStruct { name, alias })
}

pub type ImportAlias = Rc<ImportAliasStruct>;

#[derive(Debug)]
pub struct ImportAliasStruct {
    // Do we care about range in source code?
    pub as_keyword: AsKeyword,
    pub identifier: Identifier,
}

pub fn new_import_alias(as_keyword: AsKeyword, identifier: Identifier) -> ImportAlias {
    Rc::new(ImportAliasStruct {
        as_keyword,
        identifier,
    })
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

#[derive(Debug)]
pub struct UsingDirectiveStruct {
    // Do we care about range in source code?
    pub using_keyword: UsingKeyword,
    pub clause: UsingClause,
    pub for_keyword: ForKeyword,
    pub target: UsingTarget,
    pub global_keyword: Option<GlobalKeyword>,
    pub semicolon: Semicolon,
}

pub fn new_using_directive(
    using_keyword: UsingKeyword,
    clause: UsingClause,
    for_keyword: ForKeyword,
    target: UsingTarget,
    global_keyword: Option<GlobalKeyword>,
    semicolon: Semicolon,
) -> UsingDirective {
    Rc::new(UsingDirectiveStruct {
        using_keyword,
        clause,
        for_keyword,
        target,
        global_keyword,
        semicolon,
    })
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionStruct {
    // Do we care about range in source code?
    pub open_brace: OpenBrace,
    pub symbols: UsingDeconstructionSymbols,
    pub close_brace: CloseBrace,
}

pub fn new_using_deconstruction(
    open_brace: OpenBrace,
    symbols: UsingDeconstructionSymbols,
    close_brace: CloseBrace,
) -> UsingDeconstruction {
    Rc::new(UsingDeconstructionStruct {
        open_brace,
        symbols,
        close_brace,
    })
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionSymbolStruct {
    // Do we care about range in source code?
    pub name: IdentifierPath,
    pub alias: Option<UsingAlias>,
}

pub fn new_using_deconstruction_symbol(
    name: IdentifierPath,
    alias: Option<UsingAlias>,
) -> UsingDeconstructionSymbol {
    Rc::new(UsingDeconstructionSymbolStruct { name, alias })
}

pub type UsingAlias = Rc<UsingAliasStruct>;

#[derive(Debug)]
pub struct UsingAliasStruct {
    // Do we care about range in source code?
    pub as_keyword: AsKeyword,
    pub operator: UsingOperator,
}

pub fn new_using_alias(as_keyword: AsKeyword, operator: UsingOperator) -> UsingAlias {
    Rc::new(UsingAliasStruct {
        as_keyword,
        operator,
    })
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

#[derive(Debug)]
pub struct ContractDefinitionStruct {
    // Do we care about range in source code?
    pub abstract_keyword: Option<AbstractKeyword>,
    pub contract_keyword: ContractKeyword,
    pub name: Identifier,
    pub specifiers: ContractSpecifiers,
    pub open_brace: OpenBrace,
    pub members: ContractMembers,
    pub close_brace: CloseBrace,
}

pub fn new_contract_definition(
    abstract_keyword: Option<AbstractKeyword>,
    contract_keyword: ContractKeyword,
    name: Identifier,
    specifiers: ContractSpecifiers,
    open_brace: OpenBrace,
    members: ContractMembers,
    close_brace: CloseBrace,
) -> ContractDefinition {
    Rc::new(ContractDefinitionStruct {
        abstract_keyword,
        contract_keyword,
        name,
        specifiers,
        open_brace,
        members,
        close_brace,
    })
}

pub type InheritanceSpecifier = Rc<InheritanceSpecifierStruct>;

#[derive(Debug)]
pub struct InheritanceSpecifierStruct {
    // Do we care about range in source code?
    pub is_keyword: IsKeyword,
    pub types: InheritanceTypes,
}

pub fn new_inheritance_specifier(
    is_keyword: IsKeyword,
    types: InheritanceTypes,
) -> InheritanceSpecifier {
    Rc::new(InheritanceSpecifierStruct { is_keyword, types })
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

#[derive(Debug)]
pub struct InheritanceTypeStruct {
    // Do we care about range in source code?
    pub type_name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

pub fn new_inheritance_type(
    type_name: IdentifierPath,
    arguments: Option<ArgumentsDeclaration>,
) -> InheritanceType {
    Rc::new(InheritanceTypeStruct {
        type_name,
        arguments,
    })
}

pub type StorageLayoutSpecifier = Rc<StorageLayoutSpecifierStruct>;

#[derive(Debug)]
pub struct StorageLayoutSpecifierStruct {
    // Do we care about range in source code?
    pub layout_keyword: LayoutKeyword,
    pub at_keyword: AtKeyword,
    pub expression: Expression,
}

pub fn new_storage_layout_specifier(
    layout_keyword: LayoutKeyword,
    at_keyword: AtKeyword,
    expression: Expression,
) -> StorageLayoutSpecifier {
    Rc::new(StorageLayoutSpecifierStruct {
        layout_keyword,
        at_keyword,
        expression,
    })
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

#[derive(Debug)]
pub struct InterfaceDefinitionStruct {
    // Do we care about range in source code?
    pub interface_keyword: InterfaceKeyword,
    pub name: Identifier,
    pub inheritance: Option<InheritanceSpecifier>,
    pub open_brace: OpenBrace,
    pub members: InterfaceMembers,
    pub close_brace: CloseBrace,
}

pub fn new_interface_definition(
    interface_keyword: InterfaceKeyword,
    name: Identifier,
    inheritance: Option<InheritanceSpecifier>,
    open_brace: OpenBrace,
    members: InterfaceMembers,
    close_brace: CloseBrace,
) -> InterfaceDefinition {
    Rc::new(InterfaceDefinitionStruct {
        interface_keyword,
        name,
        inheritance,
        open_brace,
        members,
        close_brace,
    })
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

#[derive(Debug)]
pub struct LibraryDefinitionStruct {
    // Do we care about range in source code?
    pub library_keyword: LibraryKeyword,
    pub name: Identifier,
    pub open_brace: OpenBrace,
    pub members: LibraryMembers,
    pub close_brace: CloseBrace,
}

pub fn new_library_definition(
    library_keyword: LibraryKeyword,
    name: Identifier,
    open_brace: OpenBrace,
    members: LibraryMembers,
    close_brace: CloseBrace,
) -> LibraryDefinition {
    Rc::new(LibraryDefinitionStruct {
        library_keyword,
        name,
        open_brace,
        members,
        close_brace,
    })
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

#[derive(Debug)]
pub struct StructDefinitionStruct {
    // Do we care about range in source code?
    pub struct_keyword: StructKeyword,
    pub name: Identifier,
    pub open_brace: OpenBrace,
    pub members: StructMembers,
    pub close_brace: CloseBrace,
}

pub fn new_struct_definition(
    struct_keyword: StructKeyword,
    name: Identifier,
    open_brace: OpenBrace,
    members: StructMembers,
    close_brace: CloseBrace,
) -> StructDefinition {
    Rc::new(StructDefinitionStruct {
        struct_keyword,
        name,
        open_brace,
        members,
        close_brace,
    })
}

pub type StructMember = Rc<StructMemberStruct>;

#[derive(Debug)]
pub struct StructMemberStruct {
    // Do we care about range in source code?
    pub type_name: TypeName,
    pub name: Identifier,
    pub semicolon: Semicolon,
}

pub fn new_struct_member(
    type_name: TypeName,
    name: Identifier,
    semicolon: Semicolon,
) -> StructMember {
    Rc::new(StructMemberStruct {
        type_name,
        name,
        semicolon,
    })
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

#[derive(Debug)]
pub struct EnumDefinitionStruct {
    // Do we care about range in source code?
    pub enum_keyword: EnumKeyword,
    pub name: Identifier,
    pub open_brace: OpenBrace,
    pub members: EnumMembers,
    pub close_brace: CloseBrace,
}

pub fn new_enum_definition(
    enum_keyword: EnumKeyword,
    name: Identifier,
    open_brace: OpenBrace,
    members: EnumMembers,
    close_brace: CloseBrace,
) -> EnumDefinition {
    Rc::new(EnumDefinitionStruct {
        enum_keyword,
        name,
        open_brace,
        members,
        close_brace,
    })
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

#[derive(Debug)]
pub struct ConstantDefinitionStruct {
    // Do we care about range in source code?
    pub type_name: TypeName,
    pub constant_keyword: ConstantKeyword,
    pub name: Identifier,
    pub equal: Equal,
    pub value: Expression,
    pub semicolon: Semicolon,
}

pub fn new_constant_definition(
    type_name: TypeName,
    constant_keyword: ConstantKeyword,
    name: Identifier,
    equal: Equal,
    value: Expression,
    semicolon: Semicolon,
) -> ConstantDefinition {
    Rc::new(ConstantDefinitionStruct {
        type_name,
        constant_keyword,
        name,
        equal,
        value,
        semicolon,
    })
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

#[derive(Debug)]
pub struct StateVariableDefinitionStruct {
    // Do we care about range in source code?
    pub type_name: TypeName,
    pub attributes: StateVariableAttributes,
    pub name: Identifier,
    pub value: Option<StateVariableDefinitionValue>,
    pub semicolon: Semicolon,
}

pub fn new_state_variable_definition(
    type_name: TypeName,
    attributes: StateVariableAttributes,
    name: Identifier,
    value: Option<StateVariableDefinitionValue>,
    semicolon: Semicolon,
) -> StateVariableDefinition {
    Rc::new(StateVariableDefinitionStruct {
        type_name,
        attributes,
        name,
        value,
        semicolon,
    })
}

pub type StateVariableDefinitionValue = Rc<StateVariableDefinitionValueStruct>;

#[derive(Debug)]
pub struct StateVariableDefinitionValueStruct {
    // Do we care about range in source code?
    pub equal: Equal,
    pub value: Expression,
}

pub fn new_state_variable_definition_value(
    equal: Equal,
    value: Expression,
) -> StateVariableDefinitionValue {
    Rc::new(StateVariableDefinitionValueStruct { equal, value })
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FunctionDefinitionStruct {
    // Do we care about range in source code?
    pub function_keyword: FunctionKeyword,
    pub name: FunctionName,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

pub fn new_function_definition(
    function_keyword: FunctionKeyword,
    name: FunctionName,
    parameters: ParametersDeclaration,
    attributes: FunctionAttributes,
    returns: Option<ReturnsDeclaration>,
    body: FunctionBody,
) -> FunctionDefinition {
    Rc::new(FunctionDefinitionStruct {
        function_keyword,
        name,
        parameters,
        attributes,
        returns,
        body,
    })
}

pub type ParametersDeclaration = Rc<ParametersDeclarationStruct>;

#[derive(Debug)]
pub struct ParametersDeclarationStruct {
    // Do we care about range in source code?
    pub open_paren: OpenParen,
    pub parameters: Parameters,
    pub close_paren: CloseParen,
}

pub fn new_parameters_declaration(
    open_paren: OpenParen,
    parameters: Parameters,
    close_paren: CloseParen,
) -> ParametersDeclaration {
    Rc::new(ParametersDeclarationStruct {
        open_paren,
        parameters,
        close_paren,
    })
}

pub type Parameter = Rc<ParameterStruct>;

#[derive(Debug)]
pub struct ParameterStruct {
    // Do we care about range in source code?
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Identifier>,
}

pub fn new_parameter(
    type_name: TypeName,
    storage_location: Option<StorageLocation>,
    name: Option<Identifier>,
) -> Parameter {
    Rc::new(ParameterStruct {
        type_name,
        storage_location,
        name,
    })
}

pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

#[derive(Debug)]
pub struct OverrideSpecifierStruct {
    // Do we care about range in source code?
    pub override_keyword: OverrideKeyword,
    pub overridden: Option<OverridePathsDeclaration>,
}

pub fn new_override_specifier(
    override_keyword: OverrideKeyword,
    overridden: Option<OverridePathsDeclaration>,
) -> OverrideSpecifier {
    Rc::new(OverrideSpecifierStruct {
        override_keyword,
        overridden,
    })
}

pub type OverridePathsDeclaration = Rc<OverridePathsDeclarationStruct>;

#[derive(Debug)]
pub struct OverridePathsDeclarationStruct {
    // Do we care about range in source code?
    pub open_paren: OpenParen,
    pub paths: OverridePaths,
    pub close_paren: CloseParen,
}

pub fn new_override_paths_declaration(
    open_paren: OpenParen,
    paths: OverridePaths,
    close_paren: CloseParen,
) -> OverridePathsDeclaration {
    Rc::new(OverridePathsDeclarationStruct {
        open_paren,
        paths,
        close_paren,
    })
}

pub type ReturnsDeclaration = Rc<ReturnsDeclarationStruct>;

#[derive(Debug)]
pub struct ReturnsDeclarationStruct {
    // Do we care about range in source code?
    pub returns_keyword: ReturnsKeyword,
    pub variables: ParametersDeclaration,
}

pub fn new_returns_declaration(
    returns_keyword: ReturnsKeyword,
    variables: ParametersDeclaration,
) -> ReturnsDeclaration {
    Rc::new(ReturnsDeclarationStruct {
        returns_keyword,
        variables,
    })
}

pub type ConstructorDefinition = Rc<ConstructorDefinitionStruct>;

#[derive(Debug)]
pub struct ConstructorDefinitionStruct {
    // Do we care about range in source code?
    pub constructor_keyword: ConstructorKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: ConstructorAttributes,
    pub body: Block,
}

pub fn new_constructor_definition(
    constructor_keyword: ConstructorKeyword,
    parameters: ParametersDeclaration,
    attributes: ConstructorAttributes,
    body: Block,
) -> ConstructorDefinition {
    Rc::new(ConstructorDefinitionStruct {
        constructor_keyword,
        parameters,
        attributes,
        body,
    })
}

pub type UnnamedFunctionDefinition = Rc<UnnamedFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct UnnamedFunctionDefinitionStruct {
    // Do we care about range in source code?
    pub function_keyword: FunctionKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: UnnamedFunctionAttributes,
    pub body: FunctionBody,
}

pub fn new_unnamed_function_definition(
    function_keyword: FunctionKeyword,
    parameters: ParametersDeclaration,
    attributes: UnnamedFunctionAttributes,
    body: FunctionBody,
) -> UnnamedFunctionDefinition {
    Rc::new(UnnamedFunctionDefinitionStruct {
        function_keyword,
        parameters,
        attributes,
        body,
    })
}

pub type FallbackFunctionDefinition = Rc<FallbackFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FallbackFunctionDefinitionStruct {
    // Do we care about range in source code?
    pub fallback_keyword: FallbackKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: FallbackFunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

pub fn new_fallback_function_definition(
    fallback_keyword: FallbackKeyword,
    parameters: ParametersDeclaration,
    attributes: FallbackFunctionAttributes,
    returns: Option<ReturnsDeclaration>,
    body: FunctionBody,
) -> FallbackFunctionDefinition {
    Rc::new(FallbackFunctionDefinitionStruct {
        fallback_keyword,
        parameters,
        attributes,
        returns,
        body,
    })
}

pub type ReceiveFunctionDefinition = Rc<ReceiveFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct ReceiveFunctionDefinitionStruct {
    // Do we care about range in source code?
    pub receive_keyword: ReceiveKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: ReceiveFunctionAttributes,
    pub body: FunctionBody,
}

pub fn new_receive_function_definition(
    receive_keyword: ReceiveKeyword,
    parameters: ParametersDeclaration,
    attributes: ReceiveFunctionAttributes,
    body: FunctionBody,
) -> ReceiveFunctionDefinition {
    Rc::new(ReceiveFunctionDefinitionStruct {
        receive_keyword,
        parameters,
        attributes,
        body,
    })
}

pub type ModifierDefinition = Rc<ModifierDefinitionStruct>;

#[derive(Debug)]
pub struct ModifierDefinitionStruct {
    // Do we care about range in source code?
    pub modifier_keyword: ModifierKeyword,
    pub name: Identifier,
    pub parameters: Option<ParametersDeclaration>,
    pub attributes: ModifierAttributes,
    pub body: FunctionBody,
}

pub fn new_modifier_definition(
    modifier_keyword: ModifierKeyword,
    name: Identifier,
    parameters: Option<ParametersDeclaration>,
    attributes: ModifierAttributes,
    body: FunctionBody,
) -> ModifierDefinition {
    Rc::new(ModifierDefinitionStruct {
        modifier_keyword,
        name,
        parameters,
        attributes,
        body,
    })
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

#[derive(Debug)]
pub struct ModifierInvocationStruct {
    // Do we care about range in source code?
    pub name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

pub fn new_modifier_invocation(
    name: IdentifierPath,
    arguments: Option<ArgumentsDeclaration>,
) -> ModifierInvocation {
    Rc::new(ModifierInvocationStruct { name, arguments })
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

#[derive(Debug)]
pub struct EventDefinitionStruct {
    // Do we care about range in source code?
    pub event_keyword: EventKeyword,
    pub name: Identifier,
    pub parameters: EventParametersDeclaration,
    pub anonymous_keyword: Option<AnonymousKeyword>,
    pub semicolon: Semicolon,
}

pub fn new_event_definition(
    event_keyword: EventKeyword,
    name: Identifier,
    parameters: EventParametersDeclaration,
    anonymous_keyword: Option<AnonymousKeyword>,
    semicolon: Semicolon,
) -> EventDefinition {
    Rc::new(EventDefinitionStruct {
        event_keyword,
        name,
        parameters,
        anonymous_keyword,
        semicolon,
    })
}

pub type EventParametersDeclaration = Rc<EventParametersDeclarationStruct>;

#[derive(Debug)]
pub struct EventParametersDeclarationStruct {
    // Do we care about range in source code?
    pub open_paren: OpenParen,
    pub parameters: EventParameters,
    pub close_paren: CloseParen,
}

pub fn new_event_parameters_declaration(
    open_paren: OpenParen,
    parameters: EventParameters,
    close_paren: CloseParen,
) -> EventParametersDeclaration {
    Rc::new(EventParametersDeclarationStruct {
        open_paren,
        parameters,
        close_paren,
    })
}

pub type EventParameter = Rc<EventParameterStruct>;

#[derive(Debug)]
pub struct EventParameterStruct {
    // Do we care about range in source code?
    pub type_name: TypeName,
    pub indexed_keyword: Option<IndexedKeyword>,
    pub name: Option<Identifier>,
}

pub fn new_event_parameter(
    type_name: TypeName,
    indexed_keyword: Option<IndexedKeyword>,
    name: Option<Identifier>,
) -> EventParameter {
    Rc::new(EventParameterStruct {
        type_name,
        indexed_keyword,
        name,
    })
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinitionStruct {
    // Do we care about range in source code?
    pub type_keyword: TypeKeyword,
    pub name: Identifier,
    pub is_keyword: IsKeyword,
    pub value_type: ElementaryType,
    pub semicolon: Semicolon,
}

pub fn new_user_defined_value_type_definition(
    type_keyword: TypeKeyword,
    name: Identifier,
    is_keyword: IsKeyword,
    value_type: ElementaryType,
    semicolon: Semicolon,
) -> UserDefinedValueTypeDefinition {
    Rc::new(UserDefinedValueTypeDefinitionStruct {
        type_keyword,
        name,
        is_keyword,
        value_type,
        semicolon,
    })
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

#[derive(Debug)]
pub struct ErrorDefinitionStruct {
    // Do we care about range in source code?
    pub error_keyword: ErrorKeyword,
    pub name: Identifier,
    pub members: ErrorParametersDeclaration,
    pub semicolon: Semicolon,
}

pub fn new_error_definition(
    error_keyword: ErrorKeyword,
    name: Identifier,
    members: ErrorParametersDeclaration,
    semicolon: Semicolon,
) -> ErrorDefinition {
    Rc::new(ErrorDefinitionStruct {
        error_keyword,
        name,
        members,
        semicolon,
    })
}

pub type ErrorParametersDeclaration = Rc<ErrorParametersDeclarationStruct>;

#[derive(Debug)]
pub struct ErrorParametersDeclarationStruct {
    // Do we care about range in source code?
    pub open_paren: OpenParen,
    pub parameters: ErrorParameters,
    pub close_paren: CloseParen,
}

pub fn new_error_parameters_declaration(
    open_paren: OpenParen,
    parameters: ErrorParameters,
    close_paren: CloseParen,
) -> ErrorParametersDeclaration {
    Rc::new(ErrorParametersDeclarationStruct {
        open_paren,
        parameters,
        close_paren,
    })
}

pub type ErrorParameter = Rc<ErrorParameterStruct>;

#[derive(Debug)]
pub struct ErrorParameterStruct {
    // Do we care about range in source code?
    pub type_name: TypeName,
    pub name: Option<Identifier>,
}

pub fn new_error_parameter(type_name: TypeName, name: Option<Identifier>) -> ErrorParameter {
    Rc::new(ErrorParameterStruct { type_name, name })
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

#[derive(Debug)]
pub struct ArrayTypeNameStruct {
    // Do we care about range in source code?
    pub operand: TypeName,
    pub open_bracket: OpenBracket,
    pub index: Option<Expression>,
    pub close_bracket: CloseBracket,
}

pub fn new_array_type_name(
    operand: TypeName,
    open_bracket: OpenBracket,
    index: Option<Expression>,
    close_bracket: CloseBracket,
) -> ArrayTypeName {
    Rc::new(ArrayTypeNameStruct {
        operand,
        open_bracket,
        index,
        close_bracket,
    })
}

pub type FunctionType = Rc<FunctionTypeStruct>;

#[derive(Debug)]
pub struct FunctionTypeStruct {
    // Do we care about range in source code?
    pub function_keyword: FunctionKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionTypeAttributes,
    pub returns: Option<ReturnsDeclaration>,
}

pub fn new_function_type(
    function_keyword: FunctionKeyword,
    parameters: ParametersDeclaration,
    attributes: FunctionTypeAttributes,
    returns: Option<ReturnsDeclaration>,
) -> FunctionType {
    Rc::new(FunctionTypeStruct {
        function_keyword,
        parameters,
        attributes,
        returns,
    })
}

pub type MappingType = Rc<MappingTypeStruct>;

#[derive(Debug)]
pub struct MappingTypeStruct {
    // Do we care about range in source code?
    pub mapping_keyword: MappingKeyword,
    pub open_paren: OpenParen,
    pub key_type: MappingKey,
    pub equal_greater_than: EqualGreaterThan,
    pub value_type: MappingValue,
    pub close_paren: CloseParen,
}

pub fn new_mapping_type(
    mapping_keyword: MappingKeyword,
    open_paren: OpenParen,
    key_type: MappingKey,
    equal_greater_than: EqualGreaterThan,
    value_type: MappingValue,
    close_paren: CloseParen,
) -> MappingType {
    Rc::new(MappingTypeStruct {
        mapping_keyword,
        open_paren,
        key_type,
        equal_greater_than,
        value_type,
        close_paren,
    })
}

pub type MappingKey = Rc<MappingKeyStruct>;

#[derive(Debug)]
pub struct MappingKeyStruct {
    // Do we care about range in source code?
    pub key_type: MappingKeyType,
    pub name: Option<Identifier>,
}

pub fn new_mapping_key(key_type: MappingKeyType, name: Option<Identifier>) -> MappingKey {
    Rc::new(MappingKeyStruct { key_type, name })
}

pub type MappingValue = Rc<MappingValueStruct>;

#[derive(Debug)]
pub struct MappingValueStruct {
    // Do we care about range in source code?
    pub type_name: TypeName,
    pub name: Option<Identifier>,
}

pub fn new_mapping_value(type_name: TypeName, name: Option<Identifier>) -> MappingValue {
    Rc::new(MappingValueStruct { type_name, name })
}

pub type AddressType = Rc<AddressTypeStruct>;

#[derive(Debug)]
pub struct AddressTypeStruct {
    // Do we care about range in source code?
    pub address_keyword: AddressKeyword,
    pub payable_keyword: Option<PayableKeyword>,
}

pub fn new_address_type(
    address_keyword: AddressKeyword,
    payable_keyword: Option<PayableKeyword>,
) -> AddressType {
    Rc::new(AddressTypeStruct {
        address_keyword,
        payable_keyword,
    })
}

pub type Block = Rc<BlockStruct>;

#[derive(Debug)]
pub struct BlockStruct {
    // Do we care about range in source code?
    pub open_brace: OpenBrace,
    pub statements: Statements,
    pub close_brace: CloseBrace,
}

pub fn new_block(open_brace: OpenBrace, statements: Statements, close_brace: CloseBrace) -> Block {
    Rc::new(BlockStruct {
        open_brace,
        statements,
        close_brace,
    })
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

#[derive(Debug)]
pub struct UncheckedBlockStruct {
    // Do we care about range in source code?
    pub unchecked_keyword: UncheckedKeyword,
    pub block: Block,
}

pub fn new_unchecked_block(unchecked_keyword: UncheckedKeyword, block: Block) -> UncheckedBlock {
    Rc::new(UncheckedBlockStruct {
        unchecked_keyword,
        block,
    })
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

#[derive(Debug)]
pub struct ExpressionStatementStruct {
    // Do we care about range in source code?
    pub expression: Expression,
    pub semicolon: Semicolon,
}

pub fn new_expression_statement(
    expression: Expression,
    semicolon: Semicolon,
) -> ExpressionStatement {
    Rc::new(ExpressionStatementStruct {
        expression,
        semicolon,
    })
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

#[derive(Debug)]
pub struct AssemblyStatementStruct {
    // Do we care about range in source code?
    pub assembly_keyword: AssemblyKeyword,
    pub label: Option<StringLiteral>,
    pub flags: Option<AssemblyFlagsDeclaration>,
    pub body: YulBlock,
}

pub fn new_assembly_statement(
    assembly_keyword: AssemblyKeyword,
    label: Option<StringLiteral>,
    flags: Option<AssemblyFlagsDeclaration>,
    body: YulBlock,
) -> AssemblyStatement {
    Rc::new(AssemblyStatementStruct {
        assembly_keyword,
        label,
        flags,
        body,
    })
}

pub type AssemblyFlagsDeclaration = Rc<AssemblyFlagsDeclarationStruct>;

#[derive(Debug)]
pub struct AssemblyFlagsDeclarationStruct {
    // Do we care about range in source code?
    pub open_paren: OpenParen,
    pub flags: AssemblyFlags,
    pub close_paren: CloseParen,
}

pub fn new_assembly_flags_declaration(
    open_paren: OpenParen,
    flags: AssemblyFlags,
    close_paren: CloseParen,
) -> AssemblyFlagsDeclaration {
    Rc::new(AssemblyFlagsDeclarationStruct {
        open_paren,
        flags,
        close_paren,
    })
}

pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

#[derive(Debug)]
pub struct TupleDeconstructionStatementStruct {
    // Do we care about range in source code?
    pub var_keyword: Option<VarKeyword>,
    pub open_paren: OpenParen,
    pub elements: TupleDeconstructionElements,
    pub close_paren: CloseParen,
    pub equal: Equal,
    pub expression: Expression,
    pub semicolon: Semicolon,
}

pub fn new_tuple_deconstruction_statement(
    var_keyword: Option<VarKeyword>,
    open_paren: OpenParen,
    elements: TupleDeconstructionElements,
    close_paren: CloseParen,
    equal: Equal,
    expression: Expression,
    semicolon: Semicolon,
) -> TupleDeconstructionStatement {
    Rc::new(TupleDeconstructionStatementStruct {
        var_keyword,
        open_paren,
        elements,
        close_paren,
        equal,
        expression,
        semicolon,
    })
}

pub type TupleDeconstructionElement = Rc<TupleDeconstructionElementStruct>;

#[derive(Debug)]
pub struct TupleDeconstructionElementStruct {
    // Do we care about range in source code?
    pub member: Option<TupleMember>,
}

pub fn new_tuple_deconstruction_element(member: Option<TupleMember>) -> TupleDeconstructionElement {
    Rc::new(TupleDeconstructionElementStruct { member })
}

pub type TypedTupleMember = Rc<TypedTupleMemberStruct>;

#[derive(Debug)]
pub struct TypedTupleMemberStruct {
    // Do we care about range in source code?
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Identifier,
}

pub fn new_typed_tuple_member(
    type_name: TypeName,
    storage_location: Option<StorageLocation>,
    name: Identifier,
) -> TypedTupleMember {
    Rc::new(TypedTupleMemberStruct {
        type_name,
        storage_location,
        name,
    })
}

pub type UntypedTupleMember = Rc<UntypedTupleMemberStruct>;

#[derive(Debug)]
pub struct UntypedTupleMemberStruct {
    // Do we care about range in source code?
    pub storage_location: Option<StorageLocation>,
    pub name: Identifier,
}

pub fn new_untyped_tuple_member(
    storage_location: Option<StorageLocation>,
    name: Identifier,
) -> UntypedTupleMember {
    Rc::new(UntypedTupleMemberStruct {
        storage_location,
        name,
    })
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct VariableDeclarationStatementStruct {
    // Do we care about range in source code?
    pub variable_declaration: VariableDeclaration,
    pub value: Option<VariableDeclarationValue>,
    pub semicolon: Semicolon,
}

pub fn new_variable_declaration_statement(
    variable_declaration: VariableDeclaration,
    value: Option<VariableDeclarationValue>,
    semicolon: Semicolon,
) -> VariableDeclarationStatement {
    Rc::new(VariableDeclarationStatementStruct {
        variable_declaration,
        value,
        semicolon,
    })
}

pub type VariableDeclaration = Rc<VariableDeclarationStruct>;

#[derive(Debug)]
pub struct VariableDeclarationStruct {
    // Do we care about range in source code?
    pub variable_type: VariableDeclarationType,
    pub storage_location: Option<StorageLocation>,
    pub name: Identifier,
}

pub fn new_variable_declaration(
    variable_type: VariableDeclarationType,
    storage_location: Option<StorageLocation>,
    name: Identifier,
) -> VariableDeclaration {
    Rc::new(VariableDeclarationStruct {
        variable_type,
        storage_location,
        name,
    })
}

pub type VariableDeclarationValue = Rc<VariableDeclarationValueStruct>;

#[derive(Debug)]
pub struct VariableDeclarationValueStruct {
    // Do we care about range in source code?
    pub equal: Equal,
    pub expression: Expression,
}

pub fn new_variable_declaration_value(
    equal: Equal,
    expression: Expression,
) -> VariableDeclarationValue {
    Rc::new(VariableDeclarationValueStruct { equal, expression })
}

pub type IfStatement = Rc<IfStatementStruct>;

#[derive(Debug)]
pub struct IfStatementStruct {
    // Do we care about range in source code?
    pub if_keyword: IfKeyword,
    pub open_paren: OpenParen,
    pub condition: Expression,
    pub close_paren: CloseParen,
    pub body: Statement,
    pub else_branch: Option<ElseBranch>,
}

pub fn new_if_statement(
    if_keyword: IfKeyword,
    open_paren: OpenParen,
    condition: Expression,
    close_paren: CloseParen,
    body: Statement,
    else_branch: Option<ElseBranch>,
) -> IfStatement {
    Rc::new(IfStatementStruct {
        if_keyword,
        open_paren,
        condition,
        close_paren,
        body,
        else_branch,
    })
}

pub type ElseBranch = Rc<ElseBranchStruct>;

#[derive(Debug)]
pub struct ElseBranchStruct {
    // Do we care about range in source code?
    pub else_keyword: ElseKeyword,
    pub body: Statement,
}

pub fn new_else_branch(else_keyword: ElseKeyword, body: Statement) -> ElseBranch {
    Rc::new(ElseBranchStruct { else_keyword, body })
}

pub type ForStatement = Rc<ForStatementStruct>;

#[derive(Debug)]
pub struct ForStatementStruct {
    // Do we care about range in source code?
    pub for_keyword: ForKeyword,
    pub open_paren: OpenParen,
    pub initialization: ForStatementInitialization,
    pub condition: ForStatementCondition,
    pub iterator: Option<Expression>,
    pub close_paren: CloseParen,
    pub body: Statement,
}

pub fn new_for_statement(
    for_keyword: ForKeyword,
    open_paren: OpenParen,
    initialization: ForStatementInitialization,
    condition: ForStatementCondition,
    iterator: Option<Expression>,
    close_paren: CloseParen,
    body: Statement,
) -> ForStatement {
    Rc::new(ForStatementStruct {
        for_keyword,
        open_paren,
        initialization,
        condition,
        iterator,
        close_paren,
        body,
    })
}

pub type WhileStatement = Rc<WhileStatementStruct>;

#[derive(Debug)]
pub struct WhileStatementStruct {
    // Do we care about range in source code?
    pub while_keyword: WhileKeyword,
    pub open_paren: OpenParen,
    pub condition: Expression,
    pub close_paren: CloseParen,
    pub body: Statement,
}

pub fn new_while_statement(
    while_keyword: WhileKeyword,
    open_paren: OpenParen,
    condition: Expression,
    close_paren: CloseParen,
    body: Statement,
) -> WhileStatement {
    Rc::new(WhileStatementStruct {
        while_keyword,
        open_paren,
        condition,
        close_paren,
        body,
    })
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

#[derive(Debug)]
pub struct DoWhileStatementStruct {
    // Do we care about range in source code?
    pub do_keyword: DoKeyword,
    pub body: Statement,
    pub while_keyword: WhileKeyword,
    pub open_paren: OpenParen,
    pub condition: Expression,
    pub close_paren: CloseParen,
    pub semicolon: Semicolon,
}

pub fn new_do_while_statement(
    do_keyword: DoKeyword,
    body: Statement,
    while_keyword: WhileKeyword,
    open_paren: OpenParen,
    condition: Expression,
    close_paren: CloseParen,
    semicolon: Semicolon,
) -> DoWhileStatement {
    Rc::new(DoWhileStatementStruct {
        do_keyword,
        body,
        while_keyword,
        open_paren,
        condition,
        close_paren,
        semicolon,
    })
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

#[derive(Debug)]
pub struct ContinueStatementStruct {
    // Do we care about range in source code?
    pub continue_keyword: ContinueKeyword,
    pub semicolon: Semicolon,
}

pub fn new_continue_statement(
    continue_keyword: ContinueKeyword,
    semicolon: Semicolon,
) -> ContinueStatement {
    Rc::new(ContinueStatementStruct {
        continue_keyword,
        semicolon,
    })
}

pub type BreakStatement = Rc<BreakStatementStruct>;

#[derive(Debug)]
pub struct BreakStatementStruct {
    // Do we care about range in source code?
    pub break_keyword: BreakKeyword,
    pub semicolon: Semicolon,
}

pub fn new_break_statement(break_keyword: BreakKeyword, semicolon: Semicolon) -> BreakStatement {
    Rc::new(BreakStatementStruct {
        break_keyword,
        semicolon,
    })
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

#[derive(Debug)]
pub struct ReturnStatementStruct {
    // Do we care about range in source code?
    pub return_keyword: ReturnKeyword,
    pub expression: Option<Expression>,
    pub semicolon: Semicolon,
}

pub fn new_return_statement(
    return_keyword: ReturnKeyword,
    expression: Option<Expression>,
    semicolon: Semicolon,
) -> ReturnStatement {
    Rc::new(ReturnStatementStruct {
        return_keyword,
        expression,
        semicolon,
    })
}

pub type EmitStatement = Rc<EmitStatementStruct>;

#[derive(Debug)]
pub struct EmitStatementStruct {
    // Do we care about range in source code?
    pub emit_keyword: EmitKeyword,
    pub event: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
    pub semicolon: Semicolon,
}

pub fn new_emit_statement(
    emit_keyword: EmitKeyword,
    event: IdentifierPath,
    arguments: ArgumentsDeclaration,
    semicolon: Semicolon,
) -> EmitStatement {
    Rc::new(EmitStatementStruct {
        emit_keyword,
        event,
        arguments,
        semicolon,
    })
}

pub type TryStatement = Rc<TryStatementStruct>;

#[derive(Debug)]
pub struct TryStatementStruct {
    // Do we care about range in source code?
    pub try_keyword: TryKeyword,
    pub expression: Expression,
    pub returns: Option<ReturnsDeclaration>,
    pub body: Block,
    pub catch_clauses: CatchClauses,
}

pub fn new_try_statement(
    try_keyword: TryKeyword,
    expression: Expression,
    returns: Option<ReturnsDeclaration>,
    body: Block,
    catch_clauses: CatchClauses,
) -> TryStatement {
    Rc::new(TryStatementStruct {
        try_keyword,
        expression,
        returns,
        body,
        catch_clauses,
    })
}

pub type CatchClause = Rc<CatchClauseStruct>;

#[derive(Debug)]
pub struct CatchClauseStruct {
    // Do we care about range in source code?
    pub catch_keyword: CatchKeyword,
    pub error: Option<CatchClauseError>,
    pub body: Block,
}

pub fn new_catch_clause(
    catch_keyword: CatchKeyword,
    error: Option<CatchClauseError>,
    body: Block,
) -> CatchClause {
    Rc::new(CatchClauseStruct {
        catch_keyword,
        error,
        body,
    })
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

#[derive(Debug)]
pub struct CatchClauseErrorStruct {
    // Do we care about range in source code?
    pub name: Option<Identifier>,
    pub parameters: ParametersDeclaration,
}

pub fn new_catch_clause_error(
    name: Option<Identifier>,
    parameters: ParametersDeclaration,
) -> CatchClauseError {
    Rc::new(CatchClauseErrorStruct { name, parameters })
}

pub type RevertStatement = Rc<RevertStatementStruct>;

#[derive(Debug)]
pub struct RevertStatementStruct {
    // Do we care about range in source code?
    pub revert_keyword: RevertKeyword,
    pub error: Option<IdentifierPath>,
    pub arguments: ArgumentsDeclaration,
    pub semicolon: Semicolon,
}

pub fn new_revert_statement(
    revert_keyword: RevertKeyword,
    error: Option<IdentifierPath>,
    arguments: ArgumentsDeclaration,
    semicolon: Semicolon,
) -> RevertStatement {
    Rc::new(RevertStatementStruct {
        revert_keyword,
        error,
        arguments,
        semicolon,
    })
}

pub type ThrowStatement = Rc<ThrowStatementStruct>;

#[derive(Debug)]
pub struct ThrowStatementStruct {
    // Do we care about range in source code?
    pub throw_keyword: ThrowKeyword,
    pub semicolon: Semicolon,
}

pub fn new_throw_statement(throw_keyword: ThrowKeyword, semicolon: Semicolon) -> ThrowStatement {
    Rc::new(ThrowStatementStruct {
        throw_keyword,
        semicolon,
    })
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

#[derive(Debug)]
pub struct AssignmentExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: Equal,
    pub right_operand: Expression,
}

pub fn new_assignment_expression(
    left_operand: Expression,
    operator: Equal,
    right_operand: Expression,
) -> AssignmentExpression {
    Rc::new(AssignmentExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

#[derive(Debug)]
pub struct ConditionalExpressionStruct {
    // Do we care about range in source code?
    pub operand: Expression,
    pub question_mark: QuestionMark,
    pub true_expression: Expression,
    pub colon: Colon,
    pub false_expression: Expression,
}

pub fn new_conditional_expression(
    operand: Expression,
    question_mark: QuestionMark,
    true_expression: Expression,
    colon: Colon,
    false_expression: Expression,
) -> ConditionalExpression {
    Rc::new(ConditionalExpressionStruct {
        operand,
        question_mark,
        true_expression,
        colon,
        false_expression,
    })
}

pub type OrExpression = Rc<OrExpressionStruct>;

#[derive(Debug)]
pub struct OrExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: BarBar,
    pub right_operand: Expression,
}

pub fn new_or_expression(
    left_operand: Expression,
    operator: BarBar,
    right_operand: Expression,
) -> OrExpression {
    Rc::new(OrExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type AndExpression = Rc<AndExpressionStruct>;

#[derive(Debug)]
pub struct AndExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: AmpersandAmpersand,
    pub right_operand: Expression,
}

pub fn new_and_expression(
    left_operand: Expression,
    operator: AmpersandAmpersand,
    right_operand: Expression,
) -> AndExpression {
    Rc::new(AndExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

#[derive(Debug)]
pub struct EqualityExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: EqualEqual,
    pub right_operand: Expression,
}

pub fn new_equality_expression(
    left_operand: Expression,
    operator: EqualEqual,
    right_operand: Expression,
) -> EqualityExpression {
    Rc::new(EqualityExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

#[derive(Debug)]
pub struct InequalityExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: LessThan,
    pub right_operand: Expression,
}

pub fn new_inequality_expression(
    left_operand: Expression,
    operator: LessThan,
    right_operand: Expression,
) -> InequalityExpression {
    Rc::new(InequalityExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseOrExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: Bar,
    pub right_operand: Expression,
}

pub fn new_bitwise_or_expression(
    left_operand: Expression,
    operator: Bar,
    right_operand: Expression,
) -> BitwiseOrExpression {
    Rc::new(BitwiseOrExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseXorExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: Caret,
    pub right_operand: Expression,
}

pub fn new_bitwise_xor_expression(
    left_operand: Expression,
    operator: Caret,
    right_operand: Expression,
) -> BitwiseXorExpression {
    Rc::new(BitwiseXorExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseAndExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: Ampersand,
    pub right_operand: Expression,
}

pub fn new_bitwise_and_expression(
    left_operand: Expression,
    operator: Ampersand,
    right_operand: Expression,
) -> BitwiseAndExpression {
    Rc::new(BitwiseAndExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

#[derive(Debug)]
pub struct ShiftExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: LessThanLessThan,
    pub right_operand: Expression,
}

pub fn new_shift_expression(
    left_operand: Expression,
    operator: LessThanLessThan,
    right_operand: Expression,
) -> ShiftExpression {
    Rc::new(ShiftExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

#[derive(Debug)]
pub struct AdditiveExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: Plus,
    pub right_operand: Expression,
}

pub fn new_additive_expression(
    left_operand: Expression,
    operator: Plus,
    right_operand: Expression,
) -> AdditiveExpression {
    Rc::new(AdditiveExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

#[derive(Debug)]
pub struct MultiplicativeExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: Asterisk,
    pub right_operand: Expression,
}

pub fn new_multiplicative_expression(
    left_operand: Expression,
    operator: Asterisk,
    right_operand: Expression,
) -> MultiplicativeExpression {
    Rc::new(MultiplicativeExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

#[derive(Debug)]
pub struct ExponentiationExpressionStruct {
    // Do we care about range in source code?
    pub left_operand: Expression,
    pub operator: AsteriskAsterisk,
    pub right_operand: Expression,
}

pub fn new_exponentiation_expression(
    left_operand: Expression,
    operator: AsteriskAsterisk,
    right_operand: Expression,
) -> ExponentiationExpression {
    Rc::new(ExponentiationExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

#[derive(Debug)]
pub struct PostfixExpressionStruct {
    // Do we care about range in source code?
    pub operand: Expression,
    pub operator: PlusPlus,
}

pub fn new_postfix_expression(operand: Expression, operator: PlusPlus) -> PostfixExpression {
    Rc::new(PostfixExpressionStruct { operand, operator })
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

#[derive(Debug)]
pub struct PrefixExpressionStruct {
    // Do we care about range in source code?
    pub operator: PlusPlus,
    pub operand: Expression,
}

pub fn new_prefix_expression(operator: PlusPlus, operand: Expression) -> PrefixExpression {
    Rc::new(PrefixExpressionStruct { operator, operand })
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct FunctionCallExpressionStruct {
    // Do we care about range in source code?
    pub operand: Expression,
    pub arguments: ArgumentsDeclaration,
}

pub fn new_function_call_expression(
    operand: Expression,
    arguments: ArgumentsDeclaration,
) -> FunctionCallExpression {
    Rc::new(FunctionCallExpressionStruct { operand, arguments })
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

#[derive(Debug)]
pub struct CallOptionsExpressionStruct {
    // Do we care about range in source code?
    pub operand: Expression,
    pub open_brace: OpenBrace,
    pub options: CallOptions,
    pub close_brace: CloseBrace,
}

pub fn new_call_options_expression(
    operand: Expression,
    open_brace: OpenBrace,
    options: CallOptions,
    close_brace: CloseBrace,
) -> CallOptionsExpression {
    Rc::new(CallOptionsExpressionStruct {
        operand,
        open_brace,
        options,
        close_brace,
    })
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

#[derive(Debug)]
pub struct MemberAccessExpressionStruct {
    // Do we care about range in source code?
    pub operand: Expression,
    pub period: Period,
    pub member: Identifier,
}

pub fn new_member_access_expression(
    operand: Expression,
    period: Period,
    member: Identifier,
) -> MemberAccessExpression {
    Rc::new(MemberAccessExpressionStruct {
        operand,
        period,
        member,
    })
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

#[derive(Debug)]
pub struct IndexAccessExpressionStruct {
    // Do we care about range in source code?
    pub operand: Expression,
    pub open_bracket: OpenBracket,
    pub start: Option<Expression>,
    pub end: Option<IndexAccessEnd>,
    pub close_bracket: CloseBracket,
}

pub fn new_index_access_expression(
    operand: Expression,
    open_bracket: OpenBracket,
    start: Option<Expression>,
    end: Option<IndexAccessEnd>,
    close_bracket: CloseBracket,
) -> IndexAccessExpression {
    Rc::new(IndexAccessExpressionStruct {
        operand,
        open_bracket,
        start,
        end,
        close_bracket,
    })
}

pub type IndexAccessEnd = Rc<IndexAccessEndStruct>;

#[derive(Debug)]
pub struct IndexAccessEndStruct {
    // Do we care about range in source code?
    pub colon: Colon,
    pub end: Option<Expression>,
}

pub fn new_index_access_end(colon: Colon, end: Option<Expression>) -> IndexAccessEnd {
    Rc::new(IndexAccessEndStruct { colon, end })
}

pub type PositionalArgumentsDeclaration = Rc<PositionalArgumentsDeclarationStruct>;

#[derive(Debug)]
pub struct PositionalArgumentsDeclarationStruct {
    // Do we care about range in source code?
    pub open_paren: OpenParen,
    pub arguments: PositionalArguments,
    pub close_paren: CloseParen,
}

pub fn new_positional_arguments_declaration(
    open_paren: OpenParen,
    arguments: PositionalArguments,
    close_paren: CloseParen,
) -> PositionalArgumentsDeclaration {
    Rc::new(PositionalArgumentsDeclarationStruct {
        open_paren,
        arguments,
        close_paren,
    })
}

pub type NamedArgumentsDeclaration = Rc<NamedArgumentsDeclarationStruct>;

#[derive(Debug)]
pub struct NamedArgumentsDeclarationStruct {
    // Do we care about range in source code?
    pub open_paren: OpenParen,
    pub arguments: NamedArgumentGroup,
    pub close_paren: CloseParen,
}

pub fn new_named_arguments_declaration(
    open_paren: OpenParen,
    arguments: NamedArgumentGroup,
    close_paren: CloseParen,
) -> NamedArgumentsDeclaration {
    Rc::new(NamedArgumentsDeclarationStruct {
        open_paren,
        arguments,
        close_paren,
    })
}

pub type NamedArgumentGroup = Rc<NamedArgumentGroupStruct>;

#[derive(Debug)]
pub struct NamedArgumentGroupStruct {
    // Do we care about range in source code?
    pub open_brace: OpenBrace,
    pub arguments: NamedArguments,
    pub close_brace: CloseBrace,
}

pub fn new_named_argument_group(
    open_brace: OpenBrace,
    arguments: NamedArguments,
    close_brace: CloseBrace,
) -> NamedArgumentGroup {
    Rc::new(NamedArgumentGroupStruct {
        open_brace,
        arguments,
        close_brace,
    })
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

#[derive(Debug)]
pub struct NamedArgumentStruct {
    // Do we care about range in source code?
    pub name: Identifier,
    pub colon: Colon,
    pub value: Expression,
}

pub fn new_named_argument(name: Identifier, colon: Colon, value: Expression) -> NamedArgument {
    Rc::new(NamedArgumentStruct { name, colon, value })
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

#[derive(Debug)]
pub struct TypeExpressionStruct {
    // Do we care about range in source code?
    pub type_keyword: TypeKeyword,
    pub open_paren: OpenParen,
    pub type_name: TypeName,
    pub close_paren: CloseParen,
}

pub fn new_type_expression(
    type_keyword: TypeKeyword,
    open_paren: OpenParen,
    type_name: TypeName,
    close_paren: CloseParen,
) -> TypeExpression {
    Rc::new(TypeExpressionStruct {
        type_keyword,
        open_paren,
        type_name,
        close_paren,
    })
}

pub type NewExpression = Rc<NewExpressionStruct>;

#[derive(Debug)]
pub struct NewExpressionStruct {
    // Do we care about range in source code?
    pub new_keyword: NewKeyword,
    pub type_name: TypeName,
    pub arguments: PositionalArgumentsDeclaration,
}

pub fn new_new_expression(
    new_keyword: NewKeyword,
    type_name: TypeName,
    arguments: PositionalArgumentsDeclaration,
) -> NewExpression {
    Rc::new(NewExpressionStruct {
        new_keyword,
        type_name,
        arguments,
    })
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

#[derive(Debug)]
pub struct TupleExpressionStruct {
    // Do we care about range in source code?
    pub open_paren: OpenParen,
    pub items: TupleValues,
    pub close_paren: CloseParen,
}

pub fn new_tuple_expression(
    open_paren: OpenParen,
    items: TupleValues,
    close_paren: CloseParen,
) -> TupleExpression {
    Rc::new(TupleExpressionStruct {
        open_paren,
        items,
        close_paren,
    })
}

pub type TupleValue = Rc<TupleValueStruct>;

#[derive(Debug)]
pub struct TupleValueStruct {
    // Do we care about range in source code?
    pub expression: Option<Expression>,
}

pub fn new_tuple_value(expression: Option<Expression>) -> TupleValue {
    Rc::new(TupleValueStruct { expression })
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

#[derive(Debug)]
pub struct ArrayExpressionStruct {
    // Do we care about range in source code?
    pub open_bracket: OpenBracket,
    pub items: ArrayValues,
    pub close_bracket: CloseBracket,
}

pub fn new_array_expression(
    open_bracket: OpenBracket,
    items: ArrayValues,
    close_bracket: CloseBracket,
) -> ArrayExpression {
    Rc::new(ArrayExpressionStruct {
        open_bracket,
        items,
        close_bracket,
    })
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

#[derive(Debug)]
pub struct HexNumberExpressionStruct {
    // Do we care about range in source code?
    pub literal: HexLiteral,
    pub unit: Option<NumberUnit>,
}

pub fn new_hex_number_expression(
    literal: HexLiteral,
    unit: Option<NumberUnit>,
) -> HexNumberExpression {
    Rc::new(HexNumberExpressionStruct { literal, unit })
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

#[derive(Debug)]
pub struct DecimalNumberExpressionStruct {
    // Do we care about range in source code?
    pub literal: DecimalLiteral,
    pub unit: Option<NumberUnit>,
}

pub fn new_decimal_number_expression(
    literal: DecimalLiteral,
    unit: Option<NumberUnit>,
) -> DecimalNumberExpression {
    Rc::new(DecimalNumberExpressionStruct { literal, unit })
}

pub type YulBlock = Rc<YulBlockStruct>;

#[derive(Debug)]
pub struct YulBlockStruct {
    // Do we care about range in source code?
    pub open_brace: OpenBrace,
    pub statements: YulStatements,
    pub close_brace: CloseBrace,
}

pub fn new_yul_block(
    open_brace: OpenBrace,
    statements: YulStatements,
    close_brace: CloseBrace,
) -> YulBlock {
    Rc::new(YulBlockStruct {
        open_brace,
        statements,
        close_brace,
    })
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct YulFunctionDefinitionStruct {
    // Do we care about range in source code?
    pub function_keyword: YulFunctionKeyword,
    pub name: YulIdentifier,
    pub parameters: YulParametersDeclaration,
    pub returns: Option<YulReturnsDeclaration>,
    pub body: YulBlock,
}

pub fn new_yul_function_definition(
    function_keyword: YulFunctionKeyword,
    name: YulIdentifier,
    parameters: YulParametersDeclaration,
    returns: Option<YulReturnsDeclaration>,
    body: YulBlock,
) -> YulFunctionDefinition {
    Rc::new(YulFunctionDefinitionStruct {
        function_keyword,
        name,
        parameters,
        returns,
        body,
    })
}

pub type YulParametersDeclaration = Rc<YulParametersDeclarationStruct>;

#[derive(Debug)]
pub struct YulParametersDeclarationStruct {
    // Do we care about range in source code?
    pub open_paren: OpenParen,
    pub parameters: YulParameters,
    pub close_paren: CloseParen,
}

pub fn new_yul_parameters_declaration(
    open_paren: OpenParen,
    parameters: YulParameters,
    close_paren: CloseParen,
) -> YulParametersDeclaration {
    Rc::new(YulParametersDeclarationStruct {
        open_paren,
        parameters,
        close_paren,
    })
}

pub type YulReturnsDeclaration = Rc<YulReturnsDeclarationStruct>;

#[derive(Debug)]
pub struct YulReturnsDeclarationStruct {
    // Do we care about range in source code?
    pub minus_greater_than: MinusGreaterThan,
    pub variables: YulVariableNames,
}

pub fn new_yul_returns_declaration(
    minus_greater_than: MinusGreaterThan,
    variables: YulVariableNames,
) -> YulReturnsDeclaration {
    Rc::new(YulReturnsDeclarationStruct {
        minus_greater_than,
        variables,
    })
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationStatementStruct {
    // Do we care about range in source code?
    pub let_keyword: YulLetKeyword,
    pub variables: YulVariableNames,
    pub value: Option<YulVariableDeclarationValue>,
}

pub fn new_yul_variable_declaration_statement(
    let_keyword: YulLetKeyword,
    variables: YulVariableNames,
    value: Option<YulVariableDeclarationValue>,
) -> YulVariableDeclarationStatement {
    Rc::new(YulVariableDeclarationStatementStruct {
        let_keyword,
        variables,
        value,
    })
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationValueStruct {
    // Do we care about range in source code?
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

pub fn new_yul_variable_declaration_value(
    assignment: YulAssignmentOperator,
    expression: YulExpression,
) -> YulVariableDeclarationValue {
    Rc::new(YulVariableDeclarationValueStruct {
        assignment,
        expression,
    })
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

#[derive(Debug)]
pub struct YulVariableAssignmentStatementStruct {
    // Do we care about range in source code?
    pub variables: YulPaths,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

pub fn new_yul_variable_assignment_statement(
    variables: YulPaths,
    assignment: YulAssignmentOperator,
    expression: YulExpression,
) -> YulVariableAssignmentStatement {
    Rc::new(YulVariableAssignmentStatementStruct {
        variables,
        assignment,
        expression,
    })
}

pub type YulColonAndEqual = Rc<YulColonAndEqualStruct>;

#[derive(Debug)]
pub struct YulColonAndEqualStruct {
    // Do we care about range in source code?
    pub colon: Colon,
    pub equal: Equal,
}

pub fn new_yul_colon_and_equal(colon: Colon, equal: Equal) -> YulColonAndEqual {
    Rc::new(YulColonAndEqualStruct { colon, equal })
}

pub type YulStackAssignmentStatement = Rc<YulStackAssignmentStatementStruct>;

#[derive(Debug)]
pub struct YulStackAssignmentStatementStruct {
    // Do we care about range in source code?
    pub assignment: YulStackAssignmentOperator,
    pub variable: YulIdentifier,
}

pub fn new_yul_stack_assignment_statement(
    assignment: YulStackAssignmentOperator,
    variable: YulIdentifier,
) -> YulStackAssignmentStatement {
    Rc::new(YulStackAssignmentStatementStruct {
        assignment,
        variable,
    })
}

pub type YulEqualAndColon = Rc<YulEqualAndColonStruct>;

#[derive(Debug)]
pub struct YulEqualAndColonStruct {
    // Do we care about range in source code?
    pub equal: Equal,
    pub colon: Colon,
}

pub fn new_yul_equal_and_colon(equal: Equal, colon: Colon) -> YulEqualAndColon {
    Rc::new(YulEqualAndColonStruct { equal, colon })
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

#[derive(Debug)]
pub struct YulIfStatementStruct {
    // Do we care about range in source code?
    pub if_keyword: YulIfKeyword,
    pub condition: YulExpression,
    pub body: YulBlock,
}

pub fn new_yul_if_statement(
    if_keyword: YulIfKeyword,
    condition: YulExpression,
    body: YulBlock,
) -> YulIfStatement {
    Rc::new(YulIfStatementStruct {
        if_keyword,
        condition,
        body,
    })
}

pub type YulForStatement = Rc<YulForStatementStruct>;

#[derive(Debug)]
pub struct YulForStatementStruct {
    // Do we care about range in source code?
    pub for_keyword: YulForKeyword,
    pub initialization: YulBlock,
    pub condition: YulExpression,
    pub iterator: YulBlock,
    pub body: YulBlock,
}

pub fn new_yul_for_statement(
    for_keyword: YulForKeyword,
    initialization: YulBlock,
    condition: YulExpression,
    iterator: YulBlock,
    body: YulBlock,
) -> YulForStatement {
    Rc::new(YulForStatementStruct {
        for_keyword,
        initialization,
        condition,
        iterator,
        body,
    })
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

#[derive(Debug)]
pub struct YulSwitchStatementStruct {
    // Do we care about range in source code?
    pub switch_keyword: YulSwitchKeyword,
    pub expression: YulExpression,
    pub cases: YulSwitchCases,
}

pub fn new_yul_switch_statement(
    switch_keyword: YulSwitchKeyword,
    expression: YulExpression,
    cases: YulSwitchCases,
) -> YulSwitchStatement {
    Rc::new(YulSwitchStatementStruct {
        switch_keyword,
        expression,
        cases,
    })
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

#[derive(Debug)]
pub struct YulDefaultCaseStruct {
    // Do we care about range in source code?
    pub default_keyword: YulDefaultKeyword,
    pub body: YulBlock,
}

pub fn new_yul_default_case(default_keyword: YulDefaultKeyword, body: YulBlock) -> YulDefaultCase {
    Rc::new(YulDefaultCaseStruct {
        default_keyword,
        body,
    })
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

#[derive(Debug)]
pub struct YulValueCaseStruct {
    // Do we care about range in source code?
    pub case_keyword: YulCaseKeyword,
    pub value: YulLiteral,
    pub body: YulBlock,
}

pub fn new_yul_value_case(
    case_keyword: YulCaseKeyword,
    value: YulLiteral,
    body: YulBlock,
) -> YulValueCase {
    Rc::new(YulValueCaseStruct {
        case_keyword,
        value,
        body,
    })
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

#[derive(Debug)]
pub struct YulLeaveStatementStruct {
    // Do we care about range in source code?
    pub leave_keyword: YulLeaveKeyword,
}

pub fn new_yul_leave_statement(leave_keyword: YulLeaveKeyword) -> YulLeaveStatement {
    Rc::new(YulLeaveStatementStruct { leave_keyword })
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

#[derive(Debug)]
pub struct YulBreakStatementStruct {
    // Do we care about range in source code?
    pub break_keyword: YulBreakKeyword,
}

pub fn new_yul_break_statement(break_keyword: YulBreakKeyword) -> YulBreakStatement {
    Rc::new(YulBreakStatementStruct { break_keyword })
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

#[derive(Debug)]
pub struct YulContinueStatementStruct {
    // Do we care about range in source code?
    pub continue_keyword: YulContinueKeyword,
}

pub fn new_yul_continue_statement(continue_keyword: YulContinueKeyword) -> YulContinueStatement {
    Rc::new(YulContinueStatementStruct { continue_keyword })
}

pub type YulLabel = Rc<YulLabelStruct>;

#[derive(Debug)]
pub struct YulLabelStruct {
    // Do we care about range in source code?
    pub label: YulIdentifier,
    pub colon: Colon,
}

pub fn new_yul_label(label: YulIdentifier, colon: Colon) -> YulLabel {
    Rc::new(YulLabelStruct { label, colon })
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct YulFunctionCallExpressionStruct {
    // Do we care about range in source code?
    pub operand: YulExpression,
    pub open_paren: OpenParen,
    pub arguments: YulArguments,
    pub close_paren: CloseParen,
}

pub fn new_yul_function_call_expression(
    operand: YulExpression,
    open_paren: OpenParen,
    arguments: YulArguments,
    close_paren: CloseParen,
) -> YulFunctionCallExpression {
    Rc::new(YulFunctionCallExpressionStruct {
        operand,
        open_paren,
        arguments,
        close_paren,
    })
}

//
// Choices:
//

#[derive(Debug, Clone)]
pub enum SourceUnitMember {
    PragmaDirective(PragmaDirective),
    ImportDirective(ImportDirective),
    ContractDefinition(ContractDefinition),
    InterfaceDefinition(InterfaceDefinition),
    LibraryDefinition(LibraryDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    FunctionDefinition(FunctionDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingDirective(UsingDirective),
    EventDefinition(EventDefinition),
    ConstantDefinition(ConstantDefinition),
}

pub fn new_source_unit_member_pragma_directive(element: PragmaDirective) -> SourceUnitMember {
    SourceUnitMember::PragmaDirective(element)
}

pub fn new_source_unit_member_import_directive(element: ImportDirective) -> SourceUnitMember {
    SourceUnitMember::ImportDirective(element)
}

pub fn new_source_unit_member_contract_definition(element: ContractDefinition) -> SourceUnitMember {
    SourceUnitMember::ContractDefinition(element)
}

pub fn new_source_unit_member_interface_definition(
    element: InterfaceDefinition,
) -> SourceUnitMember {
    SourceUnitMember::InterfaceDefinition(element)
}

pub fn new_source_unit_member_library_definition(element: LibraryDefinition) -> SourceUnitMember {
    SourceUnitMember::LibraryDefinition(element)
}

pub fn new_source_unit_member_struct_definition(element: StructDefinition) -> SourceUnitMember {
    SourceUnitMember::StructDefinition(element)
}

pub fn new_source_unit_member_enum_definition(element: EnumDefinition) -> SourceUnitMember {
    SourceUnitMember::EnumDefinition(element)
}

pub fn new_source_unit_member_function_definition(element: FunctionDefinition) -> SourceUnitMember {
    SourceUnitMember::FunctionDefinition(element)
}

pub fn new_source_unit_member_error_definition(element: ErrorDefinition) -> SourceUnitMember {
    SourceUnitMember::ErrorDefinition(element)
}

pub fn new_source_unit_member_user_defined_value_type_definition(
    element: UserDefinedValueTypeDefinition,
) -> SourceUnitMember {
    SourceUnitMember::UserDefinedValueTypeDefinition(element)
}

pub fn new_source_unit_member_using_directive(element: UsingDirective) -> SourceUnitMember {
    SourceUnitMember::UsingDirective(element)
}

pub fn new_source_unit_member_event_definition(element: EventDefinition) -> SourceUnitMember {
    SourceUnitMember::EventDefinition(element)
}

pub fn new_source_unit_member_constant_definition(element: ConstantDefinition) -> SourceUnitMember {
    SourceUnitMember::ConstantDefinition(element)
}

#[derive(Debug, Clone)]
pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

pub fn new_pragma_version_pragma(element: VersionPragma) -> Pragma {
    Pragma::VersionPragma(element)
}

pub fn new_pragma_abicoder_pragma(element: AbicoderPragma) -> Pragma {
    Pragma::AbicoderPragma(element)
}

pub fn new_pragma_experimental_pragma(element: ExperimentalPragma) -> Pragma {
    Pragma::ExperimentalPragma(element)
}

#[derive(Debug, Clone)]
pub enum AbicoderVersion {
    AbicoderV1Keyword(AbicoderV1Keyword),
    AbicoderV2Keyword(AbicoderV2Keyword),
}

pub fn new_abicoder_version_abicoder_v1_keyword(element: AbicoderV1Keyword) -> AbicoderVersion {
    AbicoderVersion::AbicoderV1Keyword(element)
}

pub fn new_abicoder_version_abicoder_v2_keyword(element: AbicoderV2Keyword) -> AbicoderVersion {
    AbicoderVersion::AbicoderV2Keyword(element)
}

#[derive(Debug, Clone)]
pub enum ExperimentalFeature {
    ABIEncoderV2Keyword(ABIEncoderV2Keyword),
    SMTCheckerKeyword(SMTCheckerKeyword),
    StringLiteral(StringLiteral),
}

pub fn new_experimental_feature_abi_encoder_v2_keyword(
    element: ABIEncoderV2Keyword,
) -> ExperimentalFeature {
    ExperimentalFeature::ABIEncoderV2Keyword(element)
}

pub fn new_experimental_feature_smt_checker_keyword(
    element: SMTCheckerKeyword,
) -> ExperimentalFeature {
    ExperimentalFeature::SMTCheckerKeyword(element)
}

pub fn new_experimental_feature_string_literal(element: StringLiteral) -> ExperimentalFeature {
    ExperimentalFeature::StringLiteral(element)
}

#[derive(Debug, Clone)]
pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

pub fn new_version_expression_version_range(element: VersionRange) -> VersionExpression {
    VersionExpression::VersionRange(element)
}

pub fn new_version_expression_version_term(element: VersionTerm) -> VersionExpression {
    VersionExpression::VersionTerm(element)
}

#[derive(Debug, Clone)]
pub enum VersionOperator {
    Caret(Caret),
    Tilde(Tilde),
    Equal(Equal),
    LessThan(LessThan),
    GreaterThan(GreaterThan),
    LessThanEqual(LessThanEqual),
    GreaterThanEqual(GreaterThanEqual),
}

pub fn new_version_operator_caret(element: Caret) -> VersionOperator {
    VersionOperator::Caret(element)
}

pub fn new_version_operator_tilde(element: Tilde) -> VersionOperator {
    VersionOperator::Tilde(element)
}

pub fn new_version_operator_equal(element: Equal) -> VersionOperator {
    VersionOperator::Equal(element)
}

pub fn new_version_operator_less_than(element: LessThan) -> VersionOperator {
    VersionOperator::LessThan(element)
}

pub fn new_version_operator_greater_than(element: GreaterThan) -> VersionOperator {
    VersionOperator::GreaterThan(element)
}

pub fn new_version_operator_less_than_equal(element: LessThanEqual) -> VersionOperator {
    VersionOperator::LessThanEqual(element)
}

pub fn new_version_operator_greater_than_equal(element: GreaterThanEqual) -> VersionOperator {
    VersionOperator::GreaterThanEqual(element)
}

#[derive(Debug, Clone)]
pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    SingleQuotedVersionLiteral(SingleQuotedVersionLiteral),
    DoubleQuotedVersionLiteral(DoubleQuotedVersionLiteral),
}

pub fn new_version_literal_simple_version_literal(element: SimpleVersionLiteral) -> VersionLiteral {
    VersionLiteral::SimpleVersionLiteral(element)
}

pub fn new_version_literal_single_quoted_version_literal(
    element: SingleQuotedVersionLiteral,
) -> VersionLiteral {
    VersionLiteral::SingleQuotedVersionLiteral(element)
}

pub fn new_version_literal_double_quoted_version_literal(
    element: DoubleQuotedVersionLiteral,
) -> VersionLiteral {
    VersionLiteral::DoubleQuotedVersionLiteral(element)
}

#[derive(Debug, Clone)]
pub enum ImportClause {
    PathImport(PathImport),
    NamedImport(NamedImport),
    ImportDeconstruction(ImportDeconstruction),
}

pub fn new_import_clause_path_import(element: PathImport) -> ImportClause {
    ImportClause::PathImport(element)
}

pub fn new_import_clause_named_import(element: NamedImport) -> ImportClause {
    ImportClause::NamedImport(element)
}

pub fn new_import_clause_import_deconstruction(element: ImportDeconstruction) -> ImportClause {
    ImportClause::ImportDeconstruction(element)
}

#[derive(Debug, Clone)]
pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

pub fn new_using_clause_identifier_path(element: IdentifierPath) -> UsingClause {
    UsingClause::IdentifierPath(element)
}

pub fn new_using_clause_using_deconstruction(element: UsingDeconstruction) -> UsingClause {
    UsingClause::UsingDeconstruction(element)
}

#[derive(Debug, Clone)]
pub enum UsingOperator {
    Ampersand(Ampersand),
    Asterisk(Asterisk),
    BangEqual(BangEqual),
    Bar(Bar),
    Caret(Caret),
    EqualEqual(EqualEqual),
    GreaterThan(GreaterThan),
    GreaterThanEqual(GreaterThanEqual),
    LessThan(LessThan),
    LessThanEqual(LessThanEqual),
    Minus(Minus),
    Percent(Percent),
    Plus(Plus),
    Slash(Slash),
    Tilde(Tilde),
}

pub fn new_using_operator_ampersand(element: Ampersand) -> UsingOperator {
    UsingOperator::Ampersand(element)
}

pub fn new_using_operator_asterisk(element: Asterisk) -> UsingOperator {
    UsingOperator::Asterisk(element)
}

pub fn new_using_operator_bang_equal(element: BangEqual) -> UsingOperator {
    UsingOperator::BangEqual(element)
}

pub fn new_using_operator_bar(element: Bar) -> UsingOperator {
    UsingOperator::Bar(element)
}

pub fn new_using_operator_caret(element: Caret) -> UsingOperator {
    UsingOperator::Caret(element)
}

pub fn new_using_operator_equal_equal(element: EqualEqual) -> UsingOperator {
    UsingOperator::EqualEqual(element)
}

pub fn new_using_operator_greater_than(element: GreaterThan) -> UsingOperator {
    UsingOperator::GreaterThan(element)
}

pub fn new_using_operator_greater_than_equal(element: GreaterThanEqual) -> UsingOperator {
    UsingOperator::GreaterThanEqual(element)
}

pub fn new_using_operator_less_than(element: LessThan) -> UsingOperator {
    UsingOperator::LessThan(element)
}

pub fn new_using_operator_less_than_equal(element: LessThanEqual) -> UsingOperator {
    UsingOperator::LessThanEqual(element)
}

pub fn new_using_operator_minus(element: Minus) -> UsingOperator {
    UsingOperator::Minus(element)
}

pub fn new_using_operator_percent(element: Percent) -> UsingOperator {
    UsingOperator::Percent(element)
}

pub fn new_using_operator_plus(element: Plus) -> UsingOperator {
    UsingOperator::Plus(element)
}

pub fn new_using_operator_slash(element: Slash) -> UsingOperator {
    UsingOperator::Slash(element)
}

pub fn new_using_operator_tilde(element: Tilde) -> UsingOperator {
    UsingOperator::Tilde(element)
}

#[derive(Debug, Clone)]
pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk(Asterisk),
}

pub fn new_using_target_type_name(element: TypeName) -> UsingTarget {
    UsingTarget::TypeName(element)
}

pub fn new_using_target_asterisk(element: Asterisk) -> UsingTarget {
    UsingTarget::Asterisk(element)
}

#[derive(Debug, Clone)]
pub enum ContractSpecifier {
    InheritanceSpecifier(InheritanceSpecifier),
    StorageLayoutSpecifier(StorageLayoutSpecifier),
}

pub fn new_contract_specifier_inheritance_specifier(
    element: InheritanceSpecifier,
) -> ContractSpecifier {
    ContractSpecifier::InheritanceSpecifier(element)
}

pub fn new_contract_specifier_storage_layout_specifier(
    element: StorageLayoutSpecifier,
) -> ContractSpecifier {
    ContractSpecifier::StorageLayoutSpecifier(element)
}

#[derive(Debug, Clone)]
pub enum ContractMember {
    UsingDirective(UsingDirective),
    FunctionDefinition(FunctionDefinition),
    ConstructorDefinition(ConstructorDefinition),
    ReceiveFunctionDefinition(ReceiveFunctionDefinition),
    FallbackFunctionDefinition(FallbackFunctionDefinition),
    UnnamedFunctionDefinition(UnnamedFunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    StateVariableDefinition(StateVariableDefinition),
}

pub fn new_contract_member_using_directive(element: UsingDirective) -> ContractMember {
    ContractMember::UsingDirective(element)
}

pub fn new_contract_member_function_definition(element: FunctionDefinition) -> ContractMember {
    ContractMember::FunctionDefinition(element)
}

pub fn new_contract_member_constructor_definition(
    element: ConstructorDefinition,
) -> ContractMember {
    ContractMember::ConstructorDefinition(element)
}

pub fn new_contract_member_receive_function_definition(
    element: ReceiveFunctionDefinition,
) -> ContractMember {
    ContractMember::ReceiveFunctionDefinition(element)
}

pub fn new_contract_member_fallback_function_definition(
    element: FallbackFunctionDefinition,
) -> ContractMember {
    ContractMember::FallbackFunctionDefinition(element)
}

pub fn new_contract_member_unnamed_function_definition(
    element: UnnamedFunctionDefinition,
) -> ContractMember {
    ContractMember::UnnamedFunctionDefinition(element)
}

pub fn new_contract_member_modifier_definition(element: ModifierDefinition) -> ContractMember {
    ContractMember::ModifierDefinition(element)
}

pub fn new_contract_member_struct_definition(element: StructDefinition) -> ContractMember {
    ContractMember::StructDefinition(element)
}

pub fn new_contract_member_enum_definition(element: EnumDefinition) -> ContractMember {
    ContractMember::EnumDefinition(element)
}

pub fn new_contract_member_event_definition(element: EventDefinition) -> ContractMember {
    ContractMember::EventDefinition(element)
}

pub fn new_contract_member_error_definition(element: ErrorDefinition) -> ContractMember {
    ContractMember::ErrorDefinition(element)
}

pub fn new_contract_member_user_defined_value_type_definition(
    element: UserDefinedValueTypeDefinition,
) -> ContractMember {
    ContractMember::UserDefinedValueTypeDefinition(element)
}

pub fn new_contract_member_state_variable_definition(
    element: StateVariableDefinition,
) -> ContractMember {
    ContractMember::StateVariableDefinition(element)
}

#[derive(Debug, Clone)]
pub enum StateVariableAttribute {
    OverrideSpecifier(OverrideSpecifier),
    ConstantKeyword(ConstantKeyword),
    InternalKeyword(InternalKeyword),
    PrivateKeyword(PrivateKeyword),
    PublicKeyword(PublicKeyword),
    ImmutableKeyword(ImmutableKeyword),
    TransientKeyword(TransientKeyword),
}

pub fn new_state_variable_attribute_override_specifier(
    element: OverrideSpecifier,
) -> StateVariableAttribute {
    StateVariableAttribute::OverrideSpecifier(element)
}

pub fn new_state_variable_attribute_constant_keyword(
    element: ConstantKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::ConstantKeyword(element)
}

pub fn new_state_variable_attribute_internal_keyword(
    element: InternalKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::InternalKeyword(element)
}

pub fn new_state_variable_attribute_private_keyword(
    element: PrivateKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::PrivateKeyword(element)
}

pub fn new_state_variable_attribute_public_keyword(
    element: PublicKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::PublicKeyword(element)
}

pub fn new_state_variable_attribute_immutable_keyword(
    element: ImmutableKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::ImmutableKeyword(element)
}

pub fn new_state_variable_attribute_transient_keyword(
    element: TransientKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::TransientKeyword(element)
}

#[derive(Debug, Clone)]
pub enum FunctionName {
    Identifier(Identifier),
    FallbackKeyword(FallbackKeyword),
    ReceiveKeyword(ReceiveKeyword),
}

pub fn new_function_name_identifier(element: Identifier) -> FunctionName {
    FunctionName::Identifier(element)
}

pub fn new_function_name_fallback_keyword(element: FallbackKeyword) -> FunctionName {
    FunctionName::FallbackKeyword(element)
}

pub fn new_function_name_receive_keyword(element: ReceiveKeyword) -> FunctionName {
    FunctionName::ReceiveKeyword(element)
}

#[derive(Debug, Clone)]
pub enum FunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ConstantKeyword(ConstantKeyword),
    ExternalKeyword(ExternalKeyword),
    InternalKeyword(InternalKeyword),
    PayableKeyword(PayableKeyword),
    PrivateKeyword(PrivateKeyword),
    PublicKeyword(PublicKeyword),
    PureKeyword(PureKeyword),
    ViewKeyword(ViewKeyword),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> FunctionAttribute {
    FunctionAttribute::ModifierInvocation(element)
}

pub fn new_function_attribute_override_specifier(element: OverrideSpecifier) -> FunctionAttribute {
    FunctionAttribute::OverrideSpecifier(element)
}

pub fn new_function_attribute_constant_keyword(element: ConstantKeyword) -> FunctionAttribute {
    FunctionAttribute::ConstantKeyword(element)
}

pub fn new_function_attribute_external_keyword(element: ExternalKeyword) -> FunctionAttribute {
    FunctionAttribute::ExternalKeyword(element)
}

pub fn new_function_attribute_internal_keyword(element: InternalKeyword) -> FunctionAttribute {
    FunctionAttribute::InternalKeyword(element)
}

pub fn new_function_attribute_payable_keyword(element: PayableKeyword) -> FunctionAttribute {
    FunctionAttribute::PayableKeyword(element)
}

pub fn new_function_attribute_private_keyword(element: PrivateKeyword) -> FunctionAttribute {
    FunctionAttribute::PrivateKeyword(element)
}

pub fn new_function_attribute_public_keyword(element: PublicKeyword) -> FunctionAttribute {
    FunctionAttribute::PublicKeyword(element)
}

pub fn new_function_attribute_pure_keyword(element: PureKeyword) -> FunctionAttribute {
    FunctionAttribute::PureKeyword(element)
}

pub fn new_function_attribute_view_keyword(element: ViewKeyword) -> FunctionAttribute {
    FunctionAttribute::ViewKeyword(element)
}

pub fn new_function_attribute_virtual_keyword(element: VirtualKeyword) -> FunctionAttribute {
    FunctionAttribute::VirtualKeyword(element)
}

#[derive(Debug, Clone)]
pub enum FunctionBody {
    Block(Block),
    Semicolon(Semicolon),
}

pub fn new_function_body_block(element: Block) -> FunctionBody {
    FunctionBody::Block(element)
}

pub fn new_function_body_semicolon(element: Semicolon) -> FunctionBody {
    FunctionBody::Semicolon(element)
}

#[derive(Debug, Clone)]
pub enum ConstructorAttribute {
    ModifierInvocation(ModifierInvocation),
    InternalKeyword(InternalKeyword),
    OverrideKeyword(OverrideKeyword),
    PayableKeyword(PayableKeyword),
    PublicKeyword(PublicKeyword),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_constructor_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> ConstructorAttribute {
    ConstructorAttribute::ModifierInvocation(element)
}

pub fn new_constructor_attribute_internal_keyword(
    element: InternalKeyword,
) -> ConstructorAttribute {
    ConstructorAttribute::InternalKeyword(element)
}

pub fn new_constructor_attribute_override_keyword(
    element: OverrideKeyword,
) -> ConstructorAttribute {
    ConstructorAttribute::OverrideKeyword(element)
}

pub fn new_constructor_attribute_payable_keyword(element: PayableKeyword) -> ConstructorAttribute {
    ConstructorAttribute::PayableKeyword(element)
}

pub fn new_constructor_attribute_public_keyword(element: PublicKeyword) -> ConstructorAttribute {
    ConstructorAttribute::PublicKeyword(element)
}

pub fn new_constructor_attribute_virtual_keyword(element: VirtualKeyword) -> ConstructorAttribute {
    ConstructorAttribute::VirtualKeyword(element)
}

#[derive(Debug, Clone)]
pub enum UnnamedFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    ConstantKeyword(ConstantKeyword),
    ExternalKeyword(ExternalKeyword),
    InternalKeyword(InternalKeyword),
    PayableKeyword(PayableKeyword),
    PrivateKeyword(PrivateKeyword),
    PublicKeyword(PublicKeyword),
    PureKeyword(PureKeyword),
    ViewKeyword(ViewKeyword),
}

pub fn new_unnamed_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::ModifierInvocation(element)
}

pub fn new_unnamed_function_attribute_constant_keyword(
    element: ConstantKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::ConstantKeyword(element)
}

pub fn new_unnamed_function_attribute_external_keyword(
    element: ExternalKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::ExternalKeyword(element)
}

pub fn new_unnamed_function_attribute_internal_keyword(
    element: InternalKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::InternalKeyword(element)
}

pub fn new_unnamed_function_attribute_payable_keyword(
    element: PayableKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::PayableKeyword(element)
}

pub fn new_unnamed_function_attribute_private_keyword(
    element: PrivateKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::PrivateKeyword(element)
}

pub fn new_unnamed_function_attribute_public_keyword(
    element: PublicKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::PublicKeyword(element)
}

pub fn new_unnamed_function_attribute_pure_keyword(
    element: PureKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::PureKeyword(element)
}

pub fn new_unnamed_function_attribute_view_keyword(
    element: ViewKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::ViewKeyword(element)
}

#[derive(Debug, Clone)]
pub enum FallbackFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword(ExternalKeyword),
    PayableKeyword(PayableKeyword),
    PureKeyword(PureKeyword),
    ViewKeyword(ViewKeyword),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_fallback_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::ModifierInvocation(element)
}

pub fn new_fallback_function_attribute_override_specifier(
    element: OverrideSpecifier,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::OverrideSpecifier(element)
}

pub fn new_fallback_function_attribute_external_keyword(
    element: ExternalKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::ExternalKeyword(element)
}

pub fn new_fallback_function_attribute_payable_keyword(
    element: PayableKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::PayableKeyword(element)
}

pub fn new_fallback_function_attribute_pure_keyword(
    element: PureKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::PureKeyword(element)
}

pub fn new_fallback_function_attribute_view_keyword(
    element: ViewKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::ViewKeyword(element)
}

pub fn new_fallback_function_attribute_virtual_keyword(
    element: VirtualKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::VirtualKeyword(element)
}

#[derive(Debug, Clone)]
pub enum ReceiveFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword(ExternalKeyword),
    PayableKeyword(PayableKeyword),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_receive_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::ModifierInvocation(element)
}

pub fn new_receive_function_attribute_override_specifier(
    element: OverrideSpecifier,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::OverrideSpecifier(element)
}

pub fn new_receive_function_attribute_external_keyword(
    element: ExternalKeyword,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::ExternalKeyword(element)
}

pub fn new_receive_function_attribute_payable_keyword(
    element: PayableKeyword,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::PayableKeyword(element)
}

pub fn new_receive_function_attribute_virtual_keyword(
    element: VirtualKeyword,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::VirtualKeyword(element)
}

#[derive(Debug, Clone)]
pub enum ModifierAttribute {
    OverrideSpecifier(OverrideSpecifier),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_modifier_attribute_override_specifier(element: OverrideSpecifier) -> ModifierAttribute {
    ModifierAttribute::OverrideSpecifier(element)
}

pub fn new_modifier_attribute_virtual_keyword(element: VirtualKeyword) -> ModifierAttribute {
    ModifierAttribute::VirtualKeyword(element)
}

#[derive(Debug, Clone)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

pub fn new_type_name_array_type_name(element: ArrayTypeName) -> TypeName {
    TypeName::ArrayTypeName(element)
}

pub fn new_type_name_function_type(element: FunctionType) -> TypeName {
    TypeName::FunctionType(element)
}

pub fn new_type_name_mapping_type(element: MappingType) -> TypeName {
    TypeName::MappingType(element)
}

pub fn new_type_name_elementary_type(element: ElementaryType) -> TypeName {
    TypeName::ElementaryType(element)
}

pub fn new_type_name_identifier_path(element: IdentifierPath) -> TypeName {
    TypeName::IdentifierPath(element)
}

#[derive(Debug, Clone)]
pub enum FunctionTypeAttribute {
    InternalKeyword(InternalKeyword),
    ExternalKeyword(ExternalKeyword),
    PrivateKeyword(PrivateKeyword),
    PublicKeyword(PublicKeyword),
    ConstantKeyword(ConstantKeyword),
    PureKeyword(PureKeyword),
    ViewKeyword(ViewKeyword),
    PayableKeyword(PayableKeyword),
}

pub fn new_function_type_attribute_internal_keyword(
    element: InternalKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::InternalKeyword(element)
}

pub fn new_function_type_attribute_external_keyword(
    element: ExternalKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::ExternalKeyword(element)
}

pub fn new_function_type_attribute_private_keyword(
    element: PrivateKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::PrivateKeyword(element)
}

pub fn new_function_type_attribute_public_keyword(element: PublicKeyword) -> FunctionTypeAttribute {
    FunctionTypeAttribute::PublicKeyword(element)
}

pub fn new_function_type_attribute_constant_keyword(
    element: ConstantKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::ConstantKeyword(element)
}

pub fn new_function_type_attribute_pure_keyword(element: PureKeyword) -> FunctionTypeAttribute {
    FunctionTypeAttribute::PureKeyword(element)
}

pub fn new_function_type_attribute_view_keyword(element: ViewKeyword) -> FunctionTypeAttribute {
    FunctionTypeAttribute::ViewKeyword(element)
}

pub fn new_function_type_attribute_payable_keyword(
    element: PayableKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::PayableKeyword(element)
}

#[derive(Debug, Clone)]
pub enum MappingKeyType {
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

pub fn new_mapping_key_type_elementary_type(element: ElementaryType) -> MappingKeyType {
    MappingKeyType::ElementaryType(element)
}

pub fn new_mapping_key_type_identifier_path(element: IdentifierPath) -> MappingKeyType {
    MappingKeyType::IdentifierPath(element)
}

#[derive(Debug, Clone)]
pub enum ElementaryType {
    BoolKeyword(BoolKeyword),
    ByteKeyword(ByteKeyword),
    StringKeyword(StringKeyword),
    AddressType(AddressType),
    BytesKeyword(BytesKeyword),
    IntKeyword(IntKeyword),
    UintKeyword(UintKeyword),
    FixedKeyword(FixedKeyword),
    UfixedKeyword(UfixedKeyword),
}

pub fn new_elementary_type_bool_keyword(element: BoolKeyword) -> ElementaryType {
    ElementaryType::BoolKeyword(element)
}

pub fn new_elementary_type_byte_keyword(element: ByteKeyword) -> ElementaryType {
    ElementaryType::ByteKeyword(element)
}

pub fn new_elementary_type_string_keyword(element: StringKeyword) -> ElementaryType {
    ElementaryType::StringKeyword(element)
}

pub fn new_elementary_type_address_type(element: AddressType) -> ElementaryType {
    ElementaryType::AddressType(element)
}

pub fn new_elementary_type_bytes_keyword(element: BytesKeyword) -> ElementaryType {
    ElementaryType::BytesKeyword(element)
}

pub fn new_elementary_type_int_keyword(element: IntKeyword) -> ElementaryType {
    ElementaryType::IntKeyword(element)
}

pub fn new_elementary_type_uint_keyword(element: UintKeyword) -> ElementaryType {
    ElementaryType::UintKeyword(element)
}

pub fn new_elementary_type_fixed_keyword(element: FixedKeyword) -> ElementaryType {
    ElementaryType::FixedKeyword(element)
}

pub fn new_elementary_type_ufixed_keyword(element: UfixedKeyword) -> ElementaryType {
    ElementaryType::UfixedKeyword(element)
}

#[derive(Debug, Clone)]
pub enum Statement {
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    ThrowStatement(ThrowStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    RevertStatement(RevertStatement),
    AssemblyStatement(AssemblyStatement),
    Block(Block),
    UncheckedBlock(UncheckedBlock),
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
}

pub fn new_statement_if_statement(element: IfStatement) -> Statement {
    Statement::IfStatement(element)
}

pub fn new_statement_for_statement(element: ForStatement) -> Statement {
    Statement::ForStatement(element)
}

pub fn new_statement_while_statement(element: WhileStatement) -> Statement {
    Statement::WhileStatement(element)
}

pub fn new_statement_do_while_statement(element: DoWhileStatement) -> Statement {
    Statement::DoWhileStatement(element)
}

pub fn new_statement_continue_statement(element: ContinueStatement) -> Statement {
    Statement::ContinueStatement(element)
}

pub fn new_statement_break_statement(element: BreakStatement) -> Statement {
    Statement::BreakStatement(element)
}

pub fn new_statement_return_statement(element: ReturnStatement) -> Statement {
    Statement::ReturnStatement(element)
}

pub fn new_statement_throw_statement(element: ThrowStatement) -> Statement {
    Statement::ThrowStatement(element)
}

pub fn new_statement_emit_statement(element: EmitStatement) -> Statement {
    Statement::EmitStatement(element)
}

pub fn new_statement_try_statement(element: TryStatement) -> Statement {
    Statement::TryStatement(element)
}

pub fn new_statement_revert_statement(element: RevertStatement) -> Statement {
    Statement::RevertStatement(element)
}

pub fn new_statement_assembly_statement(element: AssemblyStatement) -> Statement {
    Statement::AssemblyStatement(element)
}

pub fn new_statement_block(element: Block) -> Statement {
    Statement::Block(element)
}

pub fn new_statement_unchecked_block(element: UncheckedBlock) -> Statement {
    Statement::UncheckedBlock(element)
}

pub fn new_statement_tuple_deconstruction_statement(
    element: TupleDeconstructionStatement,
) -> Statement {
    Statement::TupleDeconstructionStatement(element)
}

pub fn new_statement_variable_declaration_statement(
    element: VariableDeclarationStatement,
) -> Statement {
    Statement::VariableDeclarationStatement(element)
}

pub fn new_statement_expression_statement(element: ExpressionStatement) -> Statement {
    Statement::ExpressionStatement(element)
}

#[derive(Debug, Clone)]
pub enum TupleMember {
    TypedTupleMember(TypedTupleMember),
    UntypedTupleMember(UntypedTupleMember),
}

pub fn new_tuple_member_typed_tuple_member(element: TypedTupleMember) -> TupleMember {
    TupleMember::TypedTupleMember(element)
}

pub fn new_tuple_member_untyped_tuple_member(element: UntypedTupleMember) -> TupleMember {
    TupleMember::UntypedTupleMember(element)
}

#[derive(Debug, Clone)]
pub enum VariableDeclarationType {
    TypeName(TypeName),
    VarKeyword(VarKeyword),
}

pub fn new_variable_declaration_type_type_name(element: TypeName) -> VariableDeclarationType {
    VariableDeclarationType::TypeName(element)
}

pub fn new_variable_declaration_type_var_keyword(element: VarKeyword) -> VariableDeclarationType {
    VariableDeclarationType::VarKeyword(element)
}

#[derive(Debug, Clone)]
pub enum StorageLocation {
    MemoryKeyword(MemoryKeyword),
    StorageKeyword(StorageKeyword),
    CallDataKeyword(CallDataKeyword),
}

pub fn new_storage_location_memory_keyword(element: MemoryKeyword) -> StorageLocation {
    StorageLocation::MemoryKeyword(element)
}

pub fn new_storage_location_storage_keyword(element: StorageKeyword) -> StorageLocation {
    StorageLocation::StorageKeyword(element)
}

pub fn new_storage_location_call_data_keyword(element: CallDataKeyword) -> StorageLocation {
    StorageLocation::CallDataKeyword(element)
}

#[derive(Debug, Clone)]
pub enum ForStatementInitialization {
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

pub fn new_for_statement_initialization_tuple_deconstruction_statement(
    element: TupleDeconstructionStatement,
) -> ForStatementInitialization {
    ForStatementInitialization::TupleDeconstructionStatement(element)
}

pub fn new_for_statement_initialization_variable_declaration_statement(
    element: VariableDeclarationStatement,
) -> ForStatementInitialization {
    ForStatementInitialization::VariableDeclarationStatement(element)
}

pub fn new_for_statement_initialization_expression_statement(
    element: ExpressionStatement,
) -> ForStatementInitialization {
    ForStatementInitialization::ExpressionStatement(element)
}

pub fn new_for_statement_initialization_semicolon(
    element: Semicolon,
) -> ForStatementInitialization {
    ForStatementInitialization::Semicolon(element)
}

#[derive(Debug, Clone)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

pub fn new_for_statement_condition_expression_statement(
    element: ExpressionStatement,
) -> ForStatementCondition {
    ForStatementCondition::ExpressionStatement(element)
}

pub fn new_for_statement_condition_semicolon(element: Semicolon) -> ForStatementCondition {
    ForStatementCondition::Semicolon(element)
}

#[derive(Debug, Clone)]
pub enum Expression {
    AssignmentExpression(AssignmentExpression),
    ConditionalExpression(ConditionalExpression),
    OrExpression(OrExpression),
    AndExpression(AndExpression),
    EqualityExpression(EqualityExpression),
    InequalityExpression(InequalityExpression),
    BitwiseOrExpression(BitwiseOrExpression),
    BitwiseXorExpression(BitwiseXorExpression),
    BitwiseAndExpression(BitwiseAndExpression),
    ShiftExpression(ShiftExpression),
    AdditiveExpression(AdditiveExpression),
    MultiplicativeExpression(MultiplicativeExpression),
    ExponentiationExpression(ExponentiationExpression),
    PostfixExpression(PostfixExpression),
    PrefixExpression(PrefixExpression),
    FunctionCallExpression(FunctionCallExpression),
    CallOptionsExpression(CallOptionsExpression),
    MemberAccessExpression(MemberAccessExpression),
    IndexAccessExpression(IndexAccessExpression),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    TypeExpression(TypeExpression),
    ArrayExpression(ArrayExpression),
    HexNumberExpression(HexNumberExpression),
    DecimalNumberExpression(DecimalNumberExpression),
    StringExpression(StringExpression),
    ElementaryType(ElementaryType),
    PayableKeyword(PayableKeyword),
    ThisKeyword(ThisKeyword),
    SuperKeyword(SuperKeyword),
    TrueKeyword(TrueKeyword),
    FalseKeyword(FalseKeyword),
    Identifier(Identifier),
}

pub fn new_expression_assignment_expression(element: AssignmentExpression) -> Expression {
    Expression::AssignmentExpression(element)
}

pub fn new_expression_conditional_expression(element: ConditionalExpression) -> Expression {
    Expression::ConditionalExpression(element)
}

pub fn new_expression_or_expression(element: OrExpression) -> Expression {
    Expression::OrExpression(element)
}

pub fn new_expression_and_expression(element: AndExpression) -> Expression {
    Expression::AndExpression(element)
}

pub fn new_expression_equality_expression(element: EqualityExpression) -> Expression {
    Expression::EqualityExpression(element)
}

pub fn new_expression_inequality_expression(element: InequalityExpression) -> Expression {
    Expression::InequalityExpression(element)
}

pub fn new_expression_bitwise_or_expression(element: BitwiseOrExpression) -> Expression {
    Expression::BitwiseOrExpression(element)
}

pub fn new_expression_bitwise_xor_expression(element: BitwiseXorExpression) -> Expression {
    Expression::BitwiseXorExpression(element)
}

pub fn new_expression_bitwise_and_expression(element: BitwiseAndExpression) -> Expression {
    Expression::BitwiseAndExpression(element)
}

pub fn new_expression_shift_expression(element: ShiftExpression) -> Expression {
    Expression::ShiftExpression(element)
}

pub fn new_expression_additive_expression(element: AdditiveExpression) -> Expression {
    Expression::AdditiveExpression(element)
}

pub fn new_expression_multiplicative_expression(element: MultiplicativeExpression) -> Expression {
    Expression::MultiplicativeExpression(element)
}

pub fn new_expression_exponentiation_expression(element: ExponentiationExpression) -> Expression {
    Expression::ExponentiationExpression(element)
}

pub fn new_expression_postfix_expression(element: PostfixExpression) -> Expression {
    Expression::PostfixExpression(element)
}

pub fn new_expression_prefix_expression(element: PrefixExpression) -> Expression {
    Expression::PrefixExpression(element)
}

pub fn new_expression_function_call_expression(element: FunctionCallExpression) -> Expression {
    Expression::FunctionCallExpression(element)
}

pub fn new_expression_call_options_expression(element: CallOptionsExpression) -> Expression {
    Expression::CallOptionsExpression(element)
}

pub fn new_expression_member_access_expression(element: MemberAccessExpression) -> Expression {
    Expression::MemberAccessExpression(element)
}

pub fn new_expression_index_access_expression(element: IndexAccessExpression) -> Expression {
    Expression::IndexAccessExpression(element)
}

pub fn new_expression_new_expression(element: NewExpression) -> Expression {
    Expression::NewExpression(element)
}

pub fn new_expression_tuple_expression(element: TupleExpression) -> Expression {
    Expression::TupleExpression(element)
}

pub fn new_expression_type_expression(element: TypeExpression) -> Expression {
    Expression::TypeExpression(element)
}

pub fn new_expression_array_expression(element: ArrayExpression) -> Expression {
    Expression::ArrayExpression(element)
}

pub fn new_expression_hex_number_expression(element: HexNumberExpression) -> Expression {
    Expression::HexNumberExpression(element)
}

pub fn new_expression_decimal_number_expression(element: DecimalNumberExpression) -> Expression {
    Expression::DecimalNumberExpression(element)
}

pub fn new_expression_string_expression(element: StringExpression) -> Expression {
    Expression::StringExpression(element)
}

pub fn new_expression_elementary_type(element: ElementaryType) -> Expression {
    Expression::ElementaryType(element)
}

pub fn new_expression_payable_keyword(element: PayableKeyword) -> Expression {
    Expression::PayableKeyword(element)
}

pub fn new_expression_this_keyword(element: ThisKeyword) -> Expression {
    Expression::ThisKeyword(element)
}

pub fn new_expression_super_keyword(element: SuperKeyword) -> Expression {
    Expression::SuperKeyword(element)
}

pub fn new_expression_true_keyword(element: TrueKeyword) -> Expression {
    Expression::TrueKeyword(element)
}

pub fn new_expression_false_keyword(element: FalseKeyword) -> Expression {
    Expression::FalseKeyword(element)
}

pub fn new_expression_identifier(element: Identifier) -> Expression {
    Expression::Identifier(element)
}

#[derive(Debug, Clone)]
pub enum ArgumentsDeclaration {
    PositionalArgumentsDeclaration(PositionalArgumentsDeclaration),
    NamedArgumentsDeclaration(NamedArgumentsDeclaration),
}

pub fn new_arguments_declaration_positional_arguments_declaration(
    element: PositionalArgumentsDeclaration,
) -> ArgumentsDeclaration {
    ArgumentsDeclaration::PositionalArgumentsDeclaration(element)
}

pub fn new_arguments_declaration_named_arguments_declaration(
    element: NamedArgumentsDeclaration,
) -> ArgumentsDeclaration {
    ArgumentsDeclaration::NamedArgumentsDeclaration(element)
}

#[derive(Debug, Clone)]
pub enum NumberUnit {
    WeiKeyword(WeiKeyword),
    GweiKeyword(GweiKeyword),
    SzaboKeyword(SzaboKeyword),
    FinneyKeyword(FinneyKeyword),
    EtherKeyword(EtherKeyword),
    SecondsKeyword(SecondsKeyword),
    MinutesKeyword(MinutesKeyword),
    HoursKeyword(HoursKeyword),
    DaysKeyword(DaysKeyword),
    WeeksKeyword(WeeksKeyword),
    YearsKeyword(YearsKeyword),
}

pub fn new_number_unit_wei_keyword(element: WeiKeyword) -> NumberUnit {
    NumberUnit::WeiKeyword(element)
}

pub fn new_number_unit_gwei_keyword(element: GweiKeyword) -> NumberUnit {
    NumberUnit::GweiKeyword(element)
}

pub fn new_number_unit_szabo_keyword(element: SzaboKeyword) -> NumberUnit {
    NumberUnit::SzaboKeyword(element)
}

pub fn new_number_unit_finney_keyword(element: FinneyKeyword) -> NumberUnit {
    NumberUnit::FinneyKeyword(element)
}

pub fn new_number_unit_ether_keyword(element: EtherKeyword) -> NumberUnit {
    NumberUnit::EtherKeyword(element)
}

pub fn new_number_unit_seconds_keyword(element: SecondsKeyword) -> NumberUnit {
    NumberUnit::SecondsKeyword(element)
}

pub fn new_number_unit_minutes_keyword(element: MinutesKeyword) -> NumberUnit {
    NumberUnit::MinutesKeyword(element)
}

pub fn new_number_unit_hours_keyword(element: HoursKeyword) -> NumberUnit {
    NumberUnit::HoursKeyword(element)
}

pub fn new_number_unit_days_keyword(element: DaysKeyword) -> NumberUnit {
    NumberUnit::DaysKeyword(element)
}

pub fn new_number_unit_weeks_keyword(element: WeeksKeyword) -> NumberUnit {
    NumberUnit::WeeksKeyword(element)
}

pub fn new_number_unit_years_keyword(element: YearsKeyword) -> NumberUnit {
    NumberUnit::YearsKeyword(element)
}

#[derive(Debug, Clone)]
pub enum StringExpression {
    StringLiteral(StringLiteral),
    StringLiterals(StringLiterals),
    HexStringLiteral(HexStringLiteral),
    HexStringLiterals(HexStringLiterals),
    UnicodeStringLiterals(UnicodeStringLiterals),
}

pub fn new_string_expression_string_literal(element: StringLiteral) -> StringExpression {
    StringExpression::StringLiteral(element)
}

pub fn new_string_expression_string_literals(element: StringLiterals) -> StringExpression {
    StringExpression::StringLiterals(element)
}

pub fn new_string_expression_hex_string_literal(element: HexStringLiteral) -> StringExpression {
    StringExpression::HexStringLiteral(element)
}

pub fn new_string_expression_hex_string_literals(element: HexStringLiterals) -> StringExpression {
    StringExpression::HexStringLiterals(element)
}

pub fn new_string_expression_unicode_string_literals(
    element: UnicodeStringLiterals,
) -> StringExpression {
    StringExpression::UnicodeStringLiterals(element)
}

#[derive(Debug, Clone)]
pub enum StringLiteral {
    SingleQuotedStringLiteral(SingleQuotedStringLiteral),
    DoubleQuotedStringLiteral(DoubleQuotedStringLiteral),
}

pub fn new_string_literal_single_quoted_string_literal(
    element: SingleQuotedStringLiteral,
) -> StringLiteral {
    StringLiteral::SingleQuotedStringLiteral(element)
}

pub fn new_string_literal_double_quoted_string_literal(
    element: DoubleQuotedStringLiteral,
) -> StringLiteral {
    StringLiteral::DoubleQuotedStringLiteral(element)
}

#[derive(Debug, Clone)]
pub enum HexStringLiteral {
    SingleQuotedHexStringLiteral(SingleQuotedHexStringLiteral),
    DoubleQuotedHexStringLiteral(DoubleQuotedHexStringLiteral),
}

pub fn new_hex_string_literal_single_quoted_hex_string_literal(
    element: SingleQuotedHexStringLiteral,
) -> HexStringLiteral {
    HexStringLiteral::SingleQuotedHexStringLiteral(element)
}

pub fn new_hex_string_literal_double_quoted_hex_string_literal(
    element: DoubleQuotedHexStringLiteral,
) -> HexStringLiteral {
    HexStringLiteral::DoubleQuotedHexStringLiteral(element)
}

#[derive(Debug, Clone)]
pub enum UnicodeStringLiteral {
    SingleQuotedUnicodeStringLiteral(SingleQuotedUnicodeStringLiteral),
    DoubleQuotedUnicodeStringLiteral(DoubleQuotedUnicodeStringLiteral),
}

pub fn new_unicode_string_literal_single_quoted_unicode_string_literal(
    element: SingleQuotedUnicodeStringLiteral,
) -> UnicodeStringLiteral {
    UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(element)
}

pub fn new_unicode_string_literal_double_quoted_unicode_string_literal(
    element: DoubleQuotedUnicodeStringLiteral,
) -> UnicodeStringLiteral {
    UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(element)
}

#[derive(Debug, Clone)]
pub enum YulStatement {
    YulBlock(YulBlock),
    YulFunctionDefinition(YulFunctionDefinition),
    YulStackAssignmentStatement(YulStackAssignmentStatement),
    YulIfStatement(YulIfStatement),
    YulForStatement(YulForStatement),
    YulSwitchStatement(YulSwitchStatement),
    YulLeaveStatement(YulLeaveStatement),
    YulBreakStatement(YulBreakStatement),
    YulContinueStatement(YulContinueStatement),
    YulVariableAssignmentStatement(YulVariableAssignmentStatement),
    YulLabel(YulLabel),
    YulVariableDeclarationStatement(YulVariableDeclarationStatement),
    YulExpression(YulExpression),
}

pub fn new_yul_statement_yul_block(element: YulBlock) -> YulStatement {
    YulStatement::YulBlock(element)
}

pub fn new_yul_statement_yul_function_definition(element: YulFunctionDefinition) -> YulStatement {
    YulStatement::YulFunctionDefinition(element)
}

pub fn new_yul_statement_yul_stack_assignment_statement(
    element: YulStackAssignmentStatement,
) -> YulStatement {
    YulStatement::YulStackAssignmentStatement(element)
}

pub fn new_yul_statement_yul_if_statement(element: YulIfStatement) -> YulStatement {
    YulStatement::YulIfStatement(element)
}

pub fn new_yul_statement_yul_for_statement(element: YulForStatement) -> YulStatement {
    YulStatement::YulForStatement(element)
}

pub fn new_yul_statement_yul_switch_statement(element: YulSwitchStatement) -> YulStatement {
    YulStatement::YulSwitchStatement(element)
}

pub fn new_yul_statement_yul_leave_statement(element: YulLeaveStatement) -> YulStatement {
    YulStatement::YulLeaveStatement(element)
}

pub fn new_yul_statement_yul_break_statement(element: YulBreakStatement) -> YulStatement {
    YulStatement::YulBreakStatement(element)
}

pub fn new_yul_statement_yul_continue_statement(element: YulContinueStatement) -> YulStatement {
    YulStatement::YulContinueStatement(element)
}

pub fn new_yul_statement_yul_variable_assignment_statement(
    element: YulVariableAssignmentStatement,
) -> YulStatement {
    YulStatement::YulVariableAssignmentStatement(element)
}

pub fn new_yul_statement_yul_label(element: YulLabel) -> YulStatement {
    YulStatement::YulLabel(element)
}

pub fn new_yul_statement_yul_variable_declaration_statement(
    element: YulVariableDeclarationStatement,
) -> YulStatement {
    YulStatement::YulVariableDeclarationStatement(element)
}

pub fn new_yul_statement_yul_expression(element: YulExpression) -> YulStatement {
    YulStatement::YulExpression(element)
}

#[derive(Debug, Clone)]
pub enum YulAssignmentOperator {
    ColonEqual(ColonEqual),
    YulColonAndEqual(YulColonAndEqual),
}

pub fn new_yul_assignment_operator_colon_equal(element: ColonEqual) -> YulAssignmentOperator {
    YulAssignmentOperator::ColonEqual(element)
}

pub fn new_yul_assignment_operator_yul_colon_and_equal(
    element: YulColonAndEqual,
) -> YulAssignmentOperator {
    YulAssignmentOperator::YulColonAndEqual(element)
}

#[derive(Debug, Clone)]
pub enum YulStackAssignmentOperator {
    EqualColon(EqualColon),
    YulEqualAndColon(YulEqualAndColon),
}

pub fn new_yul_stack_assignment_operator_equal_colon(
    element: EqualColon,
) -> YulStackAssignmentOperator {
    YulStackAssignmentOperator::EqualColon(element)
}

pub fn new_yul_stack_assignment_operator_yul_equal_and_colon(
    element: YulEqualAndColon,
) -> YulStackAssignmentOperator {
    YulStackAssignmentOperator::YulEqualAndColon(element)
}

#[derive(Debug, Clone)]
pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

pub fn new_yul_switch_case_yul_default_case(element: YulDefaultCase) -> YulSwitchCase {
    YulSwitchCase::YulDefaultCase(element)
}

pub fn new_yul_switch_case_yul_value_case(element: YulValueCase) -> YulSwitchCase {
    YulSwitchCase::YulValueCase(element)
}

#[derive(Debug, Clone)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

pub fn new_yul_expression_yul_function_call_expression(
    element: YulFunctionCallExpression,
) -> YulExpression {
    YulExpression::YulFunctionCallExpression(element)
}

pub fn new_yul_expression_yul_literal(element: YulLiteral) -> YulExpression {
    YulExpression::YulLiteral(element)
}

pub fn new_yul_expression_yul_path(element: YulPath) -> YulExpression {
    YulExpression::YulPath(element)
}

#[derive(Debug, Clone)]
pub enum YulLiteral {
    YulTrueKeyword(YulTrueKeyword),
    YulFalseKeyword(YulFalseKeyword),
    YulDecimalLiteral(YulDecimalLiteral),
    YulHexLiteral(YulHexLiteral),
    HexStringLiteral(HexStringLiteral),
    StringLiteral(StringLiteral),
}

pub fn new_yul_literal_yul_true_keyword(element: YulTrueKeyword) -> YulLiteral {
    YulLiteral::YulTrueKeyword(element)
}

pub fn new_yul_literal_yul_false_keyword(element: YulFalseKeyword) -> YulLiteral {
    YulLiteral::YulFalseKeyword(element)
}

pub fn new_yul_literal_yul_decimal_literal(element: YulDecimalLiteral) -> YulLiteral {
    YulLiteral::YulDecimalLiteral(element)
}

pub fn new_yul_literal_yul_hex_literal(element: YulHexLiteral) -> YulLiteral {
    YulLiteral::YulHexLiteral(element)
}

pub fn new_yul_literal_hex_string_literal(element: HexStringLiteral) -> YulLiteral {
    YulLiteral::HexStringLiteral(element)
}

pub fn new_yul_literal_string_literal(element: StringLiteral) -> YulLiteral {
    YulLiteral::StringLiteral(element)
}

//
// Repeated & Separated
//

#[derive(Debug, Clone)]
pub struct SourceUnitMembers {
    pub elements: Vec<SourceUnitMember>,
}

pub fn new_source_unit_members(elements: Vec<SourceUnitMember>) -> SourceUnitMembers {
    SourceUnitMembers { elements }
}

#[derive(Debug, Clone)]
pub struct VersionExpressionSets {
    pub elements: Vec<VersionExpressionSet>,
}

pub fn new_version_expression_sets(elements: Vec<VersionExpressionSet>) -> VersionExpressionSets {
    VersionExpressionSets { elements }
}

#[derive(Debug, Clone)]
pub struct VersionExpressionSet {
    pub elements: Vec<VersionExpression>,
}

pub fn new_version_expression_set(elements: Vec<VersionExpression>) -> VersionExpressionSet {
    VersionExpressionSet { elements }
}

#[derive(Debug, Clone)]
pub struct SimpleVersionLiteral {
    pub elements: Vec<VersionSpecifier>,
}

pub fn new_simple_version_literal(elements: Vec<VersionSpecifier>) -> SimpleVersionLiteral {
    SimpleVersionLiteral { elements }
}

#[derive(Debug, Clone)]
pub struct ImportDeconstructionSymbols {
    pub elements: Vec<ImportDeconstructionSymbol>,
}

pub fn new_import_deconstruction_symbols(
    elements: Vec<ImportDeconstructionSymbol>,
) -> ImportDeconstructionSymbols {
    ImportDeconstructionSymbols { elements }
}

#[derive(Debug, Clone)]
pub struct UsingDeconstructionSymbols {
    pub elements: Vec<UsingDeconstructionSymbol>,
}

pub fn new_using_deconstruction_symbols(
    elements: Vec<UsingDeconstructionSymbol>,
) -> UsingDeconstructionSymbols {
    UsingDeconstructionSymbols { elements }
}

#[derive(Debug, Clone)]
pub struct ContractSpecifiers {
    pub elements: Vec<ContractSpecifier>,
}

pub fn new_contract_specifiers(elements: Vec<ContractSpecifier>) -> ContractSpecifiers {
    ContractSpecifiers { elements }
}

#[derive(Debug, Clone)]
pub struct InheritanceTypes {
    pub elements: Vec<InheritanceType>,
}

pub fn new_inheritance_types(elements: Vec<InheritanceType>) -> InheritanceTypes {
    InheritanceTypes { elements }
}

#[derive(Debug, Clone)]
pub struct ContractMembers {
    pub elements: Vec<ContractMember>,
}

pub fn new_contract_members(elements: Vec<ContractMember>) -> ContractMembers {
    ContractMembers { elements }
}

#[derive(Debug, Clone)]
pub struct InterfaceMembers {
    pub elements: Vec<ContractMember>,
}

pub fn new_interface_members(elements: Vec<ContractMember>) -> InterfaceMembers {
    InterfaceMembers { elements }
}

#[derive(Debug, Clone)]
pub struct LibraryMembers {
    pub elements: Vec<ContractMember>,
}

pub fn new_library_members(elements: Vec<ContractMember>) -> LibraryMembers {
    LibraryMembers { elements }
}

#[derive(Debug, Clone)]
pub struct StructMembers {
    pub elements: Vec<StructMember>,
}

pub fn new_struct_members(elements: Vec<StructMember>) -> StructMembers {
    StructMembers { elements }
}

#[derive(Debug, Clone)]
pub struct EnumMembers {
    pub elements: Vec<Identifier>,
}

pub fn new_enum_members(elements: Vec<Identifier>) -> EnumMembers {
    EnumMembers { elements }
}

#[derive(Debug, Clone)]
pub struct StateVariableAttributes {
    pub elements: Vec<StateVariableAttribute>,
}

pub fn new_state_variable_attributes(
    elements: Vec<StateVariableAttribute>,
) -> StateVariableAttributes {
    StateVariableAttributes { elements }
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub elements: Vec<Parameter>,
}

pub fn new_parameters(elements: Vec<Parameter>) -> Parameters {
    Parameters { elements }
}

#[derive(Debug, Clone)]
pub struct FunctionAttributes {
    pub elements: Vec<FunctionAttribute>,
}

pub fn new_function_attributes(elements: Vec<FunctionAttribute>) -> FunctionAttributes {
    FunctionAttributes { elements }
}

#[derive(Debug, Clone)]
pub struct OverridePaths {
    pub elements: Vec<IdentifierPath>,
}

pub fn new_override_paths(elements: Vec<IdentifierPath>) -> OverridePaths {
    OverridePaths { elements }
}

#[derive(Debug, Clone)]
pub struct ConstructorAttributes {
    pub elements: Vec<ConstructorAttribute>,
}

pub fn new_constructor_attributes(elements: Vec<ConstructorAttribute>) -> ConstructorAttributes {
    ConstructorAttributes { elements }
}

#[derive(Debug, Clone)]
pub struct UnnamedFunctionAttributes {
    pub elements: Vec<UnnamedFunctionAttribute>,
}

pub fn new_unnamed_function_attributes(
    elements: Vec<UnnamedFunctionAttribute>,
) -> UnnamedFunctionAttributes {
    UnnamedFunctionAttributes { elements }
}

#[derive(Debug, Clone)]
pub struct FallbackFunctionAttributes {
    pub elements: Vec<FallbackFunctionAttribute>,
}

pub fn new_fallback_function_attributes(
    elements: Vec<FallbackFunctionAttribute>,
) -> FallbackFunctionAttributes {
    FallbackFunctionAttributes { elements }
}

#[derive(Debug, Clone)]
pub struct ReceiveFunctionAttributes {
    pub elements: Vec<ReceiveFunctionAttribute>,
}

pub fn new_receive_function_attributes(
    elements: Vec<ReceiveFunctionAttribute>,
) -> ReceiveFunctionAttributes {
    ReceiveFunctionAttributes { elements }
}

#[derive(Debug, Clone)]
pub struct ModifierAttributes {
    pub elements: Vec<ModifierAttribute>,
}

pub fn new_modifier_attributes(elements: Vec<ModifierAttribute>) -> ModifierAttributes {
    ModifierAttributes { elements }
}

#[derive(Debug, Clone)]
pub struct EventParameters {
    pub elements: Vec<EventParameter>,
}

pub fn new_event_parameters(elements: Vec<EventParameter>) -> EventParameters {
    EventParameters { elements }
}

#[derive(Debug, Clone)]
pub struct ErrorParameters {
    pub elements: Vec<ErrorParameter>,
}

pub fn new_error_parameters(elements: Vec<ErrorParameter>) -> ErrorParameters {
    ErrorParameters { elements }
}

#[derive(Debug, Clone)]
pub struct FunctionTypeAttributes {
    pub elements: Vec<FunctionTypeAttribute>,
}

pub fn new_function_type_attributes(
    elements: Vec<FunctionTypeAttribute>,
) -> FunctionTypeAttributes {
    FunctionTypeAttributes { elements }
}

#[derive(Debug, Clone)]
pub struct Statements {
    pub elements: Vec<Statement>,
}

pub fn new_statements(elements: Vec<Statement>) -> Statements {
    Statements { elements }
}

#[derive(Debug, Clone)]
pub struct AssemblyFlags {
    pub elements: Vec<StringLiteral>,
}

pub fn new_assembly_flags(elements: Vec<StringLiteral>) -> AssemblyFlags {
    AssemblyFlags { elements }
}

#[derive(Debug, Clone)]
pub struct TupleDeconstructionElements {
    pub elements: Vec<TupleDeconstructionElement>,
}

pub fn new_tuple_deconstruction_elements(
    elements: Vec<TupleDeconstructionElement>,
) -> TupleDeconstructionElements {
    TupleDeconstructionElements { elements }
}

#[derive(Debug, Clone)]
pub struct CatchClauses {
    pub elements: Vec<CatchClause>,
}

pub fn new_catch_clauses(elements: Vec<CatchClause>) -> CatchClauses {
    CatchClauses { elements }
}

#[derive(Debug, Clone)]
pub struct PositionalArguments {
    pub elements: Vec<Expression>,
}

pub fn new_positional_arguments(elements: Vec<Expression>) -> PositionalArguments {
    PositionalArguments { elements }
}

#[derive(Debug, Clone)]
pub struct NamedArguments {
    pub elements: Vec<NamedArgument>,
}

pub fn new_named_arguments(elements: Vec<NamedArgument>) -> NamedArguments {
    NamedArguments { elements }
}

#[derive(Debug, Clone)]
pub struct CallOptions {
    pub elements: Vec<NamedArgument>,
}

pub fn new_call_options(elements: Vec<NamedArgument>) -> CallOptions {
    CallOptions { elements }
}

#[derive(Debug, Clone)]
pub struct TupleValues {
    pub elements: Vec<TupleValue>,
}

pub fn new_tuple_values(elements: Vec<TupleValue>) -> TupleValues {
    TupleValues { elements }
}

#[derive(Debug, Clone)]
pub struct ArrayValues {
    pub elements: Vec<Expression>,
}

pub fn new_array_values(elements: Vec<Expression>) -> ArrayValues {
    ArrayValues { elements }
}

#[derive(Debug, Clone)]
pub struct StringLiterals {
    pub elements: Vec<StringLiteral>,
}

pub fn new_string_literals(elements: Vec<StringLiteral>) -> StringLiterals {
    StringLiterals { elements }
}

#[derive(Debug, Clone)]
pub struct HexStringLiterals {
    pub elements: Vec<HexStringLiteral>,
}

pub fn new_hex_string_literals(elements: Vec<HexStringLiteral>) -> HexStringLiterals {
    HexStringLiterals { elements }
}

#[derive(Debug, Clone)]
pub struct UnicodeStringLiterals {
    pub elements: Vec<UnicodeStringLiteral>,
}

pub fn new_unicode_string_literals(elements: Vec<UnicodeStringLiteral>) -> UnicodeStringLiterals {
    UnicodeStringLiterals { elements }
}

#[derive(Debug, Clone)]
pub struct IdentifierPath {
    pub elements: Vec<Identifier>,
}

pub fn new_identifier_path(elements: Vec<Identifier>) -> IdentifierPath {
    IdentifierPath { elements }
}

#[derive(Debug, Clone)]
pub struct YulStatements {
    pub elements: Vec<YulStatement>,
}

pub fn new_yul_statements(elements: Vec<YulStatement>) -> YulStatements {
    YulStatements { elements }
}

#[derive(Debug, Clone)]
pub struct YulParameters {
    pub elements: Vec<YulIdentifier>,
}

pub fn new_yul_parameters(elements: Vec<YulIdentifier>) -> YulParameters {
    YulParameters { elements }
}

#[derive(Debug, Clone)]
pub struct YulVariableNames {
    pub elements: Vec<YulIdentifier>,
}

pub fn new_yul_variable_names(elements: Vec<YulIdentifier>) -> YulVariableNames {
    YulVariableNames { elements }
}

#[derive(Debug, Clone)]
pub struct YulSwitchCases {
    pub elements: Vec<YulSwitchCase>,
}

pub fn new_yul_switch_cases(elements: Vec<YulSwitchCase>) -> YulSwitchCases {
    YulSwitchCases { elements }
}

#[derive(Debug, Clone)]
pub struct YulArguments {
    pub elements: Vec<YulExpression>,
}

pub fn new_yul_arguments(elements: Vec<YulExpression>) -> YulArguments {
    YulArguments { elements }
}

#[derive(Debug, Clone)]
pub struct YulPaths {
    pub elements: Vec<YulPath>,
}

pub fn new_yul_paths(elements: Vec<YulPath>) -> YulPaths {
    YulPaths { elements }
}

#[derive(Debug, Clone)]
pub struct YulPath {
    pub elements: Vec<YulIdentifier>,
}

pub fn new_yul_path(elements: Vec<YulIdentifier>) -> YulPath {
    YulPath { elements }
}

//
// Terminals
//
#[derive(Debug, Clone)]
pub struct TerminalType {
    pub value: String,
    pub _l: usize,
    pub _r: usize,
    pub kind: LexemeKind,
}

pub fn new_empty_terminal() -> TerminalType {
    TerminalType {
        value: "".into(),
        _l: 0,
        _r: 0,
        kind: LexemeKind::UNRECOGNIZED,
    }
}

pub type AbicoderKeyword = TerminalType;

pub type AbicoderV1Keyword = TerminalType;

pub type AbicoderV2Keyword = TerminalType;

pub type ABIEncoderV2Keyword = TerminalType;

pub type AbstractKeyword = TerminalType;

pub type AddressKeyword = TerminalType;

pub type AfterKeyword = TerminalType;

pub type AliasKeyword = TerminalType;

pub type AnonymousKeyword = TerminalType;

pub type ApplyKeyword = TerminalType;

pub type AsKeyword = TerminalType;

pub type AssemblyKeyword = TerminalType;

pub type AtKeyword = TerminalType;

pub type AutoKeyword = TerminalType;

pub type BoolKeyword = TerminalType;

pub type BreakKeyword = TerminalType;

pub type ByteKeyword = TerminalType;

pub type BytesKeyword = TerminalType;

pub type CallDataKeyword = TerminalType;

pub type CaseKeyword = TerminalType;

pub type CatchKeyword = TerminalType;

pub type ConstantKeyword = TerminalType;

pub type ConstructorKeyword = TerminalType;

pub type ContinueKeyword = TerminalType;

pub type ContractKeyword = TerminalType;

pub type CopyOfKeyword = TerminalType;

pub type DaysKeyword = TerminalType;

pub type DefaultKeyword = TerminalType;

pub type DefineKeyword = TerminalType;

pub type DeleteKeyword = TerminalType;

pub type DoKeyword = TerminalType;

pub type ElseKeyword = TerminalType;

pub type EmitKeyword = TerminalType;

pub type EnumKeyword = TerminalType;

pub type ErrorKeyword = TerminalType;

pub type EtherKeyword = TerminalType;

pub type EventKeyword = TerminalType;

pub type ExperimentalKeyword = TerminalType;

pub type ExternalKeyword = TerminalType;

pub type FallbackKeyword = TerminalType;

pub type FalseKeyword = TerminalType;

pub type FinalKeyword = TerminalType;

pub type FinneyKeyword = TerminalType;

pub type FixedKeyword = TerminalType;

pub type ForKeyword = TerminalType;

pub type FromKeyword = TerminalType;

pub type FunctionKeyword = TerminalType;

pub type GlobalKeyword = TerminalType;

pub type GweiKeyword = TerminalType;

pub type HexKeyword = TerminalType;

pub type HoursKeyword = TerminalType;

pub type IfKeyword = TerminalType;

pub type ImmutableKeyword = TerminalType;

pub type ImplementsKeyword = TerminalType;

pub type ImportKeyword = TerminalType;

pub type IndexedKeyword = TerminalType;

pub type InKeyword = TerminalType;

pub type InlineKeyword = TerminalType;

pub type InterfaceKeyword = TerminalType;

pub type InternalKeyword = TerminalType;

pub type IntKeyword = TerminalType;

pub type IsKeyword = TerminalType;

pub type LayoutKeyword = TerminalType;

pub type LetKeyword = TerminalType;

pub type LibraryKeyword = TerminalType;

pub type MacroKeyword = TerminalType;

pub type MappingKeyword = TerminalType;

pub type MatchKeyword = TerminalType;

pub type MemoryKeyword = TerminalType;

pub type MinutesKeyword = TerminalType;

pub type ModifierKeyword = TerminalType;

pub type MutableKeyword = TerminalType;

pub type NewKeyword = TerminalType;

pub type NullKeyword = TerminalType;

pub type OfKeyword = TerminalType;

pub type OverrideKeyword = TerminalType;

pub type PartialKeyword = TerminalType;

pub type PayableKeyword = TerminalType;

pub type PragmaKeyword = TerminalType;

pub type PrivateKeyword = TerminalType;

pub type PromiseKeyword = TerminalType;

pub type PublicKeyword = TerminalType;

pub type PureKeyword = TerminalType;

pub type ReceiveKeyword = TerminalType;

pub type ReferenceKeyword = TerminalType;

pub type RelocatableKeyword = TerminalType;

pub type ReturnKeyword = TerminalType;

pub type ReturnsKeyword = TerminalType;

pub type RevertKeyword = TerminalType;

pub type SealedKeyword = TerminalType;

pub type SecondsKeyword = TerminalType;

pub type SizeOfKeyword = TerminalType;

pub type SMTCheckerKeyword = TerminalType;

pub type SolidityKeyword = TerminalType;

pub type StaticKeyword = TerminalType;

pub type StorageKeyword = TerminalType;

pub type StringKeyword = TerminalType;

pub type StructKeyword = TerminalType;

pub type SuperKeyword = TerminalType;

pub type SupportsKeyword = TerminalType;

pub type SwitchKeyword = TerminalType;

pub type SzaboKeyword = TerminalType;

pub type ThisKeyword = TerminalType;

pub type ThrowKeyword = TerminalType;

pub type TransientKeyword = TerminalType;

pub type TrueKeyword = TerminalType;

pub type TryKeyword = TerminalType;

pub type TypeDefKeyword = TerminalType;

pub type TypeKeyword = TerminalType;

pub type TypeOfKeyword = TerminalType;

pub type UfixedKeyword = TerminalType;

pub type UintKeyword = TerminalType;

pub type UncheckedKeyword = TerminalType;

pub type UsingKeyword = TerminalType;

pub type VarKeyword = TerminalType;

pub type ViewKeyword = TerminalType;

pub type VirtualKeyword = TerminalType;

pub type WeeksKeyword = TerminalType;

pub type WeiKeyword = TerminalType;

pub type WhileKeyword = TerminalType;

pub type YearsKeyword = TerminalType;

pub type YulAbstractKeyword = TerminalType;

pub type YulAfterKeyword = TerminalType;

pub type YulAliasKeyword = TerminalType;

pub type YulAnonymousKeyword = TerminalType;

pub type YulApplyKeyword = TerminalType;

pub type YulAsKeyword = TerminalType;

pub type YulAssemblyKeyword = TerminalType;

pub type YulAutoKeyword = TerminalType;

pub type YulBoolKeyword = TerminalType;

pub type YulBreakKeyword = TerminalType;

pub type YulBytesKeyword = TerminalType;

pub type YulCallDataKeyword = TerminalType;

pub type YulCaseKeyword = TerminalType;

pub type YulCatchKeyword = TerminalType;

pub type YulConstantKeyword = TerminalType;

pub type YulConstructorKeyword = TerminalType;

pub type YulContinueKeyword = TerminalType;

pub type YulContractKeyword = TerminalType;

pub type YulCopyOfKeyword = TerminalType;

pub type YulDaysKeyword = TerminalType;

pub type YulDefaultKeyword = TerminalType;

pub type YulDefineKeyword = TerminalType;

pub type YulDeleteKeyword = TerminalType;

pub type YulDoKeyword = TerminalType;

pub type YulElseKeyword = TerminalType;

pub type YulEmitKeyword = TerminalType;

pub type YulEnumKeyword = TerminalType;

pub type YulEtherKeyword = TerminalType;

pub type YulEventKeyword = TerminalType;

pub type YulExternalKeyword = TerminalType;

pub type YulFallbackKeyword = TerminalType;

pub type YulFalseKeyword = TerminalType;

pub type YulFinalKeyword = TerminalType;

pub type YulFinneyKeyword = TerminalType;

pub type YulFixedKeyword = TerminalType;

pub type YulForKeyword = TerminalType;

pub type YulFunctionKeyword = TerminalType;

pub type YulGweiKeyword = TerminalType;

pub type YulHexKeyword = TerminalType;

pub type YulHoursKeyword = TerminalType;

pub type YulIfKeyword = TerminalType;

pub type YulImmutableKeyword = TerminalType;

pub type YulImplementsKeyword = TerminalType;

pub type YulImportKeyword = TerminalType;

pub type YulIndexedKeyword = TerminalType;

pub type YulInKeyword = TerminalType;

pub type YulInlineKeyword = TerminalType;

pub type YulInterfaceKeyword = TerminalType;

pub type YulInternalKeyword = TerminalType;

pub type YulIntKeyword = TerminalType;

pub type YulIsKeyword = TerminalType;

pub type YulLeaveKeyword = TerminalType;

pub type YulLetKeyword = TerminalType;

pub type YulLibraryKeyword = TerminalType;

pub type YulMacroKeyword = TerminalType;

pub type YulMappingKeyword = TerminalType;

pub type YulMatchKeyword = TerminalType;

pub type YulMemoryKeyword = TerminalType;

pub type YulMinutesKeyword = TerminalType;

pub type YulModifierKeyword = TerminalType;

pub type YulMutableKeyword = TerminalType;

pub type YulNewKeyword = TerminalType;

pub type YulNullKeyword = TerminalType;

pub type YulOfKeyword = TerminalType;

pub type YulOverrideKeyword = TerminalType;

pub type YulPartialKeyword = TerminalType;

pub type YulPayableKeyword = TerminalType;

pub type YulPragmaKeyword = TerminalType;

pub type YulPrivateKeyword = TerminalType;

pub type YulPromiseKeyword = TerminalType;

pub type YulPublicKeyword = TerminalType;

pub type YulPureKeyword = TerminalType;

pub type YulReceiveKeyword = TerminalType;

pub type YulReferenceKeyword = TerminalType;

pub type YulRelocatableKeyword = TerminalType;

pub type YulReturnsKeyword = TerminalType;

pub type YulSealedKeyword = TerminalType;

pub type YulSecondsKeyword = TerminalType;

pub type YulSizeOfKeyword = TerminalType;

pub type YulStaticKeyword = TerminalType;

pub type YulStorageKeyword = TerminalType;

pub type YulStringKeyword = TerminalType;

pub type YulStructKeyword = TerminalType;

pub type YulSuperKeyword = TerminalType;

pub type YulSupportsKeyword = TerminalType;

pub type YulSwitchKeyword = TerminalType;

pub type YulSzaboKeyword = TerminalType;

pub type YulThisKeyword = TerminalType;

pub type YulThrowKeyword = TerminalType;

pub type YulTrueKeyword = TerminalType;

pub type YulTryKeyword = TerminalType;

pub type YulTypeDefKeyword = TerminalType;

pub type YulTypeKeyword = TerminalType;

pub type YulTypeOfKeyword = TerminalType;

pub type YulUfixedKeyword = TerminalType;

pub type YulUintKeyword = TerminalType;

pub type YulUncheckedKeyword = TerminalType;

pub type YulUsingKeyword = TerminalType;

pub type YulVarKeyword = TerminalType;

pub type YulViewKeyword = TerminalType;

pub type YulVirtualKeyword = TerminalType;

pub type YulWeeksKeyword = TerminalType;

pub type YulWeiKeyword = TerminalType;

pub type YulWhileKeyword = TerminalType;

pub type YulYearsKeyword = TerminalType;

pub type VersionSpecifier = TerminalType;

pub fn new_version_specifier<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> VersionSpecifier {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::VersionSpecifier,
    }
}

pub type SingleQuotedVersionLiteral = TerminalType;

pub fn new_single_quoted_version_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleQuotedVersionLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SingleQuotedVersionLiteral,
    }
}

pub type DoubleQuotedVersionLiteral = TerminalType;

pub fn new_double_quoted_version_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoubleQuotedVersionLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DoubleQuotedVersionLiteral,
    }
}

pub type Whitespace = TerminalType;

pub fn new_whitespace<'source>(l: usize, r: usize, source: &'source str) -> Whitespace {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Whitespace,
    }
}

pub type EndOfLine = TerminalType;

pub fn new_end_of_line<'source>(l: usize, r: usize, source: &'source str) -> EndOfLine {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::EndOfLine,
    }
}

pub type SingleLineComment = TerminalType;

pub fn new_single_line_comment<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleLineComment {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SingleLineComment,
    }
}

pub type MultiLineComment = TerminalType;

pub fn new_multi_line_comment<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MultiLineComment {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MultiLineComment,
    }
}

pub type SingleLineNatSpecComment = TerminalType;

pub fn new_single_line_nat_spec_comment<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleLineNatSpecComment {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SingleLineNatSpecComment,
    }
}

pub type MultiLineNatSpecComment = TerminalType;

pub fn new_multi_line_nat_spec_comment<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MultiLineNatSpecComment {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MultiLineNatSpecComment,
    }
}

pub type AbicoderKeyword_Unreserved = TerminalType;

pub fn new_abicoder_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AbicoderKeyword_Unreserved,
    }
}

pub type AbicoderV1Keyword_Unreserved = TerminalType;

pub fn new_abicoder_v1_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderV1Keyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AbicoderV1Keyword_Unreserved,
    }
}

pub type AbicoderV2Keyword_Unreserved = TerminalType;

pub fn new_abicoder_v2_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderV2Keyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AbicoderV2Keyword_Unreserved,
    }
}

pub type ABIEncoderV2Keyword_Unreserved = TerminalType;

pub fn new_abi_encoder_v2_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ABIEncoderV2Keyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ABIEncoderV2Keyword_Unreserved,
    }
}

pub type AbstractKeyword_Reserved = TerminalType;

pub fn new_abstract_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbstractKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AbstractKeyword_Reserved,
    }
}

pub type AddressKeyword_Reserved = TerminalType;

pub fn new_address_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AddressKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AddressKeyword_Reserved,
    }
}

pub type AfterKeyword_Reserved = TerminalType;

pub fn new_after_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AfterKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AfterKeyword_Reserved,
    }
}

pub type AliasKeyword_Reserved = TerminalType;

pub fn new_alias_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AliasKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AliasKeyword_Reserved,
    }
}

pub type AliasKeyword_Unreserved = TerminalType;

pub fn new_alias_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AliasKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AliasKeyword_Unreserved,
    }
}

pub type AnonymousKeyword_Reserved = TerminalType;

pub fn new_anonymous_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AnonymousKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AnonymousKeyword_Reserved,
    }
}

pub type ApplyKeyword_Reserved = TerminalType;

pub fn new_apply_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ApplyKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ApplyKeyword_Reserved,
    }
}

pub type ApplyKeyword_Unreserved = TerminalType;

pub fn new_apply_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ApplyKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ApplyKeyword_Unreserved,
    }
}

pub type AsKeyword_Reserved = TerminalType;

pub fn new_as_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AsKeyword_Reserved,
    }
}

pub type AssemblyKeyword_Reserved = TerminalType;

pub fn new_assembly_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AssemblyKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AssemblyKeyword_Reserved,
    }
}

pub type AtKeyword_Unreserved = TerminalType;

pub fn new_at_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AtKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AtKeyword_Unreserved,
    }
}

pub type AutoKeyword_Reserved = TerminalType;

pub fn new_auto_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AutoKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AutoKeyword_Reserved,
    }
}

pub type AutoKeyword_Unreserved = TerminalType;

pub fn new_auto_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AutoKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AutoKeyword_Unreserved,
    }
}

pub type BoolKeyword_Reserved = TerminalType;

pub fn new_bool_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> BoolKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::BoolKeyword_Reserved,
    }
}

pub type BreakKeyword_Reserved = TerminalType;

pub fn new_break_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> BreakKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::BreakKeyword_Reserved,
    }
}

pub type ByteKeyword_Reserved = TerminalType;

pub fn new_byte_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ByteKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ByteKeyword_Reserved,
    }
}

pub type BytesKeyword_Reserved = TerminalType;

pub fn new_bytes_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> BytesKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::BytesKeyword_Reserved,
    }
}

pub type CallDataKeyword_Reserved = TerminalType;

pub fn new_call_data_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CallDataKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CallDataKeyword_Reserved,
    }
}

pub type CallDataKeyword_Unreserved = TerminalType;

pub fn new_call_data_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CallDataKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CallDataKeyword_Unreserved,
    }
}

pub type CaseKeyword_Reserved = TerminalType;

pub fn new_case_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CaseKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CaseKeyword_Reserved,
    }
}

pub type CatchKeyword_Reserved = TerminalType;

pub fn new_catch_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CatchKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CatchKeyword_Reserved,
    }
}

pub type ConstantKeyword_Reserved = TerminalType;

pub fn new_constant_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ConstantKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ConstantKeyword_Reserved,
    }
}

pub type ConstructorKeyword_Reserved = TerminalType;

pub fn new_constructor_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ConstructorKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ConstructorKeyword_Reserved,
    }
}

pub type ConstructorKeyword_Unreserved = TerminalType;

pub fn new_constructor_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ConstructorKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ConstructorKeyword_Unreserved,
    }
}

pub type ContinueKeyword_Reserved = TerminalType;

pub fn new_continue_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ContinueKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ContinueKeyword_Reserved,
    }
}

pub type ContractKeyword_Reserved = TerminalType;

pub fn new_contract_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ContractKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ContractKeyword_Reserved,
    }
}

pub type CopyOfKeyword_Reserved = TerminalType;

pub fn new_copy_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CopyOfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CopyOfKeyword_Reserved,
    }
}

pub type CopyOfKeyword_Unreserved = TerminalType;

pub fn new_copy_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CopyOfKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CopyOfKeyword_Unreserved,
    }
}

pub type DaysKeyword_Reserved = TerminalType;

pub fn new_days_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DaysKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DaysKeyword_Reserved,
    }
}

pub type DefaultKeyword_Reserved = TerminalType;

pub fn new_default_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DefaultKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DefaultKeyword_Reserved,
    }
}

pub type DefineKeyword_Reserved = TerminalType;

pub fn new_define_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DefineKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DefineKeyword_Reserved,
    }
}

pub type DefineKeyword_Unreserved = TerminalType;

pub fn new_define_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DefineKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DefineKeyword_Unreserved,
    }
}

pub type DeleteKeyword_Reserved = TerminalType;

pub fn new_delete_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DeleteKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DeleteKeyword_Reserved,
    }
}

pub type DoKeyword_Reserved = TerminalType;

pub fn new_do_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DoKeyword_Reserved,
    }
}

pub type ElseKeyword_Reserved = TerminalType;

pub fn new_else_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ElseKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ElseKeyword_Reserved,
    }
}

pub type EmitKeyword_Reserved = TerminalType;

pub fn new_emit_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EmitKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::EmitKeyword_Reserved,
    }
}

pub type EmitKeyword_Unreserved = TerminalType;

pub fn new_emit_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EmitKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::EmitKeyword_Unreserved,
    }
}

pub type EnumKeyword_Reserved = TerminalType;

pub fn new_enum_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EnumKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::EnumKeyword_Reserved,
    }
}

pub type ErrorKeyword_Reserved = TerminalType;

pub fn new_error_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ErrorKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ErrorKeyword_Reserved,
    }
}

pub type EtherKeyword_Reserved = TerminalType;

pub fn new_ether_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EtherKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::EtherKeyword_Reserved,
    }
}

pub type EventKeyword_Reserved = TerminalType;

pub fn new_event_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EventKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::EventKeyword_Reserved,
    }
}

pub type ExperimentalKeyword_Unreserved = TerminalType;

pub fn new_experimental_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ExperimentalKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ExperimentalKeyword_Unreserved,
    }
}

pub type ExternalKeyword_Reserved = TerminalType;

pub fn new_external_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ExternalKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ExternalKeyword_Reserved,
    }
}

pub type FallbackKeyword_Reserved = TerminalType;

pub fn new_fallback_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FallbackKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FallbackKeyword_Reserved,
    }
}

pub type FallbackKeyword_Unreserved = TerminalType;

pub fn new_fallback_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FallbackKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FallbackKeyword_Unreserved,
    }
}

pub type FalseKeyword_Reserved = TerminalType;

pub fn new_false_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FalseKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FalseKeyword_Reserved,
    }
}

pub type FinalKeyword_Reserved = TerminalType;

pub fn new_final_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FinalKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FinalKeyword_Reserved,
    }
}

pub type FinneyKeyword_Reserved = TerminalType;

pub fn new_finney_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FinneyKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FinneyKeyword_Reserved,
    }
}

pub type FinneyKeyword_Unreserved = TerminalType;

pub fn new_finney_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FinneyKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FinneyKeyword_Unreserved,
    }
}

pub type FixedKeyword_Reserved = TerminalType;

pub fn new_fixed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FixedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FixedKeyword_Reserved,
    }
}

pub type FixedKeyword_Unreserved = TerminalType;

pub fn new_fixed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FixedKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FixedKeyword_Unreserved,
    }
}

pub type ForKeyword_Reserved = TerminalType;

pub fn new_for_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ForKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ForKeyword_Reserved,
    }
}

pub type FromKeyword_Unreserved = TerminalType;

pub fn new_from_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FromKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FromKeyword_Unreserved,
    }
}

pub type FunctionKeyword_Reserved = TerminalType;

pub fn new_function_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FunctionKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::FunctionKeyword_Reserved,
    }
}

pub type GlobalKeyword_Unreserved = TerminalType;

pub fn new_global_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GlobalKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::GlobalKeyword_Unreserved,
    }
}

pub type GweiKeyword_Reserved = TerminalType;

pub fn new_gwei_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GweiKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::GweiKeyword_Reserved,
    }
}

pub type GweiKeyword_Unreserved = TerminalType;

pub fn new_gwei_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GweiKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::GweiKeyword_Unreserved,
    }
}

pub type HexKeyword_Reserved = TerminalType;

pub fn new_hex_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> HexKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::HexKeyword_Reserved,
    }
}

pub type HoursKeyword_Reserved = TerminalType;

pub fn new_hours_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> HoursKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::HoursKeyword_Reserved,
    }
}

pub type IfKeyword_Reserved = TerminalType;

pub fn new_if_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::IfKeyword_Reserved,
    }
}

pub type ImmutableKeyword_Reserved = TerminalType;

pub fn new_immutable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImmutableKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ImmutableKeyword_Reserved,
    }
}

pub type ImmutableKeyword_Unreserved = TerminalType;

pub fn new_immutable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImmutableKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ImmutableKeyword_Unreserved,
    }
}

pub type ImplementsKeyword_Reserved = TerminalType;

pub fn new_implements_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImplementsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ImplementsKeyword_Reserved,
    }
}

pub type ImplementsKeyword_Unreserved = TerminalType;

pub fn new_implements_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImplementsKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ImplementsKeyword_Unreserved,
    }
}

pub type ImportKeyword_Reserved = TerminalType;

pub fn new_import_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImportKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ImportKeyword_Reserved,
    }
}

pub type IndexedKeyword_Reserved = TerminalType;

pub fn new_indexed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IndexedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::IndexedKeyword_Reserved,
    }
}

pub type InKeyword_Reserved = TerminalType;

pub fn new_in_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::InKeyword_Reserved,
    }
}

pub type InlineKeyword_Reserved = TerminalType;

pub fn new_inline_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InlineKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::InlineKeyword_Reserved,
    }
}

pub type InterfaceKeyword_Reserved = TerminalType;

pub fn new_interface_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InterfaceKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::InterfaceKeyword_Reserved,
    }
}

pub type InternalKeyword_Reserved = TerminalType;

pub fn new_internal_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InternalKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::InternalKeyword_Reserved,
    }
}

pub type IntKeyword_Reserved = TerminalType;

pub fn new_int_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IntKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::IntKeyword_Reserved,
    }
}

pub type IsKeyword_Reserved = TerminalType;

pub fn new_is_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::IsKeyword_Reserved,
    }
}

pub type LayoutKeyword_Unreserved = TerminalType;

pub fn new_layout_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LayoutKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::LayoutKeyword_Unreserved,
    }
}

pub type LetKeyword_Reserved = TerminalType;

pub fn new_let_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LetKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::LetKeyword_Reserved,
    }
}

pub type LibraryKeyword_Reserved = TerminalType;

pub fn new_library_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LibraryKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::LibraryKeyword_Reserved,
    }
}

pub type MacroKeyword_Reserved = TerminalType;

pub fn new_macro_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MacroKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MacroKeyword_Reserved,
    }
}

pub type MacroKeyword_Unreserved = TerminalType;

pub fn new_macro_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MacroKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MacroKeyword_Unreserved,
    }
}

pub type MappingKeyword_Reserved = TerminalType;

pub fn new_mapping_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MappingKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MappingKeyword_Reserved,
    }
}

pub type MatchKeyword_Reserved = TerminalType;

pub fn new_match_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MatchKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MatchKeyword_Reserved,
    }
}

pub type MemoryKeyword_Reserved = TerminalType;

pub fn new_memory_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MemoryKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MemoryKeyword_Reserved,
    }
}

pub type MinutesKeyword_Reserved = TerminalType;

pub fn new_minutes_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MinutesKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MinutesKeyword_Reserved,
    }
}

pub type ModifierKeyword_Reserved = TerminalType;

pub fn new_modifier_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ModifierKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ModifierKeyword_Reserved,
    }
}

pub type MutableKeyword_Reserved = TerminalType;

pub fn new_mutable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MutableKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MutableKeyword_Reserved,
    }
}

pub type MutableKeyword_Unreserved = TerminalType;

pub fn new_mutable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MutableKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MutableKeyword_Unreserved,
    }
}

pub type NewKeyword_Reserved = TerminalType;

pub fn new_new_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> NewKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::NewKeyword_Reserved,
    }
}

pub type NullKeyword_Reserved = TerminalType;

pub fn new_null_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> NullKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::NullKeyword_Reserved,
    }
}

pub type OfKeyword_Reserved = TerminalType;

pub fn new_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> OfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::OfKeyword_Reserved,
    }
}

pub type OverrideKeyword_Reserved = TerminalType;

pub fn new_override_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> OverrideKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::OverrideKeyword_Reserved,
    }
}

pub type OverrideKeyword_Unreserved = TerminalType;

pub fn new_override_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> OverrideKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::OverrideKeyword_Unreserved,
    }
}

pub type PartialKeyword_Reserved = TerminalType;

pub fn new_partial_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PartialKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PartialKeyword_Reserved,
    }
}

pub type PartialKeyword_Unreserved = TerminalType;

pub fn new_partial_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PartialKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PartialKeyword_Unreserved,
    }
}

pub type PayableKeyword_Reserved = TerminalType;

pub fn new_payable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PayableKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PayableKeyword_Reserved,
    }
}

pub type PragmaKeyword_Reserved = TerminalType;

pub fn new_pragma_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PragmaKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PragmaKeyword_Reserved,
    }
}

pub type PrivateKeyword_Reserved = TerminalType;

pub fn new_private_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PrivateKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PrivateKeyword_Reserved,
    }
}

pub type PromiseKeyword_Reserved = TerminalType;

pub fn new_promise_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PromiseKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PromiseKeyword_Reserved,
    }
}

pub type PromiseKeyword_Unreserved = TerminalType;

pub fn new_promise_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PromiseKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PromiseKeyword_Unreserved,
    }
}

pub type PublicKeyword_Reserved = TerminalType;

pub fn new_public_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PublicKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PublicKeyword_Reserved,
    }
}

pub type PureKeyword_Reserved = TerminalType;

pub fn new_pure_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PureKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PureKeyword_Reserved,
    }
}

pub type ReceiveKeyword_Reserved = TerminalType;

pub fn new_receive_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReceiveKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ReceiveKeyword_Reserved,
    }
}

pub type ReceiveKeyword_Unreserved = TerminalType;

pub fn new_receive_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReceiveKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ReceiveKeyword_Unreserved,
    }
}

pub type ReferenceKeyword_Reserved = TerminalType;

pub fn new_reference_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReferenceKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ReferenceKeyword_Reserved,
    }
}

pub type ReferenceKeyword_Unreserved = TerminalType;

pub fn new_reference_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReferenceKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ReferenceKeyword_Unreserved,
    }
}

pub type RelocatableKeyword_Reserved = TerminalType;

pub fn new_relocatable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> RelocatableKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::RelocatableKeyword_Reserved,
    }
}

pub type ReturnKeyword_Reserved = TerminalType;

pub fn new_return_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReturnKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ReturnKeyword_Reserved,
    }
}

pub type ReturnsKeyword_Reserved = TerminalType;

pub fn new_returns_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReturnsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ReturnsKeyword_Reserved,
    }
}

pub type RevertKeyword_Reserved = TerminalType;

pub fn new_revert_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> RevertKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::RevertKeyword_Reserved,
    }
}

pub type SealedKeyword_Reserved = TerminalType;

pub fn new_sealed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SealedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SealedKeyword_Reserved,
    }
}

pub type SealedKeyword_Unreserved = TerminalType;

pub fn new_sealed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SealedKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SealedKeyword_Unreserved,
    }
}

pub type SecondsKeyword_Reserved = TerminalType;

pub fn new_seconds_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SecondsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SecondsKeyword_Reserved,
    }
}

pub type SizeOfKeyword_Reserved = TerminalType;

pub fn new_size_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SizeOfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SizeOfKeyword_Reserved,
    }
}

pub type SizeOfKeyword_Unreserved = TerminalType;

pub fn new_size_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SizeOfKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SizeOfKeyword_Unreserved,
    }
}

pub type SMTCheckerKeyword_Unreserved = TerminalType;

pub fn new_smt_checker_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SMTCheckerKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SMTCheckerKeyword_Unreserved,
    }
}

pub type SolidityKeyword_Unreserved = TerminalType;

pub fn new_solidity_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SolidityKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SolidityKeyword_Unreserved,
    }
}

pub type StaticKeyword_Reserved = TerminalType;

pub fn new_static_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StaticKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::StaticKeyword_Reserved,
    }
}

pub type StorageKeyword_Reserved = TerminalType;

pub fn new_storage_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StorageKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::StorageKeyword_Reserved,
    }
}

pub type StringKeyword_Reserved = TerminalType;

pub fn new_string_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StringKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::StringKeyword_Reserved,
    }
}

pub type StructKeyword_Reserved = TerminalType;

pub fn new_struct_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StructKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::StructKeyword_Reserved,
    }
}

pub type SuperKeyword_Reserved = TerminalType;

pub fn new_super_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SuperKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SuperKeyword_Reserved,
    }
}

pub type SuperKeyword_Unreserved = TerminalType;

pub fn new_super_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SuperKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SuperKeyword_Unreserved,
    }
}

pub type SupportsKeyword_Reserved = TerminalType;

pub fn new_supports_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SupportsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SupportsKeyword_Reserved,
    }
}

pub type SupportsKeyword_Unreserved = TerminalType;

pub fn new_supports_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SupportsKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SupportsKeyword_Unreserved,
    }
}

pub type SwitchKeyword_Reserved = TerminalType;

pub fn new_switch_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SwitchKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SwitchKeyword_Reserved,
    }
}

pub type SzaboKeyword_Reserved = TerminalType;

pub fn new_szabo_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SzaboKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SzaboKeyword_Reserved,
    }
}

pub type SzaboKeyword_Unreserved = TerminalType;

pub fn new_szabo_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SzaboKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SzaboKeyword_Unreserved,
    }
}

pub type ThisKeyword_Reserved = TerminalType;

pub fn new_this_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ThisKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ThisKeyword_Reserved,
    }
}

pub type ThisKeyword_Unreserved = TerminalType;

pub fn new_this_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ThisKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ThisKeyword_Unreserved,
    }
}

pub type ThrowKeyword_Reserved = TerminalType;

pub fn new_throw_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ThrowKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ThrowKeyword_Reserved,
    }
}

pub type TransientKeyword_Reserved = TerminalType;

pub fn new_transient_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TransientKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::TransientKeyword_Reserved,
    }
}

pub type TrueKeyword_Reserved = TerminalType;

pub fn new_true_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TrueKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::TrueKeyword_Reserved,
    }
}

pub type TryKeyword_Reserved = TerminalType;

pub fn new_try_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TryKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::TryKeyword_Reserved,
    }
}

pub type TypeDefKeyword_Reserved = TerminalType;

pub fn new_type_def_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeDefKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::TypeDefKeyword_Reserved,
    }
}

pub type TypeDefKeyword_Unreserved = TerminalType;

pub fn new_type_def_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeDefKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::TypeDefKeyword_Unreserved,
    }
}

pub type TypeKeyword_Reserved = TerminalType;

pub fn new_type_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::TypeKeyword_Reserved,
    }
}

pub type TypeOfKeyword_Reserved = TerminalType;

pub fn new_type_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeOfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::TypeOfKeyword_Reserved,
    }
}

pub type UfixedKeyword_Reserved = TerminalType;

pub fn new_ufixed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UfixedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::UfixedKeyword_Reserved,
    }
}

pub type UfixedKeyword_Unreserved = TerminalType;

pub fn new_ufixed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UfixedKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::UfixedKeyword_Unreserved,
    }
}

pub type UintKeyword_Reserved = TerminalType;

pub fn new_uint_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UintKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::UintKeyword_Reserved,
    }
}

pub type UncheckedKeyword_Reserved = TerminalType;

pub fn new_unchecked_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UncheckedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::UncheckedKeyword_Reserved,
    }
}

pub type UncheckedKeyword_Unreserved = TerminalType;

pub fn new_unchecked_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UncheckedKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::UncheckedKeyword_Unreserved,
    }
}

pub type UsingKeyword_Reserved = TerminalType;

pub fn new_using_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UsingKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::UsingKeyword_Reserved,
    }
}

pub type VarKeyword_Reserved = TerminalType;

pub fn new_var_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> VarKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::VarKeyword_Reserved,
    }
}

pub type ViewKeyword_Reserved = TerminalType;

pub fn new_view_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ViewKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ViewKeyword_Reserved,
    }
}

pub type VirtualKeyword_Reserved = TerminalType;

pub fn new_virtual_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> VirtualKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::VirtualKeyword_Reserved,
    }
}

pub type VirtualKeyword_Unreserved = TerminalType;

pub fn new_virtual_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> VirtualKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::VirtualKeyword_Unreserved,
    }
}

pub type WeeksKeyword_Reserved = TerminalType;

pub fn new_weeks_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> WeeksKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::WeeksKeyword_Reserved,
    }
}

pub type WeiKeyword_Reserved = TerminalType;

pub fn new_wei_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> WeiKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::WeiKeyword_Reserved,
    }
}

pub type WhileKeyword_Reserved = TerminalType;

pub fn new_while_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> WhileKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::WhileKeyword_Reserved,
    }
}

pub type YearsKeyword_Reserved = TerminalType;

pub fn new_years_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YearsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YearsKeyword_Reserved,
    }
}

pub type OpenParen = TerminalType;

pub fn new_open_paren<'source>(l: usize, r: usize, source: &'source str) -> OpenParen {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::OpenParen,
    }
}

pub type CloseParen = TerminalType;

pub fn new_close_paren<'source>(l: usize, r: usize, source: &'source str) -> CloseParen {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CloseParen,
    }
}

pub type OpenBracket = TerminalType;

pub fn new_open_bracket<'source>(l: usize, r: usize, source: &'source str) -> OpenBracket {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::OpenBracket,
    }
}

pub type CloseBracket = TerminalType;

pub fn new_close_bracket<'source>(l: usize, r: usize, source: &'source str) -> CloseBracket {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CloseBracket,
    }
}

pub type OpenBrace = TerminalType;

pub fn new_open_brace<'source>(l: usize, r: usize, source: &'source str) -> OpenBrace {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::OpenBrace,
    }
}

pub type CloseBrace = TerminalType;

pub fn new_close_brace<'source>(l: usize, r: usize, source: &'source str) -> CloseBrace {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CloseBrace,
    }
}

pub type Comma = TerminalType;

pub fn new_comma<'source>(l: usize, r: usize, source: &'source str) -> Comma {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Comma,
    }
}

pub type Period = TerminalType;

pub fn new_period<'source>(l: usize, r: usize, source: &'source str) -> Period {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Period,
    }
}

pub type QuestionMark = TerminalType;

pub fn new_question_mark<'source>(l: usize, r: usize, source: &'source str) -> QuestionMark {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::QuestionMark,
    }
}

pub type Semicolon = TerminalType;

pub fn new_semicolon<'source>(l: usize, r: usize, source: &'source str) -> Semicolon {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Semicolon,
    }
}

pub type Colon = TerminalType;

pub fn new_colon<'source>(l: usize, r: usize, source: &'source str) -> Colon {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Colon,
    }
}

pub type ColonEqual = TerminalType;

pub fn new_colon_equal<'source>(l: usize, r: usize, source: &'source str) -> ColonEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::ColonEqual,
    }
}

pub type Equal = TerminalType;

pub fn new_equal<'source>(l: usize, r: usize, source: &'source str) -> Equal {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Equal,
    }
}

pub type EqualColon = TerminalType;

pub fn new_equal_colon<'source>(l: usize, r: usize, source: &'source str) -> EqualColon {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::EqualColon,
    }
}

pub type EqualEqual = TerminalType;

pub fn new_equal_equal<'source>(l: usize, r: usize, source: &'source str) -> EqualEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::EqualEqual,
    }
}

pub type EqualGreaterThan = TerminalType;

pub fn new_equal_greater_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EqualGreaterThan {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::EqualGreaterThan,
    }
}

pub type Asterisk = TerminalType;

pub fn new_asterisk<'source>(l: usize, r: usize, source: &'source str) -> Asterisk {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Asterisk,
    }
}

pub type AsteriskEqual = TerminalType;

pub fn new_asterisk_equal<'source>(l: usize, r: usize, source: &'source str) -> AsteriskEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AsteriskEqual,
    }
}

pub type AsteriskAsterisk = TerminalType;

pub fn new_asterisk_asterisk<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AsteriskAsterisk {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AsteriskAsterisk,
    }
}

pub type Bar = TerminalType;

pub fn new_bar<'source>(l: usize, r: usize, source: &'source str) -> Bar {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Bar,
    }
}

pub type BarEqual = TerminalType;

pub fn new_bar_equal<'source>(l: usize, r: usize, source: &'source str) -> BarEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::BarEqual,
    }
}

pub type BarBar = TerminalType;

pub fn new_bar_bar<'source>(l: usize, r: usize, source: &'source str) -> BarBar {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::BarBar,
    }
}

pub type Ampersand = TerminalType;

pub fn new_ampersand<'source>(l: usize, r: usize, source: &'source str) -> Ampersand {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Ampersand,
    }
}

pub type AmpersandEqual = TerminalType;

pub fn new_ampersand_equal<'source>(l: usize, r: usize, source: &'source str) -> AmpersandEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AmpersandEqual,
    }
}

pub type AmpersandAmpersand = TerminalType;

pub fn new_ampersand_ampersand<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AmpersandAmpersand {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::AmpersandAmpersand,
    }
}

pub type LessThan = TerminalType;

pub fn new_less_than<'source>(l: usize, r: usize, source: &'source str) -> LessThan {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::LessThan,
    }
}

pub type LessThanEqual = TerminalType;

pub fn new_less_than_equal<'source>(l: usize, r: usize, source: &'source str) -> LessThanEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::LessThanEqual,
    }
}

pub type LessThanLessThan = TerminalType;

pub fn new_less_than_less_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LessThanLessThan {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::LessThanLessThan,
    }
}

pub type LessThanLessThanEqual = TerminalType;

pub fn new_less_than_less_than_equal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LessThanLessThanEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::LessThanLessThanEqual,
    }
}

pub type GreaterThan = TerminalType;

pub fn new_greater_than<'source>(l: usize, r: usize, source: &'source str) -> GreaterThan {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::GreaterThan,
    }
}

pub type GreaterThanEqual = TerminalType;

pub fn new_greater_than_equal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::GreaterThanEqual,
    }
}

pub type GreaterThanGreaterThan = TerminalType;

pub fn new_greater_than_greater_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanGreaterThan {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::GreaterThanGreaterThan,
    }
}

pub type GreaterThanGreaterThanEqual = TerminalType;

pub fn new_greater_than_greater_than_equal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanGreaterThanEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::GreaterThanGreaterThanEqual,
    }
}

pub type GreaterThanGreaterThanGreaterThan = TerminalType;

pub fn new_greater_than_greater_than_greater_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanGreaterThanGreaterThan {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::GreaterThanGreaterThanGreaterThan,
    }
}

pub type GreaterThanGreaterThanGreaterThanEqual = TerminalType;

pub fn new_greater_than_greater_than_greater_than_equal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanGreaterThanGreaterThanEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::GreaterThanGreaterThanGreaterThanEqual,
    }
}

pub type Plus = TerminalType;

pub fn new_plus<'source>(l: usize, r: usize, source: &'source str) -> Plus {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Plus,
    }
}

pub type PlusEqual = TerminalType;

pub fn new_plus_equal<'source>(l: usize, r: usize, source: &'source str) -> PlusEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PlusEqual,
    }
}

pub type PlusPlus = TerminalType;

pub fn new_plus_plus<'source>(l: usize, r: usize, source: &'source str) -> PlusPlus {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PlusPlus,
    }
}

pub type Minus = TerminalType;

pub fn new_minus<'source>(l: usize, r: usize, source: &'source str) -> Minus {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Minus,
    }
}

pub type MinusEqual = TerminalType;

pub fn new_minus_equal<'source>(l: usize, r: usize, source: &'source str) -> MinusEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MinusEqual,
    }
}

pub type MinusMinus = TerminalType;

pub fn new_minus_minus<'source>(l: usize, r: usize, source: &'source str) -> MinusMinus {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MinusMinus,
    }
}

pub type MinusGreaterThan = TerminalType;

pub fn new_minus_greater_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MinusGreaterThan {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::MinusGreaterThan,
    }
}

pub type Slash = TerminalType;

pub fn new_slash<'source>(l: usize, r: usize, source: &'source str) -> Slash {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Slash,
    }
}

pub type SlashEqual = TerminalType;

pub fn new_slash_equal<'source>(l: usize, r: usize, source: &'source str) -> SlashEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SlashEqual,
    }
}

pub type Percent = TerminalType;

pub fn new_percent<'source>(l: usize, r: usize, source: &'source str) -> Percent {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Percent,
    }
}

pub type PercentEqual = TerminalType;

pub fn new_percent_equal<'source>(l: usize, r: usize, source: &'source str) -> PercentEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::PercentEqual,
    }
}

pub type Bang = TerminalType;

pub fn new_bang<'source>(l: usize, r: usize, source: &'source str) -> Bang {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Bang,
    }
}

pub type BangEqual = TerminalType;

pub fn new_bang_equal<'source>(l: usize, r: usize, source: &'source str) -> BangEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::BangEqual,
    }
}

pub type Caret = TerminalType;

pub fn new_caret<'source>(l: usize, r: usize, source: &'source str) -> Caret {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Caret,
    }
}

pub type CaretEqual = TerminalType;

pub fn new_caret_equal<'source>(l: usize, r: usize, source: &'source str) -> CaretEqual {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::CaretEqual,
    }
}

pub type Tilde = TerminalType;

pub fn new_tilde<'source>(l: usize, r: usize, source: &'source str) -> Tilde {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Tilde,
    }
}

pub type HexLiteral = TerminalType;

pub fn new_hex_literal<'source>(l: usize, r: usize, source: &'source str) -> HexLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::HexLiteral,
    }
}

pub type DecimalLiteral = TerminalType;

pub fn new_decimal_literal<'source>(l: usize, r: usize, source: &'source str) -> DecimalLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DecimalLiteral,
    }
}

pub type SingleQuotedStringLiteral = TerminalType;

pub fn new_single_quoted_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleQuotedStringLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SingleQuotedStringLiteral,
    }
}

pub type DoubleQuotedStringLiteral = TerminalType;

pub fn new_double_quoted_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoubleQuotedStringLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DoubleQuotedStringLiteral,
    }
}

pub type SingleQuotedHexStringLiteral = TerminalType;

pub fn new_single_quoted_hex_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleQuotedHexStringLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SingleQuotedHexStringLiteral,
    }
}

pub type DoubleQuotedHexStringLiteral = TerminalType;

pub fn new_double_quoted_hex_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoubleQuotedHexStringLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DoubleQuotedHexStringLiteral,
    }
}

pub type SingleQuotedUnicodeStringLiteral = TerminalType;

pub fn new_single_quoted_unicode_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleQuotedUnicodeStringLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::SingleQuotedUnicodeStringLiteral,
    }
}

pub type DoubleQuotedUnicodeStringLiteral = TerminalType;

pub fn new_double_quoted_unicode_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoubleQuotedUnicodeStringLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::DoubleQuotedUnicodeStringLiteral,
    }
}

pub type Identifier = TerminalType;

pub fn new_identifier<'source>(l: usize, r: usize, source: &'source str) -> Identifier {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::Identifier,
    }
}

pub type YulIdentifier = TerminalType;

pub fn new_yul_identifier<'source>(l: usize, r: usize, source: &'source str) -> YulIdentifier {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulIdentifier,
    }
}

pub type YulDecimalLiteral = TerminalType;

pub fn new_yul_decimal_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDecimalLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDecimalLiteral,
    }
}

pub type YulHexLiteral = TerminalType;

pub fn new_yul_hex_literal<'source>(l: usize, r: usize, source: &'source str) -> YulHexLiteral {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulHexLiteral,
    }
}

pub type YulAbstractKeyword_Reserved = TerminalType;

pub fn new_yul_abstract_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAbstractKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAbstractKeyword_Reserved,
    }
}

pub type YulAbstractKeyword_Unreserved = TerminalType;

pub fn new_yul_abstract_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAbstractKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAbstractKeyword_Unreserved,
    }
}

pub type YulAfterKeyword_Reserved = TerminalType;

pub fn new_yul_after_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAfterKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAfterKeyword_Reserved,
    }
}

pub type YulAfterKeyword_Unreserved = TerminalType;

pub fn new_yul_after_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAfterKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAfterKeyword_Unreserved,
    }
}

pub type YulAliasKeyword_Reserved = TerminalType;

pub fn new_yul_alias_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAliasKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAliasKeyword_Reserved,
    }
}

pub type YulAliasKeyword_Unreserved = TerminalType;

pub fn new_yul_alias_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAliasKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAliasKeyword_Unreserved,
    }
}

pub type YulAnonymousKeyword_Reserved = TerminalType;

pub fn new_yul_anonymous_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAnonymousKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAnonymousKeyword_Reserved,
    }
}

pub type YulAnonymousKeyword_Unreserved = TerminalType;

pub fn new_yul_anonymous_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAnonymousKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAnonymousKeyword_Unreserved,
    }
}

pub type YulApplyKeyword_Reserved = TerminalType;

pub fn new_yul_apply_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulApplyKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulApplyKeyword_Reserved,
    }
}

pub type YulApplyKeyword_Unreserved = TerminalType;

pub fn new_yul_apply_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulApplyKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulApplyKeyword_Unreserved,
    }
}

pub type YulAsKeyword_Reserved = TerminalType;

pub fn new_yul_as_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAsKeyword_Reserved,
    }
}

pub type YulAsKeyword_Unreserved = TerminalType;

pub fn new_yul_as_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAsKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAsKeyword_Unreserved,
    }
}

pub type YulAssemblyKeyword_Reserved = TerminalType;

pub fn new_yul_assembly_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAssemblyKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAssemblyKeyword_Reserved,
    }
}

pub type YulAssemblyKeyword_Unreserved = TerminalType;

pub fn new_yul_assembly_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAssemblyKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAssemblyKeyword_Unreserved,
    }
}

pub type YulAutoKeyword_Reserved = TerminalType;

pub fn new_yul_auto_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAutoKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAutoKeyword_Reserved,
    }
}

pub type YulAutoKeyword_Unreserved = TerminalType;

pub fn new_yul_auto_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAutoKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulAutoKeyword_Unreserved,
    }
}

pub type YulBoolKeyword_Reserved = TerminalType;

pub fn new_yul_bool_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBoolKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulBoolKeyword_Reserved,
    }
}

pub type YulBoolKeyword_Unreserved = TerminalType;

pub fn new_yul_bool_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBoolKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulBoolKeyword_Unreserved,
    }
}

pub type YulBreakKeyword_Reserved = TerminalType;

pub fn new_yul_break_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBreakKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulBreakKeyword_Reserved,
    }
}

pub type YulBytesKeyword_Reserved = TerminalType;

pub fn new_yul_bytes_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBytesKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulBytesKeyword_Reserved,
    }
}

pub type YulBytesKeyword_Unreserved = TerminalType;

pub fn new_yul_bytes_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBytesKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulBytesKeyword_Unreserved,
    }
}

pub type YulCallDataKeyword_Reserved = TerminalType;

pub fn new_yul_call_data_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCallDataKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulCallDataKeyword_Reserved,
    }
}

pub type YulCallDataKeyword_Unreserved = TerminalType;

pub fn new_yul_call_data_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCallDataKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulCallDataKeyword_Unreserved,
    }
}

pub type YulCaseKeyword_Reserved = TerminalType;

pub fn new_yul_case_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCaseKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulCaseKeyword_Reserved,
    }
}

pub type YulCatchKeyword_Reserved = TerminalType;

pub fn new_yul_catch_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCatchKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulCatchKeyword_Reserved,
    }
}

pub type YulCatchKeyword_Unreserved = TerminalType;

pub fn new_yul_catch_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCatchKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulCatchKeyword_Unreserved,
    }
}

pub type YulConstantKeyword_Reserved = TerminalType;

pub fn new_yul_constant_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstantKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulConstantKeyword_Reserved,
    }
}

pub type YulConstantKeyword_Unreserved = TerminalType;

pub fn new_yul_constant_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstantKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulConstantKeyword_Unreserved,
    }
}

pub type YulConstructorKeyword_Reserved = TerminalType;

pub fn new_yul_constructor_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstructorKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulConstructorKeyword_Reserved,
    }
}

pub type YulConstructorKeyword_Unreserved = TerminalType;

pub fn new_yul_constructor_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstructorKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulConstructorKeyword_Unreserved,
    }
}

pub type YulContinueKeyword_Reserved = TerminalType;

pub fn new_yul_continue_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulContinueKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulContinueKeyword_Reserved,
    }
}

pub type YulContractKeyword_Reserved = TerminalType;

pub fn new_yul_contract_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulContractKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulContractKeyword_Reserved,
    }
}

pub type YulContractKeyword_Unreserved = TerminalType;

pub fn new_yul_contract_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulContractKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulContractKeyword_Unreserved,
    }
}

pub type YulCopyOfKeyword_Reserved = TerminalType;

pub fn new_yul_copy_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCopyOfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulCopyOfKeyword_Reserved,
    }
}

pub type YulCopyOfKeyword_Unreserved = TerminalType;

pub fn new_yul_copy_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCopyOfKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulCopyOfKeyword_Unreserved,
    }
}

pub type YulDaysKeyword_Reserved = TerminalType;

pub fn new_yul_days_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDaysKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDaysKeyword_Reserved,
    }
}

pub type YulDaysKeyword_Unreserved = TerminalType;

pub fn new_yul_days_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDaysKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDaysKeyword_Unreserved,
    }
}

pub type YulDefaultKeyword_Reserved = TerminalType;

pub fn new_yul_default_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDefaultKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDefaultKeyword_Reserved,
    }
}

pub type YulDefineKeyword_Reserved = TerminalType;

pub fn new_yul_define_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDefineKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDefineKeyword_Reserved,
    }
}

pub type YulDefineKeyword_Unreserved = TerminalType;

pub fn new_yul_define_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDefineKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDefineKeyword_Unreserved,
    }
}

pub type YulDeleteKeyword_Reserved = TerminalType;

pub fn new_yul_delete_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDeleteKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDeleteKeyword_Reserved,
    }
}

pub type YulDeleteKeyword_Unreserved = TerminalType;

pub fn new_yul_delete_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDeleteKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDeleteKeyword_Unreserved,
    }
}

pub type YulDoKeyword_Reserved = TerminalType;

pub fn new_yul_do_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDoKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDoKeyword_Reserved,
    }
}

pub type YulDoKeyword_Unreserved = TerminalType;

pub fn new_yul_do_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDoKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulDoKeyword_Unreserved,
    }
}

pub type YulElseKeyword_Reserved = TerminalType;

pub fn new_yul_else_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulElseKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulElseKeyword_Reserved,
    }
}

pub type YulElseKeyword_Unreserved = TerminalType;

pub fn new_yul_else_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulElseKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulElseKeyword_Unreserved,
    }
}

pub type YulEmitKeyword_Reserved = TerminalType;

pub fn new_yul_emit_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEmitKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulEmitKeyword_Reserved,
    }
}

pub type YulEmitKeyword_Unreserved = TerminalType;

pub fn new_yul_emit_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEmitKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulEmitKeyword_Unreserved,
    }
}

pub type YulEnumKeyword_Reserved = TerminalType;

pub fn new_yul_enum_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEnumKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulEnumKeyword_Reserved,
    }
}

pub type YulEnumKeyword_Unreserved = TerminalType;

pub fn new_yul_enum_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEnumKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulEnumKeyword_Unreserved,
    }
}

pub type YulEtherKeyword_Reserved = TerminalType;

pub fn new_yul_ether_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEtherKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulEtherKeyword_Reserved,
    }
}

pub type YulEtherKeyword_Unreserved = TerminalType;

pub fn new_yul_ether_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEtherKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulEtherKeyword_Unreserved,
    }
}

pub type YulEventKeyword_Reserved = TerminalType;

pub fn new_yul_event_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEventKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulEventKeyword_Reserved,
    }
}

pub type YulEventKeyword_Unreserved = TerminalType;

pub fn new_yul_event_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEventKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulEventKeyword_Unreserved,
    }
}

pub type YulExternalKeyword_Reserved = TerminalType;

pub fn new_yul_external_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulExternalKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulExternalKeyword_Reserved,
    }
}

pub type YulExternalKeyword_Unreserved = TerminalType;

pub fn new_yul_external_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulExternalKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulExternalKeyword_Unreserved,
    }
}

pub type YulFallbackKeyword_Reserved = TerminalType;

pub fn new_yul_fallback_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFallbackKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFallbackKeyword_Reserved,
    }
}

pub type YulFallbackKeyword_Unreserved = TerminalType;

pub fn new_yul_fallback_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFallbackKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFallbackKeyword_Unreserved,
    }
}

pub type YulFalseKeyword_Reserved = TerminalType;

pub fn new_yul_false_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFalseKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFalseKeyword_Reserved,
    }
}

pub type YulFinalKeyword_Reserved = TerminalType;

pub fn new_yul_final_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFinalKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFinalKeyword_Reserved,
    }
}

pub type YulFinalKeyword_Unreserved = TerminalType;

pub fn new_yul_final_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFinalKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFinalKeyword_Unreserved,
    }
}

pub type YulFinneyKeyword_Reserved = TerminalType;

pub fn new_yul_finney_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFinneyKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFinneyKeyword_Reserved,
    }
}

pub type YulFinneyKeyword_Unreserved = TerminalType;

pub fn new_yul_finney_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFinneyKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFinneyKeyword_Unreserved,
    }
}

pub type YulFixedKeyword_Reserved = TerminalType;

pub fn new_yul_fixed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFixedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFixedKeyword_Reserved,
    }
}

pub type YulFixedKeyword_Unreserved = TerminalType;

pub fn new_yul_fixed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFixedKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFixedKeyword_Unreserved,
    }
}

pub type YulForKeyword_Reserved = TerminalType;

pub fn new_yul_for_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulForKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulForKeyword_Reserved,
    }
}

pub type YulFunctionKeyword_Reserved = TerminalType;

pub fn new_yul_function_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFunctionKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulFunctionKeyword_Reserved,
    }
}

pub type YulGweiKeyword_Reserved = TerminalType;

pub fn new_yul_gwei_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulGweiKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulGweiKeyword_Reserved,
    }
}

pub type YulGweiKeyword_Unreserved = TerminalType;

pub fn new_yul_gwei_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulGweiKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulGweiKeyword_Unreserved,
    }
}

pub type YulHexKeyword_Reserved = TerminalType;

pub fn new_yul_hex_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulHexKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulHexKeyword_Reserved,
    }
}

pub type YulHoursKeyword_Reserved = TerminalType;

pub fn new_yul_hours_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulHoursKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulHoursKeyword_Reserved,
    }
}

pub type YulHoursKeyword_Unreserved = TerminalType;

pub fn new_yul_hours_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulHoursKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulHoursKeyword_Unreserved,
    }
}

pub type YulIfKeyword_Reserved = TerminalType;

pub fn new_yul_if_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulIfKeyword_Reserved,
    }
}

pub type YulImmutableKeyword_Reserved = TerminalType;

pub fn new_yul_immutable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImmutableKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulImmutableKeyword_Reserved,
    }
}

pub type YulImmutableKeyword_Unreserved = TerminalType;

pub fn new_yul_immutable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImmutableKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulImmutableKeyword_Unreserved,
    }
}

pub type YulImplementsKeyword_Reserved = TerminalType;

pub fn new_yul_implements_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImplementsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulImplementsKeyword_Reserved,
    }
}

pub type YulImplementsKeyword_Unreserved = TerminalType;

pub fn new_yul_implements_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImplementsKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulImplementsKeyword_Unreserved,
    }
}

pub type YulImportKeyword_Reserved = TerminalType;

pub fn new_yul_import_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImportKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulImportKeyword_Reserved,
    }
}

pub type YulImportKeyword_Unreserved = TerminalType;

pub fn new_yul_import_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImportKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulImportKeyword_Unreserved,
    }
}

pub type YulIndexedKeyword_Reserved = TerminalType;

pub fn new_yul_indexed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIndexedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulIndexedKeyword_Reserved,
    }
}

pub type YulIndexedKeyword_Unreserved = TerminalType;

pub fn new_yul_indexed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIndexedKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulIndexedKeyword_Unreserved,
    }
}

pub type YulInKeyword_Reserved = TerminalType;

pub fn new_yul_in_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulInKeyword_Reserved,
    }
}

pub type YulInKeyword_Unreserved = TerminalType;

pub fn new_yul_in_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulInKeyword_Unreserved,
    }
}

pub type YulInlineKeyword_Reserved = TerminalType;

pub fn new_yul_inline_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInlineKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulInlineKeyword_Reserved,
    }
}

pub type YulInlineKeyword_Unreserved = TerminalType;

pub fn new_yul_inline_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInlineKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulInlineKeyword_Unreserved,
    }
}

pub type YulInterfaceKeyword_Reserved = TerminalType;

pub fn new_yul_interface_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInterfaceKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulInterfaceKeyword_Reserved,
    }
}

pub type YulInterfaceKeyword_Unreserved = TerminalType;

pub fn new_yul_interface_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInterfaceKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulInterfaceKeyword_Unreserved,
    }
}

pub type YulInternalKeyword_Reserved = TerminalType;

pub fn new_yul_internal_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInternalKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulInternalKeyword_Reserved,
    }
}

pub type YulInternalKeyword_Unreserved = TerminalType;

pub fn new_yul_internal_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInternalKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulInternalKeyword_Unreserved,
    }
}

pub type YulIntKeyword_Reserved = TerminalType;

pub fn new_yul_int_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIntKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulIntKeyword_Reserved,
    }
}

pub type YulIntKeyword_Unreserved = TerminalType;

pub fn new_yul_int_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIntKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulIntKeyword_Unreserved,
    }
}

pub type YulIsKeyword_Reserved = TerminalType;

pub fn new_yul_is_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulIsKeyword_Reserved,
    }
}

pub type YulIsKeyword_Unreserved = TerminalType;

pub fn new_yul_is_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIsKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulIsKeyword_Unreserved,
    }
}

pub type YulLeaveKeyword_Reserved = TerminalType;

pub fn new_yul_leave_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLeaveKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulLeaveKeyword_Reserved,
    }
}

pub type YulLeaveKeyword_Unreserved = TerminalType;

pub fn new_yul_leave_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLeaveKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulLeaveKeyword_Unreserved,
    }
}

pub type YulLetKeyword_Reserved = TerminalType;

pub fn new_yul_let_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLetKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulLetKeyword_Reserved,
    }
}

pub type YulLibraryKeyword_Reserved = TerminalType;

pub fn new_yul_library_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLibraryKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulLibraryKeyword_Reserved,
    }
}

pub type YulLibraryKeyword_Unreserved = TerminalType;

pub fn new_yul_library_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLibraryKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulLibraryKeyword_Unreserved,
    }
}

pub type YulMacroKeyword_Reserved = TerminalType;

pub fn new_yul_macro_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMacroKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMacroKeyword_Reserved,
    }
}

pub type YulMacroKeyword_Unreserved = TerminalType;

pub fn new_yul_macro_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMacroKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMacroKeyword_Unreserved,
    }
}

pub type YulMappingKeyword_Reserved = TerminalType;

pub fn new_yul_mapping_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMappingKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMappingKeyword_Reserved,
    }
}

pub type YulMappingKeyword_Unreserved = TerminalType;

pub fn new_yul_mapping_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMappingKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMappingKeyword_Unreserved,
    }
}

pub type YulMatchKeyword_Reserved = TerminalType;

pub fn new_yul_match_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMatchKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMatchKeyword_Reserved,
    }
}

pub type YulMatchKeyword_Unreserved = TerminalType;

pub fn new_yul_match_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMatchKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMatchKeyword_Unreserved,
    }
}

pub type YulMemoryKeyword_Reserved = TerminalType;

pub fn new_yul_memory_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMemoryKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMemoryKeyword_Reserved,
    }
}

pub type YulMemoryKeyword_Unreserved = TerminalType;

pub fn new_yul_memory_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMemoryKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMemoryKeyword_Unreserved,
    }
}

pub type YulMinutesKeyword_Reserved = TerminalType;

pub fn new_yul_minutes_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMinutesKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMinutesKeyword_Reserved,
    }
}

pub type YulMinutesKeyword_Unreserved = TerminalType;

pub fn new_yul_minutes_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMinutesKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMinutesKeyword_Unreserved,
    }
}

pub type YulModifierKeyword_Reserved = TerminalType;

pub fn new_yul_modifier_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulModifierKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulModifierKeyword_Reserved,
    }
}

pub type YulModifierKeyword_Unreserved = TerminalType;

pub fn new_yul_modifier_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulModifierKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulModifierKeyword_Unreserved,
    }
}

pub type YulMutableKeyword_Reserved = TerminalType;

pub fn new_yul_mutable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMutableKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMutableKeyword_Reserved,
    }
}

pub type YulMutableKeyword_Unreserved = TerminalType;

pub fn new_yul_mutable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMutableKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulMutableKeyword_Unreserved,
    }
}

pub type YulNewKeyword_Reserved = TerminalType;

pub fn new_yul_new_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulNewKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulNewKeyword_Reserved,
    }
}

pub type YulNewKeyword_Unreserved = TerminalType;

pub fn new_yul_new_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulNewKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulNewKeyword_Unreserved,
    }
}

pub type YulNullKeyword_Reserved = TerminalType;

pub fn new_yul_null_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulNullKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulNullKeyword_Reserved,
    }
}

pub type YulNullKeyword_Unreserved = TerminalType;

pub fn new_yul_null_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulNullKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulNullKeyword_Unreserved,
    }
}

pub type YulOfKeyword_Reserved = TerminalType;

pub fn new_yul_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulOfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulOfKeyword_Reserved,
    }
}

pub type YulOfKeyword_Unreserved = TerminalType;

pub fn new_yul_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulOfKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulOfKeyword_Unreserved,
    }
}

pub type YulOverrideKeyword_Reserved = TerminalType;

pub fn new_yul_override_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulOverrideKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulOverrideKeyword_Reserved,
    }
}

pub type YulOverrideKeyword_Unreserved = TerminalType;

pub fn new_yul_override_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulOverrideKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulOverrideKeyword_Unreserved,
    }
}

pub type YulPartialKeyword_Reserved = TerminalType;

pub fn new_yul_partial_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPartialKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPartialKeyword_Reserved,
    }
}

pub type YulPartialKeyword_Unreserved = TerminalType;

pub fn new_yul_partial_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPartialKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPartialKeyword_Unreserved,
    }
}

pub type YulPayableKeyword_Reserved = TerminalType;

pub fn new_yul_payable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPayableKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPayableKeyword_Reserved,
    }
}

pub type YulPayableKeyword_Unreserved = TerminalType;

pub fn new_yul_payable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPayableKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPayableKeyword_Unreserved,
    }
}

pub type YulPragmaKeyword_Reserved = TerminalType;

pub fn new_yul_pragma_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPragmaKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPragmaKeyword_Reserved,
    }
}

pub type YulPragmaKeyword_Unreserved = TerminalType;

pub fn new_yul_pragma_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPragmaKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPragmaKeyword_Unreserved,
    }
}

pub type YulPrivateKeyword_Reserved = TerminalType;

pub fn new_yul_private_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPrivateKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPrivateKeyword_Reserved,
    }
}

pub type YulPrivateKeyword_Unreserved = TerminalType;

pub fn new_yul_private_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPrivateKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPrivateKeyword_Unreserved,
    }
}

pub type YulPromiseKeyword_Reserved = TerminalType;

pub fn new_yul_promise_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPromiseKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPromiseKeyword_Reserved,
    }
}

pub type YulPromiseKeyword_Unreserved = TerminalType;

pub fn new_yul_promise_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPromiseKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPromiseKeyword_Unreserved,
    }
}

pub type YulPublicKeyword_Reserved = TerminalType;

pub fn new_yul_public_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPublicKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPublicKeyword_Reserved,
    }
}

pub type YulPublicKeyword_Unreserved = TerminalType;

pub fn new_yul_public_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPublicKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPublicKeyword_Unreserved,
    }
}

pub type YulPureKeyword_Reserved = TerminalType;

pub fn new_yul_pure_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPureKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPureKeyword_Reserved,
    }
}

pub type YulPureKeyword_Unreserved = TerminalType;

pub fn new_yul_pure_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPureKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulPureKeyword_Unreserved,
    }
}

pub type YulReceiveKeyword_Reserved = TerminalType;

pub fn new_yul_receive_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReceiveKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulReceiveKeyword_Reserved,
    }
}

pub type YulReceiveKeyword_Unreserved = TerminalType;

pub fn new_yul_receive_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReceiveKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulReceiveKeyword_Unreserved,
    }
}

pub type YulReferenceKeyword_Reserved = TerminalType;

pub fn new_yul_reference_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReferenceKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulReferenceKeyword_Reserved,
    }
}

pub type YulReferenceKeyword_Unreserved = TerminalType;

pub fn new_yul_reference_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReferenceKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulReferenceKeyword_Unreserved,
    }
}

pub type YulRelocatableKeyword_Reserved = TerminalType;

pub fn new_yul_relocatable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulRelocatableKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulRelocatableKeyword_Reserved,
    }
}

pub type YulRelocatableKeyword_Unreserved = TerminalType;

pub fn new_yul_relocatable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulRelocatableKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulRelocatableKeyword_Unreserved,
    }
}

pub type YulReturnsKeyword_Reserved = TerminalType;

pub fn new_yul_returns_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReturnsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulReturnsKeyword_Reserved,
    }
}

pub type YulReturnsKeyword_Unreserved = TerminalType;

pub fn new_yul_returns_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReturnsKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulReturnsKeyword_Unreserved,
    }
}

pub type YulSealedKeyword_Reserved = TerminalType;

pub fn new_yul_sealed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSealedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSealedKeyword_Reserved,
    }
}

pub type YulSealedKeyword_Unreserved = TerminalType;

pub fn new_yul_sealed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSealedKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSealedKeyword_Unreserved,
    }
}

pub type YulSecondsKeyword_Reserved = TerminalType;

pub fn new_yul_seconds_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSecondsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSecondsKeyword_Reserved,
    }
}

pub type YulSecondsKeyword_Unreserved = TerminalType;

pub fn new_yul_seconds_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSecondsKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSecondsKeyword_Unreserved,
    }
}

pub type YulSizeOfKeyword_Reserved = TerminalType;

pub fn new_yul_size_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSizeOfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSizeOfKeyword_Reserved,
    }
}

pub type YulSizeOfKeyword_Unreserved = TerminalType;

pub fn new_yul_size_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSizeOfKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSizeOfKeyword_Unreserved,
    }
}

pub type YulStaticKeyword_Reserved = TerminalType;

pub fn new_yul_static_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStaticKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulStaticKeyword_Reserved,
    }
}

pub type YulStaticKeyword_Unreserved = TerminalType;

pub fn new_yul_static_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStaticKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulStaticKeyword_Unreserved,
    }
}

pub type YulStorageKeyword_Reserved = TerminalType;

pub fn new_yul_storage_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStorageKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulStorageKeyword_Reserved,
    }
}

pub type YulStorageKeyword_Unreserved = TerminalType;

pub fn new_yul_storage_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStorageKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulStorageKeyword_Unreserved,
    }
}

pub type YulStringKeyword_Reserved = TerminalType;

pub fn new_yul_string_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStringKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulStringKeyword_Reserved,
    }
}

pub type YulStringKeyword_Unreserved = TerminalType;

pub fn new_yul_string_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStringKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulStringKeyword_Unreserved,
    }
}

pub type YulStructKeyword_Reserved = TerminalType;

pub fn new_yul_struct_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStructKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulStructKeyword_Reserved,
    }
}

pub type YulStructKeyword_Unreserved = TerminalType;

pub fn new_yul_struct_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStructKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulStructKeyword_Unreserved,
    }
}

pub type YulSuperKeyword_Reserved = TerminalType;

pub fn new_yul_super_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSuperKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSuperKeyword_Reserved,
    }
}

pub type YulSuperKeyword_Unreserved = TerminalType;

pub fn new_yul_super_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSuperKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSuperKeyword_Unreserved,
    }
}

pub type YulSupportsKeyword_Reserved = TerminalType;

pub fn new_yul_supports_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSupportsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSupportsKeyword_Reserved,
    }
}

pub type YulSupportsKeyword_Unreserved = TerminalType;

pub fn new_yul_supports_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSupportsKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSupportsKeyword_Unreserved,
    }
}

pub type YulSwitchKeyword_Reserved = TerminalType;

pub fn new_yul_switch_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSwitchKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSwitchKeyword_Reserved,
    }
}

pub type YulSzaboKeyword_Reserved = TerminalType;

pub fn new_yul_szabo_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSzaboKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSzaboKeyword_Reserved,
    }
}

pub type YulSzaboKeyword_Unreserved = TerminalType;

pub fn new_yul_szabo_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSzaboKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulSzaboKeyword_Unreserved,
    }
}

pub type YulThisKeyword_Reserved = TerminalType;

pub fn new_yul_this_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulThisKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulThisKeyword_Reserved,
    }
}

pub type YulThisKeyword_Unreserved = TerminalType;

pub fn new_yul_this_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulThisKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulThisKeyword_Unreserved,
    }
}

pub type YulThrowKeyword_Reserved = TerminalType;

pub fn new_yul_throw_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulThrowKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulThrowKeyword_Reserved,
    }
}

pub type YulThrowKeyword_Unreserved = TerminalType;

pub fn new_yul_throw_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulThrowKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulThrowKeyword_Unreserved,
    }
}

pub type YulTrueKeyword_Reserved = TerminalType;

pub fn new_yul_true_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTrueKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulTrueKeyword_Reserved,
    }
}

pub type YulTryKeyword_Reserved = TerminalType;

pub fn new_yul_try_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTryKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulTryKeyword_Reserved,
    }
}

pub type YulTryKeyword_Unreserved = TerminalType;

pub fn new_yul_try_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTryKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulTryKeyword_Unreserved,
    }
}

pub type YulTypeDefKeyword_Reserved = TerminalType;

pub fn new_yul_type_def_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeDefKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulTypeDefKeyword_Reserved,
    }
}

pub type YulTypeDefKeyword_Unreserved = TerminalType;

pub fn new_yul_type_def_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeDefKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulTypeDefKeyword_Unreserved,
    }
}

pub type YulTypeKeyword_Reserved = TerminalType;

pub fn new_yul_type_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulTypeKeyword_Reserved,
    }
}

pub type YulTypeKeyword_Unreserved = TerminalType;

pub fn new_yul_type_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulTypeKeyword_Unreserved,
    }
}

pub type YulTypeOfKeyword_Reserved = TerminalType;

pub fn new_yul_type_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeOfKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulTypeOfKeyword_Reserved,
    }
}

pub type YulTypeOfKeyword_Unreserved = TerminalType;

pub fn new_yul_type_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeOfKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulTypeOfKeyword_Unreserved,
    }
}

pub type YulUfixedKeyword_Reserved = TerminalType;

pub fn new_yul_ufixed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUfixedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulUfixedKeyword_Reserved,
    }
}

pub type YulUfixedKeyword_Unreserved = TerminalType;

pub fn new_yul_ufixed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUfixedKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulUfixedKeyword_Unreserved,
    }
}

pub type YulUintKeyword_Reserved = TerminalType;

pub fn new_yul_uint_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUintKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulUintKeyword_Reserved,
    }
}

pub type YulUintKeyword_Unreserved = TerminalType;

pub fn new_yul_uint_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUintKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulUintKeyword_Unreserved,
    }
}

pub type YulUncheckedKeyword_Reserved = TerminalType;

pub fn new_yul_unchecked_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUncheckedKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulUncheckedKeyword_Reserved,
    }
}

pub type YulUncheckedKeyword_Unreserved = TerminalType;

pub fn new_yul_unchecked_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUncheckedKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulUncheckedKeyword_Unreserved,
    }
}

pub type YulUsingKeyword_Reserved = TerminalType;

pub fn new_yul_using_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUsingKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulUsingKeyword_Reserved,
    }
}

pub type YulUsingKeyword_Unreserved = TerminalType;

pub fn new_yul_using_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUsingKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulUsingKeyword_Unreserved,
    }
}

pub type YulVarKeyword_Reserved = TerminalType;

pub fn new_yul_var_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulVarKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulVarKeyword_Reserved,
    }
}

pub type YulVarKeyword_Unreserved = TerminalType;

pub fn new_yul_var_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulVarKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulVarKeyword_Unreserved,
    }
}

pub type YulViewKeyword_Reserved = TerminalType;

pub fn new_yul_view_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulViewKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulViewKeyword_Reserved,
    }
}

pub type YulViewKeyword_Unreserved = TerminalType;

pub fn new_yul_view_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulViewKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulViewKeyword_Unreserved,
    }
}

pub type YulVirtualKeyword_Reserved = TerminalType;

pub fn new_yul_virtual_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulVirtualKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulVirtualKeyword_Reserved,
    }
}

pub type YulVirtualKeyword_Unreserved = TerminalType;

pub fn new_yul_virtual_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulVirtualKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulVirtualKeyword_Unreserved,
    }
}

pub type YulWeeksKeyword_Reserved = TerminalType;

pub fn new_yul_weeks_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWeeksKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulWeeksKeyword_Reserved,
    }
}

pub type YulWeeksKeyword_Unreserved = TerminalType;

pub fn new_yul_weeks_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWeeksKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulWeeksKeyword_Unreserved,
    }
}

pub type YulWeiKeyword_Reserved = TerminalType;

pub fn new_yul_wei_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWeiKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulWeiKeyword_Reserved,
    }
}

pub type YulWeiKeyword_Unreserved = TerminalType;

pub fn new_yul_wei_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWeiKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulWeiKeyword_Unreserved,
    }
}

pub type YulWhileKeyword_Reserved = TerminalType;

pub fn new_yul_while_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWhileKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulWhileKeyword_Reserved,
    }
}

pub type YulWhileKeyword_Unreserved = TerminalType;

pub fn new_yul_while_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWhileKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulWhileKeyword_Unreserved,
    }
}

pub type YulYearsKeyword_Reserved = TerminalType;

pub fn new_yul_years_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulYearsKeyword_Reserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulYearsKeyword_Reserved,
    }
}

pub type YulYearsKeyword_Unreserved = TerminalType;

pub fn new_yul_years_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulYearsKeyword_Unreserved {
    TerminalType {
        value: source[l..r].to_owned(),
        _l: l,
        _r: r,
        kind: LexemeKind::YulYearsKeyword_Unreserved,
    }
}
