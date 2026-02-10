// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangCst {
  export { TerminalKindExtensions };
  export { NonterminalNode };
  export { TerminalNode };
  export { Edge };
  export { Cursor };
  export { CursorIterator };
  export { AncestorsIterator };
  export { Query };
  export { QueryMatchIterator };
  export { TextIndexExtensions };
  export { NonterminalKind };
  export { TerminalKind };
  export { EdgeLabel };
  export { Node };
  export { NodeType };
}
/**
 * Represents different kinds of nonterminal nodes in the syntax tree.
 * These are nodes that can have child nodes and represent higher-level language constructs.
 */
export declare enum NonterminalKind {
  /**
   * Represents a node with kind `AbicoderPragma`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.5 *)
   * AbicoderPragma = (* abicoder_keyword: *) ABICODER_KEYWORD
   *                  (* version: *) AbicoderVersion;
   * ```
   */
  AbicoderPragma = "AbicoderPragma",
  /**
   * Represents a node with kind `AbicoderVersion`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.5 *)
   * AbicoderVersion = (* variant: *) ABICODER_V1_KEYWORD
   *                 | (* variant: *) ABICODER_V2_KEYWORD;
   * ```
   */
  AbicoderVersion = "AbicoderVersion",
  /**
   * Represents a node with kind `AdditiveExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * AdditiveExpression = (* left_operand: *) Expression
   *                      (* operator: *) PLUS
   *                      (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AdditiveExpression = (* left_operand: *) Expression
   *                      (* operator: *) MINUS
   *                      (* right_operand: *) Expression;
   * ```
   */
  AdditiveExpression = "AdditiveExpression",
  /**
   * Represents a node with kind `AddressType`, having the following structure:
   *
   * ```ebnf
   * AddressType = (* address_keyword: *) ADDRESS_KEYWORD
   *               (* payable_keyword: *) PAYABLE_KEYWORD?; (* Introduced in 0.5.0 *)
   * ```
   */
  AddressType = "AddressType",
  /**
   * Represents a node with kind `AndExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * AndExpression = (* left_operand: *) Expression
   *                 (* operator: *) AMPERSAND_AMPERSAND
   *                 (* right_operand: *) Expression;
   * ```
   */
  AndExpression = "AndExpression",
  /**
   * Represents a node with kind `ArgumentsDeclaration`, having the following structure:
   *
   * ```ebnf
   * ArgumentsDeclaration = (* variant: *) PositionalArgumentsDeclaration
   *                      | (* variant: *) NamedArgumentsDeclaration;
   * ```
   */
  ArgumentsDeclaration = "ArgumentsDeclaration",
  /**
   * Represents a node with kind `ArrayExpression`, having the following structure:
   *
   * ```ebnf
   * ArrayExpression = (* open_bracket: *) OPEN_BRACKET
   *                   (* items: *) ArrayValues
   *                   (* close_bracket: *) CLOSE_BRACKET;
   * ```
   */
  ArrayExpression = "ArrayExpression",
  /**
   * Represents a node with kind `ArrayTypeName`, having the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * ArrayTypeName = (* operand: *) TypeName
   *                 (* open_bracket: *) OPEN_BRACKET
   *                 (* index: *) Expression?
   *                 (* close_bracket: *) CLOSE_BRACKET;
   * ```
   */
  ArrayTypeName = "ArrayTypeName",
  /**
   * Represents a node with kind `ArrayValues`, having the following structure:
   *
   * ```ebnf
   * ArrayValues = (* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*;
   * ```
   */
  ArrayValues = "ArrayValues",
  /**
   * Represents a node with kind `AssemblyFlags`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * AssemblyFlags = (* item: *) StringLiteral ((* separator: *) COMMA (* item: *) StringLiteral)*;
   * ```
   */
  AssemblyFlags = "AssemblyFlags",
  /**
   * Represents a node with kind `AssemblyFlagsDeclaration`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * AssemblyFlagsDeclaration = (* open_paren: *) OPEN_PAREN
   *                            (* flags: *) AssemblyFlags
   *                            (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  AssemblyFlagsDeclaration = "AssemblyFlagsDeclaration",
  /**
   * Represents a node with kind `AssemblyStatement`, having the following structure:
   *
   * ```ebnf
   * AssemblyStatement = (* assembly_keyword: *) ASSEMBLY_KEYWORD
   *                     (* label: *) StringLiteral?
   *                     (* flags: *) AssemblyFlagsDeclaration? (* Introduced in 0.8.13 *)
   *                     (* body: *) YulBlock;
   * ```
   */
  AssemblyStatement = "AssemblyStatement",
  /**
   * Represents a node with kind `AssignmentExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) BAR_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) PLUS_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) MINUS_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) CARET_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) SLASH_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) PERCENT_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) ASTERISK_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) AMPERSAND_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) LESS_THAN_LESS_THAN_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) GREATER_THAN_GREATER_THAN_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   *                        (* operator: *) GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL
   *                        (* right_operand: *) Expression;
   * ```
   */
  AssignmentExpression = "AssignmentExpression",
  /**
   * Represents a node with kind `BitwiseAndExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * BitwiseAndExpression = (* left_operand: *) Expression
   *                        (* operator: *) AMPERSAND
   *                        (* right_operand: *) Expression;
   * ```
   */
  BitwiseAndExpression = "BitwiseAndExpression",
  /**
   * Represents a node with kind `BitwiseOrExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * BitwiseOrExpression = (* left_operand: *) Expression
   *                       (* operator: *) BAR
   *                       (* right_operand: *) Expression;
   * ```
   */
  BitwiseOrExpression = "BitwiseOrExpression",
  /**
   * Represents a node with kind `BitwiseXorExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * BitwiseXorExpression = (* left_operand: *) Expression
   *                        (* operator: *) CARET
   *                        (* right_operand: *) Expression;
   * ```
   */
  BitwiseXorExpression = "BitwiseXorExpression",
  /**
   * Represents a node with kind `Block`, having the following structure:
   *
   * ```ebnf
   * Block = (* open_brace: *) OPEN_BRACE
   *         (* statements: *) Statements
   *         (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  Block = "Block",
  /**
   * Represents a node with kind `BreakStatement`, having the following structure:
   *
   * ```ebnf
   * BreakStatement = (* break_keyword: *) BREAK_KEYWORD
   *                  (* semicolon: *) SEMICOLON;
   * ```
   */
  BreakStatement = "BreakStatement",
  /**
   * Represents a node with kind `CallOptions`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.2 *)
   * CallOptions = (* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*;
   * ```
   */
  CallOptions = "CallOptions",
  /**
   * Represents a node with kind `CallOptionsExpression`, having the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * (* Introduced in 0.6.2 *)
   * CallOptionsExpression = (* operand: *) Expression
   *                         (* open_brace: *) OPEN_BRACE
   *                         (* options: *) CallOptions
   *                         (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  CallOptionsExpression = "CallOptionsExpression",
  /**
   * Represents a node with kind `CatchClause`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * CatchClause = (* catch_keyword: *) CATCH_KEYWORD
   *               (* error: *) CatchClauseError?
   *               (* body: *) Block;
   * ```
   */
  CatchClause = "CatchClause",
  /**
   * Represents a node with kind `CatchClauseError`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * CatchClauseError = (* name: *) IDENTIFIER?
   *                    (* parameters: *) ParametersDeclaration;
   * ```
   */
  CatchClauseError = "CatchClauseError",
  /**
   * Represents a node with kind `CatchClauses`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * CatchClauses = (* item: *) CatchClause+;
   * ```
   */
  CatchClauses = "CatchClauses",
  /**
   * Represents a node with kind `ConditionalExpression`, having the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * ConditionalExpression = (* operand: *) Expression
   *                         (* question_mark: *) QUESTION_MARK
   *                         (* true_expression: *) Expression
   *                         (* colon: *) COLON
   *                         (* false_expression: *) Expression;
   * ```
   */
  ConditionalExpression = "ConditionalExpression",
  /**
   * Represents a node with kind `ConstantDefinition`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.4 *)
   * ConstantDefinition = (* type_name: *) TypeName
   *                      (* constant_keyword: *) CONSTANT_KEYWORD
   *                      (* name: *) IDENTIFIER
   *                      (* equal: *) EQUAL
   *                      (* value: *) Expression
   *                      (* semicolon: *) SEMICOLON;
   * ```
   */
  ConstantDefinition = "ConstantDefinition",
  /**
   * Represents a node with kind `ConstructorAttribute`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.22 *)
   * ConstructorAttribute = (* variant: *) ModifierInvocation
   *                      | (* variant: *) INTERNAL_KEYWORD
   *                      | (* variant: *) OVERRIDE_KEYWORD (* Introduced in 0.6.0 and deprecated in 0.6.7. *)
   *                      | (* variant: *) PAYABLE_KEYWORD
   *                      | (* variant: *) PUBLIC_KEYWORD
   *                      | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 and deprecated in 0.6.7. *)
   * ```
   */
  ConstructorAttribute = "ConstructorAttribute",
  /**
   * Represents a node with kind `ConstructorAttributes`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.22 *)
   * ConstructorAttributes = (* item: *) ConstructorAttribute*;
   * ```
   */
  ConstructorAttributes = "ConstructorAttributes",
  /**
   * Represents a node with kind `ConstructorDefinition`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.22 *)
   * ConstructorDefinition = (* constructor_keyword: *) CONSTRUCTOR_KEYWORD
   *                         (* parameters: *) ParametersDeclaration
   *                         (* attributes: *) ConstructorAttributes
   *                         (* body: *) Block;
   * ```
   */
  ConstructorDefinition = "ConstructorDefinition",
  /**
   * Represents a node with kind `ContinueStatement`, having the following structure:
   *
   * ```ebnf
   * ContinueStatement = (* continue_keyword: *) CONTINUE_KEYWORD
   *                     (* semicolon: *) SEMICOLON;
   * ```
   */
  ContinueStatement = "ContinueStatement",
  /**
   * Represents a node with kind `ContractDefinition`, having the following structure:
   *
   * ```ebnf
   * ContractDefinition = (* abstract_keyword: *) ABSTRACT_KEYWORD? (* Introduced in 0.6.0 *)
   *                      (* contract_keyword: *) CONTRACT_KEYWORD
   *                      (* name: *) IDENTIFIER
   *                      (* specifiers: *) ContractSpecifiers
   *                      (* open_brace: *) OPEN_BRACE
   *                      (* members: *) ContractMembers
   *                      (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  ContractDefinition = "ContractDefinition",
  /**
   * Represents a node with kind `ContractMember`, having the following structure:
   *
   * ```ebnf
   * ContractMember = (* variant: *) UsingDirective
   *                | (* variant: *) FunctionDefinition
   *                | (* variant: *) ConstructorDefinition (* Introduced in 0.4.22 *)
   *                | (* variant: *) ReceiveFunctionDefinition (* Introduced in 0.6.0 *)
   *                | (* variant: *) FallbackFunctionDefinition (* Introduced in 0.6.0 *)
   *                | (* variant: *) UnnamedFunctionDefinition (* Deprecated in 0.6.0 *)
   *                | (* variant: *) ModifierDefinition
   *                | (* variant: *) StructDefinition
   *                | (* variant: *) EnumDefinition
   *                | (* variant: *) EventDefinition
   *                | (* variant: *) ErrorDefinition (* Introduced in 0.8.4 *)
   *                | (* variant: *) UserDefinedValueTypeDefinition (* Introduced in 0.8.8 *)
   *                | (* variant: *) StateVariableDefinition;
   * ```
   */
  ContractMember = "ContractMember",
  /**
   * Represents a node with kind `ContractMembers`, having the following structure:
   *
   * ```ebnf
   * ContractMembers = (* item: *) ContractMember*;
   * ```
   */
  ContractMembers = "ContractMembers",
  /**
   * Represents a node with kind `ContractSpecifier`, having the following structure:
   *
   * ```ebnf
   * ContractSpecifier = (* variant: *) InheritanceSpecifier
   *                   | (* variant: *) StorageLayoutSpecifier; (* Introduced in 0.8.29 *)
   * ```
   */
  ContractSpecifier = "ContractSpecifier",
  /**
   * Represents a node with kind `ContractSpecifiers`, having the following structure:
   *
   * ```ebnf
   * ContractSpecifiers = (* item: *) ContractSpecifier*;
   * ```
   */
  ContractSpecifiers = "ContractSpecifiers",
  /**
   * Represents a node with kind `DecimalNumberExpression`, having the following structure:
   *
   * ```ebnf
   * DecimalNumberExpression = (* literal: *) DECIMAL_LITERAL
   *                           (* unit: *) NumberUnit?;
   * ```
   */
  DecimalNumberExpression = "DecimalNumberExpression",
  /**
   * Represents a node with kind `DoWhileStatement`, having the following structure:
   *
   * ```ebnf
   * DoWhileStatement = (* do_keyword: *) DO_KEYWORD
   *                    (* body: *) Statement
   *                    (* while_keyword: *) WHILE_KEYWORD
   *                    (* open_paren: *) OPEN_PAREN
   *                    (* condition: *) Expression
   *                    (* close_paren: *) CLOSE_PAREN
   *                    (* semicolon: *) SEMICOLON;
   * ```
   */
  DoWhileStatement = "DoWhileStatement",
  /**
   * Represents a node with kind `ElementaryType`, having the following structure:
   *
   * ```ebnf
   * ElementaryType = (* variant: *) BOOL_KEYWORD
   *                | (* variant: *) BYTE_KEYWORD (* Deprecated in 0.8.0 *)
   *                | (* variant: *) STRING_KEYWORD
   *                | (* variant: *) AddressType
   *                | (* variant: *) BYTES_KEYWORD
   *                | (* variant: *) INT_KEYWORD
   *                | (* variant: *) UINT_KEYWORD
   *                | (* variant: *) FIXED_KEYWORD
   *                | (* variant: *) UFIXED_KEYWORD;
   * ```
   */
  ElementaryType = "ElementaryType",
  /**
   * Represents a node with kind `ElseBranch`, having the following structure:
   *
   * ```ebnf
   * ElseBranch = (* else_keyword: *) ELSE_KEYWORD
   *              (* body: *) Statement;
   * ```
   */
  ElseBranch = "ElseBranch",
  /**
   * Represents a node with kind `EmitStatement`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.21 *)
   * EmitStatement = (* emit_keyword: *) EMIT_KEYWORD
   *                 (* event: *) IdentifierPath
   *                 (* arguments: *) ArgumentsDeclaration
   *                 (* semicolon: *) SEMICOLON;
   * ```
   */
  EmitStatement = "EmitStatement",
  /**
   * Represents a node with kind `EnumDefinition`, having the following structure:
   *
   * ```ebnf
   * EnumDefinition = (* enum_keyword: *) ENUM_KEYWORD
   *                  (* name: *) IDENTIFIER
   *                  (* open_brace: *) OPEN_BRACE
   *                  (* members: *) EnumMembers
   *                  (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  EnumDefinition = "EnumDefinition",
  /**
   * Represents a node with kind `EnumMembers`, having the following structure:
   *
   * ```ebnf
   * EnumMembers = ((* item: *) IDENTIFIER ((* separator: *) COMMA (* item: *) IDENTIFIER)*)?;
   * ```
   */
  EnumMembers = "EnumMembers",
  /**
   * Represents a node with kind `EqualityExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * EqualityExpression = (* left_operand: *) Expression
   *                      (* operator: *) EQUAL_EQUAL
   *                      (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * EqualityExpression = (* left_operand: *) Expression
   *                      (* operator: *) BANG_EQUAL
   *                      (* right_operand: *) Expression;
   * ```
   */
  EqualityExpression = "EqualityExpression",
  /**
   * Represents a node with kind `ErrorDefinition`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * ErrorDefinition = (* error_keyword: *) ERROR_KEYWORD
   *                   (* name: *) IDENTIFIER
   *                   (* members: *) ErrorParametersDeclaration
   *                   (* semicolon: *) SEMICOLON;
   * ```
   */
  ErrorDefinition = "ErrorDefinition",
  /**
   * Represents a node with kind `ErrorParameter`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * ErrorParameter = (* type_name: *) TypeName
   *                  (* name: *) IDENTIFIER?;
   * ```
   */
  ErrorParameter = "ErrorParameter",
  /**
   * Represents a node with kind `ErrorParameters`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * ErrorParameters = ((* item: *) ErrorParameter ((* separator: *) COMMA (* item: *) ErrorParameter)*)?;
   * ```
   */
  ErrorParameters = "ErrorParameters",
  /**
   * Represents a node with kind `ErrorParametersDeclaration`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * ErrorParametersDeclaration = (* open_paren: *) OPEN_PAREN
   *                              (* parameters: *) ErrorParameters
   *                              (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  ErrorParametersDeclaration = "ErrorParametersDeclaration",
  /**
   * Represents a node with kind `EventDefinition`, having the following structure:
   *
   * ```ebnf
   * EventDefinition = (* event_keyword: *) EVENT_KEYWORD
   *                   (* name: *) IDENTIFIER
   *                   (* parameters: *) EventParametersDeclaration
   *                   (* anonymous_keyword: *) ANONYMOUS_KEYWORD?
   *                   (* semicolon: *) SEMICOLON;
   * ```
   */
  EventDefinition = "EventDefinition",
  /**
   * Represents a node with kind `EventParameter`, having the following structure:
   *
   * ```ebnf
   * EventParameter = (* type_name: *) TypeName
   *                  (* indexed_keyword: *) INDEXED_KEYWORD?
   *                  (* name: *) IDENTIFIER?;
   * ```
   */
  EventParameter = "EventParameter",
  /**
   * Represents a node with kind `EventParameters`, having the following structure:
   *
   * ```ebnf
   * EventParameters = ((* item: *) EventParameter ((* separator: *) COMMA (* item: *) EventParameter)*)?;
   * ```
   */
  EventParameters = "EventParameters",
  /**
   * Represents a node with kind `EventParametersDeclaration`, having the following structure:
   *
   * ```ebnf
   * EventParametersDeclaration = (* open_paren: *) OPEN_PAREN
   *                              (* parameters: *) EventParameters
   *                              (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  EventParametersDeclaration = "EventParametersDeclaration",
  /**
   * Represents a node with kind `ExperimentalFeature`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.16 *)
   * ExperimentalFeature = (* variant: *) ABI_ENCODER_V2_KEYWORD
   *                     | (* variant: *) SMT_CHECKER_KEYWORD
   *                     | (* variant: *) StringLiteral;
   * ```
   */
  ExperimentalFeature = "ExperimentalFeature",
  /**
   * Represents a node with kind `ExperimentalPragma`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.16 *)
   * ExperimentalPragma = (* experimental_keyword: *) EXPERIMENTAL_KEYWORD
   *                      (* feature: *) ExperimentalFeature;
   * ```
   */
  ExperimentalPragma = "ExperimentalPragma",
  /**
   * Represents a node with kind `ExponentiationExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * (* Deprecated in 0.8.0 *)
   * ExponentiationExpression = (* left_operand: *) Expression
   *                            (* operator: *) ASTERISK_ASTERISK
   *                            (* right_operand: *) Expression;
   *
   * (* Right-associative binary operator *)
   * (* Introduced in 0.8.0 *)
   * ExponentiationExpression = (* left_operand: *) Expression
   *                            (* operator: *) ASTERISK_ASTERISK
   *                            (* right_operand: *) Expression;
   * ```
   */
  ExponentiationExpression = "ExponentiationExpression",
  /**
   * Represents a node with kind `Expression`, having the following structure:
   *
   * ```ebnf
   * Expression = (* variant: *) AssignmentExpression
   *            | (* variant: *) ConditionalExpression
   *            | (* variant: *) OrExpression
   *            | (* variant: *) AndExpression
   *            | (* variant: *) EqualityExpression
   *            | (* variant: *) InequalityExpression
   *            | (* variant: *) BitwiseOrExpression
   *            | (* variant: *) BitwiseXorExpression
   *            | (* variant: *) BitwiseAndExpression
   *            | (* variant: *) ShiftExpression
   *            | (* variant: *) AdditiveExpression
   *            | (* variant: *) MultiplicativeExpression
   *            | (* variant: *) ExponentiationExpression
   *            | (* variant: *) PostfixExpression
   *            | (* variant: *) PrefixExpression
   *            | (* variant: *) FunctionCallExpression
   *            | (* variant: *) CallOptionsExpression
   *            | (* variant: *) MemberAccessExpression
   *            | (* variant: *) IndexAccessExpression
   *            | (* variant: *) NewExpression
   *            | (* variant: *) TupleExpression
   *            | (* variant: *) TypeExpression (* Introduced in 0.5.3 *)
   *            | (* variant: *) ArrayExpression
   *            | (* variant: *) HexNumberExpression
   *            | (* variant: *) DecimalNumberExpression
   *            | (* variant: *) StringExpression
   *            | (* variant: *) ElementaryType
   *            | (* variant: *) PAYABLE_KEYWORD (* Introduced in 0.6.0 *)
   *            | (* variant: *) THIS_KEYWORD
   *            | (* variant: *) SUPER_KEYWORD
   *            | (* variant: *) TRUE_KEYWORD
   *            | (* variant: *) FALSE_KEYWORD
   *            | (* variant: *) IDENTIFIER;
   * ```
   */
  Expression = "Expression",
  /**
   * Represents a node with kind `ExpressionStatement`, having the following structure:
   *
   * ```ebnf
   * ExpressionStatement = (* expression: *) Expression
   *                       (* semicolon: *) SEMICOLON;
   * ```
   */
  ExpressionStatement = "ExpressionStatement",
  /**
   * Represents a node with kind `FallbackFunctionAttribute`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * FallbackFunctionAttribute = (* variant: *) ModifierInvocation
   *                           | (* variant: *) OverrideSpecifier
   *                           | (* variant: *) EXTERNAL_KEYWORD
   *                           | (* variant: *) PAYABLE_KEYWORD
   *                           | (* variant: *) PURE_KEYWORD
   *                           | (* variant: *) VIEW_KEYWORD
   *                           | (* variant: *) VIRTUAL_KEYWORD;
   * ```
   */
  FallbackFunctionAttribute = "FallbackFunctionAttribute",
  /**
   * Represents a node with kind `FallbackFunctionAttributes`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * FallbackFunctionAttributes = (* item: *) FallbackFunctionAttribute*;
   * ```
   */
  FallbackFunctionAttributes = "FallbackFunctionAttributes",
  /**
   * Represents a node with kind `FallbackFunctionDefinition`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * FallbackFunctionDefinition = (* fallback_keyword: *) FALLBACK_KEYWORD
   *                              (* parameters: *) ParametersDeclaration
   *                              (* attributes: *) FallbackFunctionAttributes
   *                              (* returns: *) ReturnsDeclaration?
   *                              (* body: *) FunctionBody;
   * ```
   */
  FallbackFunctionDefinition = "FallbackFunctionDefinition",
  /**
   * Represents a node with kind `ForStatement`, having the following structure:
   *
   * ```ebnf
   * ForStatement = (* for_keyword: *) FOR_KEYWORD
   *                (* open_paren: *) OPEN_PAREN
   *                (* initialization: *) ForStatementInitialization
   *                (* condition: *) ForStatementCondition
   *                (* iterator: *) Expression?
   *                (* close_paren: *) CLOSE_PAREN
   *                (* body: *) Statement;
   * ```
   */
  ForStatement = "ForStatement",
  /**
   * Represents a node with kind `ForStatementCondition`, having the following structure:
   *
   * ```ebnf
   * ForStatementCondition = (* variant: *) ExpressionStatement
   *                       | (* variant: *) SEMICOLON;
   * ```
   */
  ForStatementCondition = "ForStatementCondition",
  /**
   * Represents a node with kind `ForStatementInitialization`, having the following structure:
   *
   * ```ebnf
   * ForStatementInitialization = (* variant: *) TupleDeconstructionStatement
   *                            | (* variant: *) VariableDeclarationStatement
   *                            | (* variant: *) ExpressionStatement
   *                            | (* variant: *) SEMICOLON;
   * ```
   */
  ForStatementInitialization = "ForStatementInitialization",
  /**
   * Represents a node with kind `FunctionAttribute`, having the following structure:
   *
   * ```ebnf
   * FunctionAttribute = (* variant: *) ModifierInvocation
   *                   | (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
   *                   | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
   *                   | (* variant: *) EXTERNAL_KEYWORD
   *                   | (* variant: *) INTERNAL_KEYWORD
   *                   | (* variant: *) PAYABLE_KEYWORD
   *                   | (* variant: *) PRIVATE_KEYWORD
   *                   | (* variant: *) PUBLIC_KEYWORD
   *                   | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 *)
   *                   | (* variant: *) VIEW_KEYWORD (* Introduced in 0.4.16 *)
   *                   | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
   * ```
   */
  FunctionAttribute = "FunctionAttribute",
  /**
   * Represents a node with kind `FunctionAttributes`, having the following structure:
   *
   * ```ebnf
   * FunctionAttributes = (* item: *) FunctionAttribute*;
   * ```
   */
  FunctionAttributes = "FunctionAttributes",
  /**
   * Represents a node with kind `FunctionBody`, having the following structure:
   *
   * ```ebnf
   * FunctionBody = (* variant: *) Block
   *              | (* variant: *) SEMICOLON;
   * ```
   */
  FunctionBody = "FunctionBody",
  /**
   * Represents a node with kind `FunctionCallExpression`, having the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * FunctionCallExpression = (* operand: *) Expression
   *                          (* arguments: *) ArgumentsDeclaration;
   * ```
   */
  FunctionCallExpression = "FunctionCallExpression",
  /**
   * Represents a node with kind `FunctionDefinition`, having the following structure:
   *
   * ```ebnf
   * FunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
   *                      (* name: *) FunctionName
   *                      (* parameters: *) ParametersDeclaration
   *                      (* attributes: *) FunctionAttributes
   *                      (* returns: *) ReturnsDeclaration?
   *                      (* body: *) FunctionBody;
   * ```
   */
  FunctionDefinition = "FunctionDefinition",
  /**
   * Represents a node with kind `FunctionName`, having the following structure:
   *
   * ```ebnf
   * FunctionName = (* variant: *) IDENTIFIER
   *              | (* variant: *) FALLBACK_KEYWORD
   *              | (* variant: *) RECEIVE_KEYWORD;
   * ```
   */
  FunctionName = "FunctionName",
  /**
   * Represents a node with kind `FunctionType`, having the following structure:
   *
   * ```ebnf
   * FunctionType = (* function_keyword: *) FUNCTION_KEYWORD
   *                (* parameters: *) ParametersDeclaration
   *                (* attributes: *) FunctionTypeAttributes
   *                (* returns: *) ReturnsDeclaration?;
   * ```
   */
  FunctionType = "FunctionType",
  /**
   * Represents a node with kind `FunctionTypeAttribute`, having the following structure:
   *
   * ```ebnf
   * FunctionTypeAttribute = (* variant: *) INTERNAL_KEYWORD
   *                       | (* variant: *) EXTERNAL_KEYWORD
   *                       | (* variant: *) PRIVATE_KEYWORD
   *                       | (* variant: *) PUBLIC_KEYWORD
   *                       | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
   *                       | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 *)
   *                       | (* variant: *) VIEW_KEYWORD (* Introduced in 0.4.16 *)
   *                       | (* variant: *) PAYABLE_KEYWORD;
   * ```
   */
  FunctionTypeAttribute = "FunctionTypeAttribute",
  /**
   * Represents a node with kind `FunctionTypeAttributes`, having the following structure:
   *
   * ```ebnf
   * FunctionTypeAttributes = (* item: *) FunctionTypeAttribute*;
   * ```
   */
  FunctionTypeAttributes = "FunctionTypeAttributes",
  /**
   * Represents a node with kind `HexNumberExpression`, having the following structure:
   *
   * ```ebnf
   * HexNumberExpression = (* literal: *) HEX_LITERAL
   *                       (* unit: *) NumberUnit?; (* Deprecated in 0.5.0 *)
   * ```
   */
  HexNumberExpression = "HexNumberExpression",
  /**
   * Represents a node with kind `HexStringLiteral`, having the following structure:
   *
   * ```ebnf
   * HexStringLiteral = (* variant: *) SINGLE_QUOTED_HEX_STRING_LITERAL
   *                  | (* variant: *) DOUBLE_QUOTED_HEX_STRING_LITERAL;
   * ```
   */
  HexStringLiteral = "HexStringLiteral",
  /**
   * Represents a node with kind `HexStringLiterals`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.14 *)
   * HexStringLiterals = (* item: *) HexStringLiteral+;
   * ```
   */
  HexStringLiterals = "HexStringLiterals",
  /**
   * Represents a node with kind `IdentifierPath`, having the following structure:
   *
   * ```ebnf
   * IdentifierPath = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
   * ```
   */
  IdentifierPath = "IdentifierPath",
  /**
   * Represents a node with kind `IfStatement`, having the following structure:
   *
   * ```ebnf
   * IfStatement = (* if_keyword: *) IF_KEYWORD
   *               (* open_paren: *) OPEN_PAREN
   *               (* condition: *) Expression
   *               (* close_paren: *) CLOSE_PAREN
   *               (* body: *) Statement
   *               (* else_branch: *) ElseBranch?;
   * ```
   */
  IfStatement = "IfStatement",
  /**
   * Represents a node with kind `ImportAlias`, having the following structure:
   *
   * ```ebnf
   * ImportAlias = (* as_keyword: *) AS_KEYWORD
   *               (* identifier: *) IDENTIFIER;
   * ```
   */
  ImportAlias = "ImportAlias",
  /**
   * Represents a node with kind `ImportClause`, having the following structure:
   *
   * ```ebnf
   * ImportClause = (* variant: *) PathImport
   *              | (* variant: *) NamedImport
   *              | (* variant: *) ImportDeconstruction;
   * ```
   */
  ImportClause = "ImportClause",
  /**
   * Represents a node with kind `ImportDeconstruction`, having the following structure:
   *
   * ```ebnf
   * ImportDeconstruction = (* open_brace: *) OPEN_BRACE
   *                        (* symbols: *) ImportDeconstructionSymbols
   *                        (* close_brace: *) CLOSE_BRACE
   *                        (* from_keyword: *) FROM_KEYWORD
   *                        (* path: *) StringLiteral;
   * ```
   */
  ImportDeconstruction = "ImportDeconstruction",
  /**
   * Represents a node with kind `ImportDeconstructionSymbol`, having the following structure:
   *
   * ```ebnf
   * ImportDeconstructionSymbol = (* name: *) IDENTIFIER
   *                              (* alias: *) ImportAlias?;
   * ```
   */
  ImportDeconstructionSymbol = "ImportDeconstructionSymbol",
  /**
   * Represents a node with kind `ImportDeconstructionSymbols`, having the following structure:
   *
   * ```ebnf
   * ImportDeconstructionSymbols = (* item: *) ImportDeconstructionSymbol ((* separator: *) COMMA (* item: *) ImportDeconstructionSymbol)*;
   * ```
   */
  ImportDeconstructionSymbols = "ImportDeconstructionSymbols",
  /**
   * Represents a node with kind `ImportDirective`, having the following structure:
   *
   * ```ebnf
   * ImportDirective = (* import_keyword: *) IMPORT_KEYWORD
   *                   (* clause: *) ImportClause
   *                   (* semicolon: *) SEMICOLON;
   * ```
   */
  ImportDirective = "ImportDirective",
  /**
   * Represents a node with kind `IndexAccessEnd`, having the following structure:
   *
   * ```ebnf
   * IndexAccessEnd = (* colon: *) COLON
   *                  (* end: *) Expression?;
   * ```
   */
  IndexAccessEnd = "IndexAccessEnd",
  /**
   * Represents a node with kind `IndexAccessExpression`, having the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * IndexAccessExpression = (* operand: *) Expression
   *                         (* open_bracket: *) OPEN_BRACKET
   *                         (* start: *) Expression?
   *                         (* end: *) IndexAccessEnd?
   *                         (* close_bracket: *) CLOSE_BRACKET;
   * ```
   */
  IndexAccessExpression = "IndexAccessExpression",
  /**
   * Represents a node with kind `InequalityExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * InequalityExpression = (* left_operand: *) Expression
   *                        (* operator: *) LESS_THAN
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * InequalityExpression = (* left_operand: *) Expression
   *                        (* operator: *) GREATER_THAN
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * InequalityExpression = (* left_operand: *) Expression
   *                        (* operator: *) LESS_THAN_EQUAL
   *                        (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * InequalityExpression = (* left_operand: *) Expression
   *                        (* operator: *) GREATER_THAN_EQUAL
   *                        (* right_operand: *) Expression;
   * ```
   */
  InequalityExpression = "InequalityExpression",
  /**
   * Represents a node with kind `InheritanceSpecifier`, having the following structure:
   *
   * ```ebnf
   * InheritanceSpecifier = (* is_keyword: *) IS_KEYWORD
   *                        (* types: *) InheritanceTypes;
   * ```
   */
  InheritanceSpecifier = "InheritanceSpecifier",
  /**
   * Represents a node with kind `InheritanceType`, having the following structure:
   *
   * ```ebnf
   * InheritanceType = (* type_name: *) IdentifierPath
   *                   (* arguments: *) ArgumentsDeclaration?;
   * ```
   */
  InheritanceType = "InheritanceType",
  /**
   * Represents a node with kind `InheritanceTypes`, having the following structure:
   *
   * ```ebnf
   * InheritanceTypes = (* item: *) InheritanceType ((* separator: *) COMMA (* item: *) InheritanceType)*;
   * ```
   */
  InheritanceTypes = "InheritanceTypes",
  /**
   * Represents a node with kind `InterfaceDefinition`, having the following structure:
   *
   * ```ebnf
   * InterfaceDefinition = (* interface_keyword: *) INTERFACE_KEYWORD
   *                       (* name: *) IDENTIFIER
   *                       (* inheritance: *) InheritanceSpecifier?
   *                       (* open_brace: *) OPEN_BRACE
   *                       (* members: *) InterfaceMembers
   *                       (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  InterfaceDefinition = "InterfaceDefinition",
  /**
   * Represents a node with kind `InterfaceMembers`, having the following structure:
   *
   * ```ebnf
   * InterfaceMembers = (* item: *) ContractMember*;
   * ```
   */
  InterfaceMembers = "InterfaceMembers",
  /**
   * Represents a node with kind `LibraryDefinition`, having the following structure:
   *
   * ```ebnf
   * LibraryDefinition = (* library_keyword: *) LIBRARY_KEYWORD
   *                     (* name: *) IDENTIFIER
   *                     (* open_brace: *) OPEN_BRACE
   *                     (* members: *) LibraryMembers
   *                     (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  LibraryDefinition = "LibraryDefinition",
  /**
   * Represents a node with kind `LibraryMembers`, having the following structure:
   *
   * ```ebnf
   * LibraryMembers = (* item: *) ContractMember*;
   * ```
   */
  LibraryMembers = "LibraryMembers",
  /**
   * Represents a node with kind `MappingKey`, having the following structure:
   *
   * ```ebnf
   * MappingKey = (* key_type: *) MappingKeyType
   *              (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
   * ```
   */
  MappingKey = "MappingKey",
  /**
   * Represents a node with kind `MappingKeyType`, having the following structure:
   *
   * ```ebnf
   * MappingKeyType = (* variant: *) ElementaryType
   *                | (* variant: *) IdentifierPath;
   * ```
   */
  MappingKeyType = "MappingKeyType",
  /**
   * Represents a node with kind `MappingType`, having the following structure:
   *
   * ```ebnf
   * MappingType = (* mapping_keyword: *) MAPPING_KEYWORD
   *               (* open_paren: *) OPEN_PAREN
   *               (* key_type: *) MappingKey
   *               (* equal_greater_than: *) EQUAL_GREATER_THAN
   *               (* value_type: *) MappingValue
   *               (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  MappingType = "MappingType",
  /**
   * Represents a node with kind `MappingValue`, having the following structure:
   *
   * ```ebnf
   * MappingValue = (* type_name: *) TypeName
   *                (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
   * ```
   */
  MappingValue = "MappingValue",
  /**
   * Represents a node with kind `MemberAccessExpression`, having the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * MemberAccessExpression = (* operand: *) Expression
   *                          (* period: *) PERIOD
   *                          (* member: *) IDENTIFIER;
   * ```
   */
  MemberAccessExpression = "MemberAccessExpression",
  /**
   * Represents a node with kind `ModifierAttribute`, having the following structure:
   *
   * ```ebnf
   * ModifierAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
   *                   | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
   * ```
   */
  ModifierAttribute = "ModifierAttribute",
  /**
   * Represents a node with kind `ModifierAttributes`, having the following structure:
   *
   * ```ebnf
   * ModifierAttributes = (* item: *) ModifierAttribute*;
   * ```
   */
  ModifierAttributes = "ModifierAttributes",
  /**
   * Represents a node with kind `ModifierDefinition`, having the following structure:
   *
   * ```ebnf
   * ModifierDefinition = (* modifier_keyword: *) MODIFIER_KEYWORD
   *                      (* name: *) IDENTIFIER
   *                      (* parameters: *) ParametersDeclaration?
   *                      (* attributes: *) ModifierAttributes
   *                      (* body: *) FunctionBody;
   * ```
   */
  ModifierDefinition = "ModifierDefinition",
  /**
   * Represents a node with kind `ModifierInvocation`, having the following structure:
   *
   * ```ebnf
   * ModifierInvocation = (* name: *) IdentifierPath
   *                      (* arguments: *) ArgumentsDeclaration?;
   * ```
   */
  ModifierInvocation = "ModifierInvocation",
  /**
   * Represents a node with kind `MultiplicativeExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * MultiplicativeExpression = (* left_operand: *) Expression
   *                            (* operator: *) ASTERISK
   *                            (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * MultiplicativeExpression = (* left_operand: *) Expression
   *                            (* operator: *) SLASH
   *                            (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * MultiplicativeExpression = (* left_operand: *) Expression
   *                            (* operator: *) PERCENT
   *                            (* right_operand: *) Expression;
   * ```
   */
  MultiplicativeExpression = "MultiplicativeExpression",
  /**
   * Represents a node with kind `NamedArgument`, having the following structure:
   *
   * ```ebnf
   * NamedArgument = (* name: *) IDENTIFIER
   *                 (* colon: *) COLON
   *                 (* value: *) Expression;
   * ```
   */
  NamedArgument = "NamedArgument",
  /**
   * Represents a node with kind `NamedArgumentGroup`, having the following structure:
   *
   * ```ebnf
   * NamedArgumentGroup = (* open_brace: *) OPEN_BRACE
   *                      (* arguments: *) NamedArguments
   *                      (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  NamedArgumentGroup = "NamedArgumentGroup",
  /**
   * Represents a node with kind `NamedArguments`, having the following structure:
   *
   * ```ebnf
   * NamedArguments = ((* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*)?;
   * ```
   */
  NamedArguments = "NamedArguments",
  /**
   * Represents a node with kind `NamedArgumentsDeclaration`, having the following structure:
   *
   * ```ebnf
   * NamedArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
   *                             (* arguments: *) NamedArgumentGroup?
   *                             (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  NamedArgumentsDeclaration = "NamedArgumentsDeclaration",
  /**
   * Represents a node with kind `NamedImport`, having the following structure:
   *
   * ```ebnf
   * NamedImport = (* asterisk: *) ASTERISK
   *               (* alias: *) ImportAlias
   *               (* from_keyword: *) FROM_KEYWORD
   *               (* path: *) StringLiteral;
   * ```
   */
  NamedImport = "NamedImport",
  /**
   * Represents a node with kind `NewExpression`, having the following structure:
   *
   * ```ebnf
   * NewExpression = (* new_keyword: *) NEW_KEYWORD
   *                 (* type_name: *) TypeName;
   * ```
   */
  NewExpression = "NewExpression",
  /**
   * Represents a node with kind `NumberUnit`, having the following structure:
   *
   * ```ebnf
   * NumberUnit = (* variant: *) WEI_KEYWORD
   *            | (* variant: *) GWEI_KEYWORD (* Introduced in 0.6.11 *)
   *            | (* variant: *) SZABO_KEYWORD (* Deprecated in 0.7.0 *)
   *            | (* variant: *) FINNEY_KEYWORD (* Deprecated in 0.7.0 *)
   *            | (* variant: *) ETHER_KEYWORD
   *            | (* variant: *) SECONDS_KEYWORD
   *            | (* variant: *) MINUTES_KEYWORD
   *            | (* variant: *) HOURS_KEYWORD
   *            | (* variant: *) DAYS_KEYWORD
   *            | (* variant: *) WEEKS_KEYWORD
   *            | (* variant: *) YEARS_KEYWORD; (* Deprecated in 0.5.0 *)
   * ```
   */
  NumberUnit = "NumberUnit",
  /**
   * Represents a node with kind `OrExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * OrExpression = (* left_operand: *) Expression
   *                (* operator: *) BAR_BAR
   *                (* right_operand: *) Expression;
   * ```
   */
  OrExpression = "OrExpression",
  /**
   * Represents a node with kind `OverridePaths`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * OverridePaths = (* item: *) IdentifierPath ((* separator: *) COMMA (* item: *) IdentifierPath)*;
   * ```
   */
  OverridePaths = "OverridePaths",
  /**
   * Represents a node with kind `OverridePathsDeclaration`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * OverridePathsDeclaration = (* open_paren: *) OPEN_PAREN
   *                            (* paths: *) OverridePaths
   *                            (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  OverridePathsDeclaration = "OverridePathsDeclaration",
  /**
   * Represents a node with kind `OverrideSpecifier`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * OverrideSpecifier = (* override_keyword: *) OVERRIDE_KEYWORD
   *                     (* overridden: *) OverridePathsDeclaration?;
   * ```
   */
  OverrideSpecifier = "OverrideSpecifier",
  /**
   * Represents a node with kind `Parameter`, having the following structure:
   *
   * ```ebnf
   * Parameter = (* type_name: *) TypeName
   *             (* storage_location: *) StorageLocation?
   *             (* name: *) IDENTIFIER?;
   * ```
   */
  Parameter = "Parameter",
  /**
   * Represents a node with kind `Parameters`, having the following structure:
   *
   * ```ebnf
   * Parameters = ((* item: *) Parameter ((* separator: *) COMMA (* item: *) Parameter)*)?;
   * ```
   */
  Parameters = "Parameters",
  /**
   * Represents a node with kind `ParametersDeclaration`, having the following structure:
   *
   * ```ebnf
   * ParametersDeclaration = (* open_paren: *) OPEN_PAREN
   *                         (* parameters: *) Parameters
   *                         (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  ParametersDeclaration = "ParametersDeclaration",
  /**
   * Represents a node with kind `PathImport`, having the following structure:
   *
   * ```ebnf
   * PathImport = (* path: *) StringLiteral
   *              (* alias: *) ImportAlias?;
   * ```
   */
  PathImport = "PathImport",
  /**
   * Represents a node with kind `PositionalArguments`, having the following structure:
   *
   * ```ebnf
   * PositionalArguments = ((* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*)?;
   * ```
   */
  PositionalArguments = "PositionalArguments",
  /**
   * Represents a node with kind `PositionalArgumentsDeclaration`, having the following structure:
   *
   * ```ebnf
   * PositionalArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
   *                                  (* arguments: *) PositionalArguments
   *                                  (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  PositionalArgumentsDeclaration = "PositionalArgumentsDeclaration",
  /**
   * Represents a node with kind `PostfixExpression`, having the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * PostfixExpression = (* operand: *) Expression
   *                     (* operator: *) PLUS_PLUS;
   *
   * (* Postfix unary operator *)
   * PostfixExpression = (* operand: *) Expression
   *                     (* operator: *) MINUS_MINUS;
   * ```
   */
  PostfixExpression = "PostfixExpression",
  /**
   * Represents a node with kind `Pragma`, having the following structure:
   *
   * ```ebnf
   * Pragma = (* variant: *) VersionPragma
   *        | (* variant: *) AbicoderPragma (* Introduced in 0.7.5 *)
   *        | (* variant: *) ExperimentalPragma; (* Introduced in 0.4.16 *)
   * ```
   */
  Pragma = "Pragma",
  /**
   * Represents a node with kind `PragmaDirective`, having the following structure:
   *
   * ```ebnf
   * PragmaDirective = (* pragma_keyword: *) PRAGMA_KEYWORD
   *                   (* pragma: *) Pragma
   *                   (* semicolon: *) SEMICOLON;
   * ```
   */
  PragmaDirective = "PragmaDirective",
  /**
   * Represents a node with kind `PrefixExpression`, having the following structure:
   *
   * ```ebnf
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) PLUS_PLUS
   *                    (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) MINUS_MINUS
   *                    (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) TILDE
   *                    (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) BANG
   *                    (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) MINUS
   *                    (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * (* Deprecated in 0.5.0 *)
   * PrefixExpression = (* operator: *) PLUS
   *                    (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) DELETE_KEYWORD
   *                    (* operand: *) Expression;
   * ```
   */
  PrefixExpression = "PrefixExpression",
  /**
   * Represents a node with kind `ReceiveFunctionAttribute`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * ReceiveFunctionAttribute = (* variant: *) ModifierInvocation
   *                          | (* variant: *) OverrideSpecifier
   *                          | (* variant: *) EXTERNAL_KEYWORD
   *                          | (* variant: *) PAYABLE_KEYWORD
   *                          | (* variant: *) VIRTUAL_KEYWORD;
   * ```
   */
  ReceiveFunctionAttribute = "ReceiveFunctionAttribute",
  /**
   * Represents a node with kind `ReceiveFunctionAttributes`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * ReceiveFunctionAttributes = (* item: *) ReceiveFunctionAttribute*;
   * ```
   */
  ReceiveFunctionAttributes = "ReceiveFunctionAttributes",
  /**
   * Represents a node with kind `ReceiveFunctionDefinition`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * ReceiveFunctionDefinition = (* receive_keyword: *) RECEIVE_KEYWORD
   *                             (* parameters: *) ParametersDeclaration
   *                             (* attributes: *) ReceiveFunctionAttributes
   *                             (* body: *) FunctionBody;
   * ```
   */
  ReceiveFunctionDefinition = "ReceiveFunctionDefinition",
  /**
   * Represents a node with kind `ReturnStatement`, having the following structure:
   *
   * ```ebnf
   * ReturnStatement = (* return_keyword: *) RETURN_KEYWORD
   *                   (* expression: *) Expression?
   *                   (* semicolon: *) SEMICOLON;
   * ```
   */
  ReturnStatement = "ReturnStatement",
  /**
   * Represents a node with kind `ReturnsDeclaration`, having the following structure:
   *
   * ```ebnf
   * ReturnsDeclaration = (* returns_keyword: *) RETURNS_KEYWORD
   *                      (* variables: *) ParametersDeclaration;
   * ```
   */
  ReturnsDeclaration = "ReturnsDeclaration",
  /**
   * Represents a node with kind `RevertStatement`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * RevertStatement = (* revert_keyword: *) REVERT_KEYWORD
   *                   (* error: *) IdentifierPath
   *                   (* arguments: *) ArgumentsDeclaration
   *                   (* semicolon: *) SEMICOLON;
   * ```
   */
  RevertStatement = "RevertStatement",
  /**
   * Represents a node with kind `ShiftExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * ShiftExpression = (* left_operand: *) Expression
   *                   (* operator: *) LESS_THAN_LESS_THAN
   *                   (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * ShiftExpression = (* left_operand: *) Expression
   *                   (* operator: *) GREATER_THAN_GREATER_THAN
   *                   (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * ShiftExpression = (* left_operand: *) Expression
   *                   (* operator: *) GREATER_THAN_GREATER_THAN_GREATER_THAN
   *                   (* right_operand: *) Expression;
   * ```
   */
  ShiftExpression = "ShiftExpression",
  /**
   * Represents a node with kind `SimpleVersionLiteral`, having the following structure:
   *
   * ```ebnf
   * SimpleVersionLiteral = (* item: *) VERSION_SPECIFIER ((* separator: *) PERIOD (* item: *) VERSION_SPECIFIER)*;
   * ```
   */
  SimpleVersionLiteral = "SimpleVersionLiteral",
  /**
   * Represents a node with kind `SourceUnit`, having the following structure:
   *
   * ```ebnf
   * SourceUnit = (* members: *) SourceUnitMembers;
   * ```
   */
  SourceUnit = "SourceUnit",
  /**
   * Represents a node with kind `SourceUnitMember`, having the following structure:
   *
   * ```ebnf
   * SourceUnitMember = (* variant: *) PragmaDirective
   *                  | (* variant: *) ImportDirective
   *                  | (* variant: *) ContractDefinition
   *                  | (* variant: *) InterfaceDefinition
   *                  | (* variant: *) LibraryDefinition
   *                  | (* variant: *) StructDefinition (* Introduced in 0.6.0 *)
   *                  | (* variant: *) EnumDefinition (* Introduced in 0.6.0 *)
   *                  | (* variant: *) FunctionDefinition (* Introduced in 0.7.1 *)
   *                  | (* variant: *) ErrorDefinition (* Introduced in 0.8.4 *)
   *                  | (* variant: *) UserDefinedValueTypeDefinition (* Introduced in 0.8.8 *)
   *                  | (* variant: *) UsingDirective (* Introduced in 0.8.13 *)
   *                  | (* variant: *) EventDefinition (* Introduced in 0.8.22 *)
   *                  | (* variant: *) ConstantDefinition; (* Introduced in 0.7.4 *)
   * ```
   */
  SourceUnitMember = "SourceUnitMember",
  /**
   * Represents a node with kind `SourceUnitMembers`, having the following structure:
   *
   * ```ebnf
   * SourceUnitMembers = (* item: *) SourceUnitMember*;
   * ```
   */
  SourceUnitMembers = "SourceUnitMembers",
  /**
   * Represents a node with kind `StateVariableAttribute`, having the following structure:
   *
   * ```ebnf
   * StateVariableAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
   *                        | (* variant: *) CONSTANT_KEYWORD
   *                        | (* variant: *) INTERNAL_KEYWORD
   *                        | (* variant: *) PRIVATE_KEYWORD
   *                        | (* variant: *) PUBLIC_KEYWORD
   *                        | (* variant: *) IMMUTABLE_KEYWORD (* Introduced in 0.6.5 *)
   *                        | (* variant: *) TRANSIENT_KEYWORD; (* Introduced in 0.8.27 *)
   * ```
   */
  StateVariableAttribute = "StateVariableAttribute",
  /**
   * Represents a node with kind `StateVariableAttributes`, having the following structure:
   *
   * ```ebnf
   * StateVariableAttributes = (* item: *) StateVariableAttribute*;
   * ```
   */
  StateVariableAttributes = "StateVariableAttributes",
  /**
   * Represents a node with kind `StateVariableDefinition`, having the following structure:
   *
   * ```ebnf
   * StateVariableDefinition = (* type_name: *) TypeName
   *                           (* attributes: *) StateVariableAttributes
   *                           (* name: *) IDENTIFIER
   *                           (* value: *) StateVariableDefinitionValue?
   *                           (* semicolon: *) SEMICOLON;
   * ```
   */
  StateVariableDefinition = "StateVariableDefinition",
  /**
   * Represents a node with kind `StateVariableDefinitionValue`, having the following structure:
   *
   * ```ebnf
   * StateVariableDefinitionValue = (* equal: *) EQUAL
   *                                (* value: *) Expression;
   * ```
   */
  StateVariableDefinitionValue = "StateVariableDefinitionValue",
  /**
   * Represents a node with kind `Statement`, having the following structure:
   *
   * ```ebnf
   * Statement = (* variant: *) IfStatement
   *           | (* variant: *) ForStatement
   *           | (* variant: *) WhileStatement
   *           | (* variant: *) DoWhileStatement
   *           | (* variant: *) ContinueStatement
   *           | (* variant: *) BreakStatement
   *           | (* variant: *) ReturnStatement
   *           | (* variant: *) ThrowStatement (* Deprecated in 0.5.0 *)
   *           | (* variant: *) EmitStatement (* Introduced in 0.4.21 *)
   *           | (* variant: *) TryStatement (* Introduced in 0.6.0 *)
   *           | (* variant: *) RevertStatement (* Introduced in 0.8.4 *)
   *           | (* variant: *) AssemblyStatement
   *           | (* variant: *) Block
   *           | (* variant: *) UncheckedBlock (* Introduced in 0.8.0 *)
   *           | (* variant: *) TupleDeconstructionStatement
   *           | (* variant: *) VariableDeclarationStatement
   *           | (* variant: *) ExpressionStatement;
   * ```
   */
  Statement = "Statement",
  /**
   * Represents a node with kind `Statements`, having the following structure:
   *
   * ```ebnf
   * Statements = (* item: *) Statement*;
   * ```
   */
  Statements = "Statements",
  /**
   * Represents a node with kind `StorageLayoutSpecifier`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.29 *)
   * StorageLayoutSpecifier = (* layout_keyword: *) LAYOUT_KEYWORD
   *                          (* at_keyword: *) AT_KEYWORD
   *                          (* expression: *) Expression;
   * ```
   */
  StorageLayoutSpecifier = "StorageLayoutSpecifier",
  /**
   * Represents a node with kind `StorageLocation`, having the following structure:
   *
   * ```ebnf
   * StorageLocation = (* variant: *) MEMORY_KEYWORD
   *                 | (* variant: *) STORAGE_KEYWORD
   *                 | (* variant: *) CALL_DATA_KEYWORD; (* Introduced in 0.5.0 *)
   * ```
   */
  StorageLocation = "StorageLocation",
  /**
   * Represents a node with kind `StringExpression`, having the following structure:
   *
   * ```ebnf
   * StringExpression = (* variant: *) StringLiteral (* Deprecated in 0.5.14 *)
   *                  | (* variant: *) StringLiterals (* Introduced in 0.5.14 *)
   *                  | (* variant: *) HexStringLiteral (* Deprecated in 0.5.14 *)
   *                  | (* variant: *) HexStringLiterals (* Introduced in 0.5.14 *)
   *                  | (* variant: *) UnicodeStringLiterals; (* Introduced in 0.7.0 *)
   * ```
   */
  StringExpression = "StringExpression",
  /**
   * Represents a node with kind `StringLiteral`, having the following structure:
   *
   * ```ebnf
   * StringLiteral = (* variant: *) SINGLE_QUOTED_STRING_LITERAL
   *               | (* variant: *) DOUBLE_QUOTED_STRING_LITERAL;
   * ```
   */
  StringLiteral = "StringLiteral",
  /**
   * Represents a node with kind `StringLiterals`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.14 *)
   * StringLiterals = (* item: *) StringLiteral+;
   * ```
   */
  StringLiterals = "StringLiterals",
  /**
   * Represents a node with kind `StructDefinition`, having the following structure:
   *
   * ```ebnf
   * StructDefinition = (* struct_keyword: *) STRUCT_KEYWORD
   *                    (* name: *) IDENTIFIER
   *                    (* open_brace: *) OPEN_BRACE
   *                    (* members: *) StructMembers
   *                    (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  StructDefinition = "StructDefinition",
  /**
   * Represents a node with kind `StructMember`, having the following structure:
   *
   * ```ebnf
   * StructMember = (* type_name: *) TypeName
   *                (* name: *) IDENTIFIER
   *                (* semicolon: *) SEMICOLON;
   * ```
   */
  StructMember = "StructMember",
  /**
   * Represents a node with kind `StructMembers`, having the following structure:
   *
   * ```ebnf
   * StructMembers = (* item: *) StructMember*;
   * ```
   */
  StructMembers = "StructMembers",
  /**
   * Represents a node with kind `ThrowStatement`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * ThrowStatement = (* throw_keyword: *) THROW_KEYWORD
   *                  (* semicolon: *) SEMICOLON;
   * ```
   */
  ThrowStatement = "ThrowStatement",
  /**
   * Represents a node with kind `TryStatement`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * TryStatement = (* try_keyword: *) TRY_KEYWORD
   *                (* expression: *) Expression
   *                (* returns: *) ReturnsDeclaration?
   *                (* body: *) Block
   *                (* catch_clauses: *) CatchClauses;
   * ```
   */
  TryStatement = "TryStatement",
  /**
   * Represents a node with kind `TupleDeconstructionElement`, having the following structure:
   *
   * ```ebnf
   * TupleDeconstructionElement = (* member: *) TupleMember?;
   * ```
   */
  TupleDeconstructionElement = "TupleDeconstructionElement",
  /**
   * Represents a node with kind `TupleDeconstructionElements`, having the following structure:
   *
   * ```ebnf
   * TupleDeconstructionElements = (* item: *) TupleDeconstructionElement ((* separator: *) COMMA (* item: *) TupleDeconstructionElement)*;
   * ```
   */
  TupleDeconstructionElements = "TupleDeconstructionElements",
  /**
   * Represents a node with kind `TupleDeconstructionStatement`, having the following structure:
   *
   * ```ebnf
   * TupleDeconstructionStatement = (* var_keyword: *) VAR_KEYWORD? (* Deprecated in 0.5.0 *)
   *                                (* open_paren: *) OPEN_PAREN
   *                                (* elements: *) TupleDeconstructionElements
   *                                (* close_paren: *) CLOSE_PAREN
   *                                (* equal: *) EQUAL
   *                                (* expression: *) Expression
   *                                (* semicolon: *) SEMICOLON;
   * ```
   */
  TupleDeconstructionStatement = "TupleDeconstructionStatement",
  /**
   * Represents a node with kind `TupleExpression`, having the following structure:
   *
   * ```ebnf
   * TupleExpression = (* open_paren: *) OPEN_PAREN
   *                   (* items: *) TupleValues
   *                   (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  TupleExpression = "TupleExpression",
  /**
   * Represents a node with kind `TupleMember`, having the following structure:
   *
   * ```ebnf
   * TupleMember = (* variant: *) TypedTupleMember
   *             | (* variant: *) UntypedTupleMember;
   * ```
   */
  TupleMember = "TupleMember",
  /**
   * Represents a node with kind `TupleValue`, having the following structure:
   *
   * ```ebnf
   * TupleValue = (* expression: *) Expression?;
   * ```
   */
  TupleValue = "TupleValue",
  /**
   * Represents a node with kind `TupleValues`, having the following structure:
   *
   * ```ebnf
   * TupleValues = (* item: *) TupleValue ((* separator: *) COMMA (* item: *) TupleValue)*;
   * ```
   */
  TupleValues = "TupleValues",
  /**
   * Represents a node with kind `TypeExpression`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.3 *)
   * TypeExpression = (* type_keyword: *) TYPE_KEYWORD
   *                  (* open_paren: *) OPEN_PAREN
   *                  (* type_name: *) TypeName
   *                  (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  TypeExpression = "TypeExpression",
  /**
   * Represents a node with kind `TypeName`, having the following structure:
   *
   * ```ebnf
   * TypeName = (* variant: *) ArrayTypeName
   *          | (* variant: *) FunctionType
   *          | (* variant: *) MappingType
   *          | (* variant: *) ElementaryType
   *          | (* variant: *) IdentifierPath;
   * ```
   */
  TypeName = "TypeName",
  /**
   * Represents a node with kind `TypedTupleMember`, having the following structure:
   *
   * ```ebnf
   * TypedTupleMember = (* type_name: *) TypeName
   *                    (* storage_location: *) StorageLocation?
   *                    (* name: *) IDENTIFIER;
   * ```
   */
  TypedTupleMember = "TypedTupleMember",
  /**
   * Represents a node with kind `UncheckedBlock`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.0 *)
   * UncheckedBlock = (* unchecked_keyword: *) UNCHECKED_KEYWORD
   *                  (* block: *) Block;
   * ```
   */
  UncheckedBlock = "UncheckedBlock",
  /**
   * Represents a node with kind `UnicodeStringLiteral`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.0 *)
   * UnicodeStringLiteral = (* variant: *) SINGLE_QUOTED_UNICODE_STRING_LITERAL
   *                      | (* variant: *) DOUBLE_QUOTED_UNICODE_STRING_LITERAL;
   * ```
   */
  UnicodeStringLiteral = "UnicodeStringLiteral",
  /**
   * Represents a node with kind `UnicodeStringLiterals`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.0 *)
   * UnicodeStringLiterals = (* item: *) UnicodeStringLiteral+;
   * ```
   */
  UnicodeStringLiterals = "UnicodeStringLiterals",
  /**
   * Represents a node with kind `UnnamedFunctionAttribute`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.6.0 *)
   * UnnamedFunctionAttribute = (* variant: *) ModifierInvocation
   *                          | (* variant: *) EXTERNAL_KEYWORD
   *                          | (* variant: *) INTERNAL_KEYWORD (* Deprecated in 0.5.0 *)
   *                          | (* variant: *) PAYABLE_KEYWORD
   *                          | (* variant: *) PRIVATE_KEYWORD (* Deprecated in 0.5.0 *)
   *                          | (* variant: *) PUBLIC_KEYWORD; (* Deprecated in 0.5.0 *)
   * ```
   */
  UnnamedFunctionAttribute = "UnnamedFunctionAttribute",
  /**
   * Represents a node with kind `UnnamedFunctionAttributes`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.6.0 *)
   * UnnamedFunctionAttributes = (* item: *) UnnamedFunctionAttribute*;
   * ```
   */
  UnnamedFunctionAttributes = "UnnamedFunctionAttributes",
  /**
   * Represents a node with kind `UnnamedFunctionDefinition`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.6.0 *)
   * UnnamedFunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
   *                             (* parameters: *) ParametersDeclaration
   *                             (* attributes: *) UnnamedFunctionAttributes
   *                             (* body: *) FunctionBody;
   * ```
   */
  UnnamedFunctionDefinition = "UnnamedFunctionDefinition",
  /**
   * Represents a node with kind `UntypedTupleMember`, having the following structure:
   *
   * ```ebnf
   * UntypedTupleMember = (* storage_location: *) StorageLocation?
   *                      (* name: *) IDENTIFIER;
   * ```
   */
  UntypedTupleMember = "UntypedTupleMember",
  /**
   * Represents a node with kind `UserDefinedValueTypeDefinition`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.8 *)
   * UserDefinedValueTypeDefinition = (* type_keyword: *) TYPE_KEYWORD
   *                                  (* name: *) IDENTIFIER
   *                                  (* is_keyword: *) IS_KEYWORD
   *                                  (* value_type: *) ElementaryType
   *                                  (* semicolon: *) SEMICOLON;
   * ```
   */
  UserDefinedValueTypeDefinition = "UserDefinedValueTypeDefinition",
  /**
   * Represents a node with kind `UsingAlias`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.19 *)
   * UsingAlias = (* as_keyword: *) AS_KEYWORD
   *              (* operator: *) UsingOperator;
   * ```
   */
  UsingAlias = "UsingAlias",
  /**
   * Represents a node with kind `UsingClause`, having the following structure:
   *
   * ```ebnf
   * UsingClause = (* variant: *) IdentifierPath
   *             | (* variant: *) UsingDeconstruction; (* Introduced in 0.8.13 *)
   * ```
   */
  UsingClause = "UsingClause",
  /**
   * Represents a node with kind `UsingDeconstruction`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * UsingDeconstruction = (* open_brace: *) OPEN_BRACE
   *                       (* symbols: *) UsingDeconstructionSymbols
   *                       (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  UsingDeconstruction = "UsingDeconstruction",
  /**
   * Represents a node with kind `UsingDeconstructionSymbol`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * UsingDeconstructionSymbol = (* name: *) IdentifierPath
   *                             (* alias: *) UsingAlias?; (* Introduced in 0.8.19 *)
   * ```
   */
  UsingDeconstructionSymbol = "UsingDeconstructionSymbol",
  /**
   * Represents a node with kind `UsingDeconstructionSymbols`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * UsingDeconstructionSymbols = (* item: *) UsingDeconstructionSymbol ((* separator: *) COMMA (* item: *) UsingDeconstructionSymbol)*;
   * ```
   */
  UsingDeconstructionSymbols = "UsingDeconstructionSymbols",
  /**
   * Represents a node with kind `UsingDirective`, having the following structure:
   *
   * ```ebnf
   * UsingDirective = (* using_keyword: *) USING_KEYWORD
   *                  (* clause: *) UsingClause
   *                  (* for_keyword: *) FOR_KEYWORD
   *                  (* target: *) UsingTarget
   *                  (* global_keyword: *) GLOBAL_KEYWORD? (* Introduced in 0.8.13 *)
   *                  (* semicolon: *) SEMICOLON;
   * ```
   */
  UsingDirective = "UsingDirective",
  /**
   * Represents a node with kind `UsingOperator`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.19 *)
   * UsingOperator = (* variant: *) AMPERSAND
   *               | (* variant: *) ASTERISK
   *               | (* variant: *) BANG_EQUAL
   *               | (* variant: *) BAR
   *               | (* variant: *) CARET
   *               | (* variant: *) EQUAL_EQUAL
   *               | (* variant: *) GREATER_THAN
   *               | (* variant: *) GREATER_THAN_EQUAL
   *               | (* variant: *) LESS_THAN
   *               | (* variant: *) LESS_THAN_EQUAL
   *               | (* variant: *) MINUS
   *               | (* variant: *) PERCENT
   *               | (* variant: *) PLUS
   *               | (* variant: *) SLASH
   *               | (* variant: *) TILDE;
   * ```
   */
  UsingOperator = "UsingOperator",
  /**
   * Represents a node with kind `UsingTarget`, having the following structure:
   *
   * ```ebnf
   * UsingTarget = (* variant: *) TypeName
   *             | (* variant: *) ASTERISK;
   * ```
   */
  UsingTarget = "UsingTarget",
  /**
   * Represents a node with kind `VariableDeclarationStatement`, having the following structure:
   *
   * ```ebnf
   * VariableDeclarationStatement = (* variable_type: *) VariableDeclarationType
   *                                (* storage_location: *) StorageLocation?
   *                                (* name: *) IDENTIFIER
   *                                (* value: *) VariableDeclarationValue?
   *                                (* semicolon: *) SEMICOLON;
   * ```
   */
  VariableDeclarationStatement = "VariableDeclarationStatement",
  /**
   * Represents a node with kind `VariableDeclarationType`, having the following structure:
   *
   * ```ebnf
   * VariableDeclarationType = (* variant: *) TypeName
   *                         | (* variant: *) VAR_KEYWORD; (* Deprecated in 0.5.0 *)
   * ```
   */
  VariableDeclarationType = "VariableDeclarationType",
  /**
   * Represents a node with kind `VariableDeclarationValue`, having the following structure:
   *
   * ```ebnf
   * VariableDeclarationValue = (* equal: *) EQUAL
   *                            (* expression: *) Expression;
   * ```
   */
  VariableDeclarationValue = "VariableDeclarationValue",
  /**
   * Represents a node with kind `VersionExpression`, having the following structure:
   *
   * ```ebnf
   * VersionExpression = (* variant: *) VersionRange
   *                   | (* variant: *) VersionTerm;
   * ```
   */
  VersionExpression = "VersionExpression",
  /**
   * Represents a node with kind `VersionExpressionSet`, having the following structure:
   *
   * ```ebnf
   * VersionExpressionSet = (* item: *) VersionExpression+;
   * ```
   */
  VersionExpressionSet = "VersionExpressionSet",
  /**
   * Represents a node with kind `VersionExpressionSets`, having the following structure:
   *
   * ```ebnf
   * VersionExpressionSets = (* item: *) VersionExpressionSet ((* separator: *) BAR_BAR (* item: *) VersionExpressionSet)*;
   * ```
   */
  VersionExpressionSets = "VersionExpressionSets",
  /**
   * Represents a node with kind `VersionLiteral`, having the following structure:
   *
   * ```ebnf
   * VersionLiteral = (* variant: *) SimpleVersionLiteral
   *                | (* variant: *) SINGLE_QUOTED_VERSION_LITERAL
   *                | (* variant: *) DOUBLE_QUOTED_VERSION_LITERAL;
   * ```
   */
  VersionLiteral = "VersionLiteral",
  /**
   * Represents a node with kind `VersionOperator`, having the following structure:
   *
   * ```ebnf
   * VersionOperator = (* variant: *) CARET
   *                 | (* variant: *) TILDE
   *                 | (* variant: *) EQUAL
   *                 | (* variant: *) LESS_THAN
   *                 | (* variant: *) GREATER_THAN
   *                 | (* variant: *) LESS_THAN_EQUAL
   *                 | (* variant: *) GREATER_THAN_EQUAL;
   * ```
   */
  VersionOperator = "VersionOperator",
  /**
   * Represents a node with kind `VersionPragma`, having the following structure:
   *
   * ```ebnf
   * VersionPragma = (* solidity_keyword: *) SOLIDITY_KEYWORD
   *                 (* sets: *) VersionExpressionSets;
   * ```
   */
  VersionPragma = "VersionPragma",
  /**
   * Represents a node with kind `VersionRange`, having the following structure:
   *
   * ```ebnf
   * VersionRange = (* start: *) VersionLiteral
   *                (* minus: *) MINUS
   *                (* end: *) VersionLiteral;
   * ```
   */
  VersionRange = "VersionRange",
  /**
   * Represents a node with kind `VersionTerm`, having the following structure:
   *
   * ```ebnf
   * VersionTerm = (* operator: *) VersionOperator?
   *               (* literal: *) VersionLiteral;
   * ```
   */
  VersionTerm = "VersionTerm",
  /**
   * Represents a node with kind `WhileStatement`, having the following structure:
   *
   * ```ebnf
   * WhileStatement = (* while_keyword: *) WHILE_KEYWORD
   *                  (* open_paren: *) OPEN_PAREN
   *                  (* condition: *) Expression
   *                  (* close_paren: *) CLOSE_PAREN
   *                  (* body: *) Statement;
   * ```
   */
  WhileStatement = "WhileStatement",
  /**
   * Represents a node with kind `YulArguments`, having the following structure:
   *
   * ```ebnf
   * YulArguments = ((* item: *) YulExpression ((* separator: *) COMMA (* item: *) YulExpression)*)?;
   * ```
   */
  YulArguments = "YulArguments",
  /**
   * Represents a node with kind `YulAssignmentOperator`, having the following structure:
   *
   * ```ebnf
   * YulAssignmentOperator = (* variant: *) COLON_EQUAL
   *                       | (* variant: *) YulColonAndEqual; (* Deprecated in 0.5.5 *)
   * ```
   */
  YulAssignmentOperator = "YulAssignmentOperator",
  /**
   * Represents a node with kind `YulBlock`, having the following structure:
   *
   * ```ebnf
   * YulBlock = (* open_brace: *) OPEN_BRACE
   *            (* statements: *) YulStatements
   *            (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  YulBlock = "YulBlock",
  /**
   * Represents a node with kind `YulBreakStatement`, having the following structure:
   *
   * ```ebnf
   * YulBreakStatement = (* break_keyword: *) YUL_BREAK_KEYWORD;
   * ```
   */
  YulBreakStatement = "YulBreakStatement",
  /**
   * Represents a node with kind `YulColonAndEqual`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.5 *)
   * YulColonAndEqual = (* colon: *) COLON
   *                    (* equal: *) EQUAL;
   * ```
   */
  YulColonAndEqual = "YulColonAndEqual",
  /**
   * Represents a node with kind `YulContinueStatement`, having the following structure:
   *
   * ```ebnf
   * YulContinueStatement = (* continue_keyword: *) YUL_CONTINUE_KEYWORD;
   * ```
   */
  YulContinueStatement = "YulContinueStatement",
  /**
   * Represents a node with kind `YulDefaultCase`, having the following structure:
   *
   * ```ebnf
   * YulDefaultCase = (* default_keyword: *) YUL_DEFAULT_KEYWORD
   *                  (* body: *) YulBlock;
   * ```
   */
  YulDefaultCase = "YulDefaultCase",
  /**
   * Represents a node with kind `YulEqualAndColon`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YulEqualAndColon = (* equal: *) EQUAL
   *                    (* colon: *) COLON;
   * ```
   */
  YulEqualAndColon = "YulEqualAndColon",
  /**
   * Represents a node with kind `YulExpression`, having the following structure:
   *
   * ```ebnf
   * YulExpression = (* variant: *) YulFunctionCallExpression
   *               | (* variant: *) YulLiteral
   *               | (* variant: *) YulPath;
   * ```
   */
  YulExpression = "YulExpression",
  /**
   * Represents a node with kind `YulForStatement`, having the following structure:
   *
   * ```ebnf
   * YulForStatement = (* for_keyword: *) YUL_FOR_KEYWORD
   *                   (* initialization: *) YulBlock
   *                   (* condition: *) YulExpression
   *                   (* iterator: *) YulBlock
   *                   (* body: *) YulBlock;
   * ```
   */
  YulForStatement = "YulForStatement",
  /**
   * Represents a node with kind `YulFunctionCallExpression`, having the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * YulFunctionCallExpression = (* operand: *) YulExpression
   *                             (* open_paren: *) OPEN_PAREN
   *                             (* arguments: *) YulArguments
   *                             (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  YulFunctionCallExpression = "YulFunctionCallExpression",
  /**
   * Represents a node with kind `YulFunctionDefinition`, having the following structure:
   *
   * ```ebnf
   * YulFunctionDefinition = (* function_keyword: *) YUL_FUNCTION_KEYWORD
   *                         (* name: *) YUL_IDENTIFIER
   *                         (* parameters: *) YulParametersDeclaration
   *                         (* returns: *) YulReturnsDeclaration?
   *                         (* body: *) YulBlock;
   * ```
   */
  YulFunctionDefinition = "YulFunctionDefinition",
  /**
   * Represents a node with kind `YulIfStatement`, having the following structure:
   *
   * ```ebnf
   * YulIfStatement = (* if_keyword: *) YUL_IF_KEYWORD
   *                  (* condition: *) YulExpression
   *                  (* body: *) YulBlock;
   * ```
   */
  YulIfStatement = "YulIfStatement",
  /**
   * Represents a node with kind `YulLabel`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YulLabel = (* label: *) YUL_IDENTIFIER
   *            (* colon: *) COLON;
   * ```
   */
  YulLabel = "YulLabel",
  /**
   * Represents a node with kind `YulLeaveStatement`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * YulLeaveStatement = (* leave_keyword: *) YUL_LEAVE_KEYWORD;
   * ```
   */
  YulLeaveStatement = "YulLeaveStatement",
  /**
   * Represents a node with kind `YulLiteral`, having the following structure:
   *
   * ```ebnf
   * YulLiteral = (* variant: *) YUL_TRUE_KEYWORD (* Introduced in 0.6.2 *)
   *            | (* variant: *) YUL_FALSE_KEYWORD (* Introduced in 0.6.2 *)
   *            | (* variant: *) YUL_DECIMAL_LITERAL
   *            | (* variant: *) YUL_HEX_LITERAL
   *            | (* variant: *) HexStringLiteral
   *            | (* variant: *) StringLiteral;
   * ```
   */
  YulLiteral = "YulLiteral",
  /**
   * Represents a node with kind `YulParameters`, having the following structure:
   *
   * ```ebnf
   * YulParameters = ((* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*)?;
   * ```
   */
  YulParameters = "YulParameters",
  /**
   * Represents a node with kind `YulParametersDeclaration`, having the following structure:
   *
   * ```ebnf
   * YulParametersDeclaration = (* open_paren: *) OPEN_PAREN
   *                            (* parameters: *) YulParameters
   *                            (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  YulParametersDeclaration = "YulParametersDeclaration",
  /**
   * Represents a node with kind `YulPath`, having the following structure:
   *
   * ```ebnf
   * YulPath = (* item: *) YUL_IDENTIFIER ((* separator: *) PERIOD (* item: *) YUL_IDENTIFIER)*;
   * ```
   */
  YulPath = "YulPath",
  /**
   * Represents a node with kind `YulPaths`, having the following structure:
   *
   * ```ebnf
   * YulPaths = (* item: *) YulPath ((* separator: *) COMMA (* item: *) YulPath)*;
   * ```
   */
  YulPaths = "YulPaths",
  /**
   * Represents a node with kind `YulReturnsDeclaration`, having the following structure:
   *
   * ```ebnf
   * YulReturnsDeclaration = (* minus_greater_than: *) MINUS_GREATER_THAN
   *                         (* variables: *) YulVariableNames;
   * ```
   */
  YulReturnsDeclaration = "YulReturnsDeclaration",
  /**
   * Represents a node with kind `YulStackAssignmentOperator`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YulStackAssignmentOperator = (* variant: *) EQUAL_COLON
   *                            | (* variant: *) YulEqualAndColon;
   * ```
   */
  YulStackAssignmentOperator = "YulStackAssignmentOperator",
  /**
   * Represents a node with kind `YulStackAssignmentStatement`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YulStackAssignmentStatement = (* assignment: *) YulStackAssignmentOperator
   *                               (* variable: *) YUL_IDENTIFIER;
   * ```
   */
  YulStackAssignmentStatement = "YulStackAssignmentStatement",
  /**
   * Represents a node with kind `YulStatement`, having the following structure:
   *
   * ```ebnf
   * YulStatement = (* variant: *) YulBlock
   *              | (* variant: *) YulFunctionDefinition
   *              | (* variant: *) YulStackAssignmentStatement (* Deprecated in 0.5.0 *)
   *              | (* variant: *) YulIfStatement
   *              | (* variant: *) YulForStatement
   *              | (* variant: *) YulSwitchStatement
   *              | (* variant: *) YulLeaveStatement (* Introduced in 0.6.0 *)
   *              | (* variant: *) YulBreakStatement
   *              | (* variant: *) YulContinueStatement
   *              | (* variant: *) YulVariableAssignmentStatement
   *              | (* variant: *) YulLabel (* Deprecated in 0.5.0 *)
   *              | (* variant: *) YulVariableDeclarationStatement
   *              | (* variant: *) YulExpression;
   * ```
   */
  YulStatement = "YulStatement",
  /**
   * Represents a node with kind `YulStatements`, having the following structure:
   *
   * ```ebnf
   * YulStatements = (* item: *) YulStatement*;
   * ```
   */
  YulStatements = "YulStatements",
  /**
   * Represents a node with kind `YulSwitchCase`, having the following structure:
   *
   * ```ebnf
   * YulSwitchCase = (* variant: *) YulDefaultCase
   *               | (* variant: *) YulValueCase;
   * ```
   */
  YulSwitchCase = "YulSwitchCase",
  /**
   * Represents a node with kind `YulSwitchCases`, having the following structure:
   *
   * ```ebnf
   * YulSwitchCases = (* item: *) YulSwitchCase+;
   * ```
   */
  YulSwitchCases = "YulSwitchCases",
  /**
   * Represents a node with kind `YulSwitchStatement`, having the following structure:
   *
   * ```ebnf
   * YulSwitchStatement = (* switch_keyword: *) YUL_SWITCH_KEYWORD
   *                      (* expression: *) YulExpression
   *                      (* cases: *) YulSwitchCases;
   * ```
   */
  YulSwitchStatement = "YulSwitchStatement",
  /**
   * Represents a node with kind `YulValueCase`, having the following structure:
   *
   * ```ebnf
   * YulValueCase = (* case_keyword: *) YUL_CASE_KEYWORD
   *                (* value: *) YulLiteral
   *                (* body: *) YulBlock;
   * ```
   */
  YulValueCase = "YulValueCase",
  /**
   * Represents a node with kind `YulVariableAssignmentStatement`, having the following structure:
   *
   * ```ebnf
   * YulVariableAssignmentStatement = (* variables: *) YulPaths
   *                                  (* assignment: *) YulAssignmentOperator
   *                                  (* expression: *) YulExpression;
   * ```
   */
  YulVariableAssignmentStatement = "YulVariableAssignmentStatement",
  /**
   * Represents a node with kind `YulVariableDeclarationStatement`, having the following structure:
   *
   * ```ebnf
   * YulVariableDeclarationStatement = (* let_keyword: *) YUL_LET_KEYWORD
   *                                   (* variables: *) YulVariableNames
   *                                   (* value: *) YulVariableDeclarationValue?;
   * ```
   */
  YulVariableDeclarationStatement = "YulVariableDeclarationStatement",
  /**
   * Represents a node with kind `YulVariableDeclarationValue`, having the following structure:
   *
   * ```ebnf
   * YulVariableDeclarationValue = (* assignment: *) YulAssignmentOperator
   *                               (* expression: *) YulExpression;
   * ```
   */
  YulVariableDeclarationValue = "YulVariableDeclarationValue",
  /**
   * Represents a node with kind `YulVariableNames`, having the following structure:
   *
   * ```ebnf
   * YulVariableNames = (* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*;
   * ```
   */
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
   * This terminal is created when the parser is expecting a certain terminal but does not find it.
   * Adding the missing input in this position may allow the parser to produce a valid tree there.
   */
  Missing = "Missing",
  /**
   * Represents a node with kind `ABIEncoderV2Keyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.16 *)
   * (* Never reserved *)
   * ABI_ENCODER_V2_KEYWORD = "ABIEncoderV2";
   * ```
   */
  ABIEncoderV2Keyword = "ABIEncoderV2Keyword",
  /**
   * Represents a node with kind `AbicoderKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.5 *)
   * (* Never reserved *)
   * ABICODER_KEYWORD = "abicoder";
   * ```
   */
  AbicoderKeyword = "AbicoderKeyword",
  /**
   * Represents a node with kind `AbicoderV1Keyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.5 *)
   * (* Never reserved *)
   * ABICODER_V1_KEYWORD = "v1";
   * ```
   */
  AbicoderV1Keyword = "AbicoderV1Keyword",
  /**
   * Represents a node with kind `AbicoderV2Keyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.5 *)
   * (* Never reserved *)
   * ABICODER_V2_KEYWORD = "v2";
   * ```
   */
  AbicoderV2Keyword = "AbicoderV2Keyword",
  /**
   * Represents a node with kind `AbstractKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * ABSTRACT_KEYWORD = "abstract";
   * ```
   */
  AbstractKeyword = "AbstractKeyword",
  /**
   * Represents a node with kind `AddressKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Never reserved *)
   * ADDRESS_KEYWORD = "address";
   * ```
   */
  AddressKeyword = "AddressKeyword",
  /**
   * Represents a node with kind `AfterKeyword`, having the following structure:
   *
   * ```ebnf
   * AFTER_KEYWORD = "after";
   * ```
   */
  AfterKeyword = "AfterKeyword",
  /**
   * Represents a node with kind `AliasKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * ALIAS_KEYWORD = "alias";
   * ```
   */
  AliasKeyword = "AliasKeyword",
  /**
   * Represents a node with kind `Ampersand`, having the following structure:
   *
   * ```ebnf
   * AMPERSAND = "&";
   * ```
   */
  Ampersand = "Ampersand",
  /**
   * Represents a node with kind `AmpersandAmpersand`, having the following structure:
   *
   * ```ebnf
   * AMPERSAND_AMPERSAND = "&&";
   * ```
   */
  AmpersandAmpersand = "AmpersandAmpersand",
  /**
   * Represents a node with kind `AmpersandEqual`, having the following structure:
   *
   * ```ebnf
   * AMPERSAND_EQUAL = "&=";
   * ```
   */
  AmpersandEqual = "AmpersandEqual",
  /**
   * Represents a node with kind `AnonymousKeyword`, having the following structure:
   *
   * ```ebnf
   * ANONYMOUS_KEYWORD = "anonymous";
   * ```
   */
  AnonymousKeyword = "AnonymousKeyword",
  /**
   * Represents a node with kind `ApplyKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * APPLY_KEYWORD = "apply";
   * ```
   */
  ApplyKeyword = "ApplyKeyword",
  /**
   * Represents a node with kind `AsKeyword`, having the following structure:
   *
   * ```ebnf
   * AS_KEYWORD = "as";
   * ```
   */
  AsKeyword = "AsKeyword",
  /**
   * Represents a node with kind `AssemblyKeyword`, having the following structure:
   *
   * ```ebnf
   * ASSEMBLY_KEYWORD = "assembly";
   * ```
   */
  AssemblyKeyword = "AssemblyKeyword",
  /**
   * Represents a node with kind `Asterisk`, having the following structure:
   *
   * ```ebnf
   * ASTERISK = "*";
   * ```
   */
  Asterisk = "Asterisk",
  /**
   * Represents a node with kind `AsteriskAsterisk`, having the following structure:
   *
   * ```ebnf
   * ASTERISK_ASTERISK = "**";
   * ```
   */
  AsteriskAsterisk = "AsteriskAsterisk",
  /**
   * Represents a node with kind `AsteriskEqual`, having the following structure:
   *
   * ```ebnf
   * ASTERISK_EQUAL = "*=";
   * ```
   */
  AsteriskEqual = "AsteriskEqual",
  /**
   * Represents a node with kind `AtKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.29 *)
   * (* Never reserved *)
   * AT_KEYWORD = "at";
   * ```
   */
  AtKeyword = "AtKeyword",
  /**
   * Represents a node with kind `AutoKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * AUTO_KEYWORD = "auto";
   * ```
   */
  AutoKeyword = "AutoKeyword",
  /**
   * Represents a node with kind `Bang`, having the following structure:
   *
   * ```ebnf
   * BANG = "!";
   * ```
   */
  Bang = "Bang",
  /**
   * Represents a node with kind `BangEqual`, having the following structure:
   *
   * ```ebnf
   * BANG_EQUAL = "!=";
   * ```
   */
  BangEqual = "BangEqual",
  /**
   * Represents a node with kind `Bar`, having the following structure:
   *
   * ```ebnf
   * BAR = "|";
   * ```
   */
  Bar = "Bar",
  /**
   * Represents a node with kind `BarBar`, having the following structure:
   *
   * ```ebnf
   * BAR_BAR = "||";
   * ```
   */
  BarBar = "BarBar",
  /**
   * Represents a node with kind `BarEqual`, having the following structure:
   *
   * ```ebnf
   * BAR_EQUAL = "|=";
   * ```
   */
  BarEqual = "BarEqual",
  /**
   * Represents a node with kind `BoolKeyword`, having the following structure:
   *
   * ```ebnf
   * BOOL_KEYWORD = "bool";
   * ```
   */
  BoolKeyword = "BoolKeyword",
  /**
   * Represents a node with kind `BreakKeyword`, having the following structure:
   *
   * ```ebnf
   * BREAK_KEYWORD = "break";
   * ```
   */
  BreakKeyword = "BreakKeyword",
  /**
   * Represents a node with kind `ByteKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.8.0 *)
   * BYTE_KEYWORD = "byte";
   * ```
   */
  ByteKeyword = "ByteKeyword",
  /**
   * Represents a node with kind `BytesKeyword`, having the following structure:
   *
   * ```ebnf
   * BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
   * ```
   */
  BytesKeyword = "BytesKeyword",
  /**
   * Represents a node with kind `CallDataKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.0 *)
   * (* Reserved in 0.5.0 *)
   * CALL_DATA_KEYWORD = "calldata";
   * ```
   */
  CallDataKeyword = "CallDataKeyword",
  /**
   * Represents a node with kind `Caret`, having the following structure:
   *
   * ```ebnf
   * CARET = "^";
   * ```
   */
  Caret = "Caret",
  /**
   * Represents a node with kind `CaretEqual`, having the following structure:
   *
   * ```ebnf
   * CARET_EQUAL = "^=";
   * ```
   */
  CaretEqual = "CaretEqual",
  /**
   * Represents a node with kind `CaseKeyword`, having the following structure:
   *
   * ```ebnf
   * CASE_KEYWORD = "case";
   * ```
   */
  CaseKeyword = "CaseKeyword",
  /**
   * Represents a node with kind `CatchKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * CATCH_KEYWORD = "catch";
   * ```
   */
  CatchKeyword = "CatchKeyword",
  /**
   * Represents a node with kind `CloseBrace`, having the following structure:
   *
   * ```ebnf
   * CLOSE_BRACE = "}";
   * ```
   */
  CloseBrace = "CloseBrace",
  /**
   * Represents a node with kind `CloseBracket`, having the following structure:
   *
   * ```ebnf
   * CLOSE_BRACKET = "]";
   * ```
   */
  CloseBracket = "CloseBracket",
  /**
   * Represents a node with kind `CloseParen`, having the following structure:
   *
   * ```ebnf
   * CLOSE_PAREN = ")";
   * ```
   */
  CloseParen = "CloseParen",
  /**
   * Represents a node with kind `Colon`, having the following structure:
   *
   * ```ebnf
   * COLON = ":";
   * ```
   */
  Colon = "Colon",
  /**
   * Represents a node with kind `ColonEqual`, having the following structure:
   *
   * ```ebnf
   * COLON_EQUAL = ":=";
   * ```
   */
  ColonEqual = "ColonEqual",
  /**
   * Represents a node with kind `Comma`, having the following structure:
   *
   * ```ebnf
   * COMMA = ",";
   * ```
   */
  Comma = "Comma",
  /**
   * Represents a node with kind `ConstantKeyword`, having the following structure:
   *
   * ```ebnf
   * CONSTANT_KEYWORD = "constant";
   * ```
   */
  ConstantKeyword = "ConstantKeyword",
  /**
   * Represents a node with kind `ConstructorKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.22 *)
   * (* Reserved in 0.5.0 *)
   * CONSTRUCTOR_KEYWORD = "constructor";
   * ```
   */
  ConstructorKeyword = "ConstructorKeyword",
  /**
   * Represents a node with kind `ContinueKeyword`, having the following structure:
   *
   * ```ebnf
   * CONTINUE_KEYWORD = "continue";
   * ```
   */
  ContinueKeyword = "ContinueKeyword",
  /**
   * Represents a node with kind `ContractKeyword`, having the following structure:
   *
   * ```ebnf
   * CONTRACT_KEYWORD = "contract";
   * ```
   */
  ContractKeyword = "ContractKeyword",
  /**
   * Represents a node with kind `CopyOfKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * COPY_OF_KEYWORD = "copyof";
   * ```
   */
  CopyOfKeyword = "CopyOfKeyword",
  /**
   * Represents a node with kind `DaysKeyword`, having the following structure:
   *
   * ```ebnf
   * DAYS_KEYWORD = "days";
   * ```
   */
  DaysKeyword = "DaysKeyword",
  /**
   * Represents a node with kind `DecimalLiteral`, having the following structure:
   *
   * ```ebnf
   * DECIMAL_LITERAL = "." «DECIMAL_DIGITS» «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
   *
   * DECIMAL_LITERAL = «DECIMAL_DIGITS» (?!".") «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
   *
   * (* Deprecated in 0.5.0 *)
   * DECIMAL_LITERAL = «DECIMAL_DIGITS» "." (?!«DECIMAL_DIGITS») «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
   *
   * (* Deprecated in 0.5.0 *)
   * DECIMAL_LITERAL = «DECIMAL_DIGITS» "." «DECIMAL_DIGITS» «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
   *
   * (* Introduced in 0.5.0 *)
   * DECIMAL_LITERAL = «DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»)? «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
   * ```
   */
  DecimalLiteral = "DecimalLiteral",
  /**
   * Represents a node with kind `DefaultKeyword`, having the following structure:
   *
   * ```ebnf
   * DEFAULT_KEYWORD = "default";
   * ```
   */
  DefaultKeyword = "DefaultKeyword",
  /**
   * Represents a node with kind `DefineKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * DEFINE_KEYWORD = "define";
   * ```
   */
  DefineKeyword = "DefineKeyword",
  /**
   * Represents a node with kind `DeleteKeyword`, having the following structure:
   *
   * ```ebnf
   * DELETE_KEYWORD = "delete";
   * ```
   */
  DeleteKeyword = "DeleteKeyword",
  /**
   * Represents a node with kind `DoKeyword`, having the following structure:
   *
   * ```ebnf
   * DO_KEYWORD = "do";
   * ```
   */
  DoKeyword = "DoKeyword",
  /**
   * Represents a node with kind `DoubleQuotedHexStringLiteral`, having the following structure:
   *
   * ```ebnf
   * DOUBLE_QUOTED_HEX_STRING_LITERAL = 'hex"' «HEX_STRING_CONTENTS»? '"';
   * ```
   */
  DoubleQuotedHexStringLiteral = "DoubleQuotedHexStringLiteral",
  /**
   * Represents a node with kind `DoubleQuotedStringLiteral`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.4.25 *)
   * DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE_ARBITRARY» | !('"' | "\\" | "\r" | "\n"))* '"';
   *
   * (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
   * DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | !('"' | "\\" | "\r" | "\n"))* '"';
   *
   * DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | " "…"!" | "#"…"[" | "]"…"~")* '"';
   * ```
   */
  DoubleQuotedStringLiteral = "DoubleQuotedStringLiteral",
  /**
   * Represents a node with kind `DoubleQuotedUnicodeStringLiteral`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.0 *)
   * DOUBLE_QUOTED_UNICODE_STRING_LITERAL = 'unicode"' («ESCAPE_SEQUENCE» | !('"' | "\\" | "\r" | "\n"))* '"';
   * ```
   */
  DoubleQuotedUnicodeStringLiteral = "DoubleQuotedUnicodeStringLiteral",
  /**
   * Represents a node with kind `DoubleQuotedVersionLiteral`, having the following structure:
   *
   * ```ebnf
   * DOUBLE_QUOTED_VERSION_LITERAL = '"' «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* '"';
   * ```
   */
  DoubleQuotedVersionLiteral = "DoubleQuotedVersionLiteral",
  /**
   * Represents a node with kind `ElseKeyword`, having the following structure:
   *
   * ```ebnf
   * ELSE_KEYWORD = "else";
   * ```
   */
  ElseKeyword = "ElseKeyword",
  /**
   * Represents a node with kind `EmitKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.21 *)
   * (* Reserved in 0.5.0 *)
   * EMIT_KEYWORD = "emit";
   * ```
   */
  EmitKeyword = "EmitKeyword",
  /**
   * Represents a node with kind `EndOfLine`, having the following structure:
   *
   * ```ebnf
   * END_OF_LINE = "\n" | ("\r" "\n"?);
   * ```
   */
  EndOfLine = "EndOfLine",
  /**
   * Represents a node with kind `EnumKeyword`, having the following structure:
   *
   * ```ebnf
   * ENUM_KEYWORD = "enum";
   * ```
   */
  EnumKeyword = "EnumKeyword",
  /**
   * Represents a node with kind `Equal`, having the following structure:
   *
   * ```ebnf
   * EQUAL = "=";
   * ```
   */
  Equal = "Equal",
  /**
   * Represents a node with kind `EqualColon`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * EQUAL_COLON = "=:";
   * ```
   */
  EqualColon = "EqualColon",
  /**
   * Represents a node with kind `EqualEqual`, having the following structure:
   *
   * ```ebnf
   * EQUAL_EQUAL = "==";
   * ```
   */
  EqualEqual = "EqualEqual",
  /**
   * Represents a node with kind `EqualGreaterThan`, having the following structure:
   *
   * ```ebnf
   * EQUAL_GREATER_THAN = "=>";
   * ```
   */
  EqualGreaterThan = "EqualGreaterThan",
  /**
   * Represents a node with kind `ErrorKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * (* Never reserved *)
   * ERROR_KEYWORD = "error";
   * ```
   */
  ErrorKeyword = "ErrorKeyword",
  /**
   * Represents a node with kind `EtherKeyword`, having the following structure:
   *
   * ```ebnf
   * ETHER_KEYWORD = "ether";
   * ```
   */
  EtherKeyword = "EtherKeyword",
  /**
   * Represents a node with kind `EventKeyword`, having the following structure:
   *
   * ```ebnf
   * EVENT_KEYWORD = "event";
   * ```
   */
  EventKeyword = "EventKeyword",
  /**
   * Represents a node with kind `ExperimentalKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.16 *)
   * (* Never reserved *)
   * EXPERIMENTAL_KEYWORD = "experimental";
   * ```
   */
  ExperimentalKeyword = "ExperimentalKeyword",
  /**
   * Represents a node with kind `ExternalKeyword`, having the following structure:
   *
   * ```ebnf
   * EXTERNAL_KEYWORD = "external";
   * ```
   */
  ExternalKeyword = "ExternalKeyword",
  /**
   * Represents a node with kind `FallbackKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.6.0 *)
   * FALLBACK_KEYWORD = "fallback";
   * ```
   */
  FallbackKeyword = "FallbackKeyword",
  /**
   * Represents a node with kind `FalseKeyword`, having the following structure:
   *
   * ```ebnf
   * FALSE_KEYWORD = "false";
   * ```
   */
  FalseKeyword = "FalseKeyword",
  /**
   * Represents a node with kind `FinalKeyword`, having the following structure:
   *
   * ```ebnf
   * FINAL_KEYWORD = "final";
   * ```
   */
  FinalKeyword = "FinalKeyword",
  /**
   * Represents a node with kind `FinneyKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.7.0 *)
   * (* Reserved until 0.7.0 *)
   * FINNEY_KEYWORD = "finney";
   * ```
   */
  FinneyKeyword = "FinneyKeyword",
  /**
   * Represents a node with kind `FixedKeyword`, having the following structure:
   *
   * ```ebnf
   * FIXED_KEYWORD = "fixed";
   *
   * FIXED_KEYWORD = "fixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176") "x" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80");
   *
   * FIXED_KEYWORD = "fixed" ("184x8" | "184x16" | "184x24" | "184x32" | "184x40" | "184x48" | "184x56" | "184x64" | "184x72" | "192x8" | "192x16" | "192x24" | "192x32" | "192x40" | "192x48" | "192x56" | "192x64" | "200x8" | "200x16" | "200x24" | "200x32" | "200x40" | "200x48" | "200x56" | "208x8" | "208x16" | "208x24" | "208x32" | "208x40" | "208x48" | "216x8" | "216x16" | "216x24" | "216x32" | "216x40" | "224x8" | "224x16" | "224x24" | "224x32" | "232x8" | "232x16" | "232x24" | "240x8" | "240x16" | "248x8");
   *
   * (* Reserved in 0.4.14 *)
   * FIXED_KEYWORD = "fixed" ("184x80" | "192x72" | "192x80" | "200x64" | "200x72" | "200x80" | "208x56" | "208x64" | "208x72" | "208x80" | "216x48" | "216x56" | "216x64" | "216x72" | "216x80" | "224x40" | "224x48" | "224x56" | "224x64" | "224x72" | "224x80" | "232x32" | "232x40" | "232x48" | "232x56" | "232x64" | "232x72" | "232x80" | "240x24" | "240x32" | "240x40" | "240x48" | "240x56" | "240x64" | "240x72" | "240x80" | "248x16" | "248x24" | "248x32" | "248x40" | "248x48" | "248x56" | "248x64" | "248x72" | "248x80" | "256x8" | "256x16" | "256x24" | "256x32" | "256x40" | "256x48" | "256x56" | "256x64" | "256x72" | "256x80");
   *
   * (* Reserved in 0.4.14 *)
   * FIXED_KEYWORD = "fixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256") "x" ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "33" | "34" | "35" | "36" | "37" | "38" | "39" | "41" | "42" | "43" | "44" | "45" | "46" | "47" | "49" | "50" | "51" | "52" | "53" | "54" | "55" | "57" | "58" | "59" | "60" | "61" | "62" | "63" | "65" | "66" | "67" | "68" | "69" | "70" | "71" | "73" | "74" | "75" | "76" | "77" | "78" | "79");
   * ```
   */
  FixedKeyword = "FixedKeyword",
  /**
   * Represents a node with kind `ForKeyword`, having the following structure:
   *
   * ```ebnf
   * FOR_KEYWORD = "for";
   * ```
   */
  ForKeyword = "ForKeyword",
  /**
   * Represents a node with kind `FromKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Never reserved *)
   * FROM_KEYWORD = "from";
   * ```
   */
  FromKeyword = "FromKeyword",
  /**
   * Represents a node with kind `FunctionKeyword`, having the following structure:
   *
   * ```ebnf
   * FUNCTION_KEYWORD = "function";
   * ```
   */
  FunctionKeyword = "FunctionKeyword",
  /**
   * Represents a node with kind `GlobalKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * (* Never reserved *)
   * GLOBAL_KEYWORD = "global";
   * ```
   */
  GlobalKeyword = "GlobalKeyword",
  /**
   * Represents a node with kind `GreaterThan`, having the following structure:
   *
   * ```ebnf
   * GREATER_THAN = ">";
   * ```
   */
  GreaterThan = "GreaterThan",
  /**
   * Represents a node with kind `GreaterThanEqual`, having the following structure:
   *
   * ```ebnf
   * GREATER_THAN_EQUAL = ">=";
   * ```
   */
  GreaterThanEqual = "GreaterThanEqual",
  /**
   * Represents a node with kind `GreaterThanGreaterThan`, having the following structure:
   *
   * ```ebnf
   * GREATER_THAN_GREATER_THAN = ">>";
   * ```
   */
  GreaterThanGreaterThan = "GreaterThanGreaterThan",
  /**
   * Represents a node with kind `GreaterThanGreaterThanEqual`, having the following structure:
   *
   * ```ebnf
   * GREATER_THAN_GREATER_THAN_EQUAL = ">>=";
   * ```
   */
  GreaterThanGreaterThanEqual = "GreaterThanGreaterThanEqual",
  /**
   * Represents a node with kind `GreaterThanGreaterThanGreaterThan`, having the following structure:
   *
   * ```ebnf
   * GREATER_THAN_GREATER_THAN_GREATER_THAN = ">>>";
   * ```
   */
  GreaterThanGreaterThanGreaterThan = "GreaterThanGreaterThanGreaterThan",
  /**
   * Represents a node with kind `GreaterThanGreaterThanGreaterThanEqual`, having the following structure:
   *
   * ```ebnf
   * GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL = ">>>=";
   * ```
   */
  GreaterThanGreaterThanGreaterThanEqual = "GreaterThanGreaterThanGreaterThanEqual",
  /**
   * Represents a node with kind `GweiKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.11 *)
   * (* Reserved in 0.7.0 *)
   * GWEI_KEYWORD = "gwei";
   * ```
   */
  GweiKeyword = "GweiKeyword",
  /**
   * Represents a node with kind `HexKeyword`, having the following structure:
   *
   * ```ebnf
   * HEX_KEYWORD = "hex";
   * ```
   */
  HexKeyword = "HexKeyword",
  /**
   * Represents a node with kind `HexLiteral`, having the following structure:
   *
   * ```ebnf
   * HEX_LITERAL = "0x" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)* (?!«IDENTIFIER_START»);
   *
   * (* Deprecated in 0.5.0 *)
   * HEX_LITERAL = "0X" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)* (?!«IDENTIFIER_START»);
   * ```
   */
  HexLiteral = "HexLiteral",
  /**
   * Represents a node with kind `HoursKeyword`, having the following structure:
   *
   * ```ebnf
   * HOURS_KEYWORD = "hours";
   * ```
   */
  HoursKeyword = "HoursKeyword",
  /**
   * Represents a node with kind `Identifier`, having the following structure:
   *
   * ```ebnf
   * IDENTIFIER = «IDENTIFIER_START» «IDENTIFIER_PART»*;
   * ```
   */
  Identifier = "Identifier",
  /**
   * Represents a node with kind `IfKeyword`, having the following structure:
   *
   * ```ebnf
   * IF_KEYWORD = "if";
   * ```
   */
  IfKeyword = "IfKeyword",
  /**
   * Represents a node with kind `ImmutableKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.5 *)
   * (* Reserved in 0.5.0 *)
   * IMMUTABLE_KEYWORD = "immutable";
   * ```
   */
  ImmutableKeyword = "ImmutableKeyword",
  /**
   * Represents a node with kind `ImplementsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * IMPLEMENTS_KEYWORD = "implements";
   * ```
   */
  ImplementsKeyword = "ImplementsKeyword",
  /**
   * Represents a node with kind `ImportKeyword`, having the following structure:
   *
   * ```ebnf
   * IMPORT_KEYWORD = "import";
   * ```
   */
  ImportKeyword = "ImportKeyword",
  /**
   * Represents a node with kind `InKeyword`, having the following structure:
   *
   * ```ebnf
   * IN_KEYWORD = "in";
   * ```
   */
  InKeyword = "InKeyword",
  /**
   * Represents a node with kind `IndexedKeyword`, having the following structure:
   *
   * ```ebnf
   * INDEXED_KEYWORD = "indexed";
   * ```
   */
  IndexedKeyword = "IndexedKeyword",
  /**
   * Represents a node with kind `InlineKeyword`, having the following structure:
   *
   * ```ebnf
   * INLINE_KEYWORD = "inline";
   * ```
   */
  InlineKeyword = "InlineKeyword",
  /**
   * Represents a node with kind `IntKeyword`, having the following structure:
   *
   * ```ebnf
   * INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
   * ```
   */
  IntKeyword = "IntKeyword",
  /**
   * Represents a node with kind `InterfaceKeyword`, having the following structure:
   *
   * ```ebnf
   * INTERFACE_KEYWORD = "interface";
   * ```
   */
  InterfaceKeyword = "InterfaceKeyword",
  /**
   * Represents a node with kind `InternalKeyword`, having the following structure:
   *
   * ```ebnf
   * INTERNAL_KEYWORD = "internal";
   * ```
   */
  InternalKeyword = "InternalKeyword",
  /**
   * Represents a node with kind `IsKeyword`, having the following structure:
   *
   * ```ebnf
   * IS_KEYWORD = "is";
   * ```
   */
  IsKeyword = "IsKeyword",
  /**
   * Represents a node with kind `LayoutKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.29 *)
   * (* Never reserved *)
   * LAYOUT_KEYWORD = "layout";
   * ```
   */
  LayoutKeyword = "LayoutKeyword",
  /**
   * Represents a node with kind `LessThan`, having the following structure:
   *
   * ```ebnf
   * LESS_THAN = "<";
   * ```
   */
  LessThan = "LessThan",
  /**
   * Represents a node with kind `LessThanEqual`, having the following structure:
   *
   * ```ebnf
   * LESS_THAN_EQUAL = "<=";
   * ```
   */
  LessThanEqual = "LessThanEqual",
  /**
   * Represents a node with kind `LessThanLessThan`, having the following structure:
   *
   * ```ebnf
   * LESS_THAN_LESS_THAN = "<<";
   * ```
   */
  LessThanLessThan = "LessThanLessThan",
  /**
   * Represents a node with kind `LessThanLessThanEqual`, having the following structure:
   *
   * ```ebnf
   * LESS_THAN_LESS_THAN_EQUAL = "<<=";
   * ```
   */
  LessThanLessThanEqual = "LessThanLessThanEqual",
  /**
   * Represents a node with kind `LetKeyword`, having the following structure:
   *
   * ```ebnf
   * LET_KEYWORD = "let";
   * ```
   */
  LetKeyword = "LetKeyword",
  /**
   * Represents a node with kind `LibraryKeyword`, having the following structure:
   *
   * ```ebnf
   * LIBRARY_KEYWORD = "library";
   * ```
   */
  LibraryKeyword = "LibraryKeyword",
  /**
   * Represents a node with kind `MacroKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * MACRO_KEYWORD = "macro";
   * ```
   */
  MacroKeyword = "MacroKeyword",
  /**
   * Represents a node with kind `MappingKeyword`, having the following structure:
   *
   * ```ebnf
   * MAPPING_KEYWORD = "mapping";
   * ```
   */
  MappingKeyword = "MappingKeyword",
  /**
   * Represents a node with kind `MatchKeyword`, having the following structure:
   *
   * ```ebnf
   * MATCH_KEYWORD = "match";
   * ```
   */
  MatchKeyword = "MatchKeyword",
  /**
   * Represents a node with kind `MemoryKeyword`, having the following structure:
   *
   * ```ebnf
   * MEMORY_KEYWORD = "memory";
   * ```
   */
  MemoryKeyword = "MemoryKeyword",
  /**
   * Represents a node with kind `Minus`, having the following structure:
   *
   * ```ebnf
   * MINUS = "-";
   * ```
   */
  Minus = "Minus",
  /**
   * Represents a node with kind `MinusEqual`, having the following structure:
   *
   * ```ebnf
   * MINUS_EQUAL = "-=";
   * ```
   */
  MinusEqual = "MinusEqual",
  /**
   * Represents a node with kind `MinusGreaterThan`, having the following structure:
   *
   * ```ebnf
   * MINUS_GREATER_THAN = "->";
   * ```
   */
  MinusGreaterThan = "MinusGreaterThan",
  /**
   * Represents a node with kind `MinusMinus`, having the following structure:
   *
   * ```ebnf
   * MINUS_MINUS = "--";
   * ```
   */
  MinusMinus = "MinusMinus",
  /**
   * Represents a node with kind `MinutesKeyword`, having the following structure:
   *
   * ```ebnf
   * MINUTES_KEYWORD = "minutes";
   * ```
   */
  MinutesKeyword = "MinutesKeyword",
  /**
   * Represents a node with kind `ModifierKeyword`, having the following structure:
   *
   * ```ebnf
   * MODIFIER_KEYWORD = "modifier";
   * ```
   */
  ModifierKeyword = "ModifierKeyword",
  /**
   * Represents a node with kind `MultiLineComment`, having the following structure:
   *
   * ```ebnf
   * MULTI_LINE_COMMENT = "/*" (?!"*" !"/") (!"*" | ("*" (?!"/")))* "*\/";
   * ```
   */
  MultiLineComment = "MultiLineComment",
  /**
   * Represents a node with kind `MultiLineNatSpecComment`, having the following structure:
   *
   * ```ebnf
   * MULTI_LINE_NAT_SPEC_COMMENT = "/**" (?!"/") (!"*" | ("*" (?!"/")))* "*\/";
   * ```
   */
  MultiLineNatSpecComment = "MultiLineNatSpecComment",
  /**
   * Represents a node with kind `MutableKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * MUTABLE_KEYWORD = "mutable";
   * ```
   */
  MutableKeyword = "MutableKeyword",
  /**
   * Represents a node with kind `NewKeyword`, having the following structure:
   *
   * ```ebnf
   * NEW_KEYWORD = "new";
   * ```
   */
  NewKeyword = "NewKeyword",
  /**
   * Represents a node with kind `NullKeyword`, having the following structure:
   *
   * ```ebnf
   * NULL_KEYWORD = "null";
   * ```
   */
  NullKeyword = "NullKeyword",
  /**
   * Represents a node with kind `OfKeyword`, having the following structure:
   *
   * ```ebnf
   * OF_KEYWORD = "of";
   * ```
   */
  OfKeyword = "OfKeyword",
  /**
   * Represents a node with kind `OpenBrace`, having the following structure:
   *
   * ```ebnf
   * OPEN_BRACE = "{";
   * ```
   */
  OpenBrace = "OpenBrace",
  /**
   * Represents a node with kind `OpenBracket`, having the following structure:
   *
   * ```ebnf
   * OPEN_BRACKET = "[";
   * ```
   */
  OpenBracket = "OpenBracket",
  /**
   * Represents a node with kind `OpenParen`, having the following structure:
   *
   * ```ebnf
   * OPEN_PAREN = "(";
   * ```
   */
  OpenParen = "OpenParen",
  /**
   * Represents a node with kind `OverrideKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * (* Reserved in 0.5.0 *)
   * OVERRIDE_KEYWORD = "override";
   * ```
   */
  OverrideKeyword = "OverrideKeyword",
  /**
   * Represents a node with kind `PartialKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * PARTIAL_KEYWORD = "partial";
   * ```
   */
  PartialKeyword = "PartialKeyword",
  /**
   * Represents a node with kind `PayableKeyword`, having the following structure:
   *
   * ```ebnf
   * PAYABLE_KEYWORD = "payable";
   * ```
   */
  PayableKeyword = "PayableKeyword",
  /**
   * Represents a node with kind `Percent`, having the following structure:
   *
   * ```ebnf
   * PERCENT = "%";
   * ```
   */
  Percent = "Percent",
  /**
   * Represents a node with kind `PercentEqual`, having the following structure:
   *
   * ```ebnf
   * PERCENT_EQUAL = "%=";
   * ```
   */
  PercentEqual = "PercentEqual",
  /**
   * Represents a node with kind `Period`, having the following structure:
   *
   * ```ebnf
   * PERIOD = ".";
   * ```
   */
  Period = "Period",
  /**
   * Represents a node with kind `Plus`, having the following structure:
   *
   * ```ebnf
   * PLUS = "+";
   * ```
   */
  Plus = "Plus",
  /**
   * Represents a node with kind `PlusEqual`, having the following structure:
   *
   * ```ebnf
   * PLUS_EQUAL = "+=";
   * ```
   */
  PlusEqual = "PlusEqual",
  /**
   * Represents a node with kind `PlusPlus`, having the following structure:
   *
   * ```ebnf
   * PLUS_PLUS = "++";
   * ```
   */
  PlusPlus = "PlusPlus",
  /**
   * Represents a node with kind `PragmaKeyword`, having the following structure:
   *
   * ```ebnf
   * PRAGMA_KEYWORD = "pragma";
   * ```
   */
  PragmaKeyword = "PragmaKeyword",
  /**
   * Represents a node with kind `PrivateKeyword`, having the following structure:
   *
   * ```ebnf
   * PRIVATE_KEYWORD = "private";
   * ```
   */
  PrivateKeyword = "PrivateKeyword",
  /**
   * Represents a node with kind `PromiseKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * PROMISE_KEYWORD = "promise";
   * ```
   */
  PromiseKeyword = "PromiseKeyword",
  /**
   * Represents a node with kind `PublicKeyword`, having the following structure:
   *
   * ```ebnf
   * PUBLIC_KEYWORD = "public";
   * ```
   */
  PublicKeyword = "PublicKeyword",
  /**
   * Represents a node with kind `PureKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.16 *)
   * PURE_KEYWORD = "pure";
   * ```
   */
  PureKeyword = "PureKeyword",
  /**
   * Represents a node with kind `QuestionMark`, having the following structure:
   *
   * ```ebnf
   * QUESTION_MARK = "?";
   * ```
   */
  QuestionMark = "QuestionMark",
  /**
   * Represents a node with kind `ReceiveKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.6.0 *)
   * RECEIVE_KEYWORD = "receive";
   * ```
   */
  ReceiveKeyword = "ReceiveKeyword",
  /**
   * Represents a node with kind `ReferenceKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * REFERENCE_KEYWORD = "reference";
   * ```
   */
  ReferenceKeyword = "ReferenceKeyword",
  /**
   * Represents a node with kind `RelocatableKeyword`, having the following structure:
   *
   * ```ebnf
   * RELOCATABLE_KEYWORD = "relocatable";
   * ```
   */
  RelocatableKeyword = "RelocatableKeyword",
  /**
   * Represents a node with kind `ReturnKeyword`, having the following structure:
   *
   * ```ebnf
   * RETURN_KEYWORD = "return";
   * ```
   */
  ReturnKeyword = "ReturnKeyword",
  /**
   * Represents a node with kind `ReturnsKeyword`, having the following structure:
   *
   * ```ebnf
   * RETURNS_KEYWORD = "returns";
   * ```
   */
  ReturnsKeyword = "ReturnsKeyword",
  /**
   * Represents a node with kind `RevertKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * (* Never reserved *)
   * REVERT_KEYWORD = "revert";
   * ```
   */
  RevertKeyword = "RevertKeyword",
  /**
   * Represents a node with kind `SMTCheckerKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.16 *)
   * (* Never reserved *)
   * SMT_CHECKER_KEYWORD = "SMTChecker";
   * ```
   */
  SMTCheckerKeyword = "SMTCheckerKeyword",
  /**
   * Represents a node with kind `SealedKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * SEALED_KEYWORD = "sealed";
   * ```
   */
  SealedKeyword = "SealedKeyword",
  /**
   * Represents a node with kind `SecondsKeyword`, having the following structure:
   *
   * ```ebnf
   * SECONDS_KEYWORD = "seconds";
   * ```
   */
  SecondsKeyword = "SecondsKeyword",
  /**
   * Represents a node with kind `Semicolon`, having the following structure:
   *
   * ```ebnf
   * SEMICOLON = ";";
   * ```
   */
  Semicolon = "Semicolon",
  /**
   * Represents a node with kind `SingleLineComment`, having the following structure:
   *
   * ```ebnf
   * SINGLE_LINE_COMMENT = "//" (?!"/") (!("\r" | "\n"))*;
   * ```
   */
  SingleLineComment = "SingleLineComment",
  /**
   * Represents a node with kind `SingleLineNatSpecComment`, having the following structure:
   *
   * ```ebnf
   * SINGLE_LINE_NAT_SPEC_COMMENT = "///" (!("\r" | "\n"))*;
   * ```
   */
  SingleLineNatSpecComment = "SingleLineNatSpecComment",
  /**
   * Represents a node with kind `SingleQuotedHexStringLiteral`, having the following structure:
   *
   * ```ebnf
   * SINGLE_QUOTED_HEX_STRING_LITERAL = "hex'" «HEX_STRING_CONTENTS»? "'";
   * ```
   */
  SingleQuotedHexStringLiteral = "SingleQuotedHexStringLiteral",
  /**
   * Represents a node with kind `SingleQuotedStringLiteral`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.4.25 *)
   * SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE_ARBITRARY» | !("'" | "\\" | "\r" | "\n"))* "'";
   *
   * (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
   * SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | !("'" | "\\" | "\r" | "\n"))* "'";
   *
   * SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | " "…"&" | "("…"[" | "]"…"~")* "'";
   * ```
   */
  SingleQuotedStringLiteral = "SingleQuotedStringLiteral",
  /**
   * Represents a node with kind `SingleQuotedUnicodeStringLiteral`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.0 *)
   * SINGLE_QUOTED_UNICODE_STRING_LITERAL = "unicode'" («ESCAPE_SEQUENCE» | !("'" | "\\" | "\r" | "\n"))* "'";
   * ```
   */
  SingleQuotedUnicodeStringLiteral = "SingleQuotedUnicodeStringLiteral",
  /**
   * Represents a node with kind `SingleQuotedVersionLiteral`, having the following structure:
   *
   * ```ebnf
   * SINGLE_QUOTED_VERSION_LITERAL = "'" «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* "'";
   * ```
   */
  SingleQuotedVersionLiteral = "SingleQuotedVersionLiteral",
  /**
   * Represents a node with kind `SizeOfKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * SIZE_OF_KEYWORD = "sizeof";
   * ```
   */
  SizeOfKeyword = "SizeOfKeyword",
  /**
   * Represents a node with kind `Slash`, having the following structure:
   *
   * ```ebnf
   * SLASH = "/" (?!"*" | "/" | "=");
   * ```
   */
  Slash = "Slash",
  /**
   * Represents a node with kind `SlashEqual`, having the following structure:
   *
   * ```ebnf
   * SLASH_EQUAL = "/=";
   * ```
   */
  SlashEqual = "SlashEqual",
  /**
   * Represents a node with kind `SolidityKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Never reserved *)
   * SOLIDITY_KEYWORD = "solidity";
   * ```
   */
  SolidityKeyword = "SolidityKeyword",
  /**
   * Represents a node with kind `StaticKeyword`, having the following structure:
   *
   * ```ebnf
   * STATIC_KEYWORD = "static";
   * ```
   */
  StaticKeyword = "StaticKeyword",
  /**
   * Represents a node with kind `StorageKeyword`, having the following structure:
   *
   * ```ebnf
   * STORAGE_KEYWORD = "storage";
   * ```
   */
  StorageKeyword = "StorageKeyword",
  /**
   * Represents a node with kind `StringKeyword`, having the following structure:
   *
   * ```ebnf
   * STRING_KEYWORD = "string";
   * ```
   */
  StringKeyword = "StringKeyword",
  /**
   * Represents a node with kind `StructKeyword`, having the following structure:
   *
   * ```ebnf
   * STRUCT_KEYWORD = "struct";
   * ```
   */
  StructKeyword = "StructKeyword",
  /**
   * Represents a node with kind `SuperKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.8.0 *)
   * SUPER_KEYWORD = "super";
   * ```
   */
  SuperKeyword = "SuperKeyword",
  /**
   * Represents a node with kind `SupportsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * SUPPORTS_KEYWORD = "supports";
   * ```
   */
  SupportsKeyword = "SupportsKeyword",
  /**
   * Represents a node with kind `SwitchKeyword`, having the following structure:
   *
   * ```ebnf
   * SWITCH_KEYWORD = "switch";
   * ```
   */
  SwitchKeyword = "SwitchKeyword",
  /**
   * Represents a node with kind `SzaboKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.7.0 *)
   * (* Reserved until 0.7.0 *)
   * SZABO_KEYWORD = "szabo";
   * ```
   */
  SzaboKeyword = "SzaboKeyword",
  /**
   * Represents a node with kind `ThisKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.8.0 *)
   * THIS_KEYWORD = "this";
   * ```
   */
  ThisKeyword = "ThisKeyword",
  /**
   * Represents a node with kind `ThrowKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * THROW_KEYWORD = "throw";
   * ```
   */
  ThrowKeyword = "ThrowKeyword",
  /**
   * Represents a node with kind `Tilde`, having the following structure:
   *
   * ```ebnf
   * TILDE = "~";
   * ```
   */
  Tilde = "Tilde",
  /**
   * Represents a node with kind `TransientKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.27 *)
   * (* Never reserved *)
   * TRANSIENT_KEYWORD = "transient";
   * ```
   */
  TransientKeyword = "TransientKeyword",
  /**
   * Represents a node with kind `TrueKeyword`, having the following structure:
   *
   * ```ebnf
   * TRUE_KEYWORD = "true";
   * ```
   */
  TrueKeyword = "TrueKeyword",
  /**
   * Represents a node with kind `TryKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * TRY_KEYWORD = "try";
   * ```
   */
  TryKeyword = "TryKeyword",
  /**
   * Represents a node with kind `TypeDefKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * TYPE_DEF_KEYWORD = "typedef";
   * ```
   */
  TypeDefKeyword = "TypeDefKeyword",
  /**
   * Represents a node with kind `TypeKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.3 *)
   * TYPE_KEYWORD = "type";
   * ```
   */
  TypeKeyword = "TypeKeyword",
  /**
   * Represents a node with kind `TypeOfKeyword`, having the following structure:
   *
   * ```ebnf
   * TYPE_OF_KEYWORD = "typeof";
   * ```
   */
  TypeOfKeyword = "TypeOfKeyword",
  /**
   * Represents a node with kind `UfixedKeyword`, having the following structure:
   *
   * ```ebnf
   * UFIXED_KEYWORD = "ufixed";
   *
   * UFIXED_KEYWORD = "ufixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176") "x" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80");
   *
   * UFIXED_KEYWORD = "ufixed" ("184x8" | "184x16" | "184x24" | "184x32" | "184x40" | "184x48" | "184x56" | "184x64" | "184x72" | "192x8" | "192x16" | "192x24" | "192x32" | "192x40" | "192x48" | "192x56" | "192x64" | "200x8" | "200x16" | "200x24" | "200x32" | "200x40" | "200x48" | "200x56" | "208x8" | "208x16" | "208x24" | "208x32" | "208x40" | "208x48" | "216x8" | "216x16" | "216x24" | "216x32" | "216x40" | "224x8" | "224x16" | "224x24" | "224x32" | "232x8" | "232x16" | "232x24" | "240x8" | "240x16" | "248x8");
   *
   * (* Reserved in 0.4.14 *)
   * UFIXED_KEYWORD = "ufixed" ("184x80" | "192x72" | "192x80" | "200x64" | "200x72" | "200x80" | "208x56" | "208x64" | "208x72" | "208x80" | "216x48" | "216x56" | "216x64" | "216x72" | "216x80" | "224x40" | "224x48" | "224x56" | "224x64" | "224x72" | "224x80" | "232x32" | "232x40" | "232x48" | "232x56" | "232x64" | "232x72" | "232x80" | "240x24" | "240x32" | "240x40" | "240x48" | "240x56" | "240x64" | "240x72" | "240x80" | "248x16" | "248x24" | "248x32" | "248x40" | "248x48" | "248x56" | "248x64" | "248x72" | "248x80" | "256x8" | "256x16" | "256x24" | "256x32" | "256x40" | "256x48" | "256x56" | "256x64" | "256x72" | "256x80");
   *
   * (* Reserved in 0.4.14 *)
   * UFIXED_KEYWORD = "ufixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256") "x" ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "33" | "34" | "35" | "36" | "37" | "38" | "39" | "41" | "42" | "43" | "44" | "45" | "46" | "47" | "49" | "50" | "51" | "52" | "53" | "54" | "55" | "57" | "58" | "59" | "60" | "61" | "62" | "63" | "65" | "66" | "67" | "68" | "69" | "70" | "71" | "73" | "74" | "75" | "76" | "77" | "78" | "79");
   * ```
   */
  UfixedKeyword = "UfixedKeyword",
  /**
   * Represents a node with kind `UintKeyword`, having the following structure:
   *
   * ```ebnf
   * UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
   * ```
   */
  UintKeyword = "UintKeyword",
  /**
   * Represents a node with kind `UncheckedKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.0 *)
   * (* Reserved in 0.5.0 *)
   * UNCHECKED_KEYWORD = "unchecked";
   * ```
   */
  UncheckedKeyword = "UncheckedKeyword",
  /**
   * Represents a node with kind `UsingKeyword`, having the following structure:
   *
   * ```ebnf
   * USING_KEYWORD = "using";
   * ```
   */
  UsingKeyword = "UsingKeyword",
  /**
   * Represents a node with kind `VarKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * VAR_KEYWORD = "var";
   * ```
   */
  VarKeyword = "VarKeyword",
  /**
   * Represents a node with kind `VersionSpecifier`, having the following structure:
   *
   * ```ebnf
   * VERSION_SPECIFIER = «VERSION_SPECIFIER_FRAGMENT»;
   * ```
   */
  VersionSpecifier = "VersionSpecifier",
  /**
   * Represents a node with kind `ViewKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.16 *)
   * VIEW_KEYWORD = "view";
   * ```
   */
  ViewKeyword = "ViewKeyword",
  /**
   * Represents a node with kind `VirtualKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * (* Reserved in 0.6.0 *)
   * VIRTUAL_KEYWORD = "virtual";
   * ```
   */
  VirtualKeyword = "VirtualKeyword",
  /**
   * Represents a node with kind `WeeksKeyword`, having the following structure:
   *
   * ```ebnf
   * WEEKS_KEYWORD = "weeks";
   * ```
   */
  WeeksKeyword = "WeeksKeyword",
  /**
   * Represents a node with kind `WeiKeyword`, having the following structure:
   *
   * ```ebnf
   * WEI_KEYWORD = "wei";
   * ```
   */
  WeiKeyword = "WeiKeyword",
  /**
   * Represents a node with kind `WhileKeyword`, having the following structure:
   *
   * ```ebnf
   * WHILE_KEYWORD = "while";
   * ```
   */
  WhileKeyword = "WhileKeyword",
  /**
   * Represents a node with kind `Whitespace`, having the following structure:
   *
   * ```ebnf
   * WHITESPACE = (" " | "\t")+;
   * ```
   */
  Whitespace = "Whitespace",
  /**
   * Represents a node with kind `YearsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YEARS_KEYWORD = "years";
   * ```
   */
  YearsKeyword = "YearsKeyword",
  /**
   * Represents a node with kind `YulAbstractKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ABSTRACT_KEYWORD = "abstract";
   * ```
   */
  YulAbstractKeyword = "YulAbstractKeyword",
  /**
   * Represents a node with kind `YulAfterKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_AFTER_KEYWORD = "after";
   * ```
   */
  YulAfterKeyword = "YulAfterKeyword",
  /**
   * Represents a node with kind `YulAliasKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_ALIAS_KEYWORD = "alias";
   * ```
   */
  YulAliasKeyword = "YulAliasKeyword",
  /**
   * Represents a node with kind `YulAnonymousKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ANONYMOUS_KEYWORD = "anonymous";
   * ```
   */
  YulAnonymousKeyword = "YulAnonymousKeyword",
  /**
   * Represents a node with kind `YulApplyKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_APPLY_KEYWORD = "apply";
   * ```
   */
  YulApplyKeyword = "YulApplyKeyword",
  /**
   * Represents a node with kind `YulAsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_AS_KEYWORD = "as";
   * ```
   */
  YulAsKeyword = "YulAsKeyword",
  /**
   * Represents a node with kind `YulAssemblyKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ASSEMBLY_KEYWORD = "assembly";
   * ```
   */
  YulAssemblyKeyword = "YulAssemblyKeyword",
  /**
   * Represents a node with kind `YulAutoKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_AUTO_KEYWORD = "auto";
   * ```
   */
  YulAutoKeyword = "YulAutoKeyword",
  /**
   * Represents a node with kind `YulBoolKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.5.10 *)
   * YUL_BOOL_KEYWORD = "bool";
   * ```
   */
  YulBoolKeyword = "YulBoolKeyword",
  /**
   * Represents a node with kind `YulBreakKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_BREAK_KEYWORD = "break";
   * ```
   */
  YulBreakKeyword = "YulBreakKeyword",
  /**
   * Represents a node with kind `YulBytesKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
   * ```
   */
  YulBytesKeyword = "YulBytesKeyword",
  /**
   * Represents a node with kind `YulCallDataKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_CALL_DATA_KEYWORD = "calldata";
   * ```
   */
  YulCallDataKeyword = "YulCallDataKeyword",
  /**
   * Represents a node with kind `YulCaseKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_CASE_KEYWORD = "case";
   * ```
   */
  YulCaseKeyword = "YulCaseKeyword",
  /**
   * Represents a node with kind `YulCatchKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_CATCH_KEYWORD = "catch";
   * ```
   */
  YulCatchKeyword = "YulCatchKeyword",
  /**
   * Represents a node with kind `YulConstantKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_CONSTANT_KEYWORD = "constant";
   * ```
   */
  YulConstantKeyword = "YulConstantKeyword",
  /**
   * Represents a node with kind `YulConstructorKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_CONSTRUCTOR_KEYWORD = "constructor";
   * ```
   */
  YulConstructorKeyword = "YulConstructorKeyword",
  /**
   * Represents a node with kind `YulContinueKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_CONTINUE_KEYWORD = "continue";
   * ```
   */
  YulContinueKeyword = "YulContinueKeyword",
  /**
   * Represents a node with kind `YulContractKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_CONTRACT_KEYWORD = "contract";
   * ```
   */
  YulContractKeyword = "YulContractKeyword",
  /**
   * Represents a node with kind `YulCopyOfKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_COPY_OF_KEYWORD = "copyof";
   * ```
   */
  YulCopyOfKeyword = "YulCopyOfKeyword",
  /**
   * Represents a node with kind `YulDaysKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_DAYS_KEYWORD = "days";
   * ```
   */
  YulDaysKeyword = "YulDaysKeyword",
  /**
   * Represents a node with kind `YulDecimalLiteral`, having the following structure:
   *
   * ```ebnf
   * YUL_DECIMAL_LITERAL = ("0" | ("1"…"9" "0"…"9"*)) (?!«IDENTIFIER_START»);
   * ```
   */
  YulDecimalLiteral = "YulDecimalLiteral",
  /**
   * Represents a node with kind `YulDefaultKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_DEFAULT_KEYWORD = "default";
   * ```
   */
  YulDefaultKeyword = "YulDefaultKeyword",
  /**
   * Represents a node with kind `YulDefineKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_DEFINE_KEYWORD = "define";
   * ```
   */
  YulDefineKeyword = "YulDefineKeyword",
  /**
   * Represents a node with kind `YulDeleteKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_DELETE_KEYWORD = "delete";
   * ```
   */
  YulDeleteKeyword = "YulDeleteKeyword",
  /**
   * Represents a node with kind `YulDoKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_DO_KEYWORD = "do";
   * ```
   */
  YulDoKeyword = "YulDoKeyword",
  /**
   * Represents a node with kind `YulElseKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ELSE_KEYWORD = "else";
   * ```
   */
  YulElseKeyword = "YulElseKeyword",
  /**
   * Represents a node with kind `YulEmitKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_EMIT_KEYWORD = "emit";
   * ```
   */
  YulEmitKeyword = "YulEmitKeyword",
  /**
   * Represents a node with kind `YulEnumKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ENUM_KEYWORD = "enum";
   * ```
   */
  YulEnumKeyword = "YulEnumKeyword",
  /**
   * Represents a node with kind `YulEtherKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ETHER_KEYWORD = "ether";
   * ```
   */
  YulEtherKeyword = "YulEtherKeyword",
  /**
   * Represents a node with kind `YulEventKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_EVENT_KEYWORD = "event";
   * ```
   */
  YulEventKeyword = "YulEventKeyword",
  /**
   * Represents a node with kind `YulExternalKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_EXTERNAL_KEYWORD = "external";
   * ```
   */
  YulExternalKeyword = "YulExternalKeyword",
  /**
   * Represents a node with kind `YulFallbackKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.6.0 until 0.7.1 *)
   * YUL_FALLBACK_KEYWORD = "fallback";
   * ```
   */
  YulFallbackKeyword = "YulFallbackKeyword",
  /**
   * Represents a node with kind `YulFalseKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.2 *)
   * YUL_FALSE_KEYWORD = "false";
   * ```
   */
  YulFalseKeyword = "YulFalseKeyword",
  /**
   * Represents a node with kind `YulFinalKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_FINAL_KEYWORD = "final";
   * ```
   */
  YulFinalKeyword = "YulFinalKeyword",
  /**
   * Represents a node with kind `YulFinneyKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.0 *)
   * YUL_FINNEY_KEYWORD = "finney";
   * ```
   */
  YulFinneyKeyword = "YulFinneyKeyword",
  /**
   * Represents a node with kind `YulFixedKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_FIXED_KEYWORD = "fixed";
   *
   * (* Reserved until 0.7.1 *)
   * YUL_FIXED_KEYWORD = "fixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176") "x" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80");
   *
   * (* Reserved until 0.7.1 *)
   * YUL_FIXED_KEYWORD = "fixed" ("184x8" | "184x16" | "184x24" | "184x32" | "184x40" | "184x48" | "184x56" | "184x64" | "184x72" | "192x8" | "192x16" | "192x24" | "192x32" | "192x40" | "192x48" | "192x56" | "192x64" | "200x8" | "200x16" | "200x24" | "200x32" | "200x40" | "200x48" | "200x56" | "208x8" | "208x16" | "208x24" | "208x32" | "208x40" | "208x48" | "216x8" | "216x16" | "216x24" | "216x32" | "216x40" | "224x8" | "224x16" | "224x24" | "224x32" | "232x8" | "232x16" | "232x24" | "240x8" | "240x16" | "248x8");
   *
   * (* Reserved from 0.4.14 until 0.7.1 *)
   * YUL_FIXED_KEYWORD = "fixed" ("184x80" | "192x72" | "192x80" | "200x64" | "200x72" | "200x80" | "208x56" | "208x64" | "208x72" | "208x80" | "216x48" | "216x56" | "216x64" | "216x72" | "216x80" | "224x40" | "224x48" | "224x56" | "224x64" | "224x72" | "224x80" | "232x32" | "232x40" | "232x48" | "232x56" | "232x64" | "232x72" | "232x80" | "240x24" | "240x32" | "240x40" | "240x48" | "240x56" | "240x64" | "240x72" | "240x80" | "248x16" | "248x24" | "248x32" | "248x40" | "248x48" | "248x56" | "248x64" | "248x72" | "248x80" | "256x8" | "256x16" | "256x24" | "256x32" | "256x40" | "256x48" | "256x56" | "256x64" | "256x72" | "256x80");
   *
   * (* Reserved from 0.4.14 until 0.7.1 *)
   * YUL_FIXED_KEYWORD = "fixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256") "x" ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "33" | "34" | "35" | "36" | "37" | "38" | "39" | "41" | "42" | "43" | "44" | "45" | "46" | "47" | "49" | "50" | "51" | "52" | "53" | "54" | "55" | "57" | "58" | "59" | "60" | "61" | "62" | "63" | "65" | "66" | "67" | "68" | "69" | "70" | "71" | "73" | "74" | "75" | "76" | "77" | "78" | "79");
   * ```
   */
  YulFixedKeyword = "YulFixedKeyword",
  /**
   * Represents a node with kind `YulForKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_FOR_KEYWORD = "for";
   * ```
   */
  YulForKeyword = "YulForKeyword",
  /**
   * Represents a node with kind `YulFunctionKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_FUNCTION_KEYWORD = "function";
   * ```
   */
  YulFunctionKeyword = "YulFunctionKeyword",
  /**
   * Represents a node with kind `YulGweiKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.7.0 until 0.7.1 *)
   * YUL_GWEI_KEYWORD = "gwei";
   * ```
   */
  YulGweiKeyword = "YulGweiKeyword",
  /**
   * Represents a node with kind `YulHexKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_HEX_KEYWORD = "hex";
   * ```
   */
  YulHexKeyword = "YulHexKeyword",
  /**
   * Represents a node with kind `YulHexLiteral`, having the following structure:
   *
   * ```ebnf
   * YUL_HEX_LITERAL = "0x" «HEX_CHARACTER»+ (?!«IDENTIFIER_START»);
   * ```
   */
  YulHexLiteral = "YulHexLiteral",
  /**
   * Represents a node with kind `YulHoursKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_HOURS_KEYWORD = "hours";
   * ```
   */
  YulHoursKeyword = "YulHoursKeyword",
  /**
   * Represents a node with kind `YulIdentifier`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.8 and deprecated in 0.7.0. *)
   * YUL_IDENTIFIER = «IDENTIFIER_START» («IDENTIFIER_PART» | ".")*;
   *
   * YUL_IDENTIFIER = «IDENTIFIER_START» «IDENTIFIER_PART»*;
   * ```
   */
  YulIdentifier = "YulIdentifier",
  /**
   * Represents a node with kind `YulIfKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_IF_KEYWORD = "if";
   * ```
   */
  YulIfKeyword = "YulIfKeyword",
  /**
   * Represents a node with kind `YulImmutableKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_IMMUTABLE_KEYWORD = "immutable";
   * ```
   */
  YulImmutableKeyword = "YulImmutableKeyword",
  /**
   * Represents a node with kind `YulImplementsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_IMPLEMENTS_KEYWORD = "implements";
   * ```
   */
  YulImplementsKeyword = "YulImplementsKeyword",
  /**
   * Represents a node with kind `YulImportKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_IMPORT_KEYWORD = "import";
   * ```
   */
  YulImportKeyword = "YulImportKeyword",
  /**
   * Represents a node with kind `YulInKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.6.8 *)
   * YUL_IN_KEYWORD = "in";
   * ```
   */
  YulInKeyword = "YulInKeyword",
  /**
   * Represents a node with kind `YulIndexedKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INDEXED_KEYWORD = "indexed";
   * ```
   */
  YulIndexedKeyword = "YulIndexedKeyword",
  /**
   * Represents a node with kind `YulInlineKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INLINE_KEYWORD = "inline";
   * ```
   */
  YulInlineKeyword = "YulInlineKeyword",
  /**
   * Represents a node with kind `YulIntKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
   * ```
   */
  YulIntKeyword = "YulIntKeyword",
  /**
   * Represents a node with kind `YulInterfaceKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INTERFACE_KEYWORD = "interface";
   * ```
   */
  YulInterfaceKeyword = "YulInterfaceKeyword",
  /**
   * Represents a node with kind `YulInternalKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INTERNAL_KEYWORD = "internal";
   * ```
   */
  YulInternalKeyword = "YulInternalKeyword",
  /**
   * Represents a node with kind `YulIsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_IS_KEYWORD = "is";
   * ```
   */
  YulIsKeyword = "YulIsKeyword",
  /**
   * Represents a node with kind `YulLeaveKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * (* Reserved in 0.7.1 *)
   * YUL_LEAVE_KEYWORD = "leave";
   * ```
   */
  YulLeaveKeyword = "YulLeaveKeyword",
  /**
   * Represents a node with kind `YulLetKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_LET_KEYWORD = "let";
   * ```
   */
  YulLetKeyword = "YulLetKeyword",
  /**
   * Represents a node with kind `YulLibraryKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_LIBRARY_KEYWORD = "library";
   * ```
   */
  YulLibraryKeyword = "YulLibraryKeyword",
  /**
   * Represents a node with kind `YulMacroKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_MACRO_KEYWORD = "macro";
   * ```
   */
  YulMacroKeyword = "YulMacroKeyword",
  /**
   * Represents a node with kind `YulMappingKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MAPPING_KEYWORD = "mapping";
   * ```
   */
  YulMappingKeyword = "YulMappingKeyword",
  /**
   * Represents a node with kind `YulMatchKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MATCH_KEYWORD = "match";
   * ```
   */
  YulMatchKeyword = "YulMatchKeyword",
  /**
   * Represents a node with kind `YulMemoryKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MEMORY_KEYWORD = "memory";
   * ```
   */
  YulMemoryKeyword = "YulMemoryKeyword",
  /**
   * Represents a node with kind `YulMinutesKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MINUTES_KEYWORD = "minutes";
   * ```
   */
  YulMinutesKeyword = "YulMinutesKeyword",
  /**
   * Represents a node with kind `YulModifierKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MODIFIER_KEYWORD = "modifier";
   * ```
   */
  YulModifierKeyword = "YulModifierKeyword",
  /**
   * Represents a node with kind `YulMutableKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_MUTABLE_KEYWORD = "mutable";
   * ```
   */
  YulMutableKeyword = "YulMutableKeyword",
  /**
   * Represents a node with kind `YulNewKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_NEW_KEYWORD = "new";
   * ```
   */
  YulNewKeyword = "YulNewKeyword",
  /**
   * Represents a node with kind `YulNullKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_NULL_KEYWORD = "null";
   * ```
   */
  YulNullKeyword = "YulNullKeyword",
  /**
   * Represents a node with kind `YulOfKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_OF_KEYWORD = "of";
   * ```
   */
  YulOfKeyword = "YulOfKeyword",
  /**
   * Represents a node with kind `YulOverrideKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_OVERRIDE_KEYWORD = "override";
   * ```
   */
  YulOverrideKeyword = "YulOverrideKeyword",
  /**
   * Represents a node with kind `YulPartialKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_PARTIAL_KEYWORD = "partial";
   * ```
   */
  YulPartialKeyword = "YulPartialKeyword",
  /**
   * Represents a node with kind `YulPayableKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PAYABLE_KEYWORD = "payable";
   * ```
   */
  YulPayableKeyword = "YulPayableKeyword",
  /**
   * Represents a node with kind `YulPragmaKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PRAGMA_KEYWORD = "pragma";
   * ```
   */
  YulPragmaKeyword = "YulPragmaKeyword",
  /**
   * Represents a node with kind `YulPrivateKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PRIVATE_KEYWORD = "private";
   * ```
   */
  YulPrivateKeyword = "YulPrivateKeyword",
  /**
   * Represents a node with kind `YulPromiseKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_PROMISE_KEYWORD = "promise";
   * ```
   */
  YulPromiseKeyword = "YulPromiseKeyword",
  /**
   * Represents a node with kind `YulPublicKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PUBLIC_KEYWORD = "public";
   * ```
   */
  YulPublicKeyword = "YulPublicKeyword",
  /**
   * Represents a node with kind `YulPureKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PURE_KEYWORD = "pure";
   * ```
   */
  YulPureKeyword = "YulPureKeyword",
  /**
   * Represents a node with kind `YulReceiveKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.6.0 until 0.7.1 *)
   * YUL_RECEIVE_KEYWORD = "receive";
   * ```
   */
  YulReceiveKeyword = "YulReceiveKeyword",
  /**
   * Represents a node with kind `YulReferenceKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_REFERENCE_KEYWORD = "reference";
   * ```
   */
  YulReferenceKeyword = "YulReferenceKeyword",
  /**
   * Represents a node with kind `YulRelocatableKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_RELOCATABLE_KEYWORD = "relocatable";
   * ```
   */
  YulRelocatableKeyword = "YulRelocatableKeyword",
  /**
   * Represents a node with kind `YulReturnsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_RETURNS_KEYWORD = "returns";
   * ```
   */
  YulReturnsKeyword = "YulReturnsKeyword",
  /**
   * Represents a node with kind `YulSealedKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_SEALED_KEYWORD = "sealed";
   * ```
   */
  YulSealedKeyword = "YulSealedKeyword",
  /**
   * Represents a node with kind `YulSecondsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_SECONDS_KEYWORD = "seconds";
   * ```
   */
  YulSecondsKeyword = "YulSecondsKeyword",
  /**
   * Represents a node with kind `YulSizeOfKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_SIZE_OF_KEYWORD = "sizeof";
   * ```
   */
  YulSizeOfKeyword = "YulSizeOfKeyword",
  /**
   * Represents a node with kind `YulStaticKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_STATIC_KEYWORD = "static";
   * ```
   */
  YulStaticKeyword = "YulStaticKeyword",
  /**
   * Represents a node with kind `YulStorageKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_STORAGE_KEYWORD = "storage";
   * ```
   */
  YulStorageKeyword = "YulStorageKeyword",
  /**
   * Represents a node with kind `YulStringKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_STRING_KEYWORD = "string";
   * ```
   */
  YulStringKeyword = "YulStringKeyword",
  /**
   * Represents a node with kind `YulStructKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_STRUCT_KEYWORD = "struct";
   * ```
   */
  YulStructKeyword = "YulStructKeyword",
  /**
   * Represents a node with kind `YulSuperKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.8.0 *)
   * YUL_SUPER_KEYWORD = "super";
   * ```
   */
  YulSuperKeyword = "YulSuperKeyword",
  /**
   * Represents a node with kind `YulSupportsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_SUPPORTS_KEYWORD = "supports";
   * ```
   */
  YulSupportsKeyword = "YulSupportsKeyword",
  /**
   * Represents a node with kind `YulSwitchKeyword`, having the following structure:
   *
   * ```ebnf
   * YUL_SWITCH_KEYWORD = "switch";
   * ```
   */
  YulSwitchKeyword = "YulSwitchKeyword",
  /**
   * Represents a node with kind `YulSzaboKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.0 *)
   * YUL_SZABO_KEYWORD = "szabo";
   * ```
   */
  YulSzaboKeyword = "YulSzaboKeyword",
  /**
   * Represents a node with kind `YulThisKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.8.0 *)
   * YUL_THIS_KEYWORD = "this";
   * ```
   */
  YulThisKeyword = "YulThisKeyword",
  /**
   * Represents a node with kind `YulThrowKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_THROW_KEYWORD = "throw";
   * ```
   */
  YulThrowKeyword = "YulThrowKeyword",
  /**
   * Represents a node with kind `YulTrueKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.2 *)
   * YUL_TRUE_KEYWORD = "true";
   * ```
   */
  YulTrueKeyword = "YulTrueKeyword",
  /**
   * Represents a node with kind `YulTryKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_TRY_KEYWORD = "try";
   * ```
   */
  YulTryKeyword = "YulTryKeyword",
  /**
   * Represents a node with kind `YulTypeDefKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_TYPE_DEF_KEYWORD = "typedef";
   * ```
   */
  YulTypeDefKeyword = "YulTypeDefKeyword",
  /**
   * Represents a node with kind `YulTypeKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_TYPE_KEYWORD = "type";
   * ```
   */
  YulTypeKeyword = "YulTypeKeyword",
  /**
   * Represents a node with kind `YulTypeOfKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_TYPE_OF_KEYWORD = "typeof";
   * ```
   */
  YulTypeOfKeyword = "YulTypeOfKeyword",
  /**
   * Represents a node with kind `YulUfixedKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_UFIXED_KEYWORD = "ufixed";
   *
   * (* Reserved until 0.7.1 *)
   * YUL_UFIXED_KEYWORD = "ufixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176") "x" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80");
   *
   * (* Reserved until 0.7.1 *)
   * YUL_UFIXED_KEYWORD = "ufixed" ("184x8" | "184x16" | "184x24" | "184x32" | "184x40" | "184x48" | "184x56" | "184x64" | "184x72" | "192x8" | "192x16" | "192x24" | "192x32" | "192x40" | "192x48" | "192x56" | "192x64" | "200x8" | "200x16" | "200x24" | "200x32" | "200x40" | "200x48" | "200x56" | "208x8" | "208x16" | "208x24" | "208x32" | "208x40" | "208x48" | "216x8" | "216x16" | "216x24" | "216x32" | "216x40" | "224x8" | "224x16" | "224x24" | "224x32" | "232x8" | "232x16" | "232x24" | "240x8" | "240x16" | "248x8");
   *
   * (* Reserved from 0.4.14 until 0.7.1 *)
   * YUL_UFIXED_KEYWORD = "ufixed" ("184x80" | "192x72" | "192x80" | "200x64" | "200x72" | "200x80" | "208x56" | "208x64" | "208x72" | "208x80" | "216x48" | "216x56" | "216x64" | "216x72" | "216x80" | "224x40" | "224x48" | "224x56" | "224x64" | "224x72" | "224x80" | "232x32" | "232x40" | "232x48" | "232x56" | "232x64" | "232x72" | "232x80" | "240x24" | "240x32" | "240x40" | "240x48" | "240x56" | "240x64" | "240x72" | "240x80" | "248x16" | "248x24" | "248x32" | "248x40" | "248x48" | "248x56" | "248x64" | "248x72" | "248x80" | "256x8" | "256x16" | "256x24" | "256x32" | "256x40" | "256x48" | "256x56" | "256x64" | "256x72" | "256x80");
   *
   * (* Reserved from 0.4.14 until 0.7.1 *)
   * YUL_UFIXED_KEYWORD = "ufixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256") "x" ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "33" | "34" | "35" | "36" | "37" | "38" | "39" | "41" | "42" | "43" | "44" | "45" | "46" | "47" | "49" | "50" | "51" | "52" | "53" | "54" | "55" | "57" | "58" | "59" | "60" | "61" | "62" | "63" | "65" | "66" | "67" | "68" | "69" | "70" | "71" | "73" | "74" | "75" | "76" | "77" | "78" | "79");
   * ```
   */
  YulUfixedKeyword = "YulUfixedKeyword",
  /**
   * Represents a node with kind `YulUintKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
   * ```
   */
  YulUintKeyword = "YulUintKeyword",
  /**
   * Represents a node with kind `YulUncheckedKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_UNCHECKED_KEYWORD = "unchecked";
   * ```
   */
  YulUncheckedKeyword = "YulUncheckedKeyword",
  /**
   * Represents a node with kind `YulUsingKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_USING_KEYWORD = "using";
   * ```
   */
  YulUsingKeyword = "YulUsingKeyword",
  /**
   * Represents a node with kind `YulVarKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.6.5 *)
   * YUL_VAR_KEYWORD = "var";
   * ```
   */
  YulVarKeyword = "YulVarKeyword",
  /**
   * Represents a node with kind `YulViewKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_VIEW_KEYWORD = "view";
   * ```
   */
  YulViewKeyword = "YulViewKeyword",
  /**
   * Represents a node with kind `YulVirtualKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.6.0 until 0.7.1 *)
   * YUL_VIRTUAL_KEYWORD = "virtual";
   * ```
   */
  YulVirtualKeyword = "YulVirtualKeyword",
  /**
   * Represents a node with kind `YulWeeksKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_WEEKS_KEYWORD = "weeks";
   * ```
   */
  YulWeeksKeyword = "YulWeeksKeyword",
  /**
   * Represents a node with kind `YulWeiKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_WEI_KEYWORD = "wei";
   * ```
   */
  YulWeiKeyword = "YulWeiKeyword",
  /**
   * Represents a node with kind `YulWhileKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_WHILE_KEYWORD = "while";
   * ```
   */
  YulWhileKeyword = "YulWhileKeyword",
  /**
   * Represents a node with kind `YulYearsKeyword`, having the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_YEARS_KEYWORD = "years";
   * ```
   */
  YulYearsKeyword = "YulYearsKeyword",
}
/**
 * Represents the different types of relationships between nodes in the syntax tree.
 */
export declare enum EdgeLabel {
  /**
   * Represents a child node with the label `root`.
   */
  Root = "Root",
  /**
   * Represents a child node with the label `unrecognized`.
   */
  Unrecognized = "Unrecognized",
  /**
   * Represents a child node with the label `missing`.
   */
  Missing = "Missing",
  /**
   * Represents a child node with the label `item`.
   */
  Item = "Item",
  /**
   * Represents a child node with the label `variant`.
   */
  Variant = "Variant",
  /**
   * Represents a child node with the label `separator`.
   */
  Separator = "Separator",
  /**
   * Represents a child node with the label `operand`.
   */
  Operand = "Operand",
  /**
   * Represents a child node with the label `left_operand`.
   */
  LeftOperand = "LeftOperand",
  /**
   * Represents a child node with the label `right_operand`.
   */
  RightOperand = "RightOperand",
  /**
   * Represents a child node with the label `leading_trivia`.
   */
  LeadingTrivia = "LeadingTrivia",
  /**
   * Represents a child node with the label `trailing_trivia`.
   */
  TrailingTrivia = "TrailingTrivia",
  /**
   * Represents a child node with the label `abicoder_keyword`.
   */
  AbicoderKeyword = "AbicoderKeyword",
  /**
   * Represents a child node with the label `abstract_keyword`.
   */
  AbstractKeyword = "AbstractKeyword",
  /**
   * Represents a child node with the label `address_keyword`.
   */
  AddressKeyword = "AddressKeyword",
  /**
   * Represents a child node with the label `alias`.
   */
  Alias = "Alias",
  /**
   * Represents a child node with the label `anonymous_keyword`.
   */
  AnonymousKeyword = "AnonymousKeyword",
  /**
   * Represents a child node with the label `arguments`.
   */
  Arguments = "Arguments",
  /**
   * Represents a child node with the label `as_keyword`.
   */
  AsKeyword = "AsKeyword",
  /**
   * Represents a child node with the label `assembly_keyword`.
   */
  AssemblyKeyword = "AssemblyKeyword",
  /**
   * Represents a child node with the label `assignment`.
   */
  Assignment = "Assignment",
  /**
   * Represents a child node with the label `asterisk`.
   */
  Asterisk = "Asterisk",
  /**
   * Represents a child node with the label `at_keyword`.
   */
  AtKeyword = "AtKeyword",
  /**
   * Represents a child node with the label `attributes`.
   */
  Attributes = "Attributes",
  /**
   * Represents a child node with the label `block`.
   */
  Block = "Block",
  /**
   * Represents a child node with the label `body`.
   */
  Body = "Body",
  /**
   * Represents a child node with the label `break_keyword`.
   */
  BreakKeyword = "BreakKeyword",
  /**
   * Represents a child node with the label `case_keyword`.
   */
  CaseKeyword = "CaseKeyword",
  /**
   * Represents a child node with the label `cases`.
   */
  Cases = "Cases",
  /**
   * Represents a child node with the label `catch_clauses`.
   */
  CatchClauses = "CatchClauses",
  /**
   * Represents a child node with the label `catch_keyword`.
   */
  CatchKeyword = "CatchKeyword",
  /**
   * Represents a child node with the label `clause`.
   */
  Clause = "Clause",
  /**
   * Represents a child node with the label `close_brace`.
   */
  CloseBrace = "CloseBrace",
  /**
   * Represents a child node with the label `close_bracket`.
   */
  CloseBracket = "CloseBracket",
  /**
   * Represents a child node with the label `close_paren`.
   */
  CloseParen = "CloseParen",
  /**
   * Represents a child node with the label `colon`.
   */
  Colon = "Colon",
  /**
   * Represents a child node with the label `condition`.
   */
  Condition = "Condition",
  /**
   * Represents a child node with the label `constant_keyword`.
   */
  ConstantKeyword = "ConstantKeyword",
  /**
   * Represents a child node with the label `constructor_keyword`.
   */
  ConstructorKeyword = "ConstructorKeyword",
  /**
   * Represents a child node with the label `continue_keyword`.
   */
  ContinueKeyword = "ContinueKeyword",
  /**
   * Represents a child node with the label `contract_keyword`.
   */
  ContractKeyword = "ContractKeyword",
  /**
   * Represents a child node with the label `default_keyword`.
   */
  DefaultKeyword = "DefaultKeyword",
  /**
   * Represents a child node with the label `do_keyword`.
   */
  DoKeyword = "DoKeyword",
  /**
   * Represents a child node with the label `elements`.
   */
  Elements = "Elements",
  /**
   * Represents a child node with the label `else_branch`.
   */
  ElseBranch = "ElseBranch",
  /**
   * Represents a child node with the label `else_keyword`.
   */
  ElseKeyword = "ElseKeyword",
  /**
   * Represents a child node with the label `emit_keyword`.
   */
  EmitKeyword = "EmitKeyword",
  /**
   * Represents a child node with the label `end`.
   */
  End = "End",
  /**
   * Represents a child node with the label `enum_keyword`.
   */
  EnumKeyword = "EnumKeyword",
  /**
   * Represents a child node with the label `equal`.
   */
  Equal = "Equal",
  /**
   * Represents a child node with the label `equal_greater_than`.
   */
  EqualGreaterThan = "EqualGreaterThan",
  /**
   * Represents a child node with the label `error`.
   */
  Error = "Error",
  /**
   * Represents a child node with the label `error_keyword`.
   */
  ErrorKeyword = "ErrorKeyword",
  /**
   * Represents a child node with the label `event`.
   */
  Event = "Event",
  /**
   * Represents a child node with the label `event_keyword`.
   */
  EventKeyword = "EventKeyword",
  /**
   * Represents a child node with the label `experimental_keyword`.
   */
  ExperimentalKeyword = "ExperimentalKeyword",
  /**
   * Represents a child node with the label `expression`.
   */
  Expression = "Expression",
  /**
   * Represents a child node with the label `fallback_keyword`.
   */
  FallbackKeyword = "FallbackKeyword",
  /**
   * Represents a child node with the label `false_expression`.
   */
  FalseExpression = "FalseExpression",
  /**
   * Represents a child node with the label `feature`.
   */
  Feature = "Feature",
  /**
   * Represents a child node with the label `flags`.
   */
  Flags = "Flags",
  /**
   * Represents a child node with the label `for_keyword`.
   */
  ForKeyword = "ForKeyword",
  /**
   * Represents a child node with the label `from_keyword`.
   */
  FromKeyword = "FromKeyword",
  /**
   * Represents a child node with the label `function_keyword`.
   */
  FunctionKeyword = "FunctionKeyword",
  /**
   * Represents a child node with the label `global_keyword`.
   */
  GlobalKeyword = "GlobalKeyword",
  /**
   * Represents a child node with the label `identifier`.
   */
  Identifier = "Identifier",
  /**
   * Represents a child node with the label `if_keyword`.
   */
  IfKeyword = "IfKeyword",
  /**
   * Represents a child node with the label `import_keyword`.
   */
  ImportKeyword = "ImportKeyword",
  /**
   * Represents a child node with the label `index`.
   */
  Index = "Index",
  /**
   * Represents a child node with the label `indexed_keyword`.
   */
  IndexedKeyword = "IndexedKeyword",
  /**
   * Represents a child node with the label `inheritance`.
   */
  Inheritance = "Inheritance",
  /**
   * Represents a child node with the label `initialization`.
   */
  Initialization = "Initialization",
  /**
   * Represents a child node with the label `interface_keyword`.
   */
  InterfaceKeyword = "InterfaceKeyword",
  /**
   * Represents a child node with the label `is_keyword`.
   */
  IsKeyword = "IsKeyword",
  /**
   * Represents a child node with the label `items`.
   */
  Items = "Items",
  /**
   * Represents a child node with the label `iterator`.
   */
  Iterator = "Iterator",
  /**
   * Represents a child node with the label `key_type`.
   */
  KeyType = "KeyType",
  /**
   * Represents a child node with the label `label`.
   */
  Label = "Label",
  /**
   * Represents a child node with the label `layout_keyword`.
   */
  LayoutKeyword = "LayoutKeyword",
  /**
   * Represents a child node with the label `leave_keyword`.
   */
  LeaveKeyword = "LeaveKeyword",
  /**
   * Represents a child node with the label `let_keyword`.
   */
  LetKeyword = "LetKeyword",
  /**
   * Represents a child node with the label `library_keyword`.
   */
  LibraryKeyword = "LibraryKeyword",
  /**
   * Represents a child node with the label `literal`.
   */
  Literal = "Literal",
  /**
   * Represents a child node with the label `mapping_keyword`.
   */
  MappingKeyword = "MappingKeyword",
  /**
   * Represents a child node with the label `member`.
   */
  Member = "Member",
  /**
   * Represents a child node with the label `members`.
   */
  Members = "Members",
  /**
   * Represents a child node with the label `minus`.
   */
  Minus = "Minus",
  /**
   * Represents a child node with the label `minus_greater_than`.
   */
  MinusGreaterThan = "MinusGreaterThan",
  /**
   * Represents a child node with the label `modifier_keyword`.
   */
  ModifierKeyword = "ModifierKeyword",
  /**
   * Represents a child node with the label `name`.
   */
  Name = "Name",
  /**
   * Represents a child node with the label `new_keyword`.
   */
  NewKeyword = "NewKeyword",
  /**
   * Represents a child node with the label `open_brace`.
   */
  OpenBrace = "OpenBrace",
  /**
   * Represents a child node with the label `open_bracket`.
   */
  OpenBracket = "OpenBracket",
  /**
   * Represents a child node with the label `open_paren`.
   */
  OpenParen = "OpenParen",
  /**
   * Represents a child node with the label `operator`.
   */
  Operator = "Operator",
  /**
   * Represents a child node with the label `options`.
   */
  Options = "Options",
  /**
   * Represents a child node with the label `overridden`.
   */
  Overridden = "Overridden",
  /**
   * Represents a child node with the label `override_keyword`.
   */
  OverrideKeyword = "OverrideKeyword",
  /**
   * Represents a child node with the label `parameters`.
   */
  Parameters = "Parameters",
  /**
   * Represents a child node with the label `path`.
   */
  Path = "Path",
  /**
   * Represents a child node with the label `paths`.
   */
  Paths = "Paths",
  /**
   * Represents a child node with the label `payable_keyword`.
   */
  PayableKeyword = "PayableKeyword",
  /**
   * Represents a child node with the label `period`.
   */
  Period = "Period",
  /**
   * Represents a child node with the label `pragma`.
   */
  Pragma = "Pragma",
  /**
   * Represents a child node with the label `pragma_keyword`.
   */
  PragmaKeyword = "PragmaKeyword",
  /**
   * Represents a child node with the label `question_mark`.
   */
  QuestionMark = "QuestionMark",
  /**
   * Represents a child node with the label `receive_keyword`.
   */
  ReceiveKeyword = "ReceiveKeyword",
  /**
   * Represents a child node with the label `return_keyword`.
   */
  ReturnKeyword = "ReturnKeyword",
  /**
   * Represents a child node with the label `returns`.
   */
  Returns = "Returns",
  /**
   * Represents a child node with the label `returns_keyword`.
   */
  ReturnsKeyword = "ReturnsKeyword",
  /**
   * Represents a child node with the label `revert_keyword`.
   */
  RevertKeyword = "RevertKeyword",
  /**
   * Represents a child node with the label `semicolon`.
   */
  Semicolon = "Semicolon",
  /**
   * Represents a child node with the label `sets`.
   */
  Sets = "Sets",
  /**
   * Represents a child node with the label `solidity_keyword`.
   */
  SolidityKeyword = "SolidityKeyword",
  /**
   * Represents a child node with the label `specifiers`.
   */
  Specifiers = "Specifiers",
  /**
   * Represents a child node with the label `start`.
   */
  Start = "Start",
  /**
   * Represents a child node with the label `statements`.
   */
  Statements = "Statements",
  /**
   * Represents a child node with the label `storage_location`.
   */
  StorageLocation = "StorageLocation",
  /**
   * Represents a child node with the label `struct_keyword`.
   */
  StructKeyword = "StructKeyword",
  /**
   * Represents a child node with the label `switch_keyword`.
   */
  SwitchKeyword = "SwitchKeyword",
  /**
   * Represents a child node with the label `symbols`.
   */
  Symbols = "Symbols",
  /**
   * Represents a child node with the label `target`.
   */
  Target = "Target",
  /**
   * Represents a child node with the label `throw_keyword`.
   */
  ThrowKeyword = "ThrowKeyword",
  /**
   * Represents a child node with the label `true_expression`.
   */
  TrueExpression = "TrueExpression",
  /**
   * Represents a child node with the label `try_keyword`.
   */
  TryKeyword = "TryKeyword",
  /**
   * Represents a child node with the label `type_keyword`.
   */
  TypeKeyword = "TypeKeyword",
  /**
   * Represents a child node with the label `type_name`.
   */
  TypeName = "TypeName",
  /**
   * Represents a child node with the label `types`.
   */
  Types = "Types",
  /**
   * Represents a child node with the label `unchecked_keyword`.
   */
  UncheckedKeyword = "UncheckedKeyword",
  /**
   * Represents a child node with the label `unit`.
   */
  Unit = "Unit",
  /**
   * Represents a child node with the label `using_keyword`.
   */
  UsingKeyword = "UsingKeyword",
  /**
   * Represents a child node with the label `value`.
   */
  Value = "Value",
  /**
   * Represents a child node with the label `value_type`.
   */
  ValueType = "ValueType",
  /**
   * Represents a child node with the label `var_keyword`.
   */
  VarKeyword = "VarKeyword",
  /**
   * Represents a child node with the label `variable`.
   */
  Variable = "Variable",
  /**
   * Represents a child node with the label `variable_type`.
   */
  VariableType = "VariableType",
  /**
   * Represents a child node with the label `variables`.
   */
  Variables = "Variables",
  /**
   * Represents a child node with the label `version`.
   */
  Version = "Version",
  /**
   * Represents a child node with the label `while_keyword`.
   */
  WhileKeyword = "WhileKeyword",
}
/**
 * The super type of all nodes in a tree.
 */
export type Node = NonterminalNode | TerminalNode;

/**
 * Enumerates different variants of the `Node` type.
 */
export enum NodeType {
  /**
   * Represents a variant of type `NonterminalNode`.
   */
  NonterminalNode = "NonterminalNode",

  /**
   * Represents a variant of type `TerminalNode`.
   */
  TerminalNode = "TerminalNode",
}
/**
 * Represents a match found by executing queries on a cursor.
 */
export interface QueryMatch {
  /**
   * The index of the query that produced this match.
   */
  queryIndex: number;
  /**
   * A cursor to the matched root node.
   */
  root: Cursor;
  /**
   * List of captured nodes and their names from the query.
   */
  captures: { [key: string]: Array<Cursor> };
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
/**
 * Represents an error that occurred while parsing a query.
 */
export interface QueryError {
  /**
   * A human-readable message describing what went wrong.
   */
  message: string;
  /**
   * The text range where the error occurred in the query code.
   */
  textRange: TextRange;
}

/**
 * Iterator over all ancestors of the current node, starting with the immediate parent, and moving upwards, ending with the root node.
 */

export class AncestorsIterator {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * Returns an iterator over `NonterminalNode` objects. Called by the semantics of the for-of statement.
   */
  [Symbol.iterator](): Iterator<NonterminalNode>;
  /**
   * Returns the next nonterminal node in the iteration, or `undefined` if there are no more nodes.
   */
  next(): NonterminalNode | undefined;
}

/**
 * Provides navigation and traversal capabilities over the syntax tree.
 */

export class Cursor {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
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
   * Returns a label that describes the relationship between the current node and its parent.
   */
  get label(): EdgeLabel;
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
  children(): Array<Edge>;
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
  goToNthChild(childIndex: number): boolean;
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
  goToNextTerminalWithKinds(kinds: Array<TerminalKind>): boolean;
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
  goToNextNonterminalWithKinds(kinds: Array<NonterminalKind>): boolean;
  /**
   * Executes the given queries and returns an iterator over the matches.
   */
  query(queries: Array<Query>): QueryMatchIterator;
}

/**
 * Iterator over all the remaining nodes in the current tree, moving in pre-order traversal, until the tree is completed.
 */

export class CursorIterator {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * Returns an iterator over `Edge` objects. Called by the semantics of the for-of statement.
   */
  [Symbol.iterator](): Iterator<Edge>;
  /**
   * Returns the next edge in the iteration, or `undefined` if there are no more edges.
   */
  next(): Edge | undefined;
}

/**
 * Represents a connection between nodes in the syntax tree.
 */

export class Edge {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Creates a new edge connecting a terminal node `node` with the label `label`.
   */
  static createWithTerminal(label: EdgeLabel, node: TerminalNode): Edge;
  /**
   * Creates a new edge connecting a nonterminal node `node` with the label `label`.
   */
  static createWithNonterminal(label: EdgeLabel, node: NonterminalNode): Edge;
  /**
   * Describes the relationship between this node and its parent.
   */
  get label(): EdgeLabel;
  /**
   * The target node of this edge.
   */
  get node(): Node;
}

/**
 * Represents a non-terminal node in the syntax tree.
 * These nodes can have child nodes and represent language constructs.
 */

export class NonterminalNode {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * The variant of `NodeType` that corresponds to this class.
   */
  readonly type = NodeType.NonterminalNode;

  /**
   * Coerce this variant to a `NonterminalNode`, or `undefined` if this is not the correct type.
   */
  asNonterminalNode(): this;

  /**
   * Return `true` if this object is an instance of `NonterminalNode`.
   */
  isNonterminalNode(): this is NonterminalNode;

  /**
   * Coerce this variant to a `TerminalNode`, or `undefined` if this is not the correct type.
   */
  asTerminalNode(): undefined;

  /**
   * Return `true` if this object is an instance of `TerminalNode`.
   */
  isTerminalNode(): false;

  /**
   * Creates a new nonterminal node with the specified `kind` and `children`.
   */
  static create(kind: NonterminalKind, children: Array<Edge>): NonterminalNode;
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
  children(): Array<Edge>;
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

/**
 * Represents a tree query for pattern matching in the syntax tree.
 */

export class Query {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Parses a query string into a query object.
   *
   * It will throw a `QueryError` if the query syntax is invalid.
   */
  static create(text: string): Query;
}

/**
 * Iterator over query matches in the syntax tree.
 */

export class QueryMatchIterator {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * Returns an iterator over `QueryMatch` objects. Called by the semantics of the for-of statement.
   */
  [Symbol.iterator](): Iterator<QueryMatch>;
  /**
   * Returns the next match or `undefined` if there are no more matches.
   */
  next(): QueryMatch | undefined;
}

/**
 * Useful extension methods for working with terminals and terminal kinds.
 */

export class TerminalKindExtensions {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Returns `true` if the terminal is an identifier token.
   */
  static isIdentifier(kind: TerminalKind): boolean;
  /**
   * Returns `true` if the terminal is a trivia token. i.e. whitespace, comments, etc...
   */
  static isTrivia(kind: TerminalKind): boolean;
  /**
   * Returns `true` if the terminal is a valid token in the language grammar.
   */
  static isValid(kind: TerminalKind): boolean;
}

/**
 * Represents a terminal node in the syntax tree.
 * These are leaf nodes that represent actual tokens from the source code.
 */

export class TerminalNode {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * The variant of `NodeType` that corresponds to this class.
   */
  readonly type = NodeType.TerminalNode;

  /**
   * Coerce this variant to a `TerminalNode`, or `undefined` if this is not the correct type.
   */
  asTerminalNode(): this;

  /**
   * Return `true` if this object is an instance of `TerminalNode`.
   */
  isTerminalNode(): this is TerminalNode;

  /**
   * Coerce this variant to a `NonterminalNode`, or `undefined` if this is not the correct type.
   */
  asNonterminalNode(): undefined;

  /**
   * Return `true` if this object is an instance of `NonterminalNode`.
   */
  isNonterminalNode(): false;

  /**
   * Creates a new terminal node with the specified `kind` and `text`.
   */
  static create(kind: TerminalKind, text: string): TerminalNode;
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
  children(): Array<Edge>;
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

/**
 * Useful extension methods for working with text indices.
 */

export class TextIndexExtensions {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Creates a text index with zero offsets.
   */
  static zero(): TextIndex;
}
