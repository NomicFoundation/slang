// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import { NonterminalKind, NonterminalNode, Node, NodeType, TerminalNode, TerminalKind, Edge } from "../index.mjs";

/** Abstract class to rewrite a CST. */
export abstract class BaseRewriter {
  /** Replaces the `node` with a new node. If the result is `undefined`, the node is removed from the tree.
        This function typically the entry point of the rewrite operation. */
  public rewriteNode(node: Node): Node | undefined {
    switch (node.type) {
      case NodeType.TerminalNode:
        return this.rewriteTerminalNode(node);
      case NodeType.NonterminalNode:
        return this.rewriteNonterminalNode(node);
    }
  }

  /** Rewrites a non-terminal node. Typically called from `rewriteNode`. */
  public rewriteNonterminalNode(node: NonterminalNode): Node | undefined {
    switch (node.kind) {
      case NonterminalKind.AbicoderPragma:
        return this.rewriteAbicoderPragma(node);

      case NonterminalKind.AbicoderVersion:
        return this.rewriteAbicoderVersion(node);

      case NonterminalKind.AdditiveExpression:
        return this.rewriteAdditiveExpression(node);

      case NonterminalKind.AddressType:
        return this.rewriteAddressType(node);

      case NonterminalKind.AndExpression:
        return this.rewriteAndExpression(node);

      case NonterminalKind.ArgumentsDeclaration:
        return this.rewriteArgumentsDeclaration(node);

      case NonterminalKind.ArrayExpression:
        return this.rewriteArrayExpression(node);

      case NonterminalKind.ArrayTypeName:
        return this.rewriteArrayTypeName(node);

      case NonterminalKind.ArrayValues:
        return this.rewriteArrayValues(node);

      case NonterminalKind.AssemblyFlags:
        return this.rewriteAssemblyFlags(node);

      case NonterminalKind.AssemblyFlagsDeclaration:
        return this.rewriteAssemblyFlagsDeclaration(node);

      case NonterminalKind.AssemblyStatement:
        return this.rewriteAssemblyStatement(node);

      case NonterminalKind.AssignmentExpression:
        return this.rewriteAssignmentExpression(node);

      case NonterminalKind.BitwiseAndExpression:
        return this.rewriteBitwiseAndExpression(node);

      case NonterminalKind.BitwiseOrExpression:
        return this.rewriteBitwiseOrExpression(node);

      case NonterminalKind.BitwiseXorExpression:
        return this.rewriteBitwiseXorExpression(node);

      case NonterminalKind.Block:
        return this.rewriteBlock(node);

      case NonterminalKind.BreakStatement:
        return this.rewriteBreakStatement(node);

      case NonterminalKind.CallOptions:
        return this.rewriteCallOptions(node);

      case NonterminalKind.CallOptionsExpression:
        return this.rewriteCallOptionsExpression(node);

      case NonterminalKind.CatchClause:
        return this.rewriteCatchClause(node);

      case NonterminalKind.CatchClauseError:
        return this.rewriteCatchClauseError(node);

      case NonterminalKind.CatchClauses:
        return this.rewriteCatchClauses(node);

      case NonterminalKind.ConditionalExpression:
        return this.rewriteConditionalExpression(node);

      case NonterminalKind.ConstantDefinition:
        return this.rewriteConstantDefinition(node);

      case NonterminalKind.ConstructorAttribute:
        return this.rewriteConstructorAttribute(node);

      case NonterminalKind.ConstructorAttributes:
        return this.rewriteConstructorAttributes(node);

      case NonterminalKind.ConstructorDefinition:
        return this.rewriteConstructorDefinition(node);

      case NonterminalKind.ContinueStatement:
        return this.rewriteContinueStatement(node);

      case NonterminalKind.ContractDefinition:
        return this.rewriteContractDefinition(node);

      case NonterminalKind.ContractMember:
        return this.rewriteContractMember(node);

      case NonterminalKind.ContractMembers:
        return this.rewriteContractMembers(node);

      case NonterminalKind.ContractSpecifier:
        return this.rewriteContractSpecifier(node);

      case NonterminalKind.ContractSpecifiers:
        return this.rewriteContractSpecifiers(node);

      case NonterminalKind.DecimalNumberExpression:
        return this.rewriteDecimalNumberExpression(node);

      case NonterminalKind.DoWhileStatement:
        return this.rewriteDoWhileStatement(node);

      case NonterminalKind.ElementaryType:
        return this.rewriteElementaryType(node);

      case NonterminalKind.ElseBranch:
        return this.rewriteElseBranch(node);

      case NonterminalKind.EmitStatement:
        return this.rewriteEmitStatement(node);

      case NonterminalKind.EnumDefinition:
        return this.rewriteEnumDefinition(node);

      case NonterminalKind.EnumMembers:
        return this.rewriteEnumMembers(node);

      case NonterminalKind.EqualityExpression:
        return this.rewriteEqualityExpression(node);

      case NonterminalKind.ErrorDefinition:
        return this.rewriteErrorDefinition(node);

      case NonterminalKind.ErrorParameter:
        return this.rewriteErrorParameter(node);

      case NonterminalKind.ErrorParameters:
        return this.rewriteErrorParameters(node);

      case NonterminalKind.ErrorParametersDeclaration:
        return this.rewriteErrorParametersDeclaration(node);

      case NonterminalKind.EventDefinition:
        return this.rewriteEventDefinition(node);

      case NonterminalKind.EventParameter:
        return this.rewriteEventParameter(node);

      case NonterminalKind.EventParameters:
        return this.rewriteEventParameters(node);

      case NonterminalKind.EventParametersDeclaration:
        return this.rewriteEventParametersDeclaration(node);

      case NonterminalKind.ExperimentalFeature:
        return this.rewriteExperimentalFeature(node);

      case NonterminalKind.ExperimentalPragma:
        return this.rewriteExperimentalPragma(node);

      case NonterminalKind.ExponentiationExpression:
        return this.rewriteExponentiationExpression(node);

      case NonterminalKind.Expression:
        return this.rewriteExpression(node);

      case NonterminalKind.ExpressionStatement:
        return this.rewriteExpressionStatement(node);

      case NonterminalKind.FallbackFunctionAttribute:
        return this.rewriteFallbackFunctionAttribute(node);

      case NonterminalKind.FallbackFunctionAttributes:
        return this.rewriteFallbackFunctionAttributes(node);

      case NonterminalKind.FallbackFunctionDefinition:
        return this.rewriteFallbackFunctionDefinition(node);

      case NonterminalKind.ForStatement:
        return this.rewriteForStatement(node);

      case NonterminalKind.ForStatementCondition:
        return this.rewriteForStatementCondition(node);

      case NonterminalKind.ForStatementInitialization:
        return this.rewriteForStatementInitialization(node);

      case NonterminalKind.FunctionAttribute:
        return this.rewriteFunctionAttribute(node);

      case NonterminalKind.FunctionAttributes:
        return this.rewriteFunctionAttributes(node);

      case NonterminalKind.FunctionBody:
        return this.rewriteFunctionBody(node);

      case NonterminalKind.FunctionCallExpression:
        return this.rewriteFunctionCallExpression(node);

      case NonterminalKind.FunctionDefinition:
        return this.rewriteFunctionDefinition(node);

      case NonterminalKind.FunctionName:
        return this.rewriteFunctionName(node);

      case NonterminalKind.FunctionType:
        return this.rewriteFunctionType(node);

      case NonterminalKind.FunctionTypeAttribute:
        return this.rewriteFunctionTypeAttribute(node);

      case NonterminalKind.FunctionTypeAttributes:
        return this.rewriteFunctionTypeAttributes(node);

      case NonterminalKind.HexNumberExpression:
        return this.rewriteHexNumberExpression(node);

      case NonterminalKind.HexStringLiteral:
        return this.rewriteHexStringLiteral(node);

      case NonterminalKind.HexStringLiterals:
        return this.rewriteHexStringLiterals(node);

      case NonterminalKind.IdentifierPath:
        return this.rewriteIdentifierPath(node);

      case NonterminalKind.IfStatement:
        return this.rewriteIfStatement(node);

      case NonterminalKind.ImportAlias:
        return this.rewriteImportAlias(node);

      case NonterminalKind.ImportClause:
        return this.rewriteImportClause(node);

      case NonterminalKind.ImportDeconstruction:
        return this.rewriteImportDeconstruction(node);

      case NonterminalKind.ImportDeconstructionSymbol:
        return this.rewriteImportDeconstructionSymbol(node);

      case NonterminalKind.ImportDeconstructionSymbols:
        return this.rewriteImportDeconstructionSymbols(node);

      case NonterminalKind.ImportDirective:
        return this.rewriteImportDirective(node);

      case NonterminalKind.IndexAccessEnd:
        return this.rewriteIndexAccessEnd(node);

      case NonterminalKind.IndexAccessExpression:
        return this.rewriteIndexAccessExpression(node);

      case NonterminalKind.InequalityExpression:
        return this.rewriteInequalityExpression(node);

      case NonterminalKind.InheritanceSpecifier:
        return this.rewriteInheritanceSpecifier(node);

      case NonterminalKind.InheritanceType:
        return this.rewriteInheritanceType(node);

      case NonterminalKind.InheritanceTypes:
        return this.rewriteInheritanceTypes(node);

      case NonterminalKind.InterfaceDefinition:
        return this.rewriteInterfaceDefinition(node);

      case NonterminalKind.InterfaceMembers:
        return this.rewriteInterfaceMembers(node);

      case NonterminalKind.LibraryDefinition:
        return this.rewriteLibraryDefinition(node);

      case NonterminalKind.LibraryMembers:
        return this.rewriteLibraryMembers(node);

      case NonterminalKind.MappingKey:
        return this.rewriteMappingKey(node);

      case NonterminalKind.MappingKeyType:
        return this.rewriteMappingKeyType(node);

      case NonterminalKind.MappingType:
        return this.rewriteMappingType(node);

      case NonterminalKind.MappingValue:
        return this.rewriteMappingValue(node);

      case NonterminalKind.MemberAccessExpression:
        return this.rewriteMemberAccessExpression(node);

      case NonterminalKind.ModifierAttribute:
        return this.rewriteModifierAttribute(node);

      case NonterminalKind.ModifierAttributes:
        return this.rewriteModifierAttributes(node);

      case NonterminalKind.ModifierDefinition:
        return this.rewriteModifierDefinition(node);

      case NonterminalKind.ModifierInvocation:
        return this.rewriteModifierInvocation(node);

      case NonterminalKind.MultiplicativeExpression:
        return this.rewriteMultiplicativeExpression(node);

      case NonterminalKind.NamedArgument:
        return this.rewriteNamedArgument(node);

      case NonterminalKind.NamedArgumentGroup:
        return this.rewriteNamedArgumentGroup(node);

      case NonterminalKind.NamedArguments:
        return this.rewriteNamedArguments(node);

      case NonterminalKind.NamedArgumentsDeclaration:
        return this.rewriteNamedArgumentsDeclaration(node);

      case NonterminalKind.NamedImport:
        return this.rewriteNamedImport(node);

      case NonterminalKind.NewExpression:
        return this.rewriteNewExpression(node);

      case NonterminalKind.NumberUnit:
        return this.rewriteNumberUnit(node);

      case NonterminalKind.OrExpression:
        return this.rewriteOrExpression(node);

      case NonterminalKind.OverridePaths:
        return this.rewriteOverridePaths(node);

      case NonterminalKind.OverridePathsDeclaration:
        return this.rewriteOverridePathsDeclaration(node);

      case NonterminalKind.OverrideSpecifier:
        return this.rewriteOverrideSpecifier(node);

      case NonterminalKind.Parameter:
        return this.rewriteParameter(node);

      case NonterminalKind.Parameters:
        return this.rewriteParameters(node);

      case NonterminalKind.ParametersDeclaration:
        return this.rewriteParametersDeclaration(node);

      case NonterminalKind.PathImport:
        return this.rewritePathImport(node);

      case NonterminalKind.PositionalArguments:
        return this.rewritePositionalArguments(node);

      case NonterminalKind.PositionalArgumentsDeclaration:
        return this.rewritePositionalArgumentsDeclaration(node);

      case NonterminalKind.PostfixExpression:
        return this.rewritePostfixExpression(node);

      case NonterminalKind.Pragma:
        return this.rewritePragma(node);

      case NonterminalKind.PragmaDirective:
        return this.rewritePragmaDirective(node);

      case NonterminalKind.PrefixExpression:
        return this.rewritePrefixExpression(node);

      case NonterminalKind.ReceiveFunctionAttribute:
        return this.rewriteReceiveFunctionAttribute(node);

      case NonterminalKind.ReceiveFunctionAttributes:
        return this.rewriteReceiveFunctionAttributes(node);

      case NonterminalKind.ReceiveFunctionDefinition:
        return this.rewriteReceiveFunctionDefinition(node);

      case NonterminalKind.ReturnStatement:
        return this.rewriteReturnStatement(node);

      case NonterminalKind.ReturnsDeclaration:
        return this.rewriteReturnsDeclaration(node);

      case NonterminalKind.RevertStatement:
        return this.rewriteRevertStatement(node);

      case NonterminalKind.ShiftExpression:
        return this.rewriteShiftExpression(node);

      case NonterminalKind.SimpleVersionLiteral:
        return this.rewriteSimpleVersionLiteral(node);

      case NonterminalKind.SourceUnit:
        return this.rewriteSourceUnit(node);

      case NonterminalKind.SourceUnitMember:
        return this.rewriteSourceUnitMember(node);

      case NonterminalKind.SourceUnitMembers:
        return this.rewriteSourceUnitMembers(node);

      case NonterminalKind.StateVariableAttribute:
        return this.rewriteStateVariableAttribute(node);

      case NonterminalKind.StateVariableAttributes:
        return this.rewriteStateVariableAttributes(node);

      case NonterminalKind.StateVariableDefinition:
        return this.rewriteStateVariableDefinition(node);

      case NonterminalKind.StateVariableDefinitionValue:
        return this.rewriteStateVariableDefinitionValue(node);

      case NonterminalKind.Statement:
        return this.rewriteStatement(node);

      case NonterminalKind.Statements:
        return this.rewriteStatements(node);

      case NonterminalKind.StorageLayoutSpecifier:
        return this.rewriteStorageLayoutSpecifier(node);

      case NonterminalKind.StorageLocation:
        return this.rewriteStorageLocation(node);

      case NonterminalKind.StringExpression:
        return this.rewriteStringExpression(node);

      case NonterminalKind.StringLiteral:
        return this.rewriteStringLiteral(node);

      case NonterminalKind.StringLiterals:
        return this.rewriteStringLiterals(node);

      case NonterminalKind.StructDefinition:
        return this.rewriteStructDefinition(node);

      case NonterminalKind.StructMember:
        return this.rewriteStructMember(node);

      case NonterminalKind.StructMembers:
        return this.rewriteStructMembers(node);

      case NonterminalKind.ThrowStatement:
        return this.rewriteThrowStatement(node);

      case NonterminalKind.TryStatement:
        return this.rewriteTryStatement(node);

      case NonterminalKind.TupleDeconstructionElement:
        return this.rewriteTupleDeconstructionElement(node);

      case NonterminalKind.TupleDeconstructionElements:
        return this.rewriteTupleDeconstructionElements(node);

      case NonterminalKind.TupleDeconstructionStatement:
        return this.rewriteTupleDeconstructionStatement(node);

      case NonterminalKind.TupleExpression:
        return this.rewriteTupleExpression(node);

      case NonterminalKind.TupleMember:
        return this.rewriteTupleMember(node);

      case NonterminalKind.TupleValue:
        return this.rewriteTupleValue(node);

      case NonterminalKind.TupleValues:
        return this.rewriteTupleValues(node);

      case NonterminalKind.TypeExpression:
        return this.rewriteTypeExpression(node);

      case NonterminalKind.TypeName:
        return this.rewriteTypeName(node);

      case NonterminalKind.TypedTupleMember:
        return this.rewriteTypedTupleMember(node);

      case NonterminalKind.UncheckedBlock:
        return this.rewriteUncheckedBlock(node);

      case NonterminalKind.UnicodeStringLiteral:
        return this.rewriteUnicodeStringLiteral(node);

      case NonterminalKind.UnicodeStringLiterals:
        return this.rewriteUnicodeStringLiterals(node);

      case NonterminalKind.UnnamedFunctionAttribute:
        return this.rewriteUnnamedFunctionAttribute(node);

      case NonterminalKind.UnnamedFunctionAttributes:
        return this.rewriteUnnamedFunctionAttributes(node);

      case NonterminalKind.UnnamedFunctionDefinition:
        return this.rewriteUnnamedFunctionDefinition(node);

      case NonterminalKind.UntypedTupleMember:
        return this.rewriteUntypedTupleMember(node);

      case NonterminalKind.UserDefinedValueTypeDefinition:
        return this.rewriteUserDefinedValueTypeDefinition(node);

      case NonterminalKind.UsingAlias:
        return this.rewriteUsingAlias(node);

      case NonterminalKind.UsingClause:
        return this.rewriteUsingClause(node);

      case NonterminalKind.UsingDeconstruction:
        return this.rewriteUsingDeconstruction(node);

      case NonterminalKind.UsingDeconstructionSymbol:
        return this.rewriteUsingDeconstructionSymbol(node);

      case NonterminalKind.UsingDeconstructionSymbols:
        return this.rewriteUsingDeconstructionSymbols(node);

      case NonterminalKind.UsingDirective:
        return this.rewriteUsingDirective(node);

      case NonterminalKind.UsingOperator:
        return this.rewriteUsingOperator(node);

      case NonterminalKind.UsingTarget:
        return this.rewriteUsingTarget(node);

      case NonterminalKind.VariableDeclarationStatement:
        return this.rewriteVariableDeclarationStatement(node);

      case NonterminalKind.VariableDeclarationType:
        return this.rewriteVariableDeclarationType(node);

      case NonterminalKind.VariableDeclarationValue:
        return this.rewriteVariableDeclarationValue(node);

      case NonterminalKind.VersionExpression:
        return this.rewriteVersionExpression(node);

      case NonterminalKind.VersionExpressionSet:
        return this.rewriteVersionExpressionSet(node);

      case NonterminalKind.VersionExpressionSets:
        return this.rewriteVersionExpressionSets(node);

      case NonterminalKind.VersionLiteral:
        return this.rewriteVersionLiteral(node);

      case NonterminalKind.VersionOperator:
        return this.rewriteVersionOperator(node);

      case NonterminalKind.VersionPragma:
        return this.rewriteVersionPragma(node);

      case NonterminalKind.VersionRange:
        return this.rewriteVersionRange(node);

      case NonterminalKind.VersionTerm:
        return this.rewriteVersionTerm(node);

      case NonterminalKind.WhileStatement:
        return this.rewriteWhileStatement(node);

      case NonterminalKind.YulArguments:
        return this.rewriteYulArguments(node);

      case NonterminalKind.YulAssignmentOperator:
        return this.rewriteYulAssignmentOperator(node);

      case NonterminalKind.YulBlock:
        return this.rewriteYulBlock(node);

      case NonterminalKind.YulBreakStatement:
        return this.rewriteYulBreakStatement(node);

      case NonterminalKind.YulColonAndEqual:
        return this.rewriteYulColonAndEqual(node);

      case NonterminalKind.YulContinueStatement:
        return this.rewriteYulContinueStatement(node);

      case NonterminalKind.YulDefaultCase:
        return this.rewriteYulDefaultCase(node);

      case NonterminalKind.YulEqualAndColon:
        return this.rewriteYulEqualAndColon(node);

      case NonterminalKind.YulExpression:
        return this.rewriteYulExpression(node);

      case NonterminalKind.YulForStatement:
        return this.rewriteYulForStatement(node);

      case NonterminalKind.YulFunctionCallExpression:
        return this.rewriteYulFunctionCallExpression(node);

      case NonterminalKind.YulFunctionDefinition:
        return this.rewriteYulFunctionDefinition(node);

      case NonterminalKind.YulIfStatement:
        return this.rewriteYulIfStatement(node);

      case NonterminalKind.YulLabel:
        return this.rewriteYulLabel(node);

      case NonterminalKind.YulLeaveStatement:
        return this.rewriteYulLeaveStatement(node);

      case NonterminalKind.YulLiteral:
        return this.rewriteYulLiteral(node);

      case NonterminalKind.YulParameters:
        return this.rewriteYulParameters(node);

      case NonterminalKind.YulParametersDeclaration:
        return this.rewriteYulParametersDeclaration(node);

      case NonterminalKind.YulPath:
        return this.rewriteYulPath(node);

      case NonterminalKind.YulPaths:
        return this.rewriteYulPaths(node);

      case NonterminalKind.YulReturnsDeclaration:
        return this.rewriteYulReturnsDeclaration(node);

      case NonterminalKind.YulStackAssignmentOperator:
        return this.rewriteYulStackAssignmentOperator(node);

      case NonterminalKind.YulStackAssignmentStatement:
        return this.rewriteYulStackAssignmentStatement(node);

      case NonterminalKind.YulStatement:
        return this.rewriteYulStatement(node);

      case NonterminalKind.YulStatements:
        return this.rewriteYulStatements(node);

      case NonterminalKind.YulSwitchCase:
        return this.rewriteYulSwitchCase(node);

      case NonterminalKind.YulSwitchCases:
        return this.rewriteYulSwitchCases(node);

      case NonterminalKind.YulSwitchStatement:
        return this.rewriteYulSwitchStatement(node);

      case NonterminalKind.YulValueCase:
        return this.rewriteYulValueCase(node);

      case NonterminalKind.YulVariableAssignmentStatement:
        return this.rewriteYulVariableAssignmentStatement(node);

      case NonterminalKind.YulVariableDeclarationStatement:
        return this.rewriteYulVariableDeclarationStatement(node);

      case NonterminalKind.YulVariableDeclarationValue:
        return this.rewriteYulVariableDeclarationValue(node);

      case NonterminalKind.YulVariableNames:
        return this.rewriteYulVariableNames(node);
    }
  }

  /** Rewrites a terminal node. Typically called from `rewriteNode`. */
  public rewriteTerminalNode(node: TerminalNode): Node | undefined {
    switch (node.kind) {
      case TerminalKind.ABIEncoderV2Keyword:
        return this.rewriteABIEncoderV2Keyword(node);

      case TerminalKind.AbicoderKeyword:
        return this.rewriteAbicoderKeyword(node);

      case TerminalKind.AbicoderV1Keyword:
        return this.rewriteAbicoderV1Keyword(node);

      case TerminalKind.AbicoderV2Keyword:
        return this.rewriteAbicoderV2Keyword(node);

      case TerminalKind.AbstractKeyword:
        return this.rewriteAbstractKeyword(node);

      case TerminalKind.AddressKeyword:
        return this.rewriteAddressKeyword(node);

      case TerminalKind.AfterKeyword:
        return this.rewriteAfterKeyword(node);

      case TerminalKind.AliasKeyword:
        return this.rewriteAliasKeyword(node);

      case TerminalKind.Ampersand:
        return this.rewriteAmpersand(node);

      case TerminalKind.AmpersandAmpersand:
        return this.rewriteAmpersandAmpersand(node);

      case TerminalKind.AmpersandEqual:
        return this.rewriteAmpersandEqual(node);

      case TerminalKind.AnonymousKeyword:
        return this.rewriteAnonymousKeyword(node);

      case TerminalKind.ApplyKeyword:
        return this.rewriteApplyKeyword(node);

      case TerminalKind.AsKeyword:
        return this.rewriteAsKeyword(node);

      case TerminalKind.AssemblyKeyword:
        return this.rewriteAssemblyKeyword(node);

      case TerminalKind.Asterisk:
        return this.rewriteAsterisk(node);

      case TerminalKind.AsteriskAsterisk:
        return this.rewriteAsteriskAsterisk(node);

      case TerminalKind.AsteriskEqual:
        return this.rewriteAsteriskEqual(node);

      case TerminalKind.AtKeyword:
        return this.rewriteAtKeyword(node);

      case TerminalKind.AutoKeyword:
        return this.rewriteAutoKeyword(node);

      case TerminalKind.Bang:
        return this.rewriteBang(node);

      case TerminalKind.BangEqual:
        return this.rewriteBangEqual(node);

      case TerminalKind.Bar:
        return this.rewriteBar(node);

      case TerminalKind.BarBar:
        return this.rewriteBarBar(node);

      case TerminalKind.BarEqual:
        return this.rewriteBarEqual(node);

      case TerminalKind.BoolKeyword:
        return this.rewriteBoolKeyword(node);

      case TerminalKind.BreakKeyword:
        return this.rewriteBreakKeyword(node);

      case TerminalKind.ByteKeyword:
        return this.rewriteByteKeyword(node);

      case TerminalKind.BytesKeyword:
        return this.rewriteBytesKeyword(node);

      case TerminalKind.CallDataKeyword:
        return this.rewriteCallDataKeyword(node);

      case TerminalKind.Caret:
        return this.rewriteCaret(node);

      case TerminalKind.CaretEqual:
        return this.rewriteCaretEqual(node);

      case TerminalKind.CaseKeyword:
        return this.rewriteCaseKeyword(node);

      case TerminalKind.CatchKeyword:
        return this.rewriteCatchKeyword(node);

      case TerminalKind.CloseBrace:
        return this.rewriteCloseBrace(node);

      case TerminalKind.CloseBracket:
        return this.rewriteCloseBracket(node);

      case TerminalKind.CloseParen:
        return this.rewriteCloseParen(node);

      case TerminalKind.Colon:
        return this.rewriteColon(node);

      case TerminalKind.ColonEqual:
        return this.rewriteColonEqual(node);

      case TerminalKind.Comma:
        return this.rewriteComma(node);

      case TerminalKind.ConstantKeyword:
        return this.rewriteConstantKeyword(node);

      case TerminalKind.ConstructorKeyword:
        return this.rewriteConstructorKeyword(node);

      case TerminalKind.ContinueKeyword:
        return this.rewriteContinueKeyword(node);

      case TerminalKind.ContractKeyword:
        return this.rewriteContractKeyword(node);

      case TerminalKind.CopyOfKeyword:
        return this.rewriteCopyOfKeyword(node);

      case TerminalKind.DaysKeyword:
        return this.rewriteDaysKeyword(node);

      case TerminalKind.DecimalLiteral:
        return this.rewriteDecimalLiteral(node);

      case TerminalKind.DefaultKeyword:
        return this.rewriteDefaultKeyword(node);

      case TerminalKind.DefineKeyword:
        return this.rewriteDefineKeyword(node);

      case TerminalKind.DeleteKeyword:
        return this.rewriteDeleteKeyword(node);

      case TerminalKind.DoKeyword:
        return this.rewriteDoKeyword(node);

      case TerminalKind.DoubleQuotedHexStringLiteral:
        return this.rewriteDoubleQuotedHexStringLiteral(node);

      case TerminalKind.DoubleQuotedStringLiteral:
        return this.rewriteDoubleQuotedStringLiteral(node);

      case TerminalKind.DoubleQuotedUnicodeStringLiteral:
        return this.rewriteDoubleQuotedUnicodeStringLiteral(node);

      case TerminalKind.DoubleQuotedVersionLiteral:
        return this.rewriteDoubleQuotedVersionLiteral(node);

      case TerminalKind.ElseKeyword:
        return this.rewriteElseKeyword(node);

      case TerminalKind.EmitKeyword:
        return this.rewriteEmitKeyword(node);

      case TerminalKind.EndOfLine:
        return this.rewriteEndOfLine(node);

      case TerminalKind.EnumKeyword:
        return this.rewriteEnumKeyword(node);

      case TerminalKind.Equal:
        return this.rewriteEqual(node);

      case TerminalKind.EqualColon:
        return this.rewriteEqualColon(node);

      case TerminalKind.EqualEqual:
        return this.rewriteEqualEqual(node);

      case TerminalKind.EqualGreaterThan:
        return this.rewriteEqualGreaterThan(node);

      case TerminalKind.ErrorKeyword:
        return this.rewriteErrorKeyword(node);

      case TerminalKind.EtherKeyword:
        return this.rewriteEtherKeyword(node);

      case TerminalKind.EventKeyword:
        return this.rewriteEventKeyword(node);

      case TerminalKind.ExperimentalKeyword:
        return this.rewriteExperimentalKeyword(node);

      case TerminalKind.ExternalKeyword:
        return this.rewriteExternalKeyword(node);

      case TerminalKind.FallbackKeyword:
        return this.rewriteFallbackKeyword(node);

      case TerminalKind.FalseKeyword:
        return this.rewriteFalseKeyword(node);

      case TerminalKind.FinalKeyword:
        return this.rewriteFinalKeyword(node);

      case TerminalKind.FinneyKeyword:
        return this.rewriteFinneyKeyword(node);

      case TerminalKind.FixedKeyword:
        return this.rewriteFixedKeyword(node);

      case TerminalKind.ForKeyword:
        return this.rewriteForKeyword(node);

      case TerminalKind.FromKeyword:
        return this.rewriteFromKeyword(node);

      case TerminalKind.FunctionKeyword:
        return this.rewriteFunctionKeyword(node);

      case TerminalKind.GlobalKeyword:
        return this.rewriteGlobalKeyword(node);

      case TerminalKind.GreaterThan:
        return this.rewriteGreaterThan(node);

      case TerminalKind.GreaterThanEqual:
        return this.rewriteGreaterThanEqual(node);

      case TerminalKind.GreaterThanGreaterThan:
        return this.rewriteGreaterThanGreaterThan(node);

      case TerminalKind.GreaterThanGreaterThanEqual:
        return this.rewriteGreaterThanGreaterThanEqual(node);

      case TerminalKind.GreaterThanGreaterThanGreaterThan:
        return this.rewriteGreaterThanGreaterThanGreaterThan(node);

      case TerminalKind.GreaterThanGreaterThanGreaterThanEqual:
        return this.rewriteGreaterThanGreaterThanGreaterThanEqual(node);

      case TerminalKind.GweiKeyword:
        return this.rewriteGweiKeyword(node);

      case TerminalKind.HexKeyword:
        return this.rewriteHexKeyword(node);

      case TerminalKind.HexLiteral:
        return this.rewriteHexLiteral(node);

      case TerminalKind.HoursKeyword:
        return this.rewriteHoursKeyword(node);

      case TerminalKind.Identifier:
        return this.rewriteIdentifier(node);

      case TerminalKind.IfKeyword:
        return this.rewriteIfKeyword(node);

      case TerminalKind.ImmutableKeyword:
        return this.rewriteImmutableKeyword(node);

      case TerminalKind.ImplementsKeyword:
        return this.rewriteImplementsKeyword(node);

      case TerminalKind.ImportKeyword:
        return this.rewriteImportKeyword(node);

      case TerminalKind.InKeyword:
        return this.rewriteInKeyword(node);

      case TerminalKind.IndexedKeyword:
        return this.rewriteIndexedKeyword(node);

      case TerminalKind.InlineKeyword:
        return this.rewriteInlineKeyword(node);

      case TerminalKind.IntKeyword:
        return this.rewriteIntKeyword(node);

      case TerminalKind.InterfaceKeyword:
        return this.rewriteInterfaceKeyword(node);

      case TerminalKind.InternalKeyword:
        return this.rewriteInternalKeyword(node);

      case TerminalKind.IsKeyword:
        return this.rewriteIsKeyword(node);

      case TerminalKind.LayoutKeyword:
        return this.rewriteLayoutKeyword(node);

      case TerminalKind.LessThan:
        return this.rewriteLessThan(node);

      case TerminalKind.LessThanEqual:
        return this.rewriteLessThanEqual(node);

      case TerminalKind.LessThanLessThan:
        return this.rewriteLessThanLessThan(node);

      case TerminalKind.LessThanLessThanEqual:
        return this.rewriteLessThanLessThanEqual(node);

      case TerminalKind.LetKeyword:
        return this.rewriteLetKeyword(node);

      case TerminalKind.LibraryKeyword:
        return this.rewriteLibraryKeyword(node);

      case TerminalKind.MacroKeyword:
        return this.rewriteMacroKeyword(node);

      case TerminalKind.MappingKeyword:
        return this.rewriteMappingKeyword(node);

      case TerminalKind.MatchKeyword:
        return this.rewriteMatchKeyword(node);

      case TerminalKind.MemoryKeyword:
        return this.rewriteMemoryKeyword(node);

      case TerminalKind.Minus:
        return this.rewriteMinus(node);

      case TerminalKind.MinusEqual:
        return this.rewriteMinusEqual(node);

      case TerminalKind.MinusGreaterThan:
        return this.rewriteMinusGreaterThan(node);

      case TerminalKind.MinusMinus:
        return this.rewriteMinusMinus(node);

      case TerminalKind.MinutesKeyword:
        return this.rewriteMinutesKeyword(node);

      case TerminalKind.ModifierKeyword:
        return this.rewriteModifierKeyword(node);

      case TerminalKind.MultiLineComment:
        return this.rewriteMultiLineComment(node);

      case TerminalKind.MultiLineNatSpecComment:
        return this.rewriteMultiLineNatSpecComment(node);

      case TerminalKind.MutableKeyword:
        return this.rewriteMutableKeyword(node);

      case TerminalKind.NewKeyword:
        return this.rewriteNewKeyword(node);

      case TerminalKind.NullKeyword:
        return this.rewriteNullKeyword(node);

      case TerminalKind.OfKeyword:
        return this.rewriteOfKeyword(node);

      case TerminalKind.OpenBrace:
        return this.rewriteOpenBrace(node);

      case TerminalKind.OpenBracket:
        return this.rewriteOpenBracket(node);

      case TerminalKind.OpenParen:
        return this.rewriteOpenParen(node);

      case TerminalKind.OverrideKeyword:
        return this.rewriteOverrideKeyword(node);

      case TerminalKind.PartialKeyword:
        return this.rewritePartialKeyword(node);

      case TerminalKind.PayableKeyword:
        return this.rewritePayableKeyword(node);

      case TerminalKind.Percent:
        return this.rewritePercent(node);

      case TerminalKind.PercentEqual:
        return this.rewritePercentEqual(node);

      case TerminalKind.Period:
        return this.rewritePeriod(node);

      case TerminalKind.Plus:
        return this.rewritePlus(node);

      case TerminalKind.PlusEqual:
        return this.rewritePlusEqual(node);

      case TerminalKind.PlusPlus:
        return this.rewritePlusPlus(node);

      case TerminalKind.PragmaKeyword:
        return this.rewritePragmaKeyword(node);

      case TerminalKind.PrivateKeyword:
        return this.rewritePrivateKeyword(node);

      case TerminalKind.PromiseKeyword:
        return this.rewritePromiseKeyword(node);

      case TerminalKind.PublicKeyword:
        return this.rewritePublicKeyword(node);

      case TerminalKind.PureKeyword:
        return this.rewritePureKeyword(node);

      case TerminalKind.QuestionMark:
        return this.rewriteQuestionMark(node);

      case TerminalKind.ReceiveKeyword:
        return this.rewriteReceiveKeyword(node);

      case TerminalKind.ReferenceKeyword:
        return this.rewriteReferenceKeyword(node);

      case TerminalKind.RelocatableKeyword:
        return this.rewriteRelocatableKeyword(node);

      case TerminalKind.ReturnKeyword:
        return this.rewriteReturnKeyword(node);

      case TerminalKind.ReturnsKeyword:
        return this.rewriteReturnsKeyword(node);

      case TerminalKind.RevertKeyword:
        return this.rewriteRevertKeyword(node);

      case TerminalKind.SMTCheckerKeyword:
        return this.rewriteSMTCheckerKeyword(node);

      case TerminalKind.SealedKeyword:
        return this.rewriteSealedKeyword(node);

      case TerminalKind.SecondsKeyword:
        return this.rewriteSecondsKeyword(node);

      case TerminalKind.Semicolon:
        return this.rewriteSemicolon(node);

      case TerminalKind.SingleLineComment:
        return this.rewriteSingleLineComment(node);

      case TerminalKind.SingleLineNatSpecComment:
        return this.rewriteSingleLineNatSpecComment(node);

      case TerminalKind.SingleQuotedHexStringLiteral:
        return this.rewriteSingleQuotedHexStringLiteral(node);

      case TerminalKind.SingleQuotedStringLiteral:
        return this.rewriteSingleQuotedStringLiteral(node);

      case TerminalKind.SingleQuotedUnicodeStringLiteral:
        return this.rewriteSingleQuotedUnicodeStringLiteral(node);

      case TerminalKind.SingleQuotedVersionLiteral:
        return this.rewriteSingleQuotedVersionLiteral(node);

      case TerminalKind.SizeOfKeyword:
        return this.rewriteSizeOfKeyword(node);

      case TerminalKind.Slash:
        return this.rewriteSlash(node);

      case TerminalKind.SlashEqual:
        return this.rewriteSlashEqual(node);

      case TerminalKind.SolidityKeyword:
        return this.rewriteSolidityKeyword(node);

      case TerminalKind.StaticKeyword:
        return this.rewriteStaticKeyword(node);

      case TerminalKind.StorageKeyword:
        return this.rewriteStorageKeyword(node);

      case TerminalKind.StringKeyword:
        return this.rewriteStringKeyword(node);

      case TerminalKind.StructKeyword:
        return this.rewriteStructKeyword(node);

      case TerminalKind.SuperKeyword:
        return this.rewriteSuperKeyword(node);

      case TerminalKind.SupportsKeyword:
        return this.rewriteSupportsKeyword(node);

      case TerminalKind.SwitchKeyword:
        return this.rewriteSwitchKeyword(node);

      case TerminalKind.SzaboKeyword:
        return this.rewriteSzaboKeyword(node);

      case TerminalKind.ThisKeyword:
        return this.rewriteThisKeyword(node);

      case TerminalKind.ThrowKeyword:
        return this.rewriteThrowKeyword(node);

      case TerminalKind.Tilde:
        return this.rewriteTilde(node);

      case TerminalKind.TransientKeyword:
        return this.rewriteTransientKeyword(node);

      case TerminalKind.TrueKeyword:
        return this.rewriteTrueKeyword(node);

      case TerminalKind.TryKeyword:
        return this.rewriteTryKeyword(node);

      case TerminalKind.TypeDefKeyword:
        return this.rewriteTypeDefKeyword(node);

      case TerminalKind.TypeKeyword:
        return this.rewriteTypeKeyword(node);

      case TerminalKind.TypeOfKeyword:
        return this.rewriteTypeOfKeyword(node);

      case TerminalKind.UfixedKeyword:
        return this.rewriteUfixedKeyword(node);

      case TerminalKind.UintKeyword:
        return this.rewriteUintKeyword(node);

      case TerminalKind.UncheckedKeyword:
        return this.rewriteUncheckedKeyword(node);

      case TerminalKind.UsingKeyword:
        return this.rewriteUsingKeyword(node);

      case TerminalKind.VarKeyword:
        return this.rewriteVarKeyword(node);

      case TerminalKind.VersionSpecifier:
        return this.rewriteVersionSpecifier(node);

      case TerminalKind.ViewKeyword:
        return this.rewriteViewKeyword(node);

      case TerminalKind.VirtualKeyword:
        return this.rewriteVirtualKeyword(node);

      case TerminalKind.WeeksKeyword:
        return this.rewriteWeeksKeyword(node);

      case TerminalKind.WeiKeyword:
        return this.rewriteWeiKeyword(node);

      case TerminalKind.WhileKeyword:
        return this.rewriteWhileKeyword(node);

      case TerminalKind.Whitespace:
        return this.rewriteWhitespace(node);

      case TerminalKind.YearsKeyword:
        return this.rewriteYearsKeyword(node);

      case TerminalKind.YulAbstractKeyword:
        return this.rewriteYulAbstractKeyword(node);

      case TerminalKind.YulAfterKeyword:
        return this.rewriteYulAfterKeyword(node);

      case TerminalKind.YulAliasKeyword:
        return this.rewriteYulAliasKeyword(node);

      case TerminalKind.YulAnonymousKeyword:
        return this.rewriteYulAnonymousKeyword(node);

      case TerminalKind.YulApplyKeyword:
        return this.rewriteYulApplyKeyword(node);

      case TerminalKind.YulAsKeyword:
        return this.rewriteYulAsKeyword(node);

      case TerminalKind.YulAssemblyKeyword:
        return this.rewriteYulAssemblyKeyword(node);

      case TerminalKind.YulAutoKeyword:
        return this.rewriteYulAutoKeyword(node);

      case TerminalKind.YulBoolKeyword:
        return this.rewriteYulBoolKeyword(node);

      case TerminalKind.YulBreakKeyword:
        return this.rewriteYulBreakKeyword(node);

      case TerminalKind.YulBytesKeyword:
        return this.rewriteYulBytesKeyword(node);

      case TerminalKind.YulCallDataKeyword:
        return this.rewriteYulCallDataKeyword(node);

      case TerminalKind.YulCaseKeyword:
        return this.rewriteYulCaseKeyword(node);

      case TerminalKind.YulCatchKeyword:
        return this.rewriteYulCatchKeyword(node);

      case TerminalKind.YulConstantKeyword:
        return this.rewriteYulConstantKeyword(node);

      case TerminalKind.YulConstructorKeyword:
        return this.rewriteYulConstructorKeyword(node);

      case TerminalKind.YulContinueKeyword:
        return this.rewriteYulContinueKeyword(node);

      case TerminalKind.YulContractKeyword:
        return this.rewriteYulContractKeyword(node);

      case TerminalKind.YulCopyOfKeyword:
        return this.rewriteYulCopyOfKeyword(node);

      case TerminalKind.YulDaysKeyword:
        return this.rewriteYulDaysKeyword(node);

      case TerminalKind.YulDecimalLiteral:
        return this.rewriteYulDecimalLiteral(node);

      case TerminalKind.YulDefaultKeyword:
        return this.rewriteYulDefaultKeyword(node);

      case TerminalKind.YulDefineKeyword:
        return this.rewriteYulDefineKeyword(node);

      case TerminalKind.YulDeleteKeyword:
        return this.rewriteYulDeleteKeyword(node);

      case TerminalKind.YulDoKeyword:
        return this.rewriteYulDoKeyword(node);

      case TerminalKind.YulElseKeyword:
        return this.rewriteYulElseKeyword(node);

      case TerminalKind.YulEmitKeyword:
        return this.rewriteYulEmitKeyword(node);

      case TerminalKind.YulEnumKeyword:
        return this.rewriteYulEnumKeyword(node);

      case TerminalKind.YulEtherKeyword:
        return this.rewriteYulEtherKeyword(node);

      case TerminalKind.YulEventKeyword:
        return this.rewriteYulEventKeyword(node);

      case TerminalKind.YulExternalKeyword:
        return this.rewriteYulExternalKeyword(node);

      case TerminalKind.YulFallbackKeyword:
        return this.rewriteYulFallbackKeyword(node);

      case TerminalKind.YulFalseKeyword:
        return this.rewriteYulFalseKeyword(node);

      case TerminalKind.YulFinalKeyword:
        return this.rewriteYulFinalKeyword(node);

      case TerminalKind.YulFinneyKeyword:
        return this.rewriteYulFinneyKeyword(node);

      case TerminalKind.YulFixedKeyword:
        return this.rewriteYulFixedKeyword(node);

      case TerminalKind.YulForKeyword:
        return this.rewriteYulForKeyword(node);

      case TerminalKind.YulFunctionKeyword:
        return this.rewriteYulFunctionKeyword(node);

      case TerminalKind.YulGweiKeyword:
        return this.rewriteYulGweiKeyword(node);

      case TerminalKind.YulHexKeyword:
        return this.rewriteYulHexKeyword(node);

      case TerminalKind.YulHexLiteral:
        return this.rewriteYulHexLiteral(node);

      case TerminalKind.YulHoursKeyword:
        return this.rewriteYulHoursKeyword(node);

      case TerminalKind.YulIdentifier:
        return this.rewriteYulIdentifier(node);

      case TerminalKind.YulIfKeyword:
        return this.rewriteYulIfKeyword(node);

      case TerminalKind.YulImmutableKeyword:
        return this.rewriteYulImmutableKeyword(node);

      case TerminalKind.YulImplementsKeyword:
        return this.rewriteYulImplementsKeyword(node);

      case TerminalKind.YulImportKeyword:
        return this.rewriteYulImportKeyword(node);

      case TerminalKind.YulInKeyword:
        return this.rewriteYulInKeyword(node);

      case TerminalKind.YulIndexedKeyword:
        return this.rewriteYulIndexedKeyword(node);

      case TerminalKind.YulInlineKeyword:
        return this.rewriteYulInlineKeyword(node);

      case TerminalKind.YulIntKeyword:
        return this.rewriteYulIntKeyword(node);

      case TerminalKind.YulInterfaceKeyword:
        return this.rewriteYulInterfaceKeyword(node);

      case TerminalKind.YulInternalKeyword:
        return this.rewriteYulInternalKeyword(node);

      case TerminalKind.YulIsKeyword:
        return this.rewriteYulIsKeyword(node);

      case TerminalKind.YulLeaveKeyword:
        return this.rewriteYulLeaveKeyword(node);

      case TerminalKind.YulLetKeyword:
        return this.rewriteYulLetKeyword(node);

      case TerminalKind.YulLibraryKeyword:
        return this.rewriteYulLibraryKeyword(node);

      case TerminalKind.YulMacroKeyword:
        return this.rewriteYulMacroKeyword(node);

      case TerminalKind.YulMappingKeyword:
        return this.rewriteYulMappingKeyword(node);

      case TerminalKind.YulMatchKeyword:
        return this.rewriteYulMatchKeyword(node);

      case TerminalKind.YulMemoryKeyword:
        return this.rewriteYulMemoryKeyword(node);

      case TerminalKind.YulMinutesKeyword:
        return this.rewriteYulMinutesKeyword(node);

      case TerminalKind.YulModifierKeyword:
        return this.rewriteYulModifierKeyword(node);

      case TerminalKind.YulMutableKeyword:
        return this.rewriteYulMutableKeyword(node);

      case TerminalKind.YulNewKeyword:
        return this.rewriteYulNewKeyword(node);

      case TerminalKind.YulNullKeyword:
        return this.rewriteYulNullKeyword(node);

      case TerminalKind.YulOfKeyword:
        return this.rewriteYulOfKeyword(node);

      case TerminalKind.YulOverrideKeyword:
        return this.rewriteYulOverrideKeyword(node);

      case TerminalKind.YulPartialKeyword:
        return this.rewriteYulPartialKeyword(node);

      case TerminalKind.YulPayableKeyword:
        return this.rewriteYulPayableKeyword(node);

      case TerminalKind.YulPragmaKeyword:
        return this.rewriteYulPragmaKeyword(node);

      case TerminalKind.YulPrivateKeyword:
        return this.rewriteYulPrivateKeyword(node);

      case TerminalKind.YulPromiseKeyword:
        return this.rewriteYulPromiseKeyword(node);

      case TerminalKind.YulPublicKeyword:
        return this.rewriteYulPublicKeyword(node);

      case TerminalKind.YulPureKeyword:
        return this.rewriteYulPureKeyword(node);

      case TerminalKind.YulReceiveKeyword:
        return this.rewriteYulReceiveKeyword(node);

      case TerminalKind.YulReferenceKeyword:
        return this.rewriteYulReferenceKeyword(node);

      case TerminalKind.YulRelocatableKeyword:
        return this.rewriteYulRelocatableKeyword(node);

      case TerminalKind.YulReturnsKeyword:
        return this.rewriteYulReturnsKeyword(node);

      case TerminalKind.YulSealedKeyword:
        return this.rewriteYulSealedKeyword(node);

      case TerminalKind.YulSecondsKeyword:
        return this.rewriteYulSecondsKeyword(node);

      case TerminalKind.YulSizeOfKeyword:
        return this.rewriteYulSizeOfKeyword(node);

      case TerminalKind.YulStaticKeyword:
        return this.rewriteYulStaticKeyword(node);

      case TerminalKind.YulStorageKeyword:
        return this.rewriteYulStorageKeyword(node);

      case TerminalKind.YulStringKeyword:
        return this.rewriteYulStringKeyword(node);

      case TerminalKind.YulStructKeyword:
        return this.rewriteYulStructKeyword(node);

      case TerminalKind.YulSuperKeyword:
        return this.rewriteYulSuperKeyword(node);

      case TerminalKind.YulSupportsKeyword:
        return this.rewriteYulSupportsKeyword(node);

      case TerminalKind.YulSwitchKeyword:
        return this.rewriteYulSwitchKeyword(node);

      case TerminalKind.YulSzaboKeyword:
        return this.rewriteYulSzaboKeyword(node);

      case TerminalKind.YulThisKeyword:
        return this.rewriteYulThisKeyword(node);

      case TerminalKind.YulThrowKeyword:
        return this.rewriteYulThrowKeyword(node);

      case TerminalKind.YulTrueKeyword:
        return this.rewriteYulTrueKeyword(node);

      case TerminalKind.YulTryKeyword:
        return this.rewriteYulTryKeyword(node);

      case TerminalKind.YulTypeDefKeyword:
        return this.rewriteYulTypeDefKeyword(node);

      case TerminalKind.YulTypeKeyword:
        return this.rewriteYulTypeKeyword(node);

      case TerminalKind.YulTypeOfKeyword:
        return this.rewriteYulTypeOfKeyword(node);

      case TerminalKind.YulUfixedKeyword:
        return this.rewriteYulUfixedKeyword(node);

      case TerminalKind.YulUintKeyword:
        return this.rewriteYulUintKeyword(node);

      case TerminalKind.YulUncheckedKeyword:
        return this.rewriteYulUncheckedKeyword(node);

      case TerminalKind.YulUsingKeyword:
        return this.rewriteYulUsingKeyword(node);

      case TerminalKind.YulVarKeyword:
        return this.rewriteYulVarKeyword(node);

      case TerminalKind.YulViewKeyword:
        return this.rewriteYulViewKeyword(node);

      case TerminalKind.YulVirtualKeyword:
        return this.rewriteYulVirtualKeyword(node);

      case TerminalKind.YulWeeksKeyword:
        return this.rewriteYulWeeksKeyword(node);

      case TerminalKind.YulWeiKeyword:
        return this.rewriteYulWeiKeyword(node);

      case TerminalKind.YulWhileKeyword:
        return this.rewriteYulWhileKeyword(node);

      case TerminalKind.YulYearsKeyword:
        return this.rewriteYulYearsKeyword(node);

      case TerminalKind.Unrecognized:
        return this.rewriteUnrecognized(node);
      case TerminalKind.Missing:
        return this.rewriteMissing(node);
    }
  }

  /** @virtual Rewrites a `AbicoderPragma` node, recursively traversing the children (unless overriden). */
  public rewriteAbicoderPragma(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `AbicoderVersion` node, recursively traversing the children (unless overriden). */
  public rewriteAbicoderVersion(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `AdditiveExpression` node, recursively traversing the children (unless overriden). */
  public rewriteAdditiveExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `AddressType` node, recursively traversing the children (unless overriden). */
  public rewriteAddressType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `AndExpression` node, recursively traversing the children (unless overriden). */
  public rewriteAndExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ArgumentsDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteArgumentsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ArrayExpression` node, recursively traversing the children (unless overriden). */
  public rewriteArrayExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ArrayTypeName` node, recursively traversing the children (unless overriden). */
  public rewriteArrayTypeName(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ArrayValues` node, recursively traversing the children (unless overriden). */
  public rewriteArrayValues(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `AssemblyFlags` node, recursively traversing the children (unless overriden). */
  public rewriteAssemblyFlags(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `AssemblyFlagsDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteAssemblyFlagsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `AssemblyStatement` node, recursively traversing the children (unless overriden). */
  public rewriteAssemblyStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `AssignmentExpression` node, recursively traversing the children (unless overriden). */
  public rewriteAssignmentExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `BitwiseAndExpression` node, recursively traversing the children (unless overriden). */
  public rewriteBitwiseAndExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `BitwiseOrExpression` node, recursively traversing the children (unless overriden). */
  public rewriteBitwiseOrExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `BitwiseXorExpression` node, recursively traversing the children (unless overriden). */
  public rewriteBitwiseXorExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `Block` node, recursively traversing the children (unless overriden). */
  public rewriteBlock(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `BreakStatement` node, recursively traversing the children (unless overriden). */
  public rewriteBreakStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `CallOptions` node, recursively traversing the children (unless overriden). */
  public rewriteCallOptions(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `CallOptionsExpression` node, recursively traversing the children (unless overriden). */
  public rewriteCallOptionsExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `CatchClause` node, recursively traversing the children (unless overriden). */
  public rewriteCatchClause(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `CatchClauseError` node, recursively traversing the children (unless overriden). */
  public rewriteCatchClauseError(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `CatchClauses` node, recursively traversing the children (unless overriden). */
  public rewriteCatchClauses(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ConditionalExpression` node, recursively traversing the children (unless overriden). */
  public rewriteConditionalExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ConstantDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteConstantDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ConstructorAttribute` node, recursively traversing the children (unless overriden). */
  public rewriteConstructorAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ConstructorAttributes` node, recursively traversing the children (unless overriden). */
  public rewriteConstructorAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ConstructorDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteConstructorDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ContinueStatement` node, recursively traversing the children (unless overriden). */
  public rewriteContinueStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ContractDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteContractDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ContractMember` node, recursively traversing the children (unless overriden). */
  public rewriteContractMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ContractMembers` node, recursively traversing the children (unless overriden). */
  public rewriteContractMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ContractSpecifier` node, recursively traversing the children (unless overriden). */
  public rewriteContractSpecifier(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ContractSpecifiers` node, recursively traversing the children (unless overriden). */
  public rewriteContractSpecifiers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `DecimalNumberExpression` node, recursively traversing the children (unless overriden). */
  public rewriteDecimalNumberExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `DoWhileStatement` node, recursively traversing the children (unless overriden). */
  public rewriteDoWhileStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ElementaryType` node, recursively traversing the children (unless overriden). */
  public rewriteElementaryType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ElseBranch` node, recursively traversing the children (unless overriden). */
  public rewriteElseBranch(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `EmitStatement` node, recursively traversing the children (unless overriden). */
  public rewriteEmitStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `EnumDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteEnumDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `EnumMembers` node, recursively traversing the children (unless overriden). */
  public rewriteEnumMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `EqualityExpression` node, recursively traversing the children (unless overriden). */
  public rewriteEqualityExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ErrorDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteErrorDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ErrorParameter` node, recursively traversing the children (unless overriden). */
  public rewriteErrorParameter(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ErrorParameters` node, recursively traversing the children (unless overriden). */
  public rewriteErrorParameters(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ErrorParametersDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteErrorParametersDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `EventDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteEventDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `EventParameter` node, recursively traversing the children (unless overriden). */
  public rewriteEventParameter(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `EventParameters` node, recursively traversing the children (unless overriden). */
  public rewriteEventParameters(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `EventParametersDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteEventParametersDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ExperimentalFeature` node, recursively traversing the children (unless overriden). */
  public rewriteExperimentalFeature(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ExperimentalPragma` node, recursively traversing the children (unless overriden). */
  public rewriteExperimentalPragma(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ExponentiationExpression` node, recursively traversing the children (unless overriden). */
  public rewriteExponentiationExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `Expression` node, recursively traversing the children (unless overriden). */
  public rewriteExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ExpressionStatement` node, recursively traversing the children (unless overriden). */
  public rewriteExpressionStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FallbackFunctionAttribute` node, recursively traversing the children (unless overriden). */
  public rewriteFallbackFunctionAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FallbackFunctionAttributes` node, recursively traversing the children (unless overriden). */
  public rewriteFallbackFunctionAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FallbackFunctionDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteFallbackFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ForStatement` node, recursively traversing the children (unless overriden). */
  public rewriteForStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ForStatementCondition` node, recursively traversing the children (unless overriden). */
  public rewriteForStatementCondition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ForStatementInitialization` node, recursively traversing the children (unless overriden). */
  public rewriteForStatementInitialization(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FunctionAttribute` node, recursively traversing the children (unless overriden). */
  public rewriteFunctionAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FunctionAttributes` node, recursively traversing the children (unless overriden). */
  public rewriteFunctionAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FunctionBody` node, recursively traversing the children (unless overriden). */
  public rewriteFunctionBody(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FunctionCallExpression` node, recursively traversing the children (unless overriden). */
  public rewriteFunctionCallExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FunctionDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FunctionName` node, recursively traversing the children (unless overriden). */
  public rewriteFunctionName(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FunctionType` node, recursively traversing the children (unless overriden). */
  public rewriteFunctionType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FunctionTypeAttribute` node, recursively traversing the children (unless overriden). */
  public rewriteFunctionTypeAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `FunctionTypeAttributes` node, recursively traversing the children (unless overriden). */
  public rewriteFunctionTypeAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `HexNumberExpression` node, recursively traversing the children (unless overriden). */
  public rewriteHexNumberExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `HexStringLiteral` node, recursively traversing the children (unless overriden). */
  public rewriteHexStringLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `HexStringLiterals` node, recursively traversing the children (unless overriden). */
  public rewriteHexStringLiterals(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `IdentifierPath` node, recursively traversing the children (unless overriden). */
  public rewriteIdentifierPath(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `IfStatement` node, recursively traversing the children (unless overriden). */
  public rewriteIfStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ImportAlias` node, recursively traversing the children (unless overriden). */
  public rewriteImportAlias(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ImportClause` node, recursively traversing the children (unless overriden). */
  public rewriteImportClause(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ImportDeconstruction` node, recursively traversing the children (unless overriden). */
  public rewriteImportDeconstruction(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ImportDeconstructionSymbol` node, recursively traversing the children (unless overriden). */
  public rewriteImportDeconstructionSymbol(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ImportDeconstructionSymbols` node, recursively traversing the children (unless overriden). */
  public rewriteImportDeconstructionSymbols(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ImportDirective` node, recursively traversing the children (unless overriden). */
  public rewriteImportDirective(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `IndexAccessEnd` node, recursively traversing the children (unless overriden). */
  public rewriteIndexAccessEnd(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `IndexAccessExpression` node, recursively traversing the children (unless overriden). */
  public rewriteIndexAccessExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `InequalityExpression` node, recursively traversing the children (unless overriden). */
  public rewriteInequalityExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `InheritanceSpecifier` node, recursively traversing the children (unless overriden). */
  public rewriteInheritanceSpecifier(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `InheritanceType` node, recursively traversing the children (unless overriden). */
  public rewriteInheritanceType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `InheritanceTypes` node, recursively traversing the children (unless overriden). */
  public rewriteInheritanceTypes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `InterfaceDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteInterfaceDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `InterfaceMembers` node, recursively traversing the children (unless overriden). */
  public rewriteInterfaceMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `LibraryDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteLibraryDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `LibraryMembers` node, recursively traversing the children (unless overriden). */
  public rewriteLibraryMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `MappingKey` node, recursively traversing the children (unless overriden). */
  public rewriteMappingKey(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `MappingKeyType` node, recursively traversing the children (unless overriden). */
  public rewriteMappingKeyType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `MappingType` node, recursively traversing the children (unless overriden). */
  public rewriteMappingType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `MappingValue` node, recursively traversing the children (unless overriden). */
  public rewriteMappingValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `MemberAccessExpression` node, recursively traversing the children (unless overriden). */
  public rewriteMemberAccessExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ModifierAttribute` node, recursively traversing the children (unless overriden). */
  public rewriteModifierAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ModifierAttributes` node, recursively traversing the children (unless overriden). */
  public rewriteModifierAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ModifierDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteModifierDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ModifierInvocation` node, recursively traversing the children (unless overriden). */
  public rewriteModifierInvocation(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `MultiplicativeExpression` node, recursively traversing the children (unless overriden). */
  public rewriteMultiplicativeExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `NamedArgument` node, recursively traversing the children (unless overriden). */
  public rewriteNamedArgument(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `NamedArgumentGroup` node, recursively traversing the children (unless overriden). */
  public rewriteNamedArgumentGroup(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `NamedArguments` node, recursively traversing the children (unless overriden). */
  public rewriteNamedArguments(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `NamedArgumentsDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteNamedArgumentsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `NamedImport` node, recursively traversing the children (unless overriden). */
  public rewriteNamedImport(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `NewExpression` node, recursively traversing the children (unless overriden). */
  public rewriteNewExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `NumberUnit` node, recursively traversing the children (unless overriden). */
  public rewriteNumberUnit(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `OrExpression` node, recursively traversing the children (unless overriden). */
  public rewriteOrExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `OverridePaths` node, recursively traversing the children (unless overriden). */
  public rewriteOverridePaths(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `OverridePathsDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteOverridePathsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `OverrideSpecifier` node, recursively traversing the children (unless overriden). */
  public rewriteOverrideSpecifier(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `Parameter` node, recursively traversing the children (unless overriden). */
  public rewriteParameter(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `Parameters` node, recursively traversing the children (unless overriden). */
  public rewriteParameters(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ParametersDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteParametersDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `PathImport` node, recursively traversing the children (unless overriden). */
  public rewritePathImport(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `PositionalArguments` node, recursively traversing the children (unless overriden). */
  public rewritePositionalArguments(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `PositionalArgumentsDeclaration` node, recursively traversing the children (unless overriden). */
  public rewritePositionalArgumentsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `PostfixExpression` node, recursively traversing the children (unless overriden). */
  public rewritePostfixExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `Pragma` node, recursively traversing the children (unless overriden). */
  public rewritePragma(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `PragmaDirective` node, recursively traversing the children (unless overriden). */
  public rewritePragmaDirective(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `PrefixExpression` node, recursively traversing the children (unless overriden). */
  public rewritePrefixExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ReceiveFunctionAttribute` node, recursively traversing the children (unless overriden). */
  public rewriteReceiveFunctionAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ReceiveFunctionAttributes` node, recursively traversing the children (unless overriden). */
  public rewriteReceiveFunctionAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ReceiveFunctionDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteReceiveFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ReturnStatement` node, recursively traversing the children (unless overriden). */
  public rewriteReturnStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ReturnsDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteReturnsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `RevertStatement` node, recursively traversing the children (unless overriden). */
  public rewriteRevertStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ShiftExpression` node, recursively traversing the children (unless overriden). */
  public rewriteShiftExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `SimpleVersionLiteral` node, recursively traversing the children (unless overriden). */
  public rewriteSimpleVersionLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `SourceUnit` node, recursively traversing the children (unless overriden). */
  public rewriteSourceUnit(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `SourceUnitMember` node, recursively traversing the children (unless overriden). */
  public rewriteSourceUnitMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `SourceUnitMembers` node, recursively traversing the children (unless overriden). */
  public rewriteSourceUnitMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StateVariableAttribute` node, recursively traversing the children (unless overriden). */
  public rewriteStateVariableAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StateVariableAttributes` node, recursively traversing the children (unless overriden). */
  public rewriteStateVariableAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StateVariableDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteStateVariableDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StateVariableDefinitionValue` node, recursively traversing the children (unless overriden). */
  public rewriteStateVariableDefinitionValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `Statement` node, recursively traversing the children (unless overriden). */
  public rewriteStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `Statements` node, recursively traversing the children (unless overriden). */
  public rewriteStatements(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StorageLayoutSpecifier` node, recursively traversing the children (unless overriden). */
  public rewriteStorageLayoutSpecifier(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StorageLocation` node, recursively traversing the children (unless overriden). */
  public rewriteStorageLocation(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StringExpression` node, recursively traversing the children (unless overriden). */
  public rewriteStringExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StringLiteral` node, recursively traversing the children (unless overriden). */
  public rewriteStringLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StringLiterals` node, recursively traversing the children (unless overriden). */
  public rewriteStringLiterals(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StructDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteStructDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StructMember` node, recursively traversing the children (unless overriden). */
  public rewriteStructMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `StructMembers` node, recursively traversing the children (unless overriden). */
  public rewriteStructMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ThrowStatement` node, recursively traversing the children (unless overriden). */
  public rewriteThrowStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TryStatement` node, recursively traversing the children (unless overriden). */
  public rewriteTryStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TupleDeconstructionElement` node, recursively traversing the children (unless overriden). */
  public rewriteTupleDeconstructionElement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TupleDeconstructionElements` node, recursively traversing the children (unless overriden). */
  public rewriteTupleDeconstructionElements(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TupleDeconstructionStatement` node, recursively traversing the children (unless overriden). */
  public rewriteTupleDeconstructionStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TupleExpression` node, recursively traversing the children (unless overriden). */
  public rewriteTupleExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TupleMember` node, recursively traversing the children (unless overriden). */
  public rewriteTupleMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TupleValue` node, recursively traversing the children (unless overriden). */
  public rewriteTupleValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TupleValues` node, recursively traversing the children (unless overriden). */
  public rewriteTupleValues(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TypeExpression` node, recursively traversing the children (unless overriden). */
  public rewriteTypeExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TypeName` node, recursively traversing the children (unless overriden). */
  public rewriteTypeName(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `TypedTupleMember` node, recursively traversing the children (unless overriden). */
  public rewriteTypedTupleMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UncheckedBlock` node, recursively traversing the children (unless overriden). */
  public rewriteUncheckedBlock(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UnicodeStringLiteral` node, recursively traversing the children (unless overriden). */
  public rewriteUnicodeStringLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UnicodeStringLiterals` node, recursively traversing the children (unless overriden). */
  public rewriteUnicodeStringLiterals(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UnnamedFunctionAttribute` node, recursively traversing the children (unless overriden). */
  public rewriteUnnamedFunctionAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UnnamedFunctionAttributes` node, recursively traversing the children (unless overriden). */
  public rewriteUnnamedFunctionAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UnnamedFunctionDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteUnnamedFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UntypedTupleMember` node, recursively traversing the children (unless overriden). */
  public rewriteUntypedTupleMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UserDefinedValueTypeDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteUserDefinedValueTypeDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UsingAlias` node, recursively traversing the children (unless overriden). */
  public rewriteUsingAlias(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UsingClause` node, recursively traversing the children (unless overriden). */
  public rewriteUsingClause(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UsingDeconstruction` node, recursively traversing the children (unless overriden). */
  public rewriteUsingDeconstruction(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UsingDeconstructionSymbol` node, recursively traversing the children (unless overriden). */
  public rewriteUsingDeconstructionSymbol(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UsingDeconstructionSymbols` node, recursively traversing the children (unless overriden). */
  public rewriteUsingDeconstructionSymbols(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UsingDirective` node, recursively traversing the children (unless overriden). */
  public rewriteUsingDirective(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UsingOperator` node, recursively traversing the children (unless overriden). */
  public rewriteUsingOperator(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `UsingTarget` node, recursively traversing the children (unless overriden). */
  public rewriteUsingTarget(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VariableDeclarationStatement` node, recursively traversing the children (unless overriden). */
  public rewriteVariableDeclarationStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VariableDeclarationType` node, recursively traversing the children (unless overriden). */
  public rewriteVariableDeclarationType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VariableDeclarationValue` node, recursively traversing the children (unless overriden). */
  public rewriteVariableDeclarationValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VersionExpression` node, recursively traversing the children (unless overriden). */
  public rewriteVersionExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VersionExpressionSet` node, recursively traversing the children (unless overriden). */
  public rewriteVersionExpressionSet(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VersionExpressionSets` node, recursively traversing the children (unless overriden). */
  public rewriteVersionExpressionSets(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VersionLiteral` node, recursively traversing the children (unless overriden). */
  public rewriteVersionLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VersionOperator` node, recursively traversing the children (unless overriden). */
  public rewriteVersionOperator(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VersionPragma` node, recursively traversing the children (unless overriden). */
  public rewriteVersionPragma(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VersionRange` node, recursively traversing the children (unless overriden). */
  public rewriteVersionRange(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `VersionTerm` node, recursively traversing the children (unless overriden). */
  public rewriteVersionTerm(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `WhileStatement` node, recursively traversing the children (unless overriden). */
  public rewriteWhileStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulArguments` node, recursively traversing the children (unless overriden). */
  public rewriteYulArguments(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulAssignmentOperator` node, recursively traversing the children (unless overriden). */
  public rewriteYulAssignmentOperator(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulBlock` node, recursively traversing the children (unless overriden). */
  public rewriteYulBlock(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulBreakStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulBreakStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulColonAndEqual` node, recursively traversing the children (unless overriden). */
  public rewriteYulColonAndEqual(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulContinueStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulContinueStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulDefaultCase` node, recursively traversing the children (unless overriden). */
  public rewriteYulDefaultCase(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulEqualAndColon` node, recursively traversing the children (unless overriden). */
  public rewriteYulEqualAndColon(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulExpression` node, recursively traversing the children (unless overriden). */
  public rewriteYulExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulForStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulForStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulFunctionCallExpression` node, recursively traversing the children (unless overriden). */
  public rewriteYulFunctionCallExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulFunctionDefinition` node, recursively traversing the children (unless overriden). */
  public rewriteYulFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulIfStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulIfStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulLabel` node, recursively traversing the children (unless overriden). */
  public rewriteYulLabel(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulLeaveStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulLeaveStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulLiteral` node, recursively traversing the children (unless overriden). */
  public rewriteYulLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulParameters` node, recursively traversing the children (unless overriden). */
  public rewriteYulParameters(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulParametersDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteYulParametersDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulPath` node, recursively traversing the children (unless overriden). */
  public rewriteYulPath(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulPaths` node, recursively traversing the children (unless overriden). */
  public rewriteYulPaths(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulReturnsDeclaration` node, recursively traversing the children (unless overriden). */
  public rewriteYulReturnsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulStackAssignmentOperator` node, recursively traversing the children (unless overriden). */
  public rewriteYulStackAssignmentOperator(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulStackAssignmentStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulStackAssignmentStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulStatements` node, recursively traversing the children (unless overriden). */
  public rewriteYulStatements(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulSwitchCase` node, recursively traversing the children (unless overriden). */
  public rewriteYulSwitchCase(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulSwitchCases` node, recursively traversing the children (unless overriden). */
  public rewriteYulSwitchCases(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulSwitchStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulSwitchStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulValueCase` node, recursively traversing the children (unless overriden). */
  public rewriteYulValueCase(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulVariableAssignmentStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulVariableAssignmentStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulVariableDeclarationStatement` node, recursively traversing the children (unless overriden). */
  public rewriteYulVariableDeclarationStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulVariableDeclarationValue` node, recursively traversing the children (unless overriden). */
  public rewriteYulVariableDeclarationValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `YulVariableNames` node, recursively traversing the children (unless overriden). */
  public rewriteYulVariableNames(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(node);
  }

  /** @virtual Rewrites a `ABIEncoderV2Keyword` node. */
  public rewriteABIEncoderV2Keyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AbicoderKeyword` node. */
  public rewriteAbicoderKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AbicoderV1Keyword` node. */
  public rewriteAbicoderV1Keyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AbicoderV2Keyword` node. */
  public rewriteAbicoderV2Keyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AbstractKeyword` node. */
  public rewriteAbstractKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AddressKeyword` node. */
  public rewriteAddressKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AfterKeyword` node. */
  public rewriteAfterKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AliasKeyword` node. */
  public rewriteAliasKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Ampersand` node. */
  public rewriteAmpersand(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AmpersandAmpersand` node. */
  public rewriteAmpersandAmpersand(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AmpersandEqual` node. */
  public rewriteAmpersandEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AnonymousKeyword` node. */
  public rewriteAnonymousKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ApplyKeyword` node. */
  public rewriteApplyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AsKeyword` node. */
  public rewriteAsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AssemblyKeyword` node. */
  public rewriteAssemblyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Asterisk` node. */
  public rewriteAsterisk(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AsteriskAsterisk` node. */
  public rewriteAsteriskAsterisk(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AsteriskEqual` node. */
  public rewriteAsteriskEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AtKeyword` node. */
  public rewriteAtKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `AutoKeyword` node. */
  public rewriteAutoKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Bang` node. */
  public rewriteBang(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `BangEqual` node. */
  public rewriteBangEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Bar` node. */
  public rewriteBar(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `BarBar` node. */
  public rewriteBarBar(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `BarEqual` node. */
  public rewriteBarEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `BoolKeyword` node. */
  public rewriteBoolKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `BreakKeyword` node. */
  public rewriteBreakKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ByteKeyword` node. */
  public rewriteByteKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `BytesKeyword` node. */
  public rewriteBytesKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `CallDataKeyword` node. */
  public rewriteCallDataKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Caret` node. */
  public rewriteCaret(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `CaretEqual` node. */
  public rewriteCaretEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `CaseKeyword` node. */
  public rewriteCaseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `CatchKeyword` node. */
  public rewriteCatchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `CloseBrace` node. */
  public rewriteCloseBrace(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `CloseBracket` node. */
  public rewriteCloseBracket(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `CloseParen` node. */
  public rewriteCloseParen(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Colon` node. */
  public rewriteColon(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ColonEqual` node. */
  public rewriteColonEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Comma` node. */
  public rewriteComma(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ConstantKeyword` node. */
  public rewriteConstantKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ConstructorKeyword` node. */
  public rewriteConstructorKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ContinueKeyword` node. */
  public rewriteContinueKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ContractKeyword` node. */
  public rewriteContractKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `CopyOfKeyword` node. */
  public rewriteCopyOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DaysKeyword` node. */
  public rewriteDaysKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DecimalLiteral` node. */
  public rewriteDecimalLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DefaultKeyword` node. */
  public rewriteDefaultKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DefineKeyword` node. */
  public rewriteDefineKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DeleteKeyword` node. */
  public rewriteDeleteKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DoKeyword` node. */
  public rewriteDoKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DoubleQuotedHexStringLiteral` node. */
  public rewriteDoubleQuotedHexStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DoubleQuotedStringLiteral` node. */
  public rewriteDoubleQuotedStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DoubleQuotedUnicodeStringLiteral` node. */
  public rewriteDoubleQuotedUnicodeStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `DoubleQuotedVersionLiteral` node. */
  public rewriteDoubleQuotedVersionLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ElseKeyword` node. */
  public rewriteElseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `EmitKeyword` node. */
  public rewriteEmitKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `EndOfLine` node. */
  public rewriteEndOfLine(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `EnumKeyword` node. */
  public rewriteEnumKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Equal` node. */
  public rewriteEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `EqualColon` node. */
  public rewriteEqualColon(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `EqualEqual` node. */
  public rewriteEqualEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `EqualGreaterThan` node. */
  public rewriteEqualGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ErrorKeyword` node. */
  public rewriteErrorKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `EtherKeyword` node. */
  public rewriteEtherKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `EventKeyword` node. */
  public rewriteEventKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ExperimentalKeyword` node. */
  public rewriteExperimentalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ExternalKeyword` node. */
  public rewriteExternalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `FallbackKeyword` node. */
  public rewriteFallbackKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `FalseKeyword` node. */
  public rewriteFalseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `FinalKeyword` node. */
  public rewriteFinalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `FinneyKeyword` node. */
  public rewriteFinneyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `FixedKeyword` node. */
  public rewriteFixedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ForKeyword` node. */
  public rewriteForKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `FromKeyword` node. */
  public rewriteFromKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `FunctionKeyword` node. */
  public rewriteFunctionKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `GlobalKeyword` node. */
  public rewriteGlobalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `GreaterThan` node. */
  public rewriteGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `GreaterThanEqual` node. */
  public rewriteGreaterThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `GreaterThanGreaterThan` node. */
  public rewriteGreaterThanGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `GreaterThanGreaterThanEqual` node. */
  public rewriteGreaterThanGreaterThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `GreaterThanGreaterThanGreaterThan` node. */
  public rewriteGreaterThanGreaterThanGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `GreaterThanGreaterThanGreaterThanEqual` node. */
  public rewriteGreaterThanGreaterThanGreaterThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `GweiKeyword` node. */
  public rewriteGweiKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `HexKeyword` node. */
  public rewriteHexKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `HexLiteral` node. */
  public rewriteHexLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `HoursKeyword` node. */
  public rewriteHoursKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Identifier` node. */
  public rewriteIdentifier(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `IfKeyword` node. */
  public rewriteIfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ImmutableKeyword` node. */
  public rewriteImmutableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ImplementsKeyword` node. */
  public rewriteImplementsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ImportKeyword` node. */
  public rewriteImportKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `InKeyword` node. */
  public rewriteInKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `IndexedKeyword` node. */
  public rewriteIndexedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `InlineKeyword` node. */
  public rewriteInlineKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `IntKeyword` node. */
  public rewriteIntKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `InterfaceKeyword` node. */
  public rewriteInterfaceKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `InternalKeyword` node. */
  public rewriteInternalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `IsKeyword` node. */
  public rewriteIsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `LayoutKeyword` node. */
  public rewriteLayoutKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `LessThan` node. */
  public rewriteLessThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `LessThanEqual` node. */
  public rewriteLessThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `LessThanLessThan` node. */
  public rewriteLessThanLessThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `LessThanLessThanEqual` node. */
  public rewriteLessThanLessThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `LetKeyword` node. */
  public rewriteLetKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `LibraryKeyword` node. */
  public rewriteLibraryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MacroKeyword` node. */
  public rewriteMacroKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MappingKeyword` node. */
  public rewriteMappingKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MatchKeyword` node. */
  public rewriteMatchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MemoryKeyword` node. */
  public rewriteMemoryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Minus` node. */
  public rewriteMinus(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MinusEqual` node. */
  public rewriteMinusEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MinusGreaterThan` node. */
  public rewriteMinusGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MinusMinus` node. */
  public rewriteMinusMinus(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MinutesKeyword` node. */
  public rewriteMinutesKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ModifierKeyword` node. */
  public rewriteModifierKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MultiLineComment` node. */
  public rewriteMultiLineComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MultiLineNatSpecComment` node. */
  public rewriteMultiLineNatSpecComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `MutableKeyword` node. */
  public rewriteMutableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `NewKeyword` node. */
  public rewriteNewKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `NullKeyword` node. */
  public rewriteNullKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `OfKeyword` node. */
  public rewriteOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `OpenBrace` node. */
  public rewriteOpenBrace(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `OpenBracket` node. */
  public rewriteOpenBracket(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `OpenParen` node. */
  public rewriteOpenParen(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `OverrideKeyword` node. */
  public rewriteOverrideKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PartialKeyword` node. */
  public rewritePartialKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PayableKeyword` node. */
  public rewritePayableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Percent` node. */
  public rewritePercent(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PercentEqual` node. */
  public rewritePercentEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Period` node. */
  public rewritePeriod(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Plus` node. */
  public rewritePlus(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PlusEqual` node. */
  public rewritePlusEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PlusPlus` node. */
  public rewritePlusPlus(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PragmaKeyword` node. */
  public rewritePragmaKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PrivateKeyword` node. */
  public rewritePrivateKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PromiseKeyword` node. */
  public rewritePromiseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PublicKeyword` node. */
  public rewritePublicKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `PureKeyword` node. */
  public rewritePureKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `QuestionMark` node. */
  public rewriteQuestionMark(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ReceiveKeyword` node. */
  public rewriteReceiveKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ReferenceKeyword` node. */
  public rewriteReferenceKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `RelocatableKeyword` node. */
  public rewriteRelocatableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ReturnKeyword` node. */
  public rewriteReturnKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ReturnsKeyword` node. */
  public rewriteReturnsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `RevertKeyword` node. */
  public rewriteRevertKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SMTCheckerKeyword` node. */
  public rewriteSMTCheckerKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SealedKeyword` node. */
  public rewriteSealedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SecondsKeyword` node. */
  public rewriteSecondsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Semicolon` node. */
  public rewriteSemicolon(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SingleLineComment` node. */
  public rewriteSingleLineComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SingleLineNatSpecComment` node. */
  public rewriteSingleLineNatSpecComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SingleQuotedHexStringLiteral` node. */
  public rewriteSingleQuotedHexStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SingleQuotedStringLiteral` node. */
  public rewriteSingleQuotedStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SingleQuotedUnicodeStringLiteral` node. */
  public rewriteSingleQuotedUnicodeStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SingleQuotedVersionLiteral` node. */
  public rewriteSingleQuotedVersionLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SizeOfKeyword` node. */
  public rewriteSizeOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Slash` node. */
  public rewriteSlash(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SlashEqual` node. */
  public rewriteSlashEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SolidityKeyword` node. */
  public rewriteSolidityKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `StaticKeyword` node. */
  public rewriteStaticKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `StorageKeyword` node. */
  public rewriteStorageKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `StringKeyword` node. */
  public rewriteStringKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `StructKeyword` node. */
  public rewriteStructKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SuperKeyword` node. */
  public rewriteSuperKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SupportsKeyword` node. */
  public rewriteSupportsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SwitchKeyword` node. */
  public rewriteSwitchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `SzaboKeyword` node. */
  public rewriteSzaboKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ThisKeyword` node. */
  public rewriteThisKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ThrowKeyword` node. */
  public rewriteThrowKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Tilde` node. */
  public rewriteTilde(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `TransientKeyword` node. */
  public rewriteTransientKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `TrueKeyword` node. */
  public rewriteTrueKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `TryKeyword` node. */
  public rewriteTryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `TypeDefKeyword` node. */
  public rewriteTypeDefKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `TypeKeyword` node. */
  public rewriteTypeKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `TypeOfKeyword` node. */
  public rewriteTypeOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `UfixedKeyword` node. */
  public rewriteUfixedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `UintKeyword` node. */
  public rewriteUintKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `UncheckedKeyword` node. */
  public rewriteUncheckedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `UsingKeyword` node. */
  public rewriteUsingKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `VarKeyword` node. */
  public rewriteVarKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `VersionSpecifier` node. */
  public rewriteVersionSpecifier(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `ViewKeyword` node. */
  public rewriteViewKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `VirtualKeyword` node. */
  public rewriteVirtualKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `WeeksKeyword` node. */
  public rewriteWeeksKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `WeiKeyword` node. */
  public rewriteWeiKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `WhileKeyword` node. */
  public rewriteWhileKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Whitespace` node. */
  public rewriteWhitespace(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YearsKeyword` node. */
  public rewriteYearsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulAbstractKeyword` node. */
  public rewriteYulAbstractKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulAfterKeyword` node. */
  public rewriteYulAfterKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulAliasKeyword` node. */
  public rewriteYulAliasKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulAnonymousKeyword` node. */
  public rewriteYulAnonymousKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulApplyKeyword` node. */
  public rewriteYulApplyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulAsKeyword` node. */
  public rewriteYulAsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulAssemblyKeyword` node. */
  public rewriteYulAssemblyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulAutoKeyword` node. */
  public rewriteYulAutoKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulBoolKeyword` node. */
  public rewriteYulBoolKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulBreakKeyword` node. */
  public rewriteYulBreakKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulBytesKeyword` node. */
  public rewriteYulBytesKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulCallDataKeyword` node. */
  public rewriteYulCallDataKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulCaseKeyword` node. */
  public rewriteYulCaseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulCatchKeyword` node. */
  public rewriteYulCatchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulConstantKeyword` node. */
  public rewriteYulConstantKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulConstructorKeyword` node. */
  public rewriteYulConstructorKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulContinueKeyword` node. */
  public rewriteYulContinueKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulContractKeyword` node. */
  public rewriteYulContractKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulCopyOfKeyword` node. */
  public rewriteYulCopyOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulDaysKeyword` node. */
  public rewriteYulDaysKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulDecimalLiteral` node. */
  public rewriteYulDecimalLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulDefaultKeyword` node. */
  public rewriteYulDefaultKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulDefineKeyword` node. */
  public rewriteYulDefineKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulDeleteKeyword` node. */
  public rewriteYulDeleteKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulDoKeyword` node. */
  public rewriteYulDoKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulElseKeyword` node. */
  public rewriteYulElseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulEmitKeyword` node. */
  public rewriteYulEmitKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulEnumKeyword` node. */
  public rewriteYulEnumKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulEtherKeyword` node. */
  public rewriteYulEtherKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulEventKeyword` node. */
  public rewriteYulEventKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulExternalKeyword` node. */
  public rewriteYulExternalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulFallbackKeyword` node. */
  public rewriteYulFallbackKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulFalseKeyword` node. */
  public rewriteYulFalseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulFinalKeyword` node. */
  public rewriteYulFinalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulFinneyKeyword` node. */
  public rewriteYulFinneyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulFixedKeyword` node. */
  public rewriteYulFixedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulForKeyword` node. */
  public rewriteYulForKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulFunctionKeyword` node. */
  public rewriteYulFunctionKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulGweiKeyword` node. */
  public rewriteYulGweiKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulHexKeyword` node. */
  public rewriteYulHexKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulHexLiteral` node. */
  public rewriteYulHexLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulHoursKeyword` node. */
  public rewriteYulHoursKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulIdentifier` node. */
  public rewriteYulIdentifier(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulIfKeyword` node. */
  public rewriteYulIfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulImmutableKeyword` node. */
  public rewriteYulImmutableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulImplementsKeyword` node. */
  public rewriteYulImplementsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulImportKeyword` node. */
  public rewriteYulImportKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulInKeyword` node. */
  public rewriteYulInKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulIndexedKeyword` node. */
  public rewriteYulIndexedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulInlineKeyword` node. */
  public rewriteYulInlineKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulIntKeyword` node. */
  public rewriteYulIntKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulInterfaceKeyword` node. */
  public rewriteYulInterfaceKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulInternalKeyword` node. */
  public rewriteYulInternalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulIsKeyword` node. */
  public rewriteYulIsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulLeaveKeyword` node. */
  public rewriteYulLeaveKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulLetKeyword` node. */
  public rewriteYulLetKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulLibraryKeyword` node. */
  public rewriteYulLibraryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulMacroKeyword` node. */
  public rewriteYulMacroKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulMappingKeyword` node. */
  public rewriteYulMappingKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulMatchKeyword` node. */
  public rewriteYulMatchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulMemoryKeyword` node. */
  public rewriteYulMemoryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulMinutesKeyword` node. */
  public rewriteYulMinutesKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulModifierKeyword` node. */
  public rewriteYulModifierKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulMutableKeyword` node. */
  public rewriteYulMutableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulNewKeyword` node. */
  public rewriteYulNewKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulNullKeyword` node. */
  public rewriteYulNullKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulOfKeyword` node. */
  public rewriteYulOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulOverrideKeyword` node. */
  public rewriteYulOverrideKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulPartialKeyword` node. */
  public rewriteYulPartialKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulPayableKeyword` node. */
  public rewriteYulPayableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulPragmaKeyword` node. */
  public rewriteYulPragmaKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulPrivateKeyword` node. */
  public rewriteYulPrivateKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulPromiseKeyword` node. */
  public rewriteYulPromiseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulPublicKeyword` node. */
  public rewriteYulPublicKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulPureKeyword` node. */
  public rewriteYulPureKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulReceiveKeyword` node. */
  public rewriteYulReceiveKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulReferenceKeyword` node. */
  public rewriteYulReferenceKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulRelocatableKeyword` node. */
  public rewriteYulRelocatableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulReturnsKeyword` node. */
  public rewriteYulReturnsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulSealedKeyword` node. */
  public rewriteYulSealedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulSecondsKeyword` node. */
  public rewriteYulSecondsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulSizeOfKeyword` node. */
  public rewriteYulSizeOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulStaticKeyword` node. */
  public rewriteYulStaticKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulStorageKeyword` node. */
  public rewriteYulStorageKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulStringKeyword` node. */
  public rewriteYulStringKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulStructKeyword` node. */
  public rewriteYulStructKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulSuperKeyword` node. */
  public rewriteYulSuperKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulSupportsKeyword` node. */
  public rewriteYulSupportsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulSwitchKeyword` node. */
  public rewriteYulSwitchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulSzaboKeyword` node. */
  public rewriteYulSzaboKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulThisKeyword` node. */
  public rewriteYulThisKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulThrowKeyword` node. */
  public rewriteYulThrowKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulTrueKeyword` node. */
  public rewriteYulTrueKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulTryKeyword` node. */
  public rewriteYulTryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulTypeDefKeyword` node. */
  public rewriteYulTypeDefKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulTypeKeyword` node. */
  public rewriteYulTypeKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulTypeOfKeyword` node. */
  public rewriteYulTypeOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulUfixedKeyword` node. */
  public rewriteYulUfixedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulUintKeyword` node. */
  public rewriteYulUintKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulUncheckedKeyword` node. */
  public rewriteYulUncheckedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulUsingKeyword` node. */
  public rewriteYulUsingKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulVarKeyword` node. */
  public rewriteYulVarKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulViewKeyword` node. */
  public rewriteYulViewKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulVirtualKeyword` node. */
  public rewriteYulVirtualKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulWeeksKeyword` node. */
  public rewriteYulWeeksKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulWeiKeyword` node. */
  public rewriteYulWeiKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulWhileKeyword` node. */
  public rewriteYulWhileKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `YulYearsKeyword` node. */
  public rewriteYulYearsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites an `Unrecognized` node. */
  public rewriteUnrecognized(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual Rewrites a `Missing` node. */
  public rewriteMissing(node: TerminalNode): Node | undefined {
    return node;
  }

  /** Rewrites all the children of a given non-terminal node. */
  protected rewriteChildren(node: NonterminalNode): NonterminalNode {
    let newChildren: Array<Edge> | undefined = undefined;
    const children = node.children();

    children.forEach((child, index) => {
      const newChild = this.rewriteNode(child.node);
      if (newChild == undefined) {
        // node was removed. if `newChildren` is set, just skip this one
        // otheriwse, copy the first ones from `children` (but the last)
        if (newChildren == undefined) {
          newChildren = children.slice(0, index);
        }
      } else if (newChild.id != child.node.id) {
        // node has changed, produce new edge
        let edge;
        switch (newChild.type) {
          case NodeType.TerminalNode:
            edge = Edge.createWithTerminal(child.label, newChild);
            break;
          case NodeType.NonterminalNode:
            edge = Edge.createWithNonterminal(child.label, newChild);
            break;
        }

        if (newChildren == undefined) {
          newChildren = children.slice(0, index);
        }
        newChildren.push(edge);
      }
    });

    if (newChildren != undefined) {
      return NonterminalNode.create(node.kind, children);
    } else {
      return node;
    }
  }
}
