// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::doc_markdown)]
#[allow(clippy::doc_link_with_quotes)]
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
pub enum NonterminalKind {
    /// ```ebnf
    /// AbicoderPragma = (* abicoder_keyword: *) ABICODER_KEYWORD
    ///                  (* version: *) IDENTIFIER;
    /// ```
    AbicoderPragma,
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
    /// ```ebnf
    /// AddressType = (* address_keyword: *) ADDRESS_KEYWORD
    ///               (* payable_keyword: *) PAYABLE_KEYWORD?;
    /// ```
    AddressType,
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// AndExpression = (* left_operand: *) Expression
    ///                 (* operator: *) AMPERSAND_AMPERSAND
    ///                 (* right_operand: *) Expression;
    /// ```
    AndExpression,
    /// ```ebnf
    /// ArgumentsDeclaration = (* variant: *) PositionalArgumentsDeclaration
    ///                      | (* variant: *) NamedArgumentsDeclaration;
    /// ```
    ArgumentsDeclaration,
    /// ```ebnf
    /// ArrayExpression = (* open_bracket: *) OPEN_BRACKET
    ///                   (* items: *) ArrayValues
    ///                   (* close_bracket: *) CLOSE_BRACKET;
    /// ```
    ArrayExpression,
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// ArrayTypeName = (* operand: *) TypeName
    ///                 (* open_bracket: *) OPEN_BRACKET
    ///                 (* index: *) Expression?
    ///                 (* close_bracket: *) CLOSE_BRACKET;
    /// ```
    ArrayTypeName,
    /// ```ebnf
    /// ArrayValues = (* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*;
    /// ```
    ArrayValues,
    /// ```ebnf
    /// AssemblyFlags = (* item: *) StringLiteral ((* separator: *) COMMA (* item: *) StringLiteral)*;
    /// ```
    AssemblyFlags,
    /// ```ebnf
    /// AssemblyFlagsDeclaration = (* open_paren: *) OPEN_PAREN
    ///                            (* flags: *) AssemblyFlags
    ///                            (* close_paren: *) CLOSE_PAREN;
    /// ```
    AssemblyFlagsDeclaration,
    /// ```ebnf
    /// AssemblyStatement = (* assembly_keyword: *) ASSEMBLY_KEYWORD
    ///                     (* label: *) StringLiteral?
    ///                     (* flags: *) AssemblyFlagsDeclaration?
    ///                     (* body: *) YulBlock;
    /// ```
    AssemblyStatement,
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
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// BitwiseAndExpression = (* left_operand: *) Expression
    ///                        (* operator: *) AMPERSAND
    ///                        (* right_operand: *) Expression;
    /// ```
    BitwiseAndExpression,
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// BitwiseOrExpression = (* left_operand: *) Expression
    ///                       (* operator: *) BAR
    ///                       (* right_operand: *) Expression;
    /// ```
    BitwiseOrExpression,
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// BitwiseXorExpression = (* left_operand: *) Expression
    ///                        (* operator: *) CARET
    ///                        (* right_operand: *) Expression;
    /// ```
    BitwiseXorExpression,
    /// ```ebnf
    /// Block = (* open_brace: *) OPEN_BRACE
    ///         (* statements: *) Statements
    ///         (* close_brace: *) CLOSE_BRACE;
    /// ```
    Block,
    /// ```ebnf
    /// BreakStatement = (* break_keyword: *) BREAK_KEYWORD
    ///                  (* semicolon: *) SEMICOLON;
    /// ```
    BreakStatement,
    /// ```ebnf
    /// (* Introduced in 0.6.2 *)
    /// CallOptions = (* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*;
    /// ```
    CallOptions,
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// (* Introduced in 0.6.2 *)
    /// CallOptionsExpression = (* operand: *) Expression
    ///                         (* open_brace: *) OPEN_BRACE
    ///                         (* options: *) CallOptions
    ///                         (* close_brace: *) CLOSE_BRACE;
    /// ```
    CallOptionsExpression,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// CatchClause = (* catch_keyword: *) CATCH_KEYWORD
    ///               (* error: *) CatchClauseError?
    ///               (* body: *) Block;
    /// ```
    CatchClause,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// CatchClauseError = (* name: *) IDENTIFIER?
    ///                    (* parameters: *) ParametersDeclaration;
    /// ```
    CatchClauseError,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// CatchClauses = (* item: *) CatchClause+;
    /// ```
    CatchClauses,
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// ComparisonExpression = (* left_operand: *) Expression
    ///                        (* operator: *) LESS_THAN
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// ComparisonExpression = (* left_operand: *) Expression
    ///                        (* operator: *) GREATER_THAN
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// ComparisonExpression = (* left_operand: *) Expression
    ///                        (* operator: *) LESS_THAN_EQUAL
    ///                        (* right_operand: *) Expression;
    ///
    /// (* Left-associative binary operator *)
    /// ComparisonExpression = (* left_operand: *) Expression
    ///                        (* operator: *) GREATER_THAN_EQUAL
    ///                        (* right_operand: *) Expression;
    /// ```
    ComparisonExpression,
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// ConditionalExpression = (* operand: *) Expression
    ///                         (* question_mark: *) QUESTION_MARK
    ///                         (* true_expression: *) Expression
    ///                         (* colon: *) COLON
    ///                         (* false_expression: *) Expression;
    /// ```
    ConditionalExpression,
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
    /// ```ebnf
    /// (* Introduced in 0.4.22 *)
    /// ConstructorAttributes = (* item: *) ConstructorAttribute*;
    /// ```
    ConstructorAttributes,
    /// ```ebnf
    /// (* Introduced in 0.4.22 *)
    /// ConstructorDefinition = (* constructor_keyword: *) CONSTRUCTOR_KEYWORD
    ///                         (* parameters: *) ParametersDeclaration
    ///                         (* attributes: *) ConstructorAttributes
    ///                         (* body: *) Block;
    /// ```
    ConstructorDefinition,
    /// ```ebnf
    /// ContinueStatement = (* continue_keyword: *) CONTINUE_KEYWORD
    ///                     (* semicolon: *) SEMICOLON;
    /// ```
    ContinueStatement,
    /// ```ebnf
    /// ContractDefinition = (* abstract_keyword: *) ABSTRACT_KEYWORD? (* Introduced in 0.6.0 *)
    ///                      (* contract_keyword: *) CONTRACT_KEYWORD
    ///                      (* name: *) IDENTIFIER
    ///                      (* inheritance: *) InheritanceSpecifier?
    ///                      (* open_brace: *) OPEN_BRACE
    ///                      (* members: *) ContractMembers
    ///                      (* close_brace: *) CLOSE_BRACE;
    /// ```
    ContractDefinition,
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
    /// ```ebnf
    /// ContractMembers = (* item: *) ContractMember*;
    /// ```
    ContractMembers,
    /// ```ebnf
    /// DecimalNumberExpression = (* literal: *) DECIMAL_LITERAL
    ///                           (* unit: *) NumberUnit?;
    /// ```
    DecimalNumberExpression,
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
    /// ```ebnf
    /// ElseBranch = (* else_keyword: *) ELSE_KEYWORD
    ///              (* body: *) Statement;
    /// ```
    ElseBranch,
    /// ```ebnf
    /// (* Introduced in 0.4.21 *)
    /// EmitStatement = (* emit_keyword: *) EMIT_KEYWORD
    ///                 (* event: *) IdentifierPath
    ///                 (* arguments: *) ArgumentsDeclaration
    ///                 (* semicolon: *) SEMICOLON;
    /// ```
    EmitStatement,
    /// ```ebnf
    /// EnumDefinition = (* enum_keyword: *) ENUM_KEYWORD
    ///                  (* name: *) IDENTIFIER
    ///                  (* open_brace: *) OPEN_BRACE
    ///                  (* members: *) EnumMembers
    ///                  (* close_brace: *) CLOSE_BRACE;
    /// ```
    EnumDefinition,
    /// ```ebnf
    /// EnumMembers = ((* item: *) IDENTIFIER ((* separator: *) COMMA (* item: *) IDENTIFIER)*)?;
    /// ```
    EnumMembers,
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
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// ErrorDefinition = (* error_keyword: *) ERROR_KEYWORD
    ///                   (* name: *) IDENTIFIER
    ///                   (* members: *) ErrorParametersDeclaration
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    ErrorDefinition,
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// ErrorParameter = (* type_name: *) TypeName
    ///                  (* name: *) IDENTIFIER?;
    /// ```
    ErrorParameter,
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// ErrorParameters = ((* item: *) ErrorParameter ((* separator: *) COMMA (* item: *) ErrorParameter)*)?;
    /// ```
    ErrorParameters,
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// ErrorParametersDeclaration = (* open_paren: *) OPEN_PAREN
    ///                              (* parameters: *) ErrorParameters
    ///                              (* close_paren: *) CLOSE_PAREN;
    /// ```
    ErrorParametersDeclaration,
    /// ```ebnf
    /// EventDefinition = (* event_keyword: *) EVENT_KEYWORD
    ///                   (* name: *) IDENTIFIER
    ///                   (* parameters: *) EventParametersDeclaration
    ///                   (* anonymous_keyword: *) ANONYMOUS_KEYWORD?
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    EventDefinition,
    /// ```ebnf
    /// EventParameter = (* type_name: *) TypeName
    ///                  (* indexed_keyword: *) INDEXED_KEYWORD?
    ///                  (* name: *) IDENTIFIER?;
    /// ```
    EventParameter,
    /// ```ebnf
    /// EventParameters = ((* item: *) EventParameter ((* separator: *) COMMA (* item: *) EventParameter)*)?;
    /// ```
    EventParameters,
    /// ```ebnf
    /// EventParametersDeclaration = (* open_paren: *) OPEN_PAREN
    ///                              (* parameters: *) EventParameters
    ///                              (* close_paren: *) CLOSE_PAREN;
    /// ```
    EventParametersDeclaration,
    /// ```ebnf
    /// ExperimentalFeature = (* variant: *) IDENTIFIER
    ///                     | (* variant: *) StringLiteral;
    /// ```
    ExperimentalFeature,
    /// ```ebnf
    /// ExperimentalPragma = (* experimental_keyword: *) EXPERIMENTAL_KEYWORD
    ///                      (* feature: *) ExperimentalFeature;
    /// ```
    ExperimentalPragma,
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
    /// ```ebnf
    /// Expression = (* variant: *) AssignmentExpression
    ///            | (* variant: *) ConditionalExpression
    ///            | (* variant: *) OrExpression
    ///            | (* variant: *) AndExpression
    ///            | (* variant: *) EqualityExpression
    ///            | (* variant: *) ComparisonExpression
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
    /// ```ebnf
    /// ExpressionStatement = (* expression: *) Expression
    ///                       (* semicolon: *) SEMICOLON;
    /// ```
    ExpressionStatement,
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
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// FallbackFunctionAttributes = (* item: *) FallbackFunctionAttribute*;
    /// ```
    FallbackFunctionAttributes,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// FallbackFunctionDefinition = (* fallback_keyword: *) FALLBACK_KEYWORD
    ///                              (* parameters: *) ParametersDeclaration
    ///                              (* attributes: *) FallbackFunctionAttributes
    ///                              (* returns: *) ReturnsDeclaration?
    ///                              (* body: *) FunctionBody;
    /// ```
    FallbackFunctionDefinition,
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
    /// ```ebnf
    /// ForStatementCondition = (* variant: *) ExpressionStatement
    ///                       | (* variant: *) SEMICOLON;
    /// ```
    ForStatementCondition,
    /// ```ebnf
    /// ForStatementInitialization = (* variant: *) TupleDeconstructionStatement
    ///                            | (* variant: *) VariableDeclarationStatement
    ///                            | (* variant: *) ExpressionStatement
    ///                            | (* variant: *) SEMICOLON;
    /// ```
    ForStatementInitialization,
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
    /// ```ebnf
    /// FunctionAttributes = (* item: *) FunctionAttribute*;
    /// ```
    FunctionAttributes,
    /// ```ebnf
    /// FunctionBody = (* variant: *) Block
    ///              | (* variant: *) SEMICOLON;
    /// ```
    FunctionBody,
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// FunctionCallExpression = (* operand: *) Expression
    ///                          (* arguments: *) ArgumentsDeclaration;
    /// ```
    FunctionCallExpression,
    /// ```ebnf
    /// FunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
    ///                      (* name: *) FunctionName
    ///                      (* parameters: *) ParametersDeclaration
    ///                      (* attributes: *) FunctionAttributes
    ///                      (* returns: *) ReturnsDeclaration?
    ///                      (* body: *) FunctionBody;
    /// ```
    FunctionDefinition,
    /// ```ebnf
    /// FunctionName = (* variant: *) IDENTIFIER
    ///              | (* variant: *) FALLBACK_KEYWORD
    ///              | (* variant: *) RECEIVE_KEYWORD;
    /// ```
    FunctionName,
    /// ```ebnf
    /// FunctionType = (* function_keyword: *) FUNCTION_KEYWORD
    ///                (* parameters: *) ParametersDeclaration
    ///                (* attributes: *) FunctionTypeAttributes
    ///                (* returns: *) ReturnsDeclaration?;
    /// ```
    FunctionType,
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
    /// ```ebnf
    /// FunctionTypeAttributes = (* item: *) FunctionTypeAttribute*;
    /// ```
    FunctionTypeAttributes,
    /// ```ebnf
    /// HexNumberExpression = (* literal: *) HEX_LITERAL
    ///                       (* unit: *) NumberUnit?; (* Deprecated in 0.5.0 *)
    /// ```
    HexNumberExpression,
    /// ```ebnf
    /// HexStringLiteral = (* variant: *) SINGLE_QUOTED_HEX_STRING_LITERAL
    ///                  | (* variant: *) DOUBLE_QUOTED_HEX_STRING_LITERAL;
    /// ```
    HexStringLiteral,
    /// ```ebnf
    /// (* Introduced in 0.5.14 *)
    /// HexStringLiterals = (* item: *) HexStringLiteral+;
    /// ```
    HexStringLiterals,
    /// ```ebnf
    /// IdentifierPath = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
    /// ```
    IdentifierPath,
    /// ```ebnf
    /// IfStatement = (* if_keyword: *) IF_KEYWORD
    ///               (* open_paren: *) OPEN_PAREN
    ///               (* condition: *) Expression
    ///               (* close_paren: *) CLOSE_PAREN
    ///               (* body: *) Statement
    ///               (* else_branch: *) ElseBranch?;
    /// ```
    IfStatement,
    /// ```ebnf
    /// ImportAlias = (* as_keyword: *) AS_KEYWORD
    ///               (* identifier: *) IDENTIFIER;
    /// ```
    ImportAlias,
    /// ```ebnf
    /// ImportClause = (* variant: *) PathImport
    ///              | (* variant: *) NamedImport
    ///              | (* variant: *) ImportDeconstruction;
    /// ```
    ImportClause,
    /// ```ebnf
    /// ImportDeconstruction = (* open_brace: *) OPEN_BRACE
    ///                        (* symbols: *) ImportDeconstructionSymbols
    ///                        (* close_brace: *) CLOSE_BRACE
    ///                        (* from_keyword: *) FROM_KEYWORD
    ///                        (* path: *) StringLiteral;
    /// ```
    ImportDeconstruction,
    /// ```ebnf
    /// ImportDeconstructionSymbol = (* name: *) IDENTIFIER
    ///                              (* alias: *) ImportAlias?;
    /// ```
    ImportDeconstructionSymbol,
    /// ```ebnf
    /// ImportDeconstructionSymbols = (* item: *) ImportDeconstructionSymbol ((* separator: *) COMMA (* item: *) ImportDeconstructionSymbol)*;
    /// ```
    ImportDeconstructionSymbols,
    /// ```ebnf
    /// ImportDirective = (* import_keyword: *) IMPORT_KEYWORD
    ///                   (* clause: *) ImportClause
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    ImportDirective,
    /// ```ebnf
    /// IndexAccessEnd = (* colon: *) COLON
    ///                  (* end: *) Expression?;
    /// ```
    IndexAccessEnd,
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// IndexAccessExpression = (* operand: *) Expression
    ///                         (* open_bracket: *) OPEN_BRACKET
    ///                         (* start: *) Expression?
    ///                         (* end: *) IndexAccessEnd?
    ///                         (* close_bracket: *) CLOSE_BRACKET;
    /// ```
    IndexAccessExpression,
    /// ```ebnf
    /// InheritanceSpecifier = (* is_keyword: *) IS_KEYWORD
    ///                        (* types: *) InheritanceTypes;
    /// ```
    InheritanceSpecifier,
    /// ```ebnf
    /// InheritanceType = (* type_name: *) IdentifierPath
    ///                   (* arguments: *) ArgumentsDeclaration?;
    /// ```
    InheritanceType,
    /// ```ebnf
    /// InheritanceTypes = (* item: *) InheritanceType ((* separator: *) COMMA (* item: *) InheritanceType)*;
    /// ```
    InheritanceTypes,
    /// ```ebnf
    /// InterfaceDefinition = (* interface_keyword: *) INTERFACE_KEYWORD
    ///                       (* name: *) IDENTIFIER
    ///                       (* inheritance: *) InheritanceSpecifier?
    ///                       (* open_brace: *) OPEN_BRACE
    ///                       (* members: *) InterfaceMembers
    ///                       (* close_brace: *) CLOSE_BRACE;
    /// ```
    InterfaceDefinition,
    /// ```ebnf
    /// InterfaceMembers = (* item: *) ContractMember*;
    /// ```
    InterfaceMembers,
    /// ```ebnf
    /// LibraryDefinition = (* library_keyword: *) LIBRARY_KEYWORD
    ///                     (* name: *) IDENTIFIER
    ///                     (* open_brace: *) OPEN_BRACE
    ///                     (* members: *) LibraryMembers
    ///                     (* close_brace: *) CLOSE_BRACE;
    /// ```
    LibraryDefinition,
    /// ```ebnf
    /// LibraryMembers = (* item: *) ContractMember*;
    /// ```
    LibraryMembers,
    /// ```ebnf
    /// MappingKey = (* key_type: *) MappingKeyType
    ///              (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
    /// ```
    MappingKey,
    /// ```ebnf
    /// MappingKeyType = (* variant: *) ElementaryType
    ///                | (* variant: *) IdentifierPath;
    /// ```
    MappingKeyType,
    /// ```ebnf
    /// MappingType = (* mapping_keyword: *) MAPPING_KEYWORD
    ///               (* open_paren: *) OPEN_PAREN
    ///               (* key_type: *) MappingKey
    ///               (* equal_greater_than: *) EQUAL_GREATER_THAN
    ///               (* value_type: *) MappingValue
    ///               (* close_paren: *) CLOSE_PAREN;
    /// ```
    MappingType,
    /// ```ebnf
    /// MappingValue = (* type_name: *) TypeName
    ///                (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
    /// ```
    MappingValue,
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// MemberAccessExpression = (* operand: *) Expression
    ///                          (* period: *) PERIOD
    ///                          (* member: *) IDENTIFIER;
    /// ```
    MemberAccessExpression,
    /// ```ebnf
    /// ModifierAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
    ///                   | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
    /// ```
    ModifierAttribute,
    /// ```ebnf
    /// ModifierAttributes = (* item: *) ModifierAttribute*;
    /// ```
    ModifierAttributes,
    /// ```ebnf
    /// ModifierDefinition = (* modifier_keyword: *) MODIFIER_KEYWORD
    ///                      (* name: *) IDENTIFIER
    ///                      (* parameters: *) ParametersDeclaration?
    ///                      (* attributes: *) ModifierAttributes
    ///                      (* body: *) FunctionBody;
    /// ```
    ModifierDefinition,
    /// ```ebnf
    /// ModifierInvocation = (* name: *) IdentifierPath
    ///                      (* arguments: *) ArgumentsDeclaration?;
    /// ```
    ModifierInvocation,
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
    /// ```ebnf
    /// NamedArgument = (* name: *) IDENTIFIER
    ///                 (* colon: *) COLON
    ///                 (* value: *) Expression;
    /// ```
    NamedArgument,
    /// ```ebnf
    /// NamedArgumentGroup = (* open_brace: *) OPEN_BRACE
    ///                      (* arguments: *) NamedArguments
    ///                      (* close_brace: *) CLOSE_BRACE;
    /// ```
    NamedArgumentGroup,
    /// ```ebnf
    /// NamedArguments = ((* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*)?;
    /// ```
    NamedArguments,
    /// ```ebnf
    /// NamedArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
    ///                             (* arguments: *) NamedArgumentGroup?
    ///                             (* close_paren: *) CLOSE_PAREN;
    /// ```
    NamedArgumentsDeclaration,
    /// ```ebnf
    /// NamedImport = (* asterisk: *) ASTERISK
    ///               (* alias: *) ImportAlias
    ///               (* from_keyword: *) FROM_KEYWORD
    ///               (* path: *) StringLiteral;
    /// ```
    NamedImport,
    /// ```ebnf
    /// NewExpression = (* new_keyword: *) NEW_KEYWORD
    ///                 (* type_name: *) TypeName;
    /// ```
    NewExpression,
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
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// OrExpression = (* left_operand: *) Expression
    ///                (* operator: *) BAR_BAR
    ///                (* right_operand: *) Expression;
    /// ```
    OrExpression,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// OverridePaths = (* item: *) IdentifierPath ((* separator: *) COMMA (* item: *) IdentifierPath)*;
    /// ```
    OverridePaths,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// OverridePathsDeclaration = (* open_paren: *) OPEN_PAREN
    ///                            (* paths: *) OverridePaths
    ///                            (* close_paren: *) CLOSE_PAREN;
    /// ```
    OverridePathsDeclaration,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// OverrideSpecifier = (* override_keyword: *) OVERRIDE_KEYWORD
    ///                     (* overridden: *) OverridePathsDeclaration?;
    /// ```
    OverrideSpecifier,
    /// ```ebnf
    /// Parameter = (* type_name: *) TypeName
    ///             (* storage_location: *) StorageLocation?
    ///             (* name: *) IDENTIFIER?;
    /// ```
    Parameter,
    /// ```ebnf
    /// Parameters = ((* item: *) Parameter ((* separator: *) COMMA (* item: *) Parameter)*)?;
    /// ```
    Parameters,
    /// ```ebnf
    /// ParametersDeclaration = (* open_paren: *) OPEN_PAREN
    ///                         (* parameters: *) Parameters
    ///                         (* close_paren: *) CLOSE_PAREN;
    /// ```
    ParametersDeclaration,
    /// ```ebnf
    /// PathImport = (* path: *) StringLiteral
    ///              (* alias: *) ImportAlias?;
    /// ```
    PathImport,
    /// ```ebnf
    /// PositionalArguments = ((* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*)?;
    /// ```
    PositionalArguments,
    /// ```ebnf
    /// PositionalArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
    ///                                  (* arguments: *) PositionalArguments
    ///                                  (* close_paren: *) CLOSE_PAREN;
    /// ```
    PositionalArgumentsDeclaration,
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
    /// ```ebnf
    /// Pragma = (* variant: *) AbicoderPragma
    ///        | (* variant: *) ExperimentalPragma
    ///        | (* variant: *) VersionPragma;
    /// ```
    Pragma,
    /// ```ebnf
    /// PragmaDirective = (* pragma_keyword: *) PRAGMA_KEYWORD
    ///                   (* pragma: *) Pragma
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    PragmaDirective,
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
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// ReceiveFunctionAttribute = (* variant: *) ModifierInvocation
    ///                          | (* variant: *) OverrideSpecifier
    ///                          | (* variant: *) EXTERNAL_KEYWORD
    ///                          | (* variant: *) PAYABLE_KEYWORD
    ///                          | (* variant: *) VIRTUAL_KEYWORD;
    /// ```
    ReceiveFunctionAttribute,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// ReceiveFunctionAttributes = (* item: *) ReceiveFunctionAttribute*;
    /// ```
    ReceiveFunctionAttributes,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// ReceiveFunctionDefinition = (* receive_keyword: *) RECEIVE_KEYWORD
    ///                             (* parameters: *) ParametersDeclaration
    ///                             (* attributes: *) ReceiveFunctionAttributes
    ///                             (* body: *) FunctionBody;
    /// ```
    ReceiveFunctionDefinition,
    /// ```ebnf
    /// ReturnStatement = (* return_keyword: *) RETURN_KEYWORD
    ///                   (* expression: *) Expression?
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    ReturnStatement,
    /// ```ebnf
    /// ReturnsDeclaration = (* returns_keyword: *) RETURNS_KEYWORD
    ///                      (* variables: *) ParametersDeclaration;
    /// ```
    ReturnsDeclaration,
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// RevertStatement = (* revert_keyword: *) REVERT_KEYWORD
    ///                   (* error: *) IdentifierPath?
    ///                   (* arguments: *) ArgumentsDeclaration
    ///                   (* semicolon: *) SEMICOLON;
    /// ```
    RevertStatement,
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
    /// ```ebnf
    /// SimpleVersionLiteral = (* item: *) VERSION_SPECIFIER ((* separator: *) PERIOD (* item: *) VERSION_SPECIFIER)*;
    /// ```
    SimpleVersionLiteral,
    /// ```ebnf
    /// SourceUnit = (* members: *) SourceUnitMembers;
    /// ```
    SourceUnit,
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
    /// ```ebnf
    /// SourceUnitMembers = (* item: *) SourceUnitMember*;
    /// ```
    SourceUnitMembers,
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
    /// ```ebnf
    /// StateVariableAttributes = (* item: *) StateVariableAttribute*;
    /// ```
    StateVariableAttributes,
    /// ```ebnf
    /// StateVariableDefinition = (* type_name: *) TypeName
    ///                           (* attributes: *) StateVariableAttributes
    ///                           (* name: *) IDENTIFIER
    ///                           (* value: *) StateVariableDefinitionValue?
    ///                           (* semicolon: *) SEMICOLON;
    /// ```
    StateVariableDefinition,
    /// ```ebnf
    /// StateVariableDefinitionValue = (* equal: *) EQUAL
    ///                                (* value: *) Expression;
    /// ```
    StateVariableDefinitionValue,
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
    /// ```ebnf
    /// Statements = (* item: *) Statement*;
    /// ```
    Statements,
    /// ```ebnf
    /// StorageLocation = (* variant: *) MEMORY_KEYWORD
    ///                 | (* variant: *) STORAGE_KEYWORD
    ///                 | (* variant: *) CALL_DATA_KEYWORD; (* Introduced in 0.5.0 *)
    /// ```
    StorageLocation,
    /// ```ebnf
    /// StringExpression = (* variant: *) StringLiteral (* Deprecated in 0.5.14 *)
    ///                  | (* variant: *) StringLiterals (* Introduced in 0.5.14 *)
    ///                  | (* variant: *) HexStringLiteral (* Deprecated in 0.5.14 *)
    ///                  | (* variant: *) HexStringLiterals (* Introduced in 0.5.14 *)
    ///                  | (* variant: *) UnicodeStringLiterals; (* Introduced in 0.7.0 *)
    /// ```
    StringExpression,
    /// ```ebnf
    /// StringLiteral = (* variant: *) SINGLE_QUOTED_STRING_LITERAL
    ///               | (* variant: *) DOUBLE_QUOTED_STRING_LITERAL;
    /// ```
    StringLiteral,
    /// ```ebnf
    /// (* Introduced in 0.5.14 *)
    /// StringLiterals = (* item: *) StringLiteral+;
    /// ```
    StringLiterals,
    /// ```ebnf
    /// StructDefinition = (* struct_keyword: *) STRUCT_KEYWORD
    ///                    (* name: *) IDENTIFIER
    ///                    (* open_brace: *) OPEN_BRACE
    ///                    (* members: *) StructMembers
    ///                    (* close_brace: *) CLOSE_BRACE;
    /// ```
    StructDefinition,
    /// ```ebnf
    /// StructMember = (* type_name: *) TypeName
    ///                (* name: *) IDENTIFIER
    ///                (* semicolon: *) SEMICOLON;
    /// ```
    StructMember,
    /// ```ebnf
    /// StructMembers = (* item: *) StructMember*;
    /// ```
    StructMembers,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// ThrowStatement = (* throw_keyword: *) THROW_KEYWORD
    ///                  (* semicolon: *) SEMICOLON;
    /// ```
    ThrowStatement,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// TryStatement = (* try_keyword: *) TRY_KEYWORD
    ///                (* expression: *) Expression
    ///                (* returns: *) ReturnsDeclaration?
    ///                (* body: *) Block
    ///                (* catch_clauses: *) CatchClauses;
    /// ```
    TryStatement,
    /// ```ebnf
    /// TupleDeconstructionElement = (* member: *) TupleMember?;
    /// ```
    TupleDeconstructionElement,
    /// ```ebnf
    /// TupleDeconstructionElements = (* item: *) TupleDeconstructionElement ((* separator: *) COMMA (* item: *) TupleDeconstructionElement)*;
    /// ```
    TupleDeconstructionElements,
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
    /// ```ebnf
    /// TupleExpression = (* open_paren: *) OPEN_PAREN
    ///                   (* items: *) TupleValues
    ///                   (* close_paren: *) CLOSE_PAREN;
    /// ```
    TupleExpression,
    /// ```ebnf
    /// TupleMember = (* variant: *) TypedTupleMember
    ///             | (* variant: *) UntypedTupleMember;
    /// ```
    TupleMember,
    /// ```ebnf
    /// TupleValue = (* expression: *) Expression?;
    /// ```
    TupleValue,
    /// ```ebnf
    /// TupleValues = (* item: *) TupleValue ((* separator: *) COMMA (* item: *) TupleValue)*;
    /// ```
    TupleValues,
    /// ```ebnf
    /// (* Introduced in 0.5.3 *)
    /// TypeExpression = (* type_keyword: *) TYPE_KEYWORD
    ///                  (* open_paren: *) OPEN_PAREN
    ///                  (* type_name: *) TypeName
    ///                  (* close_paren: *) CLOSE_PAREN;
    /// ```
    TypeExpression,
    /// ```ebnf
    /// TypeName = (* variant: *) ArrayTypeName
    ///          | (* variant: *) FunctionType
    ///          | (* variant: *) MappingType
    ///          | (* variant: *) ElementaryType
    ///          | (* variant: *) IdentifierPath;
    /// ```
    TypeName,
    /// ```ebnf
    /// TypedTupleMember = (* type_name: *) TypeName
    ///                    (* storage_location: *) StorageLocation?
    ///                    (* name: *) IDENTIFIER;
    /// ```
    TypedTupleMember,
    /// ```ebnf
    /// (* Introduced in 0.8.0 *)
    /// UncheckedBlock = (* unchecked_keyword: *) UNCHECKED_KEYWORD
    ///                  (* block: *) Block;
    /// ```
    UncheckedBlock,
    /// ```ebnf
    /// (* Introduced in 0.7.0 *)
    /// UnicodeStringLiteral = (* variant: *) SINGLE_QUOTED_UNICODE_STRING_LITERAL
    ///                      | (* variant: *) DOUBLE_QUOTED_UNICODE_STRING_LITERAL;
    /// ```
    UnicodeStringLiteral,
    /// ```ebnf
    /// (* Introduced in 0.7.0 *)
    /// UnicodeStringLiterals = (* item: *) UnicodeStringLiteral+;
    /// ```
    UnicodeStringLiterals,
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
    /// ```ebnf
    /// (* Deprecated in 0.6.0 *)
    /// UnnamedFunctionAttributes = (* item: *) UnnamedFunctionAttribute*;
    /// ```
    UnnamedFunctionAttributes,
    /// ```ebnf
    /// (* Deprecated in 0.6.0 *)
    /// UnnamedFunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
    ///                             (* parameters: *) ParametersDeclaration
    ///                             (* attributes: *) UnnamedFunctionAttributes
    ///                             (* body: *) FunctionBody;
    /// ```
    UnnamedFunctionDefinition,
    /// ```ebnf
    /// UntypedTupleMember = (* storage_location: *) StorageLocation?
    ///                      (* name: *) IDENTIFIER;
    /// ```
    UntypedTupleMember,
    /// ```ebnf
    /// (* Introduced in 0.8.8 *)
    /// UserDefinedValueTypeDefinition = (* type_keyword: *) TYPE_KEYWORD
    ///                                  (* name: *) IDENTIFIER
    ///                                  (* is_keyword: *) IS_KEYWORD
    ///                                  (* value_type: *) ElementaryType
    ///                                  (* semicolon: *) SEMICOLON;
    /// ```
    UserDefinedValueTypeDefinition,
    /// ```ebnf
    /// (* Introduced in 0.8.19 *)
    /// UsingAlias = (* as_keyword: *) AS_KEYWORD
    ///              (* operator: *) UsingOperator;
    /// ```
    UsingAlias,
    /// ```ebnf
    /// UsingClause = (* variant: *) IdentifierPath
    ///             | (* variant: *) UsingDeconstruction; (* Introduced in 0.8.13 *)
    /// ```
    UsingClause,
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// UsingDeconstruction = (* open_brace: *) OPEN_BRACE
    ///                       (* symbols: *) UsingDeconstructionSymbols
    ///                       (* close_brace: *) CLOSE_BRACE;
    /// ```
    UsingDeconstruction,
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// UsingDeconstructionSymbol = (* name: *) IdentifierPath
    ///                             (* alias: *) UsingAlias?; (* Introduced in 0.8.19 *)
    /// ```
    UsingDeconstructionSymbol,
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// UsingDeconstructionSymbols = (* item: *) UsingDeconstructionSymbol ((* separator: *) COMMA (* item: *) UsingDeconstructionSymbol)*;
    /// ```
    UsingDeconstructionSymbols,
    /// ```ebnf
    /// UsingDirective = (* using_keyword: *) USING_KEYWORD
    ///                  (* clause: *) UsingClause
    ///                  (* for_keyword: *) FOR_KEYWORD
    ///                  (* target: *) UsingTarget
    ///                  (* global_keyword: *) GLOBAL_KEYWORD? (* Introduced in 0.8.13 *)
    ///                  (* semicolon: *) SEMICOLON;
    /// ```
    UsingDirective,
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
    /// ```ebnf
    /// UsingTarget = (* variant: *) TypeName
    ///             | (* variant: *) ASTERISK;
    /// ```
    UsingTarget,
    /// ```ebnf
    /// VariableDeclarationStatement = (* variable_type: *) VariableDeclarationType
    ///                                (* storage_location: *) StorageLocation?
    ///                                (* name: *) IDENTIFIER
    ///                                (* value: *) VariableDeclarationValue?
    ///                                (* semicolon: *) SEMICOLON;
    /// ```
    VariableDeclarationStatement,
    /// ```ebnf
    /// VariableDeclarationType = (* variant: *) TypeName
    ///                         | (* variant: *) VAR_KEYWORD; (* Deprecated in 0.5.0 *)
    /// ```
    VariableDeclarationType,
    /// ```ebnf
    /// VariableDeclarationValue = (* equal: *) EQUAL
    ///                            (* expression: *) Expression;
    /// ```
    VariableDeclarationValue,
    /// ```ebnf
    /// VersionExpression = (* variant: *) VersionRange
    ///                   | (* variant: *) VersionTerm;
    /// ```
    VersionExpression,
    /// ```ebnf
    /// VersionExpressionSet = (* item: *) VersionExpression+;
    /// ```
    VersionExpressionSet,
    /// ```ebnf
    /// VersionExpressionSets = (* item: *) VersionExpressionSet ((* separator: *) BAR_BAR (* item: *) VersionExpressionSet)*;
    /// ```
    VersionExpressionSets,
    /// ```ebnf
    /// VersionLiteral = (* variant: *) SimpleVersionLiteral
    ///                | (* variant: *) SINGLE_QUOTED_VERSION_LITERAL
    ///                | (* variant: *) DOUBLE_QUOTED_VERSION_LITERAL;
    /// ```
    VersionLiteral,
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
    /// ```ebnf
    /// VersionPragma = (* solidity_keyword: *) SOLIDITY_KEYWORD
    ///                 (* sets: *) VersionExpressionSets;
    /// ```
    VersionPragma,
    /// ```ebnf
    /// VersionRange = (* start: *) VersionLiteral
    ///                (* minus: *) MINUS
    ///                (* end: *) VersionLiteral;
    /// ```
    VersionRange,
    /// ```ebnf
    /// VersionTerm = (* operator: *) VersionOperator?
    ///               (* literal: *) VersionLiteral;
    /// ```
    VersionTerm,
    /// ```ebnf
    /// WhileStatement = (* while_keyword: *) WHILE_KEYWORD
    ///                  (* open_paren: *) OPEN_PAREN
    ///                  (* condition: *) Expression
    ///                  (* close_paren: *) CLOSE_PAREN
    ///                  (* body: *) Statement;
    /// ```
    WhileStatement,
    /// ```ebnf
    /// YulArguments = ((* item: *) YulExpression ((* separator: *) COMMA (* item: *) YulExpression)*)?;
    /// ```
    YulArguments,
    /// ```ebnf
    /// YulAssignmentOperator = (* variant: *) COLON_EQUAL
    ///                       | (* variant: *) YulColonAndEqual; (* Deprecated in 0.5.5 *)
    /// ```
    YulAssignmentOperator,
    /// ```ebnf
    /// YulBlock = (* open_brace: *) OPEN_BRACE
    ///            (* statements: *) YulStatements
    ///            (* close_brace: *) CLOSE_BRACE;
    /// ```
    YulBlock,
    /// ```ebnf
    /// YulBreakStatement = (* break_keyword: *) YUL_BREAK_KEYWORD;
    /// ```
    YulBreakStatement,
    /// ```ebnf
    /// YulBuiltInFunction = (* variant: *) YUL_JUMP_KEYWORD (* Deprecated in 0.5.0 *)
    ///                    | (* variant: *) YUL_JUMPI_KEYWORD; (* Deprecated in 0.5.0 *)
    /// ```
    YulBuiltInFunction,
    /// ```ebnf
    /// (* Deprecated in 0.5.5 *)
    /// YulColonAndEqual = (* colon: *) COLON
    ///                    (* equal: *) EQUAL;
    /// ```
    YulColonAndEqual,
    /// ```ebnf
    /// YulContinueStatement = (* continue_keyword: *) YUL_CONTINUE_KEYWORD;
    /// ```
    YulContinueStatement,
    /// ```ebnf
    /// YulDefaultCase = (* default_keyword: *) YUL_DEFAULT_KEYWORD
    ///                  (* body: *) YulBlock;
    /// ```
    YulDefaultCase,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YulEqualAndColon = (* equal: *) EQUAL
    ///                    (* colon: *) COLON;
    /// ```
    YulEqualAndColon,
    /// ```ebnf
    /// YulExpression = (* variant: *) YulFunctionCallExpression
    ///               | (* variant: *) YulLiteral
    ///               | (* variant: *) YulBuiltInFunction
    ///               | (* variant: *) YulPath;
    /// ```
    YulExpression,
    /// ```ebnf
    /// YulForStatement = (* for_keyword: *) YUL_FOR_KEYWORD
    ///                   (* initialization: *) YulBlock
    ///                   (* condition: *) YulExpression
    ///                   (* iterator: *) YulBlock
    ///                   (* body: *) YulBlock;
    /// ```
    YulForStatement,
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// YulFunctionCallExpression = (* operand: *) YulExpression
    ///                             (* open_paren: *) OPEN_PAREN
    ///                             (* arguments: *) YulArguments
    ///                             (* close_paren: *) CLOSE_PAREN;
    /// ```
    YulFunctionCallExpression,
    /// ```ebnf
    /// YulFunctionDefinition = (* function_keyword: *) YUL_FUNCTION_KEYWORD
    ///                         (* name: *) YUL_IDENTIFIER
    ///                         (* parameters: *) YulParametersDeclaration
    ///                         (* returns: *) YulReturnsDeclaration?
    ///                         (* body: *) YulBlock;
    /// ```
    YulFunctionDefinition,
    /// ```ebnf
    /// YulIfStatement = (* if_keyword: *) YUL_IF_KEYWORD
    ///                  (* condition: *) YulExpression
    ///                  (* body: *) YulBlock;
    /// ```
    YulIfStatement,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YulLabel = (* label: *) YUL_IDENTIFIER
    ///            (* colon: *) COLON;
    /// ```
    YulLabel,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// YulLeaveStatement = (* leave_keyword: *) YUL_LEAVE_KEYWORD;
    /// ```
    YulLeaveStatement,
    /// ```ebnf
    /// YulLiteral = (* variant: *) YUL_TRUE_KEYWORD
    ///            | (* variant: *) YUL_FALSE_KEYWORD
    ///            | (* variant: *) YUL_DECIMAL_LITERAL
    ///            | (* variant: *) YUL_HEX_LITERAL
    ///            | (* variant: *) HexStringLiteral
    ///            | (* variant: *) StringLiteral;
    /// ```
    YulLiteral,
    /// ```ebnf
    /// YulParameters = ((* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*)?;
    /// ```
    YulParameters,
    /// ```ebnf
    /// YulParametersDeclaration = (* open_paren: *) OPEN_PAREN
    ///                            (* parameters: *) YulParameters
    ///                            (* close_paren: *) CLOSE_PAREN;
    /// ```
    YulParametersDeclaration,
    /// ```ebnf
    /// YulPath = (* item: *) YUL_IDENTIFIER ((* separator: *) PERIOD (* item: *) YUL_IDENTIFIER)*;
    /// ```
    YulPath,
    /// ```ebnf
    /// YulPaths = (* item: *) YulPath ((* separator: *) COMMA (* item: *) YulPath)*;
    /// ```
    YulPaths,
    /// ```ebnf
    /// YulReturnsDeclaration = (* minus_greater_than: *) MINUS_GREATER_THAN
    ///                         (* variables: *) YulVariableNames;
    /// ```
    YulReturnsDeclaration,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YulStackAssignmentOperator = (* variant: *) EQUAL_COLON
    ///                            | (* variant: *) YulEqualAndColon;
    /// ```
    YulStackAssignmentOperator,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YulStackAssignmentStatement = (* assignment: *) YulStackAssignmentOperator
    ///                               (* variable: *) YUL_IDENTIFIER;
    /// ```
    YulStackAssignmentStatement,
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
    /// ```ebnf
    /// YulStatements = (* item: *) YulStatement*;
    /// ```
    YulStatements,
    /// ```ebnf
    /// YulSwitchCase = (* variant: *) YulDefaultCase
    ///               | (* variant: *) YulValueCase;
    /// ```
    YulSwitchCase,
    /// ```ebnf
    /// YulSwitchCases = (* item: *) YulSwitchCase+;
    /// ```
    YulSwitchCases,
    /// ```ebnf
    /// YulSwitchStatement = (* switch_keyword: *) YUL_SWITCH_KEYWORD
    ///                      (* expression: *) YulExpression
    ///                      (* cases: *) YulSwitchCases;
    /// ```
    YulSwitchStatement,
    /// ```ebnf
    /// YulValueCase = (* case_keyword: *) YUL_CASE_KEYWORD
    ///                (* value: *) YulLiteral
    ///                (* body: *) YulBlock;
    /// ```
    YulValueCase,
    /// ```ebnf
    /// YulVariableAssignmentStatement = (* variables: *) YulPaths
    ///                                  (* assignment: *) YulAssignmentOperator
    ///                                  (* expression: *) YulExpression;
    /// ```
    YulVariableAssignmentStatement,
    /// ```ebnf
    /// YulVariableDeclarationStatement = (* let_keyword: *) YUL_LET_KEYWORD
    ///                                   (* variables: *) YulVariableNames
    ///                                   (* value: *) YulVariableDeclarationValue?;
    /// ```
    YulVariableDeclarationStatement,
    /// ```ebnf
    /// YulVariableDeclarationValue = (* assignment: *) YulAssignmentOperator
    ///                               (* expression: *) YulExpression;
    /// ```
    YulVariableDeclarationValue,
    /// ```ebnf
    /// YulVariableNames = (* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*;
    /// ```
    YulVariableNames,
}

impl crate::cst::NonterminalKindExtensions for NonterminalKind {}
