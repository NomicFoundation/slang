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
  /**
   * This kind represents a `AbicoderPragma` node, with the following structure:
   *
   * ```ebnf
   * AbicoderPragma = (* abicoder_keyword: *) ABICODER_KEYWORD
   * (* version: *) IDENTIFIER;
   * ```
   */
  AbicoderPragma = "AbicoderPragma",
  /**
   * This kind represents a `AdditiveExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * AdditiveExpression = (* left_operand: *) Expression
   * (* operator: *) PLUS
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AdditiveExpression = (* left_operand: *) Expression
   * (* operator: *) MINUS
   * (* right_operand: *) Expression;
   * ```
   */
  AdditiveExpression = "AdditiveExpression",
  /**
   * This kind represents a `AddressType` node, with the following structure:
   *
   * ```ebnf
   * AddressType = (* address_keyword: *) ADDRESS_KEYWORD
   * (* payable_keyword: *) PAYABLE_KEYWORD?;
   * ```
   */
  AddressType = "AddressType",
  /**
   * This kind represents a `AndExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * AndExpression = (* left_operand: *) Expression
   * (* operator: *) AMPERSAND_AMPERSAND
   * (* right_operand: *) Expression;
   * ```
   */
  AndExpression = "AndExpression",
  /**
   * This kind represents a `ArgumentsDeclaration` node, with the following structure:
   *
   * ```ebnf
   * ArgumentsDeclaration = (* variant: *) PositionalArgumentsDeclaration
   * | (* variant: *) NamedArgumentsDeclaration;
   * ```
   */
  ArgumentsDeclaration = "ArgumentsDeclaration",
  /**
   * This kind represents a `ArrayExpression` node, with the following structure:
   *
   * ```ebnf
   * ArrayExpression = (* open_bracket: *) OPEN_BRACKET
   * (* items: *) ArrayValues
   * (* close_bracket: *) CLOSE_BRACKET;
   * ```
   */
  ArrayExpression = "ArrayExpression",
  /**
   * This kind represents a `ArrayTypeName` node, with the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * ArrayTypeName = (* operand: *) TypeName
   * (* open_bracket: *) OPEN_BRACKET
   * (* index: *) Expression?
   * (* close_bracket: *) CLOSE_BRACKET;
   * ```
   */
  ArrayTypeName = "ArrayTypeName",
  /**
   * This kind represents a `ArrayValues` node, with the following structure:
   *
   * ```ebnf
   * ArrayValues = (* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*;
   * ```
   */
  ArrayValues = "ArrayValues",
  /**
   * This kind represents a `AssemblyFlags` node, with the following structure:
   *
   * ```ebnf
   * AssemblyFlags = (* item: *) StringLiteral ((* separator: *) COMMA (* item: *) StringLiteral)*;
   * ```
   */
  AssemblyFlags = "AssemblyFlags",
  /**
   * This kind represents a `AssemblyFlagsDeclaration` node, with the following structure:
   *
   * ```ebnf
   * AssemblyFlagsDeclaration = (* open_paren: *) OPEN_PAREN
   * (* flags: *) AssemblyFlags
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  AssemblyFlagsDeclaration = "AssemblyFlagsDeclaration",
  /**
   * This kind represents a `AssemblyStatement` node, with the following structure:
   *
   * ```ebnf
   * AssemblyStatement = (* assembly_keyword: *) ASSEMBLY_KEYWORD
   * (* label: *) StringLiteral?
   * (* flags: *) AssemblyFlagsDeclaration?
   * (* body: *) YulBlock;
   * ```
   */
  AssemblyStatement = "AssemblyStatement",
  /**
   * This kind represents a `AssignmentExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) BAR_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) PLUS_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) MINUS_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) CARET_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) SLASH_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) PERCENT_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) ASTERISK_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) AMPERSAND_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) LESS_THAN_LESS_THAN_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) GREATER_THAN_GREATER_THAN_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * AssignmentExpression = (* left_operand: *) Expression
   * (* operator: *) GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL
   * (* right_operand: *) Expression;
   * ```
   */
  AssignmentExpression = "AssignmentExpression",
  /**
   * This kind represents a `BitwiseAndExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * BitwiseAndExpression = (* left_operand: *) Expression
   * (* operator: *) AMPERSAND
   * (* right_operand: *) Expression;
   * ```
   */
  BitwiseAndExpression = "BitwiseAndExpression",
  /**
   * This kind represents a `BitwiseOrExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * BitwiseOrExpression = (* left_operand: *) Expression
   * (* operator: *) BAR
   * (* right_operand: *) Expression;
   * ```
   */
  BitwiseOrExpression = "BitwiseOrExpression",
  /**
   * This kind represents a `BitwiseXorExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * BitwiseXorExpression = (* left_operand: *) Expression
   * (* operator: *) CARET
   * (* right_operand: *) Expression;
   * ```
   */
  BitwiseXorExpression = "BitwiseXorExpression",
  /**
   * This kind represents a `Block` node, with the following structure:
   *
   * ```ebnf
   * Block = (* open_brace: *) OPEN_BRACE
   * (* statements: *) Statements
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  Block = "Block",
  /**
   * This kind represents a `BreakStatement` node, with the following structure:
   *
   * ```ebnf
   * BreakStatement = (* break_keyword: *) BREAK_KEYWORD
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  BreakStatement = "BreakStatement",
  /**
   * This kind represents a `CallOptions` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.2 *)
   * CallOptions = (* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*;
   * ```
   */
  CallOptions = "CallOptions",
  /**
   * This kind represents a `CallOptionsExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * (* Introduced in 0.6.2 *)
   * CallOptionsExpression = (* operand: *) Expression
   * (* open_brace: *) OPEN_BRACE
   * (* options: *) CallOptions
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  CallOptionsExpression = "CallOptionsExpression",
  /**
   * This kind represents a `CatchClause` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * CatchClause = (* catch_keyword: *) CATCH_KEYWORD
   * (* error: *) CatchClauseError?
   * (* body: *) Block;
   * ```
   */
  CatchClause = "CatchClause",
  /**
   * This kind represents a `CatchClauseError` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * CatchClauseError = (* name: *) IDENTIFIER?
   * (* parameters: *) ParametersDeclaration;
   * ```
   */
  CatchClauseError = "CatchClauseError",
  /**
   * This kind represents a `CatchClauses` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * CatchClauses = (* item: *) CatchClause+;
   * ```
   */
  CatchClauses = "CatchClauses",
  /**
   * This kind represents a `ComparisonExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * ComparisonExpression = (* left_operand: *) Expression
   * (* operator: *) LESS_THAN
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * ComparisonExpression = (* left_operand: *) Expression
   * (* operator: *) GREATER_THAN
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * ComparisonExpression = (* left_operand: *) Expression
   * (* operator: *) LESS_THAN_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * ComparisonExpression = (* left_operand: *) Expression
   * (* operator: *) GREATER_THAN_EQUAL
   * (* right_operand: *) Expression;
   * ```
   */
  ComparisonExpression = "ComparisonExpression",
  /**
   * This kind represents a `ConditionalExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * ConditionalExpression = (* operand: *) Expression
   * (* question_mark: *) QUESTION_MARK
   * (* true_expression: *) Expression
   * (* colon: *) COLON
   * (* false_expression: *) Expression;
   * ```
   */
  ConditionalExpression = "ConditionalExpression",
  /**
   * This kind represents a `ConstantDefinition` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.4 *)
   * ConstantDefinition = (* type_name: *) TypeName
   * (* constant_keyword: *) CONSTANT_KEYWORD
   * (* name: *) IDENTIFIER
   * (* equal: *) EQUAL
   * (* value: *) Expression
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  ConstantDefinition = "ConstantDefinition",
  /**
   * This kind represents a `ConstructorAttribute` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.22 *)
   * ConstructorAttribute = (* variant: *) ModifierInvocation
   * | (* variant: *) INTERNAL_KEYWORD
   * | (* variant: *) OVERRIDE_KEYWORD (* Introduced in 0.6.0 and deprecated in 0.6.7. *)
   * | (* variant: *) PAYABLE_KEYWORD
   * | (* variant: *) PUBLIC_KEYWORD
   * | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 and deprecated in 0.6.7. *)
   * ```
   */
  ConstructorAttribute = "ConstructorAttribute",
  /**
   * This kind represents a `ConstructorAttributes` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.22 *)
   * ConstructorAttributes = (* item: *) ConstructorAttribute*;
   * ```
   */
  ConstructorAttributes = "ConstructorAttributes",
  /**
   * This kind represents a `ConstructorDefinition` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.22 *)
   * ConstructorDefinition = (* constructor_keyword: *) CONSTRUCTOR_KEYWORD
   * (* parameters: *) ParametersDeclaration
   * (* attributes: *) ConstructorAttributes
   * (* body: *) Block;
   * ```
   */
  ConstructorDefinition = "ConstructorDefinition",
  /**
   * This kind represents a `ContinueStatement` node, with the following structure:
   *
   * ```ebnf
   * ContinueStatement = (* continue_keyword: *) CONTINUE_KEYWORD
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  ContinueStatement = "ContinueStatement",
  /**
   * This kind represents a `ContractDefinition` node, with the following structure:
   *
   * ```ebnf
   * ContractDefinition = (* abstract_keyword: *) ABSTRACT_KEYWORD? (* Introduced in 0.6.0 *)
   * (* contract_keyword: *) CONTRACT_KEYWORD
   * (* name: *) IDENTIFIER
   * (* inheritance: *) InheritanceSpecifier?
   * (* open_brace: *) OPEN_BRACE
   * (* members: *) ContractMembers
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  ContractDefinition = "ContractDefinition",
  /**
   * This kind represents a `ContractMember` node, with the following structure:
   *
   * ```ebnf
   * ContractMember = (* variant: *) UsingDirective
   * | (* variant: *) FunctionDefinition
   * | (* variant: *) ConstructorDefinition (* Introduced in 0.4.22 *)
   * | (* variant: *) ReceiveFunctionDefinition (* Introduced in 0.6.0 *)
   * | (* variant: *) FallbackFunctionDefinition (* Introduced in 0.6.0 *)
   * | (* variant: *) UnnamedFunctionDefinition (* Deprecated in 0.6.0 *)
   * | (* variant: *) ModifierDefinition
   * | (* variant: *) StructDefinition
   * | (* variant: *) EnumDefinition
   * | (* variant: *) EventDefinition
   * | (* variant: *) ErrorDefinition (* Introduced in 0.8.4 *)
   * | (* variant: *) UserDefinedValueTypeDefinition (* Introduced in 0.8.8 *)
   * | (* variant: *) StateVariableDefinition;
   * ```
   */
  ContractMember = "ContractMember",
  /**
   * This kind represents a `ContractMembers` node, with the following structure:
   *
   * ```ebnf
   * ContractMembers = (* item: *) ContractMember*;
   * ```
   */
  ContractMembers = "ContractMembers",
  /**
   * This kind represents a `DecimalNumberExpression` node, with the following structure:
   *
   * ```ebnf
   * DecimalNumberExpression = (* literal: *) DECIMAL_LITERAL
   * (* unit: *) NumberUnit?;
   * ```
   */
  DecimalNumberExpression = "DecimalNumberExpression",
  /**
   * This kind represents a `DoWhileStatement` node, with the following structure:
   *
   * ```ebnf
   * DoWhileStatement = (* do_keyword: *) DO_KEYWORD
   * (* body: *) Statement
   * (* while_keyword: *) WHILE_KEYWORD
   * (* open_paren: *) OPEN_PAREN
   * (* condition: *) Expression
   * (* close_paren: *) CLOSE_PAREN
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  DoWhileStatement = "DoWhileStatement",
  /**
   * This kind represents a `ElementaryType` node, with the following structure:
   *
   * ```ebnf
   * ElementaryType = (* variant: *) BOOL_KEYWORD
   * | (* variant: *) BYTE_KEYWORD (* Deprecated in 0.8.0 *)
   * | (* variant: *) STRING_KEYWORD
   * | (* variant: *) AddressType
   * | (* variant: *) BYTES_KEYWORD
   * | (* variant: *) INT_KEYWORD
   * | (* variant: *) UINT_KEYWORD
   * | (* variant: *) FIXED_KEYWORD
   * | (* variant: *) UFIXED_KEYWORD;
   * ```
   */
  ElementaryType = "ElementaryType",
  /**
   * This kind represents a `ElseBranch` node, with the following structure:
   *
   * ```ebnf
   * ElseBranch = (* else_keyword: *) ELSE_KEYWORD
   * (* body: *) Statement;
   * ```
   */
  ElseBranch = "ElseBranch",
  /**
   * This kind represents a `EmitStatement` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.21 *)
   * EmitStatement = (* emit_keyword: *) EMIT_KEYWORD
   * (* event: *) IdentifierPath
   * (* arguments: *) ArgumentsDeclaration
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  EmitStatement = "EmitStatement",
  /**
   * This kind represents a `EnumDefinition` node, with the following structure:
   *
   * ```ebnf
   * EnumDefinition = (* enum_keyword: *) ENUM_KEYWORD
   * (* name: *) IDENTIFIER
   * (* open_brace: *) OPEN_BRACE
   * (* members: *) EnumMembers
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  EnumDefinition = "EnumDefinition",
  /**
   * This kind represents a `EnumMembers` node, with the following structure:
   *
   * ```ebnf
   * EnumMembers = ((* item: *) IDENTIFIER ((* separator: *) COMMA (* item: *) IDENTIFIER)*)?;
   * ```
   */
  EnumMembers = "EnumMembers",
  /**
   * This kind represents a `EqualityExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * EqualityExpression = (* left_operand: *) Expression
   * (* operator: *) EQUAL_EQUAL
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * EqualityExpression = (* left_operand: *) Expression
   * (* operator: *) BANG_EQUAL
   * (* right_operand: *) Expression;
   * ```
   */
  EqualityExpression = "EqualityExpression",
  /**
   * This kind represents a `ErrorDefinition` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * ErrorDefinition = (* error_keyword: *) ERROR_KEYWORD
   * (* name: *) IDENTIFIER
   * (* members: *) ErrorParametersDeclaration
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  ErrorDefinition = "ErrorDefinition",
  /**
   * This kind represents a `ErrorParameter` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * ErrorParameter = (* type_name: *) TypeName
   * (* name: *) IDENTIFIER?;
   * ```
   */
  ErrorParameter = "ErrorParameter",
  /**
   * This kind represents a `ErrorParameters` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * ErrorParameters = ((* item: *) ErrorParameter ((* separator: *) COMMA (* item: *) ErrorParameter)*)?;
   * ```
   */
  ErrorParameters = "ErrorParameters",
  /**
   * This kind represents a `ErrorParametersDeclaration` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * ErrorParametersDeclaration = (* open_paren: *) OPEN_PAREN
   * (* parameters: *) ErrorParameters
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  ErrorParametersDeclaration = "ErrorParametersDeclaration",
  /**
   * This kind represents a `EventDefinition` node, with the following structure:
   *
   * ```ebnf
   * EventDefinition = (* event_keyword: *) EVENT_KEYWORD
   * (* name: *) IDENTIFIER
   * (* parameters: *) EventParametersDeclaration
   * (* anonymous_keyword: *) ANONYMOUS_KEYWORD?
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  EventDefinition = "EventDefinition",
  /**
   * This kind represents a `EventParameter` node, with the following structure:
   *
   * ```ebnf
   * EventParameter = (* type_name: *) TypeName
   * (* indexed_keyword: *) INDEXED_KEYWORD?
   * (* name: *) IDENTIFIER?;
   * ```
   */
  EventParameter = "EventParameter",
  /**
   * This kind represents a `EventParameters` node, with the following structure:
   *
   * ```ebnf
   * EventParameters = ((* item: *) EventParameter ((* separator: *) COMMA (* item: *) EventParameter)*)?;
   * ```
   */
  EventParameters = "EventParameters",
  /**
   * This kind represents a `EventParametersDeclaration` node, with the following structure:
   *
   * ```ebnf
   * EventParametersDeclaration = (* open_paren: *) OPEN_PAREN
   * (* parameters: *) EventParameters
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  EventParametersDeclaration = "EventParametersDeclaration",
  /**
   * This kind represents a `ExperimentalFeature` node, with the following structure:
   *
   * ```ebnf
   * ExperimentalFeature = (* variant: *) IDENTIFIER
   * | (* variant: *) StringLiteral;
   * ```
   */
  ExperimentalFeature = "ExperimentalFeature",
  /**
   * This kind represents a `ExperimentalPragma` node, with the following structure:
   *
   * ```ebnf
   * ExperimentalPragma = (* experimental_keyword: *) EXPERIMENTAL_KEYWORD
   * (* feature: *) ExperimentalFeature;
   * ```
   */
  ExperimentalPragma = "ExperimentalPragma",
  /**
   * This kind represents a `ExponentiationExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * (* Deprecated in 0.8.0 *)
   * ExponentiationExpression = (* left_operand: *) Expression
   * (* operator: *) ASTERISK_ASTERISK
   * (* right_operand: *) Expression;
   *
   * (* Right-associative binary operator *)
   * (* Introduced in 0.8.0 *)
   * ExponentiationExpression = (* left_operand: *) Expression
   * (* operator: *) ASTERISK_ASTERISK
   * (* right_operand: *) Expression;
   * ```
   */
  ExponentiationExpression = "ExponentiationExpression",
  /**
   * This kind represents a `Expression` node, with the following structure:
   *
   * ```ebnf
   * Expression = (* variant: *) AssignmentExpression
   * | (* variant: *) ConditionalExpression
   * | (* variant: *) OrExpression
   * | (* variant: *) AndExpression
   * | (* variant: *) EqualityExpression
   * | (* variant: *) ComparisonExpression
   * | (* variant: *) BitwiseOrExpression
   * | (* variant: *) BitwiseXorExpression
   * | (* variant: *) BitwiseAndExpression
   * | (* variant: *) ShiftExpression
   * | (* variant: *) AdditiveExpression
   * | (* variant: *) MultiplicativeExpression
   * | (* variant: *) ExponentiationExpression
   * | (* variant: *) PostfixExpression
   * | (* variant: *) PrefixExpression
   * | (* variant: *) FunctionCallExpression
   * | (* variant: *) CallOptionsExpression
   * | (* variant: *) MemberAccessExpression
   * | (* variant: *) IndexAccessExpression
   * | (* variant: *) NewExpression
   * | (* variant: *) TupleExpression
   * | (* variant: *) TypeExpression (* Introduced in 0.5.3 *)
   * | (* variant: *) ArrayExpression
   * | (* variant: *) HexNumberExpression
   * | (* variant: *) DecimalNumberExpression
   * | (* variant: *) StringExpression
   * | (* variant: *) ElementaryType
   * | (* variant: *) PAYABLE_KEYWORD (* Introduced in 0.6.0 *)
   * | (* variant: *) THIS_KEYWORD
   * | (* variant: *) SUPER_KEYWORD
   * | (* variant: *) TRUE_KEYWORD
   * | (* variant: *) FALSE_KEYWORD
   * | (* variant: *) IDENTIFIER;
   * ```
   */
  Expression = "Expression",
  /**
   * This kind represents a `ExpressionStatement` node, with the following structure:
   *
   * ```ebnf
   * ExpressionStatement = (* expression: *) Expression
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  ExpressionStatement = "ExpressionStatement",
  /**
   * This kind represents a `FallbackFunctionAttribute` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * FallbackFunctionAttribute = (* variant: *) ModifierInvocation
   * | (* variant: *) OverrideSpecifier
   * | (* variant: *) EXTERNAL_KEYWORD
   * | (* variant: *) PAYABLE_KEYWORD
   * | (* variant: *) PURE_KEYWORD
   * | (* variant: *) VIEW_KEYWORD
   * | (* variant: *) VIRTUAL_KEYWORD;
   * ```
   */
  FallbackFunctionAttribute = "FallbackFunctionAttribute",
  /**
   * This kind represents a `FallbackFunctionAttributes` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * FallbackFunctionAttributes = (* item: *) FallbackFunctionAttribute*;
   * ```
   */
  FallbackFunctionAttributes = "FallbackFunctionAttributes",
  /**
   * This kind represents a `FallbackFunctionDefinition` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * FallbackFunctionDefinition = (* fallback_keyword: *) FALLBACK_KEYWORD
   * (* parameters: *) ParametersDeclaration
   * (* attributes: *) FallbackFunctionAttributes
   * (* returns: *) ReturnsDeclaration?
   * (* body: *) FunctionBody;
   * ```
   */
  FallbackFunctionDefinition = "FallbackFunctionDefinition",
  /**
   * This kind represents a `ForStatement` node, with the following structure:
   *
   * ```ebnf
   * ForStatement = (* for_keyword: *) FOR_KEYWORD
   * (* open_paren: *) OPEN_PAREN
   * (* initialization: *) ForStatementInitialization
   * (* condition: *) ForStatementCondition
   * (* iterator: *) Expression?
   * (* close_paren: *) CLOSE_PAREN
   * (* body: *) Statement;
   * ```
   */
  ForStatement = "ForStatement",
  /**
   * This kind represents a `ForStatementCondition` node, with the following structure:
   *
   * ```ebnf
   * ForStatementCondition = (* variant: *) ExpressionStatement
   * | (* variant: *) SEMICOLON;
   * ```
   */
  ForStatementCondition = "ForStatementCondition",
  /**
   * This kind represents a `ForStatementInitialization` node, with the following structure:
   *
   * ```ebnf
   * ForStatementInitialization = (* variant: *) TupleDeconstructionStatement
   * | (* variant: *) VariableDeclarationStatement
   * | (* variant: *) ExpressionStatement
   * | (* variant: *) SEMICOLON;
   * ```
   */
  ForStatementInitialization = "ForStatementInitialization",
  /**
   * This kind represents a `FunctionAttribute` node, with the following structure:
   *
   * ```ebnf
   * FunctionAttribute = (* variant: *) ModifierInvocation
   * | (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
   * | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) EXTERNAL_KEYWORD
   * | (* variant: *) INTERNAL_KEYWORD
   * | (* variant: *) PAYABLE_KEYWORD
   * | (* variant: *) PRIVATE_KEYWORD
   * | (* variant: *) PUBLIC_KEYWORD
   * | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 *)
   * | (* variant: *) VIEW_KEYWORD (* Introduced in 0.4.16 *)
   * | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
   * ```
   */
  FunctionAttribute = "FunctionAttribute",
  /**
   * This kind represents a `FunctionAttributes` node, with the following structure:
   *
   * ```ebnf
   * FunctionAttributes = (* item: *) FunctionAttribute*;
   * ```
   */
  FunctionAttributes = "FunctionAttributes",
  /**
   * This kind represents a `FunctionBody` node, with the following structure:
   *
   * ```ebnf
   * FunctionBody = (* variant: *) Block
   * | (* variant: *) SEMICOLON;
   * ```
   */
  FunctionBody = "FunctionBody",
  /**
   * This kind represents a `FunctionCallExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * FunctionCallExpression = (* operand: *) Expression
   * (* arguments: *) ArgumentsDeclaration;
   * ```
   */
  FunctionCallExpression = "FunctionCallExpression",
  /**
   * This kind represents a `FunctionDefinition` node, with the following structure:
   *
   * ```ebnf
   * FunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
   * (* name: *) FunctionName
   * (* parameters: *) ParametersDeclaration
   * (* attributes: *) FunctionAttributes
   * (* returns: *) ReturnsDeclaration?
   * (* body: *) FunctionBody;
   * ```
   */
  FunctionDefinition = "FunctionDefinition",
  /**
   * This kind represents a `FunctionName` node, with the following structure:
   *
   * ```ebnf
   * FunctionName = (* variant: *) IDENTIFIER
   * | (* variant: *) FALLBACK_KEYWORD
   * | (* variant: *) RECEIVE_KEYWORD;
   * ```
   */
  FunctionName = "FunctionName",
  /**
   * This kind represents a `FunctionType` node, with the following structure:
   *
   * ```ebnf
   * FunctionType = (* function_keyword: *) FUNCTION_KEYWORD
   * (* parameters: *) ParametersDeclaration
   * (* attributes: *) FunctionTypeAttributes
   * (* returns: *) ReturnsDeclaration?;
   * ```
   */
  FunctionType = "FunctionType",
  /**
   * This kind represents a `FunctionTypeAttribute` node, with the following structure:
   *
   * ```ebnf
   * FunctionTypeAttribute = (* variant: *) INTERNAL_KEYWORD
   * | (* variant: *) EXTERNAL_KEYWORD
   * | (* variant: *) PRIVATE_KEYWORD
   * | (* variant: *) PUBLIC_KEYWORD
   * | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 *)
   * | (* variant: *) VIEW_KEYWORD (* Introduced in 0.4.16 *)
   * | (* variant: *) PAYABLE_KEYWORD;
   * ```
   */
  FunctionTypeAttribute = "FunctionTypeAttribute",
  /**
   * This kind represents a `FunctionTypeAttributes` node, with the following structure:
   *
   * ```ebnf
   * FunctionTypeAttributes = (* item: *) FunctionTypeAttribute*;
   * ```
   */
  FunctionTypeAttributes = "FunctionTypeAttributes",
  /**
   * This kind represents a `HexNumberExpression` node, with the following structure:
   *
   * ```ebnf
   * HexNumberExpression = (* literal: *) HEX_LITERAL
   * (* unit: *) NumberUnit?; (* Deprecated in 0.5.0 *)
   * ```
   */
  HexNumberExpression = "HexNumberExpression",
  /**
   * This kind represents a `HexStringLiteral` node, with the following structure:
   *
   * ```ebnf
   * HexStringLiteral = (* variant: *) SINGLE_QUOTED_HEX_STRING_LITERAL
   * | (* variant: *) DOUBLE_QUOTED_HEX_STRING_LITERAL;
   * ```
   */
  HexStringLiteral = "HexStringLiteral",
  /**
   * This kind represents a `HexStringLiterals` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.14 *)
   * HexStringLiterals = (* item: *) HexStringLiteral+;
   * ```
   */
  HexStringLiterals = "HexStringLiterals",
  /**
   * This kind represents a `IdentifierPath` node, with the following structure:
   *
   * ```ebnf
   * IdentifierPath = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
   * ```
   */
  IdentifierPath = "IdentifierPath",
  /**
   * This kind represents a `IfStatement` node, with the following structure:
   *
   * ```ebnf
   * IfStatement = (* if_keyword: *) IF_KEYWORD
   * (* open_paren: *) OPEN_PAREN
   * (* condition: *) Expression
   * (* close_paren: *) CLOSE_PAREN
   * (* body: *) Statement
   * (* else_branch: *) ElseBranch?;
   * ```
   */
  IfStatement = "IfStatement",
  /**
   * This kind represents a `ImportAlias` node, with the following structure:
   *
   * ```ebnf
   * ImportAlias = (* as_keyword: *) AS_KEYWORD
   * (* identifier: *) IDENTIFIER;
   * ```
   */
  ImportAlias = "ImportAlias",
  /**
   * This kind represents a `ImportClause` node, with the following structure:
   *
   * ```ebnf
   * ImportClause = (* variant: *) PathImport
   * | (* variant: *) NamedImport
   * | (* variant: *) ImportDeconstruction;
   * ```
   */
  ImportClause = "ImportClause",
  /**
   * This kind represents a `ImportDeconstruction` node, with the following structure:
   *
   * ```ebnf
   * ImportDeconstruction = (* open_brace: *) OPEN_BRACE
   * (* symbols: *) ImportDeconstructionSymbols
   * (* close_brace: *) CLOSE_BRACE
   * (* from_keyword: *) FROM_KEYWORD
   * (* path: *) StringLiteral;
   * ```
   */
  ImportDeconstruction = "ImportDeconstruction",
  /**
   * This kind represents a `ImportDeconstructionSymbol` node, with the following structure:
   *
   * ```ebnf
   * ImportDeconstructionSymbol = (* name: *) IDENTIFIER
   * (* alias: *) ImportAlias?;
   * ```
   */
  ImportDeconstructionSymbol = "ImportDeconstructionSymbol",
  /**
   * This kind represents a `ImportDeconstructionSymbols` node, with the following structure:
   *
   * ```ebnf
   * ImportDeconstructionSymbols = (* item: *) ImportDeconstructionSymbol ((* separator: *) COMMA (* item: *) ImportDeconstructionSymbol)*;
   * ```
   */
  ImportDeconstructionSymbols = "ImportDeconstructionSymbols",
  /**
   * This kind represents a `ImportDirective` node, with the following structure:
   *
   * ```ebnf
   * ImportDirective = (* import_keyword: *) IMPORT_KEYWORD
   * (* clause: *) ImportClause
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  ImportDirective = "ImportDirective",
  /**
   * This kind represents a `IndexAccessEnd` node, with the following structure:
   *
   * ```ebnf
   * IndexAccessEnd = (* colon: *) COLON
   * (* end: *) Expression?;
   * ```
   */
  IndexAccessEnd = "IndexAccessEnd",
  /**
   * This kind represents a `IndexAccessExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * IndexAccessExpression = (* operand: *) Expression
   * (* open_bracket: *) OPEN_BRACKET
   * (* start: *) Expression?
   * (* end: *) IndexAccessEnd?
   * (* close_bracket: *) CLOSE_BRACKET;
   * ```
   */
  IndexAccessExpression = "IndexAccessExpression",
  /**
   * This kind represents a `InheritanceSpecifier` node, with the following structure:
   *
   * ```ebnf
   * InheritanceSpecifier = (* is_keyword: *) IS_KEYWORD
   * (* types: *) InheritanceTypes;
   * ```
   */
  InheritanceSpecifier = "InheritanceSpecifier",
  /**
   * This kind represents a `InheritanceType` node, with the following structure:
   *
   * ```ebnf
   * InheritanceType = (* type_name: *) IdentifierPath
   * (* arguments: *) ArgumentsDeclaration?;
   * ```
   */
  InheritanceType = "InheritanceType",
  /**
   * This kind represents a `InheritanceTypes` node, with the following structure:
   *
   * ```ebnf
   * InheritanceTypes = (* item: *) InheritanceType ((* separator: *) COMMA (* item: *) InheritanceType)*;
   * ```
   */
  InheritanceTypes = "InheritanceTypes",
  /**
   * This kind represents a `InterfaceDefinition` node, with the following structure:
   *
   * ```ebnf
   * InterfaceDefinition = (* interface_keyword: *) INTERFACE_KEYWORD
   * (* name: *) IDENTIFIER
   * (* inheritance: *) InheritanceSpecifier?
   * (* open_brace: *) OPEN_BRACE
   * (* members: *) InterfaceMembers
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  InterfaceDefinition = "InterfaceDefinition",
  /**
   * This kind represents a `InterfaceMembers` node, with the following structure:
   *
   * ```ebnf
   * InterfaceMembers = (* item: *) ContractMember*;
   * ```
   */
  InterfaceMembers = "InterfaceMembers",
  /**
   * This kind represents a `LibraryDefinition` node, with the following structure:
   *
   * ```ebnf
   * LibraryDefinition = (* library_keyword: *) LIBRARY_KEYWORD
   * (* name: *) IDENTIFIER
   * (* open_brace: *) OPEN_BRACE
   * (* members: *) LibraryMembers
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  LibraryDefinition = "LibraryDefinition",
  /**
   * This kind represents a `LibraryMembers` node, with the following structure:
   *
   * ```ebnf
   * LibraryMembers = (* item: *) ContractMember*;
   * ```
   */
  LibraryMembers = "LibraryMembers",
  /**
   * This kind represents a `MappingKey` node, with the following structure:
   *
   * ```ebnf
   * MappingKey = (* key_type: *) MappingKeyType
   * (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
   * ```
   */
  MappingKey = "MappingKey",
  /**
   * This kind represents a `MappingKeyType` node, with the following structure:
   *
   * ```ebnf
   * MappingKeyType = (* variant: *) ElementaryType
   * | (* variant: *) IdentifierPath;
   * ```
   */
  MappingKeyType = "MappingKeyType",
  /**
   * This kind represents a `MappingType` node, with the following structure:
   *
   * ```ebnf
   * MappingType = (* mapping_keyword: *) MAPPING_KEYWORD
   * (* open_paren: *) OPEN_PAREN
   * (* key_type: *) MappingKey
   * (* equal_greater_than: *) EQUAL_GREATER_THAN
   * (* value_type: *) MappingValue
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  MappingType = "MappingType",
  /**
   * This kind represents a `MappingValue` node, with the following structure:
   *
   * ```ebnf
   * MappingValue = (* type_name: *) TypeName
   * (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
   * ```
   */
  MappingValue = "MappingValue",
  /**
   * This kind represents a `MemberAccessExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * MemberAccessExpression = (* operand: *) Expression
   * (* period: *) PERIOD
   * (* member: *) IDENTIFIER;
   * ```
   */
  MemberAccessExpression = "MemberAccessExpression",
  /**
   * This kind represents a `ModifierAttribute` node, with the following structure:
   *
   * ```ebnf
   * ModifierAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
   * | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
   * ```
   */
  ModifierAttribute = "ModifierAttribute",
  /**
   * This kind represents a `ModifierAttributes` node, with the following structure:
   *
   * ```ebnf
   * ModifierAttributes = (* item: *) ModifierAttribute*;
   * ```
   */
  ModifierAttributes = "ModifierAttributes",
  /**
   * This kind represents a `ModifierDefinition` node, with the following structure:
   *
   * ```ebnf
   * ModifierDefinition = (* modifier_keyword: *) MODIFIER_KEYWORD
   * (* name: *) IDENTIFIER
   * (* parameters: *) ParametersDeclaration?
   * (* attributes: *) ModifierAttributes
   * (* body: *) FunctionBody;
   * ```
   */
  ModifierDefinition = "ModifierDefinition",
  /**
   * This kind represents a `ModifierInvocation` node, with the following structure:
   *
   * ```ebnf
   * ModifierInvocation = (* name: *) IdentifierPath
   * (* arguments: *) ArgumentsDeclaration?;
   * ```
   */
  ModifierInvocation = "ModifierInvocation",
  /**
   * This kind represents a `MultiplicativeExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * MultiplicativeExpression = (* left_operand: *) Expression
   * (* operator: *) ASTERISK
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * MultiplicativeExpression = (* left_operand: *) Expression
   * (* operator: *) SLASH
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * MultiplicativeExpression = (* left_operand: *) Expression
   * (* operator: *) PERCENT
   * (* right_operand: *) Expression;
   * ```
   */
  MultiplicativeExpression = "MultiplicativeExpression",
  /**
   * This kind represents a `NamedArgument` node, with the following structure:
   *
   * ```ebnf
   * NamedArgument = (* name: *) IDENTIFIER
   * (* colon: *) COLON
   * (* value: *) Expression;
   * ```
   */
  NamedArgument = "NamedArgument",
  /**
   * This kind represents a `NamedArgumentGroup` node, with the following structure:
   *
   * ```ebnf
   * NamedArgumentGroup = (* open_brace: *) OPEN_BRACE
   * (* arguments: *) NamedArguments
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  NamedArgumentGroup = "NamedArgumentGroup",
  /**
   * This kind represents a `NamedArguments` node, with the following structure:
   *
   * ```ebnf
   * NamedArguments = ((* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*)?;
   * ```
   */
  NamedArguments = "NamedArguments",
  /**
   * This kind represents a `NamedArgumentsDeclaration` node, with the following structure:
   *
   * ```ebnf
   * NamedArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
   * (* arguments: *) NamedArgumentGroup?
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  NamedArgumentsDeclaration = "NamedArgumentsDeclaration",
  /**
   * This kind represents a `NamedImport` node, with the following structure:
   *
   * ```ebnf
   * NamedImport = (* asterisk: *) ASTERISK
   * (* alias: *) ImportAlias
   * (* from_keyword: *) FROM_KEYWORD
   * (* path: *) StringLiteral;
   * ```
   */
  NamedImport = "NamedImport",
  /**
   * This kind represents a `NewExpression` node, with the following structure:
   *
   * ```ebnf
   * NewExpression = (* new_keyword: *) NEW_KEYWORD
   * (* type_name: *) TypeName;
   * ```
   */
  NewExpression = "NewExpression",
  /**
   * This kind represents a `NumberUnit` node, with the following structure:
   *
   * ```ebnf
   * NumberUnit = (* variant: *) WEI_KEYWORD
   * | (* variant: *) GWEI_KEYWORD (* Introduced in 0.6.11 *)
   * | (* variant: *) SZABO_KEYWORD (* Deprecated in 0.7.0 *)
   * | (* variant: *) FINNEY_KEYWORD (* Deprecated in 0.7.0 *)
   * | (* variant: *) ETHER_KEYWORD
   * | (* variant: *) SECONDS_KEYWORD
   * | (* variant: *) MINUTES_KEYWORD
   * | (* variant: *) HOURS_KEYWORD
   * | (* variant: *) DAYS_KEYWORD
   * | (* variant: *) WEEKS_KEYWORD
   * | (* variant: *) YEARS_KEYWORD; (* Deprecated in 0.5.0 *)
   * ```
   */
  NumberUnit = "NumberUnit",
  /**
   * This kind represents a `OrExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * OrExpression = (* left_operand: *) Expression
   * (* operator: *) BAR_BAR
   * (* right_operand: *) Expression;
   * ```
   */
  OrExpression = "OrExpression",
  /**
   * This kind represents a `OverridePaths` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * OverridePaths = (* item: *) IdentifierPath ((* separator: *) COMMA (* item: *) IdentifierPath)*;
   * ```
   */
  OverridePaths = "OverridePaths",
  /**
   * This kind represents a `OverridePathsDeclaration` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * OverridePathsDeclaration = (* open_paren: *) OPEN_PAREN
   * (* paths: *) OverridePaths
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  OverridePathsDeclaration = "OverridePathsDeclaration",
  /**
   * This kind represents a `OverrideSpecifier` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * OverrideSpecifier = (* override_keyword: *) OVERRIDE_KEYWORD
   * (* overridden: *) OverridePathsDeclaration?;
   * ```
   */
  OverrideSpecifier = "OverrideSpecifier",
  /**
   * This kind represents a `Parameter` node, with the following structure:
   *
   * ```ebnf
   * Parameter = (* type_name: *) TypeName
   * (* storage_location: *) StorageLocation?
   * (* name: *) IDENTIFIER?;
   * ```
   */
  Parameter = "Parameter",
  /**
   * This kind represents a `Parameters` node, with the following structure:
   *
   * ```ebnf
   * Parameters = ((* item: *) Parameter ((* separator: *) COMMA (* item: *) Parameter)*)?;
   * ```
   */
  Parameters = "Parameters",
  /**
   * This kind represents a `ParametersDeclaration` node, with the following structure:
   *
   * ```ebnf
   * ParametersDeclaration = (* open_paren: *) OPEN_PAREN
   * (* parameters: *) Parameters
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  ParametersDeclaration = "ParametersDeclaration",
  /**
   * This kind represents a `PathImport` node, with the following structure:
   *
   * ```ebnf
   * PathImport = (* path: *) StringLiteral
   * (* alias: *) ImportAlias?;
   * ```
   */
  PathImport = "PathImport",
  /**
   * This kind represents a `PositionalArguments` node, with the following structure:
   *
   * ```ebnf
   * PositionalArguments = ((* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*)?;
   * ```
   */
  PositionalArguments = "PositionalArguments",
  /**
   * This kind represents a `PositionalArgumentsDeclaration` node, with the following structure:
   *
   * ```ebnf
   * PositionalArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
   * (* arguments: *) PositionalArguments
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  PositionalArgumentsDeclaration = "PositionalArgumentsDeclaration",
  /**
   * This kind represents a `PostfixExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * PostfixExpression = (* operand: *) Expression
   * (* operator: *) PLUS_PLUS;
   *
   * (* Postfix unary operator *)
   * PostfixExpression = (* operand: *) Expression
   * (* operator: *) MINUS_MINUS;
   * ```
   */
  PostfixExpression = "PostfixExpression",
  /**
   * This kind represents a `Pragma` node, with the following structure:
   *
   * ```ebnf
   * Pragma = (* variant: *) AbicoderPragma
   * | (* variant: *) ExperimentalPragma
   * | (* variant: *) VersionPragma;
   * ```
   */
  Pragma = "Pragma",
  /**
   * This kind represents a `PragmaDirective` node, with the following structure:
   *
   * ```ebnf
   * PragmaDirective = (* pragma_keyword: *) PRAGMA_KEYWORD
   * (* pragma: *) Pragma
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  PragmaDirective = "PragmaDirective",
  /**
   * This kind represents a `PrefixExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) PLUS_PLUS
   * (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) MINUS_MINUS
   * (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) TILDE
   * (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) BANG
   * (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) MINUS
   * (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * (* Deprecated in 0.5.0 *)
   * PrefixExpression = (* operator: *) PLUS
   * (* operand: *) Expression;
   *
   * (* Prefix unary operator *)
   * PrefixExpression = (* operator: *) DELETE_KEYWORD
   * (* operand: *) Expression;
   * ```
   */
  PrefixExpression = "PrefixExpression",
  /**
   * This kind represents a `ReceiveFunctionAttribute` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * ReceiveFunctionAttribute = (* variant: *) ModifierInvocation
   * | (* variant: *) OverrideSpecifier
   * | (* variant: *) EXTERNAL_KEYWORD
   * | (* variant: *) PAYABLE_KEYWORD
   * | (* variant: *) VIRTUAL_KEYWORD;
   * ```
   */
  ReceiveFunctionAttribute = "ReceiveFunctionAttribute",
  /**
   * This kind represents a `ReceiveFunctionAttributes` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * ReceiveFunctionAttributes = (* item: *) ReceiveFunctionAttribute*;
   * ```
   */
  ReceiveFunctionAttributes = "ReceiveFunctionAttributes",
  /**
   * This kind represents a `ReceiveFunctionDefinition` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * ReceiveFunctionDefinition = (* receive_keyword: *) RECEIVE_KEYWORD
   * (* parameters: *) ParametersDeclaration
   * (* attributes: *) ReceiveFunctionAttributes
   * (* body: *) FunctionBody;
   * ```
   */
  ReceiveFunctionDefinition = "ReceiveFunctionDefinition",
  /**
   * This kind represents a `ReturnStatement` node, with the following structure:
   *
   * ```ebnf
   * ReturnStatement = (* return_keyword: *) RETURN_KEYWORD
   * (* expression: *) Expression?
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  ReturnStatement = "ReturnStatement",
  /**
   * This kind represents a `ReturnsDeclaration` node, with the following structure:
   *
   * ```ebnf
   * ReturnsDeclaration = (* returns_keyword: *) RETURNS_KEYWORD
   * (* variables: *) ParametersDeclaration;
   * ```
   */
  ReturnsDeclaration = "ReturnsDeclaration",
  /**
   * This kind represents a `RevertStatement` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * RevertStatement = (* revert_keyword: *) REVERT_KEYWORD
   * (* error: *) IdentifierPath?
   * (* arguments: *) ArgumentsDeclaration
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  RevertStatement = "RevertStatement",
  /**
   * This kind represents a `ShiftExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * ShiftExpression = (* left_operand: *) Expression
   * (* operator: *) LESS_THAN_LESS_THAN
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * ShiftExpression = (* left_operand: *) Expression
   * (* operator: *) GREATER_THAN_GREATER_THAN
   * (* right_operand: *) Expression;
   *
   * (* Left-associative binary operator *)
   * ShiftExpression = (* left_operand: *) Expression
   * (* operator: *) GREATER_THAN_GREATER_THAN_GREATER_THAN
   * (* right_operand: *) Expression;
   * ```
   */
  ShiftExpression = "ShiftExpression",
  /**
   * This kind represents a `SimpleVersionLiteral` node, with the following structure:
   *
   * ```ebnf
   * SimpleVersionLiteral = (* item: *) VERSION_SPECIFIER ((* separator: *) PERIOD (* item: *) VERSION_SPECIFIER)*;
   * ```
   */
  SimpleVersionLiteral = "SimpleVersionLiteral",
  /**
   * This kind represents a `SourceUnit` node, with the following structure:
   *
   * ```ebnf
   * SourceUnit = (* members: *) SourceUnitMembers;
   * ```
   */
  SourceUnit = "SourceUnit",
  /**
   * This kind represents a `SourceUnitMember` node, with the following structure:
   *
   * ```ebnf
   * SourceUnitMember = (* variant: *) PragmaDirective
   * | (* variant: *) ImportDirective
   * | (* variant: *) ContractDefinition
   * | (* variant: *) InterfaceDefinition
   * | (* variant: *) LibraryDefinition
   * | (* variant: *) StructDefinition (* Introduced in 0.6.0 *)
   * | (* variant: *) EnumDefinition (* Introduced in 0.6.0 *)
   * | (* variant: *) FunctionDefinition (* Introduced in 0.7.1 *)
   * | (* variant: *) ErrorDefinition (* Introduced in 0.8.4 *)
   * | (* variant: *) UserDefinedValueTypeDefinition (* Introduced in 0.8.8 *)
   * | (* variant: *) UsingDirective (* Introduced in 0.8.13 *)
   * | (* variant: *) EventDefinition (* Introduced in 0.8.22 *)
   * | (* variant: *) ConstantDefinition; (* Introduced in 0.7.4 *)
   * ```
   */
  SourceUnitMember = "SourceUnitMember",
  /**
   * This kind represents a `SourceUnitMembers` node, with the following structure:
   *
   * ```ebnf
   * SourceUnitMembers = (* item: *) SourceUnitMember*;
   * ```
   */
  SourceUnitMembers = "SourceUnitMembers",
  /**
   * This kind represents a `StateVariableAttribute` node, with the following structure:
   *
   * ```ebnf
   * StateVariableAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
   * | (* variant: *) CONSTANT_KEYWORD
   * | (* variant: *) INTERNAL_KEYWORD
   * | (* variant: *) PRIVATE_KEYWORD
   * | (* variant: *) PUBLIC_KEYWORD
   * | (* variant: *) IMMUTABLE_KEYWORD (* Introduced in 0.6.5 *)
   * | (* variant: *) TRANSIENT_KEYWORD; (* Introduced in 0.8.27 *)
   * ```
   */
  StateVariableAttribute = "StateVariableAttribute",
  /**
   * This kind represents a `StateVariableAttributes` node, with the following structure:
   *
   * ```ebnf
   * StateVariableAttributes = (* item: *) StateVariableAttribute*;
   * ```
   */
  StateVariableAttributes = "StateVariableAttributes",
  /**
   * This kind represents a `StateVariableDefinition` node, with the following structure:
   *
   * ```ebnf
   * StateVariableDefinition = (* type_name: *) TypeName
   * (* attributes: *) StateVariableAttributes
   * (* name: *) IDENTIFIER
   * (* value: *) StateVariableDefinitionValue?
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  StateVariableDefinition = "StateVariableDefinition",
  /**
   * This kind represents a `StateVariableDefinitionValue` node, with the following structure:
   *
   * ```ebnf
   * StateVariableDefinitionValue = (* equal: *) EQUAL
   * (* value: *) Expression;
   * ```
   */
  StateVariableDefinitionValue = "StateVariableDefinitionValue",
  /**
   * This kind represents a `Statement` node, with the following structure:
   *
   * ```ebnf
   * Statement = (* variant: *) IfStatement
   * | (* variant: *) ForStatement
   * | (* variant: *) WhileStatement
   * | (* variant: *) DoWhileStatement
   * | (* variant: *) ContinueStatement
   * | (* variant: *) BreakStatement
   * | (* variant: *) ReturnStatement
   * | (* variant: *) ThrowStatement (* Deprecated in 0.5.0 *)
   * | (* variant: *) EmitStatement (* Introduced in 0.4.21 *)
   * | (* variant: *) TryStatement (* Introduced in 0.6.0 *)
   * | (* variant: *) RevertStatement (* Introduced in 0.8.4 *)
   * | (* variant: *) AssemblyStatement
   * | (* variant: *) Block
   * | (* variant: *) UncheckedBlock (* Introduced in 0.8.0 *)
   * | (* variant: *) TupleDeconstructionStatement
   * | (* variant: *) VariableDeclarationStatement
   * | (* variant: *) ExpressionStatement;
   * ```
   */
  Statement = "Statement",
  /**
   * This kind represents a `Statements` node, with the following structure:
   *
   * ```ebnf
   * Statements = (* item: *) Statement*;
   * ```
   */
  Statements = "Statements",
  /**
   * This kind represents a `StorageLocation` node, with the following structure:
   *
   * ```ebnf
   * StorageLocation = (* variant: *) MEMORY_KEYWORD
   * | (* variant: *) STORAGE_KEYWORD
   * | (* variant: *) CALL_DATA_KEYWORD; (* Introduced in 0.5.0 *)
   * ```
   */
  StorageLocation = "StorageLocation",
  /**
   * This kind represents a `StringExpression` node, with the following structure:
   *
   * ```ebnf
   * StringExpression = (* variant: *) StringLiteral (* Deprecated in 0.5.14 *)
   * | (* variant: *) StringLiterals (* Introduced in 0.5.14 *)
   * | (* variant: *) HexStringLiteral (* Deprecated in 0.5.14 *)
   * | (* variant: *) HexStringLiterals (* Introduced in 0.5.14 *)
   * | (* variant: *) UnicodeStringLiterals; (* Introduced in 0.7.0 *)
   * ```
   */
  StringExpression = "StringExpression",
  /**
   * This kind represents a `StringLiteral` node, with the following structure:
   *
   * ```ebnf
   * StringLiteral = (* variant: *) SINGLE_QUOTED_STRING_LITERAL
   * | (* variant: *) DOUBLE_QUOTED_STRING_LITERAL;
   * ```
   */
  StringLiteral = "StringLiteral",
  /**
   * This kind represents a `StringLiterals` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.14 *)
   * StringLiterals = (* item: *) StringLiteral+;
   * ```
   */
  StringLiterals = "StringLiterals",
  /**
   * This kind represents a `StructDefinition` node, with the following structure:
   *
   * ```ebnf
   * StructDefinition = (* struct_keyword: *) STRUCT_KEYWORD
   * (* name: *) IDENTIFIER
   * (* open_brace: *) OPEN_BRACE
   * (* members: *) StructMembers
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  StructDefinition = "StructDefinition",
  /**
   * This kind represents a `StructMember` node, with the following structure:
   *
   * ```ebnf
   * StructMember = (* type_name: *) TypeName
   * (* name: *) IDENTIFIER
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  StructMember = "StructMember",
  /**
   * This kind represents a `StructMembers` node, with the following structure:
   *
   * ```ebnf
   * StructMembers = (* item: *) StructMember*;
   * ```
   */
  StructMembers = "StructMembers",
  /**
   * This kind represents a `ThrowStatement` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * ThrowStatement = (* throw_keyword: *) THROW_KEYWORD
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  ThrowStatement = "ThrowStatement",
  /**
   * This kind represents a `TryStatement` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * TryStatement = (* try_keyword: *) TRY_KEYWORD
   * (* expression: *) Expression
   * (* returns: *) ReturnsDeclaration?
   * (* body: *) Block
   * (* catch_clauses: *) CatchClauses;
   * ```
   */
  TryStatement = "TryStatement",
  /**
   * This kind represents a `TupleDeconstructionElement` node, with the following structure:
   *
   * ```ebnf
   * TupleDeconstructionElement = (* member: *) TupleMember?;
   * ```
   */
  TupleDeconstructionElement = "TupleDeconstructionElement",
  /**
   * This kind represents a `TupleDeconstructionElements` node, with the following structure:
   *
   * ```ebnf
   * TupleDeconstructionElements = (* item: *) TupleDeconstructionElement ((* separator: *) COMMA (* item: *) TupleDeconstructionElement)*;
   * ```
   */
  TupleDeconstructionElements = "TupleDeconstructionElements",
  /**
   * This kind represents a `TupleDeconstructionStatement` node, with the following structure:
   *
   * ```ebnf
   * TupleDeconstructionStatement = (* var_keyword: *) VAR_KEYWORD? (* Deprecated in 0.5.0 *)
   * (* open_paren: *) OPEN_PAREN
   * (* elements: *) TupleDeconstructionElements
   * (* close_paren: *) CLOSE_PAREN
   * (* equal: *) EQUAL
   * (* expression: *) Expression
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  TupleDeconstructionStatement = "TupleDeconstructionStatement",
  /**
   * This kind represents a `TupleExpression` node, with the following structure:
   *
   * ```ebnf
   * TupleExpression = (* open_paren: *) OPEN_PAREN
   * (* items: *) TupleValues
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  TupleExpression = "TupleExpression",
  /**
   * This kind represents a `TupleMember` node, with the following structure:
   *
   * ```ebnf
   * TupleMember = (* variant: *) TypedTupleMember
   * | (* variant: *) UntypedTupleMember;
   * ```
   */
  TupleMember = "TupleMember",
  /**
   * This kind represents a `TupleValue` node, with the following structure:
   *
   * ```ebnf
   * TupleValue = (* expression: *) Expression?;
   * ```
   */
  TupleValue = "TupleValue",
  /**
   * This kind represents a `TupleValues` node, with the following structure:
   *
   * ```ebnf
   * TupleValues = (* item: *) TupleValue ((* separator: *) COMMA (* item: *) TupleValue)*;
   * ```
   */
  TupleValues = "TupleValues",
  /**
   * This kind represents a `TypeExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.3 *)
   * TypeExpression = (* type_keyword: *) TYPE_KEYWORD
   * (* open_paren: *) OPEN_PAREN
   * (* type_name: *) TypeName
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  TypeExpression = "TypeExpression",
  /**
   * This kind represents a `TypeName` node, with the following structure:
   *
   * ```ebnf
   * TypeName = (* variant: *) ArrayTypeName
   * | (* variant: *) FunctionType
   * | (* variant: *) MappingType
   * | (* variant: *) ElementaryType
   * | (* variant: *) IdentifierPath;
   * ```
   */
  TypeName = "TypeName",
  /**
   * This kind represents a `TypedTupleMember` node, with the following structure:
   *
   * ```ebnf
   * TypedTupleMember = (* type_name: *) TypeName
   * (* storage_location: *) StorageLocation?
   * (* name: *) IDENTIFIER;
   * ```
   */
  TypedTupleMember = "TypedTupleMember",
  /**
   * This kind represents a `UncheckedBlock` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.0 *)
   * UncheckedBlock = (* unchecked_keyword: *) UNCHECKED_KEYWORD
   * (* block: *) Block;
   * ```
   */
  UncheckedBlock = "UncheckedBlock",
  /**
   * This kind represents a `UnicodeStringLiteral` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.0 *)
   * UnicodeStringLiteral = (* variant: *) SINGLE_QUOTED_UNICODE_STRING_LITERAL
   * | (* variant: *) DOUBLE_QUOTED_UNICODE_STRING_LITERAL;
   * ```
   */
  UnicodeStringLiteral = "UnicodeStringLiteral",
  /**
   * This kind represents a `UnicodeStringLiterals` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.0 *)
   * UnicodeStringLiterals = (* item: *) UnicodeStringLiteral+;
   * ```
   */
  UnicodeStringLiterals = "UnicodeStringLiterals",
  /**
   * This kind represents a `UnnamedFunctionAttribute` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.6.0 *)
   * UnnamedFunctionAttribute = (* variant: *) ModifierInvocation
   * | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) EXTERNAL_KEYWORD
   * | (* variant: *) INTERNAL_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) PAYABLE_KEYWORD
   * | (* variant: *) PRIVATE_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) PUBLIC_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 and deprecated in 0.6.0. *)
   * | (* variant: *) VIEW_KEYWORD; (* Introduced in 0.4.16 and deprecated in 0.6.0. *)
   * ```
   */
  UnnamedFunctionAttribute = "UnnamedFunctionAttribute",
  /**
   * This kind represents a `UnnamedFunctionAttributes` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.6.0 *)
   * UnnamedFunctionAttributes = (* item: *) UnnamedFunctionAttribute*;
   * ```
   */
  UnnamedFunctionAttributes = "UnnamedFunctionAttributes",
  /**
   * This kind represents a `UnnamedFunctionDefinition` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.6.0 *)
   * UnnamedFunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
   * (* parameters: *) ParametersDeclaration
   * (* attributes: *) UnnamedFunctionAttributes
   * (* body: *) FunctionBody;
   * ```
   */
  UnnamedFunctionDefinition = "UnnamedFunctionDefinition",
  /**
   * This kind represents a `UntypedTupleMember` node, with the following structure:
   *
   * ```ebnf
   * UntypedTupleMember = (* storage_location: *) StorageLocation?
   * (* name: *) IDENTIFIER;
   * ```
   */
  UntypedTupleMember = "UntypedTupleMember",
  /**
   * This kind represents a `UserDefinedValueTypeDefinition` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.8 *)
   * UserDefinedValueTypeDefinition = (* type_keyword: *) TYPE_KEYWORD
   * (* name: *) IDENTIFIER
   * (* is_keyword: *) IS_KEYWORD
   * (* value_type: *) ElementaryType
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  UserDefinedValueTypeDefinition = "UserDefinedValueTypeDefinition",
  /**
   * This kind represents a `UsingAlias` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.19 *)
   * UsingAlias = (* as_keyword: *) AS_KEYWORD
   * (* operator: *) UsingOperator;
   * ```
   */
  UsingAlias = "UsingAlias",
  /**
   * This kind represents a `UsingClause` node, with the following structure:
   *
   * ```ebnf
   * UsingClause = (* variant: *) IdentifierPath
   * | (* variant: *) UsingDeconstruction; (* Introduced in 0.8.13 *)
   * ```
   */
  UsingClause = "UsingClause",
  /**
   * This kind represents a `UsingDeconstruction` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * UsingDeconstruction = (* open_brace: *) OPEN_BRACE
   * (* symbols: *) UsingDeconstructionSymbols
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  UsingDeconstruction = "UsingDeconstruction",
  /**
   * This kind represents a `UsingDeconstructionSymbol` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * UsingDeconstructionSymbol = (* name: *) IdentifierPath
   * (* alias: *) UsingAlias?; (* Introduced in 0.8.19 *)
   * ```
   */
  UsingDeconstructionSymbol = "UsingDeconstructionSymbol",
  /**
   * This kind represents a `UsingDeconstructionSymbols` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * UsingDeconstructionSymbols = (* item: *) UsingDeconstructionSymbol ((* separator: *) COMMA (* item: *) UsingDeconstructionSymbol)*;
   * ```
   */
  UsingDeconstructionSymbols = "UsingDeconstructionSymbols",
  /**
   * This kind represents a `UsingDirective` node, with the following structure:
   *
   * ```ebnf
   * UsingDirective = (* using_keyword: *) USING_KEYWORD
   * (* clause: *) UsingClause
   * (* for_keyword: *) FOR_KEYWORD
   * (* target: *) UsingTarget
   * (* global_keyword: *) GLOBAL_KEYWORD? (* Introduced in 0.8.13 *)
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  UsingDirective = "UsingDirective",
  /**
   * This kind represents a `UsingOperator` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.19 *)
   * UsingOperator = (* variant: *) AMPERSAND
   * | (* variant: *) ASTERISK
   * | (* variant: *) BANG_EQUAL
   * | (* variant: *) BAR
   * | (* variant: *) CARET
   * | (* variant: *) EQUAL_EQUAL
   * | (* variant: *) GREATER_THAN
   * | (* variant: *) GREATER_THAN_EQUAL
   * | (* variant: *) LESS_THAN
   * | (* variant: *) LESS_THAN_EQUAL
   * | (* variant: *) MINUS
   * | (* variant: *) PERCENT
   * | (* variant: *) PLUS
   * | (* variant: *) SLASH
   * | (* variant: *) TILDE;
   * ```
   */
  UsingOperator = "UsingOperator",
  /**
   * This kind represents a `UsingTarget` node, with the following structure:
   *
   * ```ebnf
   * UsingTarget = (* variant: *) TypeName
   * | (* variant: *) ASTERISK;
   * ```
   */
  UsingTarget = "UsingTarget",
  /**
   * This kind represents a `VariableDeclarationStatement` node, with the following structure:
   *
   * ```ebnf
   * VariableDeclarationStatement = (* variable_type: *) VariableDeclarationType
   * (* storage_location: *) StorageLocation?
   * (* name: *) IDENTIFIER
   * (* value: *) VariableDeclarationValue?
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  VariableDeclarationStatement = "VariableDeclarationStatement",
  /**
   * This kind represents a `VariableDeclarationType` node, with the following structure:
   *
   * ```ebnf
   * VariableDeclarationType = (* variant: *) TypeName
   * | (* variant: *) VAR_KEYWORD; (* Deprecated in 0.5.0 *)
   * ```
   */
  VariableDeclarationType = "VariableDeclarationType",
  /**
   * This kind represents a `VariableDeclarationValue` node, with the following structure:
   *
   * ```ebnf
   * VariableDeclarationValue = (* equal: *) EQUAL
   * (* expression: *) Expression;
   * ```
   */
  VariableDeclarationValue = "VariableDeclarationValue",
  /**
   * This kind represents a `VersionExpression` node, with the following structure:
   *
   * ```ebnf
   * VersionExpression = (* variant: *) VersionRange
   * | (* variant: *) VersionTerm;
   * ```
   */
  VersionExpression = "VersionExpression",
  /**
   * This kind represents a `VersionExpressionSet` node, with the following structure:
   *
   * ```ebnf
   * VersionExpressionSet = (* item: *) VersionExpression+;
   * ```
   */
  VersionExpressionSet = "VersionExpressionSet",
  /**
   * This kind represents a `VersionExpressionSets` node, with the following structure:
   *
   * ```ebnf
   * VersionExpressionSets = (* item: *) VersionExpressionSet ((* separator: *) BAR_BAR (* item: *) VersionExpressionSet)*;
   * ```
   */
  VersionExpressionSets = "VersionExpressionSets",
  /**
   * This kind represents a `VersionLiteral` node, with the following structure:
   *
   * ```ebnf
   * VersionLiteral = (* variant: *) SimpleVersionLiteral
   * | (* variant: *) SINGLE_QUOTED_VERSION_LITERAL
   * | (* variant: *) DOUBLE_QUOTED_VERSION_LITERAL;
   * ```
   */
  VersionLiteral = "VersionLiteral",
  /**
   * This kind represents a `VersionOperator` node, with the following structure:
   *
   * ```ebnf
   * VersionOperator = (* variant: *) CARET
   * | (* variant: *) TILDE
   * | (* variant: *) EQUAL
   * | (* variant: *) LESS_THAN
   * | (* variant: *) GREATER_THAN
   * | (* variant: *) LESS_THAN_EQUAL
   * | (* variant: *) GREATER_THAN_EQUAL;
   * ```
   */
  VersionOperator = "VersionOperator",
  /**
   * This kind represents a `VersionPragma` node, with the following structure:
   *
   * ```ebnf
   * VersionPragma = (* solidity_keyword: *) SOLIDITY_KEYWORD
   * (* sets: *) VersionExpressionSets;
   * ```
   */
  VersionPragma = "VersionPragma",
  /**
   * This kind represents a `VersionRange` node, with the following structure:
   *
   * ```ebnf
   * VersionRange = (* start: *) VersionLiteral
   * (* minus: *) MINUS
   * (* end: *) VersionLiteral;
   * ```
   */
  VersionRange = "VersionRange",
  /**
   * This kind represents a `VersionTerm` node, with the following structure:
   *
   * ```ebnf
   * VersionTerm = (* operator: *) VersionOperator?
   * (* literal: *) VersionLiteral;
   * ```
   */
  VersionTerm = "VersionTerm",
  /**
   * This kind represents a `WhileStatement` node, with the following structure:
   *
   * ```ebnf
   * WhileStatement = (* while_keyword: *) WHILE_KEYWORD
   * (* open_paren: *) OPEN_PAREN
   * (* condition: *) Expression
   * (* close_paren: *) CLOSE_PAREN
   * (* body: *) Statement;
   * ```
   */
  WhileStatement = "WhileStatement",
  /**
   * This kind represents a `YulArguments` node, with the following structure:
   *
   * ```ebnf
   * YulArguments = ((* item: *) YulExpression ((* separator: *) COMMA (* item: *) YulExpression)*)?;
   * ```
   */
  YulArguments = "YulArguments",
  /**
   * This kind represents a `YulAssignmentOperator` node, with the following structure:
   *
   * ```ebnf
   * YulAssignmentOperator = (* variant: *) COLON_EQUAL
   * | (* variant: *) YulColonAndEqual; (* Deprecated in 0.5.5 *)
   * ```
   */
  YulAssignmentOperator = "YulAssignmentOperator",
  /**
   * This kind represents a `YulBlock` node, with the following structure:
   *
   * ```ebnf
   * YulBlock = (* open_brace: *) OPEN_BRACE
   * (* statements: *) YulStatements
   * (* close_brace: *) CLOSE_BRACE;
   * ```
   */
  YulBlock = "YulBlock",
  /**
   * This kind represents a `YulBreakStatement` node, with the following structure:
   *
   * ```ebnf
   * YulBreakStatement = (* break_keyword: *) YUL_BREAK_KEYWORD;
   * ```
   */
  YulBreakStatement = "YulBreakStatement",
  /**
   * This kind represents a `YulBuiltInFunction` node, with the following structure:
   *
   * ```ebnf
   * YulBuiltInFunction = (* variant: *) YUL_ADD_KEYWORD
   * | (* variant: *) YUL_ADD_MOD_KEYWORD
   * | (* variant: *) YUL_ADDRESS_KEYWORD
   * | (* variant: *) YUL_AND_KEYWORD
   * | (* variant: *) YUL_BALANCE_KEYWORD
   * | (* variant: *) YUL_BLOCK_HASH_KEYWORD
   * | (* variant: *) YUL_BYTE_KEYWORD
   * | (* variant: *) YUL_CALL_CODE_KEYWORD
   * | (* variant: *) YUL_CALL_DATA_COPY_KEYWORD
   * | (* variant: *) YUL_CALL_DATA_LOAD_KEYWORD
   * | (* variant: *) YUL_CALL_DATA_SIZE_KEYWORD
   * | (* variant: *) YUL_CALLER_KEYWORD
   * | (* variant: *) YUL_CALL_KEYWORD
   * | (* variant: *) YUL_CALL_VALUE_KEYWORD
   * | (* variant: *) YUL_COIN_BASE_KEYWORD
   * | (* variant: *) YUL_CREATE_KEYWORD
   * | (* variant: *) YUL_DELEGATE_CALL_KEYWORD
   * | (* variant: *) YUL_DIV_KEYWORD
   * | (* variant: *) YUL_EQ_KEYWORD
   * | (* variant: *) YUL_EXP_KEYWORD
   * | (* variant: *) YUL_EXT_CODE_COPY_KEYWORD
   * | (* variant: *) YUL_EXT_CODE_SIZE_KEYWORD
   * | (* variant: *) YUL_GAS_KEYWORD
   * | (* variant: *) YUL_GAS_LIMIT_KEYWORD
   * | (* variant: *) YUL_GAS_PRICE_KEYWORD
   * | (* variant: *) YUL_GT_KEYWORD
   * | (* variant: *) YUL_INVALID_KEYWORD
   * | (* variant: *) YUL_IS_ZERO_KEYWORD
   * | (* variant: *) YUL_JUMP_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) YUL_JUMPI_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) YUL_LOG_0_KEYWORD
   * | (* variant: *) YUL_LOG_1_KEYWORD
   * | (* variant: *) YUL_LOG_2_KEYWORD
   * | (* variant: *) YUL_LOG_3_KEYWORD
   * | (* variant: *) YUL_LOG_4_KEYWORD
   * | (* variant: *) YUL_LT_KEYWORD
   * | (* variant: *) YUL_M_LOAD_KEYWORD
   * | (* variant: *) YUL_MOD_KEYWORD
   * | (* variant: *) YUL_M_SIZE_KEYWORD
   * | (* variant: *) YUL_M_STORE_8_KEYWORD
   * | (* variant: *) YUL_M_STORE_KEYWORD
   * | (* variant: *) YUL_MUL_KEYWORD
   * | (* variant: *) YUL_MUL_MOD_KEYWORD
   * | (* variant: *) YUL_NOT_KEYWORD
   * | (* variant: *) YUL_NUMBER_KEYWORD
   * | (* variant: *) YUL_ORIGIN_KEYWORD
   * | (* variant: *) YUL_OR_KEYWORD
   * | (* variant: *) YUL_POP_KEYWORD
   * | (* variant: *) YUL_RETURN_KEYWORD
   * | (* variant: *) YUL_REVERT_KEYWORD
   * | (* variant: *) YUL_S_DIV_KEYWORD
   * | (* variant: *) YUL_SELF_DESTRUCT_KEYWORD
   * | (* variant: *) YUL_SGT_KEYWORD
   * | (* variant: *) YUL_SIGN_EXTEND_KEYWORD
   * | (* variant: *) YUL_S_LOAD_KEYWORD
   * | (* variant: *) YUL_SLT_KEYWORD
   * | (* variant: *) YUL_S_MOD_KEYWORD
   * | (* variant: *) YUL_S_STORE_KEYWORD
   * | (* variant: *) YUL_STOP_KEYWORD
   * | (* variant: *) YUL_SUB_KEYWORD
   * | (* variant: *) YUL_TIMESTAMP_KEYWORD
   * | (* variant: *) YUL_XOR_KEYWORD
   * | (* variant: *) YUL_KECCAK_256_KEYWORD (* Introduced in 0.4.12 *)
   * | (* variant: *) YUL_SHA_3_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) YUL_SUICIDE_KEYWORD (* Deprecated in 0.5.0 *)
   * | (* variant: *) YUL_RETURN_DATA_COPY_KEYWORD (* Introduced in 0.4.12 *)
   * | (* variant: *) YUL_RETURN_DATA_SIZE_KEYWORD (* Introduced in 0.4.12 *)
   * | (* variant: *) YUL_STATIC_CALL_KEYWORD (* Introduced in 0.4.12 *)
   * | (* variant: *) YUL_CREATE_2_KEYWORD (* Introduced in 0.4.12 *)
   * | (* variant: *) YUL_EXT_CODE_HASH_KEYWORD (* Introduced in 0.5.0 *)
   * | (* variant: *) YUL_SAR_KEYWORD
   * | (* variant: *) YUL_SHL_KEYWORD
   * | (* variant: *) YUL_SHR_KEYWORD
   * | (* variant: *) YUL_CHAIN_ID_KEYWORD
   * | (* variant: *) YUL_SELF_BALANCE_KEYWORD
   * | (* variant: *) YUL_BASE_FEE_KEYWORD (* Introduced in 0.8.7 *)
   * | (* variant: *) YUL_DIFFICULTY_KEYWORD (* Deprecated in 0.8.18 *)
   * | (* variant: *) YUL_PREV_RANDAO_KEYWORD (* Introduced in 0.8.18 *)
   * | (* variant: *) YUL_BLOB_BASE_FEE_KEYWORD (* Introduced in 0.8.24 *)
   * | (* variant: *) YUL_BLOB_HASH_KEYWORD (* Introduced in 0.8.24 *)
   * | (* variant: *) YUL_T_LOAD_KEYWORD (* Introduced in 0.8.24 *)
   * | (* variant: *) YUL_T_STORE_KEYWORD (* Introduced in 0.8.24 *)
   * | (* variant: *) YUL_M_COPY_KEYWORD; (* Introduced in 0.8.24 *)
   * ```
   */
  YulBuiltInFunction = "YulBuiltInFunction",
  /**
   * This kind represents a `YulColonAndEqual` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.5 *)
   * YulColonAndEqual = (* colon: *) COLON
   * (* equal: *) EQUAL;
   * ```
   */
  YulColonAndEqual = "YulColonAndEqual",
  /**
   * This kind represents a `YulContinueStatement` node, with the following structure:
   *
   * ```ebnf
   * YulContinueStatement = (* continue_keyword: *) YUL_CONTINUE_KEYWORD;
   * ```
   */
  YulContinueStatement = "YulContinueStatement",
  /**
   * This kind represents a `YulDefaultCase` node, with the following structure:
   *
   * ```ebnf
   * YulDefaultCase = (* default_keyword: *) YUL_DEFAULT_KEYWORD
   * (* body: *) YulBlock;
   * ```
   */
  YulDefaultCase = "YulDefaultCase",
  /**
   * This kind represents a `YulEqualAndColon` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YulEqualAndColon = (* equal: *) EQUAL
   * (* colon: *) COLON;
   * ```
   */
  YulEqualAndColon = "YulEqualAndColon",
  /**
   * This kind represents a `YulExpression` node, with the following structure:
   *
   * ```ebnf
   * YulExpression = (* variant: *) YulFunctionCallExpression
   * | (* variant: *) YulLiteral
   * | (* variant: *) YulBuiltInFunction
   * | (* variant: *) YulPath;
   * ```
   */
  YulExpression = "YulExpression",
  /**
   * This kind represents a `YulForStatement` node, with the following structure:
   *
   * ```ebnf
   * YulForStatement = (* for_keyword: *) YUL_FOR_KEYWORD
   * (* initialization: *) YulBlock
   * (* condition: *) YulExpression
   * (* iterator: *) YulBlock
   * (* body: *) YulBlock;
   * ```
   */
  YulForStatement = "YulForStatement",
  /**
   * This kind represents a `YulFunctionCallExpression` node, with the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * YulFunctionCallExpression = (* operand: *) YulExpression
   * (* open_paren: *) OPEN_PAREN
   * (* arguments: *) YulArguments
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  YulFunctionCallExpression = "YulFunctionCallExpression",
  /**
   * This kind represents a `YulFunctionDefinition` node, with the following structure:
   *
   * ```ebnf
   * YulFunctionDefinition = (* function_keyword: *) YUL_FUNCTION_KEYWORD
   * (* name: *) YUL_IDENTIFIER
   * (* parameters: *) YulParametersDeclaration
   * (* returns: *) YulReturnsDeclaration?
   * (* body: *) YulBlock;
   * ```
   */
  YulFunctionDefinition = "YulFunctionDefinition",
  /**
   * This kind represents a `YulIfStatement` node, with the following structure:
   *
   * ```ebnf
   * YulIfStatement = (* if_keyword: *) YUL_IF_KEYWORD
   * (* condition: *) YulExpression
   * (* body: *) YulBlock;
   * ```
   */
  YulIfStatement = "YulIfStatement",
  /**
   * This kind represents a `YulLabel` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YulLabel = (* label: *) YUL_IDENTIFIER
   * (* colon: *) COLON;
   * ```
   */
  YulLabel = "YulLabel",
  /**
   * This kind represents a `YulLeaveStatement` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * YulLeaveStatement = (* leave_keyword: *) YUL_LEAVE_KEYWORD;
   * ```
   */
  YulLeaveStatement = "YulLeaveStatement",
  /**
   * This kind represents a `YulLiteral` node, with the following structure:
   *
   * ```ebnf
   * YulLiteral = (* variant: *) YUL_TRUE_KEYWORD
   * | (* variant: *) YUL_FALSE_KEYWORD
   * | (* variant: *) YUL_DECIMAL_LITERAL
   * | (* variant: *) YUL_HEX_LITERAL
   * | (* variant: *) HexStringLiteral
   * | (* variant: *) StringLiteral;
   * ```
   */
  YulLiteral = "YulLiteral",
  /**
   * This kind represents a `YulParameters` node, with the following structure:
   *
   * ```ebnf
   * YulParameters = ((* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*)?;
   * ```
   */
  YulParameters = "YulParameters",
  /**
   * This kind represents a `YulParametersDeclaration` node, with the following structure:
   *
   * ```ebnf
   * YulParametersDeclaration = (* open_paren: *) OPEN_PAREN
   * (* parameters: *) YulParameters
   * (* close_paren: *) CLOSE_PAREN;
   * ```
   */
  YulParametersDeclaration = "YulParametersDeclaration",
  /**
   * This kind represents a `YulPath` node, with the following structure:
   *
   * ```ebnf
   * YulPath = (* item: *) YUL_IDENTIFIER ((* separator: *) PERIOD (* item: *) YUL_IDENTIFIER)*;
   * ```
   */
  YulPath = "YulPath",
  /**
   * This kind represents a `YulPaths` node, with the following structure:
   *
   * ```ebnf
   * YulPaths = (* item: *) YulPath ((* separator: *) COMMA (* item: *) YulPath)*;
   * ```
   */
  YulPaths = "YulPaths",
  /**
   * This kind represents a `YulReturnsDeclaration` node, with the following structure:
   *
   * ```ebnf
   * YulReturnsDeclaration = (* minus_greater_than: *) MINUS_GREATER_THAN
   * (* variables: *) YulVariableNames;
   * ```
   */
  YulReturnsDeclaration = "YulReturnsDeclaration",
  /**
   * This kind represents a `YulStackAssignmentOperator` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YulStackAssignmentOperator = (* variant: *) EQUAL_COLON
   * | (* variant: *) YulEqualAndColon;
   * ```
   */
  YulStackAssignmentOperator = "YulStackAssignmentOperator",
  /**
   * This kind represents a `YulStackAssignmentStatement` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YulStackAssignmentStatement = (* assignment: *) YulStackAssignmentOperator
   * (* variable: *) YUL_IDENTIFIER;
   * ```
   */
  YulStackAssignmentStatement = "YulStackAssignmentStatement",
  /**
   * This kind represents a `YulStatement` node, with the following structure:
   *
   * ```ebnf
   * YulStatement = (* variant: *) YulBlock
   * | (* variant: *) YulFunctionDefinition
   * | (* variant: *) YulStackAssignmentStatement (* Deprecated in 0.5.0 *)
   * | (* variant: *) YulIfStatement
   * | (* variant: *) YulForStatement
   * | (* variant: *) YulSwitchStatement
   * | (* variant: *) YulLeaveStatement (* Introduced in 0.6.0 *)
   * | (* variant: *) YulBreakStatement
   * | (* variant: *) YulContinueStatement
   * | (* variant: *) YulLabel (* Deprecated in 0.5.0 *)
   * | (* variant: *) YulVariableDeclarationStatement
   * | (* variant: *) YulVariableAssignmentStatement
   * | (* variant: *) YulExpression;
   * ```
   */
  YulStatement = "YulStatement",
  /**
   * This kind represents a `YulStatements` node, with the following structure:
   *
   * ```ebnf
   * YulStatements = (* item: *) YulStatement*;
   * ```
   */
  YulStatements = "YulStatements",
  /**
   * This kind represents a `YulSwitchCase` node, with the following structure:
   *
   * ```ebnf
   * YulSwitchCase = (* variant: *) YulDefaultCase
   * | (* variant: *) YulValueCase;
   * ```
   */
  YulSwitchCase = "YulSwitchCase",
  /**
   * This kind represents a `YulSwitchCases` node, with the following structure:
   *
   * ```ebnf
   * YulSwitchCases = (* item: *) YulSwitchCase+;
   * ```
   */
  YulSwitchCases = "YulSwitchCases",
  /**
   * This kind represents a `YulSwitchStatement` node, with the following structure:
   *
   * ```ebnf
   * YulSwitchStatement = (* switch_keyword: *) YUL_SWITCH_KEYWORD
   * (* expression: *) YulExpression
   * (* cases: *) YulSwitchCases;
   * ```
   */
  YulSwitchStatement = "YulSwitchStatement",
  /**
   * This kind represents a `YulValueCase` node, with the following structure:
   *
   * ```ebnf
   * YulValueCase = (* case_keyword: *) YUL_CASE_KEYWORD
   * (* value: *) YulLiteral
   * (* body: *) YulBlock;
   * ```
   */
  YulValueCase = "YulValueCase",
  /**
   * This kind represents a `YulVariableAssignmentStatement` node, with the following structure:
   *
   * ```ebnf
   * YulVariableAssignmentStatement = (* variables: *) YulPaths
   * (* assignment: *) YulAssignmentOperator
   * (* expression: *) YulExpression;
   * ```
   */
  YulVariableAssignmentStatement = "YulVariableAssignmentStatement",
  /**
   * This kind represents a `YulVariableDeclarationStatement` node, with the following structure:
   *
   * ```ebnf
   * YulVariableDeclarationStatement = (* let_keyword: *) YUL_LET_KEYWORD
   * (* variables: *) YulVariableNames
   * (* value: *) YulVariableDeclarationValue?;
   * ```
   */
  YulVariableDeclarationStatement = "YulVariableDeclarationStatement",
  /**
   * This kind represents a `YulVariableDeclarationValue` node, with the following structure:
   *
   * ```ebnf
   * YulVariableDeclarationValue = (* assignment: *) YulAssignmentOperator
   * (* expression: *) YulExpression;
   * ```
   */
  YulVariableDeclarationValue = "YulVariableDeclarationValue",
  /**
   * This kind represents a `YulVariableNames` node, with the following structure:
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
   * This terminal is created when the parser is expecting a certain terminal, but it cannot find it.
   * Adding the missing input in this position may allow the parser to produce a valid tree there.
   */
  Missing = "Missing",
  /**
   * This kind represents a `AbicoderKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Never reserved *)
   * ABICODER_KEYWORD = "abicoder";
   * ```
   */
  AbicoderKeyword = "AbicoderKeyword",
  /**
   * This kind represents a `AbstractKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * ABSTRACT_KEYWORD = "abstract";
   * ```
   */
  AbstractKeyword = "AbstractKeyword",
  /**
   * This kind represents a `AddressKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Never reserved *)
   * ADDRESS_KEYWORD = "address";
   * ```
   */
  AddressKeyword = "AddressKeyword",
  /**
   * This kind represents a `AfterKeyword` node, with the following structure:
   *
   * ```ebnf
   * AFTER_KEYWORD = "after";
   * ```
   */
  AfterKeyword = "AfterKeyword",
  /**
   * This kind represents a `AliasKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * ALIAS_KEYWORD = "alias";
   * ```
   */
  AliasKeyword = "AliasKeyword",
  /**
   * This kind represents a `Ampersand` node, with the following structure:
   *
   * ```ebnf
   * AMPERSAND = "&";
   * ```
   */
  Ampersand = "Ampersand",
  /**
   * This kind represents a `AmpersandAmpersand` node, with the following structure:
   *
   * ```ebnf
   * AMPERSAND_AMPERSAND = "&&";
   * ```
   */
  AmpersandAmpersand = "AmpersandAmpersand",
  /**
   * This kind represents a `AmpersandEqual` node, with the following structure:
   *
   * ```ebnf
   * AMPERSAND_EQUAL = "&=";
   * ```
   */
  AmpersandEqual = "AmpersandEqual",
  /**
   * This kind represents a `AnonymousKeyword` node, with the following structure:
   *
   * ```ebnf
   * ANONYMOUS_KEYWORD = "anonymous";
   * ```
   */
  AnonymousKeyword = "AnonymousKeyword",
  /**
   * This kind represents a `ApplyKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * APPLY_KEYWORD = "apply";
   * ```
   */
  ApplyKeyword = "ApplyKeyword",
  /**
   * This kind represents a `AsKeyword` node, with the following structure:
   *
   * ```ebnf
   * AS_KEYWORD = "as";
   * ```
   */
  AsKeyword = "AsKeyword",
  /**
   * This kind represents a `AssemblyKeyword` node, with the following structure:
   *
   * ```ebnf
   * ASSEMBLY_KEYWORD = "assembly";
   * ```
   */
  AssemblyKeyword = "AssemblyKeyword",
  /**
   * This kind represents a `Asterisk` node, with the following structure:
   *
   * ```ebnf
   * ASTERISK = "*";
   * ```
   */
  Asterisk = "Asterisk",
  /**
   * This kind represents a `AsteriskAsterisk` node, with the following structure:
   *
   * ```ebnf
   * ASTERISK_ASTERISK = "**";
   * ```
   */
  AsteriskAsterisk = "AsteriskAsterisk",
  /**
   * This kind represents a `AsteriskEqual` node, with the following structure:
   *
   * ```ebnf
   * ASTERISK_EQUAL = "*=";
   * ```
   */
  AsteriskEqual = "AsteriskEqual",
  /**
   * This kind represents a `AutoKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * AUTO_KEYWORD = "auto";
   * ```
   */
  AutoKeyword = "AutoKeyword",
  /**
   * This kind represents a `Bang` node, with the following structure:
   *
   * ```ebnf
   * BANG = "!";
   * ```
   */
  Bang = "Bang",
  /**
   * This kind represents a `BangEqual` node, with the following structure:
   *
   * ```ebnf
   * BANG_EQUAL = "!=";
   * ```
   */
  BangEqual = "BangEqual",
  /**
   * This kind represents a `Bar` node, with the following structure:
   *
   * ```ebnf
   * BAR = "|";
   * ```
   */
  Bar = "Bar",
  /**
   * This kind represents a `BarBar` node, with the following structure:
   *
   * ```ebnf
   * BAR_BAR = "||";
   * ```
   */
  BarBar = "BarBar",
  /**
   * This kind represents a `BarEqual` node, with the following structure:
   *
   * ```ebnf
   * BAR_EQUAL = "|=";
   * ```
   */
  BarEqual = "BarEqual",
  /**
   * This kind represents a `BoolKeyword` node, with the following structure:
   *
   * ```ebnf
   * BOOL_KEYWORD = "bool";
   * ```
   */
  BoolKeyword = "BoolKeyword",
  /**
   * This kind represents a `BreakKeyword` node, with the following structure:
   *
   * ```ebnf
   * BREAK_KEYWORD = "break";
   * ```
   */
  BreakKeyword = "BreakKeyword",
  /**
   * This kind represents a `ByteKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.8.0 *)
   * BYTE_KEYWORD = "byte";
   * ```
   */
  ByteKeyword = "ByteKeyword",
  /**
   * This kind represents a `BytesKeyword` node, with the following structure:
   *
   * ```ebnf
   * BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
   * ```
   */
  BytesKeyword = "BytesKeyword",
  /**
   * This kind represents a `CallDataKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.0 *)
   * (* Reserved in 0.5.0 *)
   * CALL_DATA_KEYWORD = "calldata";
   * ```
   */
  CallDataKeyword = "CallDataKeyword",
  /**
   * This kind represents a `Caret` node, with the following structure:
   *
   * ```ebnf
   * CARET = "^";
   * ```
   */
  Caret = "Caret",
  /**
   * This kind represents a `CaretEqual` node, with the following structure:
   *
   * ```ebnf
   * CARET_EQUAL = "^=";
   * ```
   */
  CaretEqual = "CaretEqual",
  /**
   * This kind represents a `CaseKeyword` node, with the following structure:
   *
   * ```ebnf
   * CASE_KEYWORD = "case";
   * ```
   */
  CaseKeyword = "CaseKeyword",
  /**
   * This kind represents a `CatchKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * CATCH_KEYWORD = "catch";
   * ```
   */
  CatchKeyword = "CatchKeyword",
  /**
   * This kind represents a `CloseBrace` node, with the following structure:
   *
   * ```ebnf
   * CLOSE_BRACE = "}";
   * ```
   */
  CloseBrace = "CloseBrace",
  /**
   * This kind represents a `CloseBracket` node, with the following structure:
   *
   * ```ebnf
   * CLOSE_BRACKET = "]";
   * ```
   */
  CloseBracket = "CloseBracket",
  /**
   * This kind represents a `CloseParen` node, with the following structure:
   *
   * ```ebnf
   * CLOSE_PAREN = ")";
   * ```
   */
  CloseParen = "CloseParen",
  /**
   * This kind represents a `Colon` node, with the following structure:
   *
   * ```ebnf
   * COLON = ":";
   * ```
   */
  Colon = "Colon",
  /**
   * This kind represents a `ColonEqual` node, with the following structure:
   *
   * ```ebnf
   * COLON_EQUAL = ":=";
   * ```
   */
  ColonEqual = "ColonEqual",
  /**
   * This kind represents a `Comma` node, with the following structure:
   *
   * ```ebnf
   * COMMA = ",";
   * ```
   */
  Comma = "Comma",
  /**
   * This kind represents a `ConstantKeyword` node, with the following structure:
   *
   * ```ebnf
   * CONSTANT_KEYWORD = "constant";
   * ```
   */
  ConstantKeyword = "ConstantKeyword",
  /**
   * This kind represents a `ConstructorKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.22 *)
   * (* Reserved in 0.5.0 *)
   * CONSTRUCTOR_KEYWORD = "constructor";
   * ```
   */
  ConstructorKeyword = "ConstructorKeyword",
  /**
   * This kind represents a `ContinueKeyword` node, with the following structure:
   *
   * ```ebnf
   * CONTINUE_KEYWORD = "continue";
   * ```
   */
  ContinueKeyword = "ContinueKeyword",
  /**
   * This kind represents a `ContractKeyword` node, with the following structure:
   *
   * ```ebnf
   * CONTRACT_KEYWORD = "contract";
   * ```
   */
  ContractKeyword = "ContractKeyword",
  /**
   * This kind represents a `CopyOfKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * COPY_OF_KEYWORD = "copyof";
   * ```
   */
  CopyOfKeyword = "CopyOfKeyword",
  /**
   * This kind represents a `DaysKeyword` node, with the following structure:
   *
   * ```ebnf
   * DAYS_KEYWORD = "days";
   * ```
   */
  DaysKeyword = "DaysKeyword",
  /**
   * This kind represents a `DecimalLiteral` node, with the following structure:
   *
   * ```ebnf
   * DECIMAL_LITERAL = "." «DECIMAL_DIGITS» «DECIMAL_EXPONENT»?;
   *
   * DECIMAL_LITERAL = «DECIMAL_DIGITS» «DECIMAL_EXPONENT»?;
   *
   * (* Deprecated in 0.5.0 *)
   * DECIMAL_LITERAL = «DECIMAL_DIGITS» "." «DECIMAL_EXPONENT»?;
   *
   * (* Deprecated in 0.5.0 *)
   * DECIMAL_LITERAL = «DECIMAL_DIGITS» "." «DECIMAL_DIGITS» «DECIMAL_EXPONENT»?;
   *
   * (* Introduced in 0.5.0 *)
   * DECIMAL_LITERAL = «DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»)? «DECIMAL_EXPONENT»?;
   * ```
   */
  DecimalLiteral = "DecimalLiteral",
  /**
   * This kind represents a `DefaultKeyword` node, with the following structure:
   *
   * ```ebnf
   * DEFAULT_KEYWORD = "default";
   * ```
   */
  DefaultKeyword = "DefaultKeyword",
  /**
   * This kind represents a `DefineKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * DEFINE_KEYWORD = "define";
   * ```
   */
  DefineKeyword = "DefineKeyword",
  /**
   * This kind represents a `DeleteKeyword` node, with the following structure:
   *
   * ```ebnf
   * DELETE_KEYWORD = "delete";
   * ```
   */
  DeleteKeyword = "DeleteKeyword",
  /**
   * This kind represents a `DoKeyword` node, with the following structure:
   *
   * ```ebnf
   * DO_KEYWORD = "do";
   * ```
   */
  DoKeyword = "DoKeyword",
  /**
   * This kind represents a `DoubleQuotedHexStringLiteral` node, with the following structure:
   *
   * ```ebnf
   * DOUBLE_QUOTED_HEX_STRING_LITERAL = 'hex"' «HEX_STRING_CONTENTS»? '"';
   * ```
   */
  DoubleQuotedHexStringLiteral = "DoubleQuotedHexStringLiteral",
  /**
   * This kind represents a `DoubleQuotedStringLiteral` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.4.25 *)
   * DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE_ARBITRARY» | !('"' "\\" "\r" "\n"))* '"';
   *
   * (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
   * DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | !('"' "\\" "\r" "\n"))* '"';
   *
   * DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | (" "…"!") | ("#"…"[") | ("]"…"~"))* '"';
   * ```
   */
  DoubleQuotedStringLiteral = "DoubleQuotedStringLiteral",
  /**
   * This kind represents a `DoubleQuotedUnicodeStringLiteral` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.0 *)
   * DOUBLE_QUOTED_UNICODE_STRING_LITERAL = 'unicode"' («ESCAPE_SEQUENCE» | !('"' "\\" "\r" "\n"))* '"';
   * ```
   */
  DoubleQuotedUnicodeStringLiteral = "DoubleQuotedUnicodeStringLiteral",
  /**
   * This kind represents a `DoubleQuotedVersionLiteral` node, with the following structure:
   *
   * ```ebnf
   * DOUBLE_QUOTED_VERSION_LITERAL = '"' «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* '"';
   * ```
   */
  DoubleQuotedVersionLiteral = "DoubleQuotedVersionLiteral",
  /**
   * This kind represents a `ElseKeyword` node, with the following structure:
   *
   * ```ebnf
   * ELSE_KEYWORD = "else";
   * ```
   */
  ElseKeyword = "ElseKeyword",
  /**
   * This kind represents a `EmitKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.21 *)
   * (* Reserved in 0.5.0 *)
   * EMIT_KEYWORD = "emit";
   * ```
   */
  EmitKeyword = "EmitKeyword",
  /**
   * This kind represents a `EndOfLine` node, with the following structure:
   *
   * ```ebnf
   * END_OF_LINE = "\n" | ("\r" "\n"?);
   * ```
   */
  EndOfLine = "EndOfLine",
  /**
   * This kind represents a `EnumKeyword` node, with the following structure:
   *
   * ```ebnf
   * ENUM_KEYWORD = "enum";
   * ```
   */
  EnumKeyword = "EnumKeyword",
  /**
   * This kind represents a `Equal` node, with the following structure:
   *
   * ```ebnf
   * EQUAL = "=";
   * ```
   */
  Equal = "Equal",
  /**
   * This kind represents a `EqualColon` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * EQUAL_COLON = "=:";
   * ```
   */
  EqualColon = "EqualColon",
  /**
   * This kind represents a `EqualEqual` node, with the following structure:
   *
   * ```ebnf
   * EQUAL_EQUAL = "==";
   * ```
   */
  EqualEqual = "EqualEqual",
  /**
   * This kind represents a `EqualGreaterThan` node, with the following structure:
   *
   * ```ebnf
   * EQUAL_GREATER_THAN = "=>";
   * ```
   */
  EqualGreaterThan = "EqualGreaterThan",
  /**
   * This kind represents a `ErrorKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * (* Never reserved *)
   * ERROR_KEYWORD = "error";
   * ```
   */
  ErrorKeyword = "ErrorKeyword",
  /**
   * This kind represents a `EtherKeyword` node, with the following structure:
   *
   * ```ebnf
   * ETHER_KEYWORD = "ether";
   * ```
   */
  EtherKeyword = "EtherKeyword",
  /**
   * This kind represents a `EventKeyword` node, with the following structure:
   *
   * ```ebnf
   * EVENT_KEYWORD = "event";
   * ```
   */
  EventKeyword = "EventKeyword",
  /**
   * This kind represents a `ExperimentalKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Never reserved *)
   * EXPERIMENTAL_KEYWORD = "experimental";
   * ```
   */
  ExperimentalKeyword = "ExperimentalKeyword",
  /**
   * This kind represents a `ExternalKeyword` node, with the following structure:
   *
   * ```ebnf
   * EXTERNAL_KEYWORD = "external";
   * ```
   */
  ExternalKeyword = "ExternalKeyword",
  /**
   * This kind represents a `FallbackKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.6.0 *)
   * FALLBACK_KEYWORD = "fallback";
   * ```
   */
  FallbackKeyword = "FallbackKeyword",
  /**
   * This kind represents a `FalseKeyword` node, with the following structure:
   *
   * ```ebnf
   * FALSE_KEYWORD = "false";
   * ```
   */
  FalseKeyword = "FalseKeyword",
  /**
   * This kind represents a `FinalKeyword` node, with the following structure:
   *
   * ```ebnf
   * FINAL_KEYWORD = "final";
   * ```
   */
  FinalKeyword = "FinalKeyword",
  /**
   * This kind represents a `FinneyKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.7.0 *)
   * (* Reserved until 0.7.0 *)
   * FINNEY_KEYWORD = "finney";
   * ```
   */
  FinneyKeyword = "FinneyKeyword",
  /**
   * This kind represents a `FixedKeyword` node, with the following structure:
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
   * This kind represents a `ForKeyword` node, with the following structure:
   *
   * ```ebnf
   * FOR_KEYWORD = "for";
   * ```
   */
  ForKeyword = "ForKeyword",
  /**
   * This kind represents a `FromKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Never reserved *)
   * FROM_KEYWORD = "from";
   * ```
   */
  FromKeyword = "FromKeyword",
  /**
   * This kind represents a `FunctionKeyword` node, with the following structure:
   *
   * ```ebnf
   * FUNCTION_KEYWORD = "function";
   * ```
   */
  FunctionKeyword = "FunctionKeyword",
  /**
   * This kind represents a `GlobalKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.13 *)
   * (* Never reserved *)
   * GLOBAL_KEYWORD = "global";
   * ```
   */
  GlobalKeyword = "GlobalKeyword",
  /**
   * This kind represents a `GreaterThan` node, with the following structure:
   *
   * ```ebnf
   * GREATER_THAN = ">";
   * ```
   */
  GreaterThan = "GreaterThan",
  /**
   * This kind represents a `GreaterThanEqual` node, with the following structure:
   *
   * ```ebnf
   * GREATER_THAN_EQUAL = ">=";
   * ```
   */
  GreaterThanEqual = "GreaterThanEqual",
  /**
   * This kind represents a `GreaterThanGreaterThan` node, with the following structure:
   *
   * ```ebnf
   * GREATER_THAN_GREATER_THAN = ">>";
   * ```
   */
  GreaterThanGreaterThan = "GreaterThanGreaterThan",
  /**
   * This kind represents a `GreaterThanGreaterThanEqual` node, with the following structure:
   *
   * ```ebnf
   * GREATER_THAN_GREATER_THAN_EQUAL = ">>=";
   * ```
   */
  GreaterThanGreaterThanEqual = "GreaterThanGreaterThanEqual",
  /**
   * This kind represents a `GreaterThanGreaterThanGreaterThan` node, with the following structure:
   *
   * ```ebnf
   * GREATER_THAN_GREATER_THAN_GREATER_THAN = ">>>";
   * ```
   */
  GreaterThanGreaterThanGreaterThan = "GreaterThanGreaterThanGreaterThan",
  /**
   * This kind represents a `GreaterThanGreaterThanGreaterThanEqual` node, with the following structure:
   *
   * ```ebnf
   * GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL = ">>>=";
   * ```
   */
  GreaterThanGreaterThanGreaterThanEqual = "GreaterThanGreaterThanGreaterThanEqual",
  /**
   * This kind represents a `GweiKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.11 *)
   * (* Reserved in 0.7.0 *)
   * GWEI_KEYWORD = "gwei";
   * ```
   */
  GweiKeyword = "GweiKeyword",
  /**
   * This kind represents a `HexKeyword` node, with the following structure:
   *
   * ```ebnf
   * HEX_KEYWORD = "hex";
   * ```
   */
  HexKeyword = "HexKeyword",
  /**
   * This kind represents a `HexLiteral` node, with the following structure:
   *
   * ```ebnf
   * HEX_LITERAL = "0x" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
   *
   * (* Deprecated in 0.5.0 *)
   * HEX_LITERAL = "0X" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
   * ```
   */
  HexLiteral = "HexLiteral",
  /**
   * This kind represents a `HoursKeyword` node, with the following structure:
   *
   * ```ebnf
   * HOURS_KEYWORD = "hours";
   * ```
   */
  HoursKeyword = "HoursKeyword",
  /**
   * This kind represents a `Identifier` node, with the following structure:
   *
   * ```ebnf
   * IDENTIFIER = «IDENTIFIER_START» «IDENTIFIER_PART»*;
   * ```
   */
  Identifier = "Identifier",
  /**
   * This kind represents a `IfKeyword` node, with the following structure:
   *
   * ```ebnf
   * IF_KEYWORD = "if";
   * ```
   */
  IfKeyword = "IfKeyword",
  /**
   * This kind represents a `ImmutableKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.5 *)
   * (* Reserved in 0.5.0 *)
   * IMMUTABLE_KEYWORD = "immutable";
   * ```
   */
  ImmutableKeyword = "ImmutableKeyword",
  /**
   * This kind represents a `ImplementsKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * IMPLEMENTS_KEYWORD = "implements";
   * ```
   */
  ImplementsKeyword = "ImplementsKeyword",
  /**
   * This kind represents a `ImportKeyword` node, with the following structure:
   *
   * ```ebnf
   * IMPORT_KEYWORD = "import";
   * ```
   */
  ImportKeyword = "ImportKeyword",
  /**
   * This kind represents a `InKeyword` node, with the following structure:
   *
   * ```ebnf
   * IN_KEYWORD = "in";
   * ```
   */
  InKeyword = "InKeyword",
  /**
   * This kind represents a `IndexedKeyword` node, with the following structure:
   *
   * ```ebnf
   * INDEXED_KEYWORD = "indexed";
   * ```
   */
  IndexedKeyword = "IndexedKeyword",
  /**
   * This kind represents a `InlineKeyword` node, with the following structure:
   *
   * ```ebnf
   * INLINE_KEYWORD = "inline";
   * ```
   */
  InlineKeyword = "InlineKeyword",
  /**
   * This kind represents a `IntKeyword` node, with the following structure:
   *
   * ```ebnf
   * INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
   * ```
   */
  IntKeyword = "IntKeyword",
  /**
   * This kind represents a `InterfaceKeyword` node, with the following structure:
   *
   * ```ebnf
   * INTERFACE_KEYWORD = "interface";
   * ```
   */
  InterfaceKeyword = "InterfaceKeyword",
  /**
   * This kind represents a `InternalKeyword` node, with the following structure:
   *
   * ```ebnf
   * INTERNAL_KEYWORD = "internal";
   * ```
   */
  InternalKeyword = "InternalKeyword",
  /**
   * This kind represents a `IsKeyword` node, with the following structure:
   *
   * ```ebnf
   * IS_KEYWORD = "is";
   * ```
   */
  IsKeyword = "IsKeyword",
  /**
   * This kind represents a `LessThan` node, with the following structure:
   *
   * ```ebnf
   * LESS_THAN = "<";
   * ```
   */
  LessThan = "LessThan",
  /**
   * This kind represents a `LessThanEqual` node, with the following structure:
   *
   * ```ebnf
   * LESS_THAN_EQUAL = "<=";
   * ```
   */
  LessThanEqual = "LessThanEqual",
  /**
   * This kind represents a `LessThanLessThan` node, with the following structure:
   *
   * ```ebnf
   * LESS_THAN_LESS_THAN = "<<";
   * ```
   */
  LessThanLessThan = "LessThanLessThan",
  /**
   * This kind represents a `LessThanLessThanEqual` node, with the following structure:
   *
   * ```ebnf
   * LESS_THAN_LESS_THAN_EQUAL = "<<=";
   * ```
   */
  LessThanLessThanEqual = "LessThanLessThanEqual",
  /**
   * This kind represents a `LetKeyword` node, with the following structure:
   *
   * ```ebnf
   * LET_KEYWORD = "let";
   * ```
   */
  LetKeyword = "LetKeyword",
  /**
   * This kind represents a `LibraryKeyword` node, with the following structure:
   *
   * ```ebnf
   * LIBRARY_KEYWORD = "library";
   * ```
   */
  LibraryKeyword = "LibraryKeyword",
  /**
   * This kind represents a `MacroKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * MACRO_KEYWORD = "macro";
   * ```
   */
  MacroKeyword = "MacroKeyword",
  /**
   * This kind represents a `MappingKeyword` node, with the following structure:
   *
   * ```ebnf
   * MAPPING_KEYWORD = "mapping";
   * ```
   */
  MappingKeyword = "MappingKeyword",
  /**
   * This kind represents a `MatchKeyword` node, with the following structure:
   *
   * ```ebnf
   * MATCH_KEYWORD = "match";
   * ```
   */
  MatchKeyword = "MatchKeyword",
  /**
   * This kind represents a `MemoryKeyword` node, with the following structure:
   *
   * ```ebnf
   * MEMORY_KEYWORD = "memory";
   * ```
   */
  MemoryKeyword = "MemoryKeyword",
  /**
   * This kind represents a `Minus` node, with the following structure:
   *
   * ```ebnf
   * MINUS = "-";
   * ```
   */
  Minus = "Minus",
  /**
   * This kind represents a `MinusEqual` node, with the following structure:
   *
   * ```ebnf
   * MINUS_EQUAL = "-=";
   * ```
   */
  MinusEqual = "MinusEqual",
  /**
   * This kind represents a `MinusGreaterThan` node, with the following structure:
   *
   * ```ebnf
   * MINUS_GREATER_THAN = "->";
   * ```
   */
  MinusGreaterThan = "MinusGreaterThan",
  /**
   * This kind represents a `MinusMinus` node, with the following structure:
   *
   * ```ebnf
   * MINUS_MINUS = "--";
   * ```
   */
  MinusMinus = "MinusMinus",
  /**
   * This kind represents a `MinutesKeyword` node, with the following structure:
   *
   * ```ebnf
   * MINUTES_KEYWORD = "minutes";
   * ```
   */
  MinutesKeyword = "MinutesKeyword",
  /**
   * This kind represents a `ModifierKeyword` node, with the following structure:
   *
   * ```ebnf
   * MODIFIER_KEYWORD = "modifier";
   * ```
   */
  ModifierKeyword = "ModifierKeyword",
  /**
   * This kind represents a `MultiLineComment` node, with the following structure:
   *
   * ```ebnf
   * MULTI_LINE_COMMENT = "/*" (!"*" | "*")* "*\/";
   * ```
   */
  MultiLineComment = "MultiLineComment",
  /**
   * This kind represents a `MultiLineNatSpecComment` node, with the following structure:
   *
   * ```ebnf
   * MULTI_LINE_NAT_SPEC_COMMENT = "/**" (!"*" | "*")* "*\/";
   * ```
   */
  MultiLineNatSpecComment = "MultiLineNatSpecComment",
  /**
   * This kind represents a `MutableKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * MUTABLE_KEYWORD = "mutable";
   * ```
   */
  MutableKeyword = "MutableKeyword",
  /**
   * This kind represents a `NewKeyword` node, with the following structure:
   *
   * ```ebnf
   * NEW_KEYWORD = "new";
   * ```
   */
  NewKeyword = "NewKeyword",
  /**
   * This kind represents a `NullKeyword` node, with the following structure:
   *
   * ```ebnf
   * NULL_KEYWORD = "null";
   * ```
   */
  NullKeyword = "NullKeyword",
  /**
   * This kind represents a `OfKeyword` node, with the following structure:
   *
   * ```ebnf
   * OF_KEYWORD = "of";
   * ```
   */
  OfKeyword = "OfKeyword",
  /**
   * This kind represents a `OpenBrace` node, with the following structure:
   *
   * ```ebnf
   * OPEN_BRACE = "{";
   * ```
   */
  OpenBrace = "OpenBrace",
  /**
   * This kind represents a `OpenBracket` node, with the following structure:
   *
   * ```ebnf
   * OPEN_BRACKET = "[";
   * ```
   */
  OpenBracket = "OpenBracket",
  /**
   * This kind represents a `OpenParen` node, with the following structure:
   *
   * ```ebnf
   * OPEN_PAREN = "(";
   * ```
   */
  OpenParen = "OpenParen",
  /**
   * This kind represents a `OverrideKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * (* Reserved in 0.5.0 *)
   * OVERRIDE_KEYWORD = "override";
   * ```
   */
  OverrideKeyword = "OverrideKeyword",
  /**
   * This kind represents a `PartialKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * PARTIAL_KEYWORD = "partial";
   * ```
   */
  PartialKeyword = "PartialKeyword",
  /**
   * This kind represents a `PayableKeyword` node, with the following structure:
   *
   * ```ebnf
   * PAYABLE_KEYWORD = "payable";
   * ```
   */
  PayableKeyword = "PayableKeyword",
  /**
   * This kind represents a `Percent` node, with the following structure:
   *
   * ```ebnf
   * PERCENT = "%";
   * ```
   */
  Percent = "Percent",
  /**
   * This kind represents a `PercentEqual` node, with the following structure:
   *
   * ```ebnf
   * PERCENT_EQUAL = "%=";
   * ```
   */
  PercentEqual = "PercentEqual",
  /**
   * This kind represents a `Period` node, with the following structure:
   *
   * ```ebnf
   * PERIOD = ".";
   * ```
   */
  Period = "Period",
  /**
   * This kind represents a `Plus` node, with the following structure:
   *
   * ```ebnf
   * PLUS = "+";
   * ```
   */
  Plus = "Plus",
  /**
   * This kind represents a `PlusEqual` node, with the following structure:
   *
   * ```ebnf
   * PLUS_EQUAL = "+=";
   * ```
   */
  PlusEqual = "PlusEqual",
  /**
   * This kind represents a `PlusPlus` node, with the following structure:
   *
   * ```ebnf
   * PLUS_PLUS = "++";
   * ```
   */
  PlusPlus = "PlusPlus",
  /**
   * This kind represents a `PragmaKeyword` node, with the following structure:
   *
   * ```ebnf
   * PRAGMA_KEYWORD = "pragma";
   * ```
   */
  PragmaKeyword = "PragmaKeyword",
  /**
   * This kind represents a `PrivateKeyword` node, with the following structure:
   *
   * ```ebnf
   * PRIVATE_KEYWORD = "private";
   * ```
   */
  PrivateKeyword = "PrivateKeyword",
  /**
   * This kind represents a `PromiseKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * PROMISE_KEYWORD = "promise";
   * ```
   */
  PromiseKeyword = "PromiseKeyword",
  /**
   * This kind represents a `PublicKeyword` node, with the following structure:
   *
   * ```ebnf
   * PUBLIC_KEYWORD = "public";
   * ```
   */
  PublicKeyword = "PublicKeyword",
  /**
   * This kind represents a `PureKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.16 *)
   * PURE_KEYWORD = "pure";
   * ```
   */
  PureKeyword = "PureKeyword",
  /**
   * This kind represents a `QuestionMark` node, with the following structure:
   *
   * ```ebnf
   * QUESTION_MARK = "?";
   * ```
   */
  QuestionMark = "QuestionMark",
  /**
   * This kind represents a `ReceiveKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.6.0 *)
   * RECEIVE_KEYWORD = "receive";
   * ```
   */
  ReceiveKeyword = "ReceiveKeyword",
  /**
   * This kind represents a `ReferenceKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * REFERENCE_KEYWORD = "reference";
   * ```
   */
  ReferenceKeyword = "ReferenceKeyword",
  /**
   * This kind represents a `RelocatableKeyword` node, with the following structure:
   *
   * ```ebnf
   * RELOCATABLE_KEYWORD = "relocatable";
   * ```
   */
  RelocatableKeyword = "RelocatableKeyword",
  /**
   * This kind represents a `ReturnKeyword` node, with the following structure:
   *
   * ```ebnf
   * RETURN_KEYWORD = "return";
   * ```
   */
  ReturnKeyword = "ReturnKeyword",
  /**
   * This kind represents a `ReturnsKeyword` node, with the following structure:
   *
   * ```ebnf
   * RETURNS_KEYWORD = "returns";
   * ```
   */
  ReturnsKeyword = "ReturnsKeyword",
  /**
   * This kind represents a `RevertKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.4 *)
   * (* Never reserved *)
   * REVERT_KEYWORD = "revert";
   * ```
   */
  RevertKeyword = "RevertKeyword",
  /**
   * This kind represents a `SealedKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * SEALED_KEYWORD = "sealed";
   * ```
   */
  SealedKeyword = "SealedKeyword",
  /**
   * This kind represents a `SecondsKeyword` node, with the following structure:
   *
   * ```ebnf
   * SECONDS_KEYWORD = "seconds";
   * ```
   */
  SecondsKeyword = "SecondsKeyword",
  /**
   * This kind represents a `Semicolon` node, with the following structure:
   *
   * ```ebnf
   * SEMICOLON = ";";
   * ```
   */
  Semicolon = "Semicolon",
  /**
   * This kind represents a `SingleLineComment` node, with the following structure:
   *
   * ```ebnf
   * SINGLE_LINE_COMMENT = "//" (!("\r" "\n"))*;
   * ```
   */
  SingleLineComment = "SingleLineComment",
  /**
   * This kind represents a `SingleLineNatSpecComment` node, with the following structure:
   *
   * ```ebnf
   * SINGLE_LINE_NAT_SPEC_COMMENT = "///" (!("\r" "\n"))*;
   * ```
   */
  SingleLineNatSpecComment = "SingleLineNatSpecComment",
  /**
   * This kind represents a `SingleQuotedHexStringLiteral` node, with the following structure:
   *
   * ```ebnf
   * SINGLE_QUOTED_HEX_STRING_LITERAL = "hex'" «HEX_STRING_CONTENTS»? "'";
   * ```
   */
  SingleQuotedHexStringLiteral = "SingleQuotedHexStringLiteral",
  /**
   * This kind represents a `SingleQuotedStringLiteral` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.4.25 *)
   * SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE_ARBITRARY» | !("'" "\\" "\r" "\n"))* "'";
   *
   * (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
   * SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | !("'" "\\" "\r" "\n"))* "'";
   *
   * SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | (" "…"&") | ("("…"[") | ("]"…"~"))* "'";
   * ```
   */
  SingleQuotedStringLiteral = "SingleQuotedStringLiteral",
  /**
   * This kind represents a `SingleQuotedUnicodeStringLiteral` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.7.0 *)
   * SINGLE_QUOTED_UNICODE_STRING_LITERAL = "unicode'" («ESCAPE_SEQUENCE» | !("'" "\\" "\r" "\n"))* "'";
   * ```
   */
  SingleQuotedUnicodeStringLiteral = "SingleQuotedUnicodeStringLiteral",
  /**
   * This kind represents a `SingleQuotedVersionLiteral` node, with the following structure:
   *
   * ```ebnf
   * SINGLE_QUOTED_VERSION_LITERAL = "'" «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* "'";
   * ```
   */
  SingleQuotedVersionLiteral = "SingleQuotedVersionLiteral",
  /**
   * This kind represents a `SizeOfKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * SIZE_OF_KEYWORD = "sizeof";
   * ```
   */
  SizeOfKeyword = "SizeOfKeyword",
  /**
   * This kind represents a `Slash` node, with the following structure:
   *
   * ```ebnf
   * SLASH = "/";
   * ```
   */
  Slash = "Slash",
  /**
   * This kind represents a `SlashEqual` node, with the following structure:
   *
   * ```ebnf
   * SLASH_EQUAL = "/=";
   * ```
   */
  SlashEqual = "SlashEqual",
  /**
   * This kind represents a `SolidityKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Never reserved *)
   * SOLIDITY_KEYWORD = "solidity";
   * ```
   */
  SolidityKeyword = "SolidityKeyword",
  /**
   * This kind represents a `StaticKeyword` node, with the following structure:
   *
   * ```ebnf
   * STATIC_KEYWORD = "static";
   * ```
   */
  StaticKeyword = "StaticKeyword",
  /**
   * This kind represents a `StorageKeyword` node, with the following structure:
   *
   * ```ebnf
   * STORAGE_KEYWORD = "storage";
   * ```
   */
  StorageKeyword = "StorageKeyword",
  /**
   * This kind represents a `StringKeyword` node, with the following structure:
   *
   * ```ebnf
   * STRING_KEYWORD = "string";
   * ```
   */
  StringKeyword = "StringKeyword",
  /**
   * This kind represents a `StructKeyword` node, with the following structure:
   *
   * ```ebnf
   * STRUCT_KEYWORD = "struct";
   * ```
   */
  StructKeyword = "StructKeyword",
  /**
   * This kind represents a `SuperKeyword` node, with the following structure:
   *
   * ```ebnf
   * SUPER_KEYWORD = "super";
   * ```
   */
  SuperKeyword = "SuperKeyword",
  /**
   * This kind represents a `SupportsKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * SUPPORTS_KEYWORD = "supports";
   * ```
   */
  SupportsKeyword = "SupportsKeyword",
  /**
   * This kind represents a `SwitchKeyword` node, with the following structure:
   *
   * ```ebnf
   * SWITCH_KEYWORD = "switch";
   * ```
   */
  SwitchKeyword = "SwitchKeyword",
  /**
   * This kind represents a `SzaboKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.7.0 *)
   * (* Reserved until 0.7.0 *)
   * SZABO_KEYWORD = "szabo";
   * ```
   */
  SzaboKeyword = "SzaboKeyword",
  /**
   * This kind represents a `ThisKeyword` node, with the following structure:
   *
   * ```ebnf
   * THIS_KEYWORD = "this";
   * ```
   */
  ThisKeyword = "ThisKeyword",
  /**
   * This kind represents a `ThrowKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * THROW_KEYWORD = "throw";
   * ```
   */
  ThrowKeyword = "ThrowKeyword",
  /**
   * This kind represents a `Tilde` node, with the following structure:
   *
   * ```ebnf
   * TILDE = "~";
   * ```
   */
  Tilde = "Tilde",
  /**
   * This kind represents a `TransientKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.27 *)
   * (* Never reserved *)
   * TRANSIENT_KEYWORD = "transient";
   * ```
   */
  TransientKeyword = "TransientKeyword",
  /**
   * This kind represents a `TrueKeyword` node, with the following structure:
   *
   * ```ebnf
   * TRUE_KEYWORD = "true";
   * ```
   */
  TrueKeyword = "TrueKeyword",
  /**
   * This kind represents a `TryKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * TRY_KEYWORD = "try";
   * ```
   */
  TryKeyword = "TryKeyword",
  /**
   * This kind represents a `TypeDefKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.0 *)
   * TYPE_DEF_KEYWORD = "typedef";
   * ```
   */
  TypeDefKeyword = "TypeDefKeyword",
  /**
   * This kind represents a `TypeKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.3 *)
   * TYPE_KEYWORD = "type";
   * ```
   */
  TypeKeyword = "TypeKeyword",
  /**
   * This kind represents a `TypeOfKeyword` node, with the following structure:
   *
   * ```ebnf
   * TYPE_OF_KEYWORD = "typeof";
   * ```
   */
  TypeOfKeyword = "TypeOfKeyword",
  /**
   * This kind represents a `UfixedKeyword` node, with the following structure:
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
   * This kind represents a `UintKeyword` node, with the following structure:
   *
   * ```ebnf
   * UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
   * ```
   */
  UintKeyword = "UintKeyword",
  /**
   * This kind represents a `UncheckedKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.0 *)
   * (* Reserved in 0.5.0 *)
   * UNCHECKED_KEYWORD = "unchecked";
   * ```
   */
  UncheckedKeyword = "UncheckedKeyword",
  /**
   * This kind represents a `UsingKeyword` node, with the following structure:
   *
   * ```ebnf
   * USING_KEYWORD = "using";
   * ```
   */
  UsingKeyword = "UsingKeyword",
  /**
   * This kind represents a `VarKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * VAR_KEYWORD = "var";
   * ```
   */
  VarKeyword = "VarKeyword",
  /**
   * This kind represents a `VersionSpecifier` node, with the following structure:
   *
   * ```ebnf
   * VERSION_SPECIFIER = «VERSION_SPECIFIER_FRAGMENT»;
   * ```
   */
  VersionSpecifier = "VersionSpecifier",
  /**
   * This kind represents a `ViewKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.16 *)
   * VIEW_KEYWORD = "view";
   * ```
   */
  ViewKeyword = "ViewKeyword",
  /**
   * This kind represents a `VirtualKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * (* Reserved in 0.6.0 *)
   * VIRTUAL_KEYWORD = "virtual";
   * ```
   */
  VirtualKeyword = "VirtualKeyword",
  /**
   * This kind represents a `WeeksKeyword` node, with the following structure:
   *
   * ```ebnf
   * WEEKS_KEYWORD = "weeks";
   * ```
   */
  WeeksKeyword = "WeeksKeyword",
  /**
   * This kind represents a `WeiKeyword` node, with the following structure:
   *
   * ```ebnf
   * WEI_KEYWORD = "wei";
   * ```
   */
  WeiKeyword = "WeiKeyword",
  /**
   * This kind represents a `WhileKeyword` node, with the following structure:
   *
   * ```ebnf
   * WHILE_KEYWORD = "while";
   * ```
   */
  WhileKeyword = "WhileKeyword",
  /**
   * This kind represents a `Whitespace` node, with the following structure:
   *
   * ```ebnf
   * WHITESPACE = (" " | "\t")+;
   * ```
   */
  Whitespace = "Whitespace",
  /**
   * This kind represents a `YearsKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YEARS_KEYWORD = "years";
   * ```
   */
  YearsKeyword = "YearsKeyword",
  /**
   * This kind represents a `YulAbstractKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ABSTRACT_KEYWORD = "abstract";
   * ```
   */
  YulAbstractKeyword = "YulAbstractKeyword",
  /**
   * This kind represents a `YulAddKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_ADD_KEYWORD = "add";
   * ```
   */
  YulAddKeyword = "YulAddKeyword",
  /**
   * This kind represents a `YulAddModKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_ADD_MOD_KEYWORD = "addmod";
   * ```
   */
  YulAddModKeyword = "YulAddModKeyword",
  /**
   * This kind represents a `YulAddressKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Never reserved *)
   * YUL_ADDRESS_KEYWORD = "address";
   * ```
   */
  YulAddressKeyword = "YulAddressKeyword",
  /**
   * This kind represents a `YulAfterKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_AFTER_KEYWORD = "after";
   * ```
   */
  YulAfterKeyword = "YulAfterKeyword",
  /**
   * This kind represents a `YulAliasKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_ALIAS_KEYWORD = "alias";
   * ```
   */
  YulAliasKeyword = "YulAliasKeyword",
  /**
   * This kind represents a `YulAndKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_AND_KEYWORD = "and";
   * ```
   */
  YulAndKeyword = "YulAndKeyword",
  /**
   * This kind represents a `YulAnonymousKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ANONYMOUS_KEYWORD = "anonymous";
   * ```
   */
  YulAnonymousKeyword = "YulAnonymousKeyword",
  /**
   * This kind represents a `YulApplyKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_APPLY_KEYWORD = "apply";
   * ```
   */
  YulApplyKeyword = "YulApplyKeyword",
  /**
   * This kind represents a `YulAsKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_AS_KEYWORD = "as";
   * ```
   */
  YulAsKeyword = "YulAsKeyword",
  /**
   * This kind represents a `YulAssemblyKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ASSEMBLY_KEYWORD = "assembly";
   * ```
   */
  YulAssemblyKeyword = "YulAssemblyKeyword",
  /**
   * This kind represents a `YulAutoKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_AUTO_KEYWORD = "auto";
   * ```
   */
  YulAutoKeyword = "YulAutoKeyword",
  /**
   * This kind represents a `YulBalanceKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_BALANCE_KEYWORD = "balance";
   * ```
   */
  YulBalanceKeyword = "YulBalanceKeyword",
  /**
   * This kind represents a `YulBaseFeeKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.7 *)
   * (* Reserved in 0.8.7 *)
   * YUL_BASE_FEE_KEYWORD = "basefee";
   * ```
   */
  YulBaseFeeKeyword = "YulBaseFeeKeyword",
  /**
   * This kind represents a `YulBlobBaseFeeKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.24 *)
   * (* Reserved in 0.8.25 *)
   * YUL_BLOB_BASE_FEE_KEYWORD = "blobbasefee";
   * ```
   */
  YulBlobBaseFeeKeyword = "YulBlobBaseFeeKeyword",
  /**
   * This kind represents a `YulBlobHashKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.24 *)
   * (* Reserved in 0.8.25 *)
   * YUL_BLOB_HASH_KEYWORD = "blobhash";
   * ```
   */
  YulBlobHashKeyword = "YulBlobHashKeyword",
  /**
   * This kind represents a `YulBlockHashKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_BLOCK_HASH_KEYWORD = "blockhash";
   * ```
   */
  YulBlockHashKeyword = "YulBlockHashKeyword",
  /**
   * This kind represents a `YulBoolKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.5.10 *)
   * YUL_BOOL_KEYWORD = "bool";
   * ```
   */
  YulBoolKeyword = "YulBoolKeyword",
  /**
   * This kind represents a `YulBreakKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_BREAK_KEYWORD = "break";
   * ```
   */
  YulBreakKeyword = "YulBreakKeyword",
  /**
   * This kind represents a `YulByteKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_BYTE_KEYWORD = "byte";
   * ```
   */
  YulByteKeyword = "YulByteKeyword",
  /**
   * This kind represents a `YulBytesKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
   * ```
   */
  YulBytesKeyword = "YulBytesKeyword",
  /**
   * This kind represents a `YulCallCodeKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CALL_CODE_KEYWORD = "callcode";
   * ```
   */
  YulCallCodeKeyword = "YulCallCodeKeyword",
  /**
   * This kind represents a `YulCallDataCopyKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CALL_DATA_COPY_KEYWORD = "calldatacopy";
   * ```
   */
  YulCallDataCopyKeyword = "YulCallDataCopyKeyword",
  /**
   * This kind represents a `YulCallDataKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_CALL_DATA_KEYWORD = "calldata";
   * ```
   */
  YulCallDataKeyword = "YulCallDataKeyword",
  /**
   * This kind represents a `YulCallDataLoadKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CALL_DATA_LOAD_KEYWORD = "calldataload";
   * ```
   */
  YulCallDataLoadKeyword = "YulCallDataLoadKeyword",
  /**
   * This kind represents a `YulCallDataSizeKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CALL_DATA_SIZE_KEYWORD = "calldatasize";
   * ```
   */
  YulCallDataSizeKeyword = "YulCallDataSizeKeyword",
  /**
   * This kind represents a `YulCallKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CALL_KEYWORD = "call";
   * ```
   */
  YulCallKeyword = "YulCallKeyword",
  /**
   * This kind represents a `YulCallValueKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CALL_VALUE_KEYWORD = "callvalue";
   * ```
   */
  YulCallValueKeyword = "YulCallValueKeyword",
  /**
   * This kind represents a `YulCallerKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CALLER_KEYWORD = "caller";
   * ```
   */
  YulCallerKeyword = "YulCallerKeyword",
  /**
   * This kind represents a `YulCaseKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CASE_KEYWORD = "case";
   * ```
   */
  YulCaseKeyword = "YulCaseKeyword",
  /**
   * This kind represents a `YulCatchKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_CATCH_KEYWORD = "catch";
   * ```
   */
  YulCatchKeyword = "YulCatchKeyword",
  /**
   * This kind represents a `YulChainIdKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.12 *)
   * YUL_CHAIN_ID_KEYWORD = "chainid";
   * ```
   */
  YulChainIdKeyword = "YulChainIdKeyword",
  /**
   * This kind represents a `YulCoinBaseKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_COIN_BASE_KEYWORD = "coinbase";
   * ```
   */
  YulCoinBaseKeyword = "YulCoinBaseKeyword",
  /**
   * This kind represents a `YulConstantKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_CONSTANT_KEYWORD = "constant";
   * ```
   */
  YulConstantKeyword = "YulConstantKeyword",
  /**
   * This kind represents a `YulConstructorKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_CONSTRUCTOR_KEYWORD = "constructor";
   * ```
   */
  YulConstructorKeyword = "YulConstructorKeyword",
  /**
   * This kind represents a `YulContinueKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CONTINUE_KEYWORD = "continue";
   * ```
   */
  YulContinueKeyword = "YulContinueKeyword",
  /**
   * This kind represents a `YulContractKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_CONTRACT_KEYWORD = "contract";
   * ```
   */
  YulContractKeyword = "YulContractKeyword",
  /**
   * This kind represents a `YulCopyOfKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_COPY_OF_KEYWORD = "copyof";
   * ```
   */
  YulCopyOfKeyword = "YulCopyOfKeyword",
  /**
   * This kind represents a `YulCreate2Keyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.12 *)
   * (* Reserved in 0.4.12 *)
   * YUL_CREATE_2_KEYWORD = "create2";
   * ```
   */
  YulCreate2Keyword = "YulCreate2Keyword",
  /**
   * This kind represents a `YulCreateKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_CREATE_KEYWORD = "create";
   * ```
   */
  YulCreateKeyword = "YulCreateKeyword",
  /**
   * This kind represents a `YulDaysKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_DAYS_KEYWORD = "days";
   * ```
   */
  YulDaysKeyword = "YulDaysKeyword",
  /**
   * This kind represents a `YulDecimalLiteral` node, with the following structure:
   *
   * ```ebnf
   * YUL_DECIMAL_LITERAL = "0" | (("1"…"9") ("0"…"9")*);
   * ```
   */
  YulDecimalLiteral = "YulDecimalLiteral",
  /**
   * This kind represents a `YulDefaultKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_DEFAULT_KEYWORD = "default";
   * ```
   */
  YulDefaultKeyword = "YulDefaultKeyword",
  /**
   * This kind represents a `YulDefineKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_DEFINE_KEYWORD = "define";
   * ```
   */
  YulDefineKeyword = "YulDefineKeyword",
  /**
   * This kind represents a `YulDelegateCallKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_DELEGATE_CALL_KEYWORD = "delegatecall";
   * ```
   */
  YulDelegateCallKeyword = "YulDelegateCallKeyword",
  /**
   * This kind represents a `YulDeleteKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_DELETE_KEYWORD = "delete";
   * ```
   */
  YulDeleteKeyword = "YulDeleteKeyword",
  /**
   * This kind represents a `YulDifficultyKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.8.18 *)
   * YUL_DIFFICULTY_KEYWORD = "difficulty";
   * ```
   */
  YulDifficultyKeyword = "YulDifficultyKeyword",
  /**
   * This kind represents a `YulDivKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_DIV_KEYWORD = "div";
   * ```
   */
  YulDivKeyword = "YulDivKeyword",
  /**
   * This kind represents a `YulDoKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_DO_KEYWORD = "do";
   * ```
   */
  YulDoKeyword = "YulDoKeyword",
  /**
   * This kind represents a `YulElseKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ELSE_KEYWORD = "else";
   * ```
   */
  YulElseKeyword = "YulElseKeyword",
  /**
   * This kind represents a `YulEmitKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_EMIT_KEYWORD = "emit";
   * ```
   */
  YulEmitKeyword = "YulEmitKeyword",
  /**
   * This kind represents a `YulEnumKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ENUM_KEYWORD = "enum";
   * ```
   */
  YulEnumKeyword = "YulEnumKeyword",
  /**
   * This kind represents a `YulEqKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_EQ_KEYWORD = "eq";
   * ```
   */
  YulEqKeyword = "YulEqKeyword",
  /**
   * This kind represents a `YulEtherKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_ETHER_KEYWORD = "ether";
   * ```
   */
  YulEtherKeyword = "YulEtherKeyword",
  /**
   * This kind represents a `YulEventKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_EVENT_KEYWORD = "event";
   * ```
   */
  YulEventKeyword = "YulEventKeyword",
  /**
   * This kind represents a `YulExpKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_EXP_KEYWORD = "exp";
   * ```
   */
  YulExpKeyword = "YulExpKeyword",
  /**
   * This kind represents a `YulExtCodeCopyKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_EXT_CODE_COPY_KEYWORD = "extcodecopy";
   * ```
   */
  YulExtCodeCopyKeyword = "YulExtCodeCopyKeyword",
  /**
   * This kind represents a `YulExtCodeHashKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.5.0 *)
   * (* Reserved in 0.5.0 *)
   * YUL_EXT_CODE_HASH_KEYWORD = "extcodehash";
   * ```
   */
  YulExtCodeHashKeyword = "YulExtCodeHashKeyword",
  /**
   * This kind represents a `YulExtCodeSizeKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_EXT_CODE_SIZE_KEYWORD = "extcodesize";
   * ```
   */
  YulExtCodeSizeKeyword = "YulExtCodeSizeKeyword",
  /**
   * This kind represents a `YulExternalKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_EXTERNAL_KEYWORD = "external";
   * ```
   */
  YulExternalKeyword = "YulExternalKeyword",
  /**
   * This kind represents a `YulFallbackKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.6.0 until 0.7.1 *)
   * YUL_FALLBACK_KEYWORD = "fallback";
   * ```
   */
  YulFallbackKeyword = "YulFallbackKeyword",
  /**
   * This kind represents a `YulFalseKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_FALSE_KEYWORD = "false";
   * ```
   */
  YulFalseKeyword = "YulFalseKeyword",
  /**
   * This kind represents a `YulFinalKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_FINAL_KEYWORD = "final";
   * ```
   */
  YulFinalKeyword = "YulFinalKeyword",
  /**
   * This kind represents a `YulFinneyKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.0 *)
   * YUL_FINNEY_KEYWORD = "finney";
   * ```
   */
  YulFinneyKeyword = "YulFinneyKeyword",
  /**
   * This kind represents a `YulFixedKeyword` node, with the following structure:
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
   * This kind represents a `YulForKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_FOR_KEYWORD = "for";
   * ```
   */
  YulForKeyword = "YulForKeyword",
  /**
   * This kind represents a `YulFunctionKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_FUNCTION_KEYWORD = "function";
   * ```
   */
  YulFunctionKeyword = "YulFunctionKeyword",
  /**
   * This kind represents a `YulGasKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_GAS_KEYWORD = "gas";
   * ```
   */
  YulGasKeyword = "YulGasKeyword",
  /**
   * This kind represents a `YulGasLimitKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_GAS_LIMIT_KEYWORD = "gaslimit";
   * ```
   */
  YulGasLimitKeyword = "YulGasLimitKeyword",
  /**
   * This kind represents a `YulGasPriceKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_GAS_PRICE_KEYWORD = "gasprice";
   * ```
   */
  YulGasPriceKeyword = "YulGasPriceKeyword",
  /**
   * This kind represents a `YulGtKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_GT_KEYWORD = "gt";
   * ```
   */
  YulGtKeyword = "YulGtKeyword",
  /**
   * This kind represents a `YulGweiKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.7.0 until 0.7.1 *)
   * YUL_GWEI_KEYWORD = "gwei";
   * ```
   */
  YulGweiKeyword = "YulGweiKeyword",
  /**
   * This kind represents a `YulHexKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_HEX_KEYWORD = "hex";
   * ```
   */
  YulHexKeyword = "YulHexKeyword",
  /**
   * This kind represents a `YulHexLiteral` node, with the following structure:
   *
   * ```ebnf
   * YUL_HEX_LITERAL = "0x" «HEX_CHARACTER»+;
   * ```
   */
  YulHexLiteral = "YulHexLiteral",
  /**
   * This kind represents a `YulHoursKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_HOURS_KEYWORD = "hours";
   * ```
   */
  YulHoursKeyword = "YulHoursKeyword",
  /**
   * This kind represents a `YulIdentifier` node, with the following structure:
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
   * This kind represents a `YulIfKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_IF_KEYWORD = "if";
   * ```
   */
  YulIfKeyword = "YulIfKeyword",
  /**
   * This kind represents a `YulImmutableKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_IMMUTABLE_KEYWORD = "immutable";
   * ```
   */
  YulImmutableKeyword = "YulImmutableKeyword",
  /**
   * This kind represents a `YulImplementsKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_IMPLEMENTS_KEYWORD = "implements";
   * ```
   */
  YulImplementsKeyword = "YulImplementsKeyword",
  /**
   * This kind represents a `YulImportKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_IMPORT_KEYWORD = "import";
   * ```
   */
  YulImportKeyword = "YulImportKeyword",
  /**
   * This kind represents a `YulInKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.6.8 *)
   * YUL_IN_KEYWORD = "in";
   * ```
   */
  YulInKeyword = "YulInKeyword",
  /**
   * This kind represents a `YulIndexedKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INDEXED_KEYWORD = "indexed";
   * ```
   */
  YulIndexedKeyword = "YulIndexedKeyword",
  /**
   * This kind represents a `YulInlineKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INLINE_KEYWORD = "inline";
   * ```
   */
  YulInlineKeyword = "YulInlineKeyword",
  /**
   * This kind represents a `YulIntKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
   * ```
   */
  YulIntKeyword = "YulIntKeyword",
  /**
   * This kind represents a `YulInterfaceKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INTERFACE_KEYWORD = "interface";
   * ```
   */
  YulInterfaceKeyword = "YulInterfaceKeyword",
  /**
   * This kind represents a `YulInternalKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_INTERNAL_KEYWORD = "internal";
   * ```
   */
  YulInternalKeyword = "YulInternalKeyword",
  /**
   * This kind represents a `YulInvalidKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_INVALID_KEYWORD = "invalid";
   * ```
   */
  YulInvalidKeyword = "YulInvalidKeyword",
  /**
   * This kind represents a `YulIsKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_IS_KEYWORD = "is";
   * ```
   */
  YulIsKeyword = "YulIsKeyword",
  /**
   * This kind represents a `YulIsZeroKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_IS_ZERO_KEYWORD = "iszero";
   * ```
   */
  YulIsZeroKeyword = "YulIsZeroKeyword",
  /**
   * This kind represents a `YulJumpKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YUL_JUMP_KEYWORD = "jump";
   * ```
   */
  YulJumpKeyword = "YulJumpKeyword",
  /**
   * This kind represents a `YulJumpiKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * YUL_JUMPI_KEYWORD = "jumpi";
   * ```
   */
  YulJumpiKeyword = "YulJumpiKeyword",
  /**
   * This kind represents a `YulKeccak256Keyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.12 *)
   * (* Reserved in 0.4.12 *)
   * YUL_KECCAK_256_KEYWORD = "keccak256";
   * ```
   */
  YulKeccak256Keyword = "YulKeccak256Keyword",
  /**
   * This kind represents a `YulLeaveKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.6.0 *)
   * (* Reserved in 0.7.1 *)
   * YUL_LEAVE_KEYWORD = "leave";
   * ```
   */
  YulLeaveKeyword = "YulLeaveKeyword",
  /**
   * This kind represents a `YulLetKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_LET_KEYWORD = "let";
   * ```
   */
  YulLetKeyword = "YulLetKeyword",
  /**
   * This kind represents a `YulLibraryKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_LIBRARY_KEYWORD = "library";
   * ```
   */
  YulLibraryKeyword = "YulLibraryKeyword",
  /**
   * This kind represents a `YulLog0Keyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_LOG_0_KEYWORD = "log0";
   * ```
   */
  YulLog0Keyword = "YulLog0Keyword",
  /**
   * This kind represents a `YulLog1Keyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_LOG_1_KEYWORD = "log1";
   * ```
   */
  YulLog1Keyword = "YulLog1Keyword",
  /**
   * This kind represents a `YulLog2Keyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_LOG_2_KEYWORD = "log2";
   * ```
   */
  YulLog2Keyword = "YulLog2Keyword",
  /**
   * This kind represents a `YulLog3Keyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_LOG_3_KEYWORD = "log3";
   * ```
   */
  YulLog3Keyword = "YulLog3Keyword",
  /**
   * This kind represents a `YulLog4Keyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_LOG_4_KEYWORD = "log4";
   * ```
   */
  YulLog4Keyword = "YulLog4Keyword",
  /**
   * This kind represents a `YulLtKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_LT_KEYWORD = "lt";
   * ```
   */
  YulLtKeyword = "YulLtKeyword",
  /**
   * This kind represents a `YulMCopyKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.24 *)
   * (* Reserved in 0.8.25 *)
   * YUL_M_COPY_KEYWORD = "mcopy";
   * ```
   */
  YulMcopyKeyword = "YulMcopyKeyword",
  /**
   * This kind represents a `YulMLoadKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_M_LOAD_KEYWORD = "mload";
   * ```
   */
  YulMloadKeyword = "YulMloadKeyword",
  /**
   * This kind represents a `YulMSizeKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_M_SIZE_KEYWORD = "msize";
   * ```
   */
  YulMsizeKeyword = "YulMsizeKeyword",
  /**
   * This kind represents a `YulMStore8Keyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_M_STORE_8_KEYWORD = "mstore8";
   * ```
   */
  YulMstore8Keyword = "YulMstore8Keyword",
  /**
   * This kind represents a `YulMStoreKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_M_STORE_KEYWORD = "mstore";
   * ```
   */
  YulMstoreKeyword = "YulMstoreKeyword",
  /**
   * This kind represents a `YulMacroKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_MACRO_KEYWORD = "macro";
   * ```
   */
  YulMacroKeyword = "YulMacroKeyword",
  /**
   * This kind represents a `YulMappingKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MAPPING_KEYWORD = "mapping";
   * ```
   */
  YulMappingKeyword = "YulMappingKeyword",
  /**
   * This kind represents a `YulMatchKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MATCH_KEYWORD = "match";
   * ```
   */
  YulMatchKeyword = "YulMatchKeyword",
  /**
   * This kind represents a `YulMemoryKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MEMORY_KEYWORD = "memory";
   * ```
   */
  YulMemoryKeyword = "YulMemoryKeyword",
  /**
   * This kind represents a `YulMinutesKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MINUTES_KEYWORD = "minutes";
   * ```
   */
  YulMinutesKeyword = "YulMinutesKeyword",
  /**
   * This kind represents a `YulModKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_MOD_KEYWORD = "mod";
   * ```
   */
  YulModKeyword = "YulModKeyword",
  /**
   * This kind represents a `YulModifierKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_MODIFIER_KEYWORD = "modifier";
   * ```
   */
  YulModifierKeyword = "YulModifierKeyword",
  /**
   * This kind represents a `YulMulKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_MUL_KEYWORD = "mul";
   * ```
   */
  YulMulKeyword = "YulMulKeyword",
  /**
   * This kind represents a `YulMulModKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_MUL_MOD_KEYWORD = "mulmod";
   * ```
   */
  YulMulModKeyword = "YulMulModKeyword",
  /**
   * This kind represents a `YulMutableKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_MUTABLE_KEYWORD = "mutable";
   * ```
   */
  YulMutableKeyword = "YulMutableKeyword",
  /**
   * This kind represents a `YulNewKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_NEW_KEYWORD = "new";
   * ```
   */
  YulNewKeyword = "YulNewKeyword",
  /**
   * This kind represents a `YulNotKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_NOT_KEYWORD = "not";
   * ```
   */
  YulNotKeyword = "YulNotKeyword",
  /**
   * This kind represents a `YulNullKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_NULL_KEYWORD = "null";
   * ```
   */
  YulNullKeyword = "YulNullKeyword",
  /**
   * This kind represents a `YulNumberKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_NUMBER_KEYWORD = "number";
   * ```
   */
  YulNumberKeyword = "YulNumberKeyword",
  /**
   * This kind represents a `YulOfKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_OF_KEYWORD = "of";
   * ```
   */
  YulOfKeyword = "YulOfKeyword",
  /**
   * This kind represents a `YulOrKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_OR_KEYWORD = "or";
   * ```
   */
  YulOrKeyword = "YulOrKeyword",
  /**
   * This kind represents a `YulOriginKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_ORIGIN_KEYWORD = "origin";
   * ```
   */
  YulOriginKeyword = "YulOriginKeyword",
  /**
   * This kind represents a `YulOverrideKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_OVERRIDE_KEYWORD = "override";
   * ```
   */
  YulOverrideKeyword = "YulOverrideKeyword",
  /**
   * This kind represents a `YulPartialKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_PARTIAL_KEYWORD = "partial";
   * ```
   */
  YulPartialKeyword = "YulPartialKeyword",
  /**
   * This kind represents a `YulPayableKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PAYABLE_KEYWORD = "payable";
   * ```
   */
  YulPayableKeyword = "YulPayableKeyword",
  /**
   * This kind represents a `YulPopKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_POP_KEYWORD = "pop";
   * ```
   */
  YulPopKeyword = "YulPopKeyword",
  /**
   * This kind represents a `YulPragmaKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PRAGMA_KEYWORD = "pragma";
   * ```
   */
  YulPragmaKeyword = "YulPragmaKeyword",
  /**
   * This kind represents a `YulPrevRandaoKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.18 *)
   * (* Reserved in 0.8.18 *)
   * YUL_PREV_RANDAO_KEYWORD = "prevrandao";
   * ```
   */
  YulPrevRandaoKeyword = "YulPrevRandaoKeyword",
  /**
   * This kind represents a `YulPrivateKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PRIVATE_KEYWORD = "private";
   * ```
   */
  YulPrivateKeyword = "YulPrivateKeyword",
  /**
   * This kind represents a `YulPromiseKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_PROMISE_KEYWORD = "promise";
   * ```
   */
  YulPromiseKeyword = "YulPromiseKeyword",
  /**
   * This kind represents a `YulPublicKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PUBLIC_KEYWORD = "public";
   * ```
   */
  YulPublicKeyword = "YulPublicKeyword",
  /**
   * This kind represents a `YulPureKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_PURE_KEYWORD = "pure";
   * ```
   */
  YulPureKeyword = "YulPureKeyword",
  /**
   * This kind represents a `YulReceiveKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.6.0 until 0.7.1 *)
   * YUL_RECEIVE_KEYWORD = "receive";
   * ```
   */
  YulReceiveKeyword = "YulReceiveKeyword",
  /**
   * This kind represents a `YulReferenceKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_REFERENCE_KEYWORD = "reference";
   * ```
   */
  YulReferenceKeyword = "YulReferenceKeyword",
  /**
   * This kind represents a `YulRelocatableKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_RELOCATABLE_KEYWORD = "relocatable";
   * ```
   */
  YulRelocatableKeyword = "YulRelocatableKeyword",
  /**
   * This kind represents a `YulReturnDataCopyKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.12 *)
   * (* Reserved in 0.4.12 *)
   * YUL_RETURN_DATA_COPY_KEYWORD = "returndatacopy";
   * ```
   */
  YulReturnDataCopyKeyword = "YulReturnDataCopyKeyword",
  /**
   * This kind represents a `YulReturnDataSizeKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.12 *)
   * (* Reserved in 0.4.12 *)
   * YUL_RETURN_DATA_SIZE_KEYWORD = "returndatasize";
   * ```
   */
  YulReturnDataSizeKeyword = "YulReturnDataSizeKeyword",
  /**
   * This kind represents a `YulReturnKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_RETURN_KEYWORD = "return";
   * ```
   */
  YulReturnKeyword = "YulReturnKeyword",
  /**
   * This kind represents a `YulReturnsKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_RETURNS_KEYWORD = "returns";
   * ```
   */
  YulReturnsKeyword = "YulReturnsKeyword",
  /**
   * This kind represents a `YulRevertKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_REVERT_KEYWORD = "revert";
   * ```
   */
  YulRevertKeyword = "YulRevertKeyword",
  /**
   * This kind represents a `YulSDivKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_S_DIV_KEYWORD = "sdiv";
   * ```
   */
  YulSdivKeyword = "YulSdivKeyword",
  /**
   * This kind represents a `YulSLoadKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_S_LOAD_KEYWORD = "sload";
   * ```
   */
  YulSloadKeyword = "YulSloadKeyword",
  /**
   * This kind represents a `YulSModKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_S_MOD_KEYWORD = "smod";
   * ```
   */
  YulSmodKeyword = "YulSmodKeyword",
  /**
   * This kind represents a `YulSStoreKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_S_STORE_KEYWORD = "sstore";
   * ```
   */
  YulSstoreKeyword = "YulSstoreKeyword",
  /**
   * This kind represents a `YulSarKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.4.21 *)
   * YUL_SAR_KEYWORD = "sar";
   * ```
   */
  YulSarKeyword = "YulSarKeyword",
  /**
   * This kind represents a `YulSealedKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_SEALED_KEYWORD = "sealed";
   * ```
   */
  YulSealedKeyword = "YulSealedKeyword",
  /**
   * This kind represents a `YulSecondsKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_SECONDS_KEYWORD = "seconds";
   * ```
   */
  YulSecondsKeyword = "YulSecondsKeyword",
  /**
   * This kind represents a `YulSelfBalanceKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.5.12 *)
   * YUL_SELF_BALANCE_KEYWORD = "selfbalance";
   * ```
   */
  YulSelfBalanceKeyword = "YulSelfBalanceKeyword",
  /**
   * This kind represents a `YulSelfDestructKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_SELF_DESTRUCT_KEYWORD = "selfdestruct";
   * ```
   */
  YulSelfDestructKeyword = "YulSelfDestructKeyword",
  /**
   * This kind represents a `YulSgtKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_SGT_KEYWORD = "sgt";
   * ```
   */
  YulSgtKeyword = "YulSgtKeyword",
  /**
   * This kind represents a `YulSha3Keyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * (* Reserved until 0.5.0 *)
   * YUL_SHA_3_KEYWORD = "sha3";
   * ```
   */
  YulSha3Keyword = "YulSha3Keyword",
  /**
   * This kind represents a `YulShlKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.4.21 *)
   * YUL_SHL_KEYWORD = "shl";
   * ```
   */
  YulShlKeyword = "YulShlKeyword",
  /**
   * This kind represents a `YulShrKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved in 0.4.21 *)
   * YUL_SHR_KEYWORD = "shr";
   * ```
   */
  YulShrKeyword = "YulShrKeyword",
  /**
   * This kind represents a `YulSignExtendKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_SIGN_EXTEND_KEYWORD = "signextend";
   * ```
   */
  YulSignExtendKeyword = "YulSignExtendKeyword",
  /**
   * This kind represents a `YulSizeOfKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_SIZE_OF_KEYWORD = "sizeof";
   * ```
   */
  YulSizeOfKeyword = "YulSizeOfKeyword",
  /**
   * This kind represents a `YulSltKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_SLT_KEYWORD = "slt";
   * ```
   */
  YulSltKeyword = "YulSltKeyword",
  /**
   * This kind represents a `YulStaticCallKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.4.12 *)
   * (* Reserved in 0.4.12 *)
   * YUL_STATIC_CALL_KEYWORD = "staticcall";
   * ```
   */
  YulStaticCallKeyword = "YulStaticCallKeyword",
  /**
   * This kind represents a `YulStaticKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_STATIC_KEYWORD = "static";
   * ```
   */
  YulStaticKeyword = "YulStaticKeyword",
  /**
   * This kind represents a `YulStopKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_STOP_KEYWORD = "stop";
   * ```
   */
  YulStopKeyword = "YulStopKeyword",
  /**
   * This kind represents a `YulStorageKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_STORAGE_KEYWORD = "storage";
   * ```
   */
  YulStorageKeyword = "YulStorageKeyword",
  /**
   * This kind represents a `YulStringKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_STRING_KEYWORD = "string";
   * ```
   */
  YulStringKeyword = "YulStringKeyword",
  /**
   * This kind represents a `YulStructKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_STRUCT_KEYWORD = "struct";
   * ```
   */
  YulStructKeyword = "YulStructKeyword",
  /**
   * This kind represents a `YulSubKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_SUB_KEYWORD = "sub";
   * ```
   */
  YulSubKeyword = "YulSubKeyword",
  /**
   * This kind represents a `YulSuicideKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Deprecated in 0.5.0 *)
   * (* Reserved until 0.5.0 *)
   * YUL_SUICIDE_KEYWORD = "suicide";
   * ```
   */
  YulSuicideKeyword = "YulSuicideKeyword",
  /**
   * This kind represents a `YulSuperKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_SUPER_KEYWORD = "super";
   * ```
   */
  YulSuperKeyword = "YulSuperKeyword",
  /**
   * This kind represents a `YulSupportsKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_SUPPORTS_KEYWORD = "supports";
   * ```
   */
  YulSupportsKeyword = "YulSupportsKeyword",
  /**
   * This kind represents a `YulSwitchKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_SWITCH_KEYWORD = "switch";
   * ```
   */
  YulSwitchKeyword = "YulSwitchKeyword",
  /**
   * This kind represents a `YulSzaboKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.0 *)
   * YUL_SZABO_KEYWORD = "szabo";
   * ```
   */
  YulSzaboKeyword = "YulSzaboKeyword",
  /**
   * This kind represents a `YulTLoadKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.24 *)
   * (* Reserved in 0.8.25 *)
   * YUL_T_LOAD_KEYWORD = "tload";
   * ```
   */
  YulTloadKeyword = "YulTloadKeyword",
  /**
   * This kind represents a `YulTStoreKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Introduced in 0.8.24 *)
   * (* Reserved in 0.8.25 *)
   * YUL_T_STORE_KEYWORD = "tstore";
   * ```
   */
  YulTstoreKeyword = "YulTstoreKeyword",
  /**
   * This kind represents a `YulThisKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_THIS_KEYWORD = "this";
   * ```
   */
  YulThisKeyword = "YulThisKeyword",
  /**
   * This kind represents a `YulThrowKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_THROW_KEYWORD = "throw";
   * ```
   */
  YulThrowKeyword = "YulThrowKeyword",
  /**
   * This kind represents a `YulTimestampKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_TIMESTAMP_KEYWORD = "timestamp";
   * ```
   */
  YulTimestampKeyword = "YulTimestampKeyword",
  /**
   * This kind represents a `YulTrueKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_TRUE_KEYWORD = "true";
   * ```
   */
  YulTrueKeyword = "YulTrueKeyword",
  /**
   * This kind represents a `YulTryKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_TRY_KEYWORD = "try";
   * ```
   */
  YulTryKeyword = "YulTryKeyword",
  /**
   * This kind represents a `YulTypeDefKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_TYPE_DEF_KEYWORD = "typedef";
   * ```
   */
  YulTypeDefKeyword = "YulTypeDefKeyword",
  /**
   * This kind represents a `YulTypeKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_TYPE_KEYWORD = "type";
   * ```
   */
  YulTypeKeyword = "YulTypeKeyword",
  /**
   * This kind represents a `YulTypeOfKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_TYPE_OF_KEYWORD = "typeof";
   * ```
   */
  YulTypeOfKeyword = "YulTypeOfKeyword",
  /**
   * This kind represents a `YulUfixedKeyword` node, with the following structure:
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
   * This kind represents a `YulUintKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
   * ```
   */
  YulUintKeyword = "YulUintKeyword",
  /**
   * This kind represents a `YulUncheckedKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.5.0 until 0.7.1 *)
   * YUL_UNCHECKED_KEYWORD = "unchecked";
   * ```
   */
  YulUncheckedKeyword = "YulUncheckedKeyword",
  /**
   * This kind represents a `YulUsingKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_USING_KEYWORD = "using";
   * ```
   */
  YulUsingKeyword = "YulUsingKeyword",
  /**
   * This kind represents a `YulVarKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.6.5 *)
   * YUL_VAR_KEYWORD = "var";
   * ```
   */
  YulVarKeyword = "YulVarKeyword",
  /**
   * This kind represents a `YulViewKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_VIEW_KEYWORD = "view";
   * ```
   */
  YulViewKeyword = "YulViewKeyword",
  /**
   * This kind represents a `YulVirtualKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved from 0.6.0 until 0.7.1 *)
   * YUL_VIRTUAL_KEYWORD = "virtual";
   * ```
   */
  YulVirtualKeyword = "YulVirtualKeyword",
  /**
   * This kind represents a `YulWeeksKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_WEEKS_KEYWORD = "weeks";
   * ```
   */
  YulWeeksKeyword = "YulWeeksKeyword",
  /**
   * This kind represents a `YulWeiKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_WEI_KEYWORD = "wei";
   * ```
   */
  YulWeiKeyword = "YulWeiKeyword",
  /**
   * This kind represents a `YulWhileKeyword` node, with the following structure:
   *
   * ```ebnf
   * (* Reserved until 0.7.1 *)
   * YUL_WHILE_KEYWORD = "while";
   * ```
   */
  YulWhileKeyword = "YulWhileKeyword",
  /**
   * This kind represents a `YulXorKeyword` node, with the following structure:
   *
   * ```ebnf
   * YUL_XOR_KEYWORD = "xor";
   * ```
   */
  YulXorKeyword = "YulXorKeyword",
  /**
   * This kind represents a `YulYearsKeyword` node, with the following structure:
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
