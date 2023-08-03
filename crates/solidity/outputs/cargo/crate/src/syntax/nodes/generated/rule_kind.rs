// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.
#![allow(clippy::all)]

#[cfg(feature = "slang_napi_interfaces")]
use napi::bindgen_prelude::*;

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr(
    // If feature is enabled, derive the NAPI version.
    // This also derives `Clone` and `Copy` automatically.
    feature = "slang_napi_interfaces",
    napi(string_enum, namespace = "syntax$nodes")
)]
#[cfg_attr(
    // If feature is not enabled, derive `Clone` and `Copy` manually.
    not(feature = "slang_napi_interfaces"),
    derive(Clone, Copy),
)]
pub enum RuleKind {
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ABICoderPragma = ABICODER_KEYWORD IDENTIFIER;
    /// ```
    ABICoderPragma,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AddressType = (ADDRESS_KEYWORD PAYABLE_KEYWORD?) | PAYABLE_KEYWORD;
    /// ```
    AddressType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ArgumentsDeclaration = OPEN_PAREN (PositionalArgumentsList | NamedArgumentsDeclaration)? CLOSE_PAREN;
    /// ```
    ArgumentsDeclaration,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ArrayExpression = OPEN_BRACKET ArrayValuesList CLOSE_BRACKET;
    /// ```
    ArrayExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ArrayTypeName = TypeName «ArrayTypeNameOperator» (* Unary Operator, Postfix *);
    /// ```
    ArrayTypeName,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ArrayValuesList = Expression (COMMA Expression)*;
    /// ```
    ArrayValuesList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AsciiStringLiteralsList = ASCII_STRING_LITERAL+;
    /// ```
    AsciiStringLiteralsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AssemblyFlagsList = ASCII_STRING_LITERAL (COMMA ASCII_STRING_LITERAL)*;
    /// ```
    AssemblyFlagsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AssemblyStatement = ASSEMBLY_KEYWORD ASCII_STRING_LITERAL? (OPEN_PAREN AssemblyFlagsList CLOSE_PAREN)? YulBlock;
    /// ```
    AssemblyStatement,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// BinaryExpression = Expression «AssignmentOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «OrOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «AndOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «EqualityComparisonOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «OrderComparisonOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «BitwiseOrOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «BitwiseXOrOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «BitwiseAndOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «ShiftOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «AddSubOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «MulDivModOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «ExponentiationOperator» Expression (* Binary Operator, Left Associative *);
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// BinaryExpression = Expression «AssignmentOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «OrOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «AndOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «EqualityComparisonOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «OrderComparisonOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «BitwiseOrOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «BitwiseXOrOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «BitwiseAndOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «ShiftOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «AddSubOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «MulDivModOperator» Expression (* Binary Operator, Left Associative *);
    /// BinaryExpression = Expression «ExponentiationOperator» Expression (* Binary Operator, Right Associative *);
    /// ```
    BinaryExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// Block = OPEN_BRACE StatementsList? CLOSE_BRACE;
    /// ```
    Block,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BreakStatement = BREAK_KEYWORD SEMICOLON;
    /// ```
    BreakStatement,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// CatchClause = CATCH_KEYWORD CatchClauseError? Block;
    /// ```
    CatchClause,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// CatchClauseError = IDENTIFIER? ParametersDeclaration;
    /// ```
    CatchClauseError,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// CatchClausesList = CatchClause+;
    /// ```
    CatchClausesList,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// ConditionalExpression = Expression «ConditionalOperator» (* Unary Operator, Postfix *);
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// ConditionalExpression = Expression «ConditionalOperator» (* Unary Operator, Postfix *);
    /// ```
    ConditionalExpression,
    /// ## v0.7.4
    ///
    /// ```ebnf
    /// ConstantDefinition = TypeName CONSTANT_KEYWORD IDENTIFIER EQUAL Expression SEMICOLON;
    /// ```
    ConstantDefinition,
    /// ## v0.4.22
    ///
    /// ```ebnf
    /// ConstructorAttributesList = «ConstructorAttribute»+;
    /// ```
    ConstructorAttributesList,
    /// ## v0.4.22
    ///
    /// ```ebnf
    /// ConstructorDefinition = CONSTRUCTOR_KEYWORD ParametersDeclaration ConstructorAttributesList? Block;
    /// ```
    ConstructorDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ContinueStatement = CONTINUE_KEYWORD SEMICOLON;
    /// ```
    ContinueStatement,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// ContractDefinition = CONTRACT_KEYWORD IDENTIFIER InheritanceSpecifier? OPEN_BRACE ContractMembersList? CLOSE_BRACE;
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// ContractDefinition = ABSTRACT_KEYWORD? CONTRACT_KEYWORD IDENTIFIER InheritanceSpecifier? OPEN_BRACE ContractMembersList? CLOSE_BRACE;
    /// ```
    ContractDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ContractMembersList = «ContractMember»+;
    /// ```
    ContractMembersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DeconstructionImport = OPEN_BRACE DeconstructionImportSymbolsList CLOSE_BRACE FROM_KEYWORD ASCII_STRING_LITERAL;
    /// ```
    DeconstructionImport,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DeconstructionImportSymbol = IDENTIFIER (AS_KEYWORD IDENTIFIER)?;
    /// ```
    DeconstructionImportSymbol,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DeconstructionImportSymbolsList = DeconstructionImportSymbol (COMMA DeconstructionImportSymbol)*;
    /// ```
    DeconstructionImportSymbolsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DeleteStatement = DELETE_KEYWORD Expression SEMICOLON;
    /// ```
    DeleteStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DoWhileStatement = DO_KEYWORD Statement WHILE_KEYWORD OPEN_PAREN Expression CLOSE_PAREN SEMICOLON;
    /// ```
    DoWhileStatement,
    /// ## v0.4.21
    ///
    /// ```ebnf
    /// EmitStatement = EMIT_KEYWORD IdentifierPath ArgumentsDeclaration SEMICOLON;
    /// ```
    EmitStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EndOfFileTrivia = (WHITESPACE | END_OF_LINE | MULTILINE_COMMENT | SINGLE_LINE_COMMENT)+;
    /// ```
    EndOfFileTrivia,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EnumDefinition = ENUM_KEYWORD IDENTIFIER OPEN_BRACE IdentifiersList? CLOSE_BRACE;
    /// ```
    EnumDefinition,
    /// ## v0.8.4
    ///
    /// ```ebnf
    /// ErrorDefinition = ERROR_KEYWORD IDENTIFIER OPEN_PAREN ErrorParametersList? CLOSE_PAREN SEMICOLON;
    /// ```
    ErrorDefinition,
    /// ## v0.8.4
    ///
    /// ```ebnf
    /// ErrorParameter = TypeName IDENTIFIER?;
    /// ```
    ErrorParameter,
    /// ## v0.8.4
    ///
    /// ```ebnf
    /// ErrorParametersList = ErrorParameter (COMMA ErrorParameter)*;
    /// ```
    ErrorParametersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EventDefinition = EVENT_KEYWORD IDENTIFIER OPEN_PAREN EventParametersList? CLOSE_PAREN ANONYMOUS_KEYWORD? SEMICOLON;
    /// ```
    EventDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EventParameter = TypeName INDEXED_KEYWORD? IDENTIFIER?;
    /// ```
    EventParameter,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EventParametersList = EventParameter (COMMA EventParameter)*;
    /// ```
    EventParametersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ExperimentalPragma = EXPERIMENTAL_KEYWORD (ASCII_STRING_LITERAL | IDENTIFIER);
    /// ```
    ExperimentalPragma,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// Expression = BinaryExpression (* Expression «AssignmentOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | ConditionalExpression (* Expression «ConditionalOperator» *) (* Unary Operator, Postfix *)
    ///            | BinaryExpression (* Expression «OrOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «AndOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «EqualityComparisonOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «OrderComparisonOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «BitwiseOrOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «BitwiseXOrOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «BitwiseAndOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «ShiftOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «AddSubOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «MulDivModOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «ExponentiationOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | UnaryPostfixExpression (* Expression «UnaryPostfixOperator» *) (* Unary Operator, Postfix *)
    ///            | UnaryPrefixExpression (* «UnaryPrefixOperator» Expression *) (* Unary Operator, Prefix *)
    ///            | FunctionCallExpression (* Expression «FunctionCallOperator» *) (* Unary Operator, Postfix *)
    ///            | MemberAccessExpression (* Expression «MemberAccessOperator» *) (* Unary Operator, Postfix *)
    ///            | IndexAccessExpression (* Expression «IndexAccessOperator» *) (* Unary Operator, Postfix *)
    ///            | «PrimaryExpression»;
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// Expression = BinaryExpression (* Expression «AssignmentOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | ConditionalExpression (* Expression «ConditionalOperator» *) (* Unary Operator, Postfix *)
    ///            | BinaryExpression (* Expression «OrOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «AndOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «EqualityComparisonOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «OrderComparisonOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «BitwiseOrOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «BitwiseXOrOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «BitwiseAndOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «ShiftOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «AddSubOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «MulDivModOperator» Expression *) (* Binary Operator, Left Associative *)
    ///            | BinaryExpression (* Expression «ExponentiationOperator» Expression *) (* Binary Operator, Right Associative *)
    ///            | UnaryPostfixExpression (* Expression «UnaryPostfixOperator» *) (* Unary Operator, Postfix *)
    ///            | UnaryPrefixExpression (* «UnaryPrefixOperator» Expression *) (* Unary Operator, Prefix *)
    ///            | FunctionCallExpression (* Expression «FunctionCallOperator» *) (* Unary Operator, Postfix *)
    ///            | MemberAccessExpression (* Expression «MemberAccessOperator» *) (* Unary Operator, Postfix *)
    ///            | IndexAccessExpression (* Expression «IndexAccessOperator» *) (* Unary Operator, Postfix *)
    ///            | «PrimaryExpression»;
    /// ```
    Expression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ExpressionStatement = Expression SEMICOLON;
    /// ```
    ExpressionStatement,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// FallbackFunctionAttributesList = «FallbackFunctionAttribute»+;
    /// ```
    FallbackFunctionAttributesList,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// FallbackFunctionDefinition = FALLBACK_KEYWORD ParametersDeclaration FallbackFunctionAttributesList? ReturnsDeclaration? (SEMICOLON | Block);
    /// ```
    FallbackFunctionDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ForStatement = FOR_KEYWORD OPEN_PAREN («SimpleStatement» | SEMICOLON) (ExpressionStatement | SEMICOLON) Expression? CLOSE_PAREN Statement;
    /// ```
    ForStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FunctionAttributesList = «FunctionAttribute»+;
    /// ```
    FunctionAttributesList,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// FunctionCallExpression = Expression «FunctionCallOperator» (* Unary Operator, Postfix *);
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// FunctionCallExpression = Expression «FunctionCallOperator» (* Unary Operator, Postfix *);
    /// ```
    FunctionCallExpression,
    /// ## v0.6.2
    ///
    /// ```ebnf
    /// FunctionCallOptions = NamedArgumentsDeclaration+;
    /// ```
    ///
    /// ## v0.8.0
    ///
    /// ```ebnf
    /// FunctionCallOptions = NamedArgumentsDeclaration;
    /// ```
    FunctionCallOptions,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FunctionDefinition = FUNCTION_KEYWORD (IDENTIFIER | FALLBACK_KEYWORD | RECEIVE_KEYWORD) ParametersDeclaration FunctionAttributesList? ReturnsDeclaration? (SEMICOLON | Block);
    /// ```
    FunctionDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FunctionType = FUNCTION_KEYWORD ParametersDeclaration FunctionTypeAttributesList? ReturnsDeclaration?;
    /// ```
    FunctionType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FunctionTypeAttributesList = «FunctionTypeAttribute»+;
    /// ```
    FunctionTypeAttributesList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// HexStringLiteralsList = HEX_STRING_LITERAL+;
    /// ```
    HexStringLiteralsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IdentifierPath = IDENTIFIER (PERIOD IDENTIFIER)*;
    /// ```
    IdentifierPath,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IdentifierPathsList = IdentifierPath (COMMA IdentifierPath)*;
    /// ```
    IdentifierPathsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IdentifiersList = IDENTIFIER (COMMA IDENTIFIER)*;
    /// ```
    IdentifiersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IfStatement = IF_KEYWORD OPEN_PAREN Expression CLOSE_PAREN Statement (ELSE_KEYWORD Statement)?;
    /// ```
    IfStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ImportDirective = IMPORT_KEYWORD (PathImport | NamedImport | DeconstructionImport) SEMICOLON;
    /// ```
    ImportDirective,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// IndexAccessExpression = Expression «IndexAccessOperator» (* Unary Operator, Postfix *);
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// IndexAccessExpression = Expression «IndexAccessOperator» (* Unary Operator, Postfix *);
    /// ```
    IndexAccessExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// InheritanceSpecifier = IS_KEYWORD InheritanceTypesList;
    /// ```
    InheritanceSpecifier,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// InheritanceType = IdentifierPath ArgumentsDeclaration?;
    /// ```
    InheritanceType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// InheritanceTypesList = InheritanceType (COMMA InheritanceType)*;
    /// ```
    InheritanceTypesList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// InterfaceDefinition = INTERFACE_KEYWORD IDENTIFIER InheritanceSpecifier? OPEN_BRACE InterfaceMembersList? CLOSE_BRACE;
    /// ```
    InterfaceDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// InterfaceMembersList = «ContractMember»+;
    /// ```
    InterfaceMembersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LeadingTrivia = (WHITESPACE | END_OF_LINE | MULTILINE_COMMENT | SINGLE_LINE_COMMENT)+;
    /// ```
    LeadingTrivia,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LibraryDefinition = LIBRARY_KEYWORD IDENTIFIER OPEN_BRACE LibraryMembersList? CLOSE_BRACE;
    /// ```
    LibraryDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LibraryMembersList = «ContractMember»+;
    /// ```
    LibraryMembersList,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// MappingKeyType = «ElementaryType» | IdentifierPath;
    /// ```
    ///
    /// ## v0.8.18
    ///
    /// ```ebnf
    /// MappingKeyType = («ElementaryType» | IdentifierPath) IDENTIFIER?;
    /// ```
    MappingKeyType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MappingType = MAPPING_KEYWORD OPEN_PAREN MappingKeyType EQUAL_GREATER_THAN MappingValueType CLOSE_PAREN;
    /// ```
    MappingType,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// MappingValueType = TypeName;
    /// ```
    ///
    /// ## v0.8.18
    ///
    /// ```ebnf
    /// MappingValueType = TypeName IDENTIFIER?;
    /// ```
    MappingValueType,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// MemberAccessExpression = Expression «MemberAccessOperator» (* Unary Operator, Postfix *);
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// MemberAccessExpression = Expression «MemberAccessOperator» (* Unary Operator, Postfix *);
    /// ```
    MemberAccessExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ModifierAttributesList = «ModifierAttribute»+;
    /// ```
    ModifierAttributesList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ModifierDefinition = MODIFIER_KEYWORD IDENTIFIER ParametersDeclaration? ModifierAttributesList? (SEMICOLON | Block);
    /// ```
    ModifierDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ModifierInvocation = IdentifierPath ArgumentsDeclaration?;
    /// ```
    ModifierInvocation,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// NamedArgument = IDENTIFIER COLON Expression;
    /// ```
    NamedArgument,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// NamedArgumentsDeclaration = OPEN_BRACE NamedArgumentsList? CLOSE_BRACE;
    /// ```
    NamedArgumentsDeclaration,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// NamedArgumentsList = NamedArgument (COMMA NamedArgument)*;
    /// ```
    NamedArgumentsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// NamedImport = ASTERISK AS_KEYWORD IDENTIFIER FROM_KEYWORD ASCII_STRING_LITERAL;
    /// ```
    NamedImport,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// NewExpression = NEW_KEYWORD TypeName;
    /// ```
    NewExpression,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// NumericExpression = (HEX_LITERAL | DECIMAL_LITERAL) «NumberUnit»?;
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// NumericExpression = HEX_LITERAL | (DECIMAL_LITERAL «NumberUnit»?);
    /// ```
    NumericExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// OverrideSpecifier = OVERRIDE_KEYWORD (OPEN_PAREN IdentifierPathsList? CLOSE_PAREN)?;
    /// ```
    OverrideSpecifier,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// Parameter = TypeName «DataLocation»? IDENTIFIER?;
    /// ```
    Parameter,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ParametersDeclaration = OPEN_PAREN ParametersList? CLOSE_PAREN;
    /// ```
    ParametersDeclaration,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ParametersList = Parameter (COMMA Parameter)*;
    /// ```
    ParametersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PathImport = ASCII_STRING_LITERAL (AS_KEYWORD IDENTIFIER)?;
    /// ```
    PathImport,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PositionalArgumentsList = Expression (COMMA Expression)*;
    /// ```
    PositionalArgumentsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PragmaDirective = PRAGMA_KEYWORD (ABICoderPragma | ExperimentalPragma | VersionPragma) SEMICOLON;
    /// ```
    PragmaDirective,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// ReceiveFunctionAttributesList = «ReceiveFunctionAttribute»+;
    /// ```
    ReceiveFunctionAttributesList,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// ReceiveFunctionDefinition = RECEIVE_KEYWORD ParametersDeclaration ReceiveFunctionAttributesList? (SEMICOLON | Block);
    /// ```
    ReceiveFunctionDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ReturnStatement = RETURN_KEYWORD Expression? SEMICOLON;
    /// ```
    ReturnStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ReturnsDeclaration = RETURNS_KEYWORD ParametersDeclaration;
    /// ```
    ReturnsDeclaration,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// RevertStatement = REVERT_KEYWORD IdentifierPath? ArgumentsDeclaration SEMICOLON;
    /// ```
    RevertStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SourceUnit = SourceUnitMembersList? EndOfFileTrivia?;
    /// ```
    SourceUnit,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SourceUnitMembersList = «SourceUnitMember»+;
    /// ```
    SourceUnitMembersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// StateVariableAttributesList = «StateVariableAttribute»+;
    /// ```
    StateVariableAttributesList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// StateVariableDefinition = TypeName StateVariableAttributesList? IDENTIFIER (EQUAL Expression)? SEMICOLON;
    /// ```
    StateVariableDefinition,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// Statement = «SimpleStatement»
    ///           | «ControlStatement»
    ///           | AssemblyStatement
    ///           | Block;
    /// ```
    ///
    /// ## v0.8.0
    ///
    /// ```ebnf
    /// Statement = «SimpleStatement»
    ///           | «ControlStatement»
    ///           | AssemblyStatement
    ///           | Block
    ///           | UncheckedBlock;
    /// ```
    Statement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// StatementsList = Statement+;
    /// ```
    StatementsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// StructDefinition = STRUCT_KEYWORD IDENTIFIER OPEN_BRACE StructMembersList? CLOSE_BRACE;
    /// ```
    StructDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// StructMember = TypeName IDENTIFIER SEMICOLON;
    /// ```
    StructMember,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// StructMembersList = StructMember+;
    /// ```
    StructMembersList,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// ThrowStatement = THROW_KEYWORD SEMICOLON;
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    ThrowStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TrailingTrivia = WHITESPACE? SINGLE_LINE_COMMENT? END_OF_LINE;
    /// ```
    TrailingTrivia,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// TryStatement = TRY_KEYWORD Expression ReturnsDeclaration? Block CatchClausesList;
    /// ```
    TryStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TupleDeconstructionStatement = OPEN_PAREN TupleMembersList? CLOSE_PAREN EQUAL Expression SEMICOLON;
    /// ```
    TupleDeconstructionStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TupleExpression = OPEN_PAREN TupleValuesList CLOSE_PAREN;
    /// ```
    TupleExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TupleMember = ((TypeName «DataLocation»? IDENTIFIER) | («DataLocation»? IDENTIFIER))?;
    /// ```
    TupleMember,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TupleMembersList = TupleMember (COMMA TupleMember)*;
    /// ```
    TupleMembersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TupleValuesList = Expression? (COMMA Expression?)*;
    /// ```
    TupleValuesList,
    /// ## v0.5.3
    ///
    /// ```ebnf
    /// TypeExpression = TYPE_KEYWORD OPEN_PAREN TypeName CLOSE_PAREN;
    /// ```
    TypeExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TypeName = ArrayTypeName (* TypeName «ArrayTypeNameOperator» *) (* Unary Operator, Postfix *)
    ///          | FunctionType
    ///          | MappingType
    ///          | «ElementaryType»
    ///          | IdentifierPath;
    /// ```
    TypeName,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// UnaryPostfixExpression = Expression «UnaryPostfixOperator» (* Unary Operator, Postfix *);
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// UnaryPostfixExpression = Expression «UnaryPostfixOperator» (* Unary Operator, Postfix *);
    /// ```
    UnaryPostfixExpression,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// UnaryPrefixExpression = «UnaryPrefixOperator» Expression (* Unary Operator, Prefix *);
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// UnaryPrefixExpression = «UnaryPrefixOperator» Expression (* Unary Operator, Prefix *);
    /// ```
    UnaryPrefixExpression,
    /// ## v0.8.0
    ///
    /// ```ebnf
    /// UncheckedBlock = UNCHECKED_KEYWORD Block;
    /// ```
    UncheckedBlock,
    /// ## v0.7.0
    ///
    /// ```ebnf
    /// UnicodeStringLiteralsList = UNICODE_STRING_LITERAL+;
    /// ```
    UnicodeStringLiteralsList,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// UnnamedFunctionAttributesList = «UnnamedFunctionAttribute»+;
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    UnnamedFunctionAttributesList,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// UnnamedFunctionDefinition = FUNCTION_KEYWORD ParametersDeclaration UnnamedFunctionAttributesList? (SEMICOLON | Block);
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    UnnamedFunctionDefinition,
    /// ## v0.8.8
    ///
    /// ```ebnf
    /// UserDefinedValueTypeDefinition = TYPE_KEYWORD IDENTIFIER IS_KEYWORD «ElementaryType» SEMICOLON;
    /// ```
    UserDefinedValueTypeDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// UsingDirective = USING_KEYWORD (UsingDirectivePath | UsingDirectiveDeconstruction) FOR_KEYWORD (ASTERISK | TypeName) GLOBAL_KEYWORD? SEMICOLON;
    /// ```
    UsingDirective,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// UsingDirectiveDeconstruction = OPEN_BRACE UsingDirectiveSymbolsList CLOSE_BRACE;
    /// ```
    UsingDirectiveDeconstruction,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// UsingDirectivePath = IdentifierPath;
    /// ```
    UsingDirectivePath,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// UsingDirectiveSymbol = IdentifierPath;
    /// ```
    ///
    /// ## v0.8.19
    ///
    /// ```ebnf
    /// UsingDirectiveSymbol = IdentifierPath (AS_KEYWORD «UsingDirectiveOperator»)?;
    /// ```
    UsingDirectiveSymbol,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// UsingDirectiveSymbolsList = UsingDirectiveSymbol (COMMA UsingDirectiveSymbol)*;
    /// ```
    UsingDirectiveSymbolsList,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// VariableDeclaration = (VAR_KEYWORD | TypeName) «DataLocation»? IDENTIFIER;
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// VariableDeclaration = TypeName «DataLocation»? IDENTIFIER;
    /// ```
    VariableDeclaration,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VariableDeclarationStatement = VariableDeclaration (EQUAL Expression)? SEMICOLON;
    /// ```
    VariableDeclarationStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VersionPragma = SOLIDITY_KEYWORD VersionPragmaExpressionsList;
    /// ```
    VersionPragma,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VersionPragmaBinaryExpression = VersionPragmaExpression «VersionPragmaOrOperator» VersionPragmaExpression (* Binary Operator, Left Associative *);
    /// VersionPragmaBinaryExpression = VersionPragmaExpression «VersionPragmaRangeOperator» VersionPragmaExpression (* Binary Operator, Left Associative *);
    /// ```
    VersionPragmaBinaryExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VersionPragmaExpression = VersionPragmaBinaryExpression (* VersionPragmaExpression «VersionPragmaOrOperator» VersionPragmaExpression *) (* Binary Operator, Left Associative *)
    ///                         | VersionPragmaBinaryExpression (* VersionPragmaExpression «VersionPragmaRangeOperator» VersionPragmaExpression *) (* Binary Operator, Left Associative *)
    ///                         | VersionPragmaUnaryExpression (* «VersionPragmaUnaryOperator» VersionPragmaExpression *) (* Unary Operator, Prefix *)
    ///                         | VersionPragmaSpecifier;
    /// ```
    VersionPragmaExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VersionPragmaExpressionsList = VersionPragmaExpression+;
    /// ```
    VersionPragmaExpressionsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VersionPragmaSpecifier = VERSION_PRAGMA_VALUE (PERIOD VERSION_PRAGMA_VALUE)*;
    /// ```
    VersionPragmaSpecifier,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VersionPragmaUnaryExpression = «VersionPragmaUnaryOperator» VersionPragmaExpression (* Unary Operator, Prefix *);
    /// ```
    VersionPragmaUnaryExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WhileStatement = WHILE_KEYWORD OPEN_PAREN Expression CLOSE_PAREN Statement;
    /// ```
    WhileStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulAssignmentStatement = YulIdentifierPathsList COLON_EQUAL YulExpression;
    /// ```
    YulAssignmentStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulBlock = OPEN_BRACE YulStatementsList? CLOSE_BRACE;
    /// ```
    YulBlock,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulBreakStatement = BREAK_KEYWORD;
    /// ```
    YulBreakStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulContinueStatement = CONTINUE_KEYWORD;
    /// ```
    YulContinueStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulDeclarationStatement = LET_KEYWORD YulIdentifierPathsList (COLON_EQUAL YulExpression)?;
    /// ```
    YulDeclarationStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulExpression = YulFunctionCallExpression (* YulExpression «YulFunctionCallOperator» *) (* Unary Operator, Postfix *)
    ///               | «YulLiteral»
    ///               | YulIdentifierPath;
    /// ```
    YulExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulExpressionsList = YulExpression (COMMA YulExpression)*;
    /// ```
    YulExpressionsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulForStatement = FOR_KEYWORD YulBlock YulExpression YulBlock YulBlock;
    /// ```
    YulForStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulFunctionCallExpression = YulExpression «YulFunctionCallOperator» (* Unary Operator, Postfix *);
    /// ```
    YulFunctionCallExpression,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulFunctionDefinition = FUNCTION_KEYWORD YUL_IDENTIFIER YulParametersDeclaration YulReturnsDeclaration? YulBlock;
    /// ```
    YulFunctionDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulIdentifierPath = YUL_IDENTIFIER (PERIOD YUL_IDENTIFIER)*;
    /// ```
    YulIdentifierPath,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulIdentifierPathsList = YulIdentifierPath (COMMA YulIdentifierPath)*;
    /// ```
    YulIdentifierPathsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulIdentifiersList = YUL_IDENTIFIER (COMMA YUL_IDENTIFIER)*;
    /// ```
    YulIdentifiersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulIfStatement = IF_KEYWORD YulExpression YulBlock;
    /// ```
    YulIfStatement,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// YulLeaveStatement = LEAVE_KEYWORD;
    /// ```
    YulLeaveStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulParametersDeclaration = OPEN_PAREN YulIdentifiersList? CLOSE_PAREN;
    /// ```
    YulParametersDeclaration,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulReturnsDeclaration = MINUS_GREATER_THAN YulIdentifiersList;
    /// ```
    YulReturnsDeclaration,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// YulStatement = YulBlock
    ///              | YulFunctionDefinition
    ///              | YulDeclarationStatement
    ///              | YulAssignmentStatement
    ///              | YulIfStatement
    ///              | YulForStatement
    ///              | YulSwitchStatement
    ///              | YulBreakStatement
    ///              | YulContinueStatement
    ///              | YulExpression;
    /// ```
    ///
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// YulStatement = YulBlock
    ///              | YulFunctionDefinition
    ///              | YulDeclarationStatement
    ///              | YulAssignmentStatement
    ///              | YulIfStatement
    ///              | YulForStatement
    ///              | YulSwitchStatement
    ///              | YulLeaveStatement
    ///              | YulBreakStatement
    ///              | YulContinueStatement
    ///              | YulExpression;
    /// ```
    YulStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulStatementsList = YulStatement+;
    /// ```
    YulStatementsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulSwitchCase = (DEFAULT_KEYWORD | (CASE_KEYWORD «YulLiteral»)) YulBlock;
    /// ```
    YulSwitchCase,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulSwitchCasesList = YulSwitchCase+;
    /// ```
    YulSwitchCasesList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YulSwitchStatement = SWITCH_KEYWORD YulExpression YulSwitchCasesList;
    /// ```
    YulSwitchStatement,
}
