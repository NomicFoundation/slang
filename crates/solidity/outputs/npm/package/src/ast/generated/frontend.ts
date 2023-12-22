// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import { ast_internal } from "../../generated";
import { RuleNode, TokenNode } from "../../cst";
import { RuleKind, TokenKind } from "../../kinds";

function once<T>(factory: () => T): () => T {
  let value: T | null = null;
  return () => {
    if (value === null) {
      value = factory();
    }
    return value;
  };
}

export class SourceUnit {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.SourceUnit));

  public constructor(public readonly cst: RuleNode) {}

  public readonly members: () => SourceUnitMembers | null = once(() => {
    const field = this.fields()[0];
    return field ? new SourceUnitMembers(field as RuleNode) : null;
  });
}

export class PragmaDirective {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.PragmaDirective));

  public constructor(public readonly cst: RuleNode) {}

  public readonly pragmaKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly pragma: () => Pragma = once(() => {
    const field = this.fields()[1];
    return new Pragma(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class ABICoderPragma {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ABICoderPragma));

  public constructor(public readonly cst: RuleNode) {}

  public readonly abicoderKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly version: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });
}

export class ExperimentalPragma {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ExperimentalPragma));

  public constructor(public readonly cst: RuleNode) {}

  public readonly experimentalKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly feature: () => ExperimentalFeature = once(() => {
    const field = this.fields()[1];
    return new ExperimentalFeature(field as RuleNode);
  });
}

export class VersionPragma {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.VersionPragma));

  public constructor(public readonly cst: RuleNode) {}

  public readonly solidityKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly expressions: () => VersionPragmaExpressions = once(() => {
    const field = this.fields()[1];
    return new VersionPragmaExpressions(field as RuleNode);
  });
}

export class VersionPragmaOrExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.VersionPragmaOrExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => VersionPragmaExpression = once(() => {
    const field = this.fields()[0];
    return new VersionPragmaExpression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => VersionPragmaExpression = once(() => {
    const field = this.fields()[2];
    return new VersionPragmaExpression(field as RuleNode);
  });
}

export class VersionPragmaRangeExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.VersionPragmaRangeExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => VersionPragmaExpression = once(() => {
    const field = this.fields()[0];
    return new VersionPragmaExpression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => VersionPragmaExpression = once(() => {
    const field = this.fields()[2];
    return new VersionPragmaExpression(field as RuleNode);
  });
}

export class VersionPragmaPrefixExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.VersionPragmaPrefixExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly operand: () => VersionPragmaExpression = once(() => {
    const field = this.fields()[0];
    return new VersionPragmaExpression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });
}

export class ImportDirective {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ImportDirective));

  public constructor(public readonly cst: RuleNode) {}

  public readonly importKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly clause: () => ImportClause = once(() => {
    const field = this.fields()[1];
    return new ImportClause(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class PathImport {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.PathImport));

  public constructor(public readonly cst: RuleNode) {}

  public readonly path: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly alias: () => ImportAlias | null = once(() => {
    const field = this.fields()[1];
    return field ? new ImportAlias(field as RuleNode) : null;
  });
}

export class NamedImport {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.NamedImport));

  public constructor(public readonly cst: RuleNode) {}

  public readonly asterisk: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly alias: () => ImportAlias = once(() => {
    const field = this.fields()[1];
    return new ImportAlias(field as RuleNode);
  });

  public readonly fromKeyword: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly path: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });
}

export class ImportDeconstruction {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ImportDeconstruction));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly symbols: () => ImportDeconstructionSymbols = once(() => {
    const field = this.fields()[1];
    return new ImportDeconstructionSymbols(field as RuleNode);
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly fromKeyword: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });

  public readonly path: () => TokenNode = once(() => {
    const field = this.fields()[4];
    return field as TokenNode;
  });
}

export class ImportDeconstructionSymbol {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ImportDeconstructionSymbol));

  public constructor(public readonly cst: RuleNode) {}

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly alias: () => ImportAlias | null = once(() => {
    const field = this.fields()[1];
    return field ? new ImportAlias(field as RuleNode) : null;
  });
}

export class ImportAlias {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ImportAlias));

  public constructor(public readonly cst: RuleNode) {}

  public readonly asKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly identifier: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });
}

export class UsingDirective {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.UsingDirective));

  public constructor(public readonly cst: RuleNode) {}

  public readonly usingKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly clause: () => UsingClause = once(() => {
    const field = this.fields()[1];
    return new UsingClause(field as RuleNode);
  });

  public readonly forKeyword: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly target: () => UsingTarget = once(() => {
    const field = this.fields()[3];
    return new UsingTarget(field as RuleNode);
  });

  public readonly globalKeyword: () => TokenNode | null = once(() => {
    const field = this.fields()[4];
    return field as TokenNode | null;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[5];
    return field as TokenNode;
  });
}

export class UsingDeconstruction {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.UsingDeconstruction));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly symbols: () => UsingDeconstructionSymbols = once(() => {
    const field = this.fields()[1];
    return new UsingDeconstructionSymbols(field as RuleNode);
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class UsingDeconstructionSymbol {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.UsingDeconstructionSymbol));

  public constructor(public readonly cst: RuleNode) {}

  public readonly name: () => IdentifierPath = once(() => {
    const field = this.fields()[0];
    return new IdentifierPath(field as RuleNode);
  });

  public readonly alias: () => UsingAlias | null = once(() => {
    const field = this.fields()[1];
    return field ? new UsingAlias(field as RuleNode) : null;
  });
}

export class UsingAlias {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.UsingAlias));

  public constructor(public readonly cst: RuleNode) {}

  public readonly asKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly operator: () => UsingOperator = once(() => {
    const field = this.fields()[1];
    return new UsingOperator(field as RuleNode);
  });
}

export class ContractDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ContractDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly abstractKeyword: () => TokenNode | null = once(() => {
    const field = this.fields()[0];
    return field as TokenNode | null;
  });

  public readonly contractKeyword: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly inheritence: () => InheritanceSpecifier | null = once(() => {
    const field = this.fields()[3];
    return field ? new InheritanceSpecifier(field as RuleNode) : null;
  });

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[4];
    return field as TokenNode;
  });

  public readonly members: () => ContractMembers | null = once(() => {
    const field = this.fields()[5];
    return field ? new ContractMembers(field as RuleNode) : null;
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[6];
    return field as TokenNode;
  });
}

export class InheritanceSpecifier {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.InheritanceSpecifier));

  public constructor(public readonly cst: RuleNode) {}

  public readonly isKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly types: () => InheritanceTypes = once(() => {
    const field = this.fields()[1];
    return new InheritanceTypes(field as RuleNode);
  });
}

export class InheritanceType {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.InheritanceType));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeName: () => IdentifierPath = once(() => {
    const field = this.fields()[0];
    return new IdentifierPath(field as RuleNode);
  });

  public readonly arguments: () => ArgumentsDeclaration | null = once(() => {
    const field = this.fields()[1];
    return field ? new ArgumentsDeclaration(field as RuleNode) : null;
  });
}

export class InterfaceDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.InterfaceDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly interfaceKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly inheritence: () => InheritanceSpecifier | null = once(() => {
    const field = this.fields()[2];
    return field ? new InheritanceSpecifier(field as RuleNode) : null;
  });

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });

  public readonly members: () => InterfaceMembers | null = once(() => {
    const field = this.fields()[4];
    return field ? new InterfaceMembers(field as RuleNode) : null;
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[5];
    return field as TokenNode;
  });
}

export class LibraryDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.LibraryDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly libraryKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly members: () => LibraryMembers | null = once(() => {
    const field = this.fields()[3];
    return field ? new LibraryMembers(field as RuleNode) : null;
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[4];
    return field as TokenNode;
  });
}

export class StructDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.StructDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly structKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly members: () => StructMembers | null = once(() => {
    const field = this.fields()[3];
    return field ? new StructMembers(field as RuleNode) : null;
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[4];
    return field as TokenNode;
  });
}

export class StructMember {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.StructMember));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[0];
    return new TypeName(field as RuleNode);
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class EnumDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.EnumDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly enumKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly members: () => EnumMembers | null = once(() => {
    const field = this.fields()[3];
    return field ? new EnumMembers(field as RuleNode) : null;
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[4];
    return field as TokenNode;
  });
}

export class ConstantDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ConstantDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[0];
    return new TypeName(field as RuleNode);
  });

  public readonly constantKeyword: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly equal: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });

  public readonly value: () => Expression = once(() => {
    const field = this.fields()[4];
    return new Expression(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[5];
    return field as TokenNode;
  });
}

export class StateVariableDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.StateVariableDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[0];
    return new TypeName(field as RuleNode);
  });

  public readonly attributes: () => StateVariableAttributes | null = once(() => {
    const field = this.fields()[1];
    return field ? new StateVariableAttributes(field as RuleNode) : null;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly value: () => StateVariableDefinitionValue | null = once(() => {
    const field = this.fields()[3];
    return field ? new StateVariableDefinitionValue(field as RuleNode) : null;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[4];
    return field as TokenNode;
  });
}

export class StateVariableDefinitionValue {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.StateVariableDefinitionValue));

  public constructor(public readonly cst: RuleNode) {}

  public readonly equal: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly value: () => Expression = once(() => {
    const field = this.fields()[1];
    return new Expression(field as RuleNode);
  });
}

export class FunctionDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.FunctionDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly functionKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => FunctionName = once(() => {
    const field = this.fields()[1];
    return new FunctionName(field as RuleNode);
  });

  public readonly parameters: () => ParametersDeclaration = once(() => {
    const field = this.fields()[2];
    return new ParametersDeclaration(field as RuleNode);
  });

  public readonly attributes: () => FunctionAttributes | null = once(() => {
    const field = this.fields()[3];
    return field ? new FunctionAttributes(field as RuleNode) : null;
  });

  public readonly returns: () => ReturnsDeclaration | null = once(() => {
    const field = this.fields()[4];
    return field ? new ReturnsDeclaration(field as RuleNode) : null;
  });

  public readonly body: () => FunctionBody = once(() => {
    const field = this.fields()[5];
    return new FunctionBody(field as RuleNode);
  });
}

export class ParametersDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ParametersDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly parameters: () => Parameters | null = once(() => {
    const field = this.fields()[1];
    return field ? new Parameters(field as RuleNode) : null;
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class Parameter {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.Parameter));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[0];
    return new TypeName(field as RuleNode);
  });

  public readonly storageLocation: () => StorageLocation | null = once(() => {
    const field = this.fields()[1];
    return field ? new StorageLocation(field as RuleNode) : null;
  });

  public readonly name: () => TokenNode | null = once(() => {
    const field = this.fields()[2];
    return field as TokenNode | null;
  });
}

export class OverrideSpecifier {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.OverrideSpecifier));

  public constructor(public readonly cst: RuleNode) {}

  public readonly overrideKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly overridden: () => OverridePathsDeclaration | null = once(() => {
    const field = this.fields()[1];
    return field ? new OverridePathsDeclaration(field as RuleNode) : null;
  });
}

export class OverridePathsDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.OverridePathsDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly paths: () => OverridePaths = once(() => {
    const field = this.fields()[1];
    return new OverridePaths(field as RuleNode);
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class ReturnsDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ReturnsDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly returnsKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly variables: () => ParametersDeclaration = once(() => {
    const field = this.fields()[1];
    return new ParametersDeclaration(field as RuleNode);
  });
}

export class ConstructorDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ConstructorDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly constructorKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly parameters: () => ParametersDeclaration = once(() => {
    const field = this.fields()[1];
    return new ParametersDeclaration(field as RuleNode);
  });

  public readonly attributes: () => ConstructorAttributes | null = once(() => {
    const field = this.fields()[2];
    return field ? new ConstructorAttributes(field as RuleNode) : null;
  });

  public readonly body: () => Block = once(() => {
    const field = this.fields()[3];
    return new Block(field as RuleNode);
  });
}

export class UnnamedFunctionDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.UnnamedFunctionDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly functionKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly parameters: () => ParametersDeclaration = once(() => {
    const field = this.fields()[1];
    return new ParametersDeclaration(field as RuleNode);
  });

  public readonly attributes: () => UnnamedFunctionAttributes | null = once(() => {
    const field = this.fields()[2];
    return field ? new UnnamedFunctionAttributes(field as RuleNode) : null;
  });

  public readonly body: () => FunctionBody = once(() => {
    const field = this.fields()[3];
    return new FunctionBody(field as RuleNode);
  });
}

export class FallbackFunctionDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.FallbackFunctionDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly fallbackKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly parameters: () => ParametersDeclaration = once(() => {
    const field = this.fields()[1];
    return new ParametersDeclaration(field as RuleNode);
  });

  public readonly attributes: () => FallbackFunctionAttributes | null = once(() => {
    const field = this.fields()[2];
    return field ? new FallbackFunctionAttributes(field as RuleNode) : null;
  });

  public readonly returns: () => ReturnsDeclaration | null = once(() => {
    const field = this.fields()[3];
    return field ? new ReturnsDeclaration(field as RuleNode) : null;
  });

  public readonly body: () => FunctionBody = once(() => {
    const field = this.fields()[4];
    return new FunctionBody(field as RuleNode);
  });
}

export class ReceiveFunctionDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ReceiveFunctionDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly receiveKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly parameters: () => ParametersDeclaration = once(() => {
    const field = this.fields()[1];
    return new ParametersDeclaration(field as RuleNode);
  });

  public readonly attributes: () => ReceiveFunctionAttributes | null = once(() => {
    const field = this.fields()[2];
    return field ? new ReceiveFunctionAttributes(field as RuleNode) : null;
  });

  public readonly body: () => FunctionBody = once(() => {
    const field = this.fields()[3];
    return new FunctionBody(field as RuleNode);
  });
}

export class ModifierDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ModifierDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly modifierKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly parameters: () => ParametersDeclaration | null = once(() => {
    const field = this.fields()[2];
    return field ? new ParametersDeclaration(field as RuleNode) : null;
  });

  public readonly attributes: () => ModifierAttributes | null = once(() => {
    const field = this.fields()[3];
    return field ? new ModifierAttributes(field as RuleNode) : null;
  });

  public readonly body: () => FunctionBody = once(() => {
    const field = this.fields()[4];
    return new FunctionBody(field as RuleNode);
  });
}

export class ModifierInvocation {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ModifierInvocation));

  public constructor(public readonly cst: RuleNode) {}

  public readonly name: () => IdentifierPath = once(() => {
    const field = this.fields()[0];
    return new IdentifierPath(field as RuleNode);
  });

  public readonly arguments: () => ArgumentsDeclaration | null = once(() => {
    const field = this.fields()[1];
    return field ? new ArgumentsDeclaration(field as RuleNode) : null;
  });
}

export class EventDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.EventDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly eventKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly parameters: () => EventParametersDeclaration = once(() => {
    const field = this.fields()[2];
    return new EventParametersDeclaration(field as RuleNode);
  });

  public readonly anonymousKeyword: () => TokenNode | null = once(() => {
    const field = this.fields()[3];
    return field as TokenNode | null;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[4];
    return field as TokenNode;
  });
}

export class EventParametersDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.EventParametersDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly parameters: () => EventParameters | null = once(() => {
    const field = this.fields()[1];
    return field ? new EventParameters(field as RuleNode) : null;
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class EventParameter {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.EventParameter));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[0];
    return new TypeName(field as RuleNode);
  });

  public readonly indexedKeyword: () => TokenNode | null = once(() => {
    const field = this.fields()[1];
    return field as TokenNode | null;
  });

  public readonly name: () => TokenNode | null = once(() => {
    const field = this.fields()[2];
    return field as TokenNode | null;
  });
}

export class UserDefinedValueTypeDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.UserDefinedValueTypeDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly isKeyword: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly valueType: () => ElementaryType = once(() => {
    const field = this.fields()[3];
    return new ElementaryType(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[4];
    return field as TokenNode;
  });
}

export class ErrorDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ErrorDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly errorKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly members: () => ErrorParametersDeclaration = once(() => {
    const field = this.fields()[2];
    return new ErrorParametersDeclaration(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });
}

export class ErrorParametersDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ErrorParametersDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly parameters: () => ErrorParameters | null = once(() => {
    const field = this.fields()[1];
    return field ? new ErrorParameters(field as RuleNode) : null;
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class ErrorParameter {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ErrorParameter));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[0];
    return new TypeName(field as RuleNode);
  });

  public readonly name: () => TokenNode | null = once(() => {
    const field = this.fields()[1];
    return field as TokenNode | null;
  });
}

export class ArrayTypeName {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ArrayTypeName));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openBracket: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly index: () => Expression | null = once(() => {
    const field = this.fields()[1];
    return field ? new Expression(field as RuleNode) : null;
  });

  public readonly closeBracket: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly operand: () => TypeName = once(() => {
    const field = this.fields()[3];
    return new TypeName(field as RuleNode);
  });
}

export class FunctionType {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.FunctionType));

  public constructor(public readonly cst: RuleNode) {}

  public readonly functionKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly parameters: () => ParametersDeclaration = once(() => {
    const field = this.fields()[1];
    return new ParametersDeclaration(field as RuleNode);
  });

  public readonly attributes: () => FunctionTypeAttributes | null = once(() => {
    const field = this.fields()[2];
    return field ? new FunctionTypeAttributes(field as RuleNode) : null;
  });

  public readonly returns: () => ReturnsDeclaration | null = once(() => {
    const field = this.fields()[3];
    return field ? new ReturnsDeclaration(field as RuleNode) : null;
  });
}

export class MappingType {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.MappingType));

  public constructor(public readonly cst: RuleNode) {}

  public readonly mappingKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly keyType: () => MappingKey = once(() => {
    const field = this.fields()[2];
    return new MappingKey(field as RuleNode);
  });

  public readonly equalGreaterThan: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });

  public readonly valueType: () => MappingValue = once(() => {
    const field = this.fields()[4];
    return new MappingValue(field as RuleNode);
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[5];
    return field as TokenNode;
  });
}

export class MappingKey {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.MappingKey));

  public constructor(public readonly cst: RuleNode) {}

  public readonly keyType: () => MappingKeyType = once(() => {
    const field = this.fields()[0];
    return new MappingKeyType(field as RuleNode);
  });

  public readonly name: () => TokenNode | null = once(() => {
    const field = this.fields()[1];
    return field as TokenNode | null;
  });
}

export class MappingValue {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.MappingValue));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[0];
    return new TypeName(field as RuleNode);
  });

  public readonly name: () => TokenNode | null = once(() => {
    const field = this.fields()[1];
    return field as TokenNode | null;
  });
}

export class AddressType {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.AddressType));

  public constructor(public readonly cst: RuleNode) {}

  public readonly addressKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly payableKeyword: () => TokenNode | null = once(() => {
    const field = this.fields()[1];
    return field as TokenNode | null;
  });
}

export class Block {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.Block));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly statements: () => Statements | null = once(() => {
    const field = this.fields()[1];
    return field ? new Statements(field as RuleNode) : null;
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class UncheckedBlock {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.UncheckedBlock));

  public constructor(public readonly cst: RuleNode) {}

  public readonly uncheckedKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly block: () => Block = once(() => {
    const field = this.fields()[1];
    return new Block(field as RuleNode);
  });
}

export class ExpressionStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ExpressionStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly expression: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });
}

export class AssemblyStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.AssemblyStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly assemblyKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly label: () => TokenNode | null = once(() => {
    const field = this.fields()[1];
    return field as TokenNode | null;
  });

  public readonly flags: () => AssemblyFlagsDeclaration | null = once(() => {
    const field = this.fields()[2];
    return field ? new AssemblyFlagsDeclaration(field as RuleNode) : null;
  });

  public readonly body: () => YulBlock = once(() => {
    const field = this.fields()[3];
    return new YulBlock(field as RuleNode);
  });
}

export class AssemblyFlagsDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.AssemblyFlagsDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly flags: () => AssemblyFlags = once(() => {
    const field = this.fields()[1];
    return new AssemblyFlags(field as RuleNode);
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class TupleDeconstructionStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.TupleDeconstructionStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly elements: () => TupleDeconstructionElements = once(() => {
    const field = this.fields()[1];
    return new TupleDeconstructionElements(field as RuleNode);
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly equal: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });

  public readonly expression: () => Expression = once(() => {
    const field = this.fields()[4];
    return new Expression(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[5];
    return field as TokenNode;
  });
}

export class TupleDeconstructionElement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.TupleDeconstructionElement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly member: () => TupleMember | null = once(() => {
    const field = this.fields()[0];
    return field ? new TupleMember(field as RuleNode) : null;
  });
}

export class TypedTupleMember {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.TypedTupleMember));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[0];
    return new TypeName(field as RuleNode);
  });

  public readonly storageLocation: () => StorageLocation | null = once(() => {
    const field = this.fields()[1];
    return field ? new StorageLocation(field as RuleNode) : null;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class UntypedTupleMember {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.UntypedTupleMember));

  public constructor(public readonly cst: RuleNode) {}

  public readonly storageLocation: () => StorageLocation | null = once(() => {
    const field = this.fields()[0];
    return field ? new StorageLocation(field as RuleNode) : null;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });
}

export class VariableDeclarationStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.VariableDeclarationStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly variableType: () => VariableDeclarationType = once(() => {
    const field = this.fields()[0];
    return new VariableDeclarationType(field as RuleNode);
  });

  public readonly storageLocation: () => StorageLocation | null = once(() => {
    const field = this.fields()[1];
    return field ? new StorageLocation(field as RuleNode) : null;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly value: () => VariableDeclarationValue | null = once(() => {
    const field = this.fields()[3];
    return field ? new VariableDeclarationValue(field as RuleNode) : null;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[4];
    return field as TokenNode;
  });
}

export class VariableDeclarationValue {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.VariableDeclarationValue));

  public constructor(public readonly cst: RuleNode) {}

  public readonly equal: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly expression: () => Expression = once(() => {
    const field = this.fields()[1];
    return new Expression(field as RuleNode);
  });
}

export class IfStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.IfStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly ifKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly condition: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });

  public readonly body: () => Statement = once(() => {
    const field = this.fields()[4];
    return new Statement(field as RuleNode);
  });

  public readonly elseBranch: () => ElseBranch | null = once(() => {
    const field = this.fields()[5];
    return field ? new ElseBranch(field as RuleNode) : null;
  });
}

export class ElseBranch {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ElseBranch));

  public constructor(public readonly cst: RuleNode) {}

  public readonly elseKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly body: () => Statement = once(() => {
    const field = this.fields()[1];
    return new Statement(field as RuleNode);
  });
}

export class ForStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ForStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly forKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly initialization: () => ForStatementInitialization = once(() => {
    const field = this.fields()[2];
    return new ForStatementInitialization(field as RuleNode);
  });

  public readonly condition: () => ForStatementCondition = once(() => {
    const field = this.fields()[3];
    return new ForStatementCondition(field as RuleNode);
  });

  public readonly iterator: () => Expression | null = once(() => {
    const field = this.fields()[4];
    return field ? new Expression(field as RuleNode) : null;
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[5];
    return field as TokenNode;
  });

  public readonly body: () => Statement = once(() => {
    const field = this.fields()[6];
    return new Statement(field as RuleNode);
  });
}

export class WhileStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.WhileStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly whileKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly condition: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });

  public readonly body: () => Statement = once(() => {
    const field = this.fields()[4];
    return new Statement(field as RuleNode);
  });
}

export class DoWhileStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.DoWhileStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly doKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly body: () => Statement = once(() => {
    const field = this.fields()[1];
    return new Statement(field as RuleNode);
  });

  public readonly whileKeyword: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });

  public readonly condition: () => Expression = once(() => {
    const field = this.fields()[4];
    return new Expression(field as RuleNode);
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[5];
    return field as TokenNode;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[6];
    return field as TokenNode;
  });
}

export class ContinueStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ContinueStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly continueKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });
}

export class BreakStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.BreakStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly breakKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });
}

export class ReturnStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ReturnStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly returnKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly expression: () => Expression | null = once(() => {
    const field = this.fields()[1];
    return field ? new Expression(field as RuleNode) : null;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class EmitStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.EmitStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly emitKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly event: () => IdentifierPath = once(() => {
    const field = this.fields()[1];
    return new IdentifierPath(field as RuleNode);
  });

  public readonly arguments: () => ArgumentsDeclaration = once(() => {
    const field = this.fields()[2];
    return new ArgumentsDeclaration(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });
}

export class DeleteStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.DeleteStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly deleteKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly expression: () => Expression = once(() => {
    const field = this.fields()[1];
    return new Expression(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class TryStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.TryStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly tryKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly expression: () => Expression = once(() => {
    const field = this.fields()[1];
    return new Expression(field as RuleNode);
  });

  public readonly returns: () => ReturnsDeclaration | null = once(() => {
    const field = this.fields()[2];
    return field ? new ReturnsDeclaration(field as RuleNode) : null;
  });

  public readonly body: () => Block = once(() => {
    const field = this.fields()[3];
    return new Block(field as RuleNode);
  });

  public readonly catchClauses: () => CatchClauses = once(() => {
    const field = this.fields()[4];
    return new CatchClauses(field as RuleNode);
  });
}

export class CatchClause {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.CatchClause));

  public constructor(public readonly cst: RuleNode) {}

  public readonly catchKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly error: () => CatchClauseError | null = once(() => {
    const field = this.fields()[1];
    return field ? new CatchClauseError(field as RuleNode) : null;
  });

  public readonly body: () => Block = once(() => {
    const field = this.fields()[2];
    return new Block(field as RuleNode);
  });
}

export class CatchClauseError {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.CatchClauseError));

  public constructor(public readonly cst: RuleNode) {}

  public readonly name: () => TokenNode | null = once(() => {
    const field = this.fields()[0];
    return field as TokenNode | null;
  });

  public readonly parameters: () => ParametersDeclaration = once(() => {
    const field = this.fields()[1];
    return new ParametersDeclaration(field as RuleNode);
  });
}

export class RevertStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.RevertStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly revertKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly error: () => IdentifierPath | null = once(() => {
    const field = this.fields()[1];
    return field ? new IdentifierPath(field as RuleNode) : null;
  });

  public readonly arguments: () => ArgumentsDeclaration = once(() => {
    const field = this.fields()[2];
    return new ArgumentsDeclaration(field as RuleNode);
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });
}

export class ThrowStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ThrowStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly throwKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly semicolon: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });
}

export class AssignmentExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.AssignmentExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class ConditionalExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ConditionalExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly questionMark: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly trueExpression: () => Expression = once(() => {
    const field = this.fields()[1];
    return new Expression(field as RuleNode);
  });

  public readonly colon: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly falseExpression: () => Expression = once(() => {
    const field = this.fields()[3];
    return new Expression(field as RuleNode);
  });

  public readonly operand: () => Expression = once(() => {
    const field = this.fields()[4];
    return new Expression(field as RuleNode);
  });
}

export class OrExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.OrExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class AndExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.AndExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class EqualityExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.EqualityExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class ComparisonExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ComparisonExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class BitwiseOrExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.BitwiseOrExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class BitwiseXorExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.BitwiseXorExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class BitwiseAndExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.BitwiseAndExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class ShiftExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ShiftExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class AdditiveExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.AdditiveExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class MultiplicativeExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.MultiplicativeExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class ExponentiationExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ExponentiationExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leftOperand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly rightOperand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class PostfixExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.PostfixExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly operand: () => Expression = once(() => {
    const field = this.fields()[1];
    return new Expression(field as RuleNode);
  });
}

export class PrefixExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.PrefixExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly operand: () => Expression = once(() => {
    const field = this.fields()[0];
    return new Expression(field as RuleNode);
  });

  public readonly operator: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });
}

export class FunctionCallExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.FunctionCallExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly options: () => FunctionCallOptions | null = once(() => {
    const field = this.fields()[0];
    return field ? new FunctionCallOptions(field as RuleNode) : null;
  });

  public readonly arguments: () => ArgumentsDeclaration = once(() => {
    const field = this.fields()[1];
    return new ArgumentsDeclaration(field as RuleNode);
  });

  public readonly operand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class MemberAccessExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.MemberAccessExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly period: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly member: () => MemberAccess = once(() => {
    const field = this.fields()[1];
    return new MemberAccess(field as RuleNode);
  });

  public readonly operand: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class IndexAccessExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.IndexAccessExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openBracket: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly start: () => Expression | null = once(() => {
    const field = this.fields()[1];
    return field ? new Expression(field as RuleNode) : null;
  });

  public readonly end: () => IndexAccessEnd | null = once(() => {
    const field = this.fields()[2];
    return field ? new IndexAccessEnd(field as RuleNode) : null;
  });

  public readonly closeBracket: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });

  public readonly operand: () => Expression = once(() => {
    const field = this.fields()[4];
    return new Expression(field as RuleNode);
  });
}

export class IndexAccessEnd {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.IndexAccessEnd));

  public constructor(public readonly cst: RuleNode) {}

  public readonly colon: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly end: () => Expression | null = once(() => {
    const field = this.fields()[1];
    return field ? new Expression(field as RuleNode) : null;
  });
}

export class PositionalArgumentsDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.PositionalArgumentsDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly arguments: () => PositionalArguments | null = once(() => {
    const field = this.fields()[1];
    return field ? new PositionalArguments(field as RuleNode) : null;
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class NamedArgumentsDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.NamedArgumentsDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly arguments: () => NamedArgumentGroup | null = once(() => {
    const field = this.fields()[1];
    return field ? new NamedArgumentGroup(field as RuleNode) : null;
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class NamedArgumentGroup {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.NamedArgumentGroup));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly arguments: () => NamedArguments | null = once(() => {
    const field = this.fields()[1];
    return field ? new NamedArguments(field as RuleNode) : null;
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class NamedArgument {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.NamedArgument));

  public constructor(public readonly cst: RuleNode) {}

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly colon: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly value: () => Expression = once(() => {
    const field = this.fields()[2];
    return new Expression(field as RuleNode);
  });
}

export class TypeExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.TypeExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly typeKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[2];
    return new TypeName(field as RuleNode);
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[3];
    return field as TokenNode;
  });
}

export class NewExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.NewExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly newKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly typeName: () => TypeName = once(() => {
    const field = this.fields()[1];
    return new TypeName(field as RuleNode);
  });
}

export class TupleExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.TupleExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly items: () => TupleValues = once(() => {
    const field = this.fields()[1];
    return new TupleValues(field as RuleNode);
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class TupleValue {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.TupleValue));

  public constructor(public readonly cst: RuleNode) {}

  public readonly expression: () => Expression | null = once(() => {
    const field = this.fields()[0];
    return field ? new Expression(field as RuleNode) : null;
  });
}

export class ArrayExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.ArrayExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openBracket: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly items: () => ArrayValues = once(() => {
    const field = this.fields()[1];
    return new ArrayValues(field as RuleNode);
  });

  public readonly closeBracket: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class HexNumberExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.HexNumberExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly literal: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly unit: () => NumberUnit | null = once(() => {
    const field = this.fields()[1];
    return field ? new NumberUnit(field as RuleNode) : null;
  });
}

export class DecimalNumberExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.DecimalNumberExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly literal: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly unit: () => NumberUnit | null = once(() => {
    const field = this.fields()[1];
    return field ? new NumberUnit(field as RuleNode) : null;
  });
}

export class YulBlock {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulBlock));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openBrace: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly statements: () => YulStatements | null = once(() => {
    const field = this.fields()[1];
    return field ? new YulStatements(field as RuleNode) : null;
  });

  public readonly closeBrace: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class YulFunctionDefinition {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulFunctionDefinition));

  public constructor(public readonly cst: RuleNode) {}

  public readonly functionKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly name: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly parameters: () => YulParametersDeclaration = once(() => {
    const field = this.fields()[2];
    return new YulParametersDeclaration(field as RuleNode);
  });

  public readonly returns: () => YulReturnsDeclaration | null = once(() => {
    const field = this.fields()[3];
    return field ? new YulReturnsDeclaration(field as RuleNode) : null;
  });

  public readonly body: () => YulBlock = once(() => {
    const field = this.fields()[4];
    return new YulBlock(field as RuleNode);
  });
}

export class YulParametersDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulParametersDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly parameters: () => YulParameters | null = once(() => {
    const field = this.fields()[1];
    return field ? new YulParameters(field as RuleNode) : null;
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });
}

export class YulReturnsDeclaration {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulReturnsDeclaration));

  public constructor(public readonly cst: RuleNode) {}

  public readonly minusGreaterThan: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly variables: () => YulReturnVariables = once(() => {
    const field = this.fields()[1];
    return new YulReturnVariables(field as RuleNode);
  });
}

export class YulVariableDeclarationStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulVariableDeclarationStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly letKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly names: () => YulIdentifierPaths = once(() => {
    const field = this.fields()[1];
    return new YulIdentifierPaths(field as RuleNode);
  });

  public readonly value: () => YulVariableDeclarationValue | null = once(() => {
    const field = this.fields()[2];
    return field ? new YulVariableDeclarationValue(field as RuleNode) : null;
  });
}

export class YulVariableDeclarationValue {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulVariableDeclarationValue));

  public constructor(public readonly cst: RuleNode) {}

  public readonly colonEqual: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly expression: () => YulExpression = once(() => {
    const field = this.fields()[1];
    return new YulExpression(field as RuleNode);
  });
}

export class YulAssignmentStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulAssignmentStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly names: () => YulIdentifierPaths = once(() => {
    const field = this.fields()[0];
    return new YulIdentifierPaths(field as RuleNode);
  });

  public readonly colonEqual: () => TokenNode = once(() => {
    const field = this.fields()[1];
    return field as TokenNode;
  });

  public readonly expression: () => YulExpression = once(() => {
    const field = this.fields()[2];
    return new YulExpression(field as RuleNode);
  });
}

export class YulIfStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulIfStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly ifKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly condition: () => YulExpression = once(() => {
    const field = this.fields()[1];
    return new YulExpression(field as RuleNode);
  });

  public readonly body: () => YulBlock = once(() => {
    const field = this.fields()[2];
    return new YulBlock(field as RuleNode);
  });
}

export class YulLeaveStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulLeaveStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly leaveKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });
}

export class YulBreakStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulBreakStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly breakKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });
}

export class YulContinueStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulContinueStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly continueKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });
}

export class YulForStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulForStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly forKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly initialization: () => YulBlock = once(() => {
    const field = this.fields()[1];
    return new YulBlock(field as RuleNode);
  });

  public readonly condition: () => YulExpression = once(() => {
    const field = this.fields()[2];
    return new YulExpression(field as RuleNode);
  });

  public readonly iterator: () => YulBlock = once(() => {
    const field = this.fields()[3];
    return new YulBlock(field as RuleNode);
  });

  public readonly body: () => YulBlock = once(() => {
    const field = this.fields()[4];
    return new YulBlock(field as RuleNode);
  });
}

export class YulSwitchStatement {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulSwitchStatement));

  public constructor(public readonly cst: RuleNode) {}

  public readonly switchKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly expression: () => YulExpression = once(() => {
    const field = this.fields()[1];
    return new YulExpression(field as RuleNode);
  });

  public readonly cases: () => YulSwitchCases = once(() => {
    const field = this.fields()[2];
    return new YulSwitchCases(field as RuleNode);
  });
}

export class YulDefaultCase {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulDefaultCase));

  public constructor(public readonly cst: RuleNode) {}

  public readonly defaultKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly body: () => YulBlock = once(() => {
    const field = this.fields()[1];
    return new YulBlock(field as RuleNode);
  });
}

export class YulValueCase {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulValueCase));

  public constructor(public readonly cst: RuleNode) {}

  public readonly caseKeyword: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly value: () => YulLiteral = once(() => {
    const field = this.fields()[1];
    return new YulLiteral(field as RuleNode);
  });

  public readonly body: () => YulBlock = once(() => {
    const field = this.fields()[2];
    return new YulBlock(field as RuleNode);
  });
}

export class YulFunctionCallExpression {
  private readonly fields = once(() => ast_internal.pickSequence(this.cst, RuleKind.YulFunctionCallExpression));

  public constructor(public readonly cst: RuleNode) {}

  public readonly openParen: () => TokenNode = once(() => {
    const field = this.fields()[0];
    return field as TokenNode;
  });

  public readonly arguments: () => YulArguments | null = once(() => {
    const field = this.fields()[1];
    return field ? new YulArguments(field as RuleNode) : null;
  });

  public readonly closeParen: () => TokenNode = once(() => {
    const field = this.fields()[2];
    return field as TokenNode;
  });

  public readonly operand: () => YulExpression = once(() => {
    const field = this.fields()[3];
    return new YulExpression(field as RuleNode);
  });
}

export class SourceUnitMember {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () =>
    | PragmaDirective
    | ImportDirective
    | ContractDefinition
    | InterfaceDefinition
    | LibraryDefinition
    | StructDefinition
    | EnumDefinition
    | FunctionDefinition
    | ConstantDefinition
    | ErrorDefinition
    | UserDefinedValueTypeDefinition
    | UsingDirective
    | EventDefinition = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.SourceUnitMember);

    switch (child.kind) {
      case RuleKind.PragmaDirective:
        return new PragmaDirective(child as RuleNode);
      case RuleKind.ImportDirective:
        return new ImportDirective(child as RuleNode);
      case RuleKind.ContractDefinition:
        return new ContractDefinition(child as RuleNode);
      case RuleKind.InterfaceDefinition:
        return new InterfaceDefinition(child as RuleNode);
      case RuleKind.LibraryDefinition:
        return new LibraryDefinition(child as RuleNode);
      case RuleKind.StructDefinition:
        return new StructDefinition(child as RuleNode);
      case RuleKind.EnumDefinition:
        return new EnumDefinition(child as RuleNode);
      case RuleKind.FunctionDefinition:
        return new FunctionDefinition(child as RuleNode);
      case RuleKind.ConstantDefinition:
        return new ConstantDefinition(child as RuleNode);
      case RuleKind.ErrorDefinition:
        return new ErrorDefinition(child as RuleNode);
      case RuleKind.UserDefinedValueTypeDefinition:
        return new UserDefinedValueTypeDefinition(child as RuleNode);
      case RuleKind.UsingDirective:
        return new UsingDirective(child as RuleNode);
      case RuleKind.EventDefinition:
        return new EventDefinition(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class Pragma {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => ABICoderPragma | ExperimentalPragma | VersionPragma = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.Pragma);

    switch (child.kind) {
      case RuleKind.ABICoderPragma:
        return new ABICoderPragma(child as RuleNode);
      case RuleKind.ExperimentalPragma:
        return new ExperimentalPragma(child as RuleNode);
      case RuleKind.VersionPragma:
        return new VersionPragma(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ExperimentalFeature {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ExperimentalFeature);

    switch (child.kind) {
      case TokenKind.Identifier:
      case TokenKind.AsciiStringLiteral:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class VersionPragmaExpression {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () =>
    | VersionPragmaOrExpression
    | VersionPragmaRangeExpression
    | VersionPragmaPrefixExpression
    | VersionPragmaSpecifier = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.VersionPragmaExpression);

    switch (child.kind) {
      case RuleKind.VersionPragmaOrExpression:
        return new VersionPragmaOrExpression(child as RuleNode);
      case RuleKind.VersionPragmaRangeExpression:
        return new VersionPragmaRangeExpression(child as RuleNode);
      case RuleKind.VersionPragmaPrefixExpression:
        return new VersionPragmaPrefixExpression(child as RuleNode);
      case RuleKind.VersionPragmaSpecifier:
        return new VersionPragmaSpecifier(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ImportClause {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => PathImport | NamedImport | ImportDeconstruction = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ImportClause);

    switch (child.kind) {
      case RuleKind.PathImport:
        return new PathImport(child as RuleNode);
      case RuleKind.NamedImport:
        return new NamedImport(child as RuleNode);
      case RuleKind.ImportDeconstruction:
        return new ImportDeconstruction(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class UsingClause {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => IdentifierPath | UsingDeconstruction = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.UsingClause);

    switch (child.kind) {
      case RuleKind.IdentifierPath:
        return new IdentifierPath(child as RuleNode);
      case RuleKind.UsingDeconstruction:
        return new UsingDeconstruction(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class UsingOperator {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.UsingOperator);

    switch (child.kind) {
      case TokenKind.Ampersand:
      case TokenKind.Asterisk:
      case TokenKind.BangEqual:
      case TokenKind.Bar:
      case TokenKind.Caret:
      case TokenKind.EqualEqual:
      case TokenKind.GreaterThan:
      case TokenKind.GreaterThanEqual:
      case TokenKind.LessThan:
      case TokenKind.LessThanEqual:
      case TokenKind.Minus:
      case TokenKind.Percent:
      case TokenKind.Plus:
      case TokenKind.Slash:
      case TokenKind.Tilde:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class UsingTarget {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TypeName | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.UsingTarget);

    switch (child.kind) {
      case RuleKind.TypeName:
        return new TypeName(child as RuleNode);

      case TokenKind.Asterisk:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ContractMember {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () =>
    | UsingDirective
    | FunctionDefinition
    | ConstructorDefinition
    | ReceiveFunctionDefinition
    | FallbackFunctionDefinition
    | UnnamedFunctionDefinition
    | ModifierDefinition
    | StructDefinition
    | EnumDefinition
    | EventDefinition
    | StateVariableDefinition
    | ErrorDefinition
    | UserDefinedValueTypeDefinition = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ContractMember);

    switch (child.kind) {
      case RuleKind.UsingDirective:
        return new UsingDirective(child as RuleNode);
      case RuleKind.FunctionDefinition:
        return new FunctionDefinition(child as RuleNode);
      case RuleKind.ConstructorDefinition:
        return new ConstructorDefinition(child as RuleNode);
      case RuleKind.ReceiveFunctionDefinition:
        return new ReceiveFunctionDefinition(child as RuleNode);
      case RuleKind.FallbackFunctionDefinition:
        return new FallbackFunctionDefinition(child as RuleNode);
      case RuleKind.UnnamedFunctionDefinition:
        return new UnnamedFunctionDefinition(child as RuleNode);
      case RuleKind.ModifierDefinition:
        return new ModifierDefinition(child as RuleNode);
      case RuleKind.StructDefinition:
        return new StructDefinition(child as RuleNode);
      case RuleKind.EnumDefinition:
        return new EnumDefinition(child as RuleNode);
      case RuleKind.EventDefinition:
        return new EventDefinition(child as RuleNode);
      case RuleKind.StateVariableDefinition:
        return new StateVariableDefinition(child as RuleNode);
      case RuleKind.ErrorDefinition:
        return new ErrorDefinition(child as RuleNode);
      case RuleKind.UserDefinedValueTypeDefinition:
        return new UserDefinedValueTypeDefinition(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class StateVariableAttribute {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => OverrideSpecifier | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.StateVariableAttribute);

    switch (child.kind) {
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(child as RuleNode);

      case TokenKind.ConstantKeyword:
      case TokenKind.InternalKeyword:
      case TokenKind.PrivateKeyword:
      case TokenKind.PublicKeyword:
      case TokenKind.ImmutableKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class FunctionName {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.FunctionName);

    switch (child.kind) {
      case TokenKind.Identifier:
      case TokenKind.FallbackKeyword:
      case TokenKind.ReceiveKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class FunctionAttribute {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => ModifierInvocation | OverrideSpecifier | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.FunctionAttribute);

    switch (child.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(child as RuleNode);
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(child as RuleNode);

      case TokenKind.ConstantKeyword:
      case TokenKind.ExternalKeyword:
      case TokenKind.InternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.PrivateKeyword:
      case TokenKind.PublicKeyword:
      case TokenKind.PureKeyword:
      case TokenKind.ViewKeyword:
      case TokenKind.VirtualKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class FunctionBody {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => Block | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.FunctionBody);

    switch (child.kind) {
      case RuleKind.Block:
        return new Block(child as RuleNode);

      case TokenKind.Semicolon:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ConstructorAttribute {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => ModifierInvocation | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ConstructorAttribute);

    switch (child.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(child as RuleNode);

      case TokenKind.InternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.PublicKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class UnnamedFunctionAttribute {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => ModifierInvocation | OverrideSpecifier | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.UnnamedFunctionAttribute);

    switch (child.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(child as RuleNode);
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(child as RuleNode);

      case TokenKind.ExternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.PureKeyword:
      case TokenKind.ViewKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class FallbackFunctionAttribute {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => ModifierInvocation | OverrideSpecifier | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.FallbackFunctionAttribute);

    switch (child.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(child as RuleNode);
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(child as RuleNode);

      case TokenKind.ExternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.PureKeyword:
      case TokenKind.ViewKeyword:
      case TokenKind.VirtualKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ReceiveFunctionAttribute {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => ModifierInvocation | OverrideSpecifier | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ReceiveFunctionAttribute);

    switch (child.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(child as RuleNode);
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(child as RuleNode);

      case TokenKind.ExternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.VirtualKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ModifierAttribute {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => OverrideSpecifier | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ModifierAttribute);

    switch (child.kind) {
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(child as RuleNode);

      case TokenKind.VirtualKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class TypeName {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath = once(
    () => {
      const child = ast_internal.pickChoice(this.cst, RuleKind.TypeName);

      switch (child.kind) {
        case RuleKind.ArrayTypeName:
          return new ArrayTypeName(child as RuleNode);
        case RuleKind.FunctionType:
          return new FunctionType(child as RuleNode);
        case RuleKind.MappingType:
          return new MappingType(child as RuleNode);
        case RuleKind.ElementaryType:
          return new ElementaryType(child as RuleNode);
        case RuleKind.IdentifierPath:
          return new IdentifierPath(child as RuleNode);

        default:
          throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
      }
    },
  );
}

export class FunctionTypeAttribute {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.FunctionTypeAttribute);

    switch (child.kind) {
      case TokenKind.InternalKeyword:
      case TokenKind.ExternalKeyword:
      case TokenKind.PrivateKeyword:
      case TokenKind.PublicKeyword:
      case TokenKind.PureKeyword:
      case TokenKind.ViewKeyword:
      case TokenKind.PayableKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class MappingKeyType {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => ElementaryType | IdentifierPath = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.MappingKeyType);

    switch (child.kind) {
      case RuleKind.ElementaryType:
        return new ElementaryType(child as RuleNode);
      case RuleKind.IdentifierPath:
        return new IdentifierPath(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ElementaryType {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => AddressType | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ElementaryType);

    switch (child.kind) {
      case RuleKind.AddressType:
        return new AddressType(child as RuleNode);

      case TokenKind.BoolKeyword:
      case TokenKind.ByteKeyword:
      case TokenKind.StringKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.BytesKeyword:
      case TokenKind.IntKeyword:
      case TokenKind.UintKeyword:
      case TokenKind.FixedKeyword:
      case TokenKind.UfixedKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class Statement {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () =>
    | ExpressionStatement
    | VariableDeclarationStatement
    | TupleDeconstructionStatement
    | IfStatement
    | ForStatement
    | WhileStatement
    | DoWhileStatement
    | ContinueStatement
    | BreakStatement
    | DeleteStatement
    | ReturnStatement
    | ThrowStatement
    | EmitStatement
    | TryStatement
    | RevertStatement
    | AssemblyStatement
    | Block
    | UncheckedBlock = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.Statement);

    switch (child.kind) {
      case RuleKind.ExpressionStatement:
        return new ExpressionStatement(child as RuleNode);
      case RuleKind.VariableDeclarationStatement:
        return new VariableDeclarationStatement(child as RuleNode);
      case RuleKind.TupleDeconstructionStatement:
        return new TupleDeconstructionStatement(child as RuleNode);
      case RuleKind.IfStatement:
        return new IfStatement(child as RuleNode);
      case RuleKind.ForStatement:
        return new ForStatement(child as RuleNode);
      case RuleKind.WhileStatement:
        return new WhileStatement(child as RuleNode);
      case RuleKind.DoWhileStatement:
        return new DoWhileStatement(child as RuleNode);
      case RuleKind.ContinueStatement:
        return new ContinueStatement(child as RuleNode);
      case RuleKind.BreakStatement:
        return new BreakStatement(child as RuleNode);
      case RuleKind.DeleteStatement:
        return new DeleteStatement(child as RuleNode);
      case RuleKind.ReturnStatement:
        return new ReturnStatement(child as RuleNode);
      case RuleKind.ThrowStatement:
        return new ThrowStatement(child as RuleNode);
      case RuleKind.EmitStatement:
        return new EmitStatement(child as RuleNode);
      case RuleKind.TryStatement:
        return new TryStatement(child as RuleNode);
      case RuleKind.RevertStatement:
        return new RevertStatement(child as RuleNode);
      case RuleKind.AssemblyStatement:
        return new AssemblyStatement(child as RuleNode);
      case RuleKind.Block:
        return new Block(child as RuleNode);
      case RuleKind.UncheckedBlock:
        return new UncheckedBlock(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class TupleMember {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TypedTupleMember | UntypedTupleMember = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.TupleMember);

    switch (child.kind) {
      case RuleKind.TypedTupleMember:
        return new TypedTupleMember(child as RuleNode);
      case RuleKind.UntypedTupleMember:
        return new UntypedTupleMember(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class VariableDeclarationType {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TypeName | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.VariableDeclarationType);

    switch (child.kind) {
      case RuleKind.TypeName:
        return new TypeName(child as RuleNode);

      case TokenKind.VarKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class StorageLocation {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.StorageLocation);

    switch (child.kind) {
      case TokenKind.MemoryKeyword:
      case TokenKind.StorageKeyword:
      case TokenKind.CallDataKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ForStatementInitialization {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () =>
    | ExpressionStatement
    | VariableDeclarationStatement
    | TupleDeconstructionStatement
    | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ForStatementInitialization);

    switch (child.kind) {
      case RuleKind.ExpressionStatement:
        return new ExpressionStatement(child as RuleNode);
      case RuleKind.VariableDeclarationStatement:
        return new VariableDeclarationStatement(child as RuleNode);
      case RuleKind.TupleDeconstructionStatement:
        return new TupleDeconstructionStatement(child as RuleNode);

      case TokenKind.Semicolon:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ForStatementCondition {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => ExpressionStatement | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ForStatementCondition);

    switch (child.kind) {
      case RuleKind.ExpressionStatement:
        return new ExpressionStatement(child as RuleNode);

      case TokenKind.Semicolon:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class Expression {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () =>
    | AssignmentExpression
    | ConditionalExpression
    | OrExpression
    | AndExpression
    | EqualityExpression
    | ComparisonExpression
    | BitwiseOrExpression
    | BitwiseXorExpression
    | BitwiseAndExpression
    | ShiftExpression
    | AdditiveExpression
    | MultiplicativeExpression
    | ExponentiationExpression
    | PostfixExpression
    | PrefixExpression
    | FunctionCallExpression
    | MemberAccessExpression
    | IndexAccessExpression
    | NewExpression
    | TupleExpression
    | TypeExpression
    | ArrayExpression
    | HexNumberExpression
    | DecimalNumberExpression
    | StringExpression
    | ElementaryType
    | TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.Expression);

    switch (child.kind) {
      case RuleKind.AssignmentExpression:
        return new AssignmentExpression(child as RuleNode);
      case RuleKind.ConditionalExpression:
        return new ConditionalExpression(child as RuleNode);
      case RuleKind.OrExpression:
        return new OrExpression(child as RuleNode);
      case RuleKind.AndExpression:
        return new AndExpression(child as RuleNode);
      case RuleKind.EqualityExpression:
        return new EqualityExpression(child as RuleNode);
      case RuleKind.ComparisonExpression:
        return new ComparisonExpression(child as RuleNode);
      case RuleKind.BitwiseOrExpression:
        return new BitwiseOrExpression(child as RuleNode);
      case RuleKind.BitwiseXorExpression:
        return new BitwiseXorExpression(child as RuleNode);
      case RuleKind.BitwiseAndExpression:
        return new BitwiseAndExpression(child as RuleNode);
      case RuleKind.ShiftExpression:
        return new ShiftExpression(child as RuleNode);
      case RuleKind.AdditiveExpression:
        return new AdditiveExpression(child as RuleNode);
      case RuleKind.MultiplicativeExpression:
        return new MultiplicativeExpression(child as RuleNode);
      case RuleKind.ExponentiationExpression:
        return new ExponentiationExpression(child as RuleNode);
      case RuleKind.PostfixExpression:
        return new PostfixExpression(child as RuleNode);
      case RuleKind.PrefixExpression:
        return new PrefixExpression(child as RuleNode);
      case RuleKind.FunctionCallExpression:
        return new FunctionCallExpression(child as RuleNode);
      case RuleKind.MemberAccessExpression:
        return new MemberAccessExpression(child as RuleNode);
      case RuleKind.IndexAccessExpression:
        return new IndexAccessExpression(child as RuleNode);
      case RuleKind.NewExpression:
        return new NewExpression(child as RuleNode);
      case RuleKind.TupleExpression:
        return new TupleExpression(child as RuleNode);
      case RuleKind.TypeExpression:
        return new TypeExpression(child as RuleNode);
      case RuleKind.ArrayExpression:
        return new ArrayExpression(child as RuleNode);
      case RuleKind.HexNumberExpression:
        return new HexNumberExpression(child as RuleNode);
      case RuleKind.DecimalNumberExpression:
        return new DecimalNumberExpression(child as RuleNode);
      case RuleKind.StringExpression:
        return new StringExpression(child as RuleNode);
      case RuleKind.ElementaryType:
        return new ElementaryType(child as RuleNode);

      case TokenKind.TrueKeyword:
      case TokenKind.FalseKeyword:
      case TokenKind.Identifier:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class MemberAccess {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.MemberAccess);

    switch (child.kind) {
      case TokenKind.Identifier:
      case TokenKind.AddressKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class FunctionCallOptions {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => NamedArgumentGroups | NamedArgumentGroup = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.FunctionCallOptions);

    switch (child.kind) {
      case RuleKind.NamedArgumentGroups:
        return new NamedArgumentGroups(child as RuleNode);
      case RuleKind.NamedArgumentGroup:
        return new NamedArgumentGroup(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class ArgumentsDeclaration {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => PositionalArgumentsDeclaration | NamedArgumentsDeclaration = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.ArgumentsDeclaration);

    switch (child.kind) {
      case RuleKind.PositionalArgumentsDeclaration:
        return new PositionalArgumentsDeclaration(child as RuleNode);
      case RuleKind.NamedArgumentsDeclaration:
        return new NamedArgumentsDeclaration(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class NumberUnit {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.NumberUnit);

    switch (child.kind) {
      case TokenKind.WeiKeyword:
      case TokenKind.GweiKeyword:
      case TokenKind.SzaboKeyword:
      case TokenKind.FinneyKeyword:
      case TokenKind.EtherKeyword:
      case TokenKind.SecondsKeyword:
      case TokenKind.MinutesKeyword:
      case TokenKind.HoursKeyword:
      case TokenKind.DaysKeyword:
      case TokenKind.WeeksKeyword:
      case TokenKind.YearsKeyword:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class StringExpression {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => HexStringLiterals | AsciiStringLiterals | UnicodeStringLiterals = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.StringExpression);

    switch (child.kind) {
      case RuleKind.HexStringLiterals:
        return new HexStringLiterals(child as RuleNode);
      case RuleKind.AsciiStringLiterals:
        return new AsciiStringLiterals(child as RuleNode);
      case RuleKind.UnicodeStringLiterals:
        return new UnicodeStringLiterals(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class YulStatement {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () =>
    | YulBlock
    | YulFunctionDefinition
    | YulVariableDeclarationStatement
    | YulAssignmentStatement
    | YulIfStatement
    | YulForStatement
    | YulSwitchStatement
    | YulLeaveStatement
    | YulBreakStatement
    | YulContinueStatement
    | YulExpression = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.YulStatement);

    switch (child.kind) {
      case RuleKind.YulBlock:
        return new YulBlock(child as RuleNode);
      case RuleKind.YulFunctionDefinition:
        return new YulFunctionDefinition(child as RuleNode);
      case RuleKind.YulVariableDeclarationStatement:
        return new YulVariableDeclarationStatement(child as RuleNode);
      case RuleKind.YulAssignmentStatement:
        return new YulAssignmentStatement(child as RuleNode);
      case RuleKind.YulIfStatement:
        return new YulIfStatement(child as RuleNode);
      case RuleKind.YulForStatement:
        return new YulForStatement(child as RuleNode);
      case RuleKind.YulSwitchStatement:
        return new YulSwitchStatement(child as RuleNode);
      case RuleKind.YulLeaveStatement:
        return new YulLeaveStatement(child as RuleNode);
      case RuleKind.YulBreakStatement:
        return new YulBreakStatement(child as RuleNode);
      case RuleKind.YulContinueStatement:
        return new YulContinueStatement(child as RuleNode);
      case RuleKind.YulExpression:
        return new YulExpression(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class YulSwitchCase {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => YulDefaultCase | YulValueCase = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.YulSwitchCase);

    switch (child.kind) {
      case RuleKind.YulDefaultCase:
        return new YulDefaultCase(child as RuleNode);
      case RuleKind.YulValueCase:
        return new YulValueCase(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class YulExpression {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => YulFunctionCallExpression | YulLiteral | YulIdentifierPath = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.YulExpression);

    switch (child.kind) {
      case RuleKind.YulFunctionCallExpression:
        return new YulFunctionCallExpression(child as RuleNode);
      case RuleKind.YulLiteral:
        return new YulLiteral(child as RuleNode);
      case RuleKind.YulIdentifierPath:
        return new YulIdentifierPath(child as RuleNode);

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class YulLiteral {
  public constructor(public readonly cst: RuleNode) {}

  public readonly variant: () => TokenNode = once(() => {
    const child = ast_internal.pickChoice(this.cst, RuleKind.YulLiteral);

    switch (child.kind) {
      case TokenKind.YulTrueKeyword:
      case TokenKind.YulFalseKeyword:
      case TokenKind.YulDecimalLiteral:
      case TokenKind.YulHexLiteral:
      case TokenKind.HexStringLiteral:
      case TokenKind.AsciiStringLiteral:
        return child as TokenNode;

      default:
        throw new Error(`Impossible: unrecognized child kind: ${child.kind}`);
    }
  });
}

export class SourceUnitMembers {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => SourceUnitMember[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.SourceUnitMembers);
    return items.map((item) => new SourceUnitMember(item as RuleNode));
  });
}

export class VersionPragmaExpressions {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => VersionPragmaExpression[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.VersionPragmaExpressions);
    return items.map((item) => new VersionPragmaExpression(item as RuleNode));
  });
}

export class ContractMembers {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => ContractMember[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.ContractMembers);
    return items.map((item) => new ContractMember(item as RuleNode));
  });
}

export class InterfaceMembers {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => ContractMember[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.InterfaceMembers);
    return items.map((item) => new ContractMember(item as RuleNode));
  });
}

export class LibraryMembers {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => ContractMember[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.LibraryMembers);
    return items.map((item) => new ContractMember(item as RuleNode));
  });
}

export class StructMembers {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => StructMember[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.StructMembers);
    return items.map((item) => new StructMember(item as RuleNode));
  });
}

export class StateVariableAttributes {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => StateVariableAttribute[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.StateVariableAttributes);
    return items.map((item) => new StateVariableAttribute(item as RuleNode));
  });
}

export class FunctionAttributes {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => FunctionAttribute[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.FunctionAttributes);
    return items.map((item) => new FunctionAttribute(item as RuleNode));
  });
}

export class ConstructorAttributes {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => ConstructorAttribute[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.ConstructorAttributes);
    return items.map((item) => new ConstructorAttribute(item as RuleNode));
  });
}

export class UnnamedFunctionAttributes {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => UnnamedFunctionAttribute[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.UnnamedFunctionAttributes);
    return items.map((item) => new UnnamedFunctionAttribute(item as RuleNode));
  });
}

export class FallbackFunctionAttributes {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => FallbackFunctionAttribute[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.FallbackFunctionAttributes);
    return items.map((item) => new FallbackFunctionAttribute(item as RuleNode));
  });
}

export class ReceiveFunctionAttributes {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => ReceiveFunctionAttribute[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.ReceiveFunctionAttributes);
    return items.map((item) => new ReceiveFunctionAttribute(item as RuleNode));
  });
}

export class ModifierAttributes {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => ModifierAttribute[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.ModifierAttributes);
    return items.map((item) => new ModifierAttribute(item as RuleNode));
  });
}

export class FunctionTypeAttributes {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => FunctionTypeAttribute[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.FunctionTypeAttributes);
    return items.map((item) => new FunctionTypeAttribute(item as RuleNode));
  });
}

export class Statements {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => Statement[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.Statements);
    return items.map((item) => new Statement(item as RuleNode));
  });
}

export class CatchClauses {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => CatchClause[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.CatchClauses);
    return items.map((item) => new CatchClause(item as RuleNode));
  });
}

export class NamedArgumentGroups {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => NamedArgumentGroup[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.NamedArgumentGroups);
    return items.map((item) => new NamedArgumentGroup(item as RuleNode));
  });
}

export class HexStringLiterals {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.HexStringLiterals);
    return items as TokenNode[];
  });
}

export class AsciiStringLiterals {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.AsciiStringLiterals);
    return items as TokenNode[];
  });
}

export class UnicodeStringLiterals {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.UnicodeStringLiterals);
    return items as TokenNode[];
  });
}

export class YulStatements {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => YulStatement[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.YulStatements);
    return items.map((item) => new YulStatement(item as RuleNode));
  });
}

export class YulSwitchCases {
  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => YulSwitchCase[] = once(() => {
    const items = ast_internal.pickRepeated(this.cst, RuleKind.YulSwitchCases);
    return items.map((item) => new YulSwitchCase(item as RuleNode));
  });
}

export class VersionPragmaSpecifier {
  private readonly items_with_separators = once(() =>
    ast_internal.pickSeparated(this.cst, RuleKind.VersionPragmaSpecifier),
  );

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    return this.items_with_separators()[0] as TokenNode[];
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class ImportDeconstructionSymbols {
  private readonly items_with_separators = once(() =>
    ast_internal.pickSeparated(this.cst, RuleKind.ImportDeconstructionSymbols),
  );

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => ImportDeconstructionSymbol[] = once(() => {
    return this.items_with_separators()[0].map((item) => new ImportDeconstructionSymbol(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class UsingDeconstructionSymbols {
  private readonly items_with_separators = once(() =>
    ast_internal.pickSeparated(this.cst, RuleKind.UsingDeconstructionSymbols),
  );

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => UsingDeconstructionSymbol[] = once(() => {
    return this.items_with_separators()[0].map((item) => new UsingDeconstructionSymbol(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class InheritanceTypes {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.InheritanceTypes));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => InheritanceType[] = once(() => {
    return this.items_with_separators()[0].map((item) => new InheritanceType(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class EnumMembers {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.EnumMembers));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    return this.items_with_separators()[0] as TokenNode[];
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class Parameters {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.Parameters));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => Parameter[] = once(() => {
    return this.items_with_separators()[0].map((item) => new Parameter(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class OverridePaths {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.OverridePaths));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => IdentifierPath[] = once(() => {
    return this.items_with_separators()[0].map((item) => new IdentifierPath(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class EventParameters {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.EventParameters));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => EventParameter[] = once(() => {
    return this.items_with_separators()[0].map((item) => new EventParameter(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class ErrorParameters {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.ErrorParameters));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => ErrorParameter[] = once(() => {
    return this.items_with_separators()[0].map((item) => new ErrorParameter(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class AssemblyFlags {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.AssemblyFlags));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    return this.items_with_separators()[0] as TokenNode[];
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class TupleDeconstructionElements {
  private readonly items_with_separators = once(() =>
    ast_internal.pickSeparated(this.cst, RuleKind.TupleDeconstructionElements),
  );

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TupleDeconstructionElement[] = once(() => {
    return this.items_with_separators()[0].map((item) => new TupleDeconstructionElement(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class PositionalArguments {
  private readonly items_with_separators = once(() =>
    ast_internal.pickSeparated(this.cst, RuleKind.PositionalArguments),
  );

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => Expression[] = once(() => {
    return this.items_with_separators()[0].map((item) => new Expression(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class NamedArguments {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.NamedArguments));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => NamedArgument[] = once(() => {
    return this.items_with_separators()[0].map((item) => new NamedArgument(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class TupleValues {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.TupleValues));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TupleValue[] = once(() => {
    return this.items_with_separators()[0].map((item) => new TupleValue(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class ArrayValues {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.ArrayValues));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => Expression[] = once(() => {
    return this.items_with_separators()[0].map((item) => new Expression(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class IdentifierPath {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.IdentifierPath));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    return this.items_with_separators()[0] as TokenNode[];
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class YulParameters {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.YulParameters));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    return this.items_with_separators()[0] as TokenNode[];
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class YulReturnVariables {
  private readonly items_with_separators = once(() =>
    ast_internal.pickSeparated(this.cst, RuleKind.YulReturnVariables),
  );

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    return this.items_with_separators()[0] as TokenNode[];
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class YulArguments {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.YulArguments));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => YulExpression[] = once(() => {
    return this.items_with_separators()[0].map((item) => new YulExpression(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class YulIdentifierPaths {
  private readonly items_with_separators = once(() =>
    ast_internal.pickSeparated(this.cst, RuleKind.YulIdentifierPaths),
  );

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => YulIdentifierPath[] = once(() => {
    return this.items_with_separators()[0].map((item) => new YulIdentifierPath(item as RuleNode));
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}

export class YulIdentifierPath {
  private readonly items_with_separators = once(() => ast_internal.pickSeparated(this.cst, RuleKind.YulIdentifierPath));

  public constructor(public readonly cst: RuleNode) {}

  public readonly items: () => TokenNode[] = once(() => {
    return this.items_with_separators()[0] as TokenNode[];
  });

  public readonly separators: () => TokenNode[] = once(() => {
    return this.items_with_separators()[1] as TokenNode[];
  });
}
