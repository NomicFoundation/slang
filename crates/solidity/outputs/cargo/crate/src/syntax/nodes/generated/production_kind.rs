// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

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
pub enum ProductionKind {
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ABICoderPragma = ABICODER_KEYWORD IDENTIFIER;
    /// ```
    ABICoderPragma,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ABICODER_KEYWORD = "abicoder";
    /// ```
    AbicoderKeyword,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// ABSTRACT_KEYWORD = "abstract";
    /// ```
    AbstractKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ADDRESS_KEYWORD = "address";
    /// ```
    AddressKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AddressType = (ADDRESS_KEYWORD PAYABLE_KEYWORD?) | PAYABLE_KEYWORD;
    /// ```
    AddressType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AMPERSAND = "&";
    /// ```
    Ampersand,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AMPERSAND_AMPERSAND = "&&";
    /// ```
    AmpersandAmpersand,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AMPERSAND_EQUAL = "&=";
    /// ```
    AmpersandEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ANONYMOUS_KEYWORD = "anonymous";
    /// ```
    AnonymousKeyword,
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
    /// ArrayValuesList = Expression (COMMA Expression)*;
    /// ```
    ArrayValuesList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AS_KEYWORD = "as";
    /// ```
    AsKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ASCII_STRING_LITERAL = «SINGLE_QUOTED_ASCII_STRING_LITERAL»
    ///                      | «DOUBLE_QUOTED_ASCII_STRING_LITERAL»;
    /// ```
    AsciiStringLiteral,
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
    /// ASSEMBLY_KEYWORD = "assembly";
    /// ```
    AssemblyKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AssemblyStatement = ASSEMBLY_KEYWORD ASCII_STRING_LITERAL? (OPEN_PAREN AssemblyFlagsList CLOSE_PAREN)? YulBlock;
    /// ```
    AssemblyStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ASTERISK = "*";
    /// ```
    Asterisk,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ASTERISK_ASTERISK = "**";
    /// ```
    AsteriskAsterisk,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ASTERISK_EQUAL = "*=";
    /// ```
    AsteriskEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BANG = "!";
    /// ```
    Bang,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BANG_EQUAL = "!=";
    /// ```
    BangEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BAR = "|";
    /// ```
    Bar,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BAR_BAR = "||";
    /// ```
    BarBar,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BAR_EQUAL = "|=";
    /// ```
    BarEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// Block = OPEN_BRACE StatementsList? CLOSE_BRACE;
    /// ```
    Block,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BOOL_KEYWORD = "bool";
    /// ```
    BoolKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BREAK_KEYWORD = "break";
    /// ```
    BreakKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BreakStatement = BREAK_KEYWORD SEMICOLON;
    /// ```
    BreakStatement,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// BYTE_KEYWORD = "byte";
    /// ```
    ///
    /// ## v0.8.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    ByteKeyword,
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// CALLDATA_KEYWORD = "calldata";
    /// ```
    CalldataKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CARET = "^";
    /// ```
    Caret,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CARET_EQUAL = "^=";
    /// ```
    CaretEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CASE_KEYWORD = "case";
    /// ```
    CaseKeyword,
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
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// CATCH_KEYWORD = "catch";
    /// ```
    CatchKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CLOSE_BRACE = "}";
    /// ```
    CloseBrace,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CLOSE_BRACKET = "]";
    /// ```
    CloseBracket,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CLOSE_PAREN = ")";
    /// ```
    CloseParen,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// COLON = ":";
    /// ```
    Colon,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// COLON_EQUAL = ":=";
    /// ```
    ColonEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// COMMA = ",";
    /// ```
    Comma,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ConstantDefinition = TypeName CONSTANT_KEYWORD IDENTIFIER EQUAL Expression SEMICOLON;
    /// ```
    ConstantDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CONSTANT_KEYWORD = "constant";
    /// ```
    ConstantKeyword,
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
    /// ## v0.4.22
    ///
    /// ```ebnf
    /// CONSTRUCTOR_KEYWORD = "constructor";
    /// ```
    ConstructorKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CONTINUE_KEYWORD = "continue";
    /// ```
    ContinueKeyword,
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
    /// CONTRACT_KEYWORD = "contract";
    /// ```
    ContractKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ContractMembersList = «ContractMember»+;
    /// ```
    ContractMembersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DAYS_KEYWORD = "days";
    /// ```
    DaysKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// DECIMAL_LITERAL = ((«DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»?)?) | ("." «DECIMAL_DIGITS»)) «DECIMAL_EXPONENT»?;
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// DECIMAL_LITERAL = ((«DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»)?) | ("." «DECIMAL_DIGITS»)) «DECIMAL_EXPONENT»?;
    /// ```
    DecimalLiteral,
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
    /// DEFAULT_KEYWORD = "default";
    /// ```
    DefaultKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DELETE_KEYWORD = "delete";
    /// ```
    DeleteKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DeleteStatement = DELETE_KEYWORD Expression SEMICOLON;
    /// ```
    DeleteStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DO_KEYWORD = "do";
    /// ```
    DoKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DoWhileStatement = DO_KEYWORD Statement WHILE_KEYWORD OPEN_PAREN Expression CLOSE_PAREN SEMICOLON;
    /// ```
    DoWhileStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ELSE_KEYWORD = "else";
    /// ```
    ElseKeyword,
    /// ## v0.4.21
    ///
    /// ```ebnf
    /// EMIT_KEYWORD = "emit";
    /// ```
    EmitKeyword,
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
    /// END_OF_LINE = "\r"? "\n";
    /// ```
    EndOfLine,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EnumDefinition = ENUM_KEYWORD IDENTIFIER OPEN_BRACE IdentifiersList? CLOSE_BRACE;
    /// ```
    EnumDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ENUM_KEYWORD = "enum";
    /// ```
    EnumKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EQUAL = "=";
    /// ```
    Equal,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EQUAL_EQUAL = "==";
    /// ```
    EqualEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EQUAL_GREATER_THAN = "=>";
    /// ```
    EqualGreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ErrorDefinition = ERROR_KEYWORD IDENTIFIER OPEN_PAREN ErrorParametersList? CLOSE_PAREN SEMICOLON;
    /// ```
    ErrorDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ERROR_KEYWORD = "error";
    /// ```
    ErrorKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ErrorParameter = TypeName IDENTIFIER?;
    /// ```
    ErrorParameter,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ErrorParametersList = ErrorParameter (COMMA ErrorParameter)*;
    /// ```
    ErrorParametersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ETHER_KEYWORD = "ether";
    /// ```
    EtherKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EventDefinition = EVENT_KEYWORD IDENTIFIER OPEN_PAREN EventParametersList? CLOSE_PAREN ANONYMOUS_KEYWORD? SEMICOLON;
    /// ```
    EventDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EVENT_KEYWORD = "event";
    /// ```
    EventKeyword,
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
    /// EXPERIMENTAL_KEYWORD = "experimental";
    /// ```
    ExperimentalKeyword,
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
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EXTERNAL_KEYWORD = "external";
    /// ```
    ExternalKeyword,
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
    /// FALLBACK_KEYWORD = "fallback";
    /// ```
    FallbackKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FALSE_KEYWORD = "false";
    /// ```
    FalseKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// FINNEY_KEYWORD = "finney";
    /// ```
    ///
    /// ## v0.7.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    FinneyKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FIXED_BYTES_TYPE = "bytes" «FIXED_BYTES_TYPE_SIZE»;
    /// ```
    FixedBytesType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FOR_KEYWORD = "for";
    /// ```
    ForKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ForStatement = FOR_KEYWORD OPEN_PAREN («SimpleStatement» | SEMICOLON) (ExpressionStatement | SEMICOLON) Expression? CLOSE_PAREN Statement;
    /// ```
    ForStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FROM_KEYWORD = "from";
    /// ```
    FromKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FunctionAttributesList = «FunctionAttribute»+;
    /// ```
    FunctionAttributesList,
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
    /// FUNCTION_KEYWORD = "function";
    /// ```
    FunctionKeyword,
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
    /// GLOBAL_KEYWORD = "global";
    /// ```
    GlobalKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN = ">";
    /// ```
    GreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_EQUAL = ">=";
    /// ```
    GreaterThanEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN = ">>";
    /// ```
    GreaterThanGreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_EQUAL = ">>=";
    /// ```
    GreaterThanGreaterThanEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN = ">>>";
    /// ```
    GreaterThanGreaterThanGreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL = ">>>=";
    /// ```
    GreaterThanGreaterThanGreaterThanEqual,
    /// ## v0.6.11
    ///
    /// ```ebnf
    /// GWEI_KEYWORD = "gwei";
    /// ```
    GweiKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// HEX_LITERAL = ("0x" | "0X") «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// HEX_LITERAL = "0x" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
    /// ```
    HexLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// HEX_STRING_LITERAL = «SINGLE_QUOTED_HEX_STRING_LITERAL»
    ///                    | «DOUBLE_QUOTED_HEX_STRING_LITERAL»;
    /// ```
    HexStringLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// HexStringLiteralsList = HEX_STRING_LITERAL+;
    /// ```
    HexStringLiteralsList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// HOURS_KEYWORD = "hours";
    /// ```
    HoursKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// IDENTIFIER = «RAW_IDENTIFIER» - («KEYWORD_IN_ANY_VERSION» | «KEYWORD_IN_SOME_VERSION» | «RESERVED_WORD_IN_ANY_VERSION»);
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// IDENTIFIER = «RAW_IDENTIFIER» - («KEYWORD_IN_ANY_VERSION» | «KEYWORD_IN_SOME_VERSION» | «RESERVED_WORD_IN_ANY_VERSION» | «RESERVED_WORD_IN_SOME_VERSION»);
    /// ```
    Identifier,
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
    /// IF_KEYWORD = "if";
    /// ```
    IfKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IfStatement = IF_KEYWORD OPEN_PAREN Expression CLOSE_PAREN Statement (ELSE_KEYWORD Statement)?;
    /// ```
    IfStatement,
    /// ## v0.6.5
    ///
    /// ```ebnf
    /// IMMUTABLE_KEYWORD = "immutable";
    /// ```
    ImmutableKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ImportDirective = IMPORT_KEYWORD (PathImport | NamedImport | DeconstructionImport) SEMICOLON;
    /// ```
    ImportDirective,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IMPORT_KEYWORD = "import";
    /// ```
    ImportKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// INDEXED_KEYWORD = "indexed";
    /// ```
    IndexedKeyword,
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
    /// INTERFACE_KEYWORD = "interface";
    /// ```
    InterfaceKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// InterfaceMembersList = «ContractMember»+;
    /// ```
    InterfaceMembersList,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// INTERNAL_KEYWORD = "internal";
    /// ```
    InternalKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IS_KEYWORD = "is";
    /// ```
    IsKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LeadingTrivia = (WHITESPACE | END_OF_LINE | MULTILINE_COMMENT | SINGLE_LINE_COMMENT)+;
    /// ```
    LeadingTrivia,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// LEAVE_KEYWORD = "leave";
    /// ```
    LeaveKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LESS_THAN = "<";
    /// ```
    LessThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LESS_THAN_EQUAL = "<=";
    /// ```
    LessThanEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LESS_THAN_LESS_THAN = "<<";
    /// ```
    LessThanLessThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LESS_THAN_LESS_THAN_EQUAL = "<<=";
    /// ```
    LessThanLessThanEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LET_KEYWORD = "let";
    /// ```
    LetKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LibraryDefinition = LIBRARY_KEYWORD IDENTIFIER OPEN_BRACE LibraryMembersList? CLOSE_BRACE;
    /// ```
    LibraryDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LIBRARY_KEYWORD = "library";
    /// ```
    LibraryKeyword,
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
    /// MAPPING_KEYWORD = "mapping";
    /// ```
    MappingKeyword,
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
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MEMORY_KEYWORD = "memory";
    /// ```
    MemoryKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUS = "-";
    /// ```
    Minus,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUS_EQUAL = "-=";
    /// ```
    MinusEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUS_GREATER_THAN = "->";
    /// ```
    MinusGreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUS_MINUS = "--";
    /// ```
    MinusMinus,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUTES_KEYWORD = "minutes";
    /// ```
    MinutesKeyword,
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
    /// MODIFIER_KEYWORD = "modifier";
    /// ```
    ModifierKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MULTILINE_COMMENT = "/" "*" (!"*" | "*")* "*" "/";
    /// ```
    MultilineComment,
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
    /// ## Unversioned
    ///
    /// ```ebnf
    /// NEW_KEYWORD = "new";
    /// ```
    NewKeyword,
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
    /// OPEN_BRACE = "{";
    /// ```
    OpenBrace,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// OPEN_BRACKET = "[";
    /// ```
    OpenBracket,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// OPEN_PAREN = "(";
    /// ```
    OpenParen,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// OVERRIDE_KEYWORD = "override";
    /// ```
    OverrideKeyword,
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
    /// PAYABLE_KEYWORD = "payable";
    /// ```
    PayableKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PERCENT = "%";
    /// ```
    Percent,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PERCENT_EQUAL = "%=";
    /// ```
    PercentEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PERIOD = ".";
    /// ```
    Period,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PLUS = "+";
    /// ```
    Plus,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PLUS_EQUAL = "+=";
    /// ```
    PlusEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PLUS_PLUS = "++";
    /// ```
    PlusPlus,
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
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PRAGMA_KEYWORD = "pragma";
    /// ```
    PragmaKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PRIVATE_KEYWORD = "private";
    /// ```
    PrivateKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PUBLIC_KEYWORD = "public";
    /// ```
    PublicKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PURE_KEYWORD = "pure";
    /// ```
    PureKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// QUESTION_MARK = "?";
    /// ```
    QuestionMark,
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
    /// RECEIVE_KEYWORD = "receive";
    /// ```
    ReceiveKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// RETURN_KEYWORD = "return";
    /// ```
    ReturnKeyword,
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
    /// RETURNS_KEYWORD = "returns";
    /// ```
    ReturnsKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// REVERT_KEYWORD = "revert";
    /// ```
    RevertKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// RevertStatement = REVERT_KEYWORD IdentifierPath? ArgumentsDeclaration SEMICOLON;
    /// ```
    RevertStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SECONDS_KEYWORD = "seconds";
    /// ```
    SecondsKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SEMICOLON = ";";
    /// ```
    Semicolon,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SIGNED_FIXED_TYPE = "fixed" «FIXED_TYPE_SIZE»?;
    /// ```
    SignedFixedType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SIGNED_INTEGER_TYPE = "int" «INTEGER_TYPE_SIZE»?;
    /// ```
    SignedIntegerType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SINGLE_LINE_COMMENT = "//" (!("\r" | "\n"))*;
    /// ```
    SingleLineComment,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SLASH = "/";
    /// ```
    Slash,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SLASH_EQUAL = "/=";
    /// ```
    SlashEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SOLIDITY_KEYWORD = "solidity";
    /// ```
    SolidityKeyword,
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
    /// STORAGE_KEYWORD = "storage";
    /// ```
    StorageKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// STRING_KEYWORD = "string";
    /// ```
    StringKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// StructDefinition = STRUCT_KEYWORD IDENTIFIER OPEN_BRACE StructMembersList? CLOSE_BRACE;
    /// ```
    StructDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// STRUCT_KEYWORD = "struct";
    /// ```
    StructKeyword,
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
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SWITCH_KEYWORD = "switch";
    /// ```
    SwitchKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// SZABO_KEYWORD = "szabo";
    /// ```
    ///
    /// ## v0.7.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    SzaboKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// THROW_KEYWORD = "throw";
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    ThrowKeyword,
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
    /// TILDE = "~";
    /// ```
    Tilde,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TrailingTrivia = WHITESPACE? SINGLE_LINE_COMMENT? END_OF_LINE;
    /// ```
    TrailingTrivia,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TRUE_KEYWORD = "true";
    /// ```
    TrueKeyword,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// TRY_KEYWORD = "try";
    /// ```
    TryKeyword,
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
    /// ## v0.5.3
    ///
    /// ```ebnf
    /// TYPE_KEYWORD = "type";
    /// ```
    TypeKeyword,
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
    /// ## v0.8.0
    ///
    /// ```ebnf
    /// UncheckedBlock = UNCHECKED_KEYWORD Block;
    /// ```
    UncheckedBlock,
    /// ## v0.8.0
    ///
    /// ```ebnf
    /// UNCHECKED_KEYWORD = "unchecked";
    /// ```
    UncheckedKeyword,
    /// ## v0.7.0
    ///
    /// ```ebnf
    /// UNICODE_STRING_LITERAL = «SINGLE_QUOTED_UNICODE_STRING_LITERAL»
    ///                        | «DOUBLE_QUOTED_UNICODE_STRING_LITERAL»;
    /// ```
    UnicodeStringLiteral,
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
    /// ## Unversioned
    ///
    /// ```ebnf
    /// UNSIGNED_FIXED_TYPE = "ufixed" «FIXED_TYPE_SIZE»?;
    /// ```
    UnsignedFixedType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// UNSIGNED_INTEGER_TYPE = "uint" «INTEGER_TYPE_SIZE»?;
    /// ```
    UnsignedIntegerType,
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
    /// ## Unversioned
    ///
    /// ```ebnf
    /// USING_KEYWORD = "using";
    /// ```
    UsingKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// VAR_KEYWORD = "var";
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    VarKeyword,
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
    /// VERSION_PRAGMA_VALUE = ("0"…"9" | "x" | "X" | "*")+;
    /// ```
    VersionPragmaValue,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VIEW_KEYWORD = "view";
    /// ```
    ViewKeyword,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// VIRTUAL_KEYWORD = "virtual";
    /// ```
    VirtualKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WEEKS_KEYWORD = "weeks";
    /// ```
    WeeksKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WEI_KEYWORD = "wei";
    /// ```
    WeiKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WHILE_KEYWORD = "while";
    /// ```
    WhileKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WhileStatement = WHILE_KEYWORD OPEN_PAREN Expression CLOSE_PAREN Statement;
    /// ```
    WhileStatement,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WHITESPACE = (" " | "\t")+;
    /// ```
    Whitespace,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// YEARS_KEYWORD = "years";
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    YearsKeyword,
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
    /// YUL_DECIMAL_LITERAL = "0" | ("1"…"9" "0"…"9"*);
    /// ```
    YulDecimalLiteral,
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
    /// YulFunctionDefinition = FUNCTION_KEYWORD YUL_IDENTIFIER YulParametersDeclaration YulReturnsDeclaration? YulBlock;
    /// ```
    YulFunctionDefinition,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YUL_HEX_LITERAL = "0x" «HEX_CHARACTER»+;
    /// ```
    YulHexLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YUL_IDENTIFIER = «RAW_IDENTIFIER» - («YUL_KEYWORD» | «YUL_RESERVED_WORD»);
    /// ```
    YulIdentifier,
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
