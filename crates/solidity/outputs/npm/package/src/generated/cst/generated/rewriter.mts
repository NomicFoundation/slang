// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import { NonterminalKind, NonterminalNode, Node, NodeType, TerminalNode, TerminalKind, Edge } from "../index.mjs";

export abstract class BaseRewriter {
  public rewriteNode(node: Node): Node | undefined {
    switch (node.type) {
      case NodeType.TerminalNode:
        return this.rewriteTerminalNode(node);
      case NodeType.NonterminalNode:
        return this.rewriteNonterminalNode(node);
    }
  }

  public rewriteNonterminalNode(node: NonterminalNode): Node | undefined {
    switch (node.kind) {
      case NonterminalKind.AbicoderPragma:
        return this.rewriteAbicoderPragma(node);

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

  public rewriteTerminalNode(node: TerminalNode): Node | undefined {
    switch (node.kind) {
      case TerminalKind.AbicoderKeyword:
        return this.rewriteAbicoderKeyword(node);

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

  /** @virtual */
  public rewriteAbicoderPragma(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.AbicoderPragma, node);
  }

  /** @virtual */
  public rewriteAdditiveExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.AdditiveExpression, node);
  }

  /** @virtual */
  public rewriteAddressType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.AddressType, node);
  }

  /** @virtual */
  public rewriteAndExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.AndExpression, node);
  }

  /** @virtual */
  public rewriteArgumentsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ArgumentsDeclaration, node);
  }

  /** @virtual */
  public rewriteArrayExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ArrayExpression, node);
  }

  /** @virtual */
  public rewriteArrayTypeName(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ArrayTypeName, node);
  }

  /** @virtual */
  public rewriteArrayValues(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ArrayValues, node);
  }

  /** @virtual */
  public rewriteAssemblyFlags(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.AssemblyFlags, node);
  }

  /** @virtual */
  public rewriteAssemblyFlagsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.AssemblyFlagsDeclaration, node);
  }

  /** @virtual */
  public rewriteAssemblyStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.AssemblyStatement, node);
  }

  /** @virtual */
  public rewriteAssignmentExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.AssignmentExpression, node);
  }

  /** @virtual */
  public rewriteBitwiseAndExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.BitwiseAndExpression, node);
  }

  /** @virtual */
  public rewriteBitwiseOrExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.BitwiseOrExpression, node);
  }

  /** @virtual */
  public rewriteBitwiseXorExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.BitwiseXorExpression, node);
  }

  /** @virtual */
  public rewriteBlock(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Block, node);
  }

  /** @virtual */
  public rewriteBreakStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.BreakStatement, node);
  }

  /** @virtual */
  public rewriteCallOptions(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.CallOptions, node);
  }

  /** @virtual */
  public rewriteCallOptionsExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.CallOptionsExpression, node);
  }

  /** @virtual */
  public rewriteCatchClause(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.CatchClause, node);
  }

  /** @virtual */
  public rewriteCatchClauseError(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.CatchClauseError, node);
  }

  /** @virtual */
  public rewriteCatchClauses(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.CatchClauses, node);
  }

  /** @virtual */
  public rewriteConditionalExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ConditionalExpression, node);
  }

  /** @virtual */
  public rewriteConstantDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ConstantDefinition, node);
  }

  /** @virtual */
  public rewriteConstructorAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ConstructorAttribute, node);
  }

  /** @virtual */
  public rewriteConstructorAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ConstructorAttributes, node);
  }

  /** @virtual */
  public rewriteConstructorDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ConstructorDefinition, node);
  }

  /** @virtual */
  public rewriteContinueStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ContinueStatement, node);
  }

  /** @virtual */
  public rewriteContractDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ContractDefinition, node);
  }

  /** @virtual */
  public rewriteContractMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ContractMember, node);
  }

  /** @virtual */
  public rewriteContractMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ContractMembers, node);
  }

  /** @virtual */
  public rewriteContractSpecifier(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ContractSpecifier, node);
  }

  /** @virtual */
  public rewriteContractSpecifiers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ContractSpecifiers, node);
  }

  /** @virtual */
  public rewriteDecimalNumberExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.DecimalNumberExpression, node);
  }

  /** @virtual */
  public rewriteDoWhileStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.DoWhileStatement, node);
  }

  /** @virtual */
  public rewriteElementaryType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ElementaryType, node);
  }

  /** @virtual */
  public rewriteElseBranch(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ElseBranch, node);
  }

  /** @virtual */
  public rewriteEmitStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.EmitStatement, node);
  }

  /** @virtual */
  public rewriteEnumDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.EnumDefinition, node);
  }

  /** @virtual */
  public rewriteEnumMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.EnumMembers, node);
  }

  /** @virtual */
  public rewriteEqualityExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.EqualityExpression, node);
  }

  /** @virtual */
  public rewriteErrorDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ErrorDefinition, node);
  }

  /** @virtual */
  public rewriteErrorParameter(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ErrorParameter, node);
  }

  /** @virtual */
  public rewriteErrorParameters(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ErrorParameters, node);
  }

  /** @virtual */
  public rewriteErrorParametersDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ErrorParametersDeclaration, node);
  }

  /** @virtual */
  public rewriteEventDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.EventDefinition, node);
  }

  /** @virtual */
  public rewriteEventParameter(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.EventParameter, node);
  }

  /** @virtual */
  public rewriteEventParameters(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.EventParameters, node);
  }

  /** @virtual */
  public rewriteEventParametersDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.EventParametersDeclaration, node);
  }

  /** @virtual */
  public rewriteExperimentalFeature(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ExperimentalFeature, node);
  }

  /** @virtual */
  public rewriteExperimentalPragma(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ExperimentalPragma, node);
  }

  /** @virtual */
  public rewriteExponentiationExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ExponentiationExpression, node);
  }

  /** @virtual */
  public rewriteExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Expression, node);
  }

  /** @virtual */
  public rewriteExpressionStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ExpressionStatement, node);
  }

  /** @virtual */
  public rewriteFallbackFunctionAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FallbackFunctionAttribute, node);
  }

  /** @virtual */
  public rewriteFallbackFunctionAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FallbackFunctionAttributes, node);
  }

  /** @virtual */
  public rewriteFallbackFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FallbackFunctionDefinition, node);
  }

  /** @virtual */
  public rewriteForStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ForStatement, node);
  }

  /** @virtual */
  public rewriteForStatementCondition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ForStatementCondition, node);
  }

  /** @virtual */
  public rewriteForStatementInitialization(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ForStatementInitialization, node);
  }

  /** @virtual */
  public rewriteFunctionAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FunctionAttribute, node);
  }

  /** @virtual */
  public rewriteFunctionAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FunctionAttributes, node);
  }

  /** @virtual */
  public rewriteFunctionBody(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FunctionBody, node);
  }

  /** @virtual */
  public rewriteFunctionCallExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FunctionCallExpression, node);
  }

  /** @virtual */
  public rewriteFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FunctionDefinition, node);
  }

  /** @virtual */
  public rewriteFunctionName(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FunctionName, node);
  }

  /** @virtual */
  public rewriteFunctionType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FunctionType, node);
  }

  /** @virtual */
  public rewriteFunctionTypeAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FunctionTypeAttribute, node);
  }

  /** @virtual */
  public rewriteFunctionTypeAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.FunctionTypeAttributes, node);
  }

  /** @virtual */
  public rewriteHexNumberExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.HexNumberExpression, node);
  }

  /** @virtual */
  public rewriteHexStringLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.HexStringLiteral, node);
  }

  /** @virtual */
  public rewriteHexStringLiterals(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.HexStringLiterals, node);
  }

  /** @virtual */
  public rewriteIdentifierPath(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.IdentifierPath, node);
  }

  /** @virtual */
  public rewriteIfStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.IfStatement, node);
  }

  /** @virtual */
  public rewriteImportAlias(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ImportAlias, node);
  }

  /** @virtual */
  public rewriteImportClause(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ImportClause, node);
  }

  /** @virtual */
  public rewriteImportDeconstruction(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ImportDeconstruction, node);
  }

  /** @virtual */
  public rewriteImportDeconstructionSymbol(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ImportDeconstructionSymbol, node);
  }

  /** @virtual */
  public rewriteImportDeconstructionSymbols(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ImportDeconstructionSymbols, node);
  }

  /** @virtual */
  public rewriteImportDirective(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ImportDirective, node);
  }

  /** @virtual */
  public rewriteIndexAccessEnd(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.IndexAccessEnd, node);
  }

  /** @virtual */
  public rewriteIndexAccessExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.IndexAccessExpression, node);
  }

  /** @virtual */
  public rewriteInequalityExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.InequalityExpression, node);
  }

  /** @virtual */
  public rewriteInheritanceSpecifier(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.InheritanceSpecifier, node);
  }

  /** @virtual */
  public rewriteInheritanceType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.InheritanceType, node);
  }

  /** @virtual */
  public rewriteInheritanceTypes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.InheritanceTypes, node);
  }

  /** @virtual */
  public rewriteInterfaceDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.InterfaceDefinition, node);
  }

  /** @virtual */
  public rewriteInterfaceMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.InterfaceMembers, node);
  }

  /** @virtual */
  public rewriteLibraryDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.LibraryDefinition, node);
  }

  /** @virtual */
  public rewriteLibraryMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.LibraryMembers, node);
  }

  /** @virtual */
  public rewriteMappingKey(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.MappingKey, node);
  }

  /** @virtual */
  public rewriteMappingKeyType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.MappingKeyType, node);
  }

  /** @virtual */
  public rewriteMappingType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.MappingType, node);
  }

  /** @virtual */
  public rewriteMappingValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.MappingValue, node);
  }

  /** @virtual */
  public rewriteMemberAccessExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.MemberAccessExpression, node);
  }

  /** @virtual */
  public rewriteModifierAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ModifierAttribute, node);
  }

  /** @virtual */
  public rewriteModifierAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ModifierAttributes, node);
  }

  /** @virtual */
  public rewriteModifierDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ModifierDefinition, node);
  }

  /** @virtual */
  public rewriteModifierInvocation(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ModifierInvocation, node);
  }

  /** @virtual */
  public rewriteMultiplicativeExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.MultiplicativeExpression, node);
  }

  /** @virtual */
  public rewriteNamedArgument(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.NamedArgument, node);
  }

  /** @virtual */
  public rewriteNamedArgumentGroup(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.NamedArgumentGroup, node);
  }

  /** @virtual */
  public rewriteNamedArguments(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.NamedArguments, node);
  }

  /** @virtual */
  public rewriteNamedArgumentsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.NamedArgumentsDeclaration, node);
  }

  /** @virtual */
  public rewriteNamedImport(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.NamedImport, node);
  }

  /** @virtual */
  public rewriteNewExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.NewExpression, node);
  }

  /** @virtual */
  public rewriteNumberUnit(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.NumberUnit, node);
  }

  /** @virtual */
  public rewriteOrExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.OrExpression, node);
  }

  /** @virtual */
  public rewriteOverridePaths(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.OverridePaths, node);
  }

  /** @virtual */
  public rewriteOverridePathsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.OverridePathsDeclaration, node);
  }

  /** @virtual */
  public rewriteOverrideSpecifier(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.OverrideSpecifier, node);
  }

  /** @virtual */
  public rewriteParameter(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Parameter, node);
  }

  /** @virtual */
  public rewriteParameters(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Parameters, node);
  }

  /** @virtual */
  public rewriteParametersDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ParametersDeclaration, node);
  }

  /** @virtual */
  public rewritePathImport(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.PathImport, node);
  }

  /** @virtual */
  public rewritePositionalArguments(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.PositionalArguments, node);
  }

  /** @virtual */
  public rewritePositionalArgumentsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.PositionalArgumentsDeclaration, node);
  }

  /** @virtual */
  public rewritePostfixExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.PostfixExpression, node);
  }

  /** @virtual */
  public rewritePragma(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Pragma, node);
  }

  /** @virtual */
  public rewritePragmaDirective(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.PragmaDirective, node);
  }

  /** @virtual */
  public rewritePrefixExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.PrefixExpression, node);
  }

  /** @virtual */
  public rewriteReceiveFunctionAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ReceiveFunctionAttribute, node);
  }

  /** @virtual */
  public rewriteReceiveFunctionAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ReceiveFunctionAttributes, node);
  }

  /** @virtual */
  public rewriteReceiveFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ReceiveFunctionDefinition, node);
  }

  /** @virtual */
  public rewriteReturnStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ReturnStatement, node);
  }

  /** @virtual */
  public rewriteReturnsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ReturnsDeclaration, node);
  }

  /** @virtual */
  public rewriteRevertStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.RevertStatement, node);
  }

  /** @virtual */
  public rewriteShiftExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ShiftExpression, node);
  }

  /** @virtual */
  public rewriteSimpleVersionLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.SimpleVersionLiteral, node);
  }

  /** @virtual */
  public rewriteSourceUnit(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.SourceUnit, node);
  }

  /** @virtual */
  public rewriteSourceUnitMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.SourceUnitMember, node);
  }

  /** @virtual */
  public rewriteSourceUnitMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.SourceUnitMembers, node);
  }

  /** @virtual */
  public rewriteStateVariableAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StateVariableAttribute, node);
  }

  /** @virtual */
  public rewriteStateVariableAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StateVariableAttributes, node);
  }

  /** @virtual */
  public rewriteStateVariableDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StateVariableDefinition, node);
  }

  /** @virtual */
  public rewriteStateVariableDefinitionValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StateVariableDefinitionValue, node);
  }

  /** @virtual */
  public rewriteStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Statement, node);
  }

  /** @virtual */
  public rewriteStatements(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Statements, node);
  }

  /** @virtual */
  public rewriteStorageLayoutSpecifier(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StorageLayoutSpecifier, node);
  }

  /** @virtual */
  public rewriteStorageLocation(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StorageLocation, node);
  }

  /** @virtual */
  public rewriteStringExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StringExpression, node);
  }

  /** @virtual */
  public rewriteStringLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StringLiteral, node);
  }

  /** @virtual */
  public rewriteStringLiterals(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StringLiterals, node);
  }

  /** @virtual */
  public rewriteStructDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StructDefinition, node);
  }

  /** @virtual */
  public rewriteStructMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StructMember, node);
  }

  /** @virtual */
  public rewriteStructMembers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.StructMembers, node);
  }

  /** @virtual */
  public rewriteThrowStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.ThrowStatement, node);
  }

  /** @virtual */
  public rewriteTryStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TryStatement, node);
  }

  /** @virtual */
  public rewriteTupleDeconstructionElement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TupleDeconstructionElement, node);
  }

  /** @virtual */
  public rewriteTupleDeconstructionElements(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TupleDeconstructionElements, node);
  }

  /** @virtual */
  public rewriteTupleDeconstructionStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TupleDeconstructionStatement, node);
  }

  /** @virtual */
  public rewriteTupleExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TupleExpression, node);
  }

  /** @virtual */
  public rewriteTupleMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TupleMember, node);
  }

  /** @virtual */
  public rewriteTupleValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TupleValue, node);
  }

  /** @virtual */
  public rewriteTupleValues(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TupleValues, node);
  }

  /** @virtual */
  public rewriteTypeExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TypeExpression, node);
  }

  /** @virtual */
  public rewriteTypeName(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TypeName, node);
  }

  /** @virtual */
  public rewriteTypedTupleMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TypedTupleMember, node);
  }

  /** @virtual */
  public rewriteUncheckedBlock(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UncheckedBlock, node);
  }

  /** @virtual */
  public rewriteUnicodeStringLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UnicodeStringLiteral, node);
  }

  /** @virtual */
  public rewriteUnicodeStringLiterals(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UnicodeStringLiterals, node);
  }

  /** @virtual */
  public rewriteUnnamedFunctionAttribute(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UnnamedFunctionAttribute, node);
  }

  /** @virtual */
  public rewriteUnnamedFunctionAttributes(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UnnamedFunctionAttributes, node);
  }

  /** @virtual */
  public rewriteUnnamedFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UnnamedFunctionDefinition, node);
  }

  /** @virtual */
  public rewriteUntypedTupleMember(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UntypedTupleMember, node);
  }

  /** @virtual */
  public rewriteUserDefinedValueTypeDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UserDefinedValueTypeDefinition, node);
  }

  /** @virtual */
  public rewriteUsingAlias(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UsingAlias, node);
  }

  /** @virtual */
  public rewriteUsingClause(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UsingClause, node);
  }

  /** @virtual */
  public rewriteUsingDeconstruction(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UsingDeconstruction, node);
  }

  /** @virtual */
  public rewriteUsingDeconstructionSymbol(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UsingDeconstructionSymbol, node);
  }

  /** @virtual */
  public rewriteUsingDeconstructionSymbols(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UsingDeconstructionSymbols, node);
  }

  /** @virtual */
  public rewriteUsingDirective(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UsingDirective, node);
  }

  /** @virtual */
  public rewriteUsingOperator(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UsingOperator, node);
  }

  /** @virtual */
  public rewriteUsingTarget(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.UsingTarget, node);
  }

  /** @virtual */
  public rewriteVariableDeclarationStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VariableDeclarationStatement, node);
  }

  /** @virtual */
  public rewriteVariableDeclarationType(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VariableDeclarationType, node);
  }

  /** @virtual */
  public rewriteVariableDeclarationValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VariableDeclarationValue, node);
  }

  /** @virtual */
  public rewriteVersionExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VersionExpression, node);
  }

  /** @virtual */
  public rewriteVersionExpressionSet(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VersionExpressionSet, node);
  }

  /** @virtual */
  public rewriteVersionExpressionSets(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VersionExpressionSets, node);
  }

  /** @virtual */
  public rewriteVersionLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VersionLiteral, node);
  }

  /** @virtual */
  public rewriteVersionOperator(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VersionOperator, node);
  }

  /** @virtual */
  public rewriteVersionPragma(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VersionPragma, node);
  }

  /** @virtual */
  public rewriteVersionRange(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VersionRange, node);
  }

  /** @virtual */
  public rewriteVersionTerm(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.VersionTerm, node);
  }

  /** @virtual */
  public rewriteWhileStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.WhileStatement, node);
  }

  /** @virtual */
  public rewriteYulArguments(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulArguments, node);
  }

  /** @virtual */
  public rewriteYulAssignmentOperator(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulAssignmentOperator, node);
  }

  /** @virtual */
  public rewriteYulBlock(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulBlock, node);
  }

  /** @virtual */
  public rewriteYulBreakStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulBreakStatement, node);
  }

  /** @virtual */
  public rewriteYulColonAndEqual(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulColonAndEqual, node);
  }

  /** @virtual */
  public rewriteYulContinueStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulContinueStatement, node);
  }

  /** @virtual */
  public rewriteYulDefaultCase(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulDefaultCase, node);
  }

  /** @virtual */
  public rewriteYulEqualAndColon(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulEqualAndColon, node);
  }

  /** @virtual */
  public rewriteYulExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulExpression, node);
  }

  /** @virtual */
  public rewriteYulForStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulForStatement, node);
  }

  /** @virtual */
  public rewriteYulFunctionCallExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulFunctionCallExpression, node);
  }

  /** @virtual */
  public rewriteYulFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulFunctionDefinition, node);
  }

  /** @virtual */
  public rewriteYulIfStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulIfStatement, node);
  }

  /** @virtual */
  public rewriteYulLabel(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulLabel, node);
  }

  /** @virtual */
  public rewriteYulLeaveStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulLeaveStatement, node);
  }

  /** @virtual */
  public rewriteYulLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulLiteral, node);
  }

  /** @virtual */
  public rewriteYulParameters(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulParameters, node);
  }

  /** @virtual */
  public rewriteYulParametersDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulParametersDeclaration, node);
  }

  /** @virtual */
  public rewriteYulPath(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulPath, node);
  }

  /** @virtual */
  public rewriteYulPaths(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulPaths, node);
  }

  /** @virtual */
  public rewriteYulReturnsDeclaration(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulReturnsDeclaration, node);
  }

  /** @virtual */
  public rewriteYulStackAssignmentOperator(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulStackAssignmentOperator, node);
  }

  /** @virtual */
  public rewriteYulStackAssignmentStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulStackAssignmentStatement, node);
  }

  /** @virtual */
  public rewriteYulStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulStatement, node);
  }

  /** @virtual */
  public rewriteYulStatements(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulStatements, node);
  }

  /** @virtual */
  public rewriteYulSwitchCase(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulSwitchCase, node);
  }

  /** @virtual */
  public rewriteYulSwitchCases(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulSwitchCases, node);
  }

  /** @virtual */
  public rewriteYulSwitchStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulSwitchStatement, node);
  }

  /** @virtual */
  public rewriteYulValueCase(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulValueCase, node);
  }

  /** @virtual */
  public rewriteYulVariableAssignmentStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulVariableAssignmentStatement, node);
  }

  /** @virtual */
  public rewriteYulVariableDeclarationStatement(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulVariableDeclarationStatement, node);
  }

  /** @virtual */
  public rewriteYulVariableDeclarationValue(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulVariableDeclarationValue, node);
  }

  /** @virtual */
  public rewriteYulVariableNames(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.YulVariableNames, node);
  }

  /** @virtual */
  public rewriteAbicoderKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAbstractKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAddressKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAfterKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAliasKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAmpersand(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAmpersandAmpersand(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAmpersandEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAnonymousKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteApplyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAssemblyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAsterisk(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAsteriskAsterisk(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAsteriskEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAtKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteAutoKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteBang(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteBangEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteBar(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteBarBar(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteBarEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteBoolKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteBreakKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteByteKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteBytesKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCallDataKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCaret(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCaretEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCaseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCatchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCloseBrace(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCloseBracket(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCloseParen(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteColon(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteColonEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteComma(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteConstantKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteConstructorKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteContinueKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteContractKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCopyOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDaysKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDecimalLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDefaultKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDefineKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDeleteKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDoKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDoubleQuotedHexStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDoubleQuotedStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDoubleQuotedUnicodeStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDoubleQuotedVersionLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteElseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEmitKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEndOfLine(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEnumKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEqualColon(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEqualEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEqualGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteErrorKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEtherKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEventKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteExperimentalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteExternalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteFallbackKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteFalseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteFinalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteFinneyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteFixedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteForKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteFromKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteFunctionKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteGlobalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteGreaterThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteGreaterThanGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteGreaterThanGreaterThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteGreaterThanGreaterThanGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteGreaterThanGreaterThanGreaterThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteGweiKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteHexKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteHexLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteHoursKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteIdentifier(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteIfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteImmutableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteImplementsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteImportKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteInKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteIndexedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteInlineKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteIntKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteInterfaceKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteInternalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteIsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteLayoutKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteLessThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteLessThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteLessThanLessThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteLessThanLessThanEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteLetKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteLibraryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMacroKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMappingKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMatchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMemoryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMinus(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMinusEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMinusGreaterThan(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMinusMinus(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMinutesKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteModifierKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMultiLineComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMultiLineNatSpecComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMutableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteNewKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteNullKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteOpenBrace(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteOpenBracket(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteOpenParen(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteOverrideKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePartialKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePayableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePercent(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePercentEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePeriod(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePlus(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePlusEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePlusPlus(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePragmaKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePrivateKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePromiseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePublicKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewritePureKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteQuestionMark(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteReceiveKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteReferenceKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteRelocatableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteReturnKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteReturnsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteRevertKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSealedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSecondsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSemicolon(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSingleLineComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSingleLineNatSpecComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSingleQuotedHexStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSingleQuotedStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSingleQuotedUnicodeStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSingleQuotedVersionLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSizeOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSlash(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSlashEqual(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSolidityKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteStaticKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteStorageKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteStringKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteStructKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSuperKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSupportsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSwitchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSzaboKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteThisKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteThrowKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteTilde(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteTransientKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteTrueKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteTryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteTypeDefKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteTypeKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteTypeOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteUfixedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteUintKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteUncheckedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteUsingKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteVarKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteVersionSpecifier(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteViewKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteVirtualKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteWeeksKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteWeiKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteWhileKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteWhitespace(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYearsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulAbstractKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulAfterKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulAliasKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulAnonymousKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulApplyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulAsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulAssemblyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulAutoKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulBoolKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulBreakKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulBytesKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulCallDataKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulCaseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulCatchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulConstantKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulConstructorKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulContinueKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulContractKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulCopyOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulDaysKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulDecimalLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulDefaultKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulDefineKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulDeleteKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulDoKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulElseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulEmitKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulEnumKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulEtherKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulEventKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulExternalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulFallbackKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulFalseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulFinalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulFinneyKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulFixedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulForKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulFunctionKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulGweiKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulHexKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulHexLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulHoursKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulIdentifier(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulIfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulImmutableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulImplementsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulImportKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulInKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulIndexedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulInlineKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulIntKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulInterfaceKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulInternalKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulIsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulLeaveKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulLetKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulLibraryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulMacroKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulMappingKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulMatchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulMemoryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulMinutesKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulModifierKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulMutableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulNewKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulNullKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulOverrideKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulPartialKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulPayableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulPragmaKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulPrivateKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulPromiseKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulPublicKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulPureKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulReceiveKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulReferenceKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulRelocatableKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulReturnsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulSealedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulSecondsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulSizeOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulStaticKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulStorageKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulStringKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulStructKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulSuperKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulSupportsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulSwitchKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulSzaboKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulThisKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulThrowKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulTrueKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulTryKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulTypeDefKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulTypeKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulTypeOfKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulUfixedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulUintKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulUncheckedKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulUsingKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulVarKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulViewKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulVirtualKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulWeeksKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulWeiKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulWhileKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteYulYearsKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  public rewriteUnrecognized(node: TerminalNode): Node | undefined {
    return node;
  }

  public rewriteMissing(node: TerminalNode): Node | undefined {
    return node;
  }

  protected rewriteChildren(kind: NonterminalKind, node: NonterminalNode): NonterminalNode {
    let newChildren: Map<number, Edge | "delete"> | undefined = undefined;
    const children = node.children();
    children.forEach((child, index) => {
      const newChild = this.rewriteNode(child.node);
      if (newChild == undefined) {
        // node was removed, mark the removal
        newChildren = newChildren || new Map<number, Edge | "delete">();
        newChildren.set(index, "delete");
      } else {
        let edge;
        if (newChild.id == child.node.id) {
          edge = child;
        } else {
          // node has changed, produce new edge
          switch (newChild.type) {
            case NodeType.TerminalNode:
              edge = Edge.createWithTerminal(child.label, newChild);
              break;
            case NodeType.NonterminalNode:
              edge = Edge.createWithNonterminal(child.label, newChild);
              break;
          }
        }
        newChildren = newChildren || new Map<number, Edge | "delete">();
        newChildren.set(index, edge);
      }
    });

    if (newChildren != undefined) {
      let deleted = 0;
      const map = newChildren as Map<number, Edge | "delete">;
      for (const [index, edge] of map) {
        if (edge == "delete") {
          children.splice(index - deleted, 1);
          deleted += 1;
        } else {
          children[index + deleted] = edge;
        }
      }

      const newNode = NonterminalNode.create(kind, children);
      return newNode;
    } else {
      return node;
    }
  }
}
