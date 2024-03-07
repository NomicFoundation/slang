// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// Slang License: https://github.com/NomicFoundation/slang/blob/main/LICENSE
// NAPI-RS License: https://github.com/napi-rs/napi-rs/blob/main/LICENSE

/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export namespace kinds {
  export enum RuleKind {
    ABICoderPragma = "ABICoderPragma",
    AdditiveExpression = "AdditiveExpression",
    AddressType = "AddressType",
    AndExpression = "AndExpression",
    ArgumentsDeclaration = "ArgumentsDeclaration",
    ArrayExpression = "ArrayExpression",
    ArrayTypeName = "ArrayTypeName",
    ArrayValues = "ArrayValues",
    AssemblyFlags = "AssemblyFlags",
    AssemblyFlagsDeclaration = "AssemblyFlagsDeclaration",
    AssemblyStatement = "AssemblyStatement",
    AssignmentExpression = "AssignmentExpression",
    BitwiseAndExpression = "BitwiseAndExpression",
    BitwiseOrExpression = "BitwiseOrExpression",
    BitwiseXorExpression = "BitwiseXorExpression",
    Block = "Block",
    BreakStatement = "BreakStatement",
    CallOptions = "CallOptions",
    CallOptionsExpression = "CallOptionsExpression",
    CatchClause = "CatchClause",
    CatchClauseError = "CatchClauseError",
    CatchClauses = "CatchClauses",
    ComparisonExpression = "ComparisonExpression",
    ConditionalExpression = "ConditionalExpression",
    ConstantDefinition = "ConstantDefinition",
    ConstructorAttribute = "ConstructorAttribute",
    ConstructorAttributes = "ConstructorAttributes",
    ConstructorDefinition = "ConstructorDefinition",
    ContinueStatement = "ContinueStatement",
    ContractDefinition = "ContractDefinition",
    ContractMember = "ContractMember",
    ContractMembers = "ContractMembers",
    DecimalNumberExpression = "DecimalNumberExpression",
    DeleteStatement = "DeleteStatement",
    DoWhileStatement = "DoWhileStatement",
    ElementaryType = "ElementaryType",
    ElseBranch = "ElseBranch",
    EmitStatement = "EmitStatement",
    EnumDefinition = "EnumDefinition",
    EnumMembers = "EnumMembers",
    EqualityExpression = "EqualityExpression",
    ErrorDefinition = "ErrorDefinition",
    ErrorParameter = "ErrorParameter",
    ErrorParameters = "ErrorParameters",
    ErrorParametersDeclaration = "ErrorParametersDeclaration",
    EventDefinition = "EventDefinition",
    EventParameter = "EventParameter",
    EventParameters = "EventParameters",
    EventParametersDeclaration = "EventParametersDeclaration",
    ExperimentalFeature = "ExperimentalFeature",
    ExperimentalPragma = "ExperimentalPragma",
    ExponentiationExpression = "ExponentiationExpression",
    Expression = "Expression",
    ExpressionStatement = "ExpressionStatement",
    FallbackFunctionAttribute = "FallbackFunctionAttribute",
    FallbackFunctionAttributes = "FallbackFunctionAttributes",
    FallbackFunctionDefinition = "FallbackFunctionDefinition",
    ForStatement = "ForStatement",
    ForStatementCondition = "ForStatementCondition",
    ForStatementInitialization = "ForStatementInitialization",
    FunctionAttribute = "FunctionAttribute",
    FunctionAttributes = "FunctionAttributes",
    FunctionBody = "FunctionBody",
    FunctionCallExpression = "FunctionCallExpression",
    FunctionDefinition = "FunctionDefinition",
    FunctionName = "FunctionName",
    FunctionType = "FunctionType",
    FunctionTypeAttribute = "FunctionTypeAttribute",
    FunctionTypeAttributes = "FunctionTypeAttributes",
    HexNumberExpression = "HexNumberExpression",
    HexStringLiteral = "HexStringLiteral",
    HexStringLiterals = "HexStringLiterals",
    IdentifierPath = "IdentifierPath",
    IfStatement = "IfStatement",
    ImportAlias = "ImportAlias",
    ImportClause = "ImportClause",
    ImportDeconstruction = "ImportDeconstruction",
    ImportDeconstructionSymbol = "ImportDeconstructionSymbol",
    ImportDeconstructionSymbols = "ImportDeconstructionSymbols",
    ImportDirective = "ImportDirective",
    IndexAccessEnd = "IndexAccessEnd",
    IndexAccessExpression = "IndexAccessExpression",
    InheritanceSpecifier = "InheritanceSpecifier",
    InheritanceType = "InheritanceType",
    InheritanceTypes = "InheritanceTypes",
    InterfaceDefinition = "InterfaceDefinition",
    InterfaceMembers = "InterfaceMembers",
    LibraryDefinition = "LibraryDefinition",
    LibraryMembers = "LibraryMembers",
    MappingKey = "MappingKey",
    MappingKeyType = "MappingKeyType",
    MappingType = "MappingType",
    MappingValue = "MappingValue",
    MemberAccess = "MemberAccess",
    MemberAccessExpression = "MemberAccessExpression",
    ModifierAttribute = "ModifierAttribute",
    ModifierAttributes = "ModifierAttributes",
    ModifierDefinition = "ModifierDefinition",
    ModifierInvocation = "ModifierInvocation",
    MultiplicativeExpression = "MultiplicativeExpression",
    NamedArgument = "NamedArgument",
    NamedArgumentGroup = "NamedArgumentGroup",
    NamedArguments = "NamedArguments",
    NamedArgumentsDeclaration = "NamedArgumentsDeclaration",
    NamedImport = "NamedImport",
    NewExpression = "NewExpression",
    NumberUnit = "NumberUnit",
    OrExpression = "OrExpression",
    OverridePaths = "OverridePaths",
    OverridePathsDeclaration = "OverridePathsDeclaration",
    OverrideSpecifier = "OverrideSpecifier",
    Parameter = "Parameter",
    Parameters = "Parameters",
    ParametersDeclaration = "ParametersDeclaration",
    PathImport = "PathImport",
    PositionalArguments = "PositionalArguments",
    PositionalArgumentsDeclaration = "PositionalArgumentsDeclaration",
    PostfixExpression = "PostfixExpression",
    Pragma = "Pragma",
    PragmaDirective = "PragmaDirective",
    PrefixExpression = "PrefixExpression",
    ReceiveFunctionAttribute = "ReceiveFunctionAttribute",
    ReceiveFunctionAttributes = "ReceiveFunctionAttributes",
    ReceiveFunctionDefinition = "ReceiveFunctionDefinition",
    ReturnStatement = "ReturnStatement",
    ReturnsDeclaration = "ReturnsDeclaration",
    RevertStatement = "RevertStatement",
    ShiftExpression = "ShiftExpression",
    SourceUnit = "SourceUnit",
    SourceUnitMember = "SourceUnitMember",
    SourceUnitMembers = "SourceUnitMembers",
    StateVariableAttribute = "StateVariableAttribute",
    StateVariableAttributes = "StateVariableAttributes",
    StateVariableDefinition = "StateVariableDefinition",
    StateVariableDefinitionValue = "StateVariableDefinitionValue",
    Statement = "Statement",
    Statements = "Statements",
    StorageLocation = "StorageLocation",
    StringExpression = "StringExpression",
    StringLiteral = "StringLiteral",
    StringLiterals = "StringLiterals",
    StructDefinition = "StructDefinition",
    StructMember = "StructMember",
    StructMembers = "StructMembers",
    ThrowStatement = "ThrowStatement",
    TryStatement = "TryStatement",
    TupleDeconstructionElement = "TupleDeconstructionElement",
    TupleDeconstructionElements = "TupleDeconstructionElements",
    TupleDeconstructionStatement = "TupleDeconstructionStatement",
    TupleExpression = "TupleExpression",
    TupleMember = "TupleMember",
    TupleValue = "TupleValue",
    TupleValues = "TupleValues",
    TypeExpression = "TypeExpression",
    TypeName = "TypeName",
    TypedTupleMember = "TypedTupleMember",
    UncheckedBlock = "UncheckedBlock",
    UnicodeStringLiteral = "UnicodeStringLiteral",
    UnicodeStringLiterals = "UnicodeStringLiterals",
    UnnamedFunctionAttribute = "UnnamedFunctionAttribute",
    UnnamedFunctionAttributes = "UnnamedFunctionAttributes",
    UnnamedFunctionDefinition = "UnnamedFunctionDefinition",
    UntypedTupleMember = "UntypedTupleMember",
    UserDefinedValueTypeDefinition = "UserDefinedValueTypeDefinition",
    UsingAlias = "UsingAlias",
    UsingClause = "UsingClause",
    UsingDeconstruction = "UsingDeconstruction",
    UsingDeconstructionSymbol = "UsingDeconstructionSymbol",
    UsingDeconstructionSymbols = "UsingDeconstructionSymbols",
    UsingDirective = "UsingDirective",
    UsingOperator = "UsingOperator",
    UsingTarget = "UsingTarget",
    VariableDeclarationStatement = "VariableDeclarationStatement",
    VariableDeclarationType = "VariableDeclarationType",
    VariableDeclarationValue = "VariableDeclarationValue",
    VersionPragma = "VersionPragma",
    VersionPragmaExpression = "VersionPragmaExpression",
    VersionPragmaExpressions = "VersionPragmaExpressions",
    VersionPragmaOrExpression = "VersionPragmaOrExpression",
    VersionPragmaPrefixExpression = "VersionPragmaPrefixExpression",
    VersionPragmaRangeExpression = "VersionPragmaRangeExpression",
    VersionPragmaSpecifier = "VersionPragmaSpecifier",
    WhileStatement = "WhileStatement",
    YulArguments = "YulArguments",
    YulAssignmentOperator = "YulAssignmentOperator",
    YulAssignmentStatement = "YulAssignmentStatement",
    YulBlock = "YulBlock",
    YulBreakStatement = "YulBreakStatement",
    YulBuiltInFunction = "YulBuiltInFunction",
    YulColonAndEqual = "YulColonAndEqual",
    YulContinueStatement = "YulContinueStatement",
    YulDefaultCase = "YulDefaultCase",
    YulExpression = "YulExpression",
    YulForStatement = "YulForStatement",
    YulFunctionCallExpression = "YulFunctionCallExpression",
    YulFunctionDefinition = "YulFunctionDefinition",
    YulIdentifierPath = "YulIdentifierPath",
    YulIdentifierPaths = "YulIdentifierPaths",
    YulIfStatement = "YulIfStatement",
    YulLabel = "YulLabel",
    YulLeaveStatement = "YulLeaveStatement",
    YulLiteral = "YulLiteral",
    YulParameters = "YulParameters",
    YulParametersDeclaration = "YulParametersDeclaration",
    YulReturnVariables = "YulReturnVariables",
    YulReturnsDeclaration = "YulReturnsDeclaration",
    YulStatement = "YulStatement",
    YulStatements = "YulStatements",
    YulSwitchCase = "YulSwitchCase",
    YulSwitchCases = "YulSwitchCases",
    YulSwitchStatement = "YulSwitchStatement",
    YulValueCase = "YulValueCase",
    YulVariableDeclarationStatement = "YulVariableDeclarationStatement",
    YulVariableDeclarationValue = "YulVariableDeclarationValue",
  }
  export enum NodeLabel {
    Item = "Item",
    Variant = "Variant",
    Separator = "Separator",
    Operand = "Operand",
    LeftOperand = "LeftOperand",
    RightOperand = "RightOperand",
    LeadingTrivia = "LeadingTrivia",
    TrailingTrivia = "TrailingTrivia",
    AbicoderKeyword = "AbicoderKeyword",
    AbstractKeyword = "AbstractKeyword",
    AddressKeyword = "AddressKeyword",
    Alias = "Alias",
    AnonymousKeyword = "AnonymousKeyword",
    Arguments = "Arguments",
    AsKeyword = "AsKeyword",
    AssemblyKeyword = "AssemblyKeyword",
    Assignment = "Assignment",
    Asterisk = "Asterisk",
    Attributes = "Attributes",
    Block = "Block",
    Body = "Body",
    BreakKeyword = "BreakKeyword",
    CaseKeyword = "CaseKeyword",
    Cases = "Cases",
    CatchClauses = "CatchClauses",
    CatchKeyword = "CatchKeyword",
    Clause = "Clause",
    CloseBrace = "CloseBrace",
    CloseBracket = "CloseBracket",
    CloseParen = "CloseParen",
    Colon = "Colon",
    Condition = "Condition",
    ConstantKeyword = "ConstantKeyword",
    ConstructorKeyword = "ConstructorKeyword",
    ContinueKeyword = "ContinueKeyword",
    ContractKeyword = "ContractKeyword",
    DefaultKeyword = "DefaultKeyword",
    DeleteKeyword = "DeleteKeyword",
    DoKeyword = "DoKeyword",
    Elements = "Elements",
    ElseBranch = "ElseBranch",
    ElseKeyword = "ElseKeyword",
    EmitKeyword = "EmitKeyword",
    End = "End",
    EnumKeyword = "EnumKeyword",
    Equal = "Equal",
    EqualGreaterThan = "EqualGreaterThan",
    Error = "Error",
    ErrorKeyword = "ErrorKeyword",
    Event = "Event",
    EventKeyword = "EventKeyword",
    ExperimentalKeyword = "ExperimentalKeyword",
    Expression = "Expression",
    Expressions = "Expressions",
    FallbackKeyword = "FallbackKeyword",
    FalseExpression = "FalseExpression",
    Feature = "Feature",
    Flags = "Flags",
    ForKeyword = "ForKeyword",
    FromKeyword = "FromKeyword",
    FunctionKeyword = "FunctionKeyword",
    GlobalKeyword = "GlobalKeyword",
    Identifier = "Identifier",
    IfKeyword = "IfKeyword",
    ImportKeyword = "ImportKeyword",
    Index = "Index",
    IndexedKeyword = "IndexedKeyword",
    Inheritence = "Inheritence",
    Initialization = "Initialization",
    InterfaceKeyword = "InterfaceKeyword",
    IsKeyword = "IsKeyword",
    Items = "Items",
    Iterator = "Iterator",
    KeyType = "KeyType",
    Label = "Label",
    LeaveKeyword = "LeaveKeyword",
    LetKeyword = "LetKeyword",
    LibraryKeyword = "LibraryKeyword",
    Literal = "Literal",
    MappingKeyword = "MappingKeyword",
    Member = "Member",
    Members = "Members",
    MinusGreaterThan = "MinusGreaterThan",
    ModifierKeyword = "ModifierKeyword",
    Name = "Name",
    Names = "Names",
    NewKeyword = "NewKeyword",
    OpenBrace = "OpenBrace",
    OpenBracket = "OpenBracket",
    OpenParen = "OpenParen",
    Operator = "Operator",
    Overridden = "Overridden",
    OverrideKeyword = "OverrideKeyword",
    Parameters = "Parameters",
    Path = "Path",
    Paths = "Paths",
    PayableKeyword = "PayableKeyword",
    Period = "Period",
    Pragma = "Pragma",
    PragmaKeyword = "PragmaKeyword",
    QuestionMark = "QuestionMark",
    ReceiveKeyword = "ReceiveKeyword",
    ReturnKeyword = "ReturnKeyword",
    Returns = "Returns",
    ReturnsKeyword = "ReturnsKeyword",
    RevertKeyword = "RevertKeyword",
    Semicolon = "Semicolon",
    SolidityKeyword = "SolidityKeyword",
    Start = "Start",
    Statements = "Statements",
    StorageLocation = "StorageLocation",
    StructKeyword = "StructKeyword",
    SwitchKeyword = "SwitchKeyword",
    Symbols = "Symbols",
    Target = "Target",
    ThrowKeyword = "ThrowKeyword",
    TrueExpression = "TrueExpression",
    TryKeyword = "TryKeyword",
    TypeKeyword = "TypeKeyword",
    TypeName = "TypeName",
    Types = "Types",
    UncheckedKeyword = "UncheckedKeyword",
    Unit = "Unit",
    UsingKeyword = "UsingKeyword",
    Value = "Value",
    ValueType = "ValueType",
    VarKeyword = "VarKeyword",
    VariableType = "VariableType",
    Variables = "Variables",
    Version = "Version",
    WhileKeyword = "WhileKeyword",
  }
  export enum TokenKind {
    SKIPPED = "SKIPPED",
    AbicoderKeyword = "AbicoderKeyword",
    AbstractKeyword = "AbstractKeyword",
    AddressKeyword = "AddressKeyword",
    AfterKeyword = "AfterKeyword",
    AliasKeyword = "AliasKeyword",
    Ampersand = "Ampersand",
    AmpersandAmpersand = "AmpersandAmpersand",
    AmpersandEqual = "AmpersandEqual",
    AnonymousKeyword = "AnonymousKeyword",
    ApplyKeyword = "ApplyKeyword",
    AsKeyword = "AsKeyword",
    AssemblyKeyword = "AssemblyKeyword",
    Asterisk = "Asterisk",
    AsteriskAsterisk = "AsteriskAsterisk",
    AsteriskEqual = "AsteriskEqual",
    AutoKeyword = "AutoKeyword",
    Bang = "Bang",
    BangEqual = "BangEqual",
    Bar = "Bar",
    BarBar = "BarBar",
    BarEqual = "BarEqual",
    BoolKeyword = "BoolKeyword",
    BreakKeyword = "BreakKeyword",
    ByteKeyword = "ByteKeyword",
    BytesKeyword = "BytesKeyword",
    CallDataKeyword = "CallDataKeyword",
    Caret = "Caret",
    CaretEqual = "CaretEqual",
    CaseKeyword = "CaseKeyword",
    CatchKeyword = "CatchKeyword",
    CloseBrace = "CloseBrace",
    CloseBracket = "CloseBracket",
    CloseParen = "CloseParen",
    Colon = "Colon",
    ColonEqual = "ColonEqual",
    Comma = "Comma",
    ConstantKeyword = "ConstantKeyword",
    ConstructorKeyword = "ConstructorKeyword",
    ContinueKeyword = "ContinueKeyword",
    ContractKeyword = "ContractKeyword",
    CopyOfKeyword = "CopyOfKeyword",
    DaysKeyword = "DaysKeyword",
    DecimalLiteral = "DecimalLiteral",
    DefaultKeyword = "DefaultKeyword",
    DefineKeyword = "DefineKeyword",
    DeleteKeyword = "DeleteKeyword",
    DoKeyword = "DoKeyword",
    DoubleQuotedHexStringLiteral = "DoubleQuotedHexStringLiteral",
    DoubleQuotedStringLiteral = "DoubleQuotedStringLiteral",
    DoubleQuotedUnicodeStringLiteral = "DoubleQuotedUnicodeStringLiteral",
    ElseKeyword = "ElseKeyword",
    EmitKeyword = "EmitKeyword",
    EndOfLine = "EndOfLine",
    EnumKeyword = "EnumKeyword",
    Equal = "Equal",
    EqualEqual = "EqualEqual",
    EqualGreaterThan = "EqualGreaterThan",
    ErrorKeyword = "ErrorKeyword",
    EtherKeyword = "EtherKeyword",
    EventKeyword = "EventKeyword",
    ExperimentalKeyword = "ExperimentalKeyword",
    ExternalKeyword = "ExternalKeyword",
    FallbackKeyword = "FallbackKeyword",
    FalseKeyword = "FalseKeyword",
    FinalKeyword = "FinalKeyword",
    FinneyKeyword = "FinneyKeyword",
    FixedKeyword = "FixedKeyword",
    ForKeyword = "ForKeyword",
    FromKeyword = "FromKeyword",
    FunctionKeyword = "FunctionKeyword",
    GlobalKeyword = "GlobalKeyword",
    GreaterThan = "GreaterThan",
    GreaterThanEqual = "GreaterThanEqual",
    GreaterThanGreaterThan = "GreaterThanGreaterThan",
    GreaterThanGreaterThanEqual = "GreaterThanGreaterThanEqual",
    GreaterThanGreaterThanGreaterThan = "GreaterThanGreaterThanGreaterThan",
    GreaterThanGreaterThanGreaterThanEqual = "GreaterThanGreaterThanGreaterThanEqual",
    GweiKeyword = "GweiKeyword",
    HexKeyword = "HexKeyword",
    HexLiteral = "HexLiteral",
    HoursKeyword = "HoursKeyword",
    Identifier = "Identifier",
    IfKeyword = "IfKeyword",
    ImmutableKeyword = "ImmutableKeyword",
    ImplementsKeyword = "ImplementsKeyword",
    ImportKeyword = "ImportKeyword",
    InKeyword = "InKeyword",
    IndexedKeyword = "IndexedKeyword",
    InlineKeyword = "InlineKeyword",
    IntKeyword = "IntKeyword",
    InterfaceKeyword = "InterfaceKeyword",
    InternalKeyword = "InternalKeyword",
    IsKeyword = "IsKeyword",
    LessThan = "LessThan",
    LessThanEqual = "LessThanEqual",
    LessThanLessThan = "LessThanLessThan",
    LessThanLessThanEqual = "LessThanLessThanEqual",
    LetKeyword = "LetKeyword",
    LibraryKeyword = "LibraryKeyword",
    MacroKeyword = "MacroKeyword",
    MappingKeyword = "MappingKeyword",
    MatchKeyword = "MatchKeyword",
    MemoryKeyword = "MemoryKeyword",
    Minus = "Minus",
    MinusEqual = "MinusEqual",
    MinusGreaterThan = "MinusGreaterThan",
    MinusMinus = "MinusMinus",
    MinutesKeyword = "MinutesKeyword",
    ModifierKeyword = "ModifierKeyword",
    MultiLineComment = "MultiLineComment",
    MultiLineNatSpecComment = "MultiLineNatSpecComment",
    MutableKeyword = "MutableKeyword",
    NewKeyword = "NewKeyword",
    NullKeyword = "NullKeyword",
    OfKeyword = "OfKeyword",
    OpenBrace = "OpenBrace",
    OpenBracket = "OpenBracket",
    OpenParen = "OpenParen",
    OverrideKeyword = "OverrideKeyword",
    PartialKeyword = "PartialKeyword",
    PayableKeyword = "PayableKeyword",
    Percent = "Percent",
    PercentEqual = "PercentEqual",
    Period = "Period",
    Plus = "Plus",
    PlusEqual = "PlusEqual",
    PlusPlus = "PlusPlus",
    PragmaKeyword = "PragmaKeyword",
    PrivateKeyword = "PrivateKeyword",
    PromiseKeyword = "PromiseKeyword",
    PublicKeyword = "PublicKeyword",
    PureKeyword = "PureKeyword",
    QuestionMark = "QuestionMark",
    ReceiveKeyword = "ReceiveKeyword",
    ReferenceKeyword = "ReferenceKeyword",
    RelocatableKeyword = "RelocatableKeyword",
    ReturnKeyword = "ReturnKeyword",
    ReturnsKeyword = "ReturnsKeyword",
    RevertKeyword = "RevertKeyword",
    SealedKeyword = "SealedKeyword",
    SecondsKeyword = "SecondsKeyword",
    Semicolon = "Semicolon",
    SingleLineComment = "SingleLineComment",
    SingleLineNatSpecComment = "SingleLineNatSpecComment",
    SingleQuotedHexStringLiteral = "SingleQuotedHexStringLiteral",
    SingleQuotedStringLiteral = "SingleQuotedStringLiteral",
    SingleQuotedUnicodeStringLiteral = "SingleQuotedUnicodeStringLiteral",
    SizeOfKeyword = "SizeOfKeyword",
    Slash = "Slash",
    SlashEqual = "SlashEqual",
    SolidityKeyword = "SolidityKeyword",
    StaticKeyword = "StaticKeyword",
    StorageKeyword = "StorageKeyword",
    StringKeyword = "StringKeyword",
    StructKeyword = "StructKeyword",
    SupportsKeyword = "SupportsKeyword",
    SwitchKeyword = "SwitchKeyword",
    SzaboKeyword = "SzaboKeyword",
    ThrowKeyword = "ThrowKeyword",
    Tilde = "Tilde",
    TrueKeyword = "TrueKeyword",
    TryKeyword = "TryKeyword",
    TypeDefKeyword = "TypeDefKeyword",
    TypeKeyword = "TypeKeyword",
    TypeOfKeyword = "TypeOfKeyword",
    UfixedKeyword = "UfixedKeyword",
    UintKeyword = "UintKeyword",
    UncheckedKeyword = "UncheckedKeyword",
    UsingKeyword = "UsingKeyword",
    VarKeyword = "VarKeyword",
    VersionPragmaValue = "VersionPragmaValue",
    ViewKeyword = "ViewKeyword",
    VirtualKeyword = "VirtualKeyword",
    WeeksKeyword = "WeeksKeyword",
    WeiKeyword = "WeiKeyword",
    WhileKeyword = "WhileKeyword",
    Whitespace = "Whitespace",
    YearsKeyword = "YearsKeyword",
    YulAbstractKeyword = "YulAbstractKeyword",
    YulAddKeyword = "YulAddKeyword",
    YulAddModKeyword = "YulAddModKeyword",
    YulAddressKeyword = "YulAddressKeyword",
    YulAfterKeyword = "YulAfterKeyword",
    YulAliasKeyword = "YulAliasKeyword",
    YulAndKeyword = "YulAndKeyword",
    YulAnonymousKeyword = "YulAnonymousKeyword",
    YulApplyKeyword = "YulApplyKeyword",
    YulAsKeyword = "YulAsKeyword",
    YulAssemblyKeyword = "YulAssemblyKeyword",
    YulAutoKeyword = "YulAutoKeyword",
    YulBalanceKeyword = "YulBalanceKeyword",
    YulBaseFeeKeyword = "YulBaseFeeKeyword",
    YulBlockHashKeyword = "YulBlockHashKeyword",
    YulBoolKeyword = "YulBoolKeyword",
    YulBreakKeyword = "YulBreakKeyword",
    YulByteKeyword = "YulByteKeyword",
    YulBytesKeyword = "YulBytesKeyword",
    YulCallCodeKeyword = "YulCallCodeKeyword",
    YulCallDataCopyKeyword = "YulCallDataCopyKeyword",
    YulCallDataKeyword = "YulCallDataKeyword",
    YulCallDataLoadKeyword = "YulCallDataLoadKeyword",
    YulCallDataSizeKeyword = "YulCallDataSizeKeyword",
    YulCallKeyword = "YulCallKeyword",
    YulCallValueKeyword = "YulCallValueKeyword",
    YulCallerKeyword = "YulCallerKeyword",
    YulCaseKeyword = "YulCaseKeyword",
    YulCatchKeyword = "YulCatchKeyword",
    YulChainIdKeyword = "YulChainIdKeyword",
    YulCoinBaseKeyword = "YulCoinBaseKeyword",
    YulConstantKeyword = "YulConstantKeyword",
    YulConstructorKeyword = "YulConstructorKeyword",
    YulContinueKeyword = "YulContinueKeyword",
    YulContractKeyword = "YulContractKeyword",
    YulCopyOfKeyword = "YulCopyOfKeyword",
    YulCreate2Keyword = "YulCreate2Keyword",
    YulCreateKeyword = "YulCreateKeyword",
    YulDaysKeyword = "YulDaysKeyword",
    YulDecimalLiteral = "YulDecimalLiteral",
    YulDefaultKeyword = "YulDefaultKeyword",
    YulDefineKeyword = "YulDefineKeyword",
    YulDelegateCallKeyword = "YulDelegateCallKeyword",
    YulDeleteKeyword = "YulDeleteKeyword",
    YulDifficultyKeyword = "YulDifficultyKeyword",
    YulDivKeyword = "YulDivKeyword",
    YulDoKeyword = "YulDoKeyword",
    YulElseKeyword = "YulElseKeyword",
    YulEmitKeyword = "YulEmitKeyword",
    YulEnumKeyword = "YulEnumKeyword",
    YulEqKeyword = "YulEqKeyword",
    YulEtherKeyword = "YulEtherKeyword",
    YulEventKeyword = "YulEventKeyword",
    YulExpKeyword = "YulExpKeyword",
    YulExtCodeCopyKeyword = "YulExtCodeCopyKeyword",
    YulExtCodeHashKeyword = "YulExtCodeHashKeyword",
    YulExtCodeSizeKeyword = "YulExtCodeSizeKeyword",
    YulExternalKeyword = "YulExternalKeyword",
    YulFallbackKeyword = "YulFallbackKeyword",
    YulFalseKeyword = "YulFalseKeyword",
    YulFinalKeyword = "YulFinalKeyword",
    YulFinneyKeyword = "YulFinneyKeyword",
    YulFixedKeyword = "YulFixedKeyword",
    YulForKeyword = "YulForKeyword",
    YulFunctionKeyword = "YulFunctionKeyword",
    YulGasKeyword = "YulGasKeyword",
    YulGasLimitKeyword = "YulGasLimitKeyword",
    YulGasPriceKeyword = "YulGasPriceKeyword",
    YulGtKeyword = "YulGtKeyword",
    YulGweiKeyword = "YulGweiKeyword",
    YulHexKeyword = "YulHexKeyword",
    YulHexLiteral = "YulHexLiteral",
    YulHoursKeyword = "YulHoursKeyword",
    YulIdentifier = "YulIdentifier",
    YulIfKeyword = "YulIfKeyword",
    YulImmutableKeyword = "YulImmutableKeyword",
    YulImplementsKeyword = "YulImplementsKeyword",
    YulImportKeyword = "YulImportKeyword",
    YulInKeyword = "YulInKeyword",
    YulIndexedKeyword = "YulIndexedKeyword",
    YulInlineKeyword = "YulInlineKeyword",
    YulIntKeyword = "YulIntKeyword",
    YulInterfaceKeyword = "YulInterfaceKeyword",
    YulInternalKeyword = "YulInternalKeyword",
    YulInvalidKeyword = "YulInvalidKeyword",
    YulIsKeyword = "YulIsKeyword",
    YulIsZeroKeyword = "YulIsZeroKeyword",
    YulKeccak256Keyword = "YulKeccak256Keyword",
    YulLeaveKeyword = "YulLeaveKeyword",
    YulLetKeyword = "YulLetKeyword",
    YulLibraryKeyword = "YulLibraryKeyword",
    YulLog0Keyword = "YulLog0Keyword",
    YulLog1Keyword = "YulLog1Keyword",
    YulLog2Keyword = "YulLog2Keyword",
    YulLog3Keyword = "YulLog3Keyword",
    YulLog4Keyword = "YulLog4Keyword",
    YulLtKeyword = "YulLtKeyword",
    YulMLoadKeyword = "YulMLoadKeyword",
    YulMSizeKeyword = "YulMSizeKeyword",
    YulMStore8Keyword = "YulMStore8Keyword",
    YulMStoreKeyword = "YulMStoreKeyword",
    YulMacroKeyword = "YulMacroKeyword",
    YulMappingKeyword = "YulMappingKeyword",
    YulMatchKeyword = "YulMatchKeyword",
    YulMemoryKeyword = "YulMemoryKeyword",
    YulMinutesKeyword = "YulMinutesKeyword",
    YulModKeyword = "YulModKeyword",
    YulModifierKeyword = "YulModifierKeyword",
    YulMulKeyword = "YulMulKeyword",
    YulMulModKeyword = "YulMulModKeyword",
    YulMutableKeyword = "YulMutableKeyword",
    YulNewKeyword = "YulNewKeyword",
    YulNotKeyword = "YulNotKeyword",
    YulNullKeyword = "YulNullKeyword",
    YulNumberKeyword = "YulNumberKeyword",
    YulOfKeyword = "YulOfKeyword",
    YulOrKeyword = "YulOrKeyword",
    YulOriginKeyword = "YulOriginKeyword",
    YulOverrideKeyword = "YulOverrideKeyword",
    YulPartialKeyword = "YulPartialKeyword",
    YulPayableKeyword = "YulPayableKeyword",
    YulPopKeyword = "YulPopKeyword",
    YulPragmaKeyword = "YulPragmaKeyword",
    YulPrevRandaoKeyword = "YulPrevRandaoKeyword",
    YulPrivateKeyword = "YulPrivateKeyword",
    YulPromiseKeyword = "YulPromiseKeyword",
    YulPublicKeyword = "YulPublicKeyword",
    YulPureKeyword = "YulPureKeyword",
    YulReceiveKeyword = "YulReceiveKeyword",
    YulReferenceKeyword = "YulReferenceKeyword",
    YulRelocatableKeyword = "YulRelocatableKeyword",
    YulReturnDataCopyKeyword = "YulReturnDataCopyKeyword",
    YulReturnDataSizeKeyword = "YulReturnDataSizeKeyword",
    YulReturnKeyword = "YulReturnKeyword",
    YulReturnsKeyword = "YulReturnsKeyword",
    YulRevertKeyword = "YulRevertKeyword",
    YulSDivKeyword = "YulSDivKeyword",
    YulSLoadKeyword = "YulSLoadKeyword",
    YulSModKeyword = "YulSModKeyword",
    YulSStoreKeyword = "YulSStoreKeyword",
    YulSarKeyword = "YulSarKeyword",
    YulSealedKeyword = "YulSealedKeyword",
    YulSecondsKeyword = "YulSecondsKeyword",
    YulSelfBalanceKeyword = "YulSelfBalanceKeyword",
    YulSelfDestructKeyword = "YulSelfDestructKeyword",
    YulSgtKeyword = "YulSgtKeyword",
    YulSha3Keyword = "YulSha3Keyword",
    YulShlKeyword = "YulShlKeyword",
    YulShrKeyword = "YulShrKeyword",
    YulSignExtendKeyword = "YulSignExtendKeyword",
    YulSizeOfKeyword = "YulSizeOfKeyword",
    YulSltKeyword = "YulSltKeyword",
    YulStaticCallKeyword = "YulStaticCallKeyword",
    YulStaticKeyword = "YulStaticKeyword",
    YulStopKeyword = "YulStopKeyword",
    YulStorageKeyword = "YulStorageKeyword",
    YulStringKeyword = "YulStringKeyword",
    YulStructKeyword = "YulStructKeyword",
    YulSubKeyword = "YulSubKeyword",
    YulSuicideKeyword = "YulSuicideKeyword",
    YulSupportsKeyword = "YulSupportsKeyword",
    YulSwitchKeyword = "YulSwitchKeyword",
    YulSzaboKeyword = "YulSzaboKeyword",
    YulThrowKeyword = "YulThrowKeyword",
    YulTimestampKeyword = "YulTimestampKeyword",
    YulTrueKeyword = "YulTrueKeyword",
    YulTryKeyword = "YulTryKeyword",
    YulTypeDefKeyword = "YulTypeDefKeyword",
    YulTypeKeyword = "YulTypeKeyword",
    YulTypeOfKeyword = "YulTypeOfKeyword",
    YulUfixedKeyword = "YulUfixedKeyword",
    YulUintKeyword = "YulUintKeyword",
    YulUncheckedKeyword = "YulUncheckedKeyword",
    YulUsingKeyword = "YulUsingKeyword",
    YulVarKeyword = "YulVarKeyword",
    YulViewKeyword = "YulViewKeyword",
    YulVirtualKeyword = "YulVirtualKeyword",
    YulWeeksKeyword = "YulWeeksKeyword",
    YulWeiKeyword = "YulWeiKeyword",
    YulWhileKeyword = "YulWhileKeyword",
    YulXorKeyword = "YulXorKeyword",
    YulYearsKeyword = "YulYearsKeyword",
  }
}
export namespace language {
  export class Language {
    constructor(version: string);
    get version(): string;
    static supportedVersions(): Array<string>;
    parse(kind: kinds.RuleKind, input: string): parse_output.ParseOutput;
  }
}
export namespace ast_internal {
  export function selectSequence(node: cst.RuleNode): Array<cst.Node | null>;
  export function selectChoice(node: cst.RuleNode): cst.Node;
  export function selectRepeated(node: cst.RuleNode): Array<cst.Node>;
  export function selectSeparated(node: cst.RuleNode): [Array<cst.Node>, Array<cst.Node>];
}
export namespace cst {
  export enum NodeType {
    Rule = 0,
    Token = 1,
  }
  export class RuleNode {
    get type(): NodeType.Rule;
    get kind(): kinds.RuleKind;
    get textLength(): text_index.TextIndex;
    children(): Array<cst.Node>;
    createCursor(textOffset: text_index.TextIndex): cursor.Cursor;
    unparse(): string;
  }
  export class TokenNode {
    get type(): NodeType.Token;
    get kind(): kinds.TokenKind;
    get textLength(): text_index.TextIndex;
    get text(): string;
    createCursor(textOffset: text_index.TextIndex): cursor.Cursor;
  }
}
export namespace cursor {
  export class Cursor {
    reset(): void;
    complete(): void;
    clone(): Cursor;
    spawn(): Cursor;
    get isCompleted(): boolean;
    node(): cst.Node;
    get label(): kinds.NodeLabel;
    get textOffset(): text_index.TextIndex;
    get textRange(): text_index.TextRange;
    get depth(): number;
    ancestors(): Array<cst.RuleNode>;
    goToNext(): boolean;
    goToNextNonDescendent(): boolean;
    goToPrevious(): boolean;
    goToParent(): boolean;
    goToFirstChild(): boolean;
    goToLastChild(): boolean;
    goToNthChild(childNumber: number): boolean;
    goToNextSibling(): boolean;
    goToPreviousSibling(): boolean;
    goToNextToken(): boolean;
    goToNextTokenWithKind(kind: kinds.TokenKind): boolean;
    goToNextTokenWithKinds(kinds: Array<kinds.TokenKind>): boolean;
    goToNextRule(): boolean;
    goToNextRuleWithKind(kind: kinds.RuleKind): boolean;
    goToNextRuleWithKinds(kinds: Array<kinds.RuleKind>): boolean;
    query(queries: Array<query.Query>): query.QueryResultIterator;
  }
}
export namespace parse_error {
  export class ParseError {
    get textRange(): text_index.TextRange;
    toErrorReport(sourceId: string, source: string, withColor: boolean): string;
  }
}
export namespace parse_output {
  export class ParseOutput {
    tree(): cst.Node;
    errors(): Array<parse_error.ParseError>;
    get isValid(): boolean;
    /** Creates a cursor that starts at the root of the parse tree. */
    createTreeCursor(): cursor.Cursor;
  }
}
export namespace query {
  export interface QueryResult {
    queryNumber: number;
    bindings: { [key: string]: cursor.Cursor[] };
  }
  export class Query {
    static parse(text: string): Query;
  }
  export class QueryResultIterator {
    next(): QueryResult | null;
  }
}
export namespace text_index {
  export interface TextIndex {
    utf8: number;
    utf16: number;
    char: number;
  }
  export interface TextRange {
    start: TextIndex;
    end: TextIndex;
  }
}

export namespace cst {
  export type Node = RuleNode | TokenNode;
}
