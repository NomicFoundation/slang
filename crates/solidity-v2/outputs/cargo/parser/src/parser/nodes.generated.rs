// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

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

pub fn new_version_specifier(element: String) -> VersionSpecifier {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"VersionSpecifier\", \"value\": {}, }} ",
        element
    )
}

pub type SingleQuotedVersionLiteral = String;

pub fn new_single_quoted_version_literal(element: String) -> SingleQuotedVersionLiteral {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SingleQuotedVersionLiteral\", \"value\": {}, }} ",
        element
    )
}

pub type DoubleQuotedVersionLiteral = String;

pub fn new_double_quoted_version_literal(element: String) -> DoubleQuotedVersionLiteral {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"DoubleQuotedVersionLiteral\", \"value\": {}, }} ",
        element
    )
}

pub type Whitespace = String;

pub fn new_whitespace(element: String) -> Whitespace {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Whitespace\", \"value\": {}, }} ",
        element
    )
}

pub type EndOfLine = String;

pub fn new_end_of_line(element: String) -> EndOfLine {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"EndOfLine\", \"value\": {}, }} ",
        element
    )
}

pub type SingleLineComment = String;

pub fn new_single_line_comment(element: String) -> SingleLineComment {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SingleLineComment\", \"value\": {}, }} ",
        element
    )
}

pub type MultiLineComment = String;

pub fn new_multi_line_comment(element: String) -> MultiLineComment {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MultiLineComment\", \"value\": {}, }} ",
        element
    )
}

pub type SingleLineNatSpecComment = String;

pub fn new_single_line_nat_spec_comment(element: String) -> SingleLineNatSpecComment {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SingleLineNatSpecComment\", \"value\": {}, }} ",
        element
    )
}

pub type MultiLineNatSpecComment = String;

pub fn new_multi_line_nat_spec_comment(element: String) -> MultiLineNatSpecComment {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MultiLineNatSpecComment\", \"value\": {}, }} ",
        element
    )
}

pub type AbicoderKeyword = String;

pub fn new_abicoder_keyword(element: String) -> AbicoderKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AbicoderKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type AbicoderV1Keyword = String;

pub fn new_abicoder_v1_keyword(element: String) -> AbicoderV1Keyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AbicoderV1Keyword\", \"value\": {}, }} ",
        element
    )
}

pub type AbicoderV2Keyword = String;

pub fn new_abicoder_v2_keyword(element: String) -> AbicoderV2Keyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AbicoderV2Keyword\", \"value\": {}, }} ",
        element
    )
}

pub type ABIEncoderV2Keyword = String;

pub fn new_abi_encoder_v2_keyword(element: String) -> ABIEncoderV2Keyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ABIEncoderV2Keyword\", \"value\": {}, }} ",
        element
    )
}

pub type AbstractKeyword = String;

pub fn new_abstract_keyword(element: String) -> AbstractKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AbstractKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type AddressKeyword = String;

pub fn new_address_keyword(element: String) -> AddressKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AddressKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type AfterKeyword = String;

pub fn new_after_keyword(element: String) -> AfterKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AfterKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type AliasKeyword = String;

pub fn new_alias_keyword(element: String) -> AliasKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AliasKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type AnonymousKeyword = String;

pub fn new_anonymous_keyword(element: String) -> AnonymousKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AnonymousKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ApplyKeyword = String;

pub fn new_apply_keyword(element: String) -> ApplyKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ApplyKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type AsKeyword = String;

pub fn new_as_keyword(element: String) -> AsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type AssemblyKeyword = String;

pub fn new_assembly_keyword(element: String) -> AssemblyKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AssemblyKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type AtKeyword = String;

pub fn new_at_keyword(element: String) -> AtKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AtKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type AutoKeyword = String;

pub fn new_auto_keyword(element: String) -> AutoKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AutoKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type BoolKeyword = String;

pub fn new_bool_keyword(element: String) -> BoolKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"BoolKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type BreakKeyword = String;

pub fn new_break_keyword(element: String) -> BreakKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"BreakKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ByteKeyword = String;

pub fn new_byte_keyword(element: String) -> ByteKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ByteKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type BytesKeyword = String;

pub fn new_bytes_keyword(element: String) -> BytesKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"BytesKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type CallDataKeyword = String;

pub fn new_call_data_keyword(element: String) -> CallDataKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"CallDataKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type CaseKeyword = String;

pub fn new_case_keyword(element: String) -> CaseKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"CaseKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type CatchKeyword = String;

pub fn new_catch_keyword(element: String) -> CatchKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"CatchKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ConstantKeyword = String;

pub fn new_constant_keyword(element: String) -> ConstantKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ConstantKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ConstructorKeyword = String;

pub fn new_constructor_keyword(element: String) -> ConstructorKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ConstructorKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ContinueKeyword = String;

pub fn new_continue_keyword(element: String) -> ContinueKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ContinueKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ContractKeyword = String;

pub fn new_contract_keyword(element: String) -> ContractKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ContractKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type CopyOfKeyword = String;

pub fn new_copy_of_keyword(element: String) -> CopyOfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"CopyOfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type DaysKeyword = String;

pub fn new_days_keyword(element: String) -> DaysKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"DaysKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type DefaultKeyword = String;

pub fn new_default_keyword(element: String) -> DefaultKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"DefaultKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type DefineKeyword = String;

pub fn new_define_keyword(element: String) -> DefineKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"DefineKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type DeleteKeyword = String;

pub fn new_delete_keyword(element: String) -> DeleteKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"DeleteKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type DoKeyword = String;

pub fn new_do_keyword(element: String) -> DoKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"DoKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ElseKeyword = String;

pub fn new_else_keyword(element: String) -> ElseKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ElseKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type EmitKeyword = String;

pub fn new_emit_keyword(element: String) -> EmitKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"EmitKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type EnumKeyword = String;

pub fn new_enum_keyword(element: String) -> EnumKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"EnumKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ErrorKeyword = String;

pub fn new_error_keyword(element: String) -> ErrorKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ErrorKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type EtherKeyword = String;

pub fn new_ether_keyword(element: String) -> EtherKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"EtherKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type EventKeyword = String;

pub fn new_event_keyword(element: String) -> EventKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"EventKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ExperimentalKeyword = String;

pub fn new_experimental_keyword(element: String) -> ExperimentalKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ExperimentalKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ExternalKeyword = String;

pub fn new_external_keyword(element: String) -> ExternalKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ExternalKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type FallbackKeyword = String;

pub fn new_fallback_keyword(element: String) -> FallbackKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"FallbackKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type FalseKeyword = String;

pub fn new_false_keyword(element: String) -> FalseKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"FalseKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type FinalKeyword = String;

pub fn new_final_keyword(element: String) -> FinalKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"FinalKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type FinneyKeyword = String;

pub fn new_finney_keyword(element: String) -> FinneyKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"FinneyKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type FixedKeyword = String;

pub fn new_fixed_keyword(element: String) -> FixedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"FixedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ForKeyword = String;

pub fn new_for_keyword(element: String) -> ForKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ForKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type FromKeyword = String;

pub fn new_from_keyword(element: String) -> FromKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"FromKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type FunctionKeyword = String;

pub fn new_function_keyword(element: String) -> FunctionKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"FunctionKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type GlobalKeyword = String;

pub fn new_global_keyword(element: String) -> GlobalKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"GlobalKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type GweiKeyword = String;

pub fn new_gwei_keyword(element: String) -> GweiKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"GweiKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type HexKeyword = String;

pub fn new_hex_keyword(element: String) -> HexKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"HexKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type HoursKeyword = String;

pub fn new_hours_keyword(element: String) -> HoursKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"HoursKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type IfKeyword = String;

pub fn new_if_keyword(element: String) -> IfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"IfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ImmutableKeyword = String;

pub fn new_immutable_keyword(element: String) -> ImmutableKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ImmutableKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ImplementsKeyword = String;

pub fn new_implements_keyword(element: String) -> ImplementsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ImplementsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ImportKeyword = String;

pub fn new_import_keyword(element: String) -> ImportKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ImportKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type IndexedKeyword = String;

pub fn new_indexed_keyword(element: String) -> IndexedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"IndexedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type InKeyword = String;

pub fn new_in_keyword(element: String) -> InKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"InKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type InlineKeyword = String;

pub fn new_inline_keyword(element: String) -> InlineKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"InlineKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type InterfaceKeyword = String;

pub fn new_interface_keyword(element: String) -> InterfaceKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"InterfaceKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type InternalKeyword = String;

pub fn new_internal_keyword(element: String) -> InternalKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"InternalKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type IntKeyword = String;

pub fn new_int_keyword(element: String) -> IntKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"IntKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type IsKeyword = String;

pub fn new_is_keyword(element: String) -> IsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"IsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type LayoutKeyword = String;

pub fn new_layout_keyword(element: String) -> LayoutKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"LayoutKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type LetKeyword = String;

pub fn new_let_keyword(element: String) -> LetKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"LetKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type LibraryKeyword = String;

pub fn new_library_keyword(element: String) -> LibraryKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"LibraryKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type MacroKeyword = String;

pub fn new_macro_keyword(element: String) -> MacroKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MacroKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type MappingKeyword = String;

pub fn new_mapping_keyword(element: String) -> MappingKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MappingKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type MatchKeyword = String;

pub fn new_match_keyword(element: String) -> MatchKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MatchKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type MemoryKeyword = String;

pub fn new_memory_keyword(element: String) -> MemoryKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MemoryKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type MinutesKeyword = String;

pub fn new_minutes_keyword(element: String) -> MinutesKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MinutesKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ModifierKeyword = String;

pub fn new_modifier_keyword(element: String) -> ModifierKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ModifierKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type MutableKeyword = String;

pub fn new_mutable_keyword(element: String) -> MutableKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MutableKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type NewKeyword = String;

pub fn new_new_keyword(element: String) -> NewKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"NewKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type NullKeyword = String;

pub fn new_null_keyword(element: String) -> NullKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"NullKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type OfKeyword = String;

pub fn new_of_keyword(element: String) -> OfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"OfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type OverrideKeyword = String;

pub fn new_override_keyword(element: String) -> OverrideKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"OverrideKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type PartialKeyword = String;

pub fn new_partial_keyword(element: String) -> PartialKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PartialKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type PayableKeyword = String;

pub fn new_payable_keyword(element: String) -> PayableKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PayableKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type PragmaKeyword = String;

pub fn new_pragma_keyword(element: String) -> PragmaKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PragmaKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type PrivateKeyword = String;

pub fn new_private_keyword(element: String) -> PrivateKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PrivateKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type PromiseKeyword = String;

pub fn new_promise_keyword(element: String) -> PromiseKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PromiseKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type PublicKeyword = String;

pub fn new_public_keyword(element: String) -> PublicKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PublicKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type PureKeyword = String;

pub fn new_pure_keyword(element: String) -> PureKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PureKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ReceiveKeyword = String;

pub fn new_receive_keyword(element: String) -> ReceiveKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ReceiveKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ReferenceKeyword = String;

pub fn new_reference_keyword(element: String) -> ReferenceKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ReferenceKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type RelocatableKeyword = String;

pub fn new_relocatable_keyword(element: String) -> RelocatableKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"RelocatableKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ReturnKeyword = String;

pub fn new_return_keyword(element: String) -> ReturnKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ReturnKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ReturnsKeyword = String;

pub fn new_returns_keyword(element: String) -> ReturnsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ReturnsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type RevertKeyword = String;

pub fn new_revert_keyword(element: String) -> RevertKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"RevertKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type SealedKeyword = String;

pub fn new_sealed_keyword(element: String) -> SealedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SealedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type SecondsKeyword = String;

pub fn new_seconds_keyword(element: String) -> SecondsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SecondsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type SizeOfKeyword = String;

pub fn new_size_of_keyword(element: String) -> SizeOfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SizeOfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type SMTCheckerKeyword = String;

pub fn new_smt_checker_keyword(element: String) -> SMTCheckerKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SMTCheckerKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type SolidityKeyword = String;

pub fn new_solidity_keyword(element: String) -> SolidityKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SolidityKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type StaticKeyword = String;

pub fn new_static_keyword(element: String) -> StaticKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"StaticKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type StorageKeyword = String;

pub fn new_storage_keyword(element: String) -> StorageKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"StorageKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type StringKeyword = String;

pub fn new_string_keyword(element: String) -> StringKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"StringKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type StructKeyword = String;

pub fn new_struct_keyword(element: String) -> StructKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"StructKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type SuperKeyword = String;

pub fn new_super_keyword(element: String) -> SuperKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SuperKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type SupportsKeyword = String;

pub fn new_supports_keyword(element: String) -> SupportsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SupportsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type SwitchKeyword = String;

pub fn new_switch_keyword(element: String) -> SwitchKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SwitchKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type SzaboKeyword = String;

pub fn new_szabo_keyword(element: String) -> SzaboKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SzaboKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ThisKeyword = String;

pub fn new_this_keyword(element: String) -> ThisKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ThisKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ThrowKeyword = String;

pub fn new_throw_keyword(element: String) -> ThrowKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ThrowKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type TransientKeyword = String;

pub fn new_transient_keyword(element: String) -> TransientKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"TransientKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type TrueKeyword = String;

pub fn new_true_keyword(element: String) -> TrueKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"TrueKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type TryKeyword = String;

pub fn new_try_keyword(element: String) -> TryKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"TryKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type TypeDefKeyword = String;

pub fn new_type_def_keyword(element: String) -> TypeDefKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"TypeDefKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type TypeKeyword = String;

pub fn new_type_keyword(element: String) -> TypeKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"TypeKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type TypeOfKeyword = String;

pub fn new_type_of_keyword(element: String) -> TypeOfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"TypeOfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type UfixedKeyword = String;

pub fn new_ufixed_keyword(element: String) -> UfixedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"UfixedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type UintKeyword = String;

pub fn new_uint_keyword(element: String) -> UintKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"UintKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type UncheckedKeyword = String;

pub fn new_unchecked_keyword(element: String) -> UncheckedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"UncheckedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type UsingKeyword = String;

pub fn new_using_keyword(element: String) -> UsingKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"UsingKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type VarKeyword = String;

pub fn new_var_keyword(element: String) -> VarKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"VarKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type ViewKeyword = String;

pub fn new_view_keyword(element: String) -> ViewKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ViewKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type VirtualKeyword = String;

pub fn new_virtual_keyword(element: String) -> VirtualKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"VirtualKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type WeeksKeyword = String;

pub fn new_weeks_keyword(element: String) -> WeeksKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"WeeksKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type WeiKeyword = String;

pub fn new_wei_keyword(element: String) -> WeiKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"WeiKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type WhileKeyword = String;

pub fn new_while_keyword(element: String) -> WhileKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"WhileKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YearsKeyword = String;

pub fn new_years_keyword(element: String) -> YearsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YearsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type OpenParen = String;

pub fn new_open_paren(element: String) -> OpenParen {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"OpenParen\", \"value\": {}, }} ",
        element
    )
}

pub type CloseParen = String;

pub fn new_close_paren(element: String) -> CloseParen {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"CloseParen\", \"value\": {}, }} ",
        element
    )
}

pub type OpenBracket = String;

pub fn new_open_bracket(element: String) -> OpenBracket {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"OpenBracket\", \"value\": {}, }} ",
        element
    )
}

pub type CloseBracket = String;

pub fn new_close_bracket(element: String) -> CloseBracket {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"CloseBracket\", \"value\": {}, }} ",
        element
    )
}

pub type OpenBrace = String;

pub fn new_open_brace(element: String) -> OpenBrace {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"OpenBrace\", \"value\": {}, }} ",
        element
    )
}

pub type CloseBrace = String;

pub fn new_close_brace(element: String) -> CloseBrace {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"CloseBrace\", \"value\": {}, }} ",
        element
    )
}

pub type Comma = String;

pub fn new_comma(element: String) -> Comma {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Comma\", \"value\": {}, }} ",
        element
    )
}

pub type Period = String;

pub fn new_period(element: String) -> Period {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Period\", \"value\": {}, }} ",
        element
    )
}

pub type QuestionMark = String;

pub fn new_question_mark(element: String) -> QuestionMark {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"QuestionMark\", \"value\": {}, }} ",
        element
    )
}

pub type Semicolon = String;

pub fn new_semicolon(element: String) -> Semicolon {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Semicolon\", \"value\": {}, }} ",
        element
    )
}

pub type Colon = String;

pub fn new_colon(element: String) -> Colon {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Colon\", \"value\": {}, }} ",
        element
    )
}

pub type ColonEqual = String;

pub fn new_colon_equal(element: String) -> ColonEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"ColonEqual\", \"value\": {}, }} ",
        element
    )
}

pub type Equal = String;

pub fn new_equal(element: String) -> Equal {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Equal\", \"value\": {}, }} ",
        element
    )
}

pub type EqualColon = String;

pub fn new_equal_colon(element: String) -> EqualColon {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"EqualColon\", \"value\": {}, }} ",
        element
    )
}

pub type EqualEqual = String;

pub fn new_equal_equal(element: String) -> EqualEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"EqualEqual\", \"value\": {}, }} ",
        element
    )
}

pub type EqualGreaterThan = String;

pub fn new_equal_greater_than(element: String) -> EqualGreaterThan {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"EqualGreaterThan\", \"value\": {}, }} ",
        element
    )
}

pub type Asterisk = String;

pub fn new_asterisk(element: String) -> Asterisk {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Asterisk\", \"value\": {}, }} ",
        element
    )
}

pub type AsteriskEqual = String;

pub fn new_asterisk_equal(element: String) -> AsteriskEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AsteriskEqual\", \"value\": {}, }} ",
        element
    )
}

pub type AsteriskAsterisk = String;

pub fn new_asterisk_asterisk(element: String) -> AsteriskAsterisk {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AsteriskAsterisk\", \"value\": {}, }} ",
        element
    )
}

pub type Bar = String;

pub fn new_bar(element: String) -> Bar {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Bar\", \"value\": {}, }} ",
        element
    )
}

pub type BarEqual = String;

pub fn new_bar_equal(element: String) -> BarEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"BarEqual\", \"value\": {}, }} ",
        element
    )
}

pub type BarBar = String;

pub fn new_bar_bar(element: String) -> BarBar {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"BarBar\", \"value\": {}, }} ",
        element
    )
}

pub type Ampersand = String;

pub fn new_ampersand(element: String) -> Ampersand {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Ampersand\", \"value\": {}, }} ",
        element
    )
}

pub type AmpersandEqual = String;

pub fn new_ampersand_equal(element: String) -> AmpersandEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AmpersandEqual\", \"value\": {}, }} ",
        element
    )
}

pub type AmpersandAmpersand = String;

pub fn new_ampersand_ampersand(element: String) -> AmpersandAmpersand {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"AmpersandAmpersand\", \"value\": {}, }} ",
        element
    )
}

pub type LessThan = String;

pub fn new_less_than(element: String) -> LessThan {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"LessThan\", \"value\": {}, }} ",
        element
    )
}

pub type LessThanEqual = String;

pub fn new_less_than_equal(element: String) -> LessThanEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"LessThanEqual\", \"value\": {}, }} ",
        element
    )
}

pub type LessThanLessThan = String;

pub fn new_less_than_less_than(element: String) -> LessThanLessThan {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"LessThanLessThan\", \"value\": {}, }} ",
        element
    )
}

pub type LessThanLessThanEqual = String;

pub fn new_less_than_less_than_equal(element: String) -> LessThanLessThanEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"LessThanLessThanEqual\", \"value\": {}, }} ",
        element
    )
}

pub type GreaterThan = String;

pub fn new_greater_than(element: String) -> GreaterThan {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"GreaterThan\", \"value\": {}, }} ",
        element
    )
}

pub type GreaterThanEqual = String;

pub fn new_greater_than_equal(element: String) -> GreaterThanEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"GreaterThanEqual\", \"value\": {}, }} ",
        element
    )
}

pub type GreaterThanGreaterThan = String;

pub fn new_greater_than_greater_than(element: String) -> GreaterThanGreaterThan {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"GreaterThanGreaterThan\", \"value\": {}, }} ",
        element
    )
}

pub type GreaterThanGreaterThanEqual = String;

pub fn new_greater_than_greater_than_equal(element: String) -> GreaterThanGreaterThanEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GreaterThanGreaterThanEqual\", \"value\": {}, }} ", element)
}

pub type GreaterThanGreaterThanGreaterThan = String;

pub fn new_greater_than_greater_than_greater_than(
    element: String,
) -> GreaterThanGreaterThanGreaterThan {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GreaterThanGreaterThanGreaterThan\", \"value\": {}, }} ", element)
}

pub type GreaterThanGreaterThanGreaterThanEqual = String;

pub fn new_greater_than_greater_than_greater_than_equal(
    element: String,
) -> GreaterThanGreaterThanGreaterThanEqual {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"GreaterThanGreaterThanGreaterThanEqual\", \"value\": {}, }} ", element)
}

pub type Plus = String;

pub fn new_plus(element: String) -> Plus {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Plus\", \"value\": {}, }} ",
        element
    )
}

pub type PlusEqual = String;

pub fn new_plus_equal(element: String) -> PlusEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PlusEqual\", \"value\": {}, }} ",
        element
    )
}

pub type PlusPlus = String;

pub fn new_plus_plus(element: String) -> PlusPlus {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PlusPlus\", \"value\": {}, }} ",
        element
    )
}

pub type Minus = String;

pub fn new_minus(element: String) -> Minus {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Minus\", \"value\": {}, }} ",
        element
    )
}

pub type MinusEqual = String;

pub fn new_minus_equal(element: String) -> MinusEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MinusEqual\", \"value\": {}, }} ",
        element
    )
}

pub type MinusMinus = String;

pub fn new_minus_minus(element: String) -> MinusMinus {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MinusMinus\", \"value\": {}, }} ",
        element
    )
}

pub type MinusGreaterThan = String;

pub fn new_minus_greater_than(element: String) -> MinusGreaterThan {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"MinusGreaterThan\", \"value\": {}, }} ",
        element
    )
}

pub type Slash = String;

pub fn new_slash(element: String) -> Slash {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Slash\", \"value\": {}, }} ",
        element
    )
}

pub type SlashEqual = String;

pub fn new_slash_equal(element: String) -> SlashEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SlashEqual\", \"value\": {}, }} ",
        element
    )
}

pub type Percent = String;

pub fn new_percent(element: String) -> Percent {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Percent\", \"value\": {}, }} ",
        element
    )
}

pub type PercentEqual = String;

pub fn new_percent_equal(element: String) -> PercentEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"PercentEqual\", \"value\": {}, }} ",
        element
    )
}

pub type Bang = String;

pub fn new_bang(element: String) -> Bang {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Bang\", \"value\": {}, }} ",
        element
    )
}

pub type BangEqual = String;

pub fn new_bang_equal(element: String) -> BangEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"BangEqual\", \"value\": {}, }} ",
        element
    )
}

pub type Caret = String;

pub fn new_caret(element: String) -> Caret {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Caret\", \"value\": {}, }} ",
        element
    )
}

pub type CaretEqual = String;

pub fn new_caret_equal(element: String) -> CaretEqual {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"CaretEqual\", \"value\": {}, }} ",
        element
    )
}

pub type Tilde = String;

pub fn new_tilde(element: String) -> Tilde {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Tilde\", \"value\": {}, }} ",
        element
    )
}

pub type HexLiteral = String;

pub fn new_hex_literal(element: String) -> HexLiteral {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"HexLiteral\", \"value\": {}, }} ",
        element
    )
}

pub type DecimalLiteral = String;

pub fn new_decimal_literal(element: String) -> DecimalLiteral {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"DecimalLiteral\", \"value\": {}, }} ",
        element
    )
}

pub type SingleQuotedStringLiteral = String;

pub fn new_single_quoted_string_literal(element: String) -> SingleQuotedStringLiteral {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"SingleQuotedStringLiteral\", \"value\": {}, }} ",
        element
    )
}

pub type DoubleQuotedStringLiteral = String;

pub fn new_double_quoted_string_literal(element: String) -> DoubleQuotedStringLiteral {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"DoubleQuotedStringLiteral\", \"value\": {}, }} ",
        element
    )
}

pub type SingleQuotedHexStringLiteral = String;

pub fn new_single_quoted_hex_string_literal(element: String) -> SingleQuotedHexStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SingleQuotedHexStringLiteral\", \"value\": {}, }} ", element)
}

pub type DoubleQuotedHexStringLiteral = String;

pub fn new_double_quoted_hex_string_literal(element: String) -> DoubleQuotedHexStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DoubleQuotedHexStringLiteral\", \"value\": {}, }} ", element)
}

pub type SingleQuotedUnicodeStringLiteral = String;

pub fn new_single_quoted_unicode_string_literal(
    element: String,
) -> SingleQuotedUnicodeStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"SingleQuotedUnicodeStringLiteral\", \"value\": {}, }} ", element)
}

pub type DoubleQuotedUnicodeStringLiteral = String;

pub fn new_double_quoted_unicode_string_literal(
    element: String,
) -> DoubleQuotedUnicodeStringLiteral {
    format!(" {{  \"item\": \"Terminal\", \"name\": \"DoubleQuotedUnicodeStringLiteral\", \"value\": {}, }} ", element)
}

pub type Identifier = String;

pub fn new_identifier(element: String) -> Identifier {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"Identifier\", \"value\": {}, }} ",
        element
    )
}

pub type YulIdentifier = String;

pub fn new_yul_identifier(element: String) -> YulIdentifier {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulIdentifier\", \"value\": {}, }} ",
        element
    )
}

pub type YulDecimalLiteral = String;

pub fn new_yul_decimal_literal(element: String) -> YulDecimalLiteral {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulDecimalLiteral\", \"value\": {}, }} ",
        element
    )
}

pub type YulHexLiteral = String;

pub fn new_yul_hex_literal(element: String) -> YulHexLiteral {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulHexLiteral\", \"value\": {}, }} ",
        element
    )
}

pub type YulAbstractKeyword = String;

pub fn new_yul_abstract_keyword(element: String) -> YulAbstractKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulAbstractKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulAfterKeyword = String;

pub fn new_yul_after_keyword(element: String) -> YulAfterKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulAfterKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulAliasKeyword = String;

pub fn new_yul_alias_keyword(element: String) -> YulAliasKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulAliasKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulAnonymousKeyword = String;

pub fn new_yul_anonymous_keyword(element: String) -> YulAnonymousKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulAnonymousKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulApplyKeyword = String;

pub fn new_yul_apply_keyword(element: String) -> YulApplyKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulApplyKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulAsKeyword = String;

pub fn new_yul_as_keyword(element: String) -> YulAsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulAsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulAssemblyKeyword = String;

pub fn new_yul_assembly_keyword(element: String) -> YulAssemblyKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulAssemblyKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulAutoKeyword = String;

pub fn new_yul_auto_keyword(element: String) -> YulAutoKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulAutoKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulBoolKeyword = String;

pub fn new_yul_bool_keyword(element: String) -> YulBoolKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulBoolKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulBreakKeyword = String;

pub fn new_yul_break_keyword(element: String) -> YulBreakKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulBreakKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulBytesKeyword = String;

pub fn new_yul_bytes_keyword(element: String) -> YulBytesKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulBytesKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulCallDataKeyword = String;

pub fn new_yul_call_data_keyword(element: String) -> YulCallDataKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulCallDataKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulCaseKeyword = String;

pub fn new_yul_case_keyword(element: String) -> YulCaseKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulCaseKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulCatchKeyword = String;

pub fn new_yul_catch_keyword(element: String) -> YulCatchKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulCatchKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulConstantKeyword = String;

pub fn new_yul_constant_keyword(element: String) -> YulConstantKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulConstantKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulConstructorKeyword = String;

pub fn new_yul_constructor_keyword(element: String) -> YulConstructorKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulConstructorKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulContinueKeyword = String;

pub fn new_yul_continue_keyword(element: String) -> YulContinueKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulContinueKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulContractKeyword = String;

pub fn new_yul_contract_keyword(element: String) -> YulContractKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulContractKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulCopyOfKeyword = String;

pub fn new_yul_copy_of_keyword(element: String) -> YulCopyOfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulCopyOfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulDaysKeyword = String;

pub fn new_yul_days_keyword(element: String) -> YulDaysKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulDaysKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulDefaultKeyword = String;

pub fn new_yul_default_keyword(element: String) -> YulDefaultKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulDefaultKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulDefineKeyword = String;

pub fn new_yul_define_keyword(element: String) -> YulDefineKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulDefineKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulDeleteKeyword = String;

pub fn new_yul_delete_keyword(element: String) -> YulDeleteKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulDeleteKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulDoKeyword = String;

pub fn new_yul_do_keyword(element: String) -> YulDoKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulDoKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulElseKeyword = String;

pub fn new_yul_else_keyword(element: String) -> YulElseKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulElseKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulEmitKeyword = String;

pub fn new_yul_emit_keyword(element: String) -> YulEmitKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulEmitKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulEnumKeyword = String;

pub fn new_yul_enum_keyword(element: String) -> YulEnumKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulEnumKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulEtherKeyword = String;

pub fn new_yul_ether_keyword(element: String) -> YulEtherKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulEtherKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulEventKeyword = String;

pub fn new_yul_event_keyword(element: String) -> YulEventKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulEventKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulExternalKeyword = String;

pub fn new_yul_external_keyword(element: String) -> YulExternalKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulExternalKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulFallbackKeyword = String;

pub fn new_yul_fallback_keyword(element: String) -> YulFallbackKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulFallbackKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulFalseKeyword = String;

pub fn new_yul_false_keyword(element: String) -> YulFalseKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulFalseKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulFinalKeyword = String;

pub fn new_yul_final_keyword(element: String) -> YulFinalKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulFinalKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulFinneyKeyword = String;

pub fn new_yul_finney_keyword(element: String) -> YulFinneyKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulFinneyKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulFixedKeyword = String;

pub fn new_yul_fixed_keyword(element: String) -> YulFixedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulFixedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulForKeyword = String;

pub fn new_yul_for_keyword(element: String) -> YulForKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulForKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulFunctionKeyword = String;

pub fn new_yul_function_keyword(element: String) -> YulFunctionKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulFunctionKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulGweiKeyword = String;

pub fn new_yul_gwei_keyword(element: String) -> YulGweiKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulGweiKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulHexKeyword = String;

pub fn new_yul_hex_keyword(element: String) -> YulHexKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulHexKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulHoursKeyword = String;

pub fn new_yul_hours_keyword(element: String) -> YulHoursKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulHoursKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulIfKeyword = String;

pub fn new_yul_if_keyword(element: String) -> YulIfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulIfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulImmutableKeyword = String;

pub fn new_yul_immutable_keyword(element: String) -> YulImmutableKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulImmutableKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulImplementsKeyword = String;

pub fn new_yul_implements_keyword(element: String) -> YulImplementsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulImplementsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulImportKeyword = String;

pub fn new_yul_import_keyword(element: String) -> YulImportKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulImportKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulIndexedKeyword = String;

pub fn new_yul_indexed_keyword(element: String) -> YulIndexedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulIndexedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulInKeyword = String;

pub fn new_yul_in_keyword(element: String) -> YulInKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulInKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulInlineKeyword = String;

pub fn new_yul_inline_keyword(element: String) -> YulInlineKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulInlineKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulInterfaceKeyword = String;

pub fn new_yul_interface_keyword(element: String) -> YulInterfaceKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulInterfaceKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulInternalKeyword = String;

pub fn new_yul_internal_keyword(element: String) -> YulInternalKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulInternalKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulIntKeyword = String;

pub fn new_yul_int_keyword(element: String) -> YulIntKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulIntKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulIsKeyword = String;

pub fn new_yul_is_keyword(element: String) -> YulIsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulIsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulLeaveKeyword = String;

pub fn new_yul_leave_keyword(element: String) -> YulLeaveKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulLeaveKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulLetKeyword = String;

pub fn new_yul_let_keyword(element: String) -> YulLetKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulLetKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulLibraryKeyword = String;

pub fn new_yul_library_keyword(element: String) -> YulLibraryKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulLibraryKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulMacroKeyword = String;

pub fn new_yul_macro_keyword(element: String) -> YulMacroKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulMacroKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulMappingKeyword = String;

pub fn new_yul_mapping_keyword(element: String) -> YulMappingKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulMappingKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulMatchKeyword = String;

pub fn new_yul_match_keyword(element: String) -> YulMatchKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulMatchKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulMemoryKeyword = String;

pub fn new_yul_memory_keyword(element: String) -> YulMemoryKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulMemoryKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulMinutesKeyword = String;

pub fn new_yul_minutes_keyword(element: String) -> YulMinutesKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulMinutesKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulModifierKeyword = String;

pub fn new_yul_modifier_keyword(element: String) -> YulModifierKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulModifierKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulMutableKeyword = String;

pub fn new_yul_mutable_keyword(element: String) -> YulMutableKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulMutableKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulNewKeyword = String;

pub fn new_yul_new_keyword(element: String) -> YulNewKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulNewKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulNullKeyword = String;

pub fn new_yul_null_keyword(element: String) -> YulNullKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulNullKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulOfKeyword = String;

pub fn new_yul_of_keyword(element: String) -> YulOfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulOfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulOverrideKeyword = String;

pub fn new_yul_override_keyword(element: String) -> YulOverrideKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulOverrideKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulPartialKeyword = String;

pub fn new_yul_partial_keyword(element: String) -> YulPartialKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulPartialKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulPayableKeyword = String;

pub fn new_yul_payable_keyword(element: String) -> YulPayableKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulPayableKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulPragmaKeyword = String;

pub fn new_yul_pragma_keyword(element: String) -> YulPragmaKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulPragmaKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulPrivateKeyword = String;

pub fn new_yul_private_keyword(element: String) -> YulPrivateKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulPrivateKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulPromiseKeyword = String;

pub fn new_yul_promise_keyword(element: String) -> YulPromiseKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulPromiseKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulPublicKeyword = String;

pub fn new_yul_public_keyword(element: String) -> YulPublicKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulPublicKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulPureKeyword = String;

pub fn new_yul_pure_keyword(element: String) -> YulPureKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulPureKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulReceiveKeyword = String;

pub fn new_yul_receive_keyword(element: String) -> YulReceiveKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulReceiveKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulReferenceKeyword = String;

pub fn new_yul_reference_keyword(element: String) -> YulReferenceKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulReferenceKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulRelocatableKeyword = String;

pub fn new_yul_relocatable_keyword(element: String) -> YulRelocatableKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulRelocatableKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulReturnsKeyword = String;

pub fn new_yul_returns_keyword(element: String) -> YulReturnsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulReturnsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulSealedKeyword = String;

pub fn new_yul_sealed_keyword(element: String) -> YulSealedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulSealedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulSecondsKeyword = String;

pub fn new_yul_seconds_keyword(element: String) -> YulSecondsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulSecondsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulSizeOfKeyword = String;

pub fn new_yul_size_of_keyword(element: String) -> YulSizeOfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulSizeOfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulStaticKeyword = String;

pub fn new_yul_static_keyword(element: String) -> YulStaticKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulStaticKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulStorageKeyword = String;

pub fn new_yul_storage_keyword(element: String) -> YulStorageKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulStorageKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulStringKeyword = String;

pub fn new_yul_string_keyword(element: String) -> YulStringKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulStringKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulStructKeyword = String;

pub fn new_yul_struct_keyword(element: String) -> YulStructKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulStructKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulSuperKeyword = String;

pub fn new_yul_super_keyword(element: String) -> YulSuperKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulSuperKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulSupportsKeyword = String;

pub fn new_yul_supports_keyword(element: String) -> YulSupportsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulSupportsKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulSwitchKeyword = String;

pub fn new_yul_switch_keyword(element: String) -> YulSwitchKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulSwitchKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulSzaboKeyword = String;

pub fn new_yul_szabo_keyword(element: String) -> YulSzaboKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulSzaboKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulThisKeyword = String;

pub fn new_yul_this_keyword(element: String) -> YulThisKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulThisKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulThrowKeyword = String;

pub fn new_yul_throw_keyword(element: String) -> YulThrowKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulThrowKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulTrueKeyword = String;

pub fn new_yul_true_keyword(element: String) -> YulTrueKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulTrueKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulTryKeyword = String;

pub fn new_yul_try_keyword(element: String) -> YulTryKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulTryKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulTypeDefKeyword = String;

pub fn new_yul_type_def_keyword(element: String) -> YulTypeDefKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulTypeDefKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulTypeKeyword = String;

pub fn new_yul_type_keyword(element: String) -> YulTypeKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulTypeKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulTypeOfKeyword = String;

pub fn new_yul_type_of_keyword(element: String) -> YulTypeOfKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulTypeOfKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulUfixedKeyword = String;

pub fn new_yul_ufixed_keyword(element: String) -> YulUfixedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulUfixedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulUintKeyword = String;

pub fn new_yul_uint_keyword(element: String) -> YulUintKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulUintKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulUncheckedKeyword = String;

pub fn new_yul_unchecked_keyword(element: String) -> YulUncheckedKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulUncheckedKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulUsingKeyword = String;

pub fn new_yul_using_keyword(element: String) -> YulUsingKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulUsingKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulVarKeyword = String;

pub fn new_yul_var_keyword(element: String) -> YulVarKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulVarKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulViewKeyword = String;

pub fn new_yul_view_keyword(element: String) -> YulViewKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulViewKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulVirtualKeyword = String;

pub fn new_yul_virtual_keyword(element: String) -> YulVirtualKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulVirtualKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulWeeksKeyword = String;

pub fn new_yul_weeks_keyword(element: String) -> YulWeeksKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulWeeksKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulWeiKeyword = String;

pub fn new_yul_wei_keyword(element: String) -> YulWeiKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulWeiKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulWhileKeyword = String;

pub fn new_yul_while_keyword(element: String) -> YulWhileKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulWhileKeyword\", \"value\": {}, }} ",
        element
    )
}

pub type YulYearsKeyword = String;

pub fn new_yul_years_keyword(element: String) -> YulYearsKeyword {
    format!(
        " {{  \"item\": \"Terminal\", \"name\": \"YulYearsKeyword\", \"value\": {}, }} ",
        element
    )
}
