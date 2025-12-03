// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

// TODO:
// - (correctness) use actual structs and types
// - (perf) don't use terminals that are not needed

//
// Sequences:
//

pub type SourceUnit = String;

pub fn new_source_unit(members: SourceUnitMembers) -> SourceUnit {
    format!(
        " {{  \"item\": \"Struct\", \"name\": \"SourceUnit\", \"members\": {},  }} ",
        members,
    )
}

pub type PragmaDirective = String;

pub fn new_pragma_directive(
    pragma_keyword: PragmaKeyword,
    pragma: Pragma,
    semicolon: Semicolon,
) -> PragmaDirective {
    format!(" {{  \"item\": \"Struct\", \"name\": \"PragmaDirective\", \"pragma_keyword\": {}, \"pragma\": {}, \"semicolon\": {},  }} ", pragma_keyword, pragma, semicolon, )
}

pub type AbicoderPragma = String;

pub fn new_abicoder_pragma(
    abicoder_keyword: AbicoderKeyword,
    version: AbicoderVersion,
) -> AbicoderPragma {
    format!(" {{  \"item\": \"Struct\", \"name\": \"AbicoderPragma\", \"abicoder_keyword\": {}, \"version\": {},  }} ", abicoder_keyword, version, )
}

pub type ExperimentalPragma = String;

pub fn new_experimental_pragma(
    experimental_keyword: ExperimentalKeyword,
    feature: ExperimentalFeature,
) -> ExperimentalPragma {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ExperimentalPragma\", \"experimental_keyword\": {}, \"feature\": {},  }} ", experimental_keyword, feature, )
}

pub type VersionPragma = String;

pub fn new_version_pragma(
    solidity_keyword: SolidityKeyword,
    sets: VersionExpressionSets,
) -> VersionPragma {
    format!(" {{  \"item\": \"Struct\", \"name\": \"VersionPragma\", \"solidity_keyword\": {}, \"sets\": {},  }} ", solidity_keyword, sets, )
}

pub type VersionRange = String;

pub fn new_version_range(start: VersionLiteral, minus: Minus, end: VersionLiteral) -> VersionRange {
    format!(" {{  \"item\": \"Struct\", \"name\": \"VersionRange\", \"start\": {}, \"minus\": {}, \"end\": {},  }} ", start, minus, end, )
}

pub type VersionTerm = String;

pub fn new_version_term(operator: Option<VersionOperator>, literal: VersionLiteral) -> VersionTerm {
    format!(" {{  \"item\": \"Struct\", \"name\": \"VersionTerm\", \"operator\": {}, \"literal\": {},  }} ", operator.unwrap_or("".into()), literal, )
}

pub type ImportDirective = String;

pub fn new_import_directive(
    import_keyword: ImportKeyword,
    clause: ImportClause,
    semicolon: Semicolon,
) -> ImportDirective {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ImportDirective\", \"import_keyword\": {}, \"clause\": {}, \"semicolon\": {},  }} ", import_keyword, clause, semicolon, )
}

pub type PathImport = String;

pub fn new_path_import(path: StringLiteral, alias: Option<ImportAlias>) -> PathImport {
    format!(
        " {{  \"item\": \"Struct\", \"name\": \"PathImport\", \"path\": {}, \"alias\": {},  }} ",
        path,
        alias.unwrap_or("".into()),
    )
}

pub type NamedImport = String;

pub fn new_named_import(
    asterisk: Asterisk,
    alias: ImportAlias,
    from_keyword: FromKeyword,
    path: StringLiteral,
) -> NamedImport {
    format!(" {{  \"item\": \"Struct\", \"name\": \"NamedImport\", \"asterisk\": {}, \"alias\": {}, \"from_keyword\": {}, \"path\": {},  }} ", asterisk, alias, from_keyword, path, )
}

pub type ImportDeconstruction = String;

pub fn new_import_deconstruction(
    open_brace: OpenBrace,
    symbols: ImportDeconstructionSymbols,
    close_brace: CloseBrace,
    from_keyword: FromKeyword,
    path: StringLiteral,
) -> ImportDeconstruction {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ImportDeconstruction\", \"open_brace\": {}, \"symbols\": {}, \"close_brace\": {}, \"from_keyword\": {}, \"path\": {},  }} ", open_brace, symbols, close_brace, from_keyword, path, )
}

pub type ImportDeconstructionSymbol = String;

pub fn new_import_deconstruction_symbol(
    name: Identifier,
    alias: Option<ImportAlias>,
) -> ImportDeconstructionSymbol {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ImportDeconstructionSymbol\", \"name\": {}, \"alias\": {},  }} ", name, alias.unwrap_or("".into()), )
}

pub type ImportAlias = String;

pub fn new_import_alias(as_keyword: AsKeyword, identifier: Identifier) -> ImportAlias {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ImportAlias\", \"as_keyword\": {}, \"identifier\": {},  }} ", as_keyword, identifier, )
}

pub type UsingDirective = String;

pub fn new_using_directive(
    using_keyword: UsingKeyword,
    clause: UsingClause,
    for_keyword: ForKeyword,
    target: UsingTarget,
    global_keyword: Option<GlobalKeyword>,
    semicolon: Semicolon,
) -> UsingDirective {
    format!(" {{  \"item\": \"Struct\", \"name\": \"UsingDirective\", \"using_keyword\": {}, \"clause\": {}, \"for_keyword\": {}, \"target\": {}, \"global_keyword\": {}, \"semicolon\": {},  }} ", using_keyword, clause, for_keyword, target, global_keyword.unwrap_or("".into()), semicolon, )
}

pub type UsingDeconstruction = String;

pub fn new_using_deconstruction(
    open_brace: OpenBrace,
    symbols: UsingDeconstructionSymbols,
    close_brace: CloseBrace,
) -> UsingDeconstruction {
    format!(" {{  \"item\": \"Struct\", \"name\": \"UsingDeconstruction\", \"open_brace\": {}, \"symbols\": {}, \"close_brace\": {},  }} ", open_brace, symbols, close_brace, )
}

pub type UsingDeconstructionSymbol = String;

pub fn new_using_deconstruction_symbol(
    name: IdentifierPath,
    alias: Option<UsingAlias>,
) -> UsingDeconstructionSymbol {
    format!(" {{  \"item\": \"Struct\", \"name\": \"UsingDeconstructionSymbol\", \"name\": {}, \"alias\": {},  }} ", name, alias.unwrap_or("".into()), )
}

pub type UsingAlias = String;

pub fn new_using_alias(as_keyword: AsKeyword, operator: UsingOperator) -> UsingAlias {
    format!(" {{  \"item\": \"Struct\", \"name\": \"UsingAlias\", \"as_keyword\": {}, \"operator\": {},  }} ", as_keyword, operator, )
}

pub type ContractDefinition = String;

pub fn new_contract_definition(
    abstract_keyword: Option<AbstractKeyword>,
    contract_keyword: ContractKeyword,
    name: Identifier,
    specifiers: ContractSpecifiers,
    open_brace: OpenBrace,
    members: ContractMembers,
    close_brace: CloseBrace,
) -> ContractDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ContractDefinition\", \"abstract_keyword\": {}, \"contract_keyword\": {}, \"name\": {}, \"specifiers\": {}, \"open_brace\": {}, \"members\": {}, \"close_brace\": {},  }} ", abstract_keyword.unwrap_or("".into()), contract_keyword, name, specifiers, open_brace, members, close_brace, )
}

pub type InheritanceSpecifier = String;

pub fn new_inheritance_specifier(
    is_keyword: IsKeyword,
    types: InheritanceTypes,
) -> InheritanceSpecifier {
    format!(" {{  \"item\": \"Struct\", \"name\": \"InheritanceSpecifier\", \"is_keyword\": {}, \"types\": {},  }} ", is_keyword, types, )
}

pub type InheritanceType = String;

pub fn new_inheritance_type(
    type_name: IdentifierPath,
    arguments: Option<ArgumentsDeclaration>,
) -> InheritanceType {
    format!(" {{  \"item\": \"Struct\", \"name\": \"InheritanceType\", \"type_name\": {}, \"arguments\": {},  }} ", type_name, arguments.unwrap_or("".into()), )
}

pub type StorageLayoutSpecifier = String;

pub fn new_storage_layout_specifier(
    layout_keyword: LayoutKeyword,
    at_keyword: AtKeyword,
    expression: Expression,
) -> StorageLayoutSpecifier {
    format!(" {{  \"item\": \"Struct\", \"name\": \"StorageLayoutSpecifier\", \"layout_keyword\": {}, \"at_keyword\": {}, \"expression\": {},  }} ", layout_keyword, at_keyword, expression, )
}

pub type InterfaceDefinition = String;

pub fn new_interface_definition(
    interface_keyword: InterfaceKeyword,
    name: Identifier,
    inheritance: Option<InheritanceSpecifier>,
    open_brace: OpenBrace,
    members: InterfaceMembers,
    close_brace: CloseBrace,
) -> InterfaceDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"InterfaceDefinition\", \"interface_keyword\": {}, \"name\": {}, \"inheritance\": {}, \"open_brace\": {}, \"members\": {}, \"close_brace\": {},  }} ", interface_keyword, name, inheritance.unwrap_or("".into()), open_brace, members, close_brace, )
}

pub type LibraryDefinition = String;

pub fn new_library_definition(
    library_keyword: LibraryKeyword,
    name: Identifier,
    open_brace: OpenBrace,
    members: LibraryMembers,
    close_brace: CloseBrace,
) -> LibraryDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"LibraryDefinition\", \"library_keyword\": {}, \"name\": {}, \"open_brace\": {}, \"members\": {}, \"close_brace\": {},  }} ", library_keyword, name, open_brace, members, close_brace, )
}

pub type StructDefinition = String;

pub fn new_struct_definition(
    struct_keyword: StructKeyword,
    name: Identifier,
    open_brace: OpenBrace,
    members: StructMembers,
    close_brace: CloseBrace,
) -> StructDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"StructDefinition\", \"struct_keyword\": {}, \"name\": {}, \"open_brace\": {}, \"members\": {}, \"close_brace\": {},  }} ", struct_keyword, name, open_brace, members, close_brace, )
}

pub type StructMember = String;

pub fn new_struct_member(
    type_name: TypeName,
    name: Identifier,
    semicolon: Semicolon,
) -> StructMember {
    format!(" {{  \"item\": \"Struct\", \"name\": \"StructMember\", \"type_name\": {}, \"name\": {}, \"semicolon\": {},  }} ", type_name, name, semicolon, )
}

pub type EnumDefinition = String;

pub fn new_enum_definition(
    enum_keyword: EnumKeyword,
    name: Identifier,
    open_brace: OpenBrace,
    members: EnumMembers,
    close_brace: CloseBrace,
) -> EnumDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"EnumDefinition\", \"enum_keyword\": {}, \"name\": {}, \"open_brace\": {}, \"members\": {}, \"close_brace\": {},  }} ", enum_keyword, name, open_brace, members, close_brace, )
}

pub type ConstantDefinition = String;

pub fn new_constant_definition(
    type_name: TypeName,
    constant_keyword: ConstantKeyword,
    name: Identifier,
    equal: Equal,
    value: Expression,
    semicolon: Semicolon,
) -> ConstantDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ConstantDefinition\", \"type_name\": {}, \"constant_keyword\": {}, \"name\": {}, \"equal\": {}, \"value\": {}, \"semicolon\": {},  }} ", type_name, constant_keyword, name, equal, value, semicolon, )
}

pub type StateVariableDefinition = String;

pub fn new_state_variable_definition(
    type_name: TypeName,
    attributes: StateVariableAttributes,
    name: Identifier,
    value: Option<StateVariableDefinitionValue>,
    semicolon: Semicolon,
) -> StateVariableDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"StateVariableDefinition\", \"type_name\": {}, \"attributes\": {}, \"name\": {}, \"value\": {}, \"semicolon\": {},  }} ", type_name, attributes, name, value.unwrap_or("".into()), semicolon, )
}

pub type StateVariableDefinitionValue = String;

pub fn new_state_variable_definition_value(
    equal: Equal,
    value: Expression,
) -> StateVariableDefinitionValue {
    format!(" {{  \"item\": \"Struct\", \"name\": \"StateVariableDefinitionValue\", \"equal\": {}, \"value\": {},  }} ", equal, value, )
}

pub type FunctionDefinition = String;

pub fn new_function_definition(
    function_keyword: FunctionKeyword,
    name: FunctionName,
    parameters: ParametersDeclaration,
    attributes: FunctionAttributes,
    returns: Option<ReturnsDeclaration>,
    body: FunctionBody,
) -> FunctionDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"FunctionDefinition\", \"function_keyword\": {}, \"name\": {}, \"parameters\": {}, \"attributes\": {}, \"returns\": {}, \"body\": {},  }} ", function_keyword, name, parameters, attributes, returns.unwrap_or("".into()), body, )
}

pub type ParametersDeclaration = String;

pub fn new_parameters_declaration(
    open_paren: OpenParen,
    parameters: Parameters,
    close_paren: CloseParen,
) -> ParametersDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ParametersDeclaration\", \"open_paren\": {}, \"parameters\": {}, \"close_paren\": {},  }} ", open_paren, parameters, close_paren, )
}

pub type Parameter = String;

pub fn new_parameter(
    type_name: TypeName,
    storage_location: Option<StorageLocation>,
    name: Option<Identifier>,
) -> Parameter {
    format!(" {{  \"item\": \"Struct\", \"name\": \"Parameter\", \"type_name\": {}, \"storage_location\": {}, \"name\": {},  }} ", type_name, storage_location.unwrap_or("".into()), name.unwrap_or("".into()), )
}

pub type OverrideSpecifier = String;

pub fn new_override_specifier(
    override_keyword: OverrideKeyword,
    overridden: Option<OverridePathsDeclaration>,
) -> OverrideSpecifier {
    format!(" {{  \"item\": \"Struct\", \"name\": \"OverrideSpecifier\", \"override_keyword\": {}, \"overridden\": {},  }} ", override_keyword, overridden.unwrap_or("".into()), )
}

pub type OverridePathsDeclaration = String;

pub fn new_override_paths_declaration(
    open_paren: OpenParen,
    paths: OverridePaths,
    close_paren: CloseParen,
) -> OverridePathsDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"OverridePathsDeclaration\", \"open_paren\": {}, \"paths\": {}, \"close_paren\": {},  }} ", open_paren, paths, close_paren, )
}

pub type ReturnsDeclaration = String;

pub fn new_returns_declaration(
    returns_keyword: ReturnsKeyword,
    variables: ParametersDeclaration,
) -> ReturnsDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ReturnsDeclaration\", \"returns_keyword\": {}, \"variables\": {},  }} ", returns_keyword, variables, )
}

pub type ConstructorDefinition = String;

pub fn new_constructor_definition(
    constructor_keyword: ConstructorKeyword,
    parameters: ParametersDeclaration,
    attributes: ConstructorAttributes,
    body: Block,
) -> ConstructorDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ConstructorDefinition\", \"constructor_keyword\": {}, \"parameters\": {}, \"attributes\": {}, \"body\": {},  }} ", constructor_keyword, parameters, attributes, body, )
}

pub type UnnamedFunctionDefinition = String;

pub fn new_unnamed_function_definition(
    function_keyword: FunctionKeyword,
    parameters: ParametersDeclaration,
    attributes: UnnamedFunctionAttributes,
    body: FunctionBody,
) -> UnnamedFunctionDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"UnnamedFunctionDefinition\", \"function_keyword\": {}, \"parameters\": {}, \"attributes\": {}, \"body\": {},  }} ", function_keyword, parameters, attributes, body, )
}

pub type FallbackFunctionDefinition = String;

pub fn new_fallback_function_definition(
    fallback_keyword: FallbackKeyword,
    parameters: ParametersDeclaration,
    attributes: FallbackFunctionAttributes,
    returns: Option<ReturnsDeclaration>,
    body: FunctionBody,
) -> FallbackFunctionDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"FallbackFunctionDefinition\", \"fallback_keyword\": {}, \"parameters\": {}, \"attributes\": {}, \"returns\": {}, \"body\": {},  }} ", fallback_keyword, parameters, attributes, returns.unwrap_or("".into()), body, )
}

pub type ReceiveFunctionDefinition = String;

pub fn new_receive_function_definition(
    receive_keyword: ReceiveKeyword,
    parameters: ParametersDeclaration,
    attributes: ReceiveFunctionAttributes,
    body: FunctionBody,
) -> ReceiveFunctionDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ReceiveFunctionDefinition\", \"receive_keyword\": {}, \"parameters\": {}, \"attributes\": {}, \"body\": {},  }} ", receive_keyword, parameters, attributes, body, )
}

pub type ModifierDefinition = String;

pub fn new_modifier_definition(
    modifier_keyword: ModifierKeyword,
    name: Identifier,
    parameters: Option<ParametersDeclaration>,
    attributes: ModifierAttributes,
    body: FunctionBody,
) -> ModifierDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ModifierDefinition\", \"modifier_keyword\": {}, \"name\": {}, \"parameters\": {}, \"attributes\": {}, \"body\": {},  }} ", modifier_keyword, name, parameters.unwrap_or("".into()), attributes, body, )
}

pub type ModifierInvocation = String;

pub fn new_modifier_invocation(
    name: IdentifierPath,
    arguments: Option<ArgumentsDeclaration>,
) -> ModifierInvocation {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ModifierInvocation\", \"name\": {}, \"arguments\": {},  }} ", name, arguments.unwrap_or("".into()), )
}

pub type EventDefinition = String;

pub fn new_event_definition(
    event_keyword: EventKeyword,
    name: Identifier,
    parameters: EventParametersDeclaration,
    anonymous_keyword: Option<AnonymousKeyword>,
    semicolon: Semicolon,
) -> EventDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"EventDefinition\", \"event_keyword\": {}, \"name\": {}, \"parameters\": {}, \"anonymous_keyword\": {}, \"semicolon\": {},  }} ", event_keyword, name, parameters, anonymous_keyword.unwrap_or("".into()), semicolon, )
}

pub type EventParametersDeclaration = String;

pub fn new_event_parameters_declaration(
    open_paren: OpenParen,
    parameters: EventParameters,
    close_paren: CloseParen,
) -> EventParametersDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"EventParametersDeclaration\", \"open_paren\": {}, \"parameters\": {}, \"close_paren\": {},  }} ", open_paren, parameters, close_paren, )
}

pub type EventParameter = String;

pub fn new_event_parameter(
    type_name: TypeName,
    indexed_keyword: Option<IndexedKeyword>,
    name: Option<Identifier>,
) -> EventParameter {
    format!(" {{  \"item\": \"Struct\", \"name\": \"EventParameter\", \"type_name\": {}, \"indexed_keyword\": {}, \"name\": {},  }} ", type_name, indexed_keyword.unwrap_or("".into()), name.unwrap_or("".into()), )
}

pub type UserDefinedValueTypeDefinition = String;

pub fn new_user_defined_value_type_definition(
    type_keyword: TypeKeyword,
    name: Identifier,
    is_keyword: IsKeyword,
    value_type: ElementaryType,
    semicolon: Semicolon,
) -> UserDefinedValueTypeDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"UserDefinedValueTypeDefinition\", \"type_keyword\": {}, \"name\": {}, \"is_keyword\": {}, \"value_type\": {}, \"semicolon\": {},  }} ", type_keyword, name, is_keyword, value_type, semicolon, )
}

pub type ErrorDefinition = String;

pub fn new_error_definition(
    error_keyword: ErrorKeyword,
    name: Identifier,
    members: ErrorParametersDeclaration,
    semicolon: Semicolon,
) -> ErrorDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ErrorDefinition\", \"error_keyword\": {}, \"name\": {}, \"members\": {}, \"semicolon\": {},  }} ", error_keyword, name, members, semicolon, )
}

pub type ErrorParametersDeclaration = String;

pub fn new_error_parameters_declaration(
    open_paren: OpenParen,
    parameters: ErrorParameters,
    close_paren: CloseParen,
) -> ErrorParametersDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ErrorParametersDeclaration\", \"open_paren\": {}, \"parameters\": {}, \"close_paren\": {},  }} ", open_paren, parameters, close_paren, )
}

pub type ErrorParameter = String;

pub fn new_error_parameter(type_name: TypeName, name: Option<Identifier>) -> ErrorParameter {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ErrorParameter\", \"type_name\": {}, \"name\": {},  }} ", type_name, name.unwrap_or("".into()), )
}

pub type ArrayTypeName = String;

pub fn new_array_type_name(
    operand: TypeName,
    open_bracket: OpenBracket,
    index: Option<Expression>,
    close_bracket: CloseBracket,
) -> ArrayTypeName {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ArrayTypeName\", \"operand\": {}, \"open_bracket\": {}, \"index\": {}, \"close_bracket\": {},  }} ", operand, open_bracket, index.unwrap_or("".into()), close_bracket, )
}

pub type NewableArrayType = String;

pub fn new_newable_array_type(
    type_name: TypeName,
    open_bracket: OpenBracket,
    close_bracket: CloseBracket,
) -> NewableArrayType {
    format!(" {{  \"item\": \"Struct\", \"name\": \"NewableArrayType\", \"type_name\": {}, \"open_bracket\": {}, \"close_bracket\": {},  }} ", type_name, open_bracket, close_bracket, )
}

pub type FunctionType = String;

pub fn new_function_type(
    function_keyword: FunctionKeyword,
    parameters: ParametersDeclaration,
    attributes: FunctionTypeAttributes,
    returns: Option<ReturnsDeclaration>,
) -> FunctionType {
    format!(" {{  \"item\": \"Struct\", \"name\": \"FunctionType\", \"function_keyword\": {}, \"parameters\": {}, \"attributes\": {}, \"returns\": {},  }} ", function_keyword, parameters, attributes, returns.unwrap_or("".into()), )
}

pub type MappingType = String;

pub fn new_mapping_type(
    mapping_keyword: MappingKeyword,
    open_paren: OpenParen,
    key_type: MappingKey,
    equal_greater_than: EqualGreaterThan,
    value_type: MappingValue,
    close_paren: CloseParen,
) -> MappingType {
    format!(" {{  \"item\": \"Struct\", \"name\": \"MappingType\", \"mapping_keyword\": {}, \"open_paren\": {}, \"key_type\": {}, \"equal_greater_than\": {}, \"value_type\": {}, \"close_paren\": {},  }} ", mapping_keyword, open_paren, key_type, equal_greater_than, value_type, close_paren, )
}

pub type MappingKey = String;

pub fn new_mapping_key(key_type: MappingKeyType, name: Option<Identifier>) -> MappingKey {
    format!(
        " {{  \"item\": \"Struct\", \"name\": \"MappingKey\", \"key_type\": {}, \"name\": {},  }} ",
        key_type,
        name.unwrap_or("".into()),
    )
}

pub type MappingValue = String;

pub fn new_mapping_value(type_name: TypeName, name: Option<Identifier>) -> MappingValue {
    format!(" {{  \"item\": \"Struct\", \"name\": \"MappingValue\", \"type_name\": {}, \"name\": {},  }} ", type_name, name.unwrap_or("".into()), )
}

pub type AddressType = String;

pub fn new_address_type(
    address_keyword: AddressKeyword,
    payable_keyword: Option<PayableKeyword>,
) -> AddressType {
    format!(" {{  \"item\": \"Struct\", \"name\": \"AddressType\", \"address_keyword\": {}, \"payable_keyword\": {},  }} ", address_keyword, payable_keyword.unwrap_or("".into()), )
}

pub type Block = String;

pub fn new_block(open_brace: OpenBrace, statements: Statements, close_brace: CloseBrace) -> Block {
    format!(" {{  \"item\": \"Struct\", \"name\": \"Block\", \"open_brace\": {}, \"statements\": {}, \"close_brace\": {},  }} ", open_brace, statements, close_brace, )
}

pub type UncheckedBlock = String;

pub fn new_unchecked_block(unchecked_keyword: UncheckedKeyword, block: Block) -> UncheckedBlock {
    format!(" {{  \"item\": \"Struct\", \"name\": \"UncheckedBlock\", \"unchecked_keyword\": {}, \"block\": {},  }} ", unchecked_keyword, block, )
}

pub type ExpressionStatement = String;

pub fn new_expression_statement(
    expression: Expression,
    semicolon: Semicolon,
) -> ExpressionStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ExpressionStatement\", \"expression\": {}, \"semicolon\": {},  }} ", expression, semicolon, )
}

pub type AssemblyStatement = String;

pub fn new_assembly_statement(
    assembly_keyword: AssemblyKeyword,
    label: Option<StringLiteral>,
    flags: Option<AssemblyFlagsDeclaration>,
    body: YulBlock,
) -> AssemblyStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"AssemblyStatement\", \"assembly_keyword\": {}, \"label\": {}, \"flags\": {}, \"body\": {},  }} ", assembly_keyword, label.unwrap_or("".into()), flags.unwrap_or("".into()), body, )
}

pub type AssemblyFlagsDeclaration = String;

pub fn new_assembly_flags_declaration(
    open_paren: OpenParen,
    flags: AssemblyFlags,
    close_paren: CloseParen,
) -> AssemblyFlagsDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"AssemblyFlagsDeclaration\", \"open_paren\": {}, \"flags\": {}, \"close_paren\": {},  }} ", open_paren, flags, close_paren, )
}

pub type TupleDeconstructionStatement = String;

pub fn new_tuple_deconstruction_statement(
    var_keyword: Option<VarKeyword>,
    open_paren: OpenParen,
    elements: TupleDeconstructionElements,
    close_paren: CloseParen,
    equal: Equal,
    expression: Expression,
    semicolon: Semicolon,
) -> TupleDeconstructionStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"TupleDeconstructionStatement\", \"var_keyword\": {}, \"open_paren\": {}, \"elements\": {}, \"close_paren\": {}, \"equal\": {}, \"expression\": {}, \"semicolon\": {},  }} ", var_keyword.unwrap_or("".into()), open_paren, elements, close_paren, equal, expression, semicolon, )
}

pub type TupleDeconstructionElement = String;

pub fn new_tuple_deconstruction_element(member: Option<TupleMember>) -> TupleDeconstructionElement {
    format!(
        " {{  \"item\": \"Struct\", \"name\": \"TupleDeconstructionElement\", \"member\": {},  }} ",
        member.unwrap_or("".into()),
    )
}

pub type TypedTupleMember = String;

pub fn new_typed_tuple_member(
    type_name: TypeName,
    storage_location: Option<StorageLocation>,
    name: Identifier,
) -> TypedTupleMember {
    format!(" {{  \"item\": \"Struct\", \"name\": \"TypedTupleMember\", \"type_name\": {}, \"storage_location\": {}, \"name\": {},  }} ", type_name, storage_location.unwrap_or("".into()), name, )
}

pub type UntypedTupleMember = String;

pub fn new_untyped_tuple_member(
    storage_location: Option<StorageLocation>,
    name: Identifier,
) -> UntypedTupleMember {
    format!(" {{  \"item\": \"Struct\", \"name\": \"UntypedTupleMember\", \"storage_location\": {}, \"name\": {},  }} ", storage_location.unwrap_or("".into()), name, )
}

pub type VariableDeclarationStatement = String;

pub fn new_variable_declaration_statement(
    variable_type: VariableDeclarationType,
    storage_location: Option<StorageLocation>,
    name: Identifier,
    value: Option<VariableDeclarationValue>,
    semicolon: Semicolon,
) -> VariableDeclarationStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"VariableDeclarationStatement\", \"variable_type\": {}, \"storage_location\": {}, \"name\": {}, \"value\": {}, \"semicolon\": {},  }} ", variable_type, storage_location.unwrap_or("".into()), name, value.unwrap_or("".into()), semicolon, )
}

pub type VariableDeclarationValue = String;

pub fn new_variable_declaration_value(
    equal: Equal,
    expression: Expression,
) -> VariableDeclarationValue {
    format!(" {{  \"item\": \"Struct\", \"name\": \"VariableDeclarationValue\", \"equal\": {}, \"expression\": {},  }} ", equal, expression, )
}

pub type IfStatement = String;

pub fn new_if_statement(
    if_keyword: IfKeyword,
    open_paren: OpenParen,
    condition: Expression,
    close_paren: CloseParen,
    body: Statement,
    else_branch: Option<ElseBranch>,
) -> IfStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"IfStatement\", \"if_keyword\": {}, \"open_paren\": {}, \"condition\": {}, \"close_paren\": {}, \"body\": {}, \"else_branch\": {},  }} ", if_keyword, open_paren, condition, close_paren, body, else_branch.unwrap_or("".into()), )
}

pub type ElseBranch = String;

pub fn new_else_branch(else_keyword: ElseKeyword, body: Statement) -> ElseBranch {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ElseBranch\", \"else_keyword\": {}, \"body\": {},  }} ", else_keyword, body, )
}

pub type ForStatement = String;

pub fn new_for_statement(
    for_keyword: ForKeyword,
    open_paren: OpenParen,
    initialization: ForStatementInitialization,
    condition: ForStatementCondition,
    iterator: Option<Expression>,
    close_paren: CloseParen,
    body: Statement,
) -> ForStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ForStatement\", \"for_keyword\": {}, \"open_paren\": {}, \"initialization\": {}, \"condition\": {}, \"iterator\": {}, \"close_paren\": {}, \"body\": {},  }} ", for_keyword, open_paren, initialization, condition, iterator.unwrap_or("".into()), close_paren, body, )
}

pub type WhileStatement = String;

pub fn new_while_statement(
    while_keyword: WhileKeyword,
    open_paren: OpenParen,
    condition: Expression,
    close_paren: CloseParen,
    body: Statement,
) -> WhileStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"WhileStatement\", \"while_keyword\": {}, \"open_paren\": {}, \"condition\": {}, \"close_paren\": {}, \"body\": {},  }} ", while_keyword, open_paren, condition, close_paren, body, )
}

pub type DoWhileStatement = String;

pub fn new_do_while_statement(
    do_keyword: DoKeyword,
    body: Statement,
    while_keyword: WhileKeyword,
    open_paren: OpenParen,
    condition: Expression,
    close_paren: CloseParen,
    semicolon: Semicolon,
) -> DoWhileStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"DoWhileStatement\", \"do_keyword\": {}, \"body\": {}, \"while_keyword\": {}, \"open_paren\": {}, \"condition\": {}, \"close_paren\": {}, \"semicolon\": {},  }} ", do_keyword, body, while_keyword, open_paren, condition, close_paren, semicolon, )
}

pub type ContinueStatement = String;

pub fn new_continue_statement(
    continue_keyword: ContinueKeyword,
    semicolon: Semicolon,
) -> ContinueStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ContinueStatement\", \"continue_keyword\": {}, \"semicolon\": {},  }} ", continue_keyword, semicolon, )
}

pub type BreakStatement = String;

pub fn new_break_statement(break_keyword: BreakKeyword, semicolon: Semicolon) -> BreakStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"BreakStatement\", \"break_keyword\": {}, \"semicolon\": {},  }} ", break_keyword, semicolon, )
}

pub type ReturnStatement = String;

pub fn new_return_statement(
    return_keyword: ReturnKeyword,
    expression: Option<Expression>,
    semicolon: Semicolon,
) -> ReturnStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ReturnStatement\", \"return_keyword\": {}, \"expression\": {}, \"semicolon\": {},  }} ", return_keyword, expression.unwrap_or("".into()), semicolon, )
}

pub type EmitStatement = String;

pub fn new_emit_statement(
    emit_keyword: EmitKeyword,
    event: IdentifierPath,
    arguments: ArgumentsDeclaration,
    semicolon: Semicolon,
) -> EmitStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"EmitStatement\", \"emit_keyword\": {}, \"event\": {}, \"arguments\": {}, \"semicolon\": {},  }} ", emit_keyword, event, arguments, semicolon, )
}

pub type TryStatement = String;

pub fn new_try_statement(
    try_keyword: TryKeyword,
    expression: Expression,
    returns: Option<ReturnsDeclaration>,
    body: Block,
    catch_clauses: CatchClauses,
) -> TryStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"TryStatement\", \"try_keyword\": {}, \"expression\": {}, \"returns\": {}, \"body\": {}, \"catch_clauses\": {},  }} ", try_keyword, expression, returns.unwrap_or("".into()), body, catch_clauses, )
}

pub type CatchClause = String;

pub fn new_catch_clause(
    catch_keyword: CatchKeyword,
    error: Option<CatchClauseError>,
    body: Block,
) -> CatchClause {
    format!(" {{  \"item\": \"Struct\", \"name\": \"CatchClause\", \"catch_keyword\": {}, \"error\": {}, \"body\": {},  }} ", catch_keyword, error.unwrap_or("".into()), body, )
}

pub type CatchClauseError = String;

pub fn new_catch_clause_error(
    name: Option<Identifier>,
    parameters: ParametersDeclaration,
) -> CatchClauseError {
    format!(" {{  \"item\": \"Struct\", \"name\": \"CatchClauseError\", \"name\": {}, \"parameters\": {},  }} ", name.unwrap_or("".into()), parameters, )
}

pub type RevertStatement = String;

pub fn new_revert_statement(
    revert_keyword: RevertKeyword,
    error: Option<IdentifierPath>,
    arguments: ArgumentsDeclaration,
    semicolon: Semicolon,
) -> RevertStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"RevertStatement\", \"revert_keyword\": {}, \"error\": {}, \"arguments\": {}, \"semicolon\": {},  }} ", revert_keyword, error.unwrap_or("".into()), arguments, semicolon, )
}

pub type ThrowStatement = String;

pub fn new_throw_statement(throw_keyword: ThrowKeyword, semicolon: Semicolon) -> ThrowStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ThrowStatement\", \"throw_keyword\": {}, \"semicolon\": {},  }} ", throw_keyword, semicolon, )
}

pub type AssignmentExpression = String;

pub fn new_assignment_expression(
    left_operand: Expression,
    operator: Equal,
    right_operand: Expression,
) -> AssignmentExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"AssignmentExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type ConditionalExpression = String;

pub fn new_conditional_expression(
    operand: Expression,
    question_mark: QuestionMark,
    true_expression: Expression,
    colon: Colon,
    false_expression: Expression,
) -> ConditionalExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ConditionalExpression\", \"operand\": {}, \"question_mark\": {}, \"true_expression\": {}, \"colon\": {}, \"false_expression\": {},  }} ", operand, question_mark, true_expression, colon, false_expression, )
}

pub type OrExpression = String;

pub fn new_or_expression(
    left_operand: Expression,
    operator: BarBar,
    right_operand: Expression,
) -> OrExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"OrExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type AndExpression = String;

pub fn new_and_expression(
    left_operand: Expression,
    operator: AmpersandAmpersand,
    right_operand: Expression,
) -> AndExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"AndExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type EqualityExpression = String;

pub fn new_equality_expression(
    left_operand: Expression,
    operator: EqualEqual,
    right_operand: Expression,
) -> EqualityExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"EqualityExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type InequalityExpression = String;

pub fn new_inequality_expression(
    left_operand: Expression,
    operator: LessThan,
    right_operand: Expression,
) -> InequalityExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"InequalityExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type BitwiseOrExpression = String;

pub fn new_bitwise_or_expression(
    left_operand: Expression,
    operator: Bar,
    right_operand: Expression,
) -> BitwiseOrExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"BitwiseOrExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type BitwiseXorExpression = String;

pub fn new_bitwise_xor_expression(
    left_operand: Expression,
    operator: Caret,
    right_operand: Expression,
) -> BitwiseXorExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"BitwiseXorExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type BitwiseAndExpression = String;

pub fn new_bitwise_and_expression(
    left_operand: Expression,
    operator: Ampersand,
    right_operand: Expression,
) -> BitwiseAndExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"BitwiseAndExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type ShiftExpression = String;

pub fn new_shift_expression(
    left_operand: Expression,
    operator: LessThanLessThan,
    right_operand: Expression,
) -> ShiftExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ShiftExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type AdditiveExpression = String;

pub fn new_additive_expression(
    left_operand: Expression,
    operator: Plus,
    right_operand: Expression,
) -> AdditiveExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"AdditiveExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type MultiplicativeExpression = String;

pub fn new_multiplicative_expression(
    left_operand: Expression,
    operator: Asterisk,
    right_operand: Expression,
) -> MultiplicativeExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"MultiplicativeExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type ExponentiationExpression = String;

pub fn new_exponentiation_expression(
    left_operand: Expression,
    operator: AsteriskAsterisk,
    right_operand: Expression,
) -> ExponentiationExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ExponentiationExpression\", \"left_operand\": {}, \"operator\": {}, \"right_operand\": {},  }} ", left_operand, operator, right_operand, )
}

pub type PostfixExpression = String;

pub fn new_postfix_expression(operand: Expression, operator: PlusPlus) -> PostfixExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"PostfixExpression\", \"operand\": {}, \"operator\": {},  }} ", operand, operator, )
}

pub type PrefixExpression = String;

pub fn new_prefix_expression(operator: PlusPlus, operand: Expression) -> PrefixExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"PrefixExpression\", \"operator\": {}, \"operand\": {},  }} ", operator, operand, )
}

pub type FunctionCallExpression = String;

pub fn new_function_call_expression(
    operand: Expression,
    arguments: ArgumentsDeclaration,
) -> FunctionCallExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"FunctionCallExpression\", \"operand\": {}, \"arguments\": {},  }} ", operand, arguments, )
}

pub type CallOptionsExpression = String;

pub fn new_call_options_expression(
    operand: Expression,
    open_brace: OpenBrace,
    options: CallOptions,
    close_brace: CloseBrace,
) -> CallOptionsExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"CallOptionsExpression\", \"operand\": {}, \"open_brace\": {}, \"options\": {}, \"close_brace\": {},  }} ", operand, open_brace, options, close_brace, )
}

pub type MemberAccessExpression = String;

pub fn new_member_access_expression(
    operand: Expression,
    period: Period,
    member: Identifier,
) -> MemberAccessExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"MemberAccessExpression\", \"operand\": {}, \"period\": {}, \"member\": {},  }} ", operand, period, member, )
}

pub type IndexAccessExpression = String;

pub fn new_index_access_expression(
    operand: Expression,
    open_bracket: OpenBracket,
    start: Option<Expression>,
    end: Option<IndexAccessEnd>,
    close_bracket: CloseBracket,
) -> IndexAccessExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"IndexAccessExpression\", \"operand\": {}, \"open_bracket\": {}, \"start\": {}, \"end\": {}, \"close_bracket\": {},  }} ", operand, open_bracket, start.unwrap_or("".into()), end.unwrap_or("".into()), close_bracket, )
}

pub type IndexAccessEnd = String;

pub fn new_index_access_end(colon: Colon, end: Option<Expression>) -> IndexAccessEnd {
    format!(
        " {{  \"item\": \"Struct\", \"name\": \"IndexAccessEnd\", \"colon\": {}, \"end\": {},  }} ",
        colon,
        end.unwrap_or("".into()),
    )
}

pub type PositionalArgumentsDeclaration = String;

pub fn new_positional_arguments_declaration(
    open_paren: OpenParen,
    arguments: PositionalArguments,
    close_paren: CloseParen,
) -> PositionalArgumentsDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"PositionalArgumentsDeclaration\", \"open_paren\": {}, \"arguments\": {}, \"close_paren\": {},  }} ", open_paren, arguments, close_paren, )
}

pub type NamedArgumentsDeclaration = String;

pub fn new_named_arguments_declaration(
    open_paren: OpenParen,
    arguments: Option<NamedArgumentGroup>,
    close_paren: CloseParen,
) -> NamedArgumentsDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"NamedArgumentsDeclaration\", \"open_paren\": {}, \"arguments\": {}, \"close_paren\": {},  }} ", open_paren, arguments.unwrap_or("".into()), close_paren, )
}

pub type NamedArgumentGroup = String;

pub fn new_named_argument_group(
    open_brace: OpenBrace,
    arguments: NamedArguments,
    close_brace: CloseBrace,
) -> NamedArgumentGroup {
    format!(" {{  \"item\": \"Struct\", \"name\": \"NamedArgumentGroup\", \"open_brace\": {}, \"arguments\": {}, \"close_brace\": {},  }} ", open_brace, arguments, close_brace, )
}

pub type NamedArgument = String;

pub fn new_named_argument(name: Identifier, colon: Colon, value: Expression) -> NamedArgument {
    format!(" {{  \"item\": \"Struct\", \"name\": \"NamedArgument\", \"name\": {}, \"colon\": {}, \"value\": {},  }} ", name, colon, value, )
}

pub type TypeExpression = String;

pub fn new_type_expression(
    type_keyword: TypeKeyword,
    open_paren: OpenParen,
    type_name: TypeName,
    close_paren: CloseParen,
) -> TypeExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"TypeExpression\", \"type_keyword\": {}, \"open_paren\": {}, \"type_name\": {}, \"close_paren\": {},  }} ", type_keyword, open_paren, type_name, close_paren, )
}

pub type NewExpression = String;

pub fn new_new_expression(new_keyword: NewKeyword, type_name: NewableTypeName) -> NewExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"NewExpression\", \"new_keyword\": {}, \"type_name\": {},  }} ", new_keyword, type_name, )
}

pub type TupleExpression = String;

pub fn new_tuple_expression(
    open_paren: OpenParen,
    items: TupleValues,
    close_paren: CloseParen,
) -> TupleExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"TupleExpression\", \"open_paren\": {}, \"items\": {}, \"close_paren\": {},  }} ", open_paren, items, close_paren, )
}

pub type TupleValue = String;

pub fn new_tuple_value(expression: Option<Expression>) -> TupleValue {
    format!(
        " {{  \"item\": \"Struct\", \"name\": \"TupleValue\", \"expression\": {},  }} ",
        expression.unwrap_or("".into()),
    )
}

pub type ArrayExpression = String;

pub fn new_array_expression(
    open_bracket: OpenBracket,
    items: ArrayValues,
    close_bracket: CloseBracket,
) -> ArrayExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"ArrayExpression\", \"open_bracket\": {}, \"items\": {}, \"close_bracket\": {},  }} ", open_bracket, items, close_bracket, )
}

pub type HexNumberExpression = String;

pub fn new_hex_number_expression(
    literal: HexLiteral,
    unit: Option<NumberUnit>,
) -> HexNumberExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"HexNumberExpression\", \"literal\": {}, \"unit\": {},  }} ", literal, unit.unwrap_or("".into()), )
}

pub type DecimalNumberExpression = String;

pub fn new_decimal_number_expression(
    literal: DecimalLiteral,
    unit: Option<NumberUnit>,
) -> DecimalNumberExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"DecimalNumberExpression\", \"literal\": {}, \"unit\": {},  }} ", literal, unit.unwrap_or("".into()), )
}

pub type YulBlock = String;

pub fn new_yul_block(
    open_brace: OpenBrace,
    statements: YulStatements,
    close_brace: CloseBrace,
) -> YulBlock {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulBlock\", \"open_brace\": {}, \"statements\": {}, \"close_brace\": {},  }} ", open_brace, statements, close_brace, )
}

pub type YulFunctionDefinition = String;

pub fn new_yul_function_definition(
    function_keyword: YulFunctionKeyword,
    name: YulIdentifier,
    parameters: YulParametersDeclaration,
    returns: Option<YulReturnsDeclaration>,
    body: YulBlock,
) -> YulFunctionDefinition {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulFunctionDefinition\", \"function_keyword\": {}, \"name\": {}, \"parameters\": {}, \"returns\": {}, \"body\": {},  }} ", function_keyword, name, parameters, returns.unwrap_or("".into()), body, )
}

pub type YulParametersDeclaration = String;

pub fn new_yul_parameters_declaration(
    open_paren: OpenParen,
    parameters: YulParameters,
    close_paren: CloseParen,
) -> YulParametersDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulParametersDeclaration\", \"open_paren\": {}, \"parameters\": {}, \"close_paren\": {},  }} ", open_paren, parameters, close_paren, )
}

pub type YulReturnsDeclaration = String;

pub fn new_yul_returns_declaration(
    minus_greater_than: MinusGreaterThan,
    variables: YulVariableNames,
) -> YulReturnsDeclaration {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulReturnsDeclaration\", \"minus_greater_than\": {}, \"variables\": {},  }} ", minus_greater_than, variables, )
}

pub type YulVariableDeclarationStatement = String;

pub fn new_yul_variable_declaration_statement(
    let_keyword: YulLetKeyword,
    variables: YulVariableNames,
    value: Option<YulVariableDeclarationValue>,
) -> YulVariableDeclarationStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulVariableDeclarationStatement\", \"let_keyword\": {}, \"variables\": {}, \"value\": {},  }} ", let_keyword, variables, value.unwrap_or("".into()), )
}

pub type YulVariableDeclarationValue = String;

pub fn new_yul_variable_declaration_value(
    assignment: YulAssignmentOperator,
    expression: YulExpression,
) -> YulVariableDeclarationValue {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulVariableDeclarationValue\", \"assignment\": {}, \"expression\": {},  }} ", assignment, expression, )
}

pub type YulVariableAssignmentStatement = String;

pub fn new_yul_variable_assignment_statement(
    variables: YulPaths,
    assignment: YulAssignmentOperator,
    expression: YulExpression,
) -> YulVariableAssignmentStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulVariableAssignmentStatement\", \"variables\": {}, \"assignment\": {}, \"expression\": {},  }} ", variables, assignment, expression, )
}

pub type YulColonAndEqual = String;

pub fn new_yul_colon_and_equal(colon: Colon, equal: Equal) -> YulColonAndEqual {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulColonAndEqual\", \"colon\": {}, \"equal\": {},  }} ", colon, equal, )
}

pub type YulStackAssignmentStatement = String;

pub fn new_yul_stack_assignment_statement(
    assignment: YulStackAssignmentOperator,
    variable: YulIdentifier,
) -> YulStackAssignmentStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulStackAssignmentStatement\", \"assignment\": {}, \"variable\": {},  }} ", assignment, variable, )
}

pub type YulEqualAndColon = String;

pub fn new_yul_equal_and_colon(equal: Equal, colon: Colon) -> YulEqualAndColon {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulEqualAndColon\", \"equal\": {}, \"colon\": {},  }} ", equal, colon, )
}

pub type YulIfStatement = String;

pub fn new_yul_if_statement(
    if_keyword: YulIfKeyword,
    condition: YulExpression,
    body: YulBlock,
) -> YulIfStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulIfStatement\", \"if_keyword\": {}, \"condition\": {}, \"body\": {},  }} ", if_keyword, condition, body, )
}

pub type YulForStatement = String;

pub fn new_yul_for_statement(
    for_keyword: YulForKeyword,
    initialization: YulBlock,
    condition: YulExpression,
    iterator: YulBlock,
    body: YulBlock,
) -> YulForStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulForStatement\", \"for_keyword\": {}, \"initialization\": {}, \"condition\": {}, \"iterator\": {}, \"body\": {},  }} ", for_keyword, initialization, condition, iterator, body, )
}

pub type YulSwitchStatement = String;

pub fn new_yul_switch_statement(
    switch_keyword: YulSwitchKeyword,
    expression: YulExpression,
    cases: YulSwitchCases,
) -> YulSwitchStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulSwitchStatement\", \"switch_keyword\": {}, \"expression\": {}, \"cases\": {},  }} ", switch_keyword, expression, cases, )
}

pub type YulDefaultCase = String;

pub fn new_yul_default_case(default_keyword: YulDefaultKeyword, body: YulBlock) -> YulDefaultCase {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulDefaultCase\", \"default_keyword\": {}, \"body\": {},  }} ", default_keyword, body, )
}

pub type YulValueCase = String;

pub fn new_yul_value_case(
    case_keyword: YulCaseKeyword,
    value: YulLiteral,
    body: YulBlock,
) -> YulValueCase {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulValueCase\", \"case_keyword\": {}, \"value\": {}, \"body\": {},  }} ", case_keyword, value, body, )
}

pub type YulLeaveStatement = String;

pub fn new_yul_leave_statement(leave_keyword: YulLeaveKeyword) -> YulLeaveStatement {
    format!(
        " {{  \"item\": \"Struct\", \"name\": \"YulLeaveStatement\", \"leave_keyword\": {},  }} ",
        leave_keyword,
    )
}

pub type YulBreakStatement = String;

pub fn new_yul_break_statement(break_keyword: YulBreakKeyword) -> YulBreakStatement {
    format!(
        " {{  \"item\": \"Struct\", \"name\": \"YulBreakStatement\", \"break_keyword\": {},  }} ",
        break_keyword,
    )
}

pub type YulContinueStatement = String;

pub fn new_yul_continue_statement(continue_keyword: YulContinueKeyword) -> YulContinueStatement {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulContinueStatement\", \"continue_keyword\": {},  }} ", continue_keyword, )
}

pub type YulLabel = String;

pub fn new_yul_label(label: YulIdentifier, colon: Colon) -> YulLabel {
    format!(
        " {{  \"item\": \"Struct\", \"name\": \"YulLabel\", \"label\": {}, \"colon\": {},  }} ",
        label, colon,
    )
}

pub type YulFunctionCallExpression = String;

pub fn new_yul_function_call_expression(
    operand: YulExpression,
    open_paren: OpenParen,
    arguments: YulArguments,
    close_paren: CloseParen,
) -> YulFunctionCallExpression {
    format!(" {{  \"item\": \"Struct\", \"name\": \"YulFunctionCallExpression\", \"operand\": {}, \"open_paren\": {}, \"arguments\": {}, \"close_paren\": {},  }} ", operand, open_paren, arguments, close_paren, )
}

//
// Choices:
//

pub type SourceUnitMember = String;

pub fn new_source_unit_member_pragma_directive(element: PragmaDirective) -> SourceUnitMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"PragmaDirective\": {}, }} ",
        element
    )
}

pub fn new_source_unit_member_import_directive(element: ImportDirective) -> SourceUnitMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"ImportDirective\": {}, }} ",
        element
    )
}

pub fn new_source_unit_member_contract_definition(element: ContractDefinition) -> SourceUnitMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"ContractDefinition\": {}, }} ", element)
}

pub fn new_source_unit_member_interface_definition(
    element: InterfaceDefinition,
) -> SourceUnitMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"InterfaceDefinition\": {}, }} ", element)
}

pub fn new_source_unit_member_library_definition(element: LibraryDefinition) -> SourceUnitMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"LibraryDefinition\": {}, }} ",
        element
    )
}

pub fn new_source_unit_member_struct_definition(element: StructDefinition) -> SourceUnitMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"StructDefinition\": {}, }} ",
        element
    )
}

pub fn new_source_unit_member_enum_definition(element: EnumDefinition) -> SourceUnitMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"EnumDefinition\": {}, }} ",
        element
    )
}

pub fn new_source_unit_member_function_definition(element: FunctionDefinition) -> SourceUnitMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"FunctionDefinition\": {}, }} ", element)
}

pub fn new_source_unit_member_error_definition(element: ErrorDefinition) -> SourceUnitMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"ErrorDefinition\": {}, }} ",
        element
    )
}

pub fn new_source_unit_member_user_defined_value_type_definition(
    element: UserDefinedValueTypeDefinition,
) -> SourceUnitMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"UserDefinedValueTypeDefinition\": {}, }} ", element)
}

pub fn new_source_unit_member_using_directive(element: UsingDirective) -> SourceUnitMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"UsingDirective\": {}, }} ",
        element
    )
}

pub fn new_source_unit_member_event_definition(element: EventDefinition) -> SourceUnitMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"EventDefinition\": {}, }} ",
        element
    )
}

pub fn new_source_unit_member_constant_definition(element: ConstantDefinition) -> SourceUnitMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"SourceUnitMember\", \"ConstantDefinition\": {}, }} ", element)
}

pub type Pragma = String;

pub fn new_pragma_version_pragma(element: VersionPragma) -> Pragma {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Pragma\", \"VersionPragma\": {}, }} ",
        element
    )
}

pub fn new_pragma_abicoder_pragma(element: AbicoderPragma) -> Pragma {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Pragma\", \"AbicoderPragma\": {}, }} ",
        element
    )
}

pub fn new_pragma_experimental_pragma(element: ExperimentalPragma) -> Pragma {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Pragma\", \"ExperimentalPragma\": {}, }} ",
        element
    )
}

pub type AbicoderVersion = String;

pub fn new_abicoder_version_abicoder_v1_keyword(element: AbicoderV1Keyword) -> AbicoderVersion {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"AbicoderVersion\", \"AbicoderV1Keyword\": {}, }} ",
        element
    )
}

pub fn new_abicoder_version_abicoder_v2_keyword(element: AbicoderV2Keyword) -> AbicoderVersion {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"AbicoderVersion\", \"AbicoderV2Keyword\": {}, }} ",
        element
    )
}

pub type ExperimentalFeature = String;

pub fn new_experimental_feature_abi_encoder_v2_keyword(
    element: ABIEncoderV2Keyword,
) -> ExperimentalFeature {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ExperimentalFeature\", \"ABIEncoderV2Keyword\": {}, }} ", element)
}

pub fn new_experimental_feature_smt_checker_keyword(
    element: SMTCheckerKeyword,
) -> ExperimentalFeature {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ExperimentalFeature\", \"SMTCheckerKeyword\": {}, }} ", element)
}

pub fn new_experimental_feature_string_literal(element: StringLiteral) -> ExperimentalFeature {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ExperimentalFeature\", \"StringLiteral\": {}, }} ",
        element
    )
}

pub type VersionExpression = String;

pub fn new_version_expression_version_range(element: VersionRange) -> VersionExpression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VersionExpression\", \"VersionRange\": {}, }} ",
        element
    )
}

pub fn new_version_expression_version_term(element: VersionTerm) -> VersionExpression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VersionExpression\", \"VersionTerm\": {}, }} ",
        element
    )
}

pub type VersionOperator = String;

pub fn new_version_operator_caret(element: Caret) -> VersionOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VersionOperator\", \"Caret\": {}, }} ",
        element
    )
}

pub fn new_version_operator_tilde(element: Tilde) -> VersionOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VersionOperator\", \"Tilde\": {}, }} ",
        element
    )
}

pub fn new_version_operator_equal(element: Equal) -> VersionOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VersionOperator\", \"Equal\": {}, }} ",
        element
    )
}

pub fn new_version_operator_less_than(element: LessThan) -> VersionOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VersionOperator\", \"LessThan\": {}, }} ",
        element
    )
}

pub fn new_version_operator_greater_than(element: GreaterThan) -> VersionOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VersionOperator\", \"GreaterThan\": {}, }} ",
        element
    )
}

pub fn new_version_operator_less_than_equal(element: LessThanEqual) -> VersionOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VersionOperator\", \"LessThanEqual\": {}, }} ",
        element
    )
}

pub fn new_version_operator_greater_than_equal(element: GreaterThanEqual) -> VersionOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VersionOperator\", \"GreaterThanEqual\": {}, }} ",
        element
    )
}

pub type VersionLiteral = String;

pub fn new_version_literal_simple_version_literal(element: SimpleVersionLiteral) -> VersionLiteral {
    format!(" {{  \"item\": \"Choice\", \"name\": \"VersionLiteral\", \"SimpleVersionLiteral\": {}, }} ", element)
}

pub fn new_version_literal_single_quoted_version_literal(
    element: SingleQuotedVersionLiteral,
) -> VersionLiteral {
    format!(" {{  \"item\": \"Choice\", \"name\": \"VersionLiteral\", \"SingleQuotedVersionLiteral\": {}, }} ", element)
}

pub fn new_version_literal_double_quoted_version_literal(
    element: DoubleQuotedVersionLiteral,
) -> VersionLiteral {
    format!(" {{  \"item\": \"Choice\", \"name\": \"VersionLiteral\", \"DoubleQuotedVersionLiteral\": {}, }} ", element)
}

pub type ImportClause = String;

pub fn new_import_clause_path_import(element: PathImport) -> ImportClause {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ImportClause\", \"PathImport\": {}, }} ",
        element
    )
}

pub fn new_import_clause_named_import(element: NamedImport) -> ImportClause {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ImportClause\", \"NamedImport\": {}, }} ",
        element
    )
}

pub fn new_import_clause_import_deconstruction(element: ImportDeconstruction) -> ImportClause {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ImportClause\", \"ImportDeconstruction\": {}, }} ",
        element
    )
}

pub type UsingClause = String;

pub fn new_using_clause_identifier_path(element: IdentifierPath) -> UsingClause {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingClause\", \"IdentifierPath\": {}, }} ",
        element
    )
}

pub fn new_using_clause_using_deconstruction(element: UsingDeconstruction) -> UsingClause {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingClause\", \"UsingDeconstruction\": {}, }} ",
        element
    )
}

pub type UsingOperator = String;

pub fn new_using_operator_ampersand(element: Ampersand) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"Ampersand\": {}, }} ",
        element
    )
}

pub fn new_using_operator_asterisk(element: Asterisk) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"Asterisk\": {}, }} ",
        element
    )
}

pub fn new_using_operator_bang_equal(element: BangEqual) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"BangEqual\": {}, }} ",
        element
    )
}

pub fn new_using_operator_bar(element: Bar) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"Bar\": {}, }} ",
        element
    )
}

pub fn new_using_operator_caret(element: Caret) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"Caret\": {}, }} ",
        element
    )
}

pub fn new_using_operator_equal_equal(element: EqualEqual) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"EqualEqual\": {}, }} ",
        element
    )
}

pub fn new_using_operator_greater_than(element: GreaterThan) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"GreaterThan\": {}, }} ",
        element
    )
}

pub fn new_using_operator_greater_than_equal(element: GreaterThanEqual) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"GreaterThanEqual\": {}, }} ",
        element
    )
}

pub fn new_using_operator_less_than(element: LessThan) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"LessThan\": {}, }} ",
        element
    )
}

pub fn new_using_operator_less_than_equal(element: LessThanEqual) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"LessThanEqual\": {}, }} ",
        element
    )
}

pub fn new_using_operator_minus(element: Minus) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"Minus\": {}, }} ",
        element
    )
}

pub fn new_using_operator_percent(element: Percent) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"Percent\": {}, }} ",
        element
    )
}

pub fn new_using_operator_plus(element: Plus) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"Plus\": {}, }} ",
        element
    )
}

pub fn new_using_operator_slash(element: Slash) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"Slash\": {}, }} ",
        element
    )
}

pub fn new_using_operator_tilde(element: Tilde) -> UsingOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingOperator\", \"Tilde\": {}, }} ",
        element
    )
}

pub type UsingTarget = String;

pub fn new_using_target_type_name(element: TypeName) -> UsingTarget {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingTarget\", \"TypeName\": {}, }} ",
        element
    )
}

pub fn new_using_target_asterisk(element: Asterisk) -> UsingTarget {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"UsingTarget\", \"Asterisk\": {}, }} ",
        element
    )
}

pub type ContractSpecifier = String;

pub fn new_contract_specifier_inheritance_specifier(
    element: InheritanceSpecifier,
) -> ContractSpecifier {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ContractSpecifier\", \"InheritanceSpecifier\": {}, }} ", element)
}

pub fn new_contract_specifier_storage_layout_specifier(
    element: StorageLayoutSpecifier,
) -> ContractSpecifier {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ContractSpecifier\", \"StorageLayoutSpecifier\": {}, }} ", element)
}

pub type ContractMember = String;

pub fn new_contract_member_using_directive(element: UsingDirective) -> ContractMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"UsingDirective\": {}, }} ",
        element
    )
}

pub fn new_contract_member_function_definition(element: FunctionDefinition) -> ContractMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"FunctionDefinition\": {}, }} ",
        element
    )
}

pub fn new_contract_member_constructor_definition(
    element: ConstructorDefinition,
) -> ContractMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"ConstructorDefinition\": {}, }} ", element)
}

pub fn new_contract_member_receive_function_definition(
    element: ReceiveFunctionDefinition,
) -> ContractMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"ReceiveFunctionDefinition\": {}, }} ", element)
}

pub fn new_contract_member_fallback_function_definition(
    element: FallbackFunctionDefinition,
) -> ContractMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"FallbackFunctionDefinition\": {}, }} ", element)
}

pub fn new_contract_member_unnamed_function_definition(
    element: UnnamedFunctionDefinition,
) -> ContractMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"UnnamedFunctionDefinition\": {}, }} ", element)
}

pub fn new_contract_member_modifier_definition(element: ModifierDefinition) -> ContractMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"ModifierDefinition\": {}, }} ",
        element
    )
}

pub fn new_contract_member_struct_definition(element: StructDefinition) -> ContractMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"StructDefinition\": {}, }} ",
        element
    )
}

pub fn new_contract_member_enum_definition(element: EnumDefinition) -> ContractMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"EnumDefinition\": {}, }} ",
        element
    )
}

pub fn new_contract_member_event_definition(element: EventDefinition) -> ContractMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"EventDefinition\": {}, }} ",
        element
    )
}

pub fn new_contract_member_error_definition(element: ErrorDefinition) -> ContractMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"ErrorDefinition\": {}, }} ",
        element
    )
}

pub fn new_contract_member_user_defined_value_type_definition(
    element: UserDefinedValueTypeDefinition,
) -> ContractMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"UserDefinedValueTypeDefinition\": {}, }} ", element)
}

pub fn new_contract_member_state_variable_definition(
    element: StateVariableDefinition,
) -> ContractMember {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ContractMember\", \"StateVariableDefinition\": {}, }} ", element)
}

pub type StateVariableAttribute = String;

pub fn new_state_variable_attribute_override_specifier(
    element: OverrideSpecifier,
) -> StateVariableAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StateVariableAttribute\", \"OverrideSpecifier\": {}, }} ", element)
}

pub fn new_state_variable_attribute_constant_keyword(
    element: ConstantKeyword,
) -> StateVariableAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StateVariableAttribute\", \"ConstantKeyword\": {}, }} ", element)
}

pub fn new_state_variable_attribute_internal_keyword(
    element: InternalKeyword,
) -> StateVariableAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StateVariableAttribute\", \"InternalKeyword\": {}, }} ", element)
}

pub fn new_state_variable_attribute_private_keyword(
    element: PrivateKeyword,
) -> StateVariableAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StateVariableAttribute\", \"PrivateKeyword\": {}, }} ", element)
}

pub fn new_state_variable_attribute_public_keyword(
    element: PublicKeyword,
) -> StateVariableAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StateVariableAttribute\", \"PublicKeyword\": {}, }} ", element)
}

pub fn new_state_variable_attribute_immutable_keyword(
    element: ImmutableKeyword,
) -> StateVariableAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StateVariableAttribute\", \"ImmutableKeyword\": {}, }} ", element)
}

pub fn new_state_variable_attribute_transient_keyword(
    element: TransientKeyword,
) -> StateVariableAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StateVariableAttribute\", \"TransientKeyword\": {}, }} ", element)
}

pub type FunctionName = String;

pub fn new_function_name_identifier(element: Identifier) -> FunctionName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionName\", \"Identifier\": {}, }} ",
        element
    )
}

pub fn new_function_name_fallback_keyword(element: FallbackKeyword) -> FunctionName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionName\", \"FallbackKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_name_receive_keyword(element: ReceiveKeyword) -> FunctionName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionName\", \"ReceiveKeyword\": {}, }} ",
        element
    )
}

pub type FunctionAttribute = String;

pub fn new_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> FunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"ModifierInvocation\": {}, }} ", element)
}

pub fn new_function_attribute_override_specifier(element: OverrideSpecifier) -> FunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"OverrideSpecifier\": {}, }} ", element)
}

pub fn new_function_attribute_constant_keyword(element: ConstantKeyword) -> FunctionAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"ConstantKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_attribute_external_keyword(element: ExternalKeyword) -> FunctionAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"ExternalKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_attribute_internal_keyword(element: InternalKeyword) -> FunctionAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"InternalKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_attribute_payable_keyword(element: PayableKeyword) -> FunctionAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"PayableKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_attribute_private_keyword(element: PrivateKeyword) -> FunctionAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"PrivateKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_attribute_public_keyword(element: PublicKeyword) -> FunctionAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"PublicKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_attribute_pure_keyword(element: PureKeyword) -> FunctionAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"PureKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_attribute_view_keyword(element: ViewKeyword) -> FunctionAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"ViewKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_attribute_virtual_keyword(element: VirtualKeyword) -> FunctionAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionAttribute\", \"VirtualKeyword\": {}, }} ",
        element
    )
}

pub type FunctionBody = String;

pub fn new_function_body_block(element: Block) -> FunctionBody {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionBody\", \"Block\": {}, }} ",
        element
    )
}

pub fn new_function_body_semicolon(element: Semicolon) -> FunctionBody {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionBody\", \"Semicolon\": {}, }} ",
        element
    )
}

pub type ConstructorAttribute = String;

pub fn new_constructor_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> ConstructorAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ConstructorAttribute\", \"ModifierInvocation\": {}, }} ", element)
}

pub fn new_constructor_attribute_internal_keyword(
    element: InternalKeyword,
) -> ConstructorAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ConstructorAttribute\", \"InternalKeyword\": {}, }} ", element)
}

pub fn new_constructor_attribute_override_keyword(
    element: OverrideKeyword,
) -> ConstructorAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ConstructorAttribute\", \"OverrideKeyword\": {}, }} ", element)
}

pub fn new_constructor_attribute_payable_keyword(element: PayableKeyword) -> ConstructorAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ConstructorAttribute\", \"PayableKeyword\": {}, }} ", element)
}

pub fn new_constructor_attribute_public_keyword(element: PublicKeyword) -> ConstructorAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ConstructorAttribute\", \"PublicKeyword\": {}, }} ",
        element
    )
}

pub fn new_constructor_attribute_virtual_keyword(element: VirtualKeyword) -> ConstructorAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ConstructorAttribute\", \"VirtualKeyword\": {}, }} ", element)
}

pub type UnnamedFunctionAttribute = String;

pub fn new_unnamed_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> UnnamedFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnnamedFunctionAttribute\", \"ModifierInvocation\": {}, }} ", element)
}

pub fn new_unnamed_function_attribute_constant_keyword(
    element: ConstantKeyword,
) -> UnnamedFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnnamedFunctionAttribute\", \"ConstantKeyword\": {}, }} ", element)
}

pub fn new_unnamed_function_attribute_external_keyword(
    element: ExternalKeyword,
) -> UnnamedFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnnamedFunctionAttribute\", \"ExternalKeyword\": {}, }} ", element)
}

pub fn new_unnamed_function_attribute_internal_keyword(
    element: InternalKeyword,
) -> UnnamedFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnnamedFunctionAttribute\", \"InternalKeyword\": {}, }} ", element)
}

pub fn new_unnamed_function_attribute_payable_keyword(
    element: PayableKeyword,
) -> UnnamedFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnnamedFunctionAttribute\", \"PayableKeyword\": {}, }} ", element)
}

pub fn new_unnamed_function_attribute_private_keyword(
    element: PrivateKeyword,
) -> UnnamedFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnnamedFunctionAttribute\", \"PrivateKeyword\": {}, }} ", element)
}

pub fn new_unnamed_function_attribute_public_keyword(
    element: PublicKeyword,
) -> UnnamedFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnnamedFunctionAttribute\", \"PublicKeyword\": {}, }} ", element)
}

pub fn new_unnamed_function_attribute_pure_keyword(
    element: PureKeyword,
) -> UnnamedFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnnamedFunctionAttribute\", \"PureKeyword\": {}, }} ", element)
}

pub fn new_unnamed_function_attribute_view_keyword(
    element: ViewKeyword,
) -> UnnamedFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnnamedFunctionAttribute\", \"ViewKeyword\": {}, }} ", element)
}

pub type FallbackFunctionAttribute = String;

pub fn new_fallback_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> FallbackFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FallbackFunctionAttribute\", \"ModifierInvocation\": {}, }} ", element)
}

pub fn new_fallback_function_attribute_override_specifier(
    element: OverrideSpecifier,
) -> FallbackFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FallbackFunctionAttribute\", \"OverrideSpecifier\": {}, }} ", element)
}

pub fn new_fallback_function_attribute_external_keyword(
    element: ExternalKeyword,
) -> FallbackFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FallbackFunctionAttribute\", \"ExternalKeyword\": {}, }} ", element)
}

pub fn new_fallback_function_attribute_payable_keyword(
    element: PayableKeyword,
) -> FallbackFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FallbackFunctionAttribute\", \"PayableKeyword\": {}, }} ", element)
}

pub fn new_fallback_function_attribute_pure_keyword(
    element: PureKeyword,
) -> FallbackFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FallbackFunctionAttribute\", \"PureKeyword\": {}, }} ", element)
}

pub fn new_fallback_function_attribute_view_keyword(
    element: ViewKeyword,
) -> FallbackFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FallbackFunctionAttribute\", \"ViewKeyword\": {}, }} ", element)
}

pub fn new_fallback_function_attribute_virtual_keyword(
    element: VirtualKeyword,
) -> FallbackFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FallbackFunctionAttribute\", \"VirtualKeyword\": {}, }} ", element)
}

pub type ReceiveFunctionAttribute = String;

pub fn new_receive_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> ReceiveFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ReceiveFunctionAttribute\", \"ModifierInvocation\": {}, }} ", element)
}

pub fn new_receive_function_attribute_override_specifier(
    element: OverrideSpecifier,
) -> ReceiveFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ReceiveFunctionAttribute\", \"OverrideSpecifier\": {}, }} ", element)
}

pub fn new_receive_function_attribute_external_keyword(
    element: ExternalKeyword,
) -> ReceiveFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ReceiveFunctionAttribute\", \"ExternalKeyword\": {}, }} ", element)
}

pub fn new_receive_function_attribute_payable_keyword(
    element: PayableKeyword,
) -> ReceiveFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ReceiveFunctionAttribute\", \"PayableKeyword\": {}, }} ", element)
}

pub fn new_receive_function_attribute_virtual_keyword(
    element: VirtualKeyword,
) -> ReceiveFunctionAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ReceiveFunctionAttribute\", \"VirtualKeyword\": {}, }} ", element)
}

pub type ModifierAttribute = String;

pub fn new_modifier_attribute_override_specifier(element: OverrideSpecifier) -> ModifierAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ModifierAttribute\", \"OverrideSpecifier\": {}, }} ", element)
}

pub fn new_modifier_attribute_virtual_keyword(element: VirtualKeyword) -> ModifierAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ModifierAttribute\", \"VirtualKeyword\": {}, }} ",
        element
    )
}

pub type TypeName = String;

pub fn new_type_name_array_type_name(element: ArrayTypeName) -> TypeName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"TypeName\", \"ArrayTypeName\": {}, }} ",
        element
    )
}

pub fn new_type_name_function_type(element: FunctionType) -> TypeName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"TypeName\", \"FunctionType\": {}, }} ",
        element
    )
}

pub fn new_type_name_mapping_type(element: MappingType) -> TypeName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"TypeName\", \"MappingType\": {}, }} ",
        element
    )
}

pub fn new_type_name_elementary_type(element: ElementaryType) -> TypeName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"TypeName\", \"ElementaryType\": {}, }} ",
        element
    )
}

pub fn new_type_name_identifier_path(element: IdentifierPath) -> TypeName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"TypeName\", \"IdentifierPath\": {}, }} ",
        element
    )
}

pub type NewableTypeName = String;

pub fn new_newable_type_name_newable_array_type(element: NewableArrayType) -> NewableTypeName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NewableTypeName\", \"NewableArrayType\": {}, }} ",
        element
    )
}

pub fn new_newable_type_name_elementary_type(element: ElementaryType) -> NewableTypeName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NewableTypeName\", \"ElementaryType\": {}, }} ",
        element
    )
}

pub fn new_newable_type_name_identifier_path(element: IdentifierPath) -> NewableTypeName {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NewableTypeName\", \"IdentifierPath\": {}, }} ",
        element
    )
}

pub type FunctionTypeAttribute = String;

pub fn new_function_type_attribute_internal_keyword(
    element: InternalKeyword,
) -> FunctionTypeAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FunctionTypeAttribute\", \"InternalKeyword\": {}, }} ", element)
}

pub fn new_function_type_attribute_external_keyword(
    element: ExternalKeyword,
) -> FunctionTypeAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FunctionTypeAttribute\", \"ExternalKeyword\": {}, }} ", element)
}

pub fn new_function_type_attribute_private_keyword(
    element: PrivateKeyword,
) -> FunctionTypeAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FunctionTypeAttribute\", \"PrivateKeyword\": {}, }} ", element)
}

pub fn new_function_type_attribute_public_keyword(element: PublicKeyword) -> FunctionTypeAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FunctionTypeAttribute\", \"PublicKeyword\": {}, }} ", element)
}

pub fn new_function_type_attribute_constant_keyword(
    element: ConstantKeyword,
) -> FunctionTypeAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FunctionTypeAttribute\", \"ConstantKeyword\": {}, }} ", element)
}

pub fn new_function_type_attribute_pure_keyword(element: PureKeyword) -> FunctionTypeAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionTypeAttribute\", \"PureKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_type_attribute_view_keyword(element: ViewKeyword) -> FunctionTypeAttribute {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"FunctionTypeAttribute\", \"ViewKeyword\": {}, }} ",
        element
    )
}

pub fn new_function_type_attribute_payable_keyword(
    element: PayableKeyword,
) -> FunctionTypeAttribute {
    format!(" {{  \"item\": \"Choice\", \"name\": \"FunctionTypeAttribute\", \"PayableKeyword\": {}, }} ", element)
}

pub type MappingKeyType = String;

pub fn new_mapping_key_type_elementary_type(element: ElementaryType) -> MappingKeyType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"MappingKeyType\", \"ElementaryType\": {}, }} ",
        element
    )
}

pub fn new_mapping_key_type_identifier_path(element: IdentifierPath) -> MappingKeyType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"MappingKeyType\", \"IdentifierPath\": {}, }} ",
        element
    )
}

pub type ElementaryType = String;

pub fn new_elementary_type_bool_keyword(element: BoolKeyword) -> ElementaryType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ElementaryType\", \"BoolKeyword\": {}, }} ",
        element
    )
}

pub fn new_elementary_type_byte_keyword(element: ByteKeyword) -> ElementaryType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ElementaryType\", \"ByteKeyword\": {}, }} ",
        element
    )
}

pub fn new_elementary_type_string_keyword(element: StringKeyword) -> ElementaryType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ElementaryType\", \"StringKeyword\": {}, }} ",
        element
    )
}

pub fn new_elementary_type_address_type(element: AddressType) -> ElementaryType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ElementaryType\", \"AddressType\": {}, }} ",
        element
    )
}

pub fn new_elementary_type_bytes_keyword(element: BytesKeyword) -> ElementaryType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ElementaryType\", \"BytesKeyword\": {}, }} ",
        element
    )
}

pub fn new_elementary_type_int_keyword(element: IntKeyword) -> ElementaryType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ElementaryType\", \"IntKeyword\": {}, }} ",
        element
    )
}

pub fn new_elementary_type_uint_keyword(element: UintKeyword) -> ElementaryType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ElementaryType\", \"UintKeyword\": {}, }} ",
        element
    )
}

pub fn new_elementary_type_fixed_keyword(element: FixedKeyword) -> ElementaryType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ElementaryType\", \"FixedKeyword\": {}, }} ",
        element
    )
}

pub fn new_elementary_type_ufixed_keyword(element: UfixedKeyword) -> ElementaryType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ElementaryType\", \"UfixedKeyword\": {}, }} ",
        element
    )
}

pub type Statement = String;

pub fn new_statement_if_statement(element: IfStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"IfStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_for_statement(element: ForStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"ForStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_while_statement(element: WhileStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"WhileStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_do_while_statement(element: DoWhileStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"DoWhileStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_continue_statement(element: ContinueStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"ContinueStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_break_statement(element: BreakStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"BreakStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_return_statement(element: ReturnStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"ReturnStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_throw_statement(element: ThrowStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"ThrowStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_emit_statement(element: EmitStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"EmitStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_try_statement(element: TryStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"TryStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_revert_statement(element: RevertStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"RevertStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_assembly_statement(element: AssemblyStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"AssemblyStatement\": {}, }} ",
        element
    )
}

pub fn new_statement_block(element: Block) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"Block\": {}, }} ",
        element
    )
}

pub fn new_statement_unchecked_block(element: UncheckedBlock) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"UncheckedBlock\": {}, }} ",
        element
    )
}

pub fn new_statement_tuple_deconstruction_statement(
    element: TupleDeconstructionStatement,
) -> Statement {
    format!(" {{  \"item\": \"Choice\", \"name\": \"Statement\", \"TupleDeconstructionStatement\": {}, }} ", element)
}

pub fn new_statement_variable_declaration_statement(
    element: VariableDeclarationStatement,
) -> Statement {
    format!(" {{  \"item\": \"Choice\", \"name\": \"Statement\", \"VariableDeclarationStatement\": {}, }} ", element)
}

pub fn new_statement_expression_statement(element: ExpressionStatement) -> Statement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Statement\", \"ExpressionStatement\": {}, }} ",
        element
    )
}

pub type TupleMember = String;

pub fn new_tuple_member_typed_tuple_member(element: TypedTupleMember) -> TupleMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"TupleMember\", \"TypedTupleMember\": {}, }} ",
        element
    )
}

pub fn new_tuple_member_untyped_tuple_member(element: UntypedTupleMember) -> TupleMember {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"TupleMember\", \"UntypedTupleMember\": {}, }} ",
        element
    )
}

pub type VariableDeclarationType = String;

pub fn new_variable_declaration_type_type_name(element: TypeName) -> VariableDeclarationType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VariableDeclarationType\", \"TypeName\": {}, }} ",
        element
    )
}

pub fn new_variable_declaration_type_var_keyword(element: VarKeyword) -> VariableDeclarationType {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"VariableDeclarationType\", \"VarKeyword\": {}, }} ",
        element
    )
}

pub type StorageLocation = String;

pub fn new_storage_location_memory_keyword(element: MemoryKeyword) -> StorageLocation {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"StorageLocation\", \"MemoryKeyword\": {}, }} ",
        element
    )
}

pub fn new_storage_location_storage_keyword(element: StorageKeyword) -> StorageLocation {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"StorageLocation\", \"StorageKeyword\": {}, }} ",
        element
    )
}

pub fn new_storage_location_call_data_keyword(element: CallDataKeyword) -> StorageLocation {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"StorageLocation\", \"CallDataKeyword\": {}, }} ",
        element
    )
}

pub type ForStatementInitialization = String;

pub fn new_for_statement_initialization_tuple_deconstruction_statement(
    element: TupleDeconstructionStatement,
) -> ForStatementInitialization {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ForStatementInitialization\", \"TupleDeconstructionStatement\": {}, }} ", element)
}

pub fn new_for_statement_initialization_variable_declaration_statement(
    element: VariableDeclarationStatement,
) -> ForStatementInitialization {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ForStatementInitialization\", \"VariableDeclarationStatement\": {}, }} ", element)
}

pub fn new_for_statement_initialization_expression_statement(
    element: ExpressionStatement,
) -> ForStatementInitialization {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ForStatementInitialization\", \"ExpressionStatement\": {}, }} ", element)
}

pub fn new_for_statement_initialization_semicolon(
    element: Semicolon,
) -> ForStatementInitialization {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ForStatementInitialization\", \"Semicolon\": {}, }} ", element)
}

pub type ForStatementCondition = String;

pub fn new_for_statement_condition_expression_statement(
    element: ExpressionStatement,
) -> ForStatementCondition {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ForStatementCondition\", \"ExpressionStatement\": {}, }} ", element)
}

pub fn new_for_statement_condition_semicolon(element: Semicolon) -> ForStatementCondition {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"ForStatementCondition\", \"Semicolon\": {}, }} ",
        element
    )
}

pub type Expression = String;

pub fn new_expression_assignment_expression(element: AssignmentExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"AssignmentExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_conditional_expression(element: ConditionalExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"ConditionalExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_or_expression(element: OrExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"OrExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_and_expression(element: AndExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"AndExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_equality_expression(element: EqualityExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"EqualityExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_inequality_expression(element: InequalityExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"InequalityExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_bitwise_or_expression(element: BitwiseOrExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"BitwiseOrExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_bitwise_xor_expression(element: BitwiseXorExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"BitwiseXorExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_bitwise_and_expression(element: BitwiseAndExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"BitwiseAndExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_shift_expression(element: ShiftExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"ShiftExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_additive_expression(element: AdditiveExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"AdditiveExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_multiplicative_expression(element: MultiplicativeExpression) -> Expression {
    format!(" {{  \"item\": \"Choice\", \"name\": \"Expression\", \"MultiplicativeExpression\": {}, }} ", element)
}

pub fn new_expression_exponentiation_expression(element: ExponentiationExpression) -> Expression {
    format!(" {{  \"item\": \"Choice\", \"name\": \"Expression\", \"ExponentiationExpression\": {}, }} ", element)
}

pub fn new_expression_postfix_expression(element: PostfixExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"PostfixExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_prefix_expression(element: PrefixExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"PrefixExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_function_call_expression(element: FunctionCallExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"FunctionCallExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_call_options_expression(element: CallOptionsExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"CallOptionsExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_member_access_expression(element: MemberAccessExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"MemberAccessExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_index_access_expression(element: IndexAccessExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"IndexAccessExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_new_expression(element: NewExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"NewExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_tuple_expression(element: TupleExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"TupleExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_type_expression(element: TypeExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"TypeExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_array_expression(element: ArrayExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"ArrayExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_hex_number_expression(element: HexNumberExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"HexNumberExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_decimal_number_expression(element: DecimalNumberExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"DecimalNumberExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_string_expression(element: StringExpression) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"StringExpression\": {}, }} ",
        element
    )
}

pub fn new_expression_elementary_type(element: ElementaryType) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"ElementaryType\": {}, }} ",
        element
    )
}

pub fn new_expression_payable_keyword(element: PayableKeyword) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"PayableKeyword\": {}, }} ",
        element
    )
}

pub fn new_expression_this_keyword(element: ThisKeyword) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"ThisKeyword\": {}, }} ",
        element
    )
}

pub fn new_expression_super_keyword(element: SuperKeyword) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"SuperKeyword\": {}, }} ",
        element
    )
}

pub fn new_expression_true_keyword(element: TrueKeyword) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"TrueKeyword\": {}, }} ",
        element
    )
}

pub fn new_expression_false_keyword(element: FalseKeyword) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"FalseKeyword\": {}, }} ",
        element
    )
}

pub fn new_expression_identifier(element: Identifier) -> Expression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"Expression\", \"Identifier\": {}, }} ",
        element
    )
}

pub type ArgumentsDeclaration = String;

pub fn new_arguments_declaration_positional_arguments_declaration(
    element: PositionalArgumentsDeclaration,
) -> ArgumentsDeclaration {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ArgumentsDeclaration\", \"PositionalArgumentsDeclaration\": {}, }} ", element)
}

pub fn new_arguments_declaration_named_arguments_declaration(
    element: NamedArgumentsDeclaration,
) -> ArgumentsDeclaration {
    format!(" {{  \"item\": \"Choice\", \"name\": \"ArgumentsDeclaration\", \"NamedArgumentsDeclaration\": {}, }} ", element)
}

pub type NumberUnit = String;

pub fn new_number_unit_wei_keyword(element: WeiKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"WeiKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_gwei_keyword(element: GweiKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"GweiKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_szabo_keyword(element: SzaboKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"SzaboKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_finney_keyword(element: FinneyKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"FinneyKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_ether_keyword(element: EtherKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"EtherKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_seconds_keyword(element: SecondsKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"SecondsKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_minutes_keyword(element: MinutesKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"MinutesKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_hours_keyword(element: HoursKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"HoursKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_days_keyword(element: DaysKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"DaysKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_weeks_keyword(element: WeeksKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"WeeksKeyword\": {}, }} ",
        element
    )
}

pub fn new_number_unit_years_keyword(element: YearsKeyword) -> NumberUnit {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"NumberUnit\", \"YearsKeyword\": {}, }} ",
        element
    )
}

pub type StringExpression = String;

pub fn new_string_expression_string_literal(element: StringLiteral) -> StringExpression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"StringExpression\", \"StringLiteral\": {}, }} ",
        element
    )
}

pub fn new_string_expression_string_literals(element: StringLiterals) -> StringExpression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"StringExpression\", \"StringLiterals\": {}, }} ",
        element
    )
}

pub fn new_string_expression_hex_string_literal(element: HexStringLiteral) -> StringExpression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"StringExpression\", \"HexStringLiteral\": {}, }} ",
        element
    )
}

pub fn new_string_expression_hex_string_literals(element: HexStringLiterals) -> StringExpression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"StringExpression\", \"HexStringLiterals\": {}, }} ",
        element
    )
}

pub fn new_string_expression_unicode_string_literals(
    element: UnicodeStringLiterals,
) -> StringExpression {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StringExpression\", \"UnicodeStringLiterals\": {}, }} ", element)
}

pub type StringLiteral = String;

pub fn new_string_literal_single_quoted_string_literal(
    element: SingleQuotedStringLiteral,
) -> StringLiteral {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StringLiteral\", \"SingleQuotedStringLiteral\": {}, }} ", element)
}

pub fn new_string_literal_double_quoted_string_literal(
    element: DoubleQuotedStringLiteral,
) -> StringLiteral {
    format!(" {{  \"item\": \"Choice\", \"name\": \"StringLiteral\", \"DoubleQuotedStringLiteral\": {}, }} ", element)
}

pub type HexStringLiteral = String;

pub fn new_hex_string_literal_single_quoted_hex_string_literal(
    element: SingleQuotedHexStringLiteral,
) -> HexStringLiteral {
    format!(" {{  \"item\": \"Choice\", \"name\": \"HexStringLiteral\", \"SingleQuotedHexStringLiteral\": {}, }} ", element)
}

pub fn new_hex_string_literal_double_quoted_hex_string_literal(
    element: DoubleQuotedHexStringLiteral,
) -> HexStringLiteral {
    format!(" {{  \"item\": \"Choice\", \"name\": \"HexStringLiteral\", \"DoubleQuotedHexStringLiteral\": {}, }} ", element)
}

pub type UnicodeStringLiteral = String;

pub fn new_unicode_string_literal_single_quoted_unicode_string_literal(
    element: SingleQuotedUnicodeStringLiteral,
) -> UnicodeStringLiteral {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnicodeStringLiteral\", \"SingleQuotedUnicodeStringLiteral\": {}, }} ", element)
}

pub fn new_unicode_string_literal_double_quoted_unicode_string_literal(
    element: DoubleQuotedUnicodeStringLiteral,
) -> UnicodeStringLiteral {
    format!(" {{  \"item\": \"Choice\", \"name\": \"UnicodeStringLiteral\", \"DoubleQuotedUnicodeStringLiteral\": {}, }} ", element)
}

pub type YulStatement = String;

pub fn new_yul_statement_yul_block(element: YulBlock) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulBlock\": {}, }} ",
        element
    )
}

pub fn new_yul_statement_yul_function_definition(element: YulFunctionDefinition) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulFunctionDefinition\": {}, }} ",
        element
    )
}

pub fn new_yul_statement_yul_stack_assignment_statement(
    element: YulStackAssignmentStatement,
) -> YulStatement {
    format!(" {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulStackAssignmentStatement\": {}, }} ", element)
}

pub fn new_yul_statement_yul_if_statement(element: YulIfStatement) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulIfStatement\": {}, }} ",
        element
    )
}

pub fn new_yul_statement_yul_for_statement(element: YulForStatement) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulForStatement\": {}, }} ",
        element
    )
}

pub fn new_yul_statement_yul_switch_statement(element: YulSwitchStatement) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulSwitchStatement\": {}, }} ",
        element
    )
}

pub fn new_yul_statement_yul_leave_statement(element: YulLeaveStatement) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulLeaveStatement\": {}, }} ",
        element
    )
}

pub fn new_yul_statement_yul_break_statement(element: YulBreakStatement) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulBreakStatement\": {}, }} ",
        element
    )
}

pub fn new_yul_statement_yul_continue_statement(element: YulContinueStatement) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulContinueStatement\": {}, }} ",
        element
    )
}

pub fn new_yul_statement_yul_variable_assignment_statement(
    element: YulVariableAssignmentStatement,
) -> YulStatement {
    format!(" {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulVariableAssignmentStatement\": {}, }} ", element)
}

pub fn new_yul_statement_yul_label(element: YulLabel) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulLabel\": {}, }} ",
        element
    )
}

pub fn new_yul_statement_yul_variable_declaration_statement(
    element: YulVariableDeclarationStatement,
) -> YulStatement {
    format!(" {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulVariableDeclarationStatement\": {}, }} ", element)
}

pub fn new_yul_statement_yul_expression(element: YulExpression) -> YulStatement {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulStatement\", \"YulExpression\": {}, }} ",
        element
    )
}

pub type YulAssignmentOperator = String;

pub fn new_yul_assignment_operator_colon_equal(element: ColonEqual) -> YulAssignmentOperator {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulAssignmentOperator\", \"ColonEqual\": {}, }} ",
        element
    )
}

pub fn new_yul_assignment_operator_yul_colon_and_equal(
    element: YulColonAndEqual,
) -> YulAssignmentOperator {
    format!(" {{  \"item\": \"Choice\", \"name\": \"YulAssignmentOperator\", \"YulColonAndEqual\": {}, }} ", element)
}

pub type YulStackAssignmentOperator = String;

pub fn new_yul_stack_assignment_operator_equal_colon(
    element: EqualColon,
) -> YulStackAssignmentOperator {
    format!(" {{  \"item\": \"Choice\", \"name\": \"YulStackAssignmentOperator\", \"EqualColon\": {}, }} ", element)
}

pub fn new_yul_stack_assignment_operator_yul_equal_and_colon(
    element: YulEqualAndColon,
) -> YulStackAssignmentOperator {
    format!(" {{  \"item\": \"Choice\", \"name\": \"YulStackAssignmentOperator\", \"YulEqualAndColon\": {}, }} ", element)
}

pub type YulSwitchCase = String;

pub fn new_yul_switch_case_yul_default_case(element: YulDefaultCase) -> YulSwitchCase {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulSwitchCase\", \"YulDefaultCase\": {}, }} ",
        element
    )
}

pub fn new_yul_switch_case_yul_value_case(element: YulValueCase) -> YulSwitchCase {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulSwitchCase\", \"YulValueCase\": {}, }} ",
        element
    )
}

pub type YulExpression = String;

pub fn new_yul_expression_yul_function_call_expression(
    element: YulFunctionCallExpression,
) -> YulExpression {
    format!(" {{  \"item\": \"Choice\", \"name\": \"YulExpression\", \"YulFunctionCallExpression\": {}, }} ", element)
}

pub fn new_yul_expression_yul_literal(element: YulLiteral) -> YulExpression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulExpression\", \"YulLiteral\": {}, }} ",
        element
    )
}

pub fn new_yul_expression_yul_path(element: YulPath) -> YulExpression {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulExpression\", \"YulPath\": {}, }} ",
        element
    )
}

pub type YulLiteral = String;

pub fn new_yul_literal_yul_true_keyword(element: YulTrueKeyword) -> YulLiteral {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulLiteral\", \"YulTrueKeyword\": {}, }} ",
        element
    )
}

pub fn new_yul_literal_yul_false_keyword(element: YulFalseKeyword) -> YulLiteral {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulLiteral\", \"YulFalseKeyword\": {}, }} ",
        element
    )
}

pub fn new_yul_literal_yul_decimal_literal(element: YulDecimalLiteral) -> YulLiteral {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulLiteral\", \"YulDecimalLiteral\": {}, }} ",
        element
    )
}

pub fn new_yul_literal_yul_hex_literal(element: YulHexLiteral) -> YulLiteral {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulLiteral\", \"YulHexLiteral\": {}, }} ",
        element
    )
}

pub fn new_yul_literal_hex_string_literal(element: HexStringLiteral) -> YulLiteral {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulLiteral\", \"HexStringLiteral\": {}, }} ",
        element
    )
}

pub fn new_yul_literal_string_literal(element: StringLiteral) -> YulLiteral {
    format!(
        " {{  \"item\": \"Choice\", \"name\": \"YulLiteral\", \"StringLiteral\": {}, }} ",
        element
    )
}

//
// Repeated & Separated
//

pub type SourceUnitMembers = String;

pub fn new_source_unit_members(elements: Vec<SourceUnitMember>) -> SourceUnitMembers {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"SourceUnitMembers\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type VersionExpressionSets = String;

pub fn new_version_expression_sets(elements: Vec<VersionExpressionSet>) -> VersionExpressionSets {
    format!(" {{  \"item\": \"Separated\", \"name\": \"VersionExpressionSets\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type VersionExpressionSet = String;

pub fn new_version_expression_set(elements: Vec<VersionExpression>) -> VersionExpressionSet {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"VersionExpressionSet\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type SimpleVersionLiteral = String;

pub fn new_simple_version_literal(elements: Vec<VersionSpecifier>) -> SimpleVersionLiteral {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"SimpleVersionLiteral\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type ImportDeconstructionSymbols = String;

pub fn new_import_deconstruction_symbols(
    elements: Vec<ImportDeconstructionSymbol>,
) -> ImportDeconstructionSymbols {
    format!(" {{  \"item\": \"Separated\", \"name\": \"ImportDeconstructionSymbols\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type UsingDeconstructionSymbols = String;

pub fn new_using_deconstruction_symbols(
    elements: Vec<UsingDeconstructionSymbol>,
) -> UsingDeconstructionSymbols {
    format!(" {{  \"item\": \"Separated\", \"name\": \"UsingDeconstructionSymbols\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type ContractSpecifiers = String;

pub fn new_contract_specifiers(elements: Vec<ContractSpecifier>) -> ContractSpecifiers {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"ContractSpecifiers\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type InheritanceTypes = String;

pub fn new_inheritance_types(elements: Vec<InheritanceType>) -> InheritanceTypes {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"InheritanceTypes\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type ContractMembers = String;

pub fn new_contract_members(elements: Vec<ContractMember>) -> ContractMembers {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"ContractMembers\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type InterfaceMembers = String;

pub fn new_interface_members(elements: Vec<ContractMember>) -> InterfaceMembers {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"InterfaceMembers\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type LibraryMembers = String;

pub fn new_library_members(elements: Vec<ContractMember>) -> LibraryMembers {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"LibraryMembers\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type StructMembers = String;

pub fn new_struct_members(elements: Vec<StructMember>) -> StructMembers {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"StructMembers\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type EnumMembers = String;

pub fn new_enum_members(elements: Vec<Identifier>) -> EnumMembers {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"EnumMembers\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type StateVariableAttributes = String;

pub fn new_state_variable_attributes(
    elements: Vec<StateVariableAttribute>,
) -> StateVariableAttributes {
    format!(" {{  \"item\": \"Separated\", \"name\": \"StateVariableAttributes\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type Parameters = String;

pub fn new_parameters(elements: Vec<Parameter>) -> Parameters {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"Parameters\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type FunctionAttributes = String;

pub fn new_function_attributes(elements: Vec<FunctionAttribute>) -> FunctionAttributes {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"FunctionAttributes\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type OverridePaths = String;

pub fn new_override_paths(elements: Vec<IdentifierPath>) -> OverridePaths {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"OverridePaths\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type ConstructorAttributes = String;

pub fn new_constructor_attributes(elements: Vec<ConstructorAttribute>) -> ConstructorAttributes {
    format!(" {{  \"item\": \"Separated\", \"name\": \"ConstructorAttributes\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type UnnamedFunctionAttributes = String;

pub fn new_unnamed_function_attributes(
    elements: Vec<UnnamedFunctionAttribute>,
) -> UnnamedFunctionAttributes {
    format!(" {{  \"item\": \"Separated\", \"name\": \"UnnamedFunctionAttributes\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type FallbackFunctionAttributes = String;

pub fn new_fallback_function_attributes(
    elements: Vec<FallbackFunctionAttribute>,
) -> FallbackFunctionAttributes {
    format!(" {{  \"item\": \"Separated\", \"name\": \"FallbackFunctionAttributes\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type ReceiveFunctionAttributes = String;

pub fn new_receive_function_attributes(
    elements: Vec<ReceiveFunctionAttribute>,
) -> ReceiveFunctionAttributes {
    format!(" {{  \"item\": \"Separated\", \"name\": \"ReceiveFunctionAttributes\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type ModifierAttributes = String;

pub fn new_modifier_attributes(elements: Vec<ModifierAttribute>) -> ModifierAttributes {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"ModifierAttributes\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type EventParameters = String;

pub fn new_event_parameters(elements: Vec<EventParameter>) -> EventParameters {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"EventParameters\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type ErrorParameters = String;

pub fn new_error_parameters(elements: Vec<ErrorParameter>) -> ErrorParameters {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"ErrorParameters\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type FunctionTypeAttributes = String;

pub fn new_function_type_attributes(
    elements: Vec<FunctionTypeAttribute>,
) -> FunctionTypeAttributes {
    format!(" {{  \"item\": \"Separated\", \"name\": \"FunctionTypeAttributes\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type Statements = String;

pub fn new_statements(elements: Vec<Statement>) -> Statements {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"Statements\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type AssemblyFlags = String;

pub fn new_assembly_flags(elements: Vec<StringLiteral>) -> AssemblyFlags {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"AssemblyFlags\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type TupleDeconstructionElements = String;

pub fn new_tuple_deconstruction_elements(
    elements: Vec<TupleDeconstructionElement>,
) -> TupleDeconstructionElements {
    format!(" {{  \"item\": \"Separated\", \"name\": \"TupleDeconstructionElements\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type CatchClauses = String;

pub fn new_catch_clauses(elements: Vec<CatchClause>) -> CatchClauses {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"CatchClauses\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type PositionalArguments = String;

pub fn new_positional_arguments(elements: Vec<Expression>) -> PositionalArguments {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"PositionalArguments\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type NamedArguments = String;

pub fn new_named_arguments(elements: Vec<NamedArgument>) -> NamedArguments {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"NamedArguments\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type CallOptions = String;

pub fn new_call_options(elements: Vec<NamedArgument>) -> CallOptions {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"CallOptions\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type TupleValues = String;

pub fn new_tuple_values(elements: Vec<TupleValue>) -> TupleValues {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"TupleValues\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type ArrayValues = String;

pub fn new_array_values(elements: Vec<Expression>) -> ArrayValues {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"ArrayValues\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type StringLiterals = String;

pub fn new_string_literals(elements: Vec<StringLiteral>) -> StringLiterals {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"StringLiterals\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type HexStringLiterals = String;

pub fn new_hex_string_literals(elements: Vec<HexStringLiteral>) -> HexStringLiterals {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"HexStringLiterals\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type UnicodeStringLiterals = String;

pub fn new_unicode_string_literals(elements: Vec<UnicodeStringLiteral>) -> UnicodeStringLiterals {
    format!(" {{  \"item\": \"Separated\", \"name\": \"UnicodeStringLiterals\", \"elements\": [{}], }} ", elements.join(", "))
}

pub type IdentifierPath = String;

pub fn new_identifier_path(elements: Vec<Identifier>) -> IdentifierPath {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"IdentifierPath\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type YulStatements = String;

pub fn new_yul_statements(elements: Vec<YulStatement>) -> YulStatements {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"YulStatements\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type YulParameters = String;

pub fn new_yul_parameters(elements: Vec<YulIdentifier>) -> YulParameters {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"YulParameters\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type YulVariableNames = String;

pub fn new_yul_variable_names(elements: Vec<YulIdentifier>) -> YulVariableNames {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"YulVariableNames\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type YulSwitchCases = String;

pub fn new_yul_switch_cases(elements: Vec<YulSwitchCase>) -> YulSwitchCases {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"YulSwitchCases\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type YulArguments = String;

pub fn new_yul_arguments(elements: Vec<YulExpression>) -> YulArguments {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"YulArguments\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type YulPaths = String;

pub fn new_yul_paths(elements: Vec<YulPath>) -> YulPaths {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"YulPaths\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

pub type YulPath = String;

pub fn new_yul_path(elements: Vec<YulIdentifier>) -> YulPath {
    format!(
        " {{  \"item\": \"Separated\", \"name\": \"YulPath\", \"elements\": [{}], }} ",
        elements.join(", ")
    )
}

//
// Terminals
//

pub type VersionSpecifier = String;

pub fn new_version_specifier<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> VersionSpecifier {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"VersionSpecifier\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SingleQuotedVersionLiteral = String;

pub fn new_single_quoted_version_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleQuotedVersionLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SingleQuotedVersionLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DoubleQuotedVersionLiteral = String;

pub fn new_double_quoted_version_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoubleQuotedVersionLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DoubleQuotedVersionLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Whitespace = String;

pub fn new_whitespace<'source>(l: usize, r: usize, source: &'source str) -> Whitespace {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Whitespace\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EndOfLine = String;

pub fn new_end_of_line<'source>(l: usize, r: usize, source: &'source str) -> EndOfLine {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EndOfLine\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SingleLineComment = String;

pub fn new_single_line_comment<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleLineComment {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SingleLineComment\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MultiLineComment = String;

pub fn new_multi_line_comment<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MultiLineComment {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MultiLineComment\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SingleLineNatSpecComment = String;

pub fn new_single_line_nat_spec_comment<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleLineNatSpecComment {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SingleLineNatSpecComment\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MultiLineNatSpecComment = String;

pub fn new_multi_line_nat_spec_comment<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MultiLineNatSpecComment {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MultiLineNatSpecComment\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbicoderKeyword_Reserved = String;

pub fn new_abicoder_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbicoderKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbicoderKeyword_Unreserved = String;

pub fn new_abicoder_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbicoderKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbicoderKeyword = String;

pub fn new_abicoder_keyword<'source>(l: usize, r: usize, source: &'source str) -> AbicoderKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbicoderKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbicoderV1Keyword_Reserved = String;

pub fn new_abicoder_v1_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderV1Keyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbicoderV1Keyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbicoderV1Keyword_Unreserved = String;

pub fn new_abicoder_v1_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderV1Keyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbicoderV1Keyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbicoderV1Keyword = String;

pub fn new_abicoder_v1_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderV1Keyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbicoderV1Keyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbicoderV2Keyword_Reserved = String;

pub fn new_abicoder_v2_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderV2Keyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbicoderV2Keyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbicoderV2Keyword_Unreserved = String;

pub fn new_abicoder_v2_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderV2Keyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbicoderV2Keyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbicoderV2Keyword = String;

pub fn new_abicoder_v2_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbicoderV2Keyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbicoderV2Keyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ABIEncoderV2Keyword_Reserved = String;

pub fn new_abi_encoder_v2_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ABIEncoderV2Keyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ABIEncoderV2Keyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ABIEncoderV2Keyword_Unreserved = String;

pub fn new_abi_encoder_v2_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ABIEncoderV2Keyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ABIEncoderV2Keyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ABIEncoderV2Keyword = String;

pub fn new_abi_encoder_v2_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ABIEncoderV2Keyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ABIEncoderV2Keyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbstractKeyword_Reserved = String;

pub fn new_abstract_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbstractKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbstractKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbstractKeyword_Unreserved = String;

pub fn new_abstract_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AbstractKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbstractKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AbstractKeyword = String;

pub fn new_abstract_keyword<'source>(l: usize, r: usize, source: &'source str) -> AbstractKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AbstractKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AddressKeyword_Reserved = String;

pub fn new_address_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AddressKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AddressKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AddressKeyword_Unreserved = String;

pub fn new_address_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AddressKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AddressKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AddressKeyword = String;

pub fn new_address_keyword<'source>(l: usize, r: usize, source: &'source str) -> AddressKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AddressKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AfterKeyword_Reserved = String;

pub fn new_after_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AfterKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AfterKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AfterKeyword_Unreserved = String;

pub fn new_after_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AfterKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AfterKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AfterKeyword = String;

pub fn new_after_keyword<'source>(l: usize, r: usize, source: &'source str) -> AfterKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AfterKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AliasKeyword_Reserved = String;

pub fn new_alias_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AliasKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AliasKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AliasKeyword_Unreserved = String;

pub fn new_alias_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AliasKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AliasKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AliasKeyword = String;

pub fn new_alias_keyword<'source>(l: usize, r: usize, source: &'source str) -> AliasKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AliasKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AnonymousKeyword_Reserved = String;

pub fn new_anonymous_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AnonymousKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AnonymousKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AnonymousKeyword_Unreserved = String;

pub fn new_anonymous_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AnonymousKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AnonymousKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AnonymousKeyword = String;

pub fn new_anonymous_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AnonymousKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AnonymousKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ApplyKeyword_Reserved = String;

pub fn new_apply_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ApplyKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ApplyKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ApplyKeyword_Unreserved = String;

pub fn new_apply_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ApplyKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ApplyKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ApplyKeyword = String;

pub fn new_apply_keyword<'source>(l: usize, r: usize, source: &'source str) -> ApplyKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ApplyKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AsKeyword_Reserved = String;

pub fn new_as_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AsKeyword_Unreserved = String;

pub fn new_as_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AsKeyword = String;

pub fn new_as_keyword<'source>(l: usize, r: usize, source: &'source str) -> AsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AssemblyKeyword_Reserved = String;

pub fn new_assembly_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AssemblyKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AssemblyKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AssemblyKeyword_Unreserved = String;

pub fn new_assembly_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AssemblyKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AssemblyKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AssemblyKeyword = String;

pub fn new_assembly_keyword<'source>(l: usize, r: usize, source: &'source str) -> AssemblyKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AssemblyKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AtKeyword_Reserved = String;

pub fn new_at_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AtKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AtKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AtKeyword_Unreserved = String;

pub fn new_at_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AtKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AtKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AtKeyword = String;

pub fn new_at_keyword<'source>(l: usize, r: usize, source: &'source str) -> AtKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AtKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AutoKeyword_Reserved = String;

pub fn new_auto_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AutoKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AutoKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AutoKeyword_Unreserved = String;

pub fn new_auto_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AutoKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AutoKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AutoKeyword = String;

pub fn new_auto_keyword<'source>(l: usize, r: usize, source: &'source str) -> AutoKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AutoKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BoolKeyword_Reserved = String;

pub fn new_bool_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> BoolKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BoolKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BoolKeyword_Unreserved = String;

pub fn new_bool_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> BoolKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BoolKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BoolKeyword = String;

pub fn new_bool_keyword<'source>(l: usize, r: usize, source: &'source str) -> BoolKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BoolKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BreakKeyword_Reserved = String;

pub fn new_break_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> BreakKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BreakKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BreakKeyword_Unreserved = String;

pub fn new_break_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> BreakKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BreakKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BreakKeyword = String;

pub fn new_break_keyword<'source>(l: usize, r: usize, source: &'source str) -> BreakKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BreakKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ByteKeyword_Reserved = String;

pub fn new_byte_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ByteKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ByteKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ByteKeyword_Unreserved = String;

pub fn new_byte_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ByteKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ByteKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ByteKeyword = String;

pub fn new_byte_keyword<'source>(l: usize, r: usize, source: &'source str) -> ByteKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ByteKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BytesKeyword_Reserved = String;

pub fn new_bytes_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> BytesKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BytesKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BytesKeyword_Unreserved = String;

pub fn new_bytes_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> BytesKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BytesKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BytesKeyword = String;

pub fn new_bytes_keyword<'source>(l: usize, r: usize, source: &'source str) -> BytesKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BytesKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CallDataKeyword_Reserved = String;

pub fn new_call_data_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CallDataKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CallDataKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CallDataKeyword_Unreserved = String;

pub fn new_call_data_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CallDataKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CallDataKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CallDataKeyword = String;

pub fn new_call_data_keyword<'source>(l: usize, r: usize, source: &'source str) -> CallDataKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CallDataKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CaseKeyword_Reserved = String;

pub fn new_case_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CaseKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CaseKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CaseKeyword_Unreserved = String;

pub fn new_case_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CaseKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CaseKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CaseKeyword = String;

pub fn new_case_keyword<'source>(l: usize, r: usize, source: &'source str) -> CaseKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CaseKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CatchKeyword_Reserved = String;

pub fn new_catch_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CatchKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CatchKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CatchKeyword_Unreserved = String;

pub fn new_catch_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CatchKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CatchKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CatchKeyword = String;

pub fn new_catch_keyword<'source>(l: usize, r: usize, source: &'source str) -> CatchKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CatchKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ConstantKeyword_Reserved = String;

pub fn new_constant_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ConstantKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ConstantKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ConstantKeyword_Unreserved = String;

pub fn new_constant_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ConstantKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ConstantKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ConstantKeyword = String;

pub fn new_constant_keyword<'source>(l: usize, r: usize, source: &'source str) -> ConstantKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ConstantKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ConstructorKeyword_Reserved = String;

pub fn new_constructor_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ConstructorKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ConstructorKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ConstructorKeyword_Unreserved = String;

pub fn new_constructor_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ConstructorKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ConstructorKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ConstructorKeyword = String;

pub fn new_constructor_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ConstructorKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ConstructorKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ContinueKeyword_Reserved = String;

pub fn new_continue_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ContinueKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ContinueKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ContinueKeyword_Unreserved = String;

pub fn new_continue_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ContinueKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ContinueKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ContinueKeyword = String;

pub fn new_continue_keyword<'source>(l: usize, r: usize, source: &'source str) -> ContinueKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ContinueKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ContractKeyword_Reserved = String;

pub fn new_contract_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ContractKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ContractKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ContractKeyword_Unreserved = String;

pub fn new_contract_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ContractKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ContractKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ContractKeyword = String;

pub fn new_contract_keyword<'source>(l: usize, r: usize, source: &'source str) -> ContractKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ContractKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CopyOfKeyword_Reserved = String;

pub fn new_copy_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CopyOfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CopyOfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CopyOfKeyword_Unreserved = String;

pub fn new_copy_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> CopyOfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CopyOfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CopyOfKeyword = String;

pub fn new_copy_of_keyword<'source>(l: usize, r: usize, source: &'source str) -> CopyOfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CopyOfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DaysKeyword_Reserved = String;

pub fn new_days_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DaysKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DaysKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DaysKeyword_Unreserved = String;

pub fn new_days_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DaysKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DaysKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DaysKeyword = String;

pub fn new_days_keyword<'source>(l: usize, r: usize, source: &'source str) -> DaysKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DaysKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DefaultKeyword_Reserved = String;

pub fn new_default_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DefaultKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DefaultKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DefaultKeyword_Unreserved = String;

pub fn new_default_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DefaultKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DefaultKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DefaultKeyword = String;

pub fn new_default_keyword<'source>(l: usize, r: usize, source: &'source str) -> DefaultKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DefaultKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DefineKeyword_Reserved = String;

pub fn new_define_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DefineKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DefineKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DefineKeyword_Unreserved = String;

pub fn new_define_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DefineKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DefineKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DefineKeyword = String;

pub fn new_define_keyword<'source>(l: usize, r: usize, source: &'source str) -> DefineKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DefineKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DeleteKeyword_Reserved = String;

pub fn new_delete_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DeleteKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DeleteKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DeleteKeyword_Unreserved = String;

pub fn new_delete_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DeleteKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DeleteKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DeleteKeyword = String;

pub fn new_delete_keyword<'source>(l: usize, r: usize, source: &'source str) -> DeleteKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DeleteKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DoKeyword_Reserved = String;

pub fn new_do_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DoKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DoKeyword_Unreserved = String;

pub fn new_do_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DoKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DoKeyword = String;

pub fn new_do_keyword<'source>(l: usize, r: usize, source: &'source str) -> DoKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DoKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ElseKeyword_Reserved = String;

pub fn new_else_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ElseKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ElseKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ElseKeyword_Unreserved = String;

pub fn new_else_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ElseKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ElseKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ElseKeyword = String;

pub fn new_else_keyword<'source>(l: usize, r: usize, source: &'source str) -> ElseKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ElseKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EmitKeyword_Reserved = String;

pub fn new_emit_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EmitKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EmitKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EmitKeyword_Unreserved = String;

pub fn new_emit_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EmitKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EmitKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EmitKeyword = String;

pub fn new_emit_keyword<'source>(l: usize, r: usize, source: &'source str) -> EmitKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EmitKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EnumKeyword_Reserved = String;

pub fn new_enum_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EnumKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EnumKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EnumKeyword_Unreserved = String;

pub fn new_enum_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EnumKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EnumKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EnumKeyword = String;

pub fn new_enum_keyword<'source>(l: usize, r: usize, source: &'source str) -> EnumKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EnumKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ErrorKeyword_Reserved = String;

pub fn new_error_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ErrorKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ErrorKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ErrorKeyword_Unreserved = String;

pub fn new_error_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ErrorKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ErrorKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ErrorKeyword = String;

pub fn new_error_keyword<'source>(l: usize, r: usize, source: &'source str) -> ErrorKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ErrorKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EtherKeyword_Reserved = String;

pub fn new_ether_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EtherKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EtherKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EtherKeyword_Unreserved = String;

pub fn new_ether_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EtherKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EtherKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EtherKeyword = String;

pub fn new_ether_keyword<'source>(l: usize, r: usize, source: &'source str) -> EtherKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EtherKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EventKeyword_Reserved = String;

pub fn new_event_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EventKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EventKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EventKeyword_Unreserved = String;

pub fn new_event_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EventKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EventKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EventKeyword = String;

pub fn new_event_keyword<'source>(l: usize, r: usize, source: &'source str) -> EventKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EventKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ExperimentalKeyword_Reserved = String;

pub fn new_experimental_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ExperimentalKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ExperimentalKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ExperimentalKeyword_Unreserved = String;

pub fn new_experimental_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ExperimentalKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ExperimentalKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ExperimentalKeyword = String;

pub fn new_experimental_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ExperimentalKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ExperimentalKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ExternalKeyword_Reserved = String;

pub fn new_external_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ExternalKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ExternalKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ExternalKeyword_Unreserved = String;

pub fn new_external_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ExternalKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ExternalKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ExternalKeyword = String;

pub fn new_external_keyword<'source>(l: usize, r: usize, source: &'source str) -> ExternalKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ExternalKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FallbackKeyword_Reserved = String;

pub fn new_fallback_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FallbackKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FallbackKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FallbackKeyword_Unreserved = String;

pub fn new_fallback_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FallbackKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FallbackKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FallbackKeyword = String;

pub fn new_fallback_keyword<'source>(l: usize, r: usize, source: &'source str) -> FallbackKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FallbackKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FalseKeyword_Reserved = String;

pub fn new_false_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FalseKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FalseKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FalseKeyword_Unreserved = String;

pub fn new_false_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FalseKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FalseKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FalseKeyword = String;

pub fn new_false_keyword<'source>(l: usize, r: usize, source: &'source str) -> FalseKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FalseKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FinalKeyword_Reserved = String;

pub fn new_final_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FinalKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FinalKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FinalKeyword_Unreserved = String;

pub fn new_final_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FinalKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FinalKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FinalKeyword = String;

pub fn new_final_keyword<'source>(l: usize, r: usize, source: &'source str) -> FinalKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FinalKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FinneyKeyword_Reserved = String;

pub fn new_finney_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FinneyKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FinneyKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FinneyKeyword_Unreserved = String;

pub fn new_finney_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FinneyKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FinneyKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FinneyKeyword = String;

pub fn new_finney_keyword<'source>(l: usize, r: usize, source: &'source str) -> FinneyKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FinneyKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FixedKeyword_Reserved = String;

pub fn new_fixed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FixedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FixedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FixedKeyword_Unreserved = String;

pub fn new_fixed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FixedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FixedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FixedKeyword = String;

pub fn new_fixed_keyword<'source>(l: usize, r: usize, source: &'source str) -> FixedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FixedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ForKeyword_Reserved = String;

pub fn new_for_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ForKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ForKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ForKeyword_Unreserved = String;

pub fn new_for_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ForKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ForKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ForKeyword = String;

pub fn new_for_keyword<'source>(l: usize, r: usize, source: &'source str) -> ForKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ForKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FromKeyword_Reserved = String;

pub fn new_from_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FromKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FromKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FromKeyword_Unreserved = String;

pub fn new_from_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FromKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FromKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FromKeyword = String;

pub fn new_from_keyword<'source>(l: usize, r: usize, source: &'source str) -> FromKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FromKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FunctionKeyword_Reserved = String;

pub fn new_function_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FunctionKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FunctionKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FunctionKeyword_Unreserved = String;

pub fn new_function_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> FunctionKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FunctionKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type FunctionKeyword = String;

pub fn new_function_keyword<'source>(l: usize, r: usize, source: &'source str) -> FunctionKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"FunctionKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GlobalKeyword_Reserved = String;

pub fn new_global_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GlobalKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GlobalKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GlobalKeyword_Unreserved = String;

pub fn new_global_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GlobalKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GlobalKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GlobalKeyword = String;

pub fn new_global_keyword<'source>(l: usize, r: usize, source: &'source str) -> GlobalKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GlobalKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GweiKeyword_Reserved = String;

pub fn new_gwei_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GweiKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GweiKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GweiKeyword_Unreserved = String;

pub fn new_gwei_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GweiKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GweiKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GweiKeyword = String;

pub fn new_gwei_keyword<'source>(l: usize, r: usize, source: &'source str) -> GweiKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GweiKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type HexKeyword_Reserved = String;

pub fn new_hex_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> HexKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"HexKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type HexKeyword_Unreserved = String;

pub fn new_hex_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> HexKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"HexKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type HexKeyword = String;

pub fn new_hex_keyword<'source>(l: usize, r: usize, source: &'source str) -> HexKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"HexKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type HoursKeyword_Reserved = String;

pub fn new_hours_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> HoursKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"HoursKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type HoursKeyword_Unreserved = String;

pub fn new_hours_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> HoursKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"HoursKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type HoursKeyword = String;

pub fn new_hours_keyword<'source>(l: usize, r: usize, source: &'source str) -> HoursKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"HoursKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IfKeyword_Reserved = String;

pub fn new_if_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IfKeyword_Unreserved = String;

pub fn new_if_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IfKeyword = String;

pub fn new_if_keyword<'source>(l: usize, r: usize, source: &'source str) -> IfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ImmutableKeyword_Reserved = String;

pub fn new_immutable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImmutableKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ImmutableKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ImmutableKeyword_Unreserved = String;

pub fn new_immutable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImmutableKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ImmutableKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ImmutableKeyword = String;

pub fn new_immutable_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImmutableKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ImmutableKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ImplementsKeyword_Reserved = String;

pub fn new_implements_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImplementsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ImplementsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ImplementsKeyword_Unreserved = String;

pub fn new_implements_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImplementsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ImplementsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ImplementsKeyword = String;

pub fn new_implements_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImplementsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ImplementsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ImportKeyword_Reserved = String;

pub fn new_import_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImportKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ImportKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ImportKeyword_Unreserved = String;

pub fn new_import_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ImportKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ImportKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ImportKeyword = String;

pub fn new_import_keyword<'source>(l: usize, r: usize, source: &'source str) -> ImportKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ImportKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IndexedKeyword_Reserved = String;

pub fn new_indexed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IndexedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IndexedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IndexedKeyword_Unreserved = String;

pub fn new_indexed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IndexedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IndexedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IndexedKeyword = String;

pub fn new_indexed_keyword<'source>(l: usize, r: usize, source: &'source str) -> IndexedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IndexedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InKeyword_Reserved = String;

pub fn new_in_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InKeyword_Unreserved = String;

pub fn new_in_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InKeyword = String;

pub fn new_in_keyword<'source>(l: usize, r: usize, source: &'source str) -> InKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InlineKeyword_Reserved = String;

pub fn new_inline_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InlineKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InlineKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InlineKeyword_Unreserved = String;

pub fn new_inline_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InlineKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InlineKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InlineKeyword = String;

pub fn new_inline_keyword<'source>(l: usize, r: usize, source: &'source str) -> InlineKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InlineKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InterfaceKeyword_Reserved = String;

pub fn new_interface_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InterfaceKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InterfaceKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InterfaceKeyword_Unreserved = String;

pub fn new_interface_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InterfaceKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InterfaceKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InterfaceKeyword = String;

pub fn new_interface_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InterfaceKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InterfaceKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InternalKeyword_Reserved = String;

pub fn new_internal_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InternalKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InternalKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InternalKeyword_Unreserved = String;

pub fn new_internal_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> InternalKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InternalKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type InternalKeyword = String;

pub fn new_internal_keyword<'source>(l: usize, r: usize, source: &'source str) -> InternalKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"InternalKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IntKeyword_Reserved = String;

pub fn new_int_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IntKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IntKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IntKeyword_Unreserved = String;

pub fn new_int_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IntKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IntKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IntKeyword = String;

pub fn new_int_keyword<'source>(l: usize, r: usize, source: &'source str) -> IntKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IntKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IsKeyword_Reserved = String;

pub fn new_is_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IsKeyword_Unreserved = String;

pub fn new_is_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> IsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type IsKeyword = String;

pub fn new_is_keyword<'source>(l: usize, r: usize, source: &'source str) -> IsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"IsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LayoutKeyword_Reserved = String;

pub fn new_layout_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LayoutKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LayoutKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LayoutKeyword_Unreserved = String;

pub fn new_layout_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LayoutKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LayoutKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LayoutKeyword = String;

pub fn new_layout_keyword<'source>(l: usize, r: usize, source: &'source str) -> LayoutKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LayoutKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LetKeyword_Reserved = String;

pub fn new_let_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LetKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LetKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LetKeyword_Unreserved = String;

pub fn new_let_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LetKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LetKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LetKeyword = String;

pub fn new_let_keyword<'source>(l: usize, r: usize, source: &'source str) -> LetKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LetKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LibraryKeyword_Reserved = String;

pub fn new_library_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LibraryKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LibraryKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LibraryKeyword_Unreserved = String;

pub fn new_library_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LibraryKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LibraryKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LibraryKeyword = String;

pub fn new_library_keyword<'source>(l: usize, r: usize, source: &'source str) -> LibraryKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LibraryKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MacroKeyword_Reserved = String;

pub fn new_macro_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MacroKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MacroKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MacroKeyword_Unreserved = String;

pub fn new_macro_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MacroKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MacroKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MacroKeyword = String;

pub fn new_macro_keyword<'source>(l: usize, r: usize, source: &'source str) -> MacroKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MacroKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MappingKeyword_Reserved = String;

pub fn new_mapping_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MappingKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MappingKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MappingKeyword_Unreserved = String;

pub fn new_mapping_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MappingKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MappingKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MappingKeyword = String;

pub fn new_mapping_keyword<'source>(l: usize, r: usize, source: &'source str) -> MappingKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MappingKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MatchKeyword_Reserved = String;

pub fn new_match_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MatchKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MatchKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MatchKeyword_Unreserved = String;

pub fn new_match_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MatchKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MatchKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MatchKeyword = String;

pub fn new_match_keyword<'source>(l: usize, r: usize, source: &'source str) -> MatchKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MatchKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MemoryKeyword_Reserved = String;

pub fn new_memory_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MemoryKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MemoryKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MemoryKeyword_Unreserved = String;

pub fn new_memory_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MemoryKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MemoryKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MemoryKeyword = String;

pub fn new_memory_keyword<'source>(l: usize, r: usize, source: &'source str) -> MemoryKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MemoryKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MinutesKeyword_Reserved = String;

pub fn new_minutes_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MinutesKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MinutesKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MinutesKeyword_Unreserved = String;

pub fn new_minutes_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MinutesKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MinutesKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MinutesKeyword = String;

pub fn new_minutes_keyword<'source>(l: usize, r: usize, source: &'source str) -> MinutesKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MinutesKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ModifierKeyword_Reserved = String;

pub fn new_modifier_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ModifierKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ModifierKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ModifierKeyword_Unreserved = String;

pub fn new_modifier_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ModifierKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ModifierKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ModifierKeyword = String;

pub fn new_modifier_keyword<'source>(l: usize, r: usize, source: &'source str) -> ModifierKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ModifierKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MutableKeyword_Reserved = String;

pub fn new_mutable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MutableKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MutableKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MutableKeyword_Unreserved = String;

pub fn new_mutable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MutableKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MutableKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MutableKeyword = String;

pub fn new_mutable_keyword<'source>(l: usize, r: usize, source: &'source str) -> MutableKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MutableKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type NewKeyword_Reserved = String;

pub fn new_new_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> NewKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"NewKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type NewKeyword_Unreserved = String;

pub fn new_new_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> NewKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"NewKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type NewKeyword = String;

pub fn new_new_keyword<'source>(l: usize, r: usize, source: &'source str) -> NewKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"NewKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type NullKeyword_Reserved = String;

pub fn new_null_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> NullKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"NullKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type NullKeyword_Unreserved = String;

pub fn new_null_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> NullKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"NullKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type NullKeyword = String;

pub fn new_null_keyword<'source>(l: usize, r: usize, source: &'source str) -> NullKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"NullKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type OfKeyword_Reserved = String;

pub fn new_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> OfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"OfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type OfKeyword_Unreserved = String;

pub fn new_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> OfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"OfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type OfKeyword = String;

pub fn new_of_keyword<'source>(l: usize, r: usize, source: &'source str) -> OfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"OfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type OverrideKeyword_Reserved = String;

pub fn new_override_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> OverrideKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"OverrideKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type OverrideKeyword_Unreserved = String;

pub fn new_override_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> OverrideKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"OverrideKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type OverrideKeyword = String;

pub fn new_override_keyword<'source>(l: usize, r: usize, source: &'source str) -> OverrideKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"OverrideKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PartialKeyword_Reserved = String;

pub fn new_partial_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PartialKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PartialKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PartialKeyword_Unreserved = String;

pub fn new_partial_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PartialKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PartialKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PartialKeyword = String;

pub fn new_partial_keyword<'source>(l: usize, r: usize, source: &'source str) -> PartialKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PartialKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PayableKeyword_Reserved = String;

pub fn new_payable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PayableKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PayableKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PayableKeyword_Unreserved = String;

pub fn new_payable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PayableKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PayableKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PayableKeyword = String;

pub fn new_payable_keyword<'source>(l: usize, r: usize, source: &'source str) -> PayableKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PayableKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PragmaKeyword_Reserved = String;

pub fn new_pragma_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PragmaKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PragmaKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PragmaKeyword_Unreserved = String;

pub fn new_pragma_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PragmaKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PragmaKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PragmaKeyword = String;

pub fn new_pragma_keyword<'source>(l: usize, r: usize, source: &'source str) -> PragmaKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PragmaKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PrivateKeyword_Reserved = String;

pub fn new_private_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PrivateKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PrivateKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PrivateKeyword_Unreserved = String;

pub fn new_private_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PrivateKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PrivateKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PrivateKeyword = String;

pub fn new_private_keyword<'source>(l: usize, r: usize, source: &'source str) -> PrivateKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PrivateKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PromiseKeyword_Reserved = String;

pub fn new_promise_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PromiseKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PromiseKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PromiseKeyword_Unreserved = String;

pub fn new_promise_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PromiseKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PromiseKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PromiseKeyword = String;

pub fn new_promise_keyword<'source>(l: usize, r: usize, source: &'source str) -> PromiseKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PromiseKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PublicKeyword_Reserved = String;

pub fn new_public_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PublicKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PublicKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PublicKeyword_Unreserved = String;

pub fn new_public_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PublicKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PublicKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PublicKeyword = String;

pub fn new_public_keyword<'source>(l: usize, r: usize, source: &'source str) -> PublicKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PublicKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PureKeyword_Reserved = String;

pub fn new_pure_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PureKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PureKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PureKeyword_Unreserved = String;

pub fn new_pure_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> PureKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PureKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PureKeyword = String;

pub fn new_pure_keyword<'source>(l: usize, r: usize, source: &'source str) -> PureKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PureKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReceiveKeyword_Reserved = String;

pub fn new_receive_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReceiveKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReceiveKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReceiveKeyword_Unreserved = String;

pub fn new_receive_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReceiveKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReceiveKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReceiveKeyword = String;

pub fn new_receive_keyword<'source>(l: usize, r: usize, source: &'source str) -> ReceiveKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReceiveKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReferenceKeyword_Reserved = String;

pub fn new_reference_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReferenceKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReferenceKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReferenceKeyword_Unreserved = String;

pub fn new_reference_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReferenceKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReferenceKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReferenceKeyword = String;

pub fn new_reference_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReferenceKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReferenceKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type RelocatableKeyword_Reserved = String;

pub fn new_relocatable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> RelocatableKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"RelocatableKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type RelocatableKeyword_Unreserved = String;

pub fn new_relocatable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> RelocatableKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"RelocatableKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type RelocatableKeyword = String;

pub fn new_relocatable_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> RelocatableKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"RelocatableKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReturnKeyword_Reserved = String;

pub fn new_return_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReturnKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReturnKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReturnKeyword_Unreserved = String;

pub fn new_return_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReturnKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReturnKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReturnKeyword = String;

pub fn new_return_keyword<'source>(l: usize, r: usize, source: &'source str) -> ReturnKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReturnKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReturnsKeyword_Reserved = String;

pub fn new_returns_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReturnsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReturnsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReturnsKeyword_Unreserved = String;

pub fn new_returns_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ReturnsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReturnsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ReturnsKeyword = String;

pub fn new_returns_keyword<'source>(l: usize, r: usize, source: &'source str) -> ReturnsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ReturnsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type RevertKeyword_Reserved = String;

pub fn new_revert_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> RevertKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"RevertKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type RevertKeyword_Unreserved = String;

pub fn new_revert_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> RevertKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"RevertKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type RevertKeyword = String;

pub fn new_revert_keyword<'source>(l: usize, r: usize, source: &'source str) -> RevertKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"RevertKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SealedKeyword_Reserved = String;

pub fn new_sealed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SealedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SealedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SealedKeyword_Unreserved = String;

pub fn new_sealed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SealedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SealedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SealedKeyword = String;

pub fn new_sealed_keyword<'source>(l: usize, r: usize, source: &'source str) -> SealedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SealedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SecondsKeyword_Reserved = String;

pub fn new_seconds_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SecondsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SecondsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SecondsKeyword_Unreserved = String;

pub fn new_seconds_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SecondsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SecondsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SecondsKeyword = String;

pub fn new_seconds_keyword<'source>(l: usize, r: usize, source: &'source str) -> SecondsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SecondsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SizeOfKeyword_Reserved = String;

pub fn new_size_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SizeOfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SizeOfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SizeOfKeyword_Unreserved = String;

pub fn new_size_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SizeOfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SizeOfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SizeOfKeyword = String;

pub fn new_size_of_keyword<'source>(l: usize, r: usize, source: &'source str) -> SizeOfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SizeOfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SMTCheckerKeyword_Reserved = String;

pub fn new_smt_checker_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SMTCheckerKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SMTCheckerKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SMTCheckerKeyword_Unreserved = String;

pub fn new_smt_checker_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SMTCheckerKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SMTCheckerKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SMTCheckerKeyword = String;

pub fn new_smt_checker_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SMTCheckerKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SMTCheckerKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SolidityKeyword_Reserved = String;

pub fn new_solidity_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SolidityKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SolidityKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SolidityKeyword_Unreserved = String;

pub fn new_solidity_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SolidityKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SolidityKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SolidityKeyword = String;

pub fn new_solidity_keyword<'source>(l: usize, r: usize, source: &'source str) -> SolidityKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SolidityKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StaticKeyword_Reserved = String;

pub fn new_static_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StaticKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StaticKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StaticKeyword_Unreserved = String;

pub fn new_static_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StaticKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StaticKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StaticKeyword = String;

pub fn new_static_keyword<'source>(l: usize, r: usize, source: &'source str) -> StaticKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StaticKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StorageKeyword_Reserved = String;

pub fn new_storage_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StorageKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StorageKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StorageKeyword_Unreserved = String;

pub fn new_storage_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StorageKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StorageKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StorageKeyword = String;

pub fn new_storage_keyword<'source>(l: usize, r: usize, source: &'source str) -> StorageKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StorageKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StringKeyword_Reserved = String;

pub fn new_string_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StringKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StringKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StringKeyword_Unreserved = String;

pub fn new_string_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StringKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StringKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StringKeyword = String;

pub fn new_string_keyword<'source>(l: usize, r: usize, source: &'source str) -> StringKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StringKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StructKeyword_Reserved = String;

pub fn new_struct_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StructKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StructKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StructKeyword_Unreserved = String;

pub fn new_struct_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> StructKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StructKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type StructKeyword = String;

pub fn new_struct_keyword<'source>(l: usize, r: usize, source: &'source str) -> StructKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"StructKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SuperKeyword_Reserved = String;

pub fn new_super_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SuperKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SuperKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SuperKeyword_Unreserved = String;

pub fn new_super_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SuperKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SuperKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SuperKeyword = String;

pub fn new_super_keyword<'source>(l: usize, r: usize, source: &'source str) -> SuperKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SuperKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SupportsKeyword_Reserved = String;

pub fn new_supports_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SupportsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SupportsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SupportsKeyword_Unreserved = String;

pub fn new_supports_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SupportsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SupportsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SupportsKeyword = String;

pub fn new_supports_keyword<'source>(l: usize, r: usize, source: &'source str) -> SupportsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SupportsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SwitchKeyword_Reserved = String;

pub fn new_switch_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SwitchKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SwitchKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SwitchKeyword_Unreserved = String;

pub fn new_switch_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SwitchKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SwitchKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SwitchKeyword = String;

pub fn new_switch_keyword<'source>(l: usize, r: usize, source: &'source str) -> SwitchKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SwitchKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SzaboKeyword_Reserved = String;

pub fn new_szabo_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SzaboKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SzaboKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SzaboKeyword_Unreserved = String;

pub fn new_szabo_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SzaboKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SzaboKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SzaboKeyword = String;

pub fn new_szabo_keyword<'source>(l: usize, r: usize, source: &'source str) -> SzaboKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SzaboKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ThisKeyword_Reserved = String;

pub fn new_this_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ThisKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ThisKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ThisKeyword_Unreserved = String;

pub fn new_this_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ThisKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ThisKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ThisKeyword = String;

pub fn new_this_keyword<'source>(l: usize, r: usize, source: &'source str) -> ThisKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ThisKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ThrowKeyword_Reserved = String;

pub fn new_throw_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ThrowKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ThrowKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ThrowKeyword_Unreserved = String;

pub fn new_throw_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ThrowKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ThrowKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ThrowKeyword = String;

pub fn new_throw_keyword<'source>(l: usize, r: usize, source: &'source str) -> ThrowKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ThrowKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TransientKeyword_Reserved = String;

pub fn new_transient_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TransientKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TransientKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TransientKeyword_Unreserved = String;

pub fn new_transient_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TransientKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TransientKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TransientKeyword = String;

pub fn new_transient_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TransientKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TransientKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TrueKeyword_Reserved = String;

pub fn new_true_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TrueKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TrueKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TrueKeyword_Unreserved = String;

pub fn new_true_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TrueKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TrueKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TrueKeyword = String;

pub fn new_true_keyword<'source>(l: usize, r: usize, source: &'source str) -> TrueKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TrueKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TryKeyword_Reserved = String;

pub fn new_try_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TryKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TryKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TryKeyword_Unreserved = String;

pub fn new_try_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TryKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TryKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TryKeyword = String;

pub fn new_try_keyword<'source>(l: usize, r: usize, source: &'source str) -> TryKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TryKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TypeDefKeyword_Reserved = String;

pub fn new_type_def_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeDefKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TypeDefKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TypeDefKeyword_Unreserved = String;

pub fn new_type_def_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeDefKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TypeDefKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TypeDefKeyword = String;

pub fn new_type_def_keyword<'source>(l: usize, r: usize, source: &'source str) -> TypeDefKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TypeDefKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TypeKeyword_Reserved = String;

pub fn new_type_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TypeKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TypeKeyword_Unreserved = String;

pub fn new_type_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TypeKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TypeKeyword = String;

pub fn new_type_keyword<'source>(l: usize, r: usize, source: &'source str) -> TypeKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TypeKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TypeOfKeyword_Reserved = String;

pub fn new_type_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeOfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TypeOfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TypeOfKeyword_Unreserved = String;

pub fn new_type_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> TypeOfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TypeOfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type TypeOfKeyword = String;

pub fn new_type_of_keyword<'source>(l: usize, r: usize, source: &'source str) -> TypeOfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"TypeOfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UfixedKeyword_Reserved = String;

pub fn new_ufixed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UfixedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UfixedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UfixedKeyword_Unreserved = String;

pub fn new_ufixed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UfixedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UfixedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UfixedKeyword = String;

pub fn new_ufixed_keyword<'source>(l: usize, r: usize, source: &'source str) -> UfixedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UfixedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UintKeyword_Reserved = String;

pub fn new_uint_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UintKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UintKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UintKeyword_Unreserved = String;

pub fn new_uint_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UintKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UintKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UintKeyword = String;

pub fn new_uint_keyword<'source>(l: usize, r: usize, source: &'source str) -> UintKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UintKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UncheckedKeyword_Reserved = String;

pub fn new_unchecked_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UncheckedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UncheckedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UncheckedKeyword_Unreserved = String;

pub fn new_unchecked_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UncheckedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UncheckedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UncheckedKeyword = String;

pub fn new_unchecked_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UncheckedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UncheckedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UsingKeyword_Reserved = String;

pub fn new_using_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UsingKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UsingKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UsingKeyword_Unreserved = String;

pub fn new_using_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> UsingKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UsingKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type UsingKeyword = String;

pub fn new_using_keyword<'source>(l: usize, r: usize, source: &'source str) -> UsingKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"UsingKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type VarKeyword_Reserved = String;

pub fn new_var_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> VarKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"VarKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type VarKeyword_Unreserved = String;

pub fn new_var_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> VarKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"VarKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type VarKeyword = String;

pub fn new_var_keyword<'source>(l: usize, r: usize, source: &'source str) -> VarKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"VarKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ViewKeyword_Reserved = String;

pub fn new_view_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ViewKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ViewKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ViewKeyword_Unreserved = String;

pub fn new_view_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> ViewKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ViewKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ViewKeyword = String;

pub fn new_view_keyword<'source>(l: usize, r: usize, source: &'source str) -> ViewKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ViewKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type VirtualKeyword_Reserved = String;

pub fn new_virtual_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> VirtualKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"VirtualKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type VirtualKeyword_Unreserved = String;

pub fn new_virtual_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> VirtualKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"VirtualKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type VirtualKeyword = String;

pub fn new_virtual_keyword<'source>(l: usize, r: usize, source: &'source str) -> VirtualKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"VirtualKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type WeeksKeyword_Reserved = String;

pub fn new_weeks_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> WeeksKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"WeeksKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type WeeksKeyword_Unreserved = String;

pub fn new_weeks_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> WeeksKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"WeeksKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type WeeksKeyword = String;

pub fn new_weeks_keyword<'source>(l: usize, r: usize, source: &'source str) -> WeeksKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"WeeksKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type WeiKeyword_Reserved = String;

pub fn new_wei_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> WeiKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"WeiKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type WeiKeyword_Unreserved = String;

pub fn new_wei_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> WeiKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"WeiKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type WeiKeyword = String;

pub fn new_wei_keyword<'source>(l: usize, r: usize, source: &'source str) -> WeiKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"WeiKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type WhileKeyword_Reserved = String;

pub fn new_while_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> WhileKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"WhileKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type WhileKeyword_Unreserved = String;

pub fn new_while_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> WhileKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"WhileKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type WhileKeyword = String;

pub fn new_while_keyword<'source>(l: usize, r: usize, source: &'source str) -> WhileKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"WhileKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YearsKeyword_Reserved = String;

pub fn new_years_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YearsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YearsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YearsKeyword_Unreserved = String;

pub fn new_years_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YearsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YearsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YearsKeyword = String;

pub fn new_years_keyword<'source>(l: usize, r: usize, source: &'source str) -> YearsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YearsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type OpenParen = String;

pub fn new_open_paren<'source>(l: usize, r: usize, source: &'source str) -> OpenParen {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"OpenParen\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CloseParen = String;

pub fn new_close_paren<'source>(l: usize, r: usize, source: &'source str) -> CloseParen {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CloseParen\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type OpenBracket = String;

pub fn new_open_bracket<'source>(l: usize, r: usize, source: &'source str) -> OpenBracket {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"OpenBracket\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CloseBracket = String;

pub fn new_close_bracket<'source>(l: usize, r: usize, source: &'source str) -> CloseBracket {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CloseBracket\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type OpenBrace = String;

pub fn new_open_brace<'source>(l: usize, r: usize, source: &'source str) -> OpenBrace {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"OpenBrace\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CloseBrace = String;

pub fn new_close_brace<'source>(l: usize, r: usize, source: &'source str) -> CloseBrace {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CloseBrace\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Comma = String;

pub fn new_comma<'source>(l: usize, r: usize, source: &'source str) -> Comma {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Comma\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Period = String;

pub fn new_period<'source>(l: usize, r: usize, source: &'source str) -> Period {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Period\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type QuestionMark = String;

pub fn new_question_mark<'source>(l: usize, r: usize, source: &'source str) -> QuestionMark {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"QuestionMark\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Semicolon = String;

pub fn new_semicolon<'source>(l: usize, r: usize, source: &'source str) -> Semicolon {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Semicolon\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Colon = String;

pub fn new_colon<'source>(l: usize, r: usize, source: &'source str) -> Colon {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Colon\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type ColonEqual = String;

pub fn new_colon_equal<'source>(l: usize, r: usize, source: &'source str) -> ColonEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"ColonEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Equal = String;

pub fn new_equal<'source>(l: usize, r: usize, source: &'source str) -> Equal {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Equal\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EqualColon = String;

pub fn new_equal_colon<'source>(l: usize, r: usize, source: &'source str) -> EqualColon {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EqualColon\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EqualEqual = String;

pub fn new_equal_equal<'source>(l: usize, r: usize, source: &'source str) -> EqualEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EqualEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type EqualGreaterThan = String;

pub fn new_equal_greater_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> EqualGreaterThan {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"EqualGreaterThan\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Asterisk = String;

pub fn new_asterisk<'source>(l: usize, r: usize, source: &'source str) -> Asterisk {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Asterisk\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AsteriskEqual = String;

pub fn new_asterisk_equal<'source>(l: usize, r: usize, source: &'source str) -> AsteriskEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AsteriskEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AsteriskAsterisk = String;

pub fn new_asterisk_asterisk<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AsteriskAsterisk {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AsteriskAsterisk\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Bar = String;

pub fn new_bar<'source>(l: usize, r: usize, source: &'source str) -> Bar {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Bar\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BarEqual = String;

pub fn new_bar_equal<'source>(l: usize, r: usize, source: &'source str) -> BarEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BarEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BarBar = String;

pub fn new_bar_bar<'source>(l: usize, r: usize, source: &'source str) -> BarBar {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BarBar\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Ampersand = String;

pub fn new_ampersand<'source>(l: usize, r: usize, source: &'source str) -> Ampersand {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Ampersand\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AmpersandEqual = String;

pub fn new_ampersand_equal<'source>(l: usize, r: usize, source: &'source str) -> AmpersandEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AmpersandEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type AmpersandAmpersand = String;

pub fn new_ampersand_ampersand<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> AmpersandAmpersand {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"AmpersandAmpersand\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LessThan = String;

pub fn new_less_than<'source>(l: usize, r: usize, source: &'source str) -> LessThan {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LessThan\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LessThanEqual = String;

pub fn new_less_than_equal<'source>(l: usize, r: usize, source: &'source str) -> LessThanEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LessThanEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LessThanLessThan = String;

pub fn new_less_than_less_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LessThanLessThan {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LessThanLessThan\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type LessThanLessThanEqual = String;

pub fn new_less_than_less_than_equal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> LessThanLessThanEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"LessThanLessThanEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GreaterThan = String;

pub fn new_greater_than<'source>(l: usize, r: usize, source: &'source str) -> GreaterThan {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GreaterThan\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GreaterThanEqual = String;

pub fn new_greater_than_equal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GreaterThanEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GreaterThanGreaterThan = String;

pub fn new_greater_than_greater_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanGreaterThan {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GreaterThanGreaterThan\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GreaterThanGreaterThanEqual = String;

pub fn new_greater_than_greater_than_equal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanGreaterThanEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GreaterThanGreaterThanEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GreaterThanGreaterThanGreaterThan = String;

pub fn new_greater_than_greater_than_greater_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanGreaterThanGreaterThan {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GreaterThanGreaterThanGreaterThan\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type GreaterThanGreaterThanGreaterThanEqual = String;

pub fn new_greater_than_greater_than_greater_than_equal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> GreaterThanGreaterThanGreaterThanEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GreaterThanGreaterThanGreaterThanEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Plus = String;

pub fn new_plus<'source>(l: usize, r: usize, source: &'source str) -> Plus {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Plus\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PlusEqual = String;

pub fn new_plus_equal<'source>(l: usize, r: usize, source: &'source str) -> PlusEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PlusEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PlusPlus = String;

pub fn new_plus_plus<'source>(l: usize, r: usize, source: &'source str) -> PlusPlus {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PlusPlus\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Minus = String;

pub fn new_minus<'source>(l: usize, r: usize, source: &'source str) -> Minus {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Minus\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MinusEqual = String;

pub fn new_minus_equal<'source>(l: usize, r: usize, source: &'source str) -> MinusEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MinusEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MinusMinus = String;

pub fn new_minus_minus<'source>(l: usize, r: usize, source: &'source str) -> MinusMinus {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MinusMinus\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type MinusGreaterThan = String;

pub fn new_minus_greater_than<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> MinusGreaterThan {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"MinusGreaterThan\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Slash = String;

pub fn new_slash<'source>(l: usize, r: usize, source: &'source str) -> Slash {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Slash\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SlashEqual = String;

pub fn new_slash_equal<'source>(l: usize, r: usize, source: &'source str) -> SlashEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SlashEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Percent = String;

pub fn new_percent<'source>(l: usize, r: usize, source: &'source str) -> Percent {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Percent\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type PercentEqual = String;

pub fn new_percent_equal<'source>(l: usize, r: usize, source: &'source str) -> PercentEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"PercentEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Bang = String;

pub fn new_bang<'source>(l: usize, r: usize, source: &'source str) -> Bang {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Bang\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type BangEqual = String;

pub fn new_bang_equal<'source>(l: usize, r: usize, source: &'source str) -> BangEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"BangEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Caret = String;

pub fn new_caret<'source>(l: usize, r: usize, source: &'source str) -> Caret {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Caret\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type CaretEqual = String;

pub fn new_caret_equal<'source>(l: usize, r: usize, source: &'source str) -> CaretEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"CaretEqual\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Tilde = String;

pub fn new_tilde<'source>(l: usize, r: usize, source: &'source str) -> Tilde {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Tilde\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type HexLiteral = String;

pub fn new_hex_literal<'source>(l: usize, r: usize, source: &'source str) -> HexLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"HexLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DecimalLiteral = String;

pub fn new_decimal_literal<'source>(l: usize, r: usize, source: &'source str) -> DecimalLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DecimalLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SingleQuotedStringLiteral = String;

pub fn new_single_quoted_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleQuotedStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SingleQuotedStringLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DoubleQuotedStringLiteral = String;

pub fn new_double_quoted_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoubleQuotedStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DoubleQuotedStringLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SingleQuotedHexStringLiteral = String;

pub fn new_single_quoted_hex_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleQuotedHexStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SingleQuotedHexStringLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DoubleQuotedHexStringLiteral = String;

pub fn new_double_quoted_hex_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoubleQuotedHexStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DoubleQuotedHexStringLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type SingleQuotedUnicodeStringLiteral = String;

pub fn new_single_quoted_unicode_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> SingleQuotedUnicodeStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SingleQuotedUnicodeStringLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type DoubleQuotedUnicodeStringLiteral = String;

pub fn new_double_quoted_unicode_string_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> DoubleQuotedUnicodeStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DoubleQuotedUnicodeStringLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type Identifier = String;

pub fn new_identifier<'source>(l: usize, r: usize, source: &'source str) -> Identifier {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"Identifier\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIdentifier = String;

pub fn new_yul_identifier<'source>(l: usize, r: usize, source: &'source str) -> YulIdentifier {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIdentifier\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDecimalLiteral = String;

pub fn new_yul_decimal_literal<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDecimalLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDecimalLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulHexLiteral = String;

pub fn new_yul_hex_literal<'source>(l: usize, r: usize, source: &'source str) -> YulHexLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulHexLiteral\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAbstractKeyword_Reserved = String;

pub fn new_yul_abstract_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAbstractKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAbstractKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAbstractKeyword_Unreserved = String;

pub fn new_yul_abstract_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAbstractKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAbstractKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAbstractKeyword = String;

pub fn new_yul_abstract_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAbstractKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAbstractKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAfterKeyword_Reserved = String;

pub fn new_yul_after_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAfterKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAfterKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAfterKeyword_Unreserved = String;

pub fn new_yul_after_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAfterKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAfterKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAfterKeyword = String;

pub fn new_yul_after_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulAfterKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAfterKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAliasKeyword_Reserved = String;

pub fn new_yul_alias_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAliasKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAliasKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAliasKeyword_Unreserved = String;

pub fn new_yul_alias_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAliasKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAliasKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAliasKeyword = String;

pub fn new_yul_alias_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulAliasKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAliasKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAnonymousKeyword_Reserved = String;

pub fn new_yul_anonymous_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAnonymousKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAnonymousKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAnonymousKeyword_Unreserved = String;

pub fn new_yul_anonymous_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAnonymousKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAnonymousKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAnonymousKeyword = String;

pub fn new_yul_anonymous_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAnonymousKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAnonymousKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulApplyKeyword_Reserved = String;

pub fn new_yul_apply_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulApplyKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulApplyKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulApplyKeyword_Unreserved = String;

pub fn new_yul_apply_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulApplyKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulApplyKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulApplyKeyword = String;

pub fn new_yul_apply_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulApplyKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulApplyKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAsKeyword_Reserved = String;

pub fn new_yul_as_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAsKeyword_Unreserved = String;

pub fn new_yul_as_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAsKeyword = String;

pub fn new_yul_as_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulAsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAssemblyKeyword_Reserved = String;

pub fn new_yul_assembly_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAssemblyKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAssemblyKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAssemblyKeyword_Unreserved = String;

pub fn new_yul_assembly_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAssemblyKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAssemblyKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAssemblyKeyword = String;

pub fn new_yul_assembly_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAssemblyKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAssemblyKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAutoKeyword_Reserved = String;

pub fn new_yul_auto_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAutoKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAutoKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAutoKeyword_Unreserved = String;

pub fn new_yul_auto_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulAutoKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAutoKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulAutoKeyword = String;

pub fn new_yul_auto_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulAutoKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulAutoKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulBoolKeyword_Reserved = String;

pub fn new_yul_bool_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBoolKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulBoolKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulBoolKeyword_Unreserved = String;

pub fn new_yul_bool_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBoolKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulBoolKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulBoolKeyword = String;

pub fn new_yul_bool_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulBoolKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulBoolKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulBreakKeyword_Reserved = String;

pub fn new_yul_break_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBreakKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulBreakKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulBreakKeyword_Unreserved = String;

pub fn new_yul_break_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBreakKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulBreakKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulBreakKeyword = String;

pub fn new_yul_break_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulBreakKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulBreakKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulBytesKeyword_Reserved = String;

pub fn new_yul_bytes_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBytesKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulBytesKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulBytesKeyword_Unreserved = String;

pub fn new_yul_bytes_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulBytesKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulBytesKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulBytesKeyword = String;

pub fn new_yul_bytes_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulBytesKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulBytesKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCallDataKeyword_Reserved = String;

pub fn new_yul_call_data_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCallDataKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCallDataKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCallDataKeyword_Unreserved = String;

pub fn new_yul_call_data_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCallDataKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCallDataKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCallDataKeyword = String;

pub fn new_yul_call_data_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCallDataKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCallDataKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCaseKeyword_Reserved = String;

pub fn new_yul_case_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCaseKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCaseKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCaseKeyword_Unreserved = String;

pub fn new_yul_case_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCaseKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCaseKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCaseKeyword = String;

pub fn new_yul_case_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulCaseKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCaseKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCatchKeyword_Reserved = String;

pub fn new_yul_catch_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCatchKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCatchKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCatchKeyword_Unreserved = String;

pub fn new_yul_catch_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCatchKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCatchKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCatchKeyword = String;

pub fn new_yul_catch_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulCatchKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCatchKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulConstantKeyword_Reserved = String;

pub fn new_yul_constant_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstantKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulConstantKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulConstantKeyword_Unreserved = String;

pub fn new_yul_constant_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstantKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulConstantKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulConstantKeyword = String;

pub fn new_yul_constant_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstantKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulConstantKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulConstructorKeyword_Reserved = String;

pub fn new_yul_constructor_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstructorKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulConstructorKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulConstructorKeyword_Unreserved = String;

pub fn new_yul_constructor_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstructorKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulConstructorKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulConstructorKeyword = String;

pub fn new_yul_constructor_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulConstructorKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulConstructorKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulContinueKeyword_Reserved = String;

pub fn new_yul_continue_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulContinueKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulContinueKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulContinueKeyword_Unreserved = String;

pub fn new_yul_continue_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulContinueKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulContinueKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulContinueKeyword = String;

pub fn new_yul_continue_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulContinueKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulContinueKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulContractKeyword_Reserved = String;

pub fn new_yul_contract_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulContractKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulContractKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulContractKeyword_Unreserved = String;

pub fn new_yul_contract_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulContractKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulContractKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulContractKeyword = String;

pub fn new_yul_contract_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulContractKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulContractKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCopyOfKeyword_Reserved = String;

pub fn new_yul_copy_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCopyOfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCopyOfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCopyOfKeyword_Unreserved = String;

pub fn new_yul_copy_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCopyOfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCopyOfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulCopyOfKeyword = String;

pub fn new_yul_copy_of_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulCopyOfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulCopyOfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDaysKeyword_Reserved = String;

pub fn new_yul_days_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDaysKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDaysKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDaysKeyword_Unreserved = String;

pub fn new_yul_days_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDaysKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDaysKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDaysKeyword = String;

pub fn new_yul_days_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulDaysKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDaysKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDefaultKeyword_Reserved = String;

pub fn new_yul_default_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDefaultKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDefaultKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDefaultKeyword_Unreserved = String;

pub fn new_yul_default_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDefaultKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDefaultKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDefaultKeyword = String;

pub fn new_yul_default_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDefaultKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDefaultKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDefineKeyword_Reserved = String;

pub fn new_yul_define_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDefineKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDefineKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDefineKeyword_Unreserved = String;

pub fn new_yul_define_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDefineKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDefineKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDefineKeyword = String;

pub fn new_yul_define_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDefineKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDefineKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDeleteKeyword_Reserved = String;

pub fn new_yul_delete_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDeleteKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDeleteKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDeleteKeyword_Unreserved = String;

pub fn new_yul_delete_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDeleteKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDeleteKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDeleteKeyword = String;

pub fn new_yul_delete_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDeleteKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDeleteKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDoKeyword_Reserved = String;

pub fn new_yul_do_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDoKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDoKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDoKeyword_Unreserved = String;

pub fn new_yul_do_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulDoKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDoKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulDoKeyword = String;

pub fn new_yul_do_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulDoKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulDoKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulElseKeyword_Reserved = String;

pub fn new_yul_else_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulElseKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulElseKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulElseKeyword_Unreserved = String;

pub fn new_yul_else_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulElseKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulElseKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulElseKeyword = String;

pub fn new_yul_else_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulElseKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulElseKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEmitKeyword_Reserved = String;

pub fn new_yul_emit_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEmitKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEmitKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEmitKeyword_Unreserved = String;

pub fn new_yul_emit_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEmitKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEmitKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEmitKeyword = String;

pub fn new_yul_emit_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulEmitKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEmitKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEnumKeyword_Reserved = String;

pub fn new_yul_enum_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEnumKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEnumKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEnumKeyword_Unreserved = String;

pub fn new_yul_enum_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEnumKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEnumKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEnumKeyword = String;

pub fn new_yul_enum_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulEnumKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEnumKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEtherKeyword_Reserved = String;

pub fn new_yul_ether_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEtherKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEtherKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEtherKeyword_Unreserved = String;

pub fn new_yul_ether_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEtherKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEtherKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEtherKeyword = String;

pub fn new_yul_ether_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulEtherKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEtherKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEventKeyword_Reserved = String;

pub fn new_yul_event_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEventKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEventKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEventKeyword_Unreserved = String;

pub fn new_yul_event_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulEventKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEventKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulEventKeyword = String;

pub fn new_yul_event_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulEventKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulEventKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulExternalKeyword_Reserved = String;

pub fn new_yul_external_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulExternalKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulExternalKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulExternalKeyword_Unreserved = String;

pub fn new_yul_external_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulExternalKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulExternalKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulExternalKeyword = String;

pub fn new_yul_external_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulExternalKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulExternalKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFallbackKeyword_Reserved = String;

pub fn new_yul_fallback_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFallbackKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFallbackKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFallbackKeyword_Unreserved = String;

pub fn new_yul_fallback_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFallbackKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFallbackKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFallbackKeyword = String;

pub fn new_yul_fallback_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFallbackKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFallbackKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFalseKeyword_Reserved = String;

pub fn new_yul_false_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFalseKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFalseKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFalseKeyword_Unreserved = String;

pub fn new_yul_false_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFalseKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFalseKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFalseKeyword = String;

pub fn new_yul_false_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulFalseKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFalseKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFinalKeyword_Reserved = String;

pub fn new_yul_final_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFinalKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFinalKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFinalKeyword_Unreserved = String;

pub fn new_yul_final_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFinalKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFinalKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFinalKeyword = String;

pub fn new_yul_final_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulFinalKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFinalKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFinneyKeyword_Reserved = String;

pub fn new_yul_finney_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFinneyKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFinneyKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFinneyKeyword_Unreserved = String;

pub fn new_yul_finney_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFinneyKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFinneyKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFinneyKeyword = String;

pub fn new_yul_finney_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFinneyKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFinneyKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFixedKeyword_Reserved = String;

pub fn new_yul_fixed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFixedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFixedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFixedKeyword_Unreserved = String;

pub fn new_yul_fixed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFixedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFixedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFixedKeyword = String;

pub fn new_yul_fixed_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulFixedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFixedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulForKeyword_Reserved = String;

pub fn new_yul_for_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulForKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulForKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulForKeyword_Unreserved = String;

pub fn new_yul_for_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulForKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulForKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulForKeyword = String;

pub fn new_yul_for_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulForKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulForKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFunctionKeyword_Reserved = String;

pub fn new_yul_function_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFunctionKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFunctionKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFunctionKeyword_Unreserved = String;

pub fn new_yul_function_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFunctionKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFunctionKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulFunctionKeyword = String;

pub fn new_yul_function_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulFunctionKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulFunctionKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulGweiKeyword_Reserved = String;

pub fn new_yul_gwei_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulGweiKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulGweiKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulGweiKeyword_Unreserved = String;

pub fn new_yul_gwei_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulGweiKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulGweiKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulGweiKeyword = String;

pub fn new_yul_gwei_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulGweiKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulGweiKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulHexKeyword_Reserved = String;

pub fn new_yul_hex_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulHexKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulHexKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulHexKeyword_Unreserved = String;

pub fn new_yul_hex_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulHexKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulHexKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulHexKeyword = String;

pub fn new_yul_hex_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulHexKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulHexKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulHoursKeyword_Reserved = String;

pub fn new_yul_hours_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulHoursKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulHoursKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulHoursKeyword_Unreserved = String;

pub fn new_yul_hours_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulHoursKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulHoursKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulHoursKeyword = String;

pub fn new_yul_hours_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulHoursKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulHoursKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIfKeyword_Reserved = String;

pub fn new_yul_if_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIfKeyword_Unreserved = String;

pub fn new_yul_if_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIfKeyword = String;

pub fn new_yul_if_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulIfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulImmutableKeyword_Reserved = String;

pub fn new_yul_immutable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImmutableKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulImmutableKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulImmutableKeyword_Unreserved = String;

pub fn new_yul_immutable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImmutableKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulImmutableKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulImmutableKeyword = String;

pub fn new_yul_immutable_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImmutableKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulImmutableKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulImplementsKeyword_Reserved = String;

pub fn new_yul_implements_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImplementsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulImplementsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulImplementsKeyword_Unreserved = String;

pub fn new_yul_implements_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImplementsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulImplementsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulImplementsKeyword = String;

pub fn new_yul_implements_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImplementsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulImplementsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulImportKeyword_Reserved = String;

pub fn new_yul_import_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImportKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulImportKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulImportKeyword_Unreserved = String;

pub fn new_yul_import_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImportKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulImportKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulImportKeyword = String;

pub fn new_yul_import_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulImportKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulImportKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIndexedKeyword_Reserved = String;

pub fn new_yul_indexed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIndexedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIndexedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIndexedKeyword_Unreserved = String;

pub fn new_yul_indexed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIndexedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIndexedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIndexedKeyword = String;

pub fn new_yul_indexed_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIndexedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIndexedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInKeyword_Reserved = String;

pub fn new_yul_in_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInKeyword_Unreserved = String;

pub fn new_yul_in_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInKeyword = String;

pub fn new_yul_in_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulInKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInlineKeyword_Reserved = String;

pub fn new_yul_inline_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInlineKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInlineKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInlineKeyword_Unreserved = String;

pub fn new_yul_inline_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInlineKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInlineKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInlineKeyword = String;

pub fn new_yul_inline_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInlineKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInlineKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInterfaceKeyword_Reserved = String;

pub fn new_yul_interface_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInterfaceKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInterfaceKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInterfaceKeyword_Unreserved = String;

pub fn new_yul_interface_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInterfaceKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInterfaceKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInterfaceKeyword = String;

pub fn new_yul_interface_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInterfaceKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInterfaceKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInternalKeyword_Reserved = String;

pub fn new_yul_internal_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInternalKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInternalKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInternalKeyword_Unreserved = String;

pub fn new_yul_internal_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInternalKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInternalKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulInternalKeyword = String;

pub fn new_yul_internal_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulInternalKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulInternalKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIntKeyword_Reserved = String;

pub fn new_yul_int_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIntKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIntKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIntKeyword_Unreserved = String;

pub fn new_yul_int_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIntKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIntKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIntKeyword = String;

pub fn new_yul_int_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulIntKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIntKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIsKeyword_Reserved = String;

pub fn new_yul_is_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIsKeyword_Unreserved = String;

pub fn new_yul_is_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulIsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulIsKeyword = String;

pub fn new_yul_is_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulIsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulIsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulLeaveKeyword_Reserved = String;

pub fn new_yul_leave_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLeaveKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulLeaveKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulLeaveKeyword_Unreserved = String;

pub fn new_yul_leave_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLeaveKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulLeaveKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulLeaveKeyword = String;

pub fn new_yul_leave_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulLeaveKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulLeaveKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulLetKeyword_Reserved = String;

pub fn new_yul_let_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLetKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulLetKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulLetKeyword_Unreserved = String;

pub fn new_yul_let_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLetKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulLetKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulLetKeyword = String;

pub fn new_yul_let_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulLetKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulLetKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulLibraryKeyword_Reserved = String;

pub fn new_yul_library_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLibraryKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulLibraryKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulLibraryKeyword_Unreserved = String;

pub fn new_yul_library_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLibraryKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulLibraryKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulLibraryKeyword = String;

pub fn new_yul_library_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulLibraryKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulLibraryKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMacroKeyword_Reserved = String;

pub fn new_yul_macro_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMacroKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMacroKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMacroKeyword_Unreserved = String;

pub fn new_yul_macro_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMacroKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMacroKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMacroKeyword = String;

pub fn new_yul_macro_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulMacroKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMacroKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMappingKeyword_Reserved = String;

pub fn new_yul_mapping_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMappingKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMappingKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMappingKeyword_Unreserved = String;

pub fn new_yul_mapping_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMappingKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMappingKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMappingKeyword = String;

pub fn new_yul_mapping_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMappingKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMappingKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMatchKeyword_Reserved = String;

pub fn new_yul_match_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMatchKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMatchKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMatchKeyword_Unreserved = String;

pub fn new_yul_match_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMatchKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMatchKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMatchKeyword = String;

pub fn new_yul_match_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulMatchKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMatchKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMemoryKeyword_Reserved = String;

pub fn new_yul_memory_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMemoryKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMemoryKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMemoryKeyword_Unreserved = String;

pub fn new_yul_memory_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMemoryKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMemoryKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMemoryKeyword = String;

pub fn new_yul_memory_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMemoryKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMemoryKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMinutesKeyword_Reserved = String;

pub fn new_yul_minutes_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMinutesKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMinutesKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMinutesKeyword_Unreserved = String;

pub fn new_yul_minutes_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMinutesKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMinutesKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMinutesKeyword = String;

pub fn new_yul_minutes_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMinutesKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMinutesKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulModifierKeyword_Reserved = String;

pub fn new_yul_modifier_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulModifierKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulModifierKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulModifierKeyword_Unreserved = String;

pub fn new_yul_modifier_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulModifierKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulModifierKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulModifierKeyword = String;

pub fn new_yul_modifier_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulModifierKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulModifierKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMutableKeyword_Reserved = String;

pub fn new_yul_mutable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMutableKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMutableKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMutableKeyword_Unreserved = String;

pub fn new_yul_mutable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMutableKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMutableKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulMutableKeyword = String;

pub fn new_yul_mutable_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulMutableKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulMutableKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulNewKeyword_Reserved = String;

pub fn new_yul_new_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulNewKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulNewKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulNewKeyword_Unreserved = String;

pub fn new_yul_new_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulNewKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulNewKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulNewKeyword = String;

pub fn new_yul_new_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulNewKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulNewKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulNullKeyword_Reserved = String;

pub fn new_yul_null_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulNullKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulNullKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulNullKeyword_Unreserved = String;

pub fn new_yul_null_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulNullKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulNullKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulNullKeyword = String;

pub fn new_yul_null_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulNullKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulNullKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulOfKeyword_Reserved = String;

pub fn new_yul_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulOfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulOfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulOfKeyword_Unreserved = String;

pub fn new_yul_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulOfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulOfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulOfKeyword = String;

pub fn new_yul_of_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulOfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulOfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulOverrideKeyword_Reserved = String;

pub fn new_yul_override_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulOverrideKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulOverrideKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulOverrideKeyword_Unreserved = String;

pub fn new_yul_override_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulOverrideKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulOverrideKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulOverrideKeyword = String;

pub fn new_yul_override_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulOverrideKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulOverrideKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPartialKeyword_Reserved = String;

pub fn new_yul_partial_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPartialKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPartialKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPartialKeyword_Unreserved = String;

pub fn new_yul_partial_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPartialKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPartialKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPartialKeyword = String;

pub fn new_yul_partial_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPartialKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPartialKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPayableKeyword_Reserved = String;

pub fn new_yul_payable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPayableKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPayableKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPayableKeyword_Unreserved = String;

pub fn new_yul_payable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPayableKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPayableKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPayableKeyword = String;

pub fn new_yul_payable_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPayableKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPayableKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPragmaKeyword_Reserved = String;

pub fn new_yul_pragma_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPragmaKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPragmaKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPragmaKeyword_Unreserved = String;

pub fn new_yul_pragma_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPragmaKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPragmaKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPragmaKeyword = String;

pub fn new_yul_pragma_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPragmaKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPragmaKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPrivateKeyword_Reserved = String;

pub fn new_yul_private_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPrivateKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPrivateKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPrivateKeyword_Unreserved = String;

pub fn new_yul_private_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPrivateKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPrivateKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPrivateKeyword = String;

pub fn new_yul_private_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPrivateKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPrivateKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPromiseKeyword_Reserved = String;

pub fn new_yul_promise_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPromiseKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPromiseKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPromiseKeyword_Unreserved = String;

pub fn new_yul_promise_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPromiseKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPromiseKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPromiseKeyword = String;

pub fn new_yul_promise_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPromiseKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPromiseKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPublicKeyword_Reserved = String;

pub fn new_yul_public_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPublicKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPublicKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPublicKeyword_Unreserved = String;

pub fn new_yul_public_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPublicKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPublicKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPublicKeyword = String;

pub fn new_yul_public_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPublicKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPublicKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPureKeyword_Reserved = String;

pub fn new_yul_pure_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPureKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPureKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPureKeyword_Unreserved = String;

pub fn new_yul_pure_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulPureKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPureKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulPureKeyword = String;

pub fn new_yul_pure_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulPureKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulPureKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulReceiveKeyword_Reserved = String;

pub fn new_yul_receive_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReceiveKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulReceiveKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulReceiveKeyword_Unreserved = String;

pub fn new_yul_receive_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReceiveKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulReceiveKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulReceiveKeyword = String;

pub fn new_yul_receive_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReceiveKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulReceiveKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulReferenceKeyword_Reserved = String;

pub fn new_yul_reference_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReferenceKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulReferenceKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulReferenceKeyword_Unreserved = String;

pub fn new_yul_reference_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReferenceKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulReferenceKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulReferenceKeyword = String;

pub fn new_yul_reference_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReferenceKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulReferenceKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulRelocatableKeyword_Reserved = String;

pub fn new_yul_relocatable_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulRelocatableKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulRelocatableKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulRelocatableKeyword_Unreserved = String;

pub fn new_yul_relocatable_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulRelocatableKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulRelocatableKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulRelocatableKeyword = String;

pub fn new_yul_relocatable_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulRelocatableKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulRelocatableKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulReturnsKeyword_Reserved = String;

pub fn new_yul_returns_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReturnsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulReturnsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulReturnsKeyword_Unreserved = String;

pub fn new_yul_returns_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReturnsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulReturnsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulReturnsKeyword = String;

pub fn new_yul_returns_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulReturnsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulReturnsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSealedKeyword_Reserved = String;

pub fn new_yul_sealed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSealedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSealedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSealedKeyword_Unreserved = String;

pub fn new_yul_sealed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSealedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSealedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSealedKeyword = String;

pub fn new_yul_sealed_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSealedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSealedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSecondsKeyword_Reserved = String;

pub fn new_yul_seconds_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSecondsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSecondsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSecondsKeyword_Unreserved = String;

pub fn new_yul_seconds_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSecondsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSecondsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSecondsKeyword = String;

pub fn new_yul_seconds_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSecondsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSecondsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSizeOfKeyword_Reserved = String;

pub fn new_yul_size_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSizeOfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSizeOfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSizeOfKeyword_Unreserved = String;

pub fn new_yul_size_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSizeOfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSizeOfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSizeOfKeyword = String;

pub fn new_yul_size_of_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSizeOfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSizeOfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStaticKeyword_Reserved = String;

pub fn new_yul_static_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStaticKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStaticKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStaticKeyword_Unreserved = String;

pub fn new_yul_static_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStaticKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStaticKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStaticKeyword = String;

pub fn new_yul_static_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStaticKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStaticKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStorageKeyword_Reserved = String;

pub fn new_yul_storage_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStorageKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStorageKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStorageKeyword_Unreserved = String;

pub fn new_yul_storage_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStorageKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStorageKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStorageKeyword = String;

pub fn new_yul_storage_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStorageKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStorageKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStringKeyword_Reserved = String;

pub fn new_yul_string_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStringKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStringKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStringKeyword_Unreserved = String;

pub fn new_yul_string_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStringKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStringKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStringKeyword = String;

pub fn new_yul_string_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStringKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStringKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStructKeyword_Reserved = String;

pub fn new_yul_struct_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStructKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStructKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStructKeyword_Unreserved = String;

pub fn new_yul_struct_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStructKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStructKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulStructKeyword = String;

pub fn new_yul_struct_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulStructKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulStructKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSuperKeyword_Reserved = String;

pub fn new_yul_super_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSuperKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSuperKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSuperKeyword_Unreserved = String;

pub fn new_yul_super_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSuperKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSuperKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSuperKeyword = String;

pub fn new_yul_super_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulSuperKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSuperKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSupportsKeyword_Reserved = String;

pub fn new_yul_supports_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSupportsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSupportsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSupportsKeyword_Unreserved = String;

pub fn new_yul_supports_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSupportsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSupportsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSupportsKeyword = String;

pub fn new_yul_supports_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSupportsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSupportsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSwitchKeyword_Reserved = String;

pub fn new_yul_switch_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSwitchKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSwitchKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSwitchKeyword_Unreserved = String;

pub fn new_yul_switch_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSwitchKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSwitchKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSwitchKeyword = String;

pub fn new_yul_switch_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSwitchKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSwitchKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSzaboKeyword_Reserved = String;

pub fn new_yul_szabo_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSzaboKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSzaboKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSzaboKeyword_Unreserved = String;

pub fn new_yul_szabo_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulSzaboKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSzaboKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulSzaboKeyword = String;

pub fn new_yul_szabo_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulSzaboKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulSzaboKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulThisKeyword_Reserved = String;

pub fn new_yul_this_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulThisKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulThisKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulThisKeyword_Unreserved = String;

pub fn new_yul_this_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulThisKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulThisKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulThisKeyword = String;

pub fn new_yul_this_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulThisKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulThisKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulThrowKeyword_Reserved = String;

pub fn new_yul_throw_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulThrowKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulThrowKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulThrowKeyword_Unreserved = String;

pub fn new_yul_throw_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulThrowKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulThrowKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulThrowKeyword = String;

pub fn new_yul_throw_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulThrowKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulThrowKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTrueKeyword_Reserved = String;

pub fn new_yul_true_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTrueKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTrueKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTrueKeyword_Unreserved = String;

pub fn new_yul_true_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTrueKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTrueKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTrueKeyword = String;

pub fn new_yul_true_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulTrueKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTrueKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTryKeyword_Reserved = String;

pub fn new_yul_try_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTryKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTryKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTryKeyword_Unreserved = String;

pub fn new_yul_try_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTryKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTryKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTryKeyword = String;

pub fn new_yul_try_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulTryKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTryKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTypeDefKeyword_Reserved = String;

pub fn new_yul_type_def_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeDefKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTypeDefKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTypeDefKeyword_Unreserved = String;

pub fn new_yul_type_def_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeDefKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTypeDefKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTypeDefKeyword = String;

pub fn new_yul_type_def_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeDefKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTypeDefKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTypeKeyword_Reserved = String;

pub fn new_yul_type_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTypeKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTypeKeyword_Unreserved = String;

pub fn new_yul_type_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTypeKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTypeKeyword = String;

pub fn new_yul_type_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulTypeKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTypeKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTypeOfKeyword_Reserved = String;

pub fn new_yul_type_of_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeOfKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTypeOfKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTypeOfKeyword_Unreserved = String;

pub fn new_yul_type_of_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeOfKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTypeOfKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulTypeOfKeyword = String;

pub fn new_yul_type_of_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulTypeOfKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulTypeOfKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUfixedKeyword_Reserved = String;

pub fn new_yul_ufixed_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUfixedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUfixedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUfixedKeyword_Unreserved = String;

pub fn new_yul_ufixed_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUfixedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUfixedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUfixedKeyword = String;

pub fn new_yul_ufixed_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUfixedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUfixedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUintKeyword_Reserved = String;

pub fn new_yul_uint_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUintKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUintKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUintKeyword_Unreserved = String;

pub fn new_yul_uint_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUintKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUintKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUintKeyword = String;

pub fn new_yul_uint_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulUintKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUintKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUncheckedKeyword_Reserved = String;

pub fn new_yul_unchecked_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUncheckedKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUncheckedKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUncheckedKeyword_Unreserved = String;

pub fn new_yul_unchecked_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUncheckedKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUncheckedKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUncheckedKeyword = String;

pub fn new_yul_unchecked_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUncheckedKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUncheckedKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUsingKeyword_Reserved = String;

pub fn new_yul_using_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUsingKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUsingKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUsingKeyword_Unreserved = String;

pub fn new_yul_using_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulUsingKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUsingKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulUsingKeyword = String;

pub fn new_yul_using_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulUsingKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulUsingKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulVarKeyword_Reserved = String;

pub fn new_yul_var_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulVarKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulVarKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulVarKeyword_Unreserved = String;

pub fn new_yul_var_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulVarKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulVarKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulVarKeyword = String;

pub fn new_yul_var_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulVarKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulVarKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulViewKeyword_Reserved = String;

pub fn new_yul_view_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulViewKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulViewKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulViewKeyword_Unreserved = String;

pub fn new_yul_view_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulViewKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulViewKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulViewKeyword = String;

pub fn new_yul_view_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulViewKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulViewKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulVirtualKeyword_Reserved = String;

pub fn new_yul_virtual_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulVirtualKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulVirtualKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulVirtualKeyword_Unreserved = String;

pub fn new_yul_virtual_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulVirtualKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulVirtualKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulVirtualKeyword = String;

pub fn new_yul_virtual_keyword<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulVirtualKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulVirtualKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulWeeksKeyword_Reserved = String;

pub fn new_yul_weeks_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWeeksKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulWeeksKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulWeeksKeyword_Unreserved = String;

pub fn new_yul_weeks_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWeeksKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulWeeksKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulWeeksKeyword = String;

pub fn new_yul_weeks_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulWeeksKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulWeeksKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulWeiKeyword_Reserved = String;

pub fn new_yul_wei_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWeiKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulWeiKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulWeiKeyword_Unreserved = String;

pub fn new_yul_wei_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWeiKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulWeiKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulWeiKeyword = String;

pub fn new_yul_wei_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulWeiKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulWeiKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulWhileKeyword_Reserved = String;

pub fn new_yul_while_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWhileKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulWhileKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulWhileKeyword_Unreserved = String;

pub fn new_yul_while_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulWhileKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulWhileKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulWhileKeyword = String;

pub fn new_yul_while_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulWhileKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulWhileKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulYearsKeyword_Reserved = String;

pub fn new_yul_years_keyword_reserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulYearsKeyword_Reserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulYearsKeyword_Reserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulYearsKeyword_Unreserved = String;

pub fn new_yul_years_keyword_unreserved<'source>(
    l: usize,
    r: usize,
    source: &'source str,
) -> YulYearsKeyword_Unreserved {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulYearsKeyword_Unreserved\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}

pub type YulYearsKeyword = String;

pub fn new_yul_years_keyword<'source>(l: usize, r: usize, source: &'source str) -> YulYearsKeyword {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"YulYearsKeyword\", \"_l\": {}, \"_r\": {}, \"value\": \"{}\"  }} ", l, r, &source[l..r])
}
