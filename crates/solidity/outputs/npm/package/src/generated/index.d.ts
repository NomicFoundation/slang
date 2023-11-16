// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// Slang License: https://github.com/NomicFoundation/slang/blob/main/LICENSE
// NAPI-RS License: https://github.com/napi-rs/napi-rs/blob/main/LICENSE

// @ts-nocheck

/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export namespace kinds {
  export enum ProductionKind {
    ABICoderPragma = "ABICoderPragma",
    AddressType = "AddressType",
    ArgumentsDeclaration = "ArgumentsDeclaration",
    ArrayExpression = "ArrayExpression",
    ArrayValues = "ArrayValues",
    AsciiStringLiterals = "AsciiStringLiterals",
    AssemblyFlags = "AssemblyFlags",
    AssemblyFlagsDeclaration = "AssemblyFlagsDeclaration",
    AssemblyStatement = "AssemblyStatement",
    Block = "Block",
    BreakStatement = "BreakStatement",
    CatchClause = "CatchClause",
    CatchClauseError = "CatchClauseError",
    CatchClauses = "CatchClauses",
    ConstantDefinition = "ConstantDefinition",
    ConstructorAttributes = "ConstructorAttributes",
    ConstructorDefinition = "ConstructorDefinition",
    ContinueStatement = "ContinueStatement",
    ContractDefinition = "ContractDefinition",
    ContractMembers = "ContractMembers",
    DecimalNumberExpression = "DecimalNumberExpression",
    DeconstructionImport = "DeconstructionImport",
    DeconstructionImportSymbol = "DeconstructionImportSymbol",
    DeconstructionImportSymbolsList = "DeconstructionImportSymbolsList",
    DeleteStatement = "DeleteStatement",
    DoWhileStatement = "DoWhileStatement",
    EmitStatement = "EmitStatement",
    EndOfFileTrivia = "EndOfFileTrivia",
    EnumDefinition = "EnumDefinition",
    EnumMembers = "EnumMembers",
    ErrorDefinition = "ErrorDefinition",
    ErrorParameter = "ErrorParameter",
    ErrorParameters = "ErrorParameters",
    ErrorParametersDeclaration = "ErrorParametersDeclaration",
    EventDefinition = "EventDefinition",
    EventParameter = "EventParameter",
    EventParameters = "EventParameters",
    EventParametersDeclaration = "EventParametersDeclaration",
    ExperimentalPragma = "ExperimentalPragma",
    Expression = "Expression",
    ExpressionStatement = "ExpressionStatement",
    FallbackFunctionAttributes = "FallbackFunctionAttributes",
    FallbackFunctionDefinition = "FallbackFunctionDefinition",
    ForStatement = "ForStatement",
    FunctionAttributes = "FunctionAttributes",
    FunctionCallOptions = "FunctionCallOptions",
    FunctionDefinition = "FunctionDefinition",
    FunctionType = "FunctionType",
    FunctionTypeAttributes = "FunctionTypeAttributes",
    HexNumberExpression = "HexNumberExpression",
    HexStringLiterals = "HexStringLiterals",
    IdentifierPath = "IdentifierPath",
    IfStatement = "IfStatement",
    ImportDirective = "ImportDirective",
    InheritanceSpecifier = "InheritanceSpecifier",
    InheritanceType = "InheritanceType",
    InheritanceTypes = "InheritanceTypes",
    InterfaceDefinition = "InterfaceDefinition",
    InterfaceMembers = "InterfaceMembers",
    LeadingTrivia = "LeadingTrivia",
    LibraryDefinition = "LibraryDefinition",
    LibraryMembers = "LibraryMembers",
    MappingKeyType = "MappingKeyType",
    MappingType = "MappingType",
    MappingValueType = "MappingValueType",
    ModifierAttributes = "ModifierAttributes",
    ModifierDefinition = "ModifierDefinition",
    ModifierInvocation = "ModifierInvocation",
    NamedArgument = "NamedArgument",
    NamedArguments = "NamedArguments",
    NamedArgumentsDeclaration = "NamedArgumentsDeclaration",
    NamedImport = "NamedImport",
    NewExpression = "NewExpression",
    OverridePaths = "OverridePaths",
    OverrideSpecifier = "OverrideSpecifier",
    Parameter = "Parameter",
    Parameters = "Parameters",
    ParametersDeclaration = "ParametersDeclaration",
    PathImport = "PathImport",
    PositionalArguments = "PositionalArguments",
    PragmaDirective = "PragmaDirective",
    ReceiveFunctionAttributes = "ReceiveFunctionAttributes",
    ReceiveFunctionDefinition = "ReceiveFunctionDefinition",
    ReturnStatement = "ReturnStatement",
    ReturnsDeclaration = "ReturnsDeclaration",
    RevertStatement = "RevertStatement",
    SourceUnit = "SourceUnit",
    SourceUnitMembers = "SourceUnitMembers",
    StateVariableAttributes = "StateVariableAttributes",
    StateVariableDefinition = "StateVariableDefinition",
    Statements = "Statements",
    StructDefinition = "StructDefinition",
    StructMember = "StructMember",
    StructMembers = "StructMembers",
    ThrowStatement = "ThrowStatement",
    TrailingTrivia = "TrailingTrivia",
    TryStatement = "TryStatement",
    TupleDeconstructionStatement = "TupleDeconstructionStatement",
    TupleExpression = "TupleExpression",
    TupleMember = "TupleMember",
    TupleMembersList = "TupleMembersList",
    TupleValues = "TupleValues",
    TypeExpression = "TypeExpression",
    TypeName = "TypeName",
    UncheckedBlock = "UncheckedBlock",
    UnicodeStringLiterals = "UnicodeStringLiterals",
    UnnamedFunctionAttributes = "UnnamedFunctionAttributes",
    UnnamedFunctionDefinition = "UnnamedFunctionDefinition",
    UserDefinedValueTypeDefinition = "UserDefinedValueTypeDefinition",
    UsingDirective = "UsingDirective",
    UsingDirectiveDeconstruction = "UsingDirectiveDeconstruction",
    UsingDirectivePath = "UsingDirectivePath",
    UsingDirectiveSymbol = "UsingDirectiveSymbol",
    UsingDirectiveSymbolsList = "UsingDirectiveSymbolsList",
    VariableDeclaration = "VariableDeclaration",
    VariableDeclarationStatement = "VariableDeclarationStatement",
    VersionPragma = "VersionPragma",
    VersionPragmaExpression = "VersionPragmaExpression",
    VersionPragmaExpressions = "VersionPragmaExpressions",
    VersionPragmaSpecifier = "VersionPragmaSpecifier",
    WhileStatement = "WhileStatement",
    YulAssignmentStatement = "YulAssignmentStatement",
    YulBlock = "YulBlock",
    YulBreakStatement = "YulBreakStatement",
    YulContinueStatement = "YulContinueStatement",
    YulDeclarationStatement = "YulDeclarationStatement",
    YulExpression = "YulExpression",
    YulExpressionsList = "YulExpressionsList",
    YulForStatement = "YulForStatement",
    YulFunctionDefinition = "YulFunctionDefinition",
    YulIdentifierPath = "YulIdentifierPath",
    YulIdentifierPaths = "YulIdentifierPaths",
    YulIdentifiersList = "YulIdentifiersList",
    YulIfStatement = "YulIfStatement",
    YulLeaveStatement = "YulLeaveStatement",
    YulParametersDeclaration = "YulParametersDeclaration",
    YulReturnsDeclaration = "YulReturnsDeclaration",
    YulStatements = "YulStatements",
    YulSwitchCase = "YulSwitchCase",
    YulSwitchCases = "YulSwitchCases",
    YulSwitchStatement = "YulSwitchStatement",
  }
  export enum RuleKind {
    ABICoderPragma = "ABICoderPragma",
    AddressType = "AddressType",
    ArgumentsDeclaration = "ArgumentsDeclaration",
    ArrayExpression = "ArrayExpression",
    ArrayTypeName = "ArrayTypeName",
    ArrayValues = "ArrayValues",
    AsciiStringLiterals = "AsciiStringLiterals",
    AssemblyFlags = "AssemblyFlags",
    AssemblyFlagsDeclaration = "AssemblyFlagsDeclaration",
    AssemblyStatement = "AssemblyStatement",
    BinaryExpression = "BinaryExpression",
    Block = "Block",
    BreakStatement = "BreakStatement",
    CatchClause = "CatchClause",
    CatchClauseError = "CatchClauseError",
    CatchClauses = "CatchClauses",
    ConditionalExpression = "ConditionalExpression",
    ConstantDefinition = "ConstantDefinition",
    ConstructorAttributes = "ConstructorAttributes",
    ConstructorDefinition = "ConstructorDefinition",
    ContinueStatement = "ContinueStatement",
    ContractDefinition = "ContractDefinition",
    ContractMembers = "ContractMembers",
    DecimalNumberExpression = "DecimalNumberExpression",
    DeconstructionImport = "DeconstructionImport",
    DeconstructionImportSymbol = "DeconstructionImportSymbol",
    DeconstructionImportSymbolsList = "DeconstructionImportSymbolsList",
    DeleteStatement = "DeleteStatement",
    DoWhileStatement = "DoWhileStatement",
    EmitStatement = "EmitStatement",
    EndOfFileTrivia = "EndOfFileTrivia",
    EnumDefinition = "EnumDefinition",
    EnumMembers = "EnumMembers",
    ErrorDefinition = "ErrorDefinition",
    ErrorParameter = "ErrorParameter",
    ErrorParameters = "ErrorParameters",
    ErrorParametersDeclaration = "ErrorParametersDeclaration",
    EventDefinition = "EventDefinition",
    EventParameter = "EventParameter",
    EventParameters = "EventParameters",
    EventParametersDeclaration = "EventParametersDeclaration",
    ExperimentalPragma = "ExperimentalPragma",
    Expression = "Expression",
    ExpressionStatement = "ExpressionStatement",
    FallbackFunctionAttributes = "FallbackFunctionAttributes",
    FallbackFunctionDefinition = "FallbackFunctionDefinition",
    ForStatement = "ForStatement",
    FunctionAttributes = "FunctionAttributes",
    FunctionCallExpression = "FunctionCallExpression",
    FunctionCallOptions = "FunctionCallOptions",
    FunctionDefinition = "FunctionDefinition",
    FunctionType = "FunctionType",
    FunctionTypeAttributes = "FunctionTypeAttributes",
    HexNumberExpression = "HexNumberExpression",
    HexStringLiterals = "HexStringLiterals",
    IdentifierPath = "IdentifierPath",
    IfStatement = "IfStatement",
    ImportDirective = "ImportDirective",
    IndexAccessExpression = "IndexAccessExpression",
    InheritanceSpecifier = "InheritanceSpecifier",
    InheritanceType = "InheritanceType",
    InheritanceTypes = "InheritanceTypes",
    InterfaceDefinition = "InterfaceDefinition",
    InterfaceMembers = "InterfaceMembers",
    LeadingTrivia = "LeadingTrivia",
    LibraryDefinition = "LibraryDefinition",
    LibraryMembers = "LibraryMembers",
    MappingKeyType = "MappingKeyType",
    MappingType = "MappingType",
    MappingValueType = "MappingValueType",
    MemberAccessExpression = "MemberAccessExpression",
    ModifierAttributes = "ModifierAttributes",
    ModifierDefinition = "ModifierDefinition",
    ModifierInvocation = "ModifierInvocation",
    NamedArgument = "NamedArgument",
    NamedArguments = "NamedArguments",
    NamedArgumentsDeclaration = "NamedArgumentsDeclaration",
    NamedImport = "NamedImport",
    NewExpression = "NewExpression",
    OverridePaths = "OverridePaths",
    OverrideSpecifier = "OverrideSpecifier",
    Parameter = "Parameter",
    Parameters = "Parameters",
    ParametersDeclaration = "ParametersDeclaration",
    PathImport = "PathImport",
    PositionalArguments = "PositionalArguments",
    PragmaDirective = "PragmaDirective",
    ReceiveFunctionAttributes = "ReceiveFunctionAttributes",
    ReceiveFunctionDefinition = "ReceiveFunctionDefinition",
    ReturnStatement = "ReturnStatement",
    ReturnsDeclaration = "ReturnsDeclaration",
    RevertStatement = "RevertStatement",
    SourceUnit = "SourceUnit",
    SourceUnitMembers = "SourceUnitMembers",
    StateVariableAttributes = "StateVariableAttributes",
    StateVariableDefinition = "StateVariableDefinition",
    Statements = "Statements",
    StructDefinition = "StructDefinition",
    StructMember = "StructMember",
    StructMembers = "StructMembers",
    ThrowStatement = "ThrowStatement",
    TrailingTrivia = "TrailingTrivia",
    TryStatement = "TryStatement",
    TupleDeconstructionStatement = "TupleDeconstructionStatement",
    TupleExpression = "TupleExpression",
    TupleMember = "TupleMember",
    TupleMembersList = "TupleMembersList",
    TupleValues = "TupleValues",
    TypeExpression = "TypeExpression",
    TypeName = "TypeName",
    UnaryPostfixExpression = "UnaryPostfixExpression",
    UnaryPrefixExpression = "UnaryPrefixExpression",
    UncheckedBlock = "UncheckedBlock",
    UnicodeStringLiterals = "UnicodeStringLiterals",
    UnnamedFunctionAttributes = "UnnamedFunctionAttributes",
    UnnamedFunctionDefinition = "UnnamedFunctionDefinition",
    UserDefinedValueTypeDefinition = "UserDefinedValueTypeDefinition",
    UsingDirective = "UsingDirective",
    UsingDirectiveDeconstruction = "UsingDirectiveDeconstruction",
    UsingDirectivePath = "UsingDirectivePath",
    UsingDirectiveSymbol = "UsingDirectiveSymbol",
    UsingDirectiveSymbolsList = "UsingDirectiveSymbolsList",
    VariableDeclaration = "VariableDeclaration",
    VariableDeclarationStatement = "VariableDeclarationStatement",
    VersionPragma = "VersionPragma",
    VersionPragmaBinaryExpression = "VersionPragmaBinaryExpression",
    VersionPragmaExpression = "VersionPragmaExpression",
    VersionPragmaExpressions = "VersionPragmaExpressions",
    VersionPragmaSpecifier = "VersionPragmaSpecifier",
    VersionPragmaUnaryExpression = "VersionPragmaUnaryExpression",
    WhileStatement = "WhileStatement",
    YulAssignmentStatement = "YulAssignmentStatement",
    YulBlock = "YulBlock",
    YulBreakStatement = "YulBreakStatement",
    YulContinueStatement = "YulContinueStatement",
    YulDeclarationStatement = "YulDeclarationStatement",
    YulExpression = "YulExpression",
    YulExpressionsList = "YulExpressionsList",
    YulForStatement = "YulForStatement",
    YulFunctionCallExpression = "YulFunctionCallExpression",
    YulFunctionDefinition = "YulFunctionDefinition",
    YulIdentifierPath = "YulIdentifierPath",
    YulIdentifierPaths = "YulIdentifierPaths",
    YulIdentifiersList = "YulIdentifiersList",
    YulIfStatement = "YulIfStatement",
    YulLeaveStatement = "YulLeaveStatement",
    YulParametersDeclaration = "YulParametersDeclaration",
    YulReturnsDeclaration = "YulReturnsDeclaration",
    YulStatements = "YulStatements",
    YulSwitchCase = "YulSwitchCase",
    YulSwitchCases = "YulSwitchCases",
    YulSwitchStatement = "YulSwitchStatement",
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
    AsciiStringLiteral = "AsciiStringLiteral",
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
    CalldataKeyword = "CalldataKeyword",
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
    CopyofKeyword = "CopyofKeyword",
    DaysKeyword = "DaysKeyword",
    DecimalLiteral = "DecimalLiteral",
    DefaultKeyword = "DefaultKeyword",
    DefineKeyword = "DefineKeyword",
    DeleteKeyword = "DeleteKeyword",
    DoKeyword = "DoKeyword",
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
    FixedBytesType = "FixedBytesType",
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
    HexStringLiteral = "HexStringLiteral",
    HoursKeyword = "HoursKeyword",
    Identifier = "Identifier",
    IfKeyword = "IfKeyword",
    ImmutableKeyword = "ImmutableKeyword",
    ImplementsKeyword = "ImplementsKeyword",
    ImportKeyword = "ImportKeyword",
    InKeyword = "InKeyword",
    IndexedKeyword = "IndexedKeyword",
    InlineKeyword = "InlineKeyword",
    InterfaceKeyword = "InterfaceKeyword",
    InternalKeyword = "InternalKeyword",
    IsKeyword = "IsKeyword",
    LeaveKeyword = "LeaveKeyword",
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
    MultilineComment = "MultilineComment",
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
    SignedFixedType = "SignedFixedType",
    SignedIntegerType = "SignedIntegerType",
    SingleLineComment = "SingleLineComment",
    SizeofKeyword = "SizeofKeyword",
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
    TypeKeyword = "TypeKeyword",
    TypedefKeyword = "TypedefKeyword",
    TypeofKeyword = "TypeofKeyword",
    UncheckedKeyword = "UncheckedKeyword",
    UnicodeStringLiteral = "UnicodeStringLiteral",
    UnsignedFixedType = "UnsignedFixedType",
    UnsignedIntegerType = "UnsignedIntegerType",
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
    YulDecimalLiteral = "YulDecimalLiteral",
    YulHexLiteral = "YulHexLiteral",
    YulIdentifier = "YulIdentifier",
  }
}
export namespace language {
  /** The lexical context of the scanner. */
  export enum LexicalContext {
    Default = "Default",
    VersionPragma = "VersionPragma",
    YulBlock = "YulBlock",
  }
  export class Language {
    constructor(version: string);
    get version(): string;
    static supportedVersions(): Array<string>;
    scan(lexicalContext: LexicalContext, input: string): kinds.TokenKind | null;
    parse(productionKind: kinds.ProductionKind, input: string): parse_output.ParseOutput;
  }
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
    children(): Array<cst.RuleNode | cst.TokenNode>;
    createCursor(textOffset: TextIndex): cursor.Cursor;
  }
  export class TokenNode {
    get type(): NodeType.Token;
    get kind(): kinds.TokenKind;
    get textLength(): text_index.TextIndex;
    get text(): string;
    createCursor(textOffset: TextIndex): cursor.Cursor;
  }
}
export namespace cursor {
  export class Cursor {
    reset(): void;
    complete(): void;
    clone(): Cursor;
    spawn(): Cursor;
    get isCompleted(): boolean;
    node(): cst.RuleNode | cst.TokenNode;
    get textOffset(): text_index.TextIndex;
    get textRange(): text_index.TextRange;
    pathRuleNodes(): Array<cst.RuleNode>;
    goToNext(): boolean;
    goToNextNonDescendent(): boolean;
    goToPrevious(): boolean;
    goToParent(): boolean;
    goToFirstChild(): boolean;
    goToLastChild(): boolean;
    goToNthChild(childNumber: number): boolean;
    goToNextSibling(): boolean;
    goToPreviousSibling(): boolean;
    findTokenWithKind(kinds: Array<kinds.TokenKind>): cst.TokenNode | null;
    findRuleWithKind(kinds: Array<kinds.RuleKind>): cst.RuleNode | null;
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
    tree(): cst.RuleNode | cst.TokenNode;
    errors(): Array<parse_error.ParseError>;
    get isValid(): boolean;
    /** Creates a cursor that starts at the root of the parse tree. */
    createTreeCursor(): cursor.Cursor;
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
