// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import * as assert from "node:assert";
import { ast_internal } from "../../generated";
import { RuleNode, TokenNode } from "../../cst";
import { RuleKind, TokenKind } from "../../kinds";

/*
 * Sequences:
 */

export class SourceUnit {
  private readonly fetch = once(() => {
    const [$members] = ast_internal.selectSequence(this.cst);

    return {
      members: $members === null ? undefined : new SourceUnitMembers($members as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.SourceUnit);
  }

  public get members(): SourceUnitMembers | undefined {
    return this.fetch().members;
  }
}

export class PragmaDirective {
  private readonly fetch = once(() => {
    const [$pragmaKeyword, $pragma, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      pragmaKeyword: $pragmaKeyword as TokenNode,
      pragma: new Pragma($pragma as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.PragmaDirective);
  }

  public get pragmaKeyword(): TokenNode {
    return this.fetch().pragmaKeyword;
  }

  public get pragma(): Pragma {
    return this.fetch().pragma;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class ABICoderPragma {
  private readonly fetch = once(() => {
    const [$abicoderKeyword, $version] = ast_internal.selectSequence(this.cst);

    return {
      abicoderKeyword: $abicoderKeyword as TokenNode,
      version: $version as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ABICoderPragma);
  }

  public get abicoderKeyword(): TokenNode {
    return this.fetch().abicoderKeyword;
  }

  public get version(): TokenNode {
    return this.fetch().version;
  }
}

export class ExperimentalPragma {
  private readonly fetch = once(() => {
    const [$experimentalKeyword, $feature] = ast_internal.selectSequence(this.cst);

    return {
      experimentalKeyword: $experimentalKeyword as TokenNode,
      feature: new ExperimentalFeature($feature as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ExperimentalPragma);
  }

  public get experimentalKeyword(): TokenNode {
    return this.fetch().experimentalKeyword;
  }

  public get feature(): ExperimentalFeature {
    return this.fetch().feature;
  }
}

export class VersionPragma {
  private readonly fetch = once(() => {
    const [$solidityKeyword, $expressions] = ast_internal.selectSequence(this.cst);

    return {
      solidityKeyword: $solidityKeyword as TokenNode,
      expressions: new VersionPragmaExpressions($expressions as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VersionPragma);
  }

  public get solidityKeyword(): TokenNode {
    return this.fetch().solidityKeyword;
  }

  public get expressions(): VersionPragmaExpressions {
    return this.fetch().expressions;
  }
}

export class VersionPragmaOrExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new VersionPragmaExpression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new VersionPragmaExpression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VersionPragmaOrExpression);
  }

  public get leftOperand(): VersionPragmaExpression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): VersionPragmaExpression {
    return this.fetch().rightOperand;
  }
}

export class VersionPragmaRangeExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new VersionPragmaExpression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new VersionPragmaExpression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VersionPragmaRangeExpression);
  }

  public get leftOperand(): VersionPragmaExpression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): VersionPragmaExpression {
    return this.fetch().rightOperand;
  }
}

export class VersionPragmaPrefixExpression {
  private readonly fetch = once(() => {
    const [$operand, $operator] = ast_internal.selectSequence(this.cst);

    return {
      operand: new VersionPragmaExpression($operand as RuleNode),
      operator: $operator as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VersionPragmaPrefixExpression);
  }

  public get operand(): VersionPragmaExpression {
    return this.fetch().operand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }
}

export class ImportDirective {
  private readonly fetch = once(() => {
    const [$importKeyword, $clause, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      importKeyword: $importKeyword as TokenNode,
      clause: new ImportClause($clause as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ImportDirective);
  }

  public get importKeyword(): TokenNode {
    return this.fetch().importKeyword;
  }

  public get clause(): ImportClause {
    return this.fetch().clause;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class PathImport {
  private readonly fetch = once(() => {
    const [$path, $alias] = ast_internal.selectSequence(this.cst);

    return {
      path: $path as TokenNode,
      alias: $alias === null ? undefined : new ImportAlias($alias as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.PathImport);
  }

  public get path(): TokenNode {
    return this.fetch().path;
  }

  public get alias(): ImportAlias | undefined {
    return this.fetch().alias;
  }
}

export class NamedImport {
  private readonly fetch = once(() => {
    const [$asterisk, $alias, $fromKeyword, $path] = ast_internal.selectSequence(this.cst);

    return {
      asterisk: $asterisk as TokenNode,
      alias: new ImportAlias($alias as RuleNode),
      fromKeyword: $fromKeyword as TokenNode,
      path: $path as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.NamedImport);
  }

  public get asterisk(): TokenNode {
    return this.fetch().asterisk;
  }

  public get alias(): ImportAlias {
    return this.fetch().alias;
  }

  public get fromKeyword(): TokenNode {
    return this.fetch().fromKeyword;
  }

  public get path(): TokenNode {
    return this.fetch().path;
  }
}

export class ImportDeconstruction {
  private readonly fetch = once(() => {
    const [$openBrace, $symbols, $closeBrace, $fromKeyword, $path] = ast_internal.selectSequence(this.cst);

    return {
      openBrace: $openBrace as TokenNode,
      symbols: new ImportDeconstructionSymbols($symbols as RuleNode),
      closeBrace: $closeBrace as TokenNode,
      fromKeyword: $fromKeyword as TokenNode,
      path: $path as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ImportDeconstruction);
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get symbols(): ImportDeconstructionSymbols {
    return this.fetch().symbols;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }

  public get fromKeyword(): TokenNode {
    return this.fetch().fromKeyword;
  }

  public get path(): TokenNode {
    return this.fetch().path;
  }
}

export class ImportDeconstructionSymbol {
  private readonly fetch = once(() => {
    const [$name, $alias] = ast_internal.selectSequence(this.cst);

    return {
      name: $name as TokenNode,
      alias: $alias === null ? undefined : new ImportAlias($alias as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ImportDeconstructionSymbol);
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get alias(): ImportAlias | undefined {
    return this.fetch().alias;
  }
}

export class ImportAlias {
  private readonly fetch = once(() => {
    const [$asKeyword, $identifier] = ast_internal.selectSequence(this.cst);

    return {
      asKeyword: $asKeyword as TokenNode,
      identifier: $identifier as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ImportAlias);
  }

  public get asKeyword(): TokenNode {
    return this.fetch().asKeyword;
  }

  public get identifier(): TokenNode {
    return this.fetch().identifier;
  }
}

export class UsingDirective {
  private readonly fetch = once(() => {
    const [$usingKeyword, $clause, $forKeyword, $target, $globalKeyword, $semicolon] = ast_internal.selectSequence(
      this.cst,
    );

    return {
      usingKeyword: $usingKeyword as TokenNode,
      clause: new UsingClause($clause as RuleNode),
      forKeyword: $forKeyword as TokenNode,
      target: new UsingTarget($target as RuleNode),
      globalKeyword: $globalKeyword === null ? undefined : ($globalKeyword as TokenNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UsingDirective);
  }

  public get usingKeyword(): TokenNode {
    return this.fetch().usingKeyword;
  }

  public get clause(): UsingClause {
    return this.fetch().clause;
  }

  public get forKeyword(): TokenNode {
    return this.fetch().forKeyword;
  }

  public get target(): UsingTarget {
    return this.fetch().target;
  }

  public get globalKeyword(): TokenNode | undefined {
    return this.fetch().globalKeyword;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class UsingDeconstruction {
  private readonly fetch = once(() => {
    const [$openBrace, $symbols, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      openBrace: $openBrace as TokenNode,
      symbols: new UsingDeconstructionSymbols($symbols as RuleNode),
      closeBrace: $closeBrace as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UsingDeconstruction);
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get symbols(): UsingDeconstructionSymbols {
    return this.fetch().symbols;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }
}

export class UsingDeconstructionSymbol {
  private readonly fetch = once(() => {
    const [$name, $alias] = ast_internal.selectSequence(this.cst);

    return {
      name: new IdentifierPath($name as RuleNode),
      alias: $alias === null ? undefined : new UsingAlias($alias as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UsingDeconstructionSymbol);
  }

  public get name(): IdentifierPath {
    return this.fetch().name;
  }

  public get alias(): UsingAlias | undefined {
    return this.fetch().alias;
  }
}

export class UsingAlias {
  private readonly fetch = once(() => {
    const [$asKeyword, $operator] = ast_internal.selectSequence(this.cst);

    return {
      asKeyword: $asKeyword as TokenNode,
      operator: new UsingOperator($operator as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UsingAlias);
  }

  public get asKeyword(): TokenNode {
    return this.fetch().asKeyword;
  }

  public get operator(): UsingOperator {
    return this.fetch().operator;
  }
}

export class ContractDefinition {
  private readonly fetch = once(() => {
    const [$abstractKeyword, $contractKeyword, $name, $inheritence, $openBrace, $members, $closeBrace] =
      ast_internal.selectSequence(this.cst);

    return {
      abstractKeyword: $abstractKeyword === null ? undefined : ($abstractKeyword as TokenNode),
      contractKeyword: $contractKeyword as TokenNode,
      name: $name as TokenNode,
      inheritence: $inheritence === null ? undefined : new InheritanceSpecifier($inheritence as RuleNode),
      openBrace: $openBrace as TokenNode,
      members: $members === null ? undefined : new ContractMembers($members as RuleNode),
      closeBrace: $closeBrace as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ContractDefinition);
  }

  public get abstractKeyword(): TokenNode | undefined {
    return this.fetch().abstractKeyword;
  }

  public get contractKeyword(): TokenNode {
    return this.fetch().contractKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get inheritence(): InheritanceSpecifier | undefined {
    return this.fetch().inheritence;
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get members(): ContractMembers | undefined {
    return this.fetch().members;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }
}

export class InheritanceSpecifier {
  private readonly fetch = once(() => {
    const [$isKeyword, $types] = ast_internal.selectSequence(this.cst);

    return {
      isKeyword: $isKeyword as TokenNode,
      types: new InheritanceTypes($types as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.InheritanceSpecifier);
  }

  public get isKeyword(): TokenNode {
    return this.fetch().isKeyword;
  }

  public get types(): InheritanceTypes {
    return this.fetch().types;
  }
}

export class InheritanceType {
  private readonly fetch = once(() => {
    const [$typeName, $arguments] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new IdentifierPath($typeName as RuleNode),
      arguments: $arguments === null ? undefined : new ArgumentsDeclaration($arguments as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.InheritanceType);
  }

  public get typeName(): IdentifierPath {
    return this.fetch().typeName;
  }

  public get arguments(): ArgumentsDeclaration | undefined {
    return this.fetch().arguments;
  }
}

export class InterfaceDefinition {
  private readonly fetch = once(() => {
    const [$interfaceKeyword, $name, $inheritence, $openBrace, $members, $closeBrace] = ast_internal.selectSequence(
      this.cst,
    );

    return {
      interfaceKeyword: $interfaceKeyword as TokenNode,
      name: $name as TokenNode,
      inheritence: $inheritence === null ? undefined : new InheritanceSpecifier($inheritence as RuleNode),
      openBrace: $openBrace as TokenNode,
      members: $members === null ? undefined : new InterfaceMembers($members as RuleNode),
      closeBrace: $closeBrace as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.InterfaceDefinition);
  }

  public get interfaceKeyword(): TokenNode {
    return this.fetch().interfaceKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get inheritence(): InheritanceSpecifier | undefined {
    return this.fetch().inheritence;
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get members(): InterfaceMembers | undefined {
    return this.fetch().members;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }
}

export class LibraryDefinition {
  private readonly fetch = once(() => {
    const [$libraryKeyword, $name, $openBrace, $members, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      libraryKeyword: $libraryKeyword as TokenNode,
      name: $name as TokenNode,
      openBrace: $openBrace as TokenNode,
      members: $members === null ? undefined : new LibraryMembers($members as RuleNode),
      closeBrace: $closeBrace as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.LibraryDefinition);
  }

  public get libraryKeyword(): TokenNode {
    return this.fetch().libraryKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get members(): LibraryMembers | undefined {
    return this.fetch().members;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }
}

export class StructDefinition {
  private readonly fetch = once(() => {
    const [$structKeyword, $name, $openBrace, $members, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      structKeyword: $structKeyword as TokenNode,
      name: $name as TokenNode,
      openBrace: $openBrace as TokenNode,
      members: $members === null ? undefined : new StructMembers($members as RuleNode),
      closeBrace: $closeBrace as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.StructDefinition);
  }

  public get structKeyword(): TokenNode {
    return this.fetch().structKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get members(): StructMembers | undefined {
    return this.fetch().members;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }
}

export class StructMember {
  private readonly fetch = once(() => {
    const [$typeName, $name, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as RuleNode),
      name: $name as TokenNode,
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.StructMember);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class EnumDefinition {
  private readonly fetch = once(() => {
    const [$enumKeyword, $name, $openBrace, $members, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      enumKeyword: $enumKeyword as TokenNode,
      name: $name as TokenNode,
      openBrace: $openBrace as TokenNode,
      members: $members === null ? undefined : new EnumMembers($members as RuleNode),
      closeBrace: $closeBrace as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.EnumDefinition);
  }

  public get enumKeyword(): TokenNode {
    return this.fetch().enumKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get members(): EnumMembers | undefined {
    return this.fetch().members;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }
}

export class ConstantDefinition {
  private readonly fetch = once(() => {
    const [$typeName, $constantKeyword, $name, $equal, $value, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as RuleNode),
      constantKeyword: $constantKeyword as TokenNode,
      name: $name as TokenNode,
      equal: $equal as TokenNode,
      value: new Expression($value as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ConstantDefinition);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get constantKeyword(): TokenNode {
    return this.fetch().constantKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get equal(): TokenNode {
    return this.fetch().equal;
  }

  public get value(): Expression {
    return this.fetch().value;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class StateVariableDefinition {
  private readonly fetch = once(() => {
    const [$typeName, $attributes, $name, $value, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as RuleNode),
      attributes: $attributes === null ? undefined : new StateVariableAttributes($attributes as RuleNode),
      name: $name as TokenNode,
      value: $value === null ? undefined : new StateVariableDefinitionValue($value as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.StateVariableDefinition);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get attributes(): StateVariableAttributes | undefined {
    return this.fetch().attributes;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get value(): StateVariableDefinitionValue | undefined {
    return this.fetch().value;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class StateVariableDefinitionValue {
  private readonly fetch = once(() => {
    const [$equal, $value] = ast_internal.selectSequence(this.cst);

    return {
      equal: $equal as TokenNode,
      value: new Expression($value as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.StateVariableDefinitionValue);
  }

  public get equal(): TokenNode {
    return this.fetch().equal;
  }

  public get value(): Expression {
    return this.fetch().value;
  }
}

export class FunctionDefinition {
  private readonly fetch = once(() => {
    const [$functionKeyword, $name, $parameters, $attributes, $returns, $body] = ast_internal.selectSequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TokenNode,
      name: new FunctionName($name as RuleNode),
      parameters: new ParametersDeclaration($parameters as RuleNode),
      attributes: $attributes === null ? undefined : new FunctionAttributes($attributes as RuleNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as RuleNode),
      body: new FunctionBody($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionDefinition);
  }

  public get functionKeyword(): TokenNode {
    return this.fetch().functionKeyword;
  }

  public get name(): FunctionName {
    return this.fetch().name;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): FunctionAttributes | undefined {
    return this.fetch().attributes;
  }

  public get returns(): ReturnsDeclaration | undefined {
    return this.fetch().returns;
  }

  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

export class ParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      parameters: $parameters === null ? undefined : new Parameters($parameters as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ParametersDeclaration);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get parameters(): Parameters | undefined {
    return this.fetch().parameters;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class Parameter {
  private readonly fetch = once(() => {
    const [$typeName, $storageLocation, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as RuleNode),
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as RuleNode),
      name: $name === null ? undefined : ($name as TokenNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.Parameter);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  public get name(): TokenNode | undefined {
    return this.fetch().name;
  }
}

export class OverrideSpecifier {
  private readonly fetch = once(() => {
    const [$overrideKeyword, $overridden] = ast_internal.selectSequence(this.cst);

    return {
      overrideKeyword: $overrideKeyword as TokenNode,
      overridden: $overridden === null ? undefined : new OverridePathsDeclaration($overridden as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.OverrideSpecifier);
  }

  public get overrideKeyword(): TokenNode {
    return this.fetch().overrideKeyword;
  }

  public get overridden(): OverridePathsDeclaration | undefined {
    return this.fetch().overridden;
  }
}

export class OverridePathsDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $paths, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      paths: new OverridePaths($paths as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.OverridePathsDeclaration);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get paths(): OverridePaths {
    return this.fetch().paths;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class ReturnsDeclaration {
  private readonly fetch = once(() => {
    const [$returnsKeyword, $variables] = ast_internal.selectSequence(this.cst);

    return {
      returnsKeyword: $returnsKeyword as TokenNode,
      variables: new ParametersDeclaration($variables as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ReturnsDeclaration);
  }

  public get returnsKeyword(): TokenNode {
    return this.fetch().returnsKeyword;
  }

  public get variables(): ParametersDeclaration {
    return this.fetch().variables;
  }
}

export class ConstructorDefinition {
  private readonly fetch = once(() => {
    const [$constructorKeyword, $parameters, $attributes, $body] = ast_internal.selectSequence(this.cst);

    return {
      constructorKeyword: $constructorKeyword as TokenNode,
      parameters: new ParametersDeclaration($parameters as RuleNode),
      attributes: $attributes === null ? undefined : new ConstructorAttributes($attributes as RuleNode),
      body: new Block($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ConstructorDefinition);
  }

  public get constructorKeyword(): TokenNode {
    return this.fetch().constructorKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): ConstructorAttributes | undefined {
    return this.fetch().attributes;
  }

  public get body(): Block {
    return this.fetch().body;
  }
}

export class UnnamedFunctionDefinition {
  private readonly fetch = once(() => {
    const [$functionKeyword, $parameters, $attributes, $body] = ast_internal.selectSequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TokenNode,
      parameters: new ParametersDeclaration($parameters as RuleNode),
      attributes: $attributes === null ? undefined : new UnnamedFunctionAttributes($attributes as RuleNode),
      body: new FunctionBody($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UnnamedFunctionDefinition);
  }

  public get functionKeyword(): TokenNode {
    return this.fetch().functionKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): UnnamedFunctionAttributes | undefined {
    return this.fetch().attributes;
  }

  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

export class FallbackFunctionDefinition {
  private readonly fetch = once(() => {
    const [$fallbackKeyword, $parameters, $attributes, $returns, $body] = ast_internal.selectSequence(this.cst);

    return {
      fallbackKeyword: $fallbackKeyword as TokenNode,
      parameters: new ParametersDeclaration($parameters as RuleNode),
      attributes: $attributes === null ? undefined : new FallbackFunctionAttributes($attributes as RuleNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as RuleNode),
      body: new FunctionBody($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FallbackFunctionDefinition);
  }

  public get fallbackKeyword(): TokenNode {
    return this.fetch().fallbackKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): FallbackFunctionAttributes | undefined {
    return this.fetch().attributes;
  }

  public get returns(): ReturnsDeclaration | undefined {
    return this.fetch().returns;
  }

  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

export class ReceiveFunctionDefinition {
  private readonly fetch = once(() => {
    const [$receiveKeyword, $parameters, $attributes, $body] = ast_internal.selectSequence(this.cst);

    return {
      receiveKeyword: $receiveKeyword as TokenNode,
      parameters: new ParametersDeclaration($parameters as RuleNode),
      attributes: $attributes === null ? undefined : new ReceiveFunctionAttributes($attributes as RuleNode),
      body: new FunctionBody($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ReceiveFunctionDefinition);
  }

  public get receiveKeyword(): TokenNode {
    return this.fetch().receiveKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): ReceiveFunctionAttributes | undefined {
    return this.fetch().attributes;
  }

  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

export class ModifierDefinition {
  private readonly fetch = once(() => {
    const [$modifierKeyword, $name, $parameters, $attributes, $body] = ast_internal.selectSequence(this.cst);

    return {
      modifierKeyword: $modifierKeyword as TokenNode,
      name: $name as TokenNode,
      parameters: $parameters === null ? undefined : new ParametersDeclaration($parameters as RuleNode),
      attributes: $attributes === null ? undefined : new ModifierAttributes($attributes as RuleNode),
      body: new FunctionBody($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ModifierDefinition);
  }

  public get modifierKeyword(): TokenNode {
    return this.fetch().modifierKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get parameters(): ParametersDeclaration | undefined {
    return this.fetch().parameters;
  }

  public get attributes(): ModifierAttributes | undefined {
    return this.fetch().attributes;
  }

  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

export class ModifierInvocation {
  private readonly fetch = once(() => {
    const [$name, $arguments] = ast_internal.selectSequence(this.cst);

    return {
      name: new IdentifierPath($name as RuleNode),
      arguments: $arguments === null ? undefined : new ArgumentsDeclaration($arguments as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ModifierInvocation);
  }

  public get name(): IdentifierPath {
    return this.fetch().name;
  }

  public get arguments(): ArgumentsDeclaration | undefined {
    return this.fetch().arguments;
  }
}

export class EventDefinition {
  private readonly fetch = once(() => {
    const [$eventKeyword, $name, $parameters, $anonymousKeyword, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      eventKeyword: $eventKeyword as TokenNode,
      name: $name as TokenNode,
      parameters: new EventParametersDeclaration($parameters as RuleNode),
      anonymousKeyword: $anonymousKeyword === null ? undefined : ($anonymousKeyword as TokenNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.EventDefinition);
  }

  public get eventKeyword(): TokenNode {
    return this.fetch().eventKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get parameters(): EventParametersDeclaration {
    return this.fetch().parameters;
  }

  public get anonymousKeyword(): TokenNode | undefined {
    return this.fetch().anonymousKeyword;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class EventParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      parameters: $parameters === null ? undefined : new EventParameters($parameters as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.EventParametersDeclaration);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get parameters(): EventParameters | undefined {
    return this.fetch().parameters;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class EventParameter {
  private readonly fetch = once(() => {
    const [$typeName, $indexedKeyword, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as RuleNode),
      indexedKeyword: $indexedKeyword === null ? undefined : ($indexedKeyword as TokenNode),
      name: $name === null ? undefined : ($name as TokenNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.EventParameter);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get indexedKeyword(): TokenNode | undefined {
    return this.fetch().indexedKeyword;
  }

  public get name(): TokenNode | undefined {
    return this.fetch().name;
  }
}

export class UserDefinedValueTypeDefinition {
  private readonly fetch = once(() => {
    const [$typeKeyword, $name, $isKeyword, $valueType, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      typeKeyword: $typeKeyword as TokenNode,
      name: $name as TokenNode,
      isKeyword: $isKeyword as TokenNode,
      valueType: new ElementaryType($valueType as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UserDefinedValueTypeDefinition);
  }

  public get typeKeyword(): TokenNode {
    return this.fetch().typeKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get isKeyword(): TokenNode {
    return this.fetch().isKeyword;
  }

  public get valueType(): ElementaryType {
    return this.fetch().valueType;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class ErrorDefinition {
  private readonly fetch = once(() => {
    const [$errorKeyword, $name, $members, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      errorKeyword: $errorKeyword as TokenNode,
      name: $name as TokenNode,
      members: new ErrorParametersDeclaration($members as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ErrorDefinition);
  }

  public get errorKeyword(): TokenNode {
    return this.fetch().errorKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get members(): ErrorParametersDeclaration {
    return this.fetch().members;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class ErrorParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      parameters: $parameters === null ? undefined : new ErrorParameters($parameters as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ErrorParametersDeclaration);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get parameters(): ErrorParameters | undefined {
    return this.fetch().parameters;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class ErrorParameter {
  private readonly fetch = once(() => {
    const [$typeName, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as RuleNode),
      name: $name === null ? undefined : ($name as TokenNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ErrorParameter);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get name(): TokenNode | undefined {
    return this.fetch().name;
  }
}

export class ArrayTypeName {
  private readonly fetch = once(() => {
    const [$openBracket, $index, $closeBracket, $operand] = ast_internal.selectSequence(this.cst);

    return {
      openBracket: $openBracket as TokenNode,
      index: $index === null ? undefined : new Expression($index as RuleNode),
      closeBracket: $closeBracket as TokenNode,
      operand: new TypeName($operand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ArrayTypeName);
  }

  public get openBracket(): TokenNode {
    return this.fetch().openBracket;
  }

  public get index(): Expression | undefined {
    return this.fetch().index;
  }

  public get closeBracket(): TokenNode {
    return this.fetch().closeBracket;
  }

  public get operand(): TypeName {
    return this.fetch().operand;
  }
}

export class FunctionType {
  private readonly fetch = once(() => {
    const [$functionKeyword, $parameters, $attributes, $returns] = ast_internal.selectSequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TokenNode,
      parameters: new ParametersDeclaration($parameters as RuleNode),
      attributes: $attributes === null ? undefined : new FunctionTypeAttributes($attributes as RuleNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionType);
  }

  public get functionKeyword(): TokenNode {
    return this.fetch().functionKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): FunctionTypeAttributes | undefined {
    return this.fetch().attributes;
  }

  public get returns(): ReturnsDeclaration | undefined {
    return this.fetch().returns;
  }
}

export class MappingType {
  private readonly fetch = once(() => {
    const [$mappingKeyword, $openParen, $keyType, $equalGreaterThan, $valueType, $closeParen] =
      ast_internal.selectSequence(this.cst);

    return {
      mappingKeyword: $mappingKeyword as TokenNode,
      openParen: $openParen as TokenNode,
      keyType: new MappingKey($keyType as RuleNode),
      equalGreaterThan: $equalGreaterThan as TokenNode,
      valueType: new MappingValue($valueType as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.MappingType);
  }

  public get mappingKeyword(): TokenNode {
    return this.fetch().mappingKeyword;
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get keyType(): MappingKey {
    return this.fetch().keyType;
  }

  public get equalGreaterThan(): TokenNode {
    return this.fetch().equalGreaterThan;
  }

  public get valueType(): MappingValue {
    return this.fetch().valueType;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class MappingKey {
  private readonly fetch = once(() => {
    const [$keyType, $name] = ast_internal.selectSequence(this.cst);

    return {
      keyType: new MappingKeyType($keyType as RuleNode),
      name: $name === null ? undefined : ($name as TokenNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.MappingKey);
  }

  public get keyType(): MappingKeyType {
    return this.fetch().keyType;
  }

  public get name(): TokenNode | undefined {
    return this.fetch().name;
  }
}

export class MappingValue {
  private readonly fetch = once(() => {
    const [$typeName, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as RuleNode),
      name: $name === null ? undefined : ($name as TokenNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.MappingValue);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get name(): TokenNode | undefined {
    return this.fetch().name;
  }
}

export class AddressType {
  private readonly fetch = once(() => {
    const [$addressKeyword, $payableKeyword] = ast_internal.selectSequence(this.cst);

    return {
      addressKeyword: $addressKeyword as TokenNode,
      payableKeyword: $payableKeyword === null ? undefined : ($payableKeyword as TokenNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.AddressType);
  }

  public get addressKeyword(): TokenNode {
    return this.fetch().addressKeyword;
  }

  public get payableKeyword(): TokenNode | undefined {
    return this.fetch().payableKeyword;
  }
}

export class Block {
  private readonly fetch = once(() => {
    const [$openBrace, $statements, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      openBrace: $openBrace as TokenNode,
      statements: $statements === null ? undefined : new Statements($statements as RuleNode),
      closeBrace: $closeBrace as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.Block);
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get statements(): Statements | undefined {
    return this.fetch().statements;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }
}

export class UncheckedBlock {
  private readonly fetch = once(() => {
    const [$uncheckedKeyword, $block] = ast_internal.selectSequence(this.cst);

    return {
      uncheckedKeyword: $uncheckedKeyword as TokenNode,
      block: new Block($block as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UncheckedBlock);
  }

  public get uncheckedKeyword(): TokenNode {
    return this.fetch().uncheckedKeyword;
  }

  public get block(): Block {
    return this.fetch().block;
  }
}

export class ExpressionStatement {
  private readonly fetch = once(() => {
    const [$expression, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      expression: new Expression($expression as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ExpressionStatement);
  }

  public get expression(): Expression {
    return this.fetch().expression;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class AssemblyStatement {
  private readonly fetch = once(() => {
    const [$assemblyKeyword, $label, $flags, $body] = ast_internal.selectSequence(this.cst);

    return {
      assemblyKeyword: $assemblyKeyword as TokenNode,
      label: $label === null ? undefined : ($label as TokenNode),
      flags: $flags === null ? undefined : new AssemblyFlagsDeclaration($flags as RuleNode),
      body: new YulBlock($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.AssemblyStatement);
  }

  public get assemblyKeyword(): TokenNode {
    return this.fetch().assemblyKeyword;
  }

  public get label(): TokenNode | undefined {
    return this.fetch().label;
  }

  public get flags(): AssemblyFlagsDeclaration | undefined {
    return this.fetch().flags;
  }

  public get body(): YulBlock {
    return this.fetch().body;
  }
}

export class AssemblyFlagsDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $flags, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      flags: new AssemblyFlags($flags as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.AssemblyFlagsDeclaration);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get flags(): AssemblyFlags {
    return this.fetch().flags;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class TupleDeconstructionStatement {
  private readonly fetch = once(() => {
    const [$openParen, $elements, $closeParen, $equal, $expression, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      elements: new TupleDeconstructionElements($elements as RuleNode),
      closeParen: $closeParen as TokenNode,
      equal: $equal as TokenNode,
      expression: new Expression($expression as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TupleDeconstructionStatement);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get elements(): TupleDeconstructionElements {
    return this.fetch().elements;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }

  public get equal(): TokenNode {
    return this.fetch().equal;
  }

  public get expression(): Expression {
    return this.fetch().expression;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class TupleDeconstructionElement {
  private readonly fetch = once(() => {
    const [$member] = ast_internal.selectSequence(this.cst);

    return {
      member: $member === null ? undefined : new TupleMember($member as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TupleDeconstructionElement);
  }

  public get member(): TupleMember | undefined {
    return this.fetch().member;
  }
}

export class TypedTupleMember {
  private readonly fetch = once(() => {
    const [$typeName, $storageLocation, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as RuleNode),
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as RuleNode),
      name: $name as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TypedTupleMember);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }
}

export class UntypedTupleMember {
  private readonly fetch = once(() => {
    const [$storageLocation, $name] = ast_internal.selectSequence(this.cst);

    return {
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as RuleNode),
      name: $name as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UntypedTupleMember);
  }

  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }
}

export class VariableDeclarationStatement {
  private readonly fetch = once(() => {
    const [$variableType, $storageLocation, $name, $value, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      variableType: new VariableDeclarationType($variableType as RuleNode),
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as RuleNode),
      name: $name as TokenNode,
      value: $value === null ? undefined : new VariableDeclarationValue($value as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VariableDeclarationStatement);
  }

  public get variableType(): VariableDeclarationType {
    return this.fetch().variableType;
  }

  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get value(): VariableDeclarationValue | undefined {
    return this.fetch().value;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class VariableDeclarationValue {
  private readonly fetch = once(() => {
    const [$equal, $expression] = ast_internal.selectSequence(this.cst);

    return {
      equal: $equal as TokenNode,
      expression: new Expression($expression as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VariableDeclarationValue);
  }

  public get equal(): TokenNode {
    return this.fetch().equal;
  }

  public get expression(): Expression {
    return this.fetch().expression;
  }
}

export class IfStatement {
  private readonly fetch = once(() => {
    const [$ifKeyword, $openParen, $condition, $closeParen, $body, $elseBranch] = ast_internal.selectSequence(this.cst);

    return {
      ifKeyword: $ifKeyword as TokenNode,
      openParen: $openParen as TokenNode,
      condition: new Expression($condition as RuleNode),
      closeParen: $closeParen as TokenNode,
      body: new Statement($body as RuleNode),
      elseBranch: $elseBranch === null ? undefined : new ElseBranch($elseBranch as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.IfStatement);
  }

  public get ifKeyword(): TokenNode {
    return this.fetch().ifKeyword;
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get condition(): Expression {
    return this.fetch().condition;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }

  public get body(): Statement {
    return this.fetch().body;
  }

  public get elseBranch(): ElseBranch | undefined {
    return this.fetch().elseBranch;
  }
}

export class ElseBranch {
  private readonly fetch = once(() => {
    const [$elseKeyword, $body] = ast_internal.selectSequence(this.cst);

    return {
      elseKeyword: $elseKeyword as TokenNode,
      body: new Statement($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ElseBranch);
  }

  public get elseKeyword(): TokenNode {
    return this.fetch().elseKeyword;
  }

  public get body(): Statement {
    return this.fetch().body;
  }
}

export class ForStatement {
  private readonly fetch = once(() => {
    const [$forKeyword, $openParen, $initialization, $condition, $iterator, $closeParen, $body] =
      ast_internal.selectSequence(this.cst);

    return {
      forKeyword: $forKeyword as TokenNode,
      openParen: $openParen as TokenNode,
      initialization: new ForStatementInitialization($initialization as RuleNode),
      condition: new ForStatementCondition($condition as RuleNode),
      iterator: $iterator === null ? undefined : new Expression($iterator as RuleNode),
      closeParen: $closeParen as TokenNode,
      body: new Statement($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ForStatement);
  }

  public get forKeyword(): TokenNode {
    return this.fetch().forKeyword;
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get initialization(): ForStatementInitialization {
    return this.fetch().initialization;
  }

  public get condition(): ForStatementCondition {
    return this.fetch().condition;
  }

  public get iterator(): Expression | undefined {
    return this.fetch().iterator;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }

  public get body(): Statement {
    return this.fetch().body;
  }
}

export class WhileStatement {
  private readonly fetch = once(() => {
    const [$whileKeyword, $openParen, $condition, $closeParen, $body] = ast_internal.selectSequence(this.cst);

    return {
      whileKeyword: $whileKeyword as TokenNode,
      openParen: $openParen as TokenNode,
      condition: new Expression($condition as RuleNode),
      closeParen: $closeParen as TokenNode,
      body: new Statement($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.WhileStatement);
  }

  public get whileKeyword(): TokenNode {
    return this.fetch().whileKeyword;
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get condition(): Expression {
    return this.fetch().condition;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }

  public get body(): Statement {
    return this.fetch().body;
  }
}

export class DoWhileStatement {
  private readonly fetch = once(() => {
    const [$doKeyword, $body, $whileKeyword, $openParen, $condition, $closeParen, $semicolon] =
      ast_internal.selectSequence(this.cst);

    return {
      doKeyword: $doKeyword as TokenNode,
      body: new Statement($body as RuleNode),
      whileKeyword: $whileKeyword as TokenNode,
      openParen: $openParen as TokenNode,
      condition: new Expression($condition as RuleNode),
      closeParen: $closeParen as TokenNode,
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.DoWhileStatement);
  }

  public get doKeyword(): TokenNode {
    return this.fetch().doKeyword;
  }

  public get body(): Statement {
    return this.fetch().body;
  }

  public get whileKeyword(): TokenNode {
    return this.fetch().whileKeyword;
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get condition(): Expression {
    return this.fetch().condition;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class ContinueStatement {
  private readonly fetch = once(() => {
    const [$continueKeyword, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      continueKeyword: $continueKeyword as TokenNode,
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ContinueStatement);
  }

  public get continueKeyword(): TokenNode {
    return this.fetch().continueKeyword;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class BreakStatement {
  private readonly fetch = once(() => {
    const [$breakKeyword, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      breakKeyword: $breakKeyword as TokenNode,
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.BreakStatement);
  }

  public get breakKeyword(): TokenNode {
    return this.fetch().breakKeyword;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class ReturnStatement {
  private readonly fetch = once(() => {
    const [$returnKeyword, $expression, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      returnKeyword: $returnKeyword as TokenNode,
      expression: $expression === null ? undefined : new Expression($expression as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ReturnStatement);
  }

  public get returnKeyword(): TokenNode {
    return this.fetch().returnKeyword;
  }

  public get expression(): Expression | undefined {
    return this.fetch().expression;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class EmitStatement {
  private readonly fetch = once(() => {
    const [$emitKeyword, $event, $arguments, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      emitKeyword: $emitKeyword as TokenNode,
      event: new IdentifierPath($event as RuleNode),
      arguments: new ArgumentsDeclaration($arguments as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.EmitStatement);
  }

  public get emitKeyword(): TokenNode {
    return this.fetch().emitKeyword;
  }

  public get event(): IdentifierPath {
    return this.fetch().event;
  }

  public get arguments(): ArgumentsDeclaration {
    return this.fetch().arguments;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class DeleteStatement {
  private readonly fetch = once(() => {
    const [$deleteKeyword, $expression, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      deleteKeyword: $deleteKeyword as TokenNode,
      expression: new Expression($expression as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.DeleteStatement);
  }

  public get deleteKeyword(): TokenNode {
    return this.fetch().deleteKeyword;
  }

  public get expression(): Expression {
    return this.fetch().expression;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class TryStatement {
  private readonly fetch = once(() => {
    const [$tryKeyword, $expression, $returns, $body, $catchClauses] = ast_internal.selectSequence(this.cst);

    return {
      tryKeyword: $tryKeyword as TokenNode,
      expression: new Expression($expression as RuleNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as RuleNode),
      body: new Block($body as RuleNode),
      catchClauses: new CatchClauses($catchClauses as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TryStatement);
  }

  public get tryKeyword(): TokenNode {
    return this.fetch().tryKeyword;
  }

  public get expression(): Expression {
    return this.fetch().expression;
  }

  public get returns(): ReturnsDeclaration | undefined {
    return this.fetch().returns;
  }

  public get body(): Block {
    return this.fetch().body;
  }

  public get catchClauses(): CatchClauses {
    return this.fetch().catchClauses;
  }
}

export class CatchClause {
  private readonly fetch = once(() => {
    const [$catchKeyword, $error, $body] = ast_internal.selectSequence(this.cst);

    return {
      catchKeyword: $catchKeyword as TokenNode,
      error: $error === null ? undefined : new CatchClauseError($error as RuleNode),
      body: new Block($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.CatchClause);
  }

  public get catchKeyword(): TokenNode {
    return this.fetch().catchKeyword;
  }

  public get error(): CatchClauseError | undefined {
    return this.fetch().error;
  }

  public get body(): Block {
    return this.fetch().body;
  }
}

export class CatchClauseError {
  private readonly fetch = once(() => {
    const [$name, $parameters] = ast_internal.selectSequence(this.cst);

    return {
      name: $name === null ? undefined : ($name as TokenNode),
      parameters: new ParametersDeclaration($parameters as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.CatchClauseError);
  }

  public get name(): TokenNode | undefined {
    return this.fetch().name;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }
}

export class RevertStatement {
  private readonly fetch = once(() => {
    const [$revertKeyword, $error, $arguments, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      revertKeyword: $revertKeyword as TokenNode,
      error: $error === null ? undefined : new IdentifierPath($error as RuleNode),
      arguments: new ArgumentsDeclaration($arguments as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.RevertStatement);
  }

  public get revertKeyword(): TokenNode {
    return this.fetch().revertKeyword;
  }

  public get error(): IdentifierPath | undefined {
    return this.fetch().error;
  }

  public get arguments(): ArgumentsDeclaration {
    return this.fetch().arguments;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class ThrowStatement {
  private readonly fetch = once(() => {
    const [$throwKeyword, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      throwKeyword: $throwKeyword as TokenNode,
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ThrowStatement);
  }

  public get throwKeyword(): TokenNode {
    return this.fetch().throwKeyword;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class AssignmentExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.AssignmentExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class ConditionalExpression {
  private readonly fetch = once(() => {
    const [$questionMark, $trueExpression, $colon, $falseExpression, $operand] = ast_internal.selectSequence(this.cst);

    return {
      questionMark: $questionMark as TokenNode,
      trueExpression: new Expression($trueExpression as RuleNode),
      colon: $colon as TokenNode,
      falseExpression: new Expression($falseExpression as RuleNode),
      operand: new Expression($operand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ConditionalExpression);
  }

  public get questionMark(): TokenNode {
    return this.fetch().questionMark;
  }

  public get trueExpression(): Expression {
    return this.fetch().trueExpression;
  }

  public get colon(): TokenNode {
    return this.fetch().colon;
  }

  public get falseExpression(): Expression {
    return this.fetch().falseExpression;
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }
}

export class OrExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.OrExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class AndExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.AndExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class EqualityExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.EqualityExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class ComparisonExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ComparisonExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class BitwiseOrExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.BitwiseOrExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class BitwiseXorExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.BitwiseXorExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class BitwiseAndExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.BitwiseAndExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class ShiftExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ShiftExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class AdditiveExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.AdditiveExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class MultiplicativeExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.MultiplicativeExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class ExponentiationExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as RuleNode),
      operator: $operator as TokenNode,
      rightOperand: new Expression($rightOperand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ExponentiationExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class PostfixExpression {
  private readonly fetch = once(() => {
    const [$operator, $operand] = ast_internal.selectSequence(this.cst);

    return {
      operator: $operator as TokenNode,
      operand: new Expression($operand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.PostfixExpression);
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }
}

export class PrefixExpression {
  private readonly fetch = once(() => {
    const [$operand, $operator] = ast_internal.selectSequence(this.cst);

    return {
      operand: new Expression($operand as RuleNode),
      operator: $operator as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.PrefixExpression);
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }

  public get operator(): TokenNode {
    return this.fetch().operator;
  }
}

export class FunctionCallExpression {
  private readonly fetch = once(() => {
    const [$options, $arguments, $operand] = ast_internal.selectSequence(this.cst);

    return {
      options: $options === null ? undefined : new FunctionCallOptions($options as RuleNode),
      arguments: new ArgumentsDeclaration($arguments as RuleNode),
      operand: new Expression($operand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionCallExpression);
  }

  public get options(): FunctionCallOptions | undefined {
    return this.fetch().options;
  }

  public get arguments(): ArgumentsDeclaration {
    return this.fetch().arguments;
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }
}

export class MemberAccessExpression {
  private readonly fetch = once(() => {
    const [$period, $member, $operand] = ast_internal.selectSequence(this.cst);

    return {
      period: $period as TokenNode,
      member: new MemberAccess($member as RuleNode),
      operand: new Expression($operand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.MemberAccessExpression);
  }

  public get period(): TokenNode {
    return this.fetch().period;
  }

  public get member(): MemberAccess {
    return this.fetch().member;
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }
}

export class IndexAccessExpression {
  private readonly fetch = once(() => {
    const [$openBracket, $start, $end, $closeBracket, $operand] = ast_internal.selectSequence(this.cst);

    return {
      openBracket: $openBracket as TokenNode,
      start: $start === null ? undefined : new Expression($start as RuleNode),
      end: $end === null ? undefined : new IndexAccessEnd($end as RuleNode),
      closeBracket: $closeBracket as TokenNode,
      operand: new Expression($operand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.IndexAccessExpression);
  }

  public get openBracket(): TokenNode {
    return this.fetch().openBracket;
  }

  public get start(): Expression | undefined {
    return this.fetch().start;
  }

  public get end(): IndexAccessEnd | undefined {
    return this.fetch().end;
  }

  public get closeBracket(): TokenNode {
    return this.fetch().closeBracket;
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }
}

export class IndexAccessEnd {
  private readonly fetch = once(() => {
    const [$colon, $end] = ast_internal.selectSequence(this.cst);

    return {
      colon: $colon as TokenNode,
      end: $end === null ? undefined : new Expression($end as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.IndexAccessEnd);
  }

  public get colon(): TokenNode {
    return this.fetch().colon;
  }

  public get end(): Expression | undefined {
    return this.fetch().end;
  }
}

export class PositionalArgumentsDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $arguments, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      arguments: $arguments === null ? undefined : new PositionalArguments($arguments as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.PositionalArgumentsDeclaration);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get arguments(): PositionalArguments | undefined {
    return this.fetch().arguments;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class NamedArgumentsDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $arguments, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      arguments: $arguments === null ? undefined : new NamedArgumentGroup($arguments as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.NamedArgumentsDeclaration);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get arguments(): NamedArgumentGroup | undefined {
    return this.fetch().arguments;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class NamedArgumentGroup {
  private readonly fetch = once(() => {
    const [$openBrace, $arguments, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      openBrace: $openBrace as TokenNode,
      arguments: $arguments === null ? undefined : new NamedArguments($arguments as RuleNode),
      closeBrace: $closeBrace as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.NamedArgumentGroup);
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get arguments(): NamedArguments | undefined {
    return this.fetch().arguments;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }
}

export class NamedArgument {
  private readonly fetch = once(() => {
    const [$name, $colon, $value] = ast_internal.selectSequence(this.cst);

    return {
      name: $name as TokenNode,
      colon: $colon as TokenNode,
      value: new Expression($value as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.NamedArgument);
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get colon(): TokenNode {
    return this.fetch().colon;
  }

  public get value(): Expression {
    return this.fetch().value;
  }
}

export class TypeExpression {
  private readonly fetch = once(() => {
    const [$typeKeyword, $openParen, $typeName, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      typeKeyword: $typeKeyword as TokenNode,
      openParen: $openParen as TokenNode,
      typeName: new TypeName($typeName as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TypeExpression);
  }

  public get typeKeyword(): TokenNode {
    return this.fetch().typeKeyword;
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class NewExpression {
  private readonly fetch = once(() => {
    const [$newKeyword, $typeName] = ast_internal.selectSequence(this.cst);

    return {
      newKeyword: $newKeyword as TokenNode,
      typeName: new TypeName($typeName as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.NewExpression);
  }

  public get newKeyword(): TokenNode {
    return this.fetch().newKeyword;
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }
}

export class TupleExpression {
  private readonly fetch = once(() => {
    const [$openParen, $items, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      items: new TupleValues($items as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TupleExpression);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get items(): TupleValues {
    return this.fetch().items;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class TupleValue {
  private readonly fetch = once(() => {
    const [$expression] = ast_internal.selectSequence(this.cst);

    return {
      expression: $expression === null ? undefined : new Expression($expression as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TupleValue);
  }

  public get expression(): Expression | undefined {
    return this.fetch().expression;
  }
}

export class ArrayExpression {
  private readonly fetch = once(() => {
    const [$openBracket, $items, $closeBracket] = ast_internal.selectSequence(this.cst);

    return {
      openBracket: $openBracket as TokenNode,
      items: new ArrayValues($items as RuleNode),
      closeBracket: $closeBracket as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ArrayExpression);
  }

  public get openBracket(): TokenNode {
    return this.fetch().openBracket;
  }

  public get items(): ArrayValues {
    return this.fetch().items;
  }

  public get closeBracket(): TokenNode {
    return this.fetch().closeBracket;
  }
}

export class HexNumberExpression {
  private readonly fetch = once(() => {
    const [$literal, $unit] = ast_internal.selectSequence(this.cst);

    return {
      literal: $literal as TokenNode,
      unit: $unit === null ? undefined : new NumberUnit($unit as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.HexNumberExpression);
  }

  public get literal(): TokenNode {
    return this.fetch().literal;
  }

  public get unit(): NumberUnit | undefined {
    return this.fetch().unit;
  }
}

export class DecimalNumberExpression {
  private readonly fetch = once(() => {
    const [$literal, $unit] = ast_internal.selectSequence(this.cst);

    return {
      literal: $literal as TokenNode,
      unit: $unit === null ? undefined : new NumberUnit($unit as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.DecimalNumberExpression);
  }

  public get literal(): TokenNode {
    return this.fetch().literal;
  }

  public get unit(): NumberUnit | undefined {
    return this.fetch().unit;
  }
}

export class YulBlock {
  private readonly fetch = once(() => {
    const [$openBrace, $statements, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      openBrace: $openBrace as TokenNode,
      statements: $statements === null ? undefined : new YulStatements($statements as RuleNode),
      closeBrace: $closeBrace as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulBlock);
  }

  public get openBrace(): TokenNode {
    return this.fetch().openBrace;
  }

  public get statements(): YulStatements | undefined {
    return this.fetch().statements;
  }

  public get closeBrace(): TokenNode {
    return this.fetch().closeBrace;
  }
}

export class YulFunctionDefinition {
  private readonly fetch = once(() => {
    const [$functionKeyword, $name, $parameters, $returns, $body] = ast_internal.selectSequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TokenNode,
      name: $name as TokenNode,
      parameters: new YulParametersDeclaration($parameters as RuleNode),
      returns: $returns === null ? undefined : new YulReturnsDeclaration($returns as RuleNode),
      body: new YulBlock($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulFunctionDefinition);
  }

  public get functionKeyword(): TokenNode {
    return this.fetch().functionKeyword;
  }

  public get name(): TokenNode {
    return this.fetch().name;
  }

  public get parameters(): YulParametersDeclaration {
    return this.fetch().parameters;
  }

  public get returns(): YulReturnsDeclaration | undefined {
    return this.fetch().returns;
  }

  public get body(): YulBlock {
    return this.fetch().body;
  }
}

export class YulParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      parameters: $parameters === null ? undefined : new YulParameters($parameters as RuleNode),
      closeParen: $closeParen as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulParametersDeclaration);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get parameters(): YulParameters | undefined {
    return this.fetch().parameters;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }
}

export class YulReturnsDeclaration {
  private readonly fetch = once(() => {
    const [$minusGreaterThan, $variables] = ast_internal.selectSequence(this.cst);

    return {
      minusGreaterThan: $minusGreaterThan as TokenNode,
      variables: new YulReturnVariables($variables as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulReturnsDeclaration);
  }

  public get minusGreaterThan(): TokenNode {
    return this.fetch().minusGreaterThan;
  }

  public get variables(): YulReturnVariables {
    return this.fetch().variables;
  }
}

export class YulVariableDeclarationStatement {
  private readonly fetch = once(() => {
    const [$letKeyword, $names, $value] = ast_internal.selectSequence(this.cst);

    return {
      letKeyword: $letKeyword as TokenNode,
      names: new YulIdentifierPaths($names as RuleNode),
      value: $value === null ? undefined : new YulVariableDeclarationValue($value as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulVariableDeclarationStatement);
  }

  public get letKeyword(): TokenNode {
    return this.fetch().letKeyword;
  }

  public get names(): YulIdentifierPaths {
    return this.fetch().names;
  }

  public get value(): YulVariableDeclarationValue | undefined {
    return this.fetch().value;
  }
}

export class YulVariableDeclarationValue {
  private readonly fetch = once(() => {
    const [$colonEqual, $expression] = ast_internal.selectSequence(this.cst);

    return {
      colonEqual: $colonEqual as TokenNode,
      expression: new YulExpression($expression as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulVariableDeclarationValue);
  }

  public get colonEqual(): TokenNode {
    return this.fetch().colonEqual;
  }

  public get expression(): YulExpression {
    return this.fetch().expression;
  }
}

export class YulAssignmentStatement {
  private readonly fetch = once(() => {
    const [$names, $colonEqual, $expression] = ast_internal.selectSequence(this.cst);

    return {
      names: new YulIdentifierPaths($names as RuleNode),
      colonEqual: $colonEqual as TokenNode,
      expression: new YulExpression($expression as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulAssignmentStatement);
  }

  public get names(): YulIdentifierPaths {
    return this.fetch().names;
  }

  public get colonEqual(): TokenNode {
    return this.fetch().colonEqual;
  }

  public get expression(): YulExpression {
    return this.fetch().expression;
  }
}

export class YulIfStatement {
  private readonly fetch = once(() => {
    const [$ifKeyword, $condition, $body] = ast_internal.selectSequence(this.cst);

    return {
      ifKeyword: $ifKeyword as TokenNode,
      condition: new YulExpression($condition as RuleNode),
      body: new YulBlock($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulIfStatement);
  }

  public get ifKeyword(): TokenNode {
    return this.fetch().ifKeyword;
  }

  public get condition(): YulExpression {
    return this.fetch().condition;
  }

  public get body(): YulBlock {
    return this.fetch().body;
  }
}

export class YulForStatement {
  private readonly fetch = once(() => {
    const [$forKeyword, $initialization, $condition, $iterator, $body] = ast_internal.selectSequence(this.cst);

    return {
      forKeyword: $forKeyword as TokenNode,
      initialization: new YulBlock($initialization as RuleNode),
      condition: new YulExpression($condition as RuleNode),
      iterator: new YulBlock($iterator as RuleNode),
      body: new YulBlock($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulForStatement);
  }

  public get forKeyword(): TokenNode {
    return this.fetch().forKeyword;
  }

  public get initialization(): YulBlock {
    return this.fetch().initialization;
  }

  public get condition(): YulExpression {
    return this.fetch().condition;
  }

  public get iterator(): YulBlock {
    return this.fetch().iterator;
  }

  public get body(): YulBlock {
    return this.fetch().body;
  }
}

export class YulSwitchStatement {
  private readonly fetch = once(() => {
    const [$switchKeyword, $expression, $cases] = ast_internal.selectSequence(this.cst);

    return {
      switchKeyword: $switchKeyword as TokenNode,
      expression: new YulExpression($expression as RuleNode),
      cases: new YulSwitchCases($cases as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulSwitchStatement);
  }

  public get switchKeyword(): TokenNode {
    return this.fetch().switchKeyword;
  }

  public get expression(): YulExpression {
    return this.fetch().expression;
  }

  public get cases(): YulSwitchCases {
    return this.fetch().cases;
  }
}

export class YulDefaultCase {
  private readonly fetch = once(() => {
    const [$defaultKeyword, $body] = ast_internal.selectSequence(this.cst);

    return {
      defaultKeyword: $defaultKeyword as TokenNode,
      body: new YulBlock($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulDefaultCase);
  }

  public get defaultKeyword(): TokenNode {
    return this.fetch().defaultKeyword;
  }

  public get body(): YulBlock {
    return this.fetch().body;
  }
}

export class YulValueCase {
  private readonly fetch = once(() => {
    const [$caseKeyword, $value, $body] = ast_internal.selectSequence(this.cst);

    return {
      caseKeyword: $caseKeyword as TokenNode,
      value: new YulLiteral($value as RuleNode),
      body: new YulBlock($body as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulValueCase);
  }

  public get caseKeyword(): TokenNode {
    return this.fetch().caseKeyword;
  }

  public get value(): YulLiteral {
    return this.fetch().value;
  }

  public get body(): YulBlock {
    return this.fetch().body;
  }
}

export class YulLeaveStatement {
  private readonly fetch = once(() => {
    const [$leaveKeyword] = ast_internal.selectSequence(this.cst);

    return {
      leaveKeyword: $leaveKeyword as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulLeaveStatement);
  }

  public get leaveKeyword(): TokenNode {
    return this.fetch().leaveKeyword;
  }
}

export class YulBreakStatement {
  private readonly fetch = once(() => {
    const [$breakKeyword] = ast_internal.selectSequence(this.cst);

    return {
      breakKeyword: $breakKeyword as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulBreakStatement);
  }

  public get breakKeyword(): TokenNode {
    return this.fetch().breakKeyword;
  }
}

export class YulContinueStatement {
  private readonly fetch = once(() => {
    const [$continueKeyword] = ast_internal.selectSequence(this.cst);

    return {
      continueKeyword: $continueKeyword as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulContinueStatement);
  }

  public get continueKeyword(): TokenNode {
    return this.fetch().continueKeyword;
  }
}

export class YulLabel {
  private readonly fetch = once(() => {
    const [$label, $colon] = ast_internal.selectSequence(this.cst);

    return {
      label: $label as TokenNode,
      colon: $colon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulLabel);
  }

  public get label(): TokenNode {
    return this.fetch().label;
  }

  public get colon(): TokenNode {
    return this.fetch().colon;
  }
}

export class YulFunctionCallExpression {
  private readonly fetch = once(() => {
    const [$openParen, $arguments, $closeParen, $operand] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TokenNode,
      arguments: $arguments === null ? undefined : new YulArguments($arguments as RuleNode),
      closeParen: $closeParen as TokenNode,
      operand: new YulExpression($operand as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulFunctionCallExpression);
  }

  public get openParen(): TokenNode {
    return this.fetch().openParen;
  }

  public get arguments(): YulArguments | undefined {
    return this.fetch().arguments;
  }

  public get closeParen(): TokenNode {
    return this.fetch().closeParen;
  }

  public get operand(): YulExpression {
    return this.fetch().operand;
  }
}

/*
 * Choices:
 */

export class SourceUnitMember {
  private readonly fetch: () =>
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
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.PragmaDirective:
        return new PragmaDirective(variant as RuleNode);
      case RuleKind.ImportDirective:
        return new ImportDirective(variant as RuleNode);
      case RuleKind.ContractDefinition:
        return new ContractDefinition(variant as RuleNode);
      case RuleKind.InterfaceDefinition:
        return new InterfaceDefinition(variant as RuleNode);
      case RuleKind.LibraryDefinition:
        return new LibraryDefinition(variant as RuleNode);
      case RuleKind.StructDefinition:
        return new StructDefinition(variant as RuleNode);
      case RuleKind.EnumDefinition:
        return new EnumDefinition(variant as RuleNode);
      case RuleKind.FunctionDefinition:
        return new FunctionDefinition(variant as RuleNode);
      case RuleKind.ConstantDefinition:
        return new ConstantDefinition(variant as RuleNode);
      case RuleKind.ErrorDefinition:
        return new ErrorDefinition(variant as RuleNode);
      case RuleKind.UserDefinedValueTypeDefinition:
        return new UserDefinedValueTypeDefinition(variant as RuleNode);
      case RuleKind.UsingDirective:
        return new UsingDirective(variant as RuleNode);
      case RuleKind.EventDefinition:
        return new EventDefinition(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.SourceUnitMember);
  }

  public get variant():
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
    | EventDefinition {
    return this.fetch();
  }
}

export class Pragma {
  private readonly fetch: () => ABICoderPragma | ExperimentalPragma | VersionPragma = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ABICoderPragma:
        return new ABICoderPragma(variant as RuleNode);
      case RuleKind.ExperimentalPragma:
        return new ExperimentalPragma(variant as RuleNode);
      case RuleKind.VersionPragma:
        return new VersionPragma(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.Pragma);
  }

  public get variant(): ABICoderPragma | ExperimentalPragma | VersionPragma {
    return this.fetch();
  }
}

export class ExperimentalFeature {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TokenKind.Identifier:
      case TokenKind.AsciiStringLiteral:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ExperimentalFeature);
  }

  public get variant(): TokenNode {
    return this.fetch();
  }
}

export class VersionPragmaExpression {
  private readonly fetch: () =>
    | VersionPragmaOrExpression
    | VersionPragmaRangeExpression
    | VersionPragmaPrefixExpression
    | VersionPragmaSpecifier = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.VersionPragmaOrExpression:
        return new VersionPragmaOrExpression(variant as RuleNode);
      case RuleKind.VersionPragmaRangeExpression:
        return new VersionPragmaRangeExpression(variant as RuleNode);
      case RuleKind.VersionPragmaPrefixExpression:
        return new VersionPragmaPrefixExpression(variant as RuleNode);
      case RuleKind.VersionPragmaSpecifier:
        return new VersionPragmaSpecifier(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VersionPragmaExpression);
  }

  public get variant():
    | VersionPragmaOrExpression
    | VersionPragmaRangeExpression
    | VersionPragmaPrefixExpression
    | VersionPragmaSpecifier {
    return this.fetch();
  }
}

export class ImportClause {
  private readonly fetch: () => PathImport | NamedImport | ImportDeconstruction = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.PathImport:
        return new PathImport(variant as RuleNode);
      case RuleKind.NamedImport:
        return new NamedImport(variant as RuleNode);
      case RuleKind.ImportDeconstruction:
        return new ImportDeconstruction(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ImportClause);
  }

  public get variant(): PathImport | NamedImport | ImportDeconstruction {
    return this.fetch();
  }
}

export class UsingClause {
  private readonly fetch: () => IdentifierPath | UsingDeconstruction = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.IdentifierPath:
        return new IdentifierPath(variant as RuleNode);
      case RuleKind.UsingDeconstruction:
        return new UsingDeconstruction(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UsingClause);
  }

  public get variant(): IdentifierPath | UsingDeconstruction {
    return this.fetch();
  }
}

export class UsingOperator {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
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
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UsingOperator);
  }

  public get variant(): TokenNode {
    return this.fetch();
  }
}

export class UsingTarget {
  private readonly fetch: () => TypeName | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.TypeName:
        return new TypeName(variant as RuleNode);

      case TokenKind.Asterisk:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UsingTarget);
  }

  public get variant(): TypeName | TokenNode {
    return this.fetch();
  }
}

export class ContractMember {
  private readonly fetch: () =>
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
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.UsingDirective:
        return new UsingDirective(variant as RuleNode);
      case RuleKind.FunctionDefinition:
        return new FunctionDefinition(variant as RuleNode);
      case RuleKind.ConstructorDefinition:
        return new ConstructorDefinition(variant as RuleNode);
      case RuleKind.ReceiveFunctionDefinition:
        return new ReceiveFunctionDefinition(variant as RuleNode);
      case RuleKind.FallbackFunctionDefinition:
        return new FallbackFunctionDefinition(variant as RuleNode);
      case RuleKind.UnnamedFunctionDefinition:
        return new UnnamedFunctionDefinition(variant as RuleNode);
      case RuleKind.ModifierDefinition:
        return new ModifierDefinition(variant as RuleNode);
      case RuleKind.StructDefinition:
        return new StructDefinition(variant as RuleNode);
      case RuleKind.EnumDefinition:
        return new EnumDefinition(variant as RuleNode);
      case RuleKind.EventDefinition:
        return new EventDefinition(variant as RuleNode);
      case RuleKind.StateVariableDefinition:
        return new StateVariableDefinition(variant as RuleNode);
      case RuleKind.ErrorDefinition:
        return new ErrorDefinition(variant as RuleNode);
      case RuleKind.UserDefinedValueTypeDefinition:
        return new UserDefinedValueTypeDefinition(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ContractMember);
  }

  public get variant():
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
    | UserDefinedValueTypeDefinition {
    return this.fetch();
  }
}

export class StateVariableAttribute {
  private readonly fetch: () => OverrideSpecifier | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as RuleNode);

      case TokenKind.ConstantKeyword:
      case TokenKind.InternalKeyword:
      case TokenKind.PrivateKeyword:
      case TokenKind.PublicKeyword:
      case TokenKind.ImmutableKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.StateVariableAttribute);
  }

  public get variant(): OverrideSpecifier | TokenNode {
    return this.fetch();
  }
}

export class FunctionName {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TokenKind.Identifier:
      case TokenKind.FallbackKeyword:
      case TokenKind.ReceiveKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionName);
  }

  public get variant(): TokenNode {
    return this.fetch();
  }
}

export class FunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(variant as RuleNode);
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as RuleNode);

      case TokenKind.ConstantKeyword:
      case TokenKind.ExternalKeyword:
      case TokenKind.InternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.PrivateKeyword:
      case TokenKind.PublicKeyword:
      case TokenKind.PureKeyword:
      case TokenKind.ViewKeyword:
      case TokenKind.VirtualKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TokenNode {
    return this.fetch();
  }
}

export class FunctionBody {
  private readonly fetch: () => Block | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.Block:
        return new Block(variant as RuleNode);

      case TokenKind.Semicolon:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionBody);
  }

  public get variant(): Block | TokenNode {
    return this.fetch();
  }
}

export class ConstructorAttribute {
  private readonly fetch: () => ModifierInvocation | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(variant as RuleNode);

      case TokenKind.InternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.PublicKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ConstructorAttribute);
  }

  public get variant(): ModifierInvocation | TokenNode {
    return this.fetch();
  }
}

export class UnnamedFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(variant as RuleNode);
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as RuleNode);

      case TokenKind.ExternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.PureKeyword:
      case TokenKind.ViewKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UnnamedFunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TokenNode {
    return this.fetch();
  }
}

export class FallbackFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(variant as RuleNode);
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as RuleNode);

      case TokenKind.ExternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.PureKeyword:
      case TokenKind.ViewKeyword:
      case TokenKind.VirtualKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FallbackFunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TokenNode {
    return this.fetch();
  }
}

export class ReceiveFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ModifierInvocation:
        return new ModifierInvocation(variant as RuleNode);
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as RuleNode);

      case TokenKind.ExternalKeyword:
      case TokenKind.PayableKeyword:
      case TokenKind.VirtualKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ReceiveFunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TokenNode {
    return this.fetch();
  }
}

export class ModifierAttribute {
  private readonly fetch: () => OverrideSpecifier | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as RuleNode);

      case TokenKind.VirtualKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ModifierAttribute);
  }

  public get variant(): OverrideSpecifier | TokenNode {
    return this.fetch();
  }
}

export class TypeName {
  private readonly fetch: () => ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath = once(
    () => {
      const variant = ast_internal.selectChoice(this.cst);

      switch (variant.kind) {
        case RuleKind.ArrayTypeName:
          return new ArrayTypeName(variant as RuleNode);
        case RuleKind.FunctionType:
          return new FunctionType(variant as RuleNode);
        case RuleKind.MappingType:
          return new MappingType(variant as RuleNode);
        case RuleKind.ElementaryType:
          return new ElementaryType(variant as RuleNode);
        case RuleKind.IdentifierPath:
          return new IdentifierPath(variant as RuleNode);

        default:
          assert.fail(`Unexpected variant: ${variant.kind}`);
      }
    },
  );

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TypeName);
  }

  public get variant(): ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath {
    return this.fetch();
  }
}

export class FunctionTypeAttribute {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TokenKind.InternalKeyword:
      case TokenKind.ExternalKeyword:
      case TokenKind.PrivateKeyword:
      case TokenKind.PublicKeyword:
      case TokenKind.PureKeyword:
      case TokenKind.ViewKeyword:
      case TokenKind.PayableKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionTypeAttribute);
  }

  public get variant(): TokenNode {
    return this.fetch();
  }
}

export class MappingKeyType {
  private readonly fetch: () => ElementaryType | IdentifierPath = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ElementaryType:
        return new ElementaryType(variant as RuleNode);
      case RuleKind.IdentifierPath:
        return new IdentifierPath(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.MappingKeyType);
  }

  public get variant(): ElementaryType | IdentifierPath {
    return this.fetch();
  }
}

export class ElementaryType {
  private readonly fetch: () => AddressType | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.AddressType:
        return new AddressType(variant as RuleNode);

      case TokenKind.BoolKeyword:
      case TokenKind.ByteKeyword:
      case TokenKind.StringKeyword:
      case TokenKind.BytesKeyword:
      case TokenKind.IntKeyword:
      case TokenKind.UintKeyword:
      case TokenKind.FixedKeyword:
      case TokenKind.UfixedKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ElementaryType);
  }

  public get variant(): AddressType | TokenNode {
    return this.fetch();
  }
}

export class Statement {
  private readonly fetch: () =>
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
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ExpressionStatement:
        return new ExpressionStatement(variant as RuleNode);
      case RuleKind.VariableDeclarationStatement:
        return new VariableDeclarationStatement(variant as RuleNode);
      case RuleKind.TupleDeconstructionStatement:
        return new TupleDeconstructionStatement(variant as RuleNode);
      case RuleKind.IfStatement:
        return new IfStatement(variant as RuleNode);
      case RuleKind.ForStatement:
        return new ForStatement(variant as RuleNode);
      case RuleKind.WhileStatement:
        return new WhileStatement(variant as RuleNode);
      case RuleKind.DoWhileStatement:
        return new DoWhileStatement(variant as RuleNode);
      case RuleKind.ContinueStatement:
        return new ContinueStatement(variant as RuleNode);
      case RuleKind.BreakStatement:
        return new BreakStatement(variant as RuleNode);
      case RuleKind.DeleteStatement:
        return new DeleteStatement(variant as RuleNode);
      case RuleKind.ReturnStatement:
        return new ReturnStatement(variant as RuleNode);
      case RuleKind.ThrowStatement:
        return new ThrowStatement(variant as RuleNode);
      case RuleKind.EmitStatement:
        return new EmitStatement(variant as RuleNode);
      case RuleKind.TryStatement:
        return new TryStatement(variant as RuleNode);
      case RuleKind.RevertStatement:
        return new RevertStatement(variant as RuleNode);
      case RuleKind.AssemblyStatement:
        return new AssemblyStatement(variant as RuleNode);
      case RuleKind.Block:
        return new Block(variant as RuleNode);
      case RuleKind.UncheckedBlock:
        return new UncheckedBlock(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.Statement);
  }

  public get variant():
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
    | UncheckedBlock {
    return this.fetch();
  }
}

export class TupleMember {
  private readonly fetch: () => TypedTupleMember | UntypedTupleMember = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.TypedTupleMember:
        return new TypedTupleMember(variant as RuleNode);
      case RuleKind.UntypedTupleMember:
        return new UntypedTupleMember(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TupleMember);
  }

  public get variant(): TypedTupleMember | UntypedTupleMember {
    return this.fetch();
  }
}

export class VariableDeclarationType {
  private readonly fetch: () => TypeName | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.TypeName:
        return new TypeName(variant as RuleNode);

      case TokenKind.VarKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VariableDeclarationType);
  }

  public get variant(): TypeName | TokenNode {
    return this.fetch();
  }
}

export class StorageLocation {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TokenKind.MemoryKeyword:
      case TokenKind.StorageKeyword:
      case TokenKind.CallDataKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.StorageLocation);
  }

  public get variant(): TokenNode {
    return this.fetch();
  }
}

export class ForStatementInitialization {
  private readonly fetch: () =>
    | ExpressionStatement
    | VariableDeclarationStatement
    | TupleDeconstructionStatement
    | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ExpressionStatement:
        return new ExpressionStatement(variant as RuleNode);
      case RuleKind.VariableDeclarationStatement:
        return new VariableDeclarationStatement(variant as RuleNode);
      case RuleKind.TupleDeconstructionStatement:
        return new TupleDeconstructionStatement(variant as RuleNode);

      case TokenKind.Semicolon:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ForStatementInitialization);
  }

  public get variant(): ExpressionStatement | VariableDeclarationStatement | TupleDeconstructionStatement | TokenNode {
    return this.fetch();
  }
}

export class ForStatementCondition {
  private readonly fetch: () => ExpressionStatement | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.ExpressionStatement:
        return new ExpressionStatement(variant as RuleNode);

      case TokenKind.Semicolon:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ForStatementCondition);
  }

  public get variant(): ExpressionStatement | TokenNode {
    return this.fetch();
  }
}

export class Expression {
  private readonly fetch: () =>
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
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.AssignmentExpression:
        return new AssignmentExpression(variant as RuleNode);
      case RuleKind.ConditionalExpression:
        return new ConditionalExpression(variant as RuleNode);
      case RuleKind.OrExpression:
        return new OrExpression(variant as RuleNode);
      case RuleKind.AndExpression:
        return new AndExpression(variant as RuleNode);
      case RuleKind.EqualityExpression:
        return new EqualityExpression(variant as RuleNode);
      case RuleKind.ComparisonExpression:
        return new ComparisonExpression(variant as RuleNode);
      case RuleKind.BitwiseOrExpression:
        return new BitwiseOrExpression(variant as RuleNode);
      case RuleKind.BitwiseXorExpression:
        return new BitwiseXorExpression(variant as RuleNode);
      case RuleKind.BitwiseAndExpression:
        return new BitwiseAndExpression(variant as RuleNode);
      case RuleKind.ShiftExpression:
        return new ShiftExpression(variant as RuleNode);
      case RuleKind.AdditiveExpression:
        return new AdditiveExpression(variant as RuleNode);
      case RuleKind.MultiplicativeExpression:
        return new MultiplicativeExpression(variant as RuleNode);
      case RuleKind.ExponentiationExpression:
        return new ExponentiationExpression(variant as RuleNode);
      case RuleKind.PostfixExpression:
        return new PostfixExpression(variant as RuleNode);
      case RuleKind.PrefixExpression:
        return new PrefixExpression(variant as RuleNode);
      case RuleKind.FunctionCallExpression:
        return new FunctionCallExpression(variant as RuleNode);
      case RuleKind.MemberAccessExpression:
        return new MemberAccessExpression(variant as RuleNode);
      case RuleKind.IndexAccessExpression:
        return new IndexAccessExpression(variant as RuleNode);
      case RuleKind.NewExpression:
        return new NewExpression(variant as RuleNode);
      case RuleKind.TupleExpression:
        return new TupleExpression(variant as RuleNode);
      case RuleKind.TypeExpression:
        return new TypeExpression(variant as RuleNode);
      case RuleKind.ArrayExpression:
        return new ArrayExpression(variant as RuleNode);
      case RuleKind.HexNumberExpression:
        return new HexNumberExpression(variant as RuleNode);
      case RuleKind.DecimalNumberExpression:
        return new DecimalNumberExpression(variant as RuleNode);
      case RuleKind.StringExpression:
        return new StringExpression(variant as RuleNode);
      case RuleKind.ElementaryType:
        return new ElementaryType(variant as RuleNode);

      case TokenKind.PayableKeyword:
      case TokenKind.TrueKeyword:
      case TokenKind.FalseKeyword:
      case TokenKind.Identifier:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.Expression);
  }

  public get variant():
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
    | TokenNode {
    return this.fetch();
  }
}

export class MemberAccess {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TokenKind.Identifier:
      case TokenKind.AddressKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.MemberAccess);
  }

  public get variant(): TokenNode {
    return this.fetch();
  }
}

export class FunctionCallOptions {
  private readonly fetch: () => NamedArgumentGroups | NamedArgumentGroup = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.NamedArgumentGroups:
        return new NamedArgumentGroups(variant as RuleNode);
      case RuleKind.NamedArgumentGroup:
        return new NamedArgumentGroup(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionCallOptions);
  }

  public get variant(): NamedArgumentGroups | NamedArgumentGroup {
    return this.fetch();
  }
}

export class ArgumentsDeclaration {
  private readonly fetch: () => PositionalArgumentsDeclaration | NamedArgumentsDeclaration = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.PositionalArgumentsDeclaration:
        return new PositionalArgumentsDeclaration(variant as RuleNode);
      case RuleKind.NamedArgumentsDeclaration:
        return new NamedArgumentsDeclaration(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ArgumentsDeclaration);
  }

  public get variant(): PositionalArgumentsDeclaration | NamedArgumentsDeclaration {
    return this.fetch();
  }
}

export class NumberUnit {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
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
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.NumberUnit);
  }

  public get variant(): TokenNode {
    return this.fetch();
  }
}

export class StringExpression {
  private readonly fetch: () => HexStringLiterals | AsciiStringLiterals | UnicodeStringLiterals = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.HexStringLiterals:
        return new HexStringLiterals(variant as RuleNode);
      case RuleKind.AsciiStringLiterals:
        return new AsciiStringLiterals(variant as RuleNode);
      case RuleKind.UnicodeStringLiterals:
        return new UnicodeStringLiterals(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.StringExpression);
  }

  public get variant(): HexStringLiterals | AsciiStringLiterals | UnicodeStringLiterals {
    return this.fetch();
  }
}

export class YulStatement {
  private readonly fetch: () =>
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
    | YulLabel
    | YulExpression = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.YulBlock:
        return new YulBlock(variant as RuleNode);
      case RuleKind.YulFunctionDefinition:
        return new YulFunctionDefinition(variant as RuleNode);
      case RuleKind.YulVariableDeclarationStatement:
        return new YulVariableDeclarationStatement(variant as RuleNode);
      case RuleKind.YulAssignmentStatement:
        return new YulAssignmentStatement(variant as RuleNode);
      case RuleKind.YulIfStatement:
        return new YulIfStatement(variant as RuleNode);
      case RuleKind.YulForStatement:
        return new YulForStatement(variant as RuleNode);
      case RuleKind.YulSwitchStatement:
        return new YulSwitchStatement(variant as RuleNode);
      case RuleKind.YulLeaveStatement:
        return new YulLeaveStatement(variant as RuleNode);
      case RuleKind.YulBreakStatement:
        return new YulBreakStatement(variant as RuleNode);
      case RuleKind.YulContinueStatement:
        return new YulContinueStatement(variant as RuleNode);
      case RuleKind.YulLabel:
        return new YulLabel(variant as RuleNode);
      case RuleKind.YulExpression:
        return new YulExpression(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulStatement);
  }

  public get variant():
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
    | YulLabel
    | YulExpression {
    return this.fetch();
  }
}

export class YulSwitchCase {
  private readonly fetch: () => YulDefaultCase | YulValueCase = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.YulDefaultCase:
        return new YulDefaultCase(variant as RuleNode);
      case RuleKind.YulValueCase:
        return new YulValueCase(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulSwitchCase);
  }

  public get variant(): YulDefaultCase | YulValueCase {
    return this.fetch();
  }
}

export class YulExpression {
  private readonly fetch: () => YulFunctionCallExpression | YulLiteral | YulBuiltInFunction | YulIdentifierPath = once(
    () => {
      const variant = ast_internal.selectChoice(this.cst);

      switch (variant.kind) {
        case RuleKind.YulFunctionCallExpression:
          return new YulFunctionCallExpression(variant as RuleNode);
        case RuleKind.YulLiteral:
          return new YulLiteral(variant as RuleNode);
        case RuleKind.YulBuiltInFunction:
          return new YulBuiltInFunction(variant as RuleNode);
        case RuleKind.YulIdentifierPath:
          return new YulIdentifierPath(variant as RuleNode);

        default:
          assert.fail(`Unexpected variant: ${variant.kind}`);
      }
    },
  );

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulExpression);
  }

  public get variant(): YulFunctionCallExpression | YulLiteral | YulBuiltInFunction | YulIdentifierPath {
    return this.fetch();
  }
}

export class YulBuiltInFunction {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TokenKind.YulAddKeyword:
      case TokenKind.YulAddModKeyword:
      case TokenKind.YulAddressKeyword:
      case TokenKind.YulAndKeyword:
      case TokenKind.YulBalanceKeyword:
      case TokenKind.YulBlockHashKeyword:
      case TokenKind.YulByteKeyword:
      case TokenKind.YulCallCodeKeyword:
      case TokenKind.YulCallDataCopyKeyword:
      case TokenKind.YulCallDataLoadKeyword:
      case TokenKind.YulCallDataSizeKeyword:
      case TokenKind.YulCallerKeyword:
      case TokenKind.YulCallKeyword:
      case TokenKind.YulCallValueKeyword:
      case TokenKind.YulCoinBaseKeyword:
      case TokenKind.YulCreateKeyword:
      case TokenKind.YulDelegateCallKeyword:
      case TokenKind.YulDivKeyword:
      case TokenKind.YulEqKeyword:
      case TokenKind.YulExpKeyword:
      case TokenKind.YulExtCodeCopyKeyword:
      case TokenKind.YulExtCodeSizeKeyword:
      case TokenKind.YulGasKeyword:
      case TokenKind.YulGasLimitKeyword:
      case TokenKind.YulGasPriceKeyword:
      case TokenKind.YulGtKeyword:
      case TokenKind.YulInvalidKeyword:
      case TokenKind.YulIsZeroKeyword:
      case TokenKind.YulLog0Keyword:
      case TokenKind.YulLog1Keyword:
      case TokenKind.YulLog2Keyword:
      case TokenKind.YulLog3Keyword:
      case TokenKind.YulLog4Keyword:
      case TokenKind.YulLtKeyword:
      case TokenKind.YulMLoadKeyword:
      case TokenKind.YulModKeyword:
      case TokenKind.YulMSizeKeyword:
      case TokenKind.YulMStore8Keyword:
      case TokenKind.YulMStoreKeyword:
      case TokenKind.YulMulKeyword:
      case TokenKind.YulMulModKeyword:
      case TokenKind.YulNotKeyword:
      case TokenKind.YulNumberKeyword:
      case TokenKind.YulOriginKeyword:
      case TokenKind.YulOrKeyword:
      case TokenKind.YulPopKeyword:
      case TokenKind.YulReturnKeyword:
      case TokenKind.YulRevertKeyword:
      case TokenKind.YulSDivKeyword:
      case TokenKind.YulSelfDestructKeyword:
      case TokenKind.YulSgtKeyword:
      case TokenKind.YulSignExtendKeyword:
      case TokenKind.YulSLoadKeyword:
      case TokenKind.YulSltKeyword:
      case TokenKind.YulSModKeyword:
      case TokenKind.YulSStoreKeyword:
      case TokenKind.YulStopKeyword:
      case TokenKind.YulSubKeyword:
      case TokenKind.YulTimestampKeyword:
      case TokenKind.YulXorKeyword:
      case TokenKind.YulKeccak256Keyword:
      case TokenKind.YulSha3Keyword:
      case TokenKind.YulSuicideKeyword:
      case TokenKind.YulReturnDataCopyKeyword:
      case TokenKind.YulReturnDataSizeKeyword:
      case TokenKind.YulStaticCallKeyword:
      case TokenKind.YulCreate2Keyword:
      case TokenKind.YulExtCodeHashKeyword:
      case TokenKind.YulSarKeyword:
      case TokenKind.YulShlKeyword:
      case TokenKind.YulShrKeyword:
      case TokenKind.YulChainIdKeyword:
      case TokenKind.YulSelfBalanceKeyword:
      case TokenKind.YulBaseFeeKeyword:
      case TokenKind.YulDifficultyKeyword:
      case TokenKind.YulPrevRandaoKeyword:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulBuiltInFunction);
  }

  public get variant(): TokenNode {
    return this.fetch();
  }
}

export class YulLiteral {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TokenKind.YulTrueKeyword:
      case TokenKind.YulFalseKeyword:
      case TokenKind.YulDecimalLiteral:
      case TokenKind.YulHexLiteral:
      case TokenKind.HexStringLiteral:
      case TokenKind.AsciiStringLiteral:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulLiteral);
  }

  public get variant(): TokenNode {
    return this.fetch();
  }
}

/*
 * Repeated:
 */

export class SourceUnitMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new SourceUnitMember(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.SourceUnitMembers);
  }

  public get items(): readonly SourceUnitMember[] {
    return this.fetch();
  }
}

export class VersionPragmaExpressions {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new VersionPragmaExpression(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VersionPragmaExpressions);
  }

  public get items(): readonly VersionPragmaExpression[] {
    return this.fetch();
  }
}

export class ContractMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ContractMember(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ContractMembers);
  }

  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

export class InterfaceMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ContractMember(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.InterfaceMembers);
  }

  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

export class LibraryMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ContractMember(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.LibraryMembers);
  }

  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

export class StructMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new StructMember(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.StructMembers);
  }

  public get items(): readonly StructMember[] {
    return this.fetch();
  }
}

export class StateVariableAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new StateVariableAttribute(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.StateVariableAttributes);
  }

  public get items(): readonly StateVariableAttribute[] {
    return this.fetch();
  }
}

export class FunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new FunctionAttribute(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionAttributes);
  }

  public get items(): readonly FunctionAttribute[] {
    return this.fetch();
  }
}

export class ConstructorAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ConstructorAttribute(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ConstructorAttributes);
  }

  public get items(): readonly ConstructorAttribute[] {
    return this.fetch();
  }
}

export class UnnamedFunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new UnnamedFunctionAttribute(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UnnamedFunctionAttributes);
  }

  public get items(): readonly UnnamedFunctionAttribute[] {
    return this.fetch();
  }
}

export class FallbackFunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new FallbackFunctionAttribute(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FallbackFunctionAttributes);
  }

  public get items(): readonly FallbackFunctionAttribute[] {
    return this.fetch();
  }
}

export class ReceiveFunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ReceiveFunctionAttribute(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ReceiveFunctionAttributes);
  }

  public get items(): readonly ReceiveFunctionAttribute[] {
    return this.fetch();
  }
}

export class ModifierAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ModifierAttribute(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ModifierAttributes);
  }

  public get items(): readonly ModifierAttribute[] {
    return this.fetch();
  }
}

export class FunctionTypeAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new FunctionTypeAttribute(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.FunctionTypeAttributes);
  }

  public get items(): readonly FunctionTypeAttribute[] {
    return this.fetch();
  }
}

export class Statements {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new Statement(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.Statements);
  }

  public get items(): readonly Statement[] {
    return this.fetch();
  }
}

export class CatchClauses {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new CatchClause(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.CatchClauses);
  }

  public get items(): readonly CatchClause[] {
    return this.fetch();
  }
}

export class NamedArgumentGroups {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new NamedArgumentGroup(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.NamedArgumentGroups);
  }

  public get items(): readonly NamedArgumentGroup[] {
    return this.fetch();
  }
}

export class HexStringLiterals {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items as TokenNode[];
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.HexStringLiterals);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch();
  }
}

export class AsciiStringLiterals {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items as TokenNode[];
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.AsciiStringLiterals);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch();
  }
}

export class UnicodeStringLiterals {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items as TokenNode[];
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UnicodeStringLiterals);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch();
  }
}

export class YulStatements {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new YulStatement(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulStatements);
  }

  public get items(): readonly YulStatement[] {
    return this.fetch();
  }
}

export class YulSwitchCases {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new YulSwitchCase(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulSwitchCases);
  }

  public get items(): readonly YulSwitchCase[] {
    return this.fetch();
  }
}

/*
 * Separated:
 */

export class VersionPragmaSpecifier {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TokenNode[], separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.VersionPragmaSpecifier);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class ImportDeconstructionSymbols {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new ImportDeconstructionSymbol(item as RuleNode)),
      separators: separators as TokenNode[],
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ImportDeconstructionSymbols);
  }

  public get items(): readonly ImportDeconstructionSymbol[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class UsingDeconstructionSymbols {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new UsingDeconstructionSymbol(item as RuleNode)),
      separators: separators as TokenNode[],
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.UsingDeconstructionSymbols);
  }

  public get items(): readonly UsingDeconstructionSymbol[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class InheritanceTypes {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new InheritanceType(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.InheritanceTypes);
  }

  public get items(): readonly InheritanceType[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class EnumMembers {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TokenNode[], separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.EnumMembers);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class Parameters {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new Parameter(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.Parameters);
  }

  public get items(): readonly Parameter[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class OverridePaths {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new IdentifierPath(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.OverridePaths);
  }

  public get items(): readonly IdentifierPath[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class EventParameters {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new EventParameter(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.EventParameters);
  }

  public get items(): readonly EventParameter[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class ErrorParameters {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new ErrorParameter(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ErrorParameters);
  }

  public get items(): readonly ErrorParameter[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class AssemblyFlags {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TokenNode[], separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.AssemblyFlags);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class TupleDeconstructionElements {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new TupleDeconstructionElement(item as RuleNode)),
      separators: separators as TokenNode[],
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TupleDeconstructionElements);
  }

  public get items(): readonly TupleDeconstructionElement[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class PositionalArguments {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new Expression(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.PositionalArguments);
  }

  public get items(): readonly Expression[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class NamedArguments {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new NamedArgument(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.NamedArguments);
  }

  public get items(): readonly NamedArgument[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class TupleValues {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new TupleValue(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TupleValues);
  }

  public get items(): readonly TupleValue[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class ArrayValues {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new Expression(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.ArrayValues);
  }

  public get items(): readonly Expression[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class IdentifierPath {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TokenNode[], separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.IdentifierPath);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class YulParameters {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TokenNode[], separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulParameters);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class YulReturnVariables {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TokenNode[], separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulReturnVariables);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class YulArguments {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items.map((item) => new YulExpression(item as RuleNode)), separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulArguments);
  }

  public get items(): readonly YulExpression[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class YulIdentifierPaths {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new YulIdentifierPath(item as RuleNode)),
      separators: separators as TokenNode[],
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulIdentifierPaths);
  }

  public get items(): readonly YulIdentifierPath[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

export class YulIdentifierPath {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TokenNode[], separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.YulIdentifierPath);
  }

  public get items(): readonly TokenNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TokenNode[] {
    return this.fetch().separators;
  }
}

/*
 * Helpers:
 */

function once<T>(factory: () => T): () => T {
  let value: T | undefined;
  return () => {
    if (value === undefined) {
      value = factory();
    }
    return value;
  };
}

function assertKind(actual: RuleKind, expected: RuleKind): void {
  assert.equal(actual, expected, `${expected} can only be initialized with a CST node of the same kind.`);
}
