// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

/// Represents different kinds of nonterminal nodes in the syntax tree.
/// These are nodes that can have child nodes and represent higher-level language constructs.
#[repr(u8)]
#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
    Clone,
    Copy,
)]
#[allow(clippy::doc_markdown)]
#[allow(clippy::doc_link_with_quotes)]
pub enum NonterminalKind {
    /// Represents a node with kind `AbicoderPragma`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.5 *)
    /// AbicoderPragma = (* abicoder_keyword: *) ABICODER_KEYWORD
    ///                  (* version: *) AbicoderVersion;
    /// ```
    AbicoderPragma,
    /// Represents a node with kind `AbicoderVersion`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.5 *)
    /// AbicoderVersion = (* variant: *) ABICODER_V1_KEYWORD
    ///                 | (* variant: *) ABICODER_V2_KEYWORD;
    /// ```
    AbicoderVersion,
    /// Represents a node with kind `AdditiveExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// AdditiveExpression = (* left_operand: *) Expression
    ///                      (* operator: *) PLUS
    ///                      (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AdditiveExpression = (* left_operand: *) Expression
    ///                      (* operator: *) MINUS
    ///                      (* right_operand: *) Expression;
    /// ```
    AdditiveExpression,
    /// Represents a node with kind `AddressType`, having the following structure:
    ///
    /// ```ebnf
    /// AddressType = (* address_keyword: *) ADDRESS_KEYWORD
    ///               (* payable_keyword: *) PAYABLE_KEYWORD?; (* Introduced in 0.5.0 *)
    /// ```
    AddressType,
    /// Represents a node with kind `AndExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// AndExpression = (* left_operand: *) Expression
    ///                 (* operator: *) AMPERSAND_AMPERSAND
    ///                 (* right_operand: *) Expression;
    /// ```
    AndExpression,
    /// Represents a node with kind `ArgumentsDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// ArgumentsDeclaration = (* variant: *) PositionalArgumentsDeclaration
    ///                      | (* variant: *) NamedArgumentsDeclaration;
    /// ```
    ArgumentsDeclaration,
    /// Represents a node with kind `ArrayExpression`, having the following structure:
    ///
    /// ```ebnf
    /// ArrayExpression = (* open_bracket: *) OPEN_BRACKET
    ///                   (* items: *) ArrayValues
    ///                   (* close_bracket: *) CLOSE_BRACKET;
    /// ```
    ArrayExpression,
    /// Represents a node with kind `ArrayTypeName`, having the following structure:
    ///
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// ArrayTypeName = (* operand: *) TypeName
    ///                 (* open_bracket: *) OPEN_BRACKET
    ///                 (* index: *) Expression?
    ///                 (* close_bracket: *) CLOSE_BRACKET;
    /// ```
    ArrayTypeName,
    /// Represents a node with kind `ArrayValues`, having the following structure:
    ///
    /// ```ebnf
    /// ArrayValues = (* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*;
    /// ```
    ArrayValues,
    /// Represents a node with kind `AssemblyFlags`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// AssemblyFlags = (* item: *) StringLiteral ((* separator: *) COMMA (* item: *) StringLiteral)*;
    /// ```
    AssemblyFlags,
    /// Represents a node with kind `AssemblyFlagsDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// AssemblyFlagsDeclaration = (* open_paren: *) OPEN_PAREN
    ///                            (* flags: *) AssemblyFlags
    ///                            (* close_paren: *) CLOSE_PAREN;
    /// ```
    AssemblyFlagsDeclaration,
    /// Represents a node with kind `AssemblyStatement`, having the following structure:
    ///
    /// ```ebnf
    /// AssemblyStatement = (* assembly_keyword: *) ASSEMBLY_KEYWORD
    ///                     (* label: *) StringLiteral?
    ///                     (* flags: *) AssemblyFlagsDeclaration? (* Introduced in 0.8.13 *)
    ///                     (* body: *) YulBlock;
    /// ```
    AssemblyStatement,
    /// Represents a node with kind `AssignmentExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) BAR_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) PLUS_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) MINUS_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) CARET_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) SLASH_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) PERCENT_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) ASTERISK_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) AMPERSAND_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) LESS_THAN_LESS_THAN_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) GREATER_THAN_GREATER_THAN_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// AssignmentExpression = (* left_operand: *) Expression
    ///                        (* operator: *) GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL
    ///                        (* right_operand: *) Expression;
    /// ```
    AssignmentExpression,
    /// Represents a node with kind `BitwiseAndExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// BitwiseAndExpression = (* left_operand: *) Expression
    ///                        (* operator: *) AMPERSAND
    ///                        (* right_operand: *) Expression;
    /// ```
    BitwiseAndExpression,
    /// Represents a node with kind `BitwiseOrExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// BitwiseOrExpression = (* left_operand: *) Expression
    ///                       (* operator: *) BAR
    ///                       (* right_operand: *) Expression;
    /// ```
    BitwiseOrExpression,
    /// Represents a node with kind `BitwiseXorExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// BitwiseXorExpression = (* left_operand: *) Expression
    ///                        (* operator: *) CARET
    ///                        (* right_operand: *) Expression;
    /// ```
    BitwiseXorExpression,
    /// Represents a node with kind `Block`, having the following structure:
    ///
    /// ```ebnf
    /// Block = (* open_brace: *) OPEN_BRACE
    ///         (* statements: *) Statements
    ///         (* close_brace: *) CLOSE_BRACE;
    /// ```
    Block,
    /// Represents a node with kind `BreakStatement`, having the following structure:
    ///
    /// ```ebnf
    /// BreakStatement = (* break_keyword: *) BREAK_KEYWORD
    ///                  (* semicolon: *) SEMICOLON;
    /// ```
    BreakStatement,
    /// Represents a node with kind `CallOptions`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.2 *)
    /// CallOptions = (* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*;
    /// ```
    CallOptions,
    /// Represents a node with kind `CallOptionsExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// (* Introduced in 0.6.2 *)
    /// CallOptionsExpression = (* operand: *) Expression
    ///                         (* open_brace: *) OPEN_BRACE
    ///                         (* options: *) CallOptions
    ///                         (* close_brace: *) CLOSE_BRACE;
    /// ```
    CallOptionsExpression,
    /// Represents a node with kind `CatchClause`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// CatchClause = (* catch_keyword: *) CATCH_KEYWORD
    ///               (* error: *) CatchClauseError?
    ///               (* body: *) Block;
    /// ```
    CatchClause,
    /// Represents a node with kind `CatchClauseError`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// CatchClauseError = (* name: *) IDENTIFIER?
    ///                    (* parameters: *) ParametersDeclaration;
    /// ```
    CatchClauseError,
    /// Represents a node with kind `CatchClauses`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// CatchClauses = (* item: *) CatchClause+;
    /// ```
    CatchClauses,
    /// Represents a node with kind `ConditionalExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// ConditionalExpression = (* operand: *) Expression
    ///                         (* question_mark: *) QUESTION_MARK
    ///                         (* true_expression: *) Expression
    ///                         (* colon: *) COLON
    ///                         (* false_expression: *) Expression;
    /// ```
    ConditionalExpression,
    /// Represents a node with kind `ConstantDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.4 *)
    /// ConstantDefinition = (* type_name: *) TypeName
    ///                      (* constant_keyword: *) CONSTANT_KEYWORD
    ///                      (* name: *) IDENTIFIER
    ///                      (* equal: *) EQUAL
    ///                      (* value: *) Expression
    ///                      (* semicolon: *) SEMICOLON;
    /// ```
    ConstantDefinition,
    /// Represents a node with kind `ConstructorAttribute`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.22 *)
    /// ConstructorAttribute = (* variant: *) ModifierInvocation
    ///                      | (* variant: *) INTERNAL_KEYWORD
    ///                      | (* variant: *) OVERRIDE_KEYWORD (* Introduced in 0.6.0 and deprecated in 0.6.7. *)
    ///                      | (* variant: *) PAYABLE_KEYWORD
    ///                      | (* variant: *) PUBLIC_KEYWORD
    ///                      | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 and deprecated in 0.6.7. *)
    /// ```
    ConstructorAttribute,
    /// Represents a node with kind `ConstructorAttributes`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.22 *)
    /// ConstructorAttributes = (* item: *) ConstructorAttribute*;
    /// ```
    ConstructorAttributes,
    /// Represents a node with kind `ConstructorDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.22 *)
    /// ConstructorDefinition = (* constructor_keyword: *) CONSTRUCTOR_KEYWORD
    ///                         (* parameters: *) ParametersDeclaration
    ///                         (* attributes: *) ConstructorAttributes
    ///                         (* body: *) Block;
    /// ```
    ConstructorDefinition,
    /// Represents a node with kind `ContinueStatement`, having the following structure:
    ///
    /// ```ebnf
    /// ContinueStatement = (* continue_keyword: *) CONTINUE_KEYWORD
    ///                     (* semicolon: *) SEMICOLON;
    /// ```
    ContinueStatement,
    /// Represents a node with kind `ContractDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// ContractDefinition = (* abstract_keyword: *) ABSTRACT_KEYWORD? (* Introduced in 0.6.0 *)
    ///                      (* contract_keyword: *) CONTRACT_KEYWORD
    ///                      (* name: *) IDENTIFIER
    ///                      (* specifiers: *) ContractSpecifiers
    ///                      (* open_brace: *) OPEN_BRACE
    ///                      (* members: *) ContractMembers
    ///                      (* close_brace: *) CLOSE_BRACE;
    /// ```
    ContractDefinition,
    /// Represents a node with kind `ContractMember`, having the following structure:
    ///
    /// ```ebnf
    /// ContractMember = (* variant: *) UsingDirective
    ///                | (* variant: *) FunctionDefinition
    ///                | (* variant: *) ConstructorDefinition (* Introduced in 0.4.22 *)
    ///                | (* variant: *) ReceiveFunctionDefinition (* Introduced in 0.6.0 *)
    ///                | (* variant: *) FallbackFunctionDefinition (* Introduced in 0.6.0 *)
    ///                | (* variant: *) UnnamedFunctionDefinition (* Deprecated in 0.6.0 *)
    ///                | (* variant: *) ModifierDefinition
    ///                | (* variant: *) StructDefinition
    ///                | (* variant: *) EnumDefinition
    ///                | (* variant: *) EventDefinition
    ///                | (* variant: *) ErrorDefinition (* Introduced in 0.8.4 *)
    ///                | (* variant: *) UserDefinedValueTypeDefinition (* Introduced in 0.8.8 *)
    ///                | (* variant: *) StateVariableDefinition;
    /// ```
    ContractMember,
    /// Represents a node with kind `ContractMembers`, having the following structure:
    ///
    /// ```ebnf
    /// ContractMembers = (* item: *) ContractMember*;
    /// ```
    ContractMembers,
    /// Represents a node with kind `ContractSpecifier`, having the following structure:
    ///
    /// ```ebnf
    /// ContractSpecifier = (* variant: *) InheritanceSpecifier
    ///                   | (* variant: *) StorageLayoutSpecifier; (* Introduced in 0.8.29 *)
    /// ```
    ContractSpecifier,
    /// Represents a node with kind `ContractSpecifiers`, having the following structure:
    ///
    /// ```ebnf
    /// ContractSpecifiers = (* item: *) ContractSpecifier*;
    /// ```
    ContractSpecifiers,
    /// Represents a node with kind `DecimalNumberExpression`, having the following structure:
    ///
    /// ```ebnf
    /// DecimalNumberExpression = (* literal: *) DECIMAL_LITERAL
    ///                           (* unit: *) NumberUnit?;
    /// ```
    DecimalNumberExpression,
    /// Represents a node with kind `DoWhileStatement`, having the following structure:
    ///
    /// ```ebnf
    /// DoWhileStatement = (* do_keyword: *) DO_KEYWORD
    ///                    (* body: *) Statement
    ///                    (* while_keyword: *) WHILE_KEYWORD
    ///                    (* open_paren: *) OPEN_PAREN
    ///                    (* condition: *) Expression
    ///                    (* close_paren: *) CLOSE_PAREN
    ///                    (* semicolon: *) SEMICOLON;
    /// ```
    DoWhileStatement,
    /// Represents a node with kind `ElementaryType`, having the following structure:
    ///
    /// ```ebnf
    /// ElementaryType = (* variant: *) BOOL_KEYWORD
    ///                | (* variant: *) BYTE_KEYWORD (* Deprecated in 0.8.0 *)
    ///                | (* variant: *) STRING_KEYWORD
    ///                | (* variant: *) AddressType
    ///                | (* variant: *) BYTES_KEYWORD
    ///                | (* variant: *) INT_KEYWORD
    ///                | (* variant: *) UINT_KEYWORD
    ///                | (* variant: *) FIXED_KEYWORD
    ///                | (* variant: *) UFIXED_KEYWORD;
    /// ```
    ElementaryType,
    /// Represents a node with kind `ElseBranch`, having the following structure:
    ///
    /// ```ebnf
    /// ElseBranch = (* else_keyword: *) ELSE_KEYWORD
    ///              (* body: *) Statement;
    /// ```
    ElseBranch,
    /// Represents a node with kind `EmitStatement`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.21 *)
    /// EmitStatement = (* emit_keyword: *) EMIT_KEYWORD
    ///                 (* event: *) IdentifierPath
    ///                 (* arguments: *) ArgumentsDeclaration
    ///                 (* semicolon: *) SEMICOLON;
    /// ```
    EmitStatement,
    /// Represents a node with kind `EnumDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// EnumDefinition = (* enum_keyword: *) ENUM_KEYWORD
    ///                  (* name: *) IDENTIFIER
    ///                  (* open_brace: *) OPEN_BRACE
    ///                  (* members: *) EnumMembers
    ///                  (* close_brace: *) CLOSE_BRACE;
    /// ```
    EnumDefinition,
    /// Represents a node with kind `EnumMembers`, having the following structure:
    ///
    /// ```ebnf
    /// EnumMembers = ((* item: *) IDENTIFIER ((* separator: *) COMMA (* item: *) IDENTIFIER)*)?;
    /// ```
    EnumMembers,
    /// Represents a node with kind `EqualityExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// EqualityExpression = (* left_operand: *) Expression
    ///                      (* operator: *) EQUAL_EQUAL
    ///                      (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// EqualityExpression = (* left_operand: *) Expression
    ///                      (* operator: *) BANG_EQUAL
    ///                      (* right_operand: *) Expression;
    /// ```
    EqualityExpression,
    /// Represents a node with kind `ErrorDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// ErrorDefinition = (* error_keyword: *) ERROR_KEYWORD
    ///                   (* name: *) IDENTIFIER
    ///                   (* members: *) ErrorParametersDeclaration
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    ErrorDefinition,
    /// Represents a node with kind `ErrorParameter`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// ErrorParameter = (* type_name: *) TypeName
    ///                  (* name: *) IDENTIFIER?;
    /// ```
    ErrorParameter,
    /// Represents a node with kind `ErrorParameters`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// ErrorParameters = ((* item: *) ErrorParameter ((* separator: *) COMMA (* item: *) ErrorParameter)*)?;
    /// ```
    ErrorParameters,
    /// Represents a node with kind `ErrorParametersDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// ErrorParametersDeclaration = (* open_paren: *) OPEN_PAREN
    ///                              (* parameters: *) ErrorParameters
    ///                              (* close_paren: *) CLOSE_PAREN;
    /// ```
    ErrorParametersDeclaration,
    /// Represents a node with kind `EventDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// EventDefinition = (* event_keyword: *) EVENT_KEYWORD
    ///                   (* name: *) IDENTIFIER
    ///                   (* parameters: *) EventParametersDeclaration
    ///                   (* anonymous_keyword: *) ANONYMOUS_KEYWORD?
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    EventDefinition,
    /// Represents a node with kind `EventParameter`, having the following structure:
    ///
    /// ```ebnf
    /// EventParameter = (* type_name: *) TypeName
    ///                  (* indexed_keyword: *) INDEXED_KEYWORD?
    ///                  (* name: *) IDENTIFIER?;
    /// ```
    EventParameter,
    /// Represents a node with kind `EventParameters`, having the following structure:
    ///
    /// ```ebnf
    /// EventParameters = ((* item: *) EventParameter ((* separator: *) COMMA (* item: *) EventParameter)*)?;
    /// ```
    EventParameters,
    /// Represents a node with kind `EventParametersDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// EventParametersDeclaration = (* open_paren: *) OPEN_PAREN
    ///                              (* parameters: *) EventParameters
    ///                              (* close_paren: *) CLOSE_PAREN;
    /// ```
    EventParametersDeclaration,
    /// Represents a node with kind `ExperimentalFeature`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.16 *)
    /// ExperimentalFeature = (* variant: *) ABIENCODER_V2_KEYWORD
    ///                     | (* variant: *) SMTCHECKER_KEYWORD
    ///                     | (* variant: *) StringLiteral;
    /// ```
    ExperimentalFeature,
    /// Represents a node with kind `ExperimentalPragma`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.16 *)
    /// ExperimentalPragma = (* experimental_keyword: *) EXPERIMENTAL_KEYWORD
    ///                      (* feature: *) ExperimentalFeature;
    /// ```
    ExperimentalPragma,
    /// Represents a node with kind `ExponentiationExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// (* Deprecated in 0.8.0 *)
    /// ExponentiationExpression = (* left_operand: *) Expression
    ///                            (* operator: *) ASTERISK_ASTERISK
    ///                            (* right_operand: *) Expression;
    ///
    /// (* Right-associative binary operator *)
    /// (* Introduced in 0.8.0 *)
    /// ExponentiationExpression = (* left_operand: *) Expression
    ///                            (* operator: *) ASTERISK_ASTERISK
    ///                            (* right_operand: *) Expression;
    /// ```
    ExponentiationExpression,
    /// Represents a node with kind `Expression`, having the following structure:
    ///
    /// ```ebnf
    /// Expression = (* variant: *) AssignmentExpression
    ///            | (* variant: *) ConditionalExpression
    ///            | (* variant: *) OrExpression
    ///            | (* variant: *) AndExpression
    ///            | (* variant: *) EqualityExpression
    ///            | (* variant: *) InequalityExpression
    ///            | (* variant: *) BitwiseOrExpression
    ///            | (* variant: *) BitwiseXorExpression
    ///            | (* variant: *) BitwiseAndExpression
    ///            | (* variant: *) ShiftExpression
    ///            | (* variant: *) AdditiveExpression
    ///            | (* variant: *) MultiplicativeExpression
    ///            | (* variant: *) ExponentiationExpression
    ///            | (* variant: *) PostfixExpression
    ///            | (* variant: *) PrefixExpression
    ///            | (* variant: *) FunctionCallExpression
    ///            | (* variant: *) CallOptionsExpression
    ///            | (* variant: *) MemberAccessExpression
    ///            | (* variant: *) IndexAccessExpression
    ///            | (* variant: *) NewExpression
    ///            | (* variant: *) TupleExpression
    ///            | (* variant: *) TypeExpression (* Introduced in 0.5.3 *)
    ///            | (* variant: *) ArrayExpression
    ///            | (* variant: *) HexNumberExpression
    ///            | (* variant: *) DecimalNumberExpression
    ///            | (* variant: *) StringExpression
    ///            | (* variant: *) ElementaryType
    ///            | (* variant: *) PAYABLE_KEYWORD (* Introduced in 0.6.0 *)
    ///            | (* variant: *) THIS_KEYWORD
    ///            | (* variant: *) SUPER_KEYWORD
    ///            | (* variant: *) TRUE_KEYWORD
    ///            | (* variant: *) FALSE_KEYWORD
    ///            | (* variant: *) IDENTIFIER;
    /// ```
    Expression,
    /// Represents a node with kind `ExpressionStatement`, having the following structure:
    ///
    /// ```ebnf
    /// ExpressionStatement = (* expression: *) Expression
    ///                       (* semicolon: *) SEMICOLON;
    /// ```
    ExpressionStatement,
    /// Represents a node with kind `FallbackFunctionAttribute`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// FallbackFunctionAttribute = (* variant: *) ModifierInvocation
    ///                           | (* variant: *) OverrideSpecifier
    ///                           | (* variant: *) EXTERNAL_KEYWORD
    ///                           | (* variant: *) PAYABLE_KEYWORD
    ///                           | (* variant: *) PURE_KEYWORD
    ///                           | (* variant: *) VIEW_KEYWORD
    ///                           | (* variant: *) VIRTUAL_KEYWORD;
    /// ```
    FallbackFunctionAttribute,
    /// Represents a node with kind `FallbackFunctionAttributes`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// FallbackFunctionAttributes = (* item: *) FallbackFunctionAttribute*;
    /// ```
    FallbackFunctionAttributes,
    /// Represents a node with kind `FallbackFunctionDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// FallbackFunctionDefinition = (* fallback_keyword: *) FALLBACK_KEYWORD
    ///                              (* parameters: *) ParametersDeclaration
    ///                              (* attributes: *) FallbackFunctionAttributes
    ///                              (* returns: *) ReturnsDeclaration?
    ///                              (* body: *) FunctionBody;
    /// ```
    FallbackFunctionDefinition,
    /// Represents a node with kind `ForStatement`, having the following structure:
    ///
    /// ```ebnf
    /// ForStatement = (* for_keyword: *) FOR_KEYWORD
    ///                (* open_paren: *) OPEN_PAREN
    ///                (* initialization: *) ForStatementInitialization
    ///                (* condition: *) ForStatementCondition
    ///                (* iterator: *) Expression?
    ///                (* close_paren: *) CLOSE_PAREN
    ///                (* body: *) Statement;
    /// ```
    ForStatement,
    /// Represents a node with kind `ForStatementCondition`, having the following structure:
    ///
    /// ```ebnf
    /// ForStatementCondition = (* variant: *) ExpressionStatement
    ///                       | (* variant: *) SEMICOLON;
    /// ```
    ForStatementCondition,
    /// Represents a node with kind `ForStatementInitialization`, having the following structure:
    ///
    /// ```ebnf
    /// ForStatementInitialization = (* variant: *) TupleDeconstructionStatement
    ///                            | (* variant: *) VariableDeclarationStatement
    ///                            | (* variant: *) ExpressionStatement
    ///                            | (* variant: *) SEMICOLON;
    /// ```
    ForStatementInitialization,
    /// Represents a node with kind `FunctionAttribute`, having the following structure:
    ///
    /// ```ebnf
    /// FunctionAttribute = (* variant: *) ModifierInvocation
    ///                   | (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
    ///                   | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
    ///                   | (* variant: *) EXTERNAL_KEYWORD
    ///                   | (* variant: *) INTERNAL_KEYWORD
    ///                   | (* variant: *) PAYABLE_KEYWORD
    ///                   | (* variant: *) PRIVATE_KEYWORD
    ///                   | (* variant: *) PUBLIC_KEYWORD
    ///                   | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 *)
    ///                   | (* variant: *) VIEW_KEYWORD (* Introduced in 0.4.16 *)
    ///                   | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
    /// ```
    FunctionAttribute,
    /// Represents a node with kind `FunctionAttributes`, having the following structure:
    ///
    /// ```ebnf
    /// FunctionAttributes = (* item: *) FunctionAttribute*;
    /// ```
    FunctionAttributes,
    /// Represents a node with kind `FunctionBody`, having the following structure:
    ///
    /// ```ebnf
    /// FunctionBody = (* variant: *) Block
    ///              | (* variant: *) SEMICOLON;
    /// ```
    FunctionBody,
    /// Represents a node with kind `FunctionCallExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// FunctionCallExpression = (* operand: *) Expression
    ///                          (* arguments: *) ArgumentsDeclaration;
    /// ```
    FunctionCallExpression,
    /// Represents a node with kind `FunctionDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// FunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
    ///                      (* name: *) FunctionName
    ///                      (* parameters: *) ParametersDeclaration
    ///                      (* attributes: *) FunctionAttributes
    ///                      (* returns: *) ReturnsDeclaration?
    ///                      (* body: *) FunctionBody;
    /// ```
    FunctionDefinition,
    /// Represents a node with kind `FunctionName`, having the following structure:
    ///
    /// ```ebnf
    /// FunctionName = (* variant: *) IDENTIFIER
    ///              | (* variant: *) FALLBACK_KEYWORD
    ///              | (* variant: *) RECEIVE_KEYWORD;
    /// ```
    FunctionName,
    /// Represents a node with kind `FunctionType`, having the following structure:
    ///
    /// ```ebnf
    /// FunctionType = (* function_keyword: *) FUNCTION_KEYWORD
    ///                (* parameters: *) ParametersDeclaration
    ///                (* attributes: *) FunctionTypeAttributes
    ///                (* returns: *) ReturnsDeclaration?;
    /// ```
    FunctionType,
    /// Represents a node with kind `FunctionTypeAttribute`, having the following structure:
    ///
    /// ```ebnf
    /// FunctionTypeAttribute = (* variant: *) INTERNAL_KEYWORD
    ///                       | (* variant: *) EXTERNAL_KEYWORD
    ///                       | (* variant: *) PRIVATE_KEYWORD
    ///                       | (* variant: *) PUBLIC_KEYWORD
    ///                       | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
    ///                       | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 *)
    ///                       | (* variant: *) VIEW_KEYWORD (* Introduced in 0.4.16 *)
    ///                       | (* variant: *) PAYABLE_KEYWORD;
    /// ```
    FunctionTypeAttribute,
    /// Represents a node with kind `FunctionTypeAttributes`, having the following structure:
    ///
    /// ```ebnf
    /// FunctionTypeAttributes = (* item: *) FunctionTypeAttribute*;
    /// ```
    FunctionTypeAttributes,
    /// Represents a node with kind `HexNumberExpression`, having the following structure:
    ///
    /// ```ebnf
    /// HexNumberExpression = (* literal: *) HEX_LITERAL
    ///                       (* unit: *) NumberUnit?; (* Deprecated in 0.5.0 *)
    /// ```
    HexNumberExpression,
    /// Represents a node with kind `HexStringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// HexStringLiteral = (* variant: *) SINGLE_QUOTED_HEX_STRING_LITERAL
    ///                  | (* variant: *) DOUBLE_QUOTED_HEX_STRING_LITERAL;
    /// ```
    HexStringLiteral,
    /// Represents a node with kind `HexStringLiterals`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.5.14 *)
    /// HexStringLiterals = (* item: *) HexStringLiteral+;
    /// ```
    HexStringLiterals,
    /// Represents a node with kind `IdentifierPath`, having the following structure:
    ///
    /// ```ebnf
    /// IdentifierPath = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
    /// ```
    IdentifierPath,
    /// Represents a node with kind `IfStatement`, having the following structure:
    ///
    /// ```ebnf
    /// IfStatement = (* if_keyword: *) IF_KEYWORD
    ///               (* open_paren: *) OPEN_PAREN
    ///               (* condition: *) Expression
    ///               (* close_paren: *) CLOSE_PAREN
    ///               (* body: *) Statement
    ///               (* else_branch: *) ElseBranch?;
    /// ```
    IfStatement,
    /// Represents a node with kind `ImportAlias`, having the following structure:
    ///
    /// ```ebnf
    /// ImportAlias = (* as_keyword: *) AS_KEYWORD
    ///               (* identifier: *) IDENTIFIER;
    /// ```
    ImportAlias,
    /// Represents a node with kind `ImportClause`, having the following structure:
    ///
    /// ```ebnf
    /// ImportClause = (* variant: *) PathImport
    ///              | (* variant: *) NamedImport
    ///              | (* variant: *) ImportDeconstruction;
    /// ```
    ImportClause,
    /// Represents a node with kind `ImportDeconstruction`, having the following structure:
    ///
    /// ```ebnf
    /// ImportDeconstruction = (* open_brace: *) OPEN_BRACE
    ///                        (* symbols: *) ImportDeconstructionSymbols
    ///                        (* close_brace: *) CLOSE_BRACE
    ///                        (* from_keyword: *) FROM_KEYWORD
    ///                        (* path: *) StringLiteral;
    /// ```
    ImportDeconstruction,
    /// Represents a node with kind `ImportDeconstructionSymbol`, having the following structure:
    ///
    /// ```ebnf
    /// ImportDeconstructionSymbol = (* name: *) IDENTIFIER
    ///                              (* alias: *) ImportAlias?;
    /// ```
    ImportDeconstructionSymbol,
    /// Represents a node with kind `ImportDeconstructionSymbols`, having the following structure:
    ///
    /// ```ebnf
    /// ImportDeconstructionSymbols = (* item: *) ImportDeconstructionSymbol ((* separator: *) COMMA (* item: *) ImportDeconstructionSymbol)*;
    /// ```
    ImportDeconstructionSymbols,
    /// Represents a node with kind `ImportDirective`, having the following structure:
    ///
    /// ```ebnf
    /// ImportDirective = (* import_keyword: *) IMPORT_KEYWORD
    ///                   (* clause: *) ImportClause
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    ImportDirective,
    /// Represents a node with kind `IndexAccessEnd`, having the following structure:
    ///
    /// ```ebnf
    /// IndexAccessEnd = (* colon: *) COLON
    ///                  (* end: *) Expression?;
    /// ```
    IndexAccessEnd,
    /// Represents a node with kind `IndexAccessExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// IndexAccessExpression = (* operand: *) Expression
    ///                         (* open_bracket: *) OPEN_BRACKET
    ///                         (* start: *) Expression?
    ///                         (* end: *) IndexAccessEnd?
    ///                         (* close_bracket: *) CLOSE_BRACKET;
    /// ```
    IndexAccessExpression,
    /// Represents a node with kind `InequalityExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// InequalityExpression = (* left_operand: *) Expression
    ///                        (* operator: *) LESS_THAN
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// InequalityExpression = (* left_operand: *) Expression
    ///                        (* operator: *) GREATER_THAN
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// InequalityExpression = (* left_operand: *) Expression
    ///                        (* operator: *) LESS_THAN_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// InequalityExpression = (* left_operand: *) Expression
    ///                        (* operator: *) GREATER_THAN_EQUAL
    ///                        (* right_operand: *) Expression;
    /// ```
    InequalityExpression,
    /// Represents a node with kind `InheritanceSpecifier`, having the following structure:
    ///
    /// ```ebnf
    /// InheritanceSpecifier = (* is_keyword: *) IS_KEYWORD
    ///                        (* types: *) InheritanceTypes;
    /// ```
    InheritanceSpecifier,
    /// Represents a node with kind `InheritanceType`, having the following structure:
    ///
    /// ```ebnf
    /// InheritanceType = (* type_name: *) IdentifierPath
    ///                   (* arguments: *) ArgumentsDeclaration?;
    /// ```
    InheritanceType,
    /// Represents a node with kind `InheritanceTypes`, having the following structure:
    ///
    /// ```ebnf
    /// InheritanceTypes = (* item: *) InheritanceType ((* separator: *) COMMA (* item: *) InheritanceType)*;
    /// ```
    InheritanceTypes,
    /// Represents a node with kind `InterfaceDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// InterfaceDefinition = (* interface_keyword: *) INTERFACE_KEYWORD
    ///                       (* name: *) IDENTIFIER
    ///                       (* inheritance: *) InheritanceSpecifier?
    ///                       (* open_brace: *) OPEN_BRACE
    ///                       (* members: *) InterfaceMembers
    ///                       (* close_brace: *) CLOSE_BRACE;
    /// ```
    InterfaceDefinition,
    /// Represents a node with kind `InterfaceMembers`, having the following structure:
    ///
    /// ```ebnf
    /// InterfaceMembers = (* item: *) ContractMember*;
    /// ```
    InterfaceMembers,
    /// Represents a node with kind `LibraryDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// LibraryDefinition = (* library_keyword: *) LIBRARY_KEYWORD
    ///                     (* name: *) IDENTIFIER
    ///                     (* open_brace: *) OPEN_BRACE
    ///                     (* members: *) LibraryMembers
    ///                     (* close_brace: *) CLOSE_BRACE;
    /// ```
    LibraryDefinition,
    /// Represents a node with kind `LibraryMembers`, having the following structure:
    ///
    /// ```ebnf
    /// LibraryMembers = (* item: *) ContractMember*;
    /// ```
    LibraryMembers,
    /// Represents a node with kind `MappingKey`, having the following structure:
    ///
    /// ```ebnf
    /// MappingKey = (* key_type: *) MappingKeyType
    ///              (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
    /// ```
    MappingKey,
    /// Represents a node with kind `MappingKeyType`, having the following structure:
    ///
    /// ```ebnf
    /// MappingKeyType = (* variant: *) ElementaryType
    ///                | (* variant: *) IdentifierPath;
    /// ```
    MappingKeyType,
    /// Represents a node with kind `MappingType`, having the following structure:
    ///
    /// ```ebnf
    /// MappingType = (* mapping_keyword: *) MAPPING_KEYWORD
    ///               (* open_paren: *) OPEN_PAREN
    ///               (* key_type: *) MappingKey
    ///               (* equal_greater_than: *) EQUAL_GREATER_THAN
    ///               (* value_type: *) MappingValue
    ///               (* close_paren: *) CLOSE_PAREN;
    /// ```
    MappingType,
    /// Represents a node with kind `MappingValue`, having the following structure:
    ///
    /// ```ebnf
    /// MappingValue = (* type_name: *) TypeName
    ///                (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
    /// ```
    MappingValue,
    /// Represents a node with kind `MemberAccessExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// MemberAccessExpression = (* operand: *) Expression
    ///                          (* period: *) PERIOD
    ///                          (* member: *) IDENTIFIER;
    /// ```
    MemberAccessExpression,
    /// Represents a node with kind `ModifierAttribute`, having the following structure:
    ///
    /// ```ebnf
    /// ModifierAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
    ///                   | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
    /// ```
    ModifierAttribute,
    /// Represents a node with kind `ModifierAttributes`, having the following structure:
    ///
    /// ```ebnf
    /// ModifierAttributes = (* item: *) ModifierAttribute*;
    /// ```
    ModifierAttributes,
    /// Represents a node with kind `ModifierDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// ModifierDefinition = (* modifier_keyword: *) MODIFIER_KEYWORD
    ///                      (* name: *) IDENTIFIER
    ///                      (* parameters: *) ParametersDeclaration?
    ///                      (* attributes: *) ModifierAttributes
    ///                      (* body: *) FunctionBody;
    /// ```
    ModifierDefinition,
    /// Represents a node with kind `ModifierInvocation`, having the following structure:
    ///
    /// ```ebnf
    /// ModifierInvocation = (* name: *) IdentifierPath
    ///                      (* arguments: *) ArgumentsDeclaration?;
    /// ```
    ModifierInvocation,
    /// Represents a node with kind `MultiplicativeExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// MultiplicativeExpression = (* left_operand: *) Expression
    ///                            (* operator: *) ASTERISK
    ///                            (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// MultiplicativeExpression = (* left_operand: *) Expression
    ///                            (* operator: *) SLASH
    ///                            (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// MultiplicativeExpression = (* left_operand: *) Expression
    ///                            (* operator: *) PERCENT
    ///                            (* right_operand: *) Expression;
    /// ```
    MultiplicativeExpression,
    /// Represents a node with kind `NamedArgument`, having the following structure:
    ///
    /// ```ebnf
    /// NamedArgument = (* name: *) IDENTIFIER
    ///                 (* colon: *) COLON
    ///                 (* value: *) Expression;
    /// ```
    NamedArgument,
    /// Represents a node with kind `NamedArgumentGroup`, having the following structure:
    ///
    /// ```ebnf
    /// NamedArgumentGroup = (* open_brace: *) OPEN_BRACE
    ///                      (* arguments: *) NamedArguments
    ///                      (* close_brace: *) CLOSE_BRACE;
    /// ```
    NamedArgumentGroup,
    /// Represents a node with kind `NamedArguments`, having the following structure:
    ///
    /// ```ebnf
    /// NamedArguments = ((* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*)?;
    /// ```
    NamedArguments,
    /// Represents a node with kind `NamedArgumentsDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// NamedArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
    ///                             (* arguments: *) NamedArgumentGroup?
    ///                             (* close_paren: *) CLOSE_PAREN;
    /// ```
    NamedArgumentsDeclaration,
    /// Represents a node with kind `NamedImport`, having the following structure:
    ///
    /// ```ebnf
    /// NamedImport = (* asterisk: *) ASTERISK
    ///               (* alias: *) ImportAlias
    ///               (* from_keyword: *) FROM_KEYWORD
    ///               (* path: *) StringLiteral;
    /// ```
    NamedImport,
    /// Represents a node with kind `NewExpression`, having the following structure:
    ///
    /// ```ebnf
    /// NewExpression = (* new_keyword: *) NEW_KEYWORD
    ///                 (* type_name: *) TypeName;
    /// ```
    NewExpression,
    /// Represents a node with kind `NumberUnit`, having the following structure:
    ///
    /// ```ebnf
    /// NumberUnit = (* variant: *) WEI_KEYWORD
    ///            | (* variant: *) GWEI_KEYWORD (* Introduced in 0.6.11 *)
    ///            | (* variant: *) SZABO_KEYWORD (* Deprecated in 0.7.0 *)
    ///            | (* variant: *) FINNEY_KEYWORD (* Deprecated in 0.7.0 *)
    ///            | (* variant: *) ETHER_KEYWORD
    ///            | (* variant: *) SECONDS_KEYWORD
    ///            | (* variant: *) MINUTES_KEYWORD
    ///            | (* variant: *) HOURS_KEYWORD
    ///            | (* variant: *) DAYS_KEYWORD
    ///            | (* variant: *) WEEKS_KEYWORD
    ///            | (* variant: *) YEARS_KEYWORD; (* Deprecated in 0.5.0 *)
    /// ```
    NumberUnit,
    /// Represents a node with kind `OrExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// OrExpression = (* left_operand: *) Expression
    ///                (* operator: *) BAR_BAR
    ///                (* right_operand: *) Expression;
    /// ```
    OrExpression,
    /// Represents a node with kind `OverridePaths`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// OverridePaths = (* item: *) IdentifierPath ((* separator: *) COMMA (* item: *) IdentifierPath)*;
    /// ```
    OverridePaths,
    /// Represents a node with kind `OverridePathsDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// OverridePathsDeclaration = (* open_paren: *) OPEN_PAREN
    ///                            (* paths: *) OverridePaths
    ///                            (* close_paren: *) CLOSE_PAREN;
    /// ```
    OverridePathsDeclaration,
    /// Represents a node with kind `OverrideSpecifier`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// OverrideSpecifier = (* override_keyword: *) OVERRIDE_KEYWORD
    ///                     (* overridden: *) OverridePathsDeclaration?;
    /// ```
    OverrideSpecifier,
    /// Represents a node with kind `Parameter`, having the following structure:
    ///
    /// ```ebnf
    /// Parameter = (* type_name: *) TypeName
    ///             (* storage_location: *) StorageLocation?
    ///             (* name: *) IDENTIFIER?;
    /// ```
    Parameter,
    /// Represents a node with kind `Parameters`, having the following structure:
    ///
    /// ```ebnf
    /// Parameters = ((* item: *) Parameter ((* separator: *) COMMA (* item: *) Parameter)*)?;
    /// ```
    Parameters,
    /// Represents a node with kind `ParametersDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// ParametersDeclaration = (* open_paren: *) OPEN_PAREN
    ///                         (* parameters: *) Parameters
    ///                         (* close_paren: *) CLOSE_PAREN;
    /// ```
    ParametersDeclaration,
    /// Represents a node with kind `PathImport`, having the following structure:
    ///
    /// ```ebnf
    /// PathImport = (* path: *) StringLiteral
    ///              (* alias: *) ImportAlias?;
    /// ```
    PathImport,
    /// Represents a node with kind `PositionalArguments`, having the following structure:
    ///
    /// ```ebnf
    /// PositionalArguments = ((* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*)?;
    /// ```
    PositionalArguments,
    /// Represents a node with kind `PositionalArgumentsDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// PositionalArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
    ///                                  (* arguments: *) PositionalArguments
    ///                                  (* close_paren: *) CLOSE_PAREN;
    /// ```
    PositionalArgumentsDeclaration,
    /// Represents a node with kind `PostfixExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// PostfixExpression = (* operand: *) Expression
    ///                     (* operator: *) PLUS_PLUS;
    ///
    /// (* Postfix unary operator *)
    /// PostfixExpression = (* operand: *) Expression
    ///                     (* operator: *) MINUS_MINUS;
    /// ```
    PostfixExpression,
    /// Represents a node with kind `Pragma`, having the following structure:
    ///
    /// ```ebnf
    /// Pragma = (* variant: *) VersionPragma
    ///        | (* variant: *) AbicoderPragma (* Introduced in 0.7.5 *)
    ///        | (* variant: *) ExperimentalPragma; (* Introduced in 0.4.16 *)
    /// ```
    Pragma,
    /// Represents a node with kind `PragmaDirective`, having the following structure:
    ///
    /// ```ebnf
    /// PragmaDirective = (* pragma_keyword: *) PRAGMA_KEYWORD
    ///                   (* pragma: *) Pragma
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    PragmaDirective,
    /// Represents a node with kind `PrefixExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Prefix unary operator *)
    /// PrefixExpression = (* operator: *) PLUS_PLUS
    ///                    (* operand: *) Expression;
    ///
    /// (* Prefix unary operator *)
    /// PrefixExpression = (* operator: *) MINUS_MINUS
    ///                    (* operand: *) Expression;
    ///
    /// (* Prefix unary operator *)
    /// PrefixExpression = (* operator: *) TILDE
    ///                    (* operand: *) Expression;
    ///
    /// (* Prefix unary operator *)
    /// PrefixExpression = (* operator: *) BANG
    ///                    (* operand: *) Expression;
    ///
    /// (* Prefix unary operator *)
    /// PrefixExpression = (* operator: *) MINUS
    ///                    (* operand: *) Expression;
    ///
    /// (* Prefix unary operator *)
    /// (* Deprecated in 0.5.0 *)
    /// PrefixExpression = (* operator: *) PLUS
    ///                    (* operand: *) Expression;
    ///
    /// (* Prefix unary operator *)
    /// PrefixExpression = (* operator: *) DELETE_KEYWORD
    ///                    (* operand: *) Expression;
    /// ```
    PrefixExpression,
    /// Represents a node with kind `ReceiveFunctionAttribute`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// ReceiveFunctionAttribute = (* variant: *) ModifierInvocation
    ///                          | (* variant: *) OverrideSpecifier
    ///                          | (* variant: *) EXTERNAL_KEYWORD
    ///                          | (* variant: *) PAYABLE_KEYWORD
    ///                          | (* variant: *) VIRTUAL_KEYWORD;
    /// ```
    ReceiveFunctionAttribute,
    /// Represents a node with kind `ReceiveFunctionAttributes`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// ReceiveFunctionAttributes = (* item: *) ReceiveFunctionAttribute*;
    /// ```
    ReceiveFunctionAttributes,
    /// Represents a node with kind `ReceiveFunctionDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// ReceiveFunctionDefinition = (* receive_keyword: *) RECEIVE_KEYWORD
    ///                             (* parameters: *) ParametersDeclaration
    ///                             (* attributes: *) ReceiveFunctionAttributes
    ///                             (* body: *) FunctionBody;
    /// ```
    ReceiveFunctionDefinition,
    /// Represents a node with kind `ReturnStatement`, having the following structure:
    ///
    /// ```ebnf
    /// ReturnStatement = (* return_keyword: *) RETURN_KEYWORD
    ///                   (* expression: *) Expression?
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    ReturnStatement,
    /// Represents a node with kind `ReturnsDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// ReturnsDeclaration = (* returns_keyword: *) RETURNS_KEYWORD
    ///                      (* variables: *) ParametersDeclaration;
    /// ```
    ReturnsDeclaration,
    /// Represents a node with kind `RevertStatement`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// RevertStatement = (* revert_keyword: *) REVERT_KEYWORD
    ///                   (* error: *) IdentifierPath?
    ///                   (* arguments: *) ArgumentsDeclaration
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    RevertStatement,
    /// Represents a node with kind `ShiftExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// ShiftExpression = (* left_operand: *) Expression
    ///                   (* operator: *) LESS_THAN_LESS_THAN
    ///                   (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// ShiftExpression = (* left_operand: *) Expression
    ///                   (* operator: *) GREATER_THAN_GREATER_THAN
    ///                   (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// ShiftExpression = (* left_operand: *) Expression
    ///                   (* operator: *) GREATER_THAN_GREATER_THAN_GREATER_THAN
    ///                   (* right_operand: *) Expression;
    /// ```
    ShiftExpression,
    /// Represents a node with kind `SimpleVersionLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// SimpleVersionLiteral = (* item: *) VERSION_SPECIFIER ((* separator: *) PERIOD (* item: *) VERSION_SPECIFIER)*;
    /// ```
    SimpleVersionLiteral,
    /// Represents a node with kind `SourceUnit`, having the following structure:
    ///
    /// ```ebnf
    /// SourceUnit = (* members: *) SourceUnitMembers;
    /// ```
    SourceUnit,
    /// Represents a node with kind `SourceUnitMember`, having the following structure:
    ///
    /// ```ebnf
    /// SourceUnitMember = (* variant: *) PragmaDirective
    ///                  | (* variant: *) ImportDirective
    ///                  | (* variant: *) ContractDefinition
    ///                  | (* variant: *) InterfaceDefinition
    ///                  | (* variant: *) LibraryDefinition
    ///                  | (* variant: *) StructDefinition (* Introduced in 0.6.0 *)
    ///                  | (* variant: *) EnumDefinition (* Introduced in 0.6.0 *)
    ///                  | (* variant: *) FunctionDefinition (* Introduced in 0.7.1 *)
    ///                  | (* variant: *) ErrorDefinition (* Introduced in 0.8.4 *)
    ///                  | (* variant: *) UserDefinedValueTypeDefinition (* Introduced in 0.8.8 *)
    ///                  | (* variant: *) UsingDirective (* Introduced in 0.8.13 *)
    ///                  | (* variant: *) EventDefinition (* Introduced in 0.8.22 *)
    ///                  | (* variant: *) ConstantDefinition; (* Introduced in 0.7.4 *)
    /// ```
    SourceUnitMember,
    /// Represents a node with kind `SourceUnitMembers`, having the following structure:
    ///
    /// ```ebnf
    /// SourceUnitMembers = (* item: *) SourceUnitMember*;
    /// ```
    SourceUnitMembers,
    /// Represents a node with kind `StateVariableAttribute`, having the following structure:
    ///
    /// ```ebnf
    /// StateVariableAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
    ///                        | (* variant: *) CONSTANT_KEYWORD
    ///                        | (* variant: *) INTERNAL_KEYWORD
    ///                        | (* variant: *) PRIVATE_KEYWORD
    ///                        | (* variant: *) PUBLIC_KEYWORD
    ///                        | (* variant: *) IMMUTABLE_KEYWORD (* Introduced in 0.6.5 *)
    ///                        | (* variant: *) TRANSIENT_KEYWORD; (* Introduced in 0.8.27 *)
    /// ```
    StateVariableAttribute,
    /// Represents a node with kind `StateVariableAttributes`, having the following structure:
    ///
    /// ```ebnf
    /// StateVariableAttributes = (* item: *) StateVariableAttribute*;
    /// ```
    StateVariableAttributes,
    /// Represents a node with kind `StateVariableDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// StateVariableDefinition = (* type_name: *) TypeName
    ///                           (* attributes: *) StateVariableAttributes
    ///                           (* name: *) IDENTIFIER
    ///                           (* value: *) StateVariableDefinitionValue?
    ///                           (* semicolon: *) SEMICOLON;
    /// ```
    StateVariableDefinition,
    /// Represents a node with kind `StateVariableDefinitionValue`, having the following structure:
    ///
    /// ```ebnf
    /// StateVariableDefinitionValue = (* equal: *) EQUAL
    ///                                (* value: *) Expression;
    /// ```
    StateVariableDefinitionValue,
    /// Represents a node with kind `Statement`, having the following structure:
    ///
    /// ```ebnf
    /// Statement = (* variant: *) IfStatement
    ///           | (* variant: *) ForStatement
    ///           | (* variant: *) WhileStatement
    ///           | (* variant: *) DoWhileStatement
    ///           | (* variant: *) ContinueStatement
    ///           | (* variant: *) BreakStatement
    ///           | (* variant: *) ReturnStatement
    ///           | (* variant: *) ThrowStatement (* Deprecated in 0.5.0 *)
    ///           | (* variant: *) EmitStatement (* Introduced in 0.4.21 *)
    ///           | (* variant: *) TryStatement (* Introduced in 0.6.0 *)
    ///           | (* variant: *) RevertStatement (* Introduced in 0.8.4 *)
    ///           | (* variant: *) AssemblyStatement
    ///           | (* variant: *) Block
    ///           | (* variant: *) UncheckedBlock (* Introduced in 0.8.0 *)
    ///           | (* variant: *) TupleDeconstructionStatement
    ///           | (* variant: *) VariableDeclarationStatement
    ///           | (* variant: *) ExpressionStatement;
    /// ```
    Statement,
    /// Represents a node with kind `Statements`, having the following structure:
    ///
    /// ```ebnf
    /// Statements = (* item: *) Statement*;
    /// ```
    Statements,
    /// Represents a node with kind `StorageLayoutSpecifier`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.29 *)
    /// StorageLayoutSpecifier = (* layout_keyword: *) LAYOUT_KEYWORD
    ///                          (* at_keyword: *) AT_KEYWORD
    ///                          (* expression: *) Expression;
    /// ```
    StorageLayoutSpecifier,
    /// Represents a node with kind `StorageLocation`, having the following structure:
    ///
    /// ```ebnf
    /// StorageLocation = (* variant: *) MEMORY_KEYWORD
    ///                 | (* variant: *) STORAGE_KEYWORD
    ///                 | (* variant: *) CALL_DATA_KEYWORD; (* Introduced in 0.5.0 *)
    /// ```
    StorageLocation,
    /// Represents a node with kind `StringExpression`, having the following structure:
    ///
    /// ```ebnf
    /// StringExpression = (* variant: *) StringLiteral (* Deprecated in 0.5.14 *)
    ///                  | (* variant: *) StringLiterals (* Introduced in 0.5.14 *)
    ///                  | (* variant: *) HexStringLiteral (* Deprecated in 0.5.14 *)
    ///                  | (* variant: *) HexStringLiterals (* Introduced in 0.5.14 *)
    ///                  | (* variant: *) UnicodeStringLiterals; (* Introduced in 0.7.0 *)
    /// ```
    StringExpression,
    /// Represents a node with kind `StringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// StringLiteral = (* variant: *) SINGLE_QUOTED_STRING_LITERAL
    ///               | (* variant: *) DOUBLE_QUOTED_STRING_LITERAL;
    /// ```
    StringLiteral,
    /// Represents a node with kind `StringLiterals`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.5.14 *)
    /// StringLiterals = (* item: *) StringLiteral+;
    /// ```
    StringLiterals,
    /// Represents a node with kind `StructDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// StructDefinition = (* struct_keyword: *) STRUCT_KEYWORD
    ///                    (* name: *) IDENTIFIER
    ///                    (* open_brace: *) OPEN_BRACE
    ///                    (* members: *) StructMembers
    ///                    (* close_brace: *) CLOSE_BRACE;
    /// ```
    StructDefinition,
    /// Represents a node with kind `StructMember`, having the following structure:
    ///
    /// ```ebnf
    /// StructMember = (* type_name: *) TypeName
    ///                (* name: *) IDENTIFIER
    ///                (* semicolon: *) SEMICOLON;
    /// ```
    StructMember,
    /// Represents a node with kind `StructMembers`, having the following structure:
    ///
    /// ```ebnf
    /// StructMembers = (* item: *) StructMember*;
    /// ```
    StructMembers,
    /// Represents a node with kind `ThrowStatement`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// ThrowStatement = (* throw_keyword: *) THROW_KEYWORD
    ///                  (* semicolon: *) SEMICOLON;
    /// ```
    ThrowStatement,
    /// Represents a node with kind `TryStatement`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// TryStatement = (* try_keyword: *) TRY_KEYWORD
    ///                (* expression: *) Expression
    ///                (* returns: *) ReturnsDeclaration?
    ///                (* body: *) Block
    ///                (* catch_clauses: *) CatchClauses;
    /// ```
    TryStatement,
    /// Represents a node with kind `TupleDeconstructionElement`, having the following structure:
    ///
    /// ```ebnf
    /// TupleDeconstructionElement = (* member: *) TupleMember?;
    /// ```
    TupleDeconstructionElement,
    /// Represents a node with kind `TupleDeconstructionElements`, having the following structure:
    ///
    /// ```ebnf
    /// TupleDeconstructionElements = (* item: *) TupleDeconstructionElement ((* separator: *) COMMA (* item: *) TupleDeconstructionElement)*;
    /// ```
    TupleDeconstructionElements,
    /// Represents a node with kind `TupleDeconstructionStatement`, having the following structure:
    ///
    /// ```ebnf
    /// TupleDeconstructionStatement = (* var_keyword: *) VAR_KEYWORD? (* Deprecated in 0.5.0 *)
    ///                                (* open_paren: *) OPEN_PAREN
    ///                                (* elements: *) TupleDeconstructionElements
    ///                                (* close_paren: *) CLOSE_PAREN
    ///                                (* equal: *) EQUAL
    ///                                (* expression: *) Expression
    ///                                (* semicolon: *) SEMICOLON;
    /// ```
    TupleDeconstructionStatement,
    /// Represents a node with kind `TupleExpression`, having the following structure:
    ///
    /// ```ebnf
    /// TupleExpression = (* open_paren: *) OPEN_PAREN
    ///                   (* items: *) TupleValues
    ///                   (* close_paren: *) CLOSE_PAREN;
    /// ```
    TupleExpression,
    /// Represents a node with kind `TupleMember`, having the following structure:
    ///
    /// ```ebnf
    /// TupleMember = (* variant: *) TypedTupleMember
    ///             | (* variant: *) UntypedTupleMember;
    /// ```
    TupleMember,
    /// Represents a node with kind `TupleValue`, having the following structure:
    ///
    /// ```ebnf
    /// TupleValue = (* expression: *) Expression?;
    /// ```
    TupleValue,
    /// Represents a node with kind `TupleValues`, having the following structure:
    ///
    /// ```ebnf
    /// TupleValues = (* item: *) TupleValue ((* separator: *) COMMA (* item: *) TupleValue)*;
    /// ```
    TupleValues,
    /// Represents a node with kind `TypeExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.5.3 *)
    /// TypeExpression = (* type_keyword: *) TYPE_KEYWORD
    ///                  (* open_paren: *) OPEN_PAREN
    ///                  (* type_name: *) TypeName
    ///                  (* close_paren: *) CLOSE_PAREN;
    /// ```
    TypeExpression,
    /// Represents a node with kind `TypeName`, having the following structure:
    ///
    /// ```ebnf
    /// TypeName = (* variant: *) ArrayTypeName
    ///          | (* variant: *) FunctionType
    ///          | (* variant: *) MappingType
    ///          | (* variant: *) ElementaryType
    ///          | (* variant: *) IdentifierPath;
    /// ```
    TypeName,
    /// Represents a node with kind `TypedTupleMember`, having the following structure:
    ///
    /// ```ebnf
    /// TypedTupleMember = (* type_name: *) TypeName
    ///                    (* storage_location: *) StorageLocation?
    ///                    (* name: *) IDENTIFIER;
    /// ```
    TypedTupleMember,
    /// Represents a node with kind `UncheckedBlock`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.0 *)
    /// UncheckedBlock = (* unchecked_keyword: *) UNCHECKED_KEYWORD
    ///                  (* block: *) Block;
    /// ```
    UncheckedBlock,
    /// Represents a node with kind `UnicodeStringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.0 *)
    /// UnicodeStringLiteral = (* variant: *) SINGLE_QUOTED_UNICODE_STRING_LITERAL
    ///                      | (* variant: *) DOUBLE_QUOTED_UNICODE_STRING_LITERAL;
    /// ```
    UnicodeStringLiteral,
    /// Represents a node with kind `UnicodeStringLiterals`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.0 *)
    /// UnicodeStringLiterals = (* item: *) UnicodeStringLiteral+;
    /// ```
    UnicodeStringLiterals,
    /// Represents a node with kind `UnnamedFunctionAttribute`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.6.0 *)
    /// UnnamedFunctionAttribute = (* variant: *) ModifierInvocation
    ///                          | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
    ///                          | (* variant: *) EXTERNAL_KEYWORD
    ///                          | (* variant: *) INTERNAL_KEYWORD (* Deprecated in 0.5.0 *)
    ///                          | (* variant: *) PAYABLE_KEYWORD
    ///                          | (* variant: *) PRIVATE_KEYWORD (* Deprecated in 0.5.0 *)
    ///                          | (* variant: *) PUBLIC_KEYWORD (* Deprecated in 0.5.0 *)
    ///                          | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 and deprecated in 0.6.0. *)
    ///                          | (* variant: *) VIEW_KEYWORD; (* Introduced in 0.4.16 and deprecated in 0.6.0. *)
    /// ```
    UnnamedFunctionAttribute,
    /// Represents a node with kind `UnnamedFunctionAttributes`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.6.0 *)
    /// UnnamedFunctionAttributes = (* item: *) UnnamedFunctionAttribute*;
    /// ```
    UnnamedFunctionAttributes,
    /// Represents a node with kind `UnnamedFunctionDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.6.0 *)
    /// UnnamedFunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
    ///                             (* parameters: *) ParametersDeclaration
    ///                             (* attributes: *) UnnamedFunctionAttributes
    ///                             (* body: *) FunctionBody;
    /// ```
    UnnamedFunctionDefinition,
    /// Represents a node with kind `UntypedTupleMember`, having the following structure:
    ///
    /// ```ebnf
    /// UntypedTupleMember = (* storage_location: *) StorageLocation?
    ///                      (* name: *) IDENTIFIER;
    /// ```
    UntypedTupleMember,
    /// Represents a node with kind `UserDefinedValueTypeDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.8 *)
    /// UserDefinedValueTypeDefinition = (* type_keyword: *) TYPE_KEYWORD
    ///                                  (* name: *) IDENTIFIER
    ///                                  (* is_keyword: *) IS_KEYWORD
    ///                                  (* value_type: *) ElementaryType
    ///                                  (* semicolon: *) SEMICOLON;
    /// ```
    UserDefinedValueTypeDefinition,
    /// Represents a node with kind `UsingAlias`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.19 *)
    /// UsingAlias = (* as_keyword: *) AS_KEYWORD
    ///              (* operator: *) UsingOperator;
    /// ```
    UsingAlias,
    /// Represents a node with kind `UsingClause`, having the following structure:
    ///
    /// ```ebnf
    /// UsingClause = (* variant: *) IdentifierPath
    ///             | (* variant: *) UsingDeconstruction; (* Introduced in 0.8.13 *)
    /// ```
    UsingClause,
    /// Represents a node with kind `UsingDeconstruction`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// UsingDeconstruction = (* open_brace: *) OPEN_BRACE
    ///                       (* symbols: *) UsingDeconstructionSymbols
    ///                       (* close_brace: *) CLOSE_BRACE;
    /// ```
    UsingDeconstruction,
    /// Represents a node with kind `UsingDeconstructionSymbol`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// UsingDeconstructionSymbol = (* name: *) IdentifierPath
    ///                             (* alias: *) UsingAlias?; (* Introduced in 0.8.19 *)
    /// ```
    UsingDeconstructionSymbol,
    /// Represents a node with kind `UsingDeconstructionSymbols`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// UsingDeconstructionSymbols = (* item: *) UsingDeconstructionSymbol ((* separator: *) COMMA (* item: *) UsingDeconstructionSymbol)*;
    /// ```
    UsingDeconstructionSymbols,
    /// Represents a node with kind `UsingDirective`, having the following structure:
    ///
    /// ```ebnf
    /// UsingDirective = (* using_keyword: *) USING_KEYWORD
    ///                  (* clause: *) UsingClause
    ///                  (* for_keyword: *) FOR_KEYWORD
    ///                  (* target: *) UsingTarget
    ///                  (* global_keyword: *) GLOBAL_KEYWORD? (* Introduced in 0.8.13 *)
    ///                  (* semicolon: *) SEMICOLON;
    /// ```
    UsingDirective,
    /// Represents a node with kind `UsingOperator`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.19 *)
    /// UsingOperator = (* variant: *) AMPERSAND
    ///               | (* variant: *) ASTERISK
    ///               | (* variant: *) BANG_EQUAL
    ///               | (* variant: *) BAR
    ///               | (* variant: *) CARET
    ///               | (* variant: *) EQUAL_EQUAL
    ///               | (* variant: *) GREATER_THAN
    ///               | (* variant: *) GREATER_THAN_EQUAL
    ///               | (* variant: *) LESS_THAN
    ///               | (* variant: *) LESS_THAN_EQUAL
    ///               | (* variant: *) MINUS
    ///               | (* variant: *) PERCENT
    ///               | (* variant: *) PLUS
    ///               | (* variant: *) SLASH
    ///               | (* variant: *) TILDE;
    /// ```
    UsingOperator,
    /// Represents a node with kind `UsingTarget`, having the following structure:
    ///
    /// ```ebnf
    /// UsingTarget = (* variant: *) TypeName
    ///             | (* variant: *) ASTERISK;
    /// ```
    UsingTarget,
    /// Represents a node with kind `VariableDeclarationStatement`, having the following structure:
    ///
    /// ```ebnf
    /// VariableDeclarationStatement = (* variable_type: *) VariableDeclarationType
    ///                                (* storage_location: *) StorageLocation?
    ///                                (* name: *) IDENTIFIER
    ///                                (* value: *) VariableDeclarationValue?
    ///                                (* semicolon: *) SEMICOLON;
    /// ```
    VariableDeclarationStatement,
    /// Represents a node with kind `VariableDeclarationType`, having the following structure:
    ///
    /// ```ebnf
    /// VariableDeclarationType = (* variant: *) TypeName
    ///                         | (* variant: *) VAR_KEYWORD; (* Deprecated in 0.5.0 *)
    /// ```
    VariableDeclarationType,
    /// Represents a node with kind `VariableDeclarationValue`, having the following structure:
    ///
    /// ```ebnf
    /// VariableDeclarationValue = (* equal: *) EQUAL
    ///                            (* expression: *) Expression;
    /// ```
    VariableDeclarationValue,
    /// Represents a node with kind `VersionExpression`, having the following structure:
    ///
    /// ```ebnf
    /// VersionExpression = (* variant: *) VersionRange
    ///                   | (* variant: *) VersionTerm;
    /// ```
    VersionExpression,
    /// Represents a node with kind `VersionExpressionSet`, having the following structure:
    ///
    /// ```ebnf
    /// VersionExpressionSet = (* item: *) VersionExpression+;
    /// ```
    VersionExpressionSet,
    /// Represents a node with kind `VersionExpressionSets`, having the following structure:
    ///
    /// ```ebnf
    /// VersionExpressionSets = (* item: *) VersionExpressionSet ((* separator: *) BAR_BAR (* item: *) VersionExpressionSet)*;
    /// ```
    VersionExpressionSets,
    /// Represents a node with kind `VersionLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// VersionLiteral = (* variant: *) SimpleVersionLiteral
    ///                | (* variant: *) SINGLE_QUOTED_VERSION_LITERAL
    ///                | (* variant: *) DOUBLE_QUOTED_VERSION_LITERAL;
    /// ```
    VersionLiteral,
    /// Represents a node with kind `VersionOperator`, having the following structure:
    ///
    /// ```ebnf
    /// VersionOperator = (* variant: *) CARET
    ///                 | (* variant: *) TILDE
    ///                 | (* variant: *) EQUAL
    ///                 | (* variant: *) LESS_THAN
    ///                 | (* variant: *) GREATER_THAN
    ///                 | (* variant: *) LESS_THAN_EQUAL
    ///                 | (* variant: *) GREATER_THAN_EQUAL;
    /// ```
    VersionOperator,
    /// Represents a node with kind `VersionPragma`, having the following structure:
    ///
    /// ```ebnf
    /// VersionPragma = (* solidity_keyword: *) SOLIDITY_KEYWORD
    ///                 (* sets: *) VersionExpressionSets;
    /// ```
    VersionPragma,
    /// Represents a node with kind `VersionRange`, having the following structure:
    ///
    /// ```ebnf
    /// VersionRange = (* start: *) VersionLiteral
    ///                (* minus: *) MINUS
    ///                (* end: *) VersionLiteral;
    /// ```
    VersionRange,
    /// Represents a node with kind `VersionTerm`, having the following structure:
    ///
    /// ```ebnf
    /// VersionTerm = (* operator: *) VersionOperator?
    ///               (* literal: *) VersionLiteral;
    /// ```
    VersionTerm,
    /// Represents a node with kind `WhileStatement`, having the following structure:
    ///
    /// ```ebnf
    /// WhileStatement = (* while_keyword: *) WHILE_KEYWORD
    ///                  (* open_paren: *) OPEN_PAREN
    ///                  (* condition: *) Expression
    ///                  (* close_paren: *) CLOSE_PAREN
    ///                  (* body: *) Statement;
    /// ```
    WhileStatement,
    /// Represents a node with kind `YulArguments`, having the following structure:
    ///
    /// ```ebnf
    /// YulArguments = ((* item: *) YulExpression ((* separator: *) COMMA (* item: *) YulExpression)*)?;
    /// ```
    YulArguments,
    /// Represents a node with kind `YulAssignmentOperator`, having the following structure:
    ///
    /// ```ebnf
    /// YulAssignmentOperator = (* variant: *) COLON_EQUAL
    ///                       | (* variant: *) YulColonAndEqual; (* Deprecated in 0.5.5 *)
    /// ```
    YulAssignmentOperator,
    /// Represents a node with kind `YulBlock`, having the following structure:
    ///
    /// ```ebnf
    /// YulBlock = (* open_brace: *) OPEN_BRACE
    ///            (* statements: *) YulStatements
    ///            (* close_brace: *) CLOSE_BRACE;
    /// ```
    YulBlock,
    /// Represents a node with kind `YulBreakStatement`, having the following structure:
    ///
    /// ```ebnf
    /// YulBreakStatement = (* break_keyword: *) YUL_BREAK_KEYWORD;
    /// ```
    YulBreakStatement,
    /// Represents a node with kind `YulColonAndEqual`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.5 *)
    /// YulColonAndEqual = (* colon: *) COLON
    ///                    (* equal: *) EQUAL;
    /// ```
    YulColonAndEqual,
    /// Represents a node with kind `YulContinueStatement`, having the following structure:
    ///
    /// ```ebnf
    /// YulContinueStatement = (* continue_keyword: *) YUL_CONTINUE_KEYWORD;
    /// ```
    YulContinueStatement,
    /// Represents a node with kind `YulDefaultCase`, having the following structure:
    ///
    /// ```ebnf
    /// YulDefaultCase = (* default_keyword: *) YUL_DEFAULT_KEYWORD
    ///                  (* body: *) YulBlock;
    /// ```
    YulDefaultCase,
    /// Represents a node with kind `YulEqualAndColon`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YulEqualAndColon = (* equal: *) EQUAL
    ///                    (* colon: *) COLON;
    /// ```
    YulEqualAndColon,
    /// Represents a node with kind `YulExpression`, having the following structure:
    ///
    /// ```ebnf
    /// YulExpression = (* variant: *) YulFunctionCallExpression
    ///               | (* variant: *) YulLiteral
    ///               | (* variant: *) YulPath;
    /// ```
    YulExpression,
    /// Represents a node with kind `YulForStatement`, having the following structure:
    ///
    /// ```ebnf
    /// YulForStatement = (* for_keyword: *) YUL_FOR_KEYWORD
    ///                   (* initialization: *) YulBlock
    ///                   (* condition: *) YulExpression
    ///                   (* iterator: *) YulBlock
    ///                   (* body: *) YulBlock;
    /// ```
    YulForStatement,
    /// Represents a node with kind `YulFunctionCallExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// YulFunctionCallExpression = (* operand: *) YulExpression
    ///                             (* open_paren: *) OPEN_PAREN
    ///                             (* arguments: *) YulArguments
    ///                             (* close_paren: *) CLOSE_PAREN;
    /// ```
    YulFunctionCallExpression,
    /// Represents a node with kind `YulFunctionDefinition`, having the following structure:
    ///
    /// ```ebnf
    /// YulFunctionDefinition = (* function_keyword: *) YUL_FUNCTION_KEYWORD
    ///                         (* name: *) YUL_IDENTIFIER
    ///                         (* parameters: *) YulParametersDeclaration
    ///                         (* returns: *) YulReturnsDeclaration?
    ///                         (* body: *) YulBlock;
    /// ```
    YulFunctionDefinition,
    /// Represents a node with kind `YulIfStatement`, having the following structure:
    ///
    /// ```ebnf
    /// YulIfStatement = (* if_keyword: *) YUL_IF_KEYWORD
    ///                  (* condition: *) YulExpression
    ///                  (* body: *) YulBlock;
    /// ```
    YulIfStatement,
    /// Represents a node with kind `YulLabel`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YulLabel = (* label: *) YUL_IDENTIFIER
    ///            (* colon: *) COLON;
    /// ```
    YulLabel,
    /// Represents a node with kind `YulLeaveStatement`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// YulLeaveStatement = (* leave_keyword: *) YUL_LEAVE_KEYWORD;
    /// ```
    YulLeaveStatement,
    /// Represents a node with kind `YulLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// YulLiteral = (* variant: *) YUL_TRUE_KEYWORD (* Introduced in 0.6.2 *)
    ///            | (* variant: *) YUL_FALSE_KEYWORD (* Introduced in 0.6.2 *)
    ///            | (* variant: *) YUL_DECIMAL_LITERAL
    ///            | (* variant: *) YUL_HEX_LITERAL
    ///            | (* variant: *) HexStringLiteral
    ///            | (* variant: *) StringLiteral;
    /// ```
    YulLiteral,
    /// Represents a node with kind `YulParameters`, having the following structure:
    ///
    /// ```ebnf
    /// YulParameters = ((* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*)?;
    /// ```
    YulParameters,
    /// Represents a node with kind `YulParametersDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// YulParametersDeclaration = (* open_paren: *) OPEN_PAREN
    ///                            (* parameters: *) YulParameters
    ///                            (* close_paren: *) CLOSE_PAREN;
    /// ```
    YulParametersDeclaration,
    /// Represents a node with kind `YulPath`, having the following structure:
    ///
    /// ```ebnf
    /// YulPath = (* item: *) YUL_IDENTIFIER ((* separator: *) PERIOD (* item: *) YUL_IDENTIFIER)*;
    /// ```
    YulPath,
    /// Represents a node with kind `YulPaths`, having the following structure:
    ///
    /// ```ebnf
    /// YulPaths = (* item: *) YulPath ((* separator: *) COMMA (* item: *) YulPath)*;
    /// ```
    YulPaths,
    /// Represents a node with kind `YulReturnsDeclaration`, having the following structure:
    ///
    /// ```ebnf
    /// YulReturnsDeclaration = (* minus_greater_than: *) MINUS_GREATER_THAN
    ///                         (* variables: *) YulVariableNames;
    /// ```
    YulReturnsDeclaration,
    /// Represents a node with kind `YulStackAssignmentOperator`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YulStackAssignmentOperator = (* variant: *) EQUAL_COLON
    ///                            | (* variant: *) YulEqualAndColon;
    /// ```
    YulStackAssignmentOperator,
    /// Represents a node with kind `YulStackAssignmentStatement`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YulStackAssignmentStatement = (* assignment: *) YulStackAssignmentOperator
    ///                               (* variable: *) YUL_IDENTIFIER;
    /// ```
    YulStackAssignmentStatement,
    /// Represents a node with kind `YulStatement`, having the following structure:
    ///
    /// ```ebnf
    /// YulStatement = (* variant: *) YulBlock
    ///              | (* variant: *) YulFunctionDefinition
    ///              | (* variant: *) YulStackAssignmentStatement (* Deprecated in 0.5.0 *)
    ///              | (* variant: *) YulIfStatement
    ///              | (* variant: *) YulForStatement
    ///              | (* variant: *) YulSwitchStatement
    ///              | (* variant: *) YulLeaveStatement (* Introduced in 0.6.0 *)
    ///              | (* variant: *) YulBreakStatement
    ///              | (* variant: *) YulContinueStatement
    ///              | (* variant: *) YulVariableAssignmentStatement
    ///              | (* variant: *) YulLabel (* Deprecated in 0.5.0 *)
    ///              | (* variant: *) YulVariableDeclarationStatement
    ///              | (* variant: *) YulExpression;
    /// ```
    YulStatement,
    /// Represents a node with kind `YulStatements`, having the following structure:
    ///
    /// ```ebnf
    /// YulStatements = (* item: *) YulStatement*;
    /// ```
    YulStatements,
    /// Represents a node with kind `YulSwitchCase`, having the following structure:
    ///
    /// ```ebnf
    /// YulSwitchCase = (* variant: *) YulDefaultCase
    ///               | (* variant: *) YulValueCase;
    /// ```
    YulSwitchCase,
    /// Represents a node with kind `YulSwitchCases`, having the following structure:
    ///
    /// ```ebnf
    /// YulSwitchCases = (* item: *) YulSwitchCase+;
    /// ```
    YulSwitchCases,
    /// Represents a node with kind `YulSwitchStatement`, having the following structure:
    ///
    /// ```ebnf
    /// YulSwitchStatement = (* switch_keyword: *) YUL_SWITCH_KEYWORD
    ///                      (* expression: *) YulExpression
    ///                      (* cases: *) YulSwitchCases;
    /// ```
    YulSwitchStatement,
    /// Represents a node with kind `YulValueCase`, having the following structure:
    ///
    /// ```ebnf
    /// YulValueCase = (* case_keyword: *) YUL_CASE_KEYWORD
    ///                (* value: *) YulLiteral
    ///                (* body: *) YulBlock;
    /// ```
    YulValueCase,
    /// Represents a node with kind `YulVariableAssignmentStatement`, having the following structure:
    ///
    /// ```ebnf
    /// YulVariableAssignmentStatement = (* variables: *) YulPaths
    ///                                  (* assignment: *) YulAssignmentOperator
    ///                                  (* expression: *) YulExpression;
    /// ```
    YulVariableAssignmentStatement,
    /// Represents a node with kind `YulVariableDeclarationStatement`, having the following structure:
    ///
    /// ```ebnf
    /// YulVariableDeclarationStatement = (* let_keyword: *) YUL_LET_KEYWORD
    ///                                   (* variables: *) YulVariableNames
    ///                                   (* value: *) YulVariableDeclarationValue?;
    /// ```
    YulVariableDeclarationStatement,
    /// Represents a node with kind `YulVariableDeclarationValue`, having the following structure:
    ///
    /// ```ebnf
    /// YulVariableDeclarationValue = (* assignment: *) YulAssignmentOperator
    ///                               (* expression: *) YulExpression;
    /// ```
    YulVariableDeclarationValue,
    /// Represents a node with kind `YulVariableNames`, having the following structure:
    ///
    /// ```ebnf
    /// YulVariableNames = (* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*;
    /// ```
    YulVariableNames,
}

impl crate::cst::NonterminalKindExtensions for NonterminalKind {}
