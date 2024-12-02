// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangCst {
  export { TerminalKindExtensions };
  export { NonterminalNode };
  export { TerminalNode };
  export { Cursor };
  export { CursorIterator };
  export { AncestorsIterator };
  export { Query };
  export { QueryMatchIterator };
  export { NonterminalKind };
  export { TerminalKind };
  export { EdgeLabel };
  export { Node };
  export { NodeVariant };
}
/**
 * Represents different kinds of nonterminal nodes in the syntax tree.
 * These are nodes that can have child nodes and represent higher-level language constructs.
 */
export declare enum NonterminalKind {
  AbicoderPragma = "AbicoderPragma",
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
  SimpleVersionLiteral = "SimpleVersionLiteral",
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
  VersionExpression = "VersionExpression",
  VersionExpressionSet = "VersionExpressionSet",
  VersionExpressionSets = "VersionExpressionSets",
  VersionLiteral = "VersionLiteral",
  VersionOperator = "VersionOperator",
  VersionPragma = "VersionPragma",
  VersionRange = "VersionRange",
  VersionTerm = "VersionTerm",
  WhileStatement = "WhileStatement",
  YulArguments = "YulArguments",
  YulAssignmentOperator = "YulAssignmentOperator",
  YulBlock = "YulBlock",
  YulBreakStatement = "YulBreakStatement",
  YulBuiltInFunction = "YulBuiltInFunction",
  YulColonAndEqual = "YulColonAndEqual",
  YulContinueStatement = "YulContinueStatement",
  YulDefaultCase = "YulDefaultCase",
  YulEqualAndColon = "YulEqualAndColon",
  YulExpression = "YulExpression",
  YulForStatement = "YulForStatement",
  YulFunctionCallExpression = "YulFunctionCallExpression",
  YulFunctionDefinition = "YulFunctionDefinition",
  YulIfStatement = "YulIfStatement",
  YulLabel = "YulLabel",
  YulLeaveStatement = "YulLeaveStatement",
  YulLiteral = "YulLiteral",
  YulParameters = "YulParameters",
  YulParametersDeclaration = "YulParametersDeclaration",
  YulPath = "YulPath",
  YulPaths = "YulPaths",
  YulReturnsDeclaration = "YulReturnsDeclaration",
  YulStackAssignmentOperator = "YulStackAssignmentOperator",
  YulStackAssignmentStatement = "YulStackAssignmentStatement",
  YulStatement = "YulStatement",
  YulStatements = "YulStatements",
  YulSwitchCase = "YulSwitchCase",
  YulSwitchCases = "YulSwitchCases",
  YulSwitchStatement = "YulSwitchStatement",
  YulValueCase = "YulValueCase",
  YulVariableAssignmentStatement = "YulVariableAssignmentStatement",
  YulVariableDeclarationStatement = "YulVariableDeclarationStatement",
  YulVariableDeclarationValue = "YulVariableDeclarationValue",
  YulVariableNames = "YulVariableNames",
}
/**
 * Represents different kinds of terminal nodes in the syntax tree.
 * These are leaf nodes that represent actual tokens in the source code.
 */
export declare enum TerminalKind {
  /**
   * This terminal is created when the parser encounters an unexpected part of the input,
   * and it cannot recognize it as any valid syntax in this position in the grammar.
   */
  Unrecognized = "Unrecognized",
  /**
   * This terminal is created when the parser is expecting a certain terminal, but it cannot find it.
   * Adding the missing input in this position may allow the parser to produce a valid tree there.
   */
  Missing = "Missing",
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
  DoubleQuotedVersionLiteral = "DoubleQuotedVersionLiteral",
  ElseKeyword = "ElseKeyword",
  EmitKeyword = "EmitKeyword",
  EndOfLine = "EndOfLine",
  EnumKeyword = "EnumKeyword",
  Equal = "Equal",
  EqualColon = "EqualColon",
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
  SingleQuotedVersionLiteral = "SingleQuotedVersionLiteral",
  SizeOfKeyword = "SizeOfKeyword",
  Slash = "Slash",
  SlashEqual = "SlashEqual",
  SolidityKeyword = "SolidityKeyword",
  StaticKeyword = "StaticKeyword",
  StorageKeyword = "StorageKeyword",
  StringKeyword = "StringKeyword",
  StructKeyword = "StructKeyword",
  SuperKeyword = "SuperKeyword",
  SupportsKeyword = "SupportsKeyword",
  SwitchKeyword = "SwitchKeyword",
  SzaboKeyword = "SzaboKeyword",
  ThisKeyword = "ThisKeyword",
  ThrowKeyword = "ThrowKeyword",
  Tilde = "Tilde",
  TransientKeyword = "TransientKeyword",
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
  VersionSpecifier = "VersionSpecifier",
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
  YulBlobBaseFeeKeyword = "YulBlobBaseFeeKeyword",
  YulBlobHashKeyword = "YulBlobHashKeyword",
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
  YulJumpKeyword = "YulJumpKeyword",
  YulJumpiKeyword = "YulJumpiKeyword",
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
  YulMcopyKeyword = "YulMcopyKeyword",
  YulMloadKeyword = "YulMloadKeyword",
  YulMsizeKeyword = "YulMsizeKeyword",
  YulMstore8Keyword = "YulMstore8Keyword",
  YulMstoreKeyword = "YulMstoreKeyword",
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
  YulSdivKeyword = "YulSdivKeyword",
  YulSloadKeyword = "YulSloadKeyword",
  YulSmodKeyword = "YulSmodKeyword",
  YulSstoreKeyword = "YulSstoreKeyword",
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
  YulSuperKeyword = "YulSuperKeyword",
  YulSupportsKeyword = "YulSupportsKeyword",
  YulSwitchKeyword = "YulSwitchKeyword",
  YulSzaboKeyword = "YulSzaboKeyword",
  YulTloadKeyword = "YulTloadKeyword",
  YulTstoreKeyword = "YulTstoreKeyword",
  YulThisKeyword = "YulThisKeyword",
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
/**
 * Represents the different types of relationships between nodes in the syntax tree.
 */
export declare enum EdgeLabel {
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
  Inheritance = "Inheritance",
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
  Minus = "Minus",
  MinusGreaterThan = "MinusGreaterThan",
  ModifierKeyword = "ModifierKeyword",
  Name = "Name",
  NewKeyword = "NewKeyword",
  OpenBrace = "OpenBrace",
  OpenBracket = "OpenBracket",
  OpenParen = "OpenParen",
  Operator = "Operator",
  Options = "Options",
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
  Sets = "Sets",
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
  Variable = "Variable",
  VariableType = "VariableType",
  Variables = "Variables",
  Version = "Version",
  WhileKeyword = "WhileKeyword",
}
/**
 * The super type of all nodes in a tree.
 */
export type Node = NonterminalNode | TerminalNode;
export enum NodeVariant {
  NonterminalNode = "NonterminalNode",
  TerminalNode = "TerminalNode",
}
/**
 * Represents a connection between nodes in the syntax tree.
 */
export interface Edge {
  /**
   * Optional label describing the relationship between nodes.
   */
  label?: EdgeLabel;
  /**
   * The target node of this edge.
   */
  node: Node;
}
/**
 * Represents an error that occurred while parsing a query.
 */
export interface QueryError {
  /**
   * The error message describing what went wrong.
   */
  message: string;
  /**
   * The line number where the error occurred.
   */
  line: number;
  /**
   * The column number where the error occurred.
   */
  column: number;
}
/**
 * Represents a match found by executing a query.
 */
export interface QueryMatch {
  /**
   * The index of the query that produced this match.
   */
  queryNumber: number;
  /**
   * List of captured nodes and their names from the query.
   */
  captures: { [key: string]: Cursor[] };
}
/**
 * Represents a position in the source text, with indices for different unicode encodings of the source.
 */
export interface TextIndex {
  /**
   * Byte offset in UTF-8 encoding.
   * This is useful when working with languages like Rust that use UTF-8.
   */
  utf8: number;
  /**
   * Byte offset in UTF-8 encoding.
   * This is useful when working with languages like JavaScript that use UTF-16.
   */
  utf16: number;
  /**
   * Line number (0-based).
   * Lines are separated by:
   *
   * - carriage return `\r`.
   * - newline `\n`.
   * - line separator `\u2028`.
   * - paragraph separator `\u2029`.
   */
  line: number;
  /**
   * Column number (0-based).
   * Columns are counted in [unicode scalar values](https://www.unicode.org/glossary/#unicode_scalar_value).
   */
  column: number;
}
/**
 * Represents a range in the source text.
 */
export interface TextRange {
  /**
   * Starting (inclusive) position of the range.
   */
  start: TextIndex;
  /**
   * Ending (exclusive) position of the range.
   */
  end: TextIndex;
}

export class AncestorsIterator {
  [Symbol.iterator](): Iterator<NonterminalNode>;
  /**
   * Returns the next nonterminal node in the iteration, or `undefined` if there are no more nodes.
   */
  next(): NonterminalNode | undefined;
}

export class Cursor {
  /**
   * Resets the cursor to its initial position.
   */
  reset(): void;
  /**
   * Marks the cursor as completed.
   */
  complete(): void;
  /**
   * Returns whether the cursor has completed its traversal.
   */
  isCompleted(): boolean;
  /**
   * Creates a copy of this cursor at its current position with the same ancestors.
   * The new cursor can be moved independently without affecting the original cursor.
   */
  clone(): Cursor;
  /**
   * Creates a copy of this cursor at its current position, but without any ancestors.
   * This is useful for limiting the scope of a search or query to the sub-tree only, without backtracking to parent nodes.
   * The new cursor can be moved independently without affecting the original cursor.
   */
  spawn(): Cursor;
  /**
   * Returns the current node under the cursor.
   */
  get node(): Node;
  /**
   * Returns the label of the edge from the parent to the current node, if any.
   */
  get label(): EdgeLabel | undefined;
  /**
   * Returns the current text offset of the cursor.
   */
  get textOffset(): TextIndex;
  /**
   * Returns the text range covered by the current node.
   */
  get textRange(): TextRange;
  /**
   * Returns the current depth in the tree (i.e. number of ancestors).
   */
  get depth(): number;
  /**
   * Returns the list of child edges directly connected to this node.
   */
  children(): Edge[];
  /**
   * Returns an iterator over all descendants of the current node in pre-order traversal.
   */
  descendants(): CursorIterator;
  /**
   * Returns an iterator over all the remaining nodes in the current tree, moving in pre-order traversal, until the tree is completed.
   */
  remainingNodes(): CursorIterator;
  /**
   * Returns an iterator over all ancestors of the current node, starting with the immediate parent, and moving upwards, ending with the root node.
   */
  ancestors(): AncestorsIterator;
  /**
   * Moves to the next node in pre-order traversal.
   */
  goToNext(): boolean;
  /**
   * Moves to the next node that isn't a descendant of the current node.
   */
  goToNextNonDescendant(): boolean;
  /**
   * Moves to the previous node in pre-order traversal.
   */
  goToPrevious(): boolean;
  /**
   * Moves up to the parent node.
   */
  goToParent(): boolean;
  /**
   * Moves to the first child of the current node.
   */
  goToFirstChild(): boolean;
  /**
   * Moves to the last child of the current node.
   */
  goToLastChild(): boolean;
  /**
   * Moves to the nth child of the current node.
   */
  goToNthChild(childNumber: number): boolean;
  /**
   * Moves to the next sibling node.
   */
  goToNextSibling(): boolean;
  /**
   * Moves to the previous sibling node.
   */
  goToPreviousSibling(): boolean;
  /**
   * Moves to the next terminal node.
   */
  goToNextTerminal(): boolean;
  /**
   * Moves to the next terminal node of a specific kind.
   */
  goToNextTerminalWithKind(kind: TerminalKind): boolean;
  /**
   * Moves to the next terminal node matching any of the given kinds.
   */
  goToNextTerminalWithKinds(kinds: TerminalKind[]): boolean;
  /**
   * Nonterminal navigation methods
   * Moves to the next nonterminal node.
   */
  goToNextNonterminal(): boolean;
  /**
   * Moves to the next nonterminal node of a specific kind.
   */
  goToNextNonterminalWithKind(kind: NonterminalKind): boolean;
  /**
   * Moves to the next nonterminal node matching any of the given kinds.
   */
  goToNextNonterminalWithKinds(kinds: NonterminalKind[]): boolean;
  /**
   * Executes the given queries and returns an iterator over the matches.
   */
  query(queries: Query[]): QueryMatchIterator;
}

export class CursorIterator {
  [Symbol.iterator](): Iterator<Edge>;
  /**
   * Returns the next edge in the iteration, or `undefined` if there are no more edges.
   */
  next(): Edge | undefined;
}

export class NonterminalNode {
  readonly nodeVariant = NodeVariant.NonterminalNode;

  asNonterminalNode(): this;
  isNonterminalNode(): this is NonterminalNode;

  asTerminalNode(): undefined;
  isTerminalNode(): false;

  /**
   * Returns a unique numerical identifier of the node.
   * It is only valid for the lifetime of the enclosing tree.
   * It can change between multiple parses, even for the same source code input.
   */
  get id(): number;
  /**
   * Returns the kind enum of this nonterminal node.
   */
  get kind(): NonterminalKind;
  /**
   * Returns the length of the text span this node covers.
   */
  get textLength(): TextIndex;
  /**
   * Returns the list of child edges directly connected to this node.
   */
  children(): Edge[];
  /**
   * Returns an iterator over all descendants of the current node in pre-order traversal.
   */
  descendants(): CursorIterator;
  /**
   * Converts the node and its children back to source code text.
   */
  unparse(): string;
  /**
   * Converts the node to a JSON representation for debugging.
   */
  toJson(): string;
  /**
   * Creates a cursor positioned at the given text offset within this node.
   */
  createCursor(textOffset: TextIndex): Cursor;
}

export class Query {
  /**
   * Parses a query string into a query object.
   * Throws an error if the query syntax is invalid.
   */
  static parse(text: string): Query;
}

export class QueryMatchIterator {
  [Symbol.iterator](): Iterator<QueryMatch>;
  /**
   * Returns the next match or `undefined` if there are no more matches.
   */
  next(): QueryMatch | undefined;
}

export class TerminalKindExtensions {
  /**
   * Returns true if the terminal is a trivia token. i.e. whitespace, comments, etc...
   */
  static isTrivia(kind: TerminalKind): boolean;
  /**
   * Returns true if the terminal is a valid token in the language grammar.
   */
  static isValid(kind: TerminalKind): boolean;
}

export class TerminalNode {
  readonly nodeVariant = NodeVariant.TerminalNode;

  asTerminalNode(): this;
  isTerminalNode(): this is TerminalNode;

  asNonterminalNode(): undefined;
  isNonterminalNode(): false;

  /**
   * Returns a unique numerical identifier of the node.
   * It is only valid for the lifetime of the enclosing tree.
   * It can change between multiple parses, even for the same source code input.
   */
  get id(): number;
  /**
   * Returns the kind enum of this terminal node.
   */
  get kind(): TerminalKind;
  /**
   * Returns the length of the text span this node covers.
   */
  get textLength(): TextIndex;
  /**
   * Returns the list of child edges directly connected to this node.
   */
  children(): Edge[];
  /**
   * Returns an iterator over all descendants of this node in pre-order traversal.
   */
  descendants(): CursorIterator;
  /**
   * Converts the node back to source code text.
   */
  unparse(): string;
  /**
   * Converts the node to a JSON representation for debugging.
   */
  toJson(): string;
}
