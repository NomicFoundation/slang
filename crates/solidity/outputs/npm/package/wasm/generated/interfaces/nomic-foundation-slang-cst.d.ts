// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangCst {
  export { TerminalKindExtensions };
  export { NonterminalNode };
  export { TerminalNode };
  export { Cursor };
  export { Query };
  export { QueryMatchIterator };
  export { NonterminalKind };
  export { TerminalKind };
  export { EdgeLabel };
  export { Node };
  export { NodeVariant };
}
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
  YulPathComponent = "YulPathComponent",
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
export declare enum TerminalKind {
  Unrecognized = "Unrecognized",
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
  SupportsKeyword = "SupportsKeyword",
  SwitchKeyword = "SwitchKeyword",
  SzaboKeyword = "SzaboKeyword",
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
  YulSupportsKeyword = "YulSupportsKeyword",
  YulSwitchKeyword = "YulSwitchKeyword",
  YulSzaboKeyword = "YulSzaboKeyword",
  YulTloadKeyword = "YulTloadKeyword",
  YulTstoreKeyword = "YulTstoreKeyword",
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
export type Node = NonterminalNode | TerminalNode;
export enum NodeVariant {
  NonterminalNode = "NonterminalNode",
  TerminalNode = "TerminalNode",
}
export interface Edge {
  label?: EdgeLabel;
  node: Node;
}
export interface QueryError {
  message: string;
  line: number;
  column: number;
}
export interface QueryMatch {
  queryNumber: number;
  captures: { [key: string]: Cursor[] };
}
export interface TextIndex {
  utf8: number;
  utf16: number;
  line: number;
  column: number;
}
export interface TextRange {
  start: TextIndex;
  end: TextIndex;
}

export class Cursor {
  reset(): void;
  complete(): void;
  isCompleted(): boolean;
  clone(): Cursor;
  spawn(): Cursor;
  get node(): Node;
  get label(): EdgeLabel | undefined;
  get textOffset(): TextIndex;
  get textRange(): TextRange;
  get depth(): number;
  get ancestors(): NonterminalNode[];
  goToNext(): boolean;
  goToNextNonDescendent(): boolean;
  goToPrevious(): boolean;
  goToParent(): boolean;
  goToFirstChild(): boolean;
  goToLastChild(): boolean;
  goToNthChild(childNumber: number): boolean;
  goToNextSibling(): boolean;
  goToPreviousSibling(): boolean;
  goToNextTerminal(): boolean;
  goToNextTerminalWithKind(kind: TerminalKind): boolean;
  goToNextTerminalWithKinds(kinds: TerminalKind[]): boolean;
  goToNextNonterminal(): boolean;
  goToNextNonterminalWithKind(kind: NonterminalKind): boolean;
  goToNextNonterminalWithKinds(kinds: NonterminalKind[]): boolean;
  query(queries: Query[]): QueryMatchIterator;
}

export class NonterminalNode {
  readonly nodeVariant = NodeVariant.NonterminalNode;

  asNonterminalNode(): this;
  isNonterminalNode(): this is NonterminalNode;

  asTerminalNode(): undefined;
  isTerminalNode(): false;

  get id(): number;
  get kind(): NonterminalKind;
  get textLength(): TextIndex;
  get children(): Edge[];
  unparse(): string;
  toJson(): string;
  createCursor(textOffset: TextIndex): Cursor;
}

export class Query {
  static parse(text: string): Query;
}

export class QueryMatchIterator {
  [Symbol.iterator](): Iterator<QueryMatch>;
  next(): QueryMatch | undefined;
}

export class TerminalKindExtensions {
  static isTrivia(kind: TerminalKind): boolean;
  static isValid(kind: TerminalKind): boolean;
}

export class TerminalNode {
  readonly nodeVariant = NodeVariant.TerminalNode;

  asTerminalNode(): this;
  isTerminalNode(): this is TerminalNode;

  asNonterminalNode(): undefined;
  isNonterminalNode(): false;

  get id(): number;
  get kind(): TerminalKind;
  get textLength(): TextIndex;
  get children(): Edge[];
  unparse(): string;
  toJson(): string;
}
