// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import * as assert from "node:assert";
import { ast_internal } from "../../napi-bindings/generated";
import { NonTerminalNode, TerminalNode } from "../../cst";
import { NonTerminalKind, TerminalKind } from "../../kinds";

/*
 * Sequences:
 */

export class SourceUnit {
  private readonly fetch = once(() => {
    const [$members] = ast_internal.selectSequence(this.cst);

    return {
      members: new SourceUnitMembers($members as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.SourceUnit);
  }

  public get members(): SourceUnitMembers {
    return this.fetch().members;
  }
}

export class PragmaDirective {
  private readonly fetch = once(() => {
    const [$pragmaKeyword, $pragma, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      pragmaKeyword: $pragmaKeyword as TerminalNode,
      pragma: new Pragma($pragma as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.PragmaDirective);
  }

  public get pragmaKeyword(): TerminalNode {
    return this.fetch().pragmaKeyword;
  }

  public get pragma(): Pragma {
    return this.fetch().pragma;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class ABICoderPragma {
  private readonly fetch = once(() => {
    const [$abicoderKeyword, $version] = ast_internal.selectSequence(this.cst);

    return {
      abicoderKeyword: $abicoderKeyword as TerminalNode,
      version: $version as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ABICoderPragma);
  }

  public get abicoderKeyword(): TerminalNode {
    return this.fetch().abicoderKeyword;
  }

  public get version(): TerminalNode {
    return this.fetch().version;
  }
}

export class ExperimentalPragma {
  private readonly fetch = once(() => {
    const [$experimentalKeyword, $feature] = ast_internal.selectSequence(this.cst);

    return {
      experimentalKeyword: $experimentalKeyword as TerminalNode,
      feature: new ExperimentalFeature($feature as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ExperimentalPragma);
  }

  public get experimentalKeyword(): TerminalNode {
    return this.fetch().experimentalKeyword;
  }

  public get feature(): ExperimentalFeature {
    return this.fetch().feature;
  }
}

export class VersionPragma {
  private readonly fetch = once(() => {
    const [$solidityKeyword, $sets] = ast_internal.selectSequence(this.cst);

    return {
      solidityKeyword: $solidityKeyword as TerminalNode,
      sets: new VersionExpressionSets($sets as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VersionPragma);
  }

  public get solidityKeyword(): TerminalNode {
    return this.fetch().solidityKeyword;
  }

  public get sets(): VersionExpressionSets {
    return this.fetch().sets;
  }
}

export class VersionRange {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new VersionExpression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new VersionExpression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VersionRange);
  }

  public get leftOperand(): VersionExpression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  public get rightOperand(): VersionExpression {
    return this.fetch().rightOperand;
  }
}

export class VersionComparator {
  private readonly fetch = once(() => {
    const [$operator, $operand] = ast_internal.selectSequence(this.cst);

    return {
      operator: $operator as TerminalNode,
      operand: new VersionExpression($operand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VersionComparator);
  }

  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  public get operand(): VersionExpression {
    return this.fetch().operand;
  }
}

export class ImportDirective {
  private readonly fetch = once(() => {
    const [$importKeyword, $clause, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      importKeyword: $importKeyword as TerminalNode,
      clause: new ImportClause($clause as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ImportDirective);
  }

  public get importKeyword(): TerminalNode {
    return this.fetch().importKeyword;
  }

  public get clause(): ImportClause {
    return this.fetch().clause;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class PathImport {
  private readonly fetch = once(() => {
    const [$path, $alias] = ast_internal.selectSequence(this.cst);

    return {
      path: new StringLiteral($path as NonTerminalNode),
      alias: $alias === null ? undefined : new ImportAlias($alias as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.PathImport);
  }

  public get path(): StringLiteral {
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
      asterisk: $asterisk as TerminalNode,
      alias: new ImportAlias($alias as NonTerminalNode),
      fromKeyword: $fromKeyword as TerminalNode,
      path: new StringLiteral($path as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.NamedImport);
  }

  public get asterisk(): TerminalNode {
    return this.fetch().asterisk;
  }

  public get alias(): ImportAlias {
    return this.fetch().alias;
  }

  public get fromKeyword(): TerminalNode {
    return this.fetch().fromKeyword;
  }

  public get path(): StringLiteral {
    return this.fetch().path;
  }
}

export class ImportDeconstruction {
  private readonly fetch = once(() => {
    const [$openBrace, $symbols, $closeBrace, $fromKeyword, $path] = ast_internal.selectSequence(this.cst);

    return {
      openBrace: $openBrace as TerminalNode,
      symbols: new ImportDeconstructionSymbols($symbols as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
      fromKeyword: $fromKeyword as TerminalNode,
      path: new StringLiteral($path as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ImportDeconstruction);
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get symbols(): ImportDeconstructionSymbols {
    return this.fetch().symbols;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }

  public get fromKeyword(): TerminalNode {
    return this.fetch().fromKeyword;
  }

  public get path(): StringLiteral {
    return this.fetch().path;
  }
}

export class ImportDeconstructionSymbol {
  private readonly fetch = once(() => {
    const [$name, $alias] = ast_internal.selectSequence(this.cst);

    return {
      name: $name as TerminalNode,
      alias: $alias === null ? undefined : new ImportAlias($alias as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ImportDeconstructionSymbol);
  }

  public get name(): TerminalNode {
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
      asKeyword: $asKeyword as TerminalNode,
      identifier: $identifier as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ImportAlias);
  }

  public get asKeyword(): TerminalNode {
    return this.fetch().asKeyword;
  }

  public get identifier(): TerminalNode {
    return this.fetch().identifier;
  }
}

export class UsingDirective {
  private readonly fetch = once(() => {
    const [$usingKeyword, $clause, $forKeyword, $target, $globalKeyword, $semicolon] = ast_internal.selectSequence(
      this.cst,
    );

    return {
      usingKeyword: $usingKeyword as TerminalNode,
      clause: new UsingClause($clause as NonTerminalNode),
      forKeyword: $forKeyword as TerminalNode,
      target: new UsingTarget($target as NonTerminalNode),
      globalKeyword: $globalKeyword === null ? undefined : ($globalKeyword as TerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UsingDirective);
  }

  public get usingKeyword(): TerminalNode {
    return this.fetch().usingKeyword;
  }

  public get clause(): UsingClause {
    return this.fetch().clause;
  }

  public get forKeyword(): TerminalNode {
    return this.fetch().forKeyword;
  }

  public get target(): UsingTarget {
    return this.fetch().target;
  }

  public get globalKeyword(): TerminalNode | undefined {
    return this.fetch().globalKeyword;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class UsingDeconstruction {
  private readonly fetch = once(() => {
    const [$openBrace, $symbols, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      openBrace: $openBrace as TerminalNode,
      symbols: new UsingDeconstructionSymbols($symbols as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UsingDeconstruction);
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get symbols(): UsingDeconstructionSymbols {
    return this.fetch().symbols;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class UsingDeconstructionSymbol {
  private readonly fetch = once(() => {
    const [$name, $alias] = ast_internal.selectSequence(this.cst);

    return {
      name: new IdentifierPath($name as NonTerminalNode),
      alias: $alias === null ? undefined : new UsingAlias($alias as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UsingDeconstructionSymbol);
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
      asKeyword: $asKeyword as TerminalNode,
      operator: new UsingOperator($operator as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UsingAlias);
  }

  public get asKeyword(): TerminalNode {
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
      abstractKeyword: $abstractKeyword === null ? undefined : ($abstractKeyword as TerminalNode),
      contractKeyword: $contractKeyword as TerminalNode,
      name: $name as TerminalNode,
      inheritence: $inheritence === null ? undefined : new InheritanceSpecifier($inheritence as NonTerminalNode),
      openBrace: $openBrace as TerminalNode,
      members: new ContractMembers($members as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ContractDefinition);
  }

  public get abstractKeyword(): TerminalNode | undefined {
    return this.fetch().abstractKeyword;
  }

  public get contractKeyword(): TerminalNode {
    return this.fetch().contractKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get inheritence(): InheritanceSpecifier | undefined {
    return this.fetch().inheritence;
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get members(): ContractMembers {
    return this.fetch().members;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class InheritanceSpecifier {
  private readonly fetch = once(() => {
    const [$isKeyword, $types] = ast_internal.selectSequence(this.cst);

    return {
      isKeyword: $isKeyword as TerminalNode,
      types: new InheritanceTypes($types as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.InheritanceSpecifier);
  }

  public get isKeyword(): TerminalNode {
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
      typeName: new IdentifierPath($typeName as NonTerminalNode),
      arguments: $arguments === null ? undefined : new ArgumentsDeclaration($arguments as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.InheritanceType);
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
      interfaceKeyword: $interfaceKeyword as TerminalNode,
      name: $name as TerminalNode,
      inheritence: $inheritence === null ? undefined : new InheritanceSpecifier($inheritence as NonTerminalNode),
      openBrace: $openBrace as TerminalNode,
      members: new InterfaceMembers($members as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.InterfaceDefinition);
  }

  public get interfaceKeyword(): TerminalNode {
    return this.fetch().interfaceKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get inheritence(): InheritanceSpecifier | undefined {
    return this.fetch().inheritence;
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get members(): InterfaceMembers {
    return this.fetch().members;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class LibraryDefinition {
  private readonly fetch = once(() => {
    const [$libraryKeyword, $name, $openBrace, $members, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      libraryKeyword: $libraryKeyword as TerminalNode,
      name: $name as TerminalNode,
      openBrace: $openBrace as TerminalNode,
      members: new LibraryMembers($members as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.LibraryDefinition);
  }

  public get libraryKeyword(): TerminalNode {
    return this.fetch().libraryKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get members(): LibraryMembers {
    return this.fetch().members;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class StructDefinition {
  private readonly fetch = once(() => {
    const [$structKeyword, $name, $openBrace, $members, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      structKeyword: $structKeyword as TerminalNode,
      name: $name as TerminalNode,
      openBrace: $openBrace as TerminalNode,
      members: new StructMembers($members as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StructDefinition);
  }

  public get structKeyword(): TerminalNode {
    return this.fetch().structKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get members(): StructMembers {
    return this.fetch().members;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class StructMember {
  private readonly fetch = once(() => {
    const [$typeName, $name, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonTerminalNode),
      name: $name as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StructMember);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class EnumDefinition {
  private readonly fetch = once(() => {
    const [$enumKeyword, $name, $openBrace, $members, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      enumKeyword: $enumKeyword as TerminalNode,
      name: $name as TerminalNode,
      openBrace: $openBrace as TerminalNode,
      members: new EnumMembers($members as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.EnumDefinition);
  }

  public get enumKeyword(): TerminalNode {
    return this.fetch().enumKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get members(): EnumMembers {
    return this.fetch().members;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class ConstantDefinition {
  private readonly fetch = once(() => {
    const [$typeName, $constantKeyword, $name, $equal, $value, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonTerminalNode),
      constantKeyword: $constantKeyword as TerminalNode,
      name: $name as TerminalNode,
      equal: $equal as TerminalNode,
      value: new Expression($value as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ConstantDefinition);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get constantKeyword(): TerminalNode {
    return this.fetch().constantKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get equal(): TerminalNode {
    return this.fetch().equal;
  }

  public get value(): Expression {
    return this.fetch().value;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class StateVariableDefinition {
  private readonly fetch = once(() => {
    const [$typeName, $attributes, $name, $value, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonTerminalNode),
      attributes: new StateVariableAttributes($attributes as NonTerminalNode),
      name: $name as TerminalNode,
      value: $value === null ? undefined : new StateVariableDefinitionValue($value as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StateVariableDefinition);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get attributes(): StateVariableAttributes {
    return this.fetch().attributes;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get value(): StateVariableDefinitionValue | undefined {
    return this.fetch().value;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class StateVariableDefinitionValue {
  private readonly fetch = once(() => {
    const [$equal, $value] = ast_internal.selectSequence(this.cst);

    return {
      equal: $equal as TerminalNode,
      value: new Expression($value as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StateVariableDefinitionValue);
  }

  public get equal(): TerminalNode {
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
      functionKeyword: $functionKeyword as TerminalNode,
      name: new FunctionName($name as NonTerminalNode),
      parameters: new ParametersDeclaration($parameters as NonTerminalNode),
      attributes: new FunctionAttributes($attributes as NonTerminalNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as NonTerminalNode),
      body: new FunctionBody($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FunctionDefinition);
  }

  public get functionKeyword(): TerminalNode {
    return this.fetch().functionKeyword;
  }

  public get name(): FunctionName {
    return this.fetch().name;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): FunctionAttributes {
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
      openParen: $openParen as TerminalNode,
      parameters: new Parameters($parameters as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ParametersDeclaration);
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get parameters(): Parameters {
    return this.fetch().parameters;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class Parameter {
  private readonly fetch = once(() => {
    const [$typeName, $storageLocation, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonTerminalNode),
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as NonTerminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Parameter);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

export class OverrideSpecifier {
  private readonly fetch = once(() => {
    const [$overrideKeyword, $overridden] = ast_internal.selectSequence(this.cst);

    return {
      overrideKeyword: $overrideKeyword as TerminalNode,
      overridden: $overridden === null ? undefined : new OverridePathsDeclaration($overridden as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.OverrideSpecifier);
  }

  public get overrideKeyword(): TerminalNode {
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
      openParen: $openParen as TerminalNode,
      paths: new OverridePaths($paths as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.OverridePathsDeclaration);
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get paths(): OverridePaths {
    return this.fetch().paths;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class ReturnsDeclaration {
  private readonly fetch = once(() => {
    const [$returnsKeyword, $variables] = ast_internal.selectSequence(this.cst);

    return {
      returnsKeyword: $returnsKeyword as TerminalNode,
      variables: new ParametersDeclaration($variables as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ReturnsDeclaration);
  }

  public get returnsKeyword(): TerminalNode {
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
      constructorKeyword: $constructorKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonTerminalNode),
      attributes: new ConstructorAttributes($attributes as NonTerminalNode),
      body: new Block($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ConstructorDefinition);
  }

  public get constructorKeyword(): TerminalNode {
    return this.fetch().constructorKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): ConstructorAttributes {
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
      functionKeyword: $functionKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonTerminalNode),
      attributes: new UnnamedFunctionAttributes($attributes as NonTerminalNode),
      body: new FunctionBody($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UnnamedFunctionDefinition);
  }

  public get functionKeyword(): TerminalNode {
    return this.fetch().functionKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): UnnamedFunctionAttributes {
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
      fallbackKeyword: $fallbackKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonTerminalNode),
      attributes: new FallbackFunctionAttributes($attributes as NonTerminalNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as NonTerminalNode),
      body: new FunctionBody($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FallbackFunctionDefinition);
  }

  public get fallbackKeyword(): TerminalNode {
    return this.fetch().fallbackKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): FallbackFunctionAttributes {
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
      receiveKeyword: $receiveKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonTerminalNode),
      attributes: new ReceiveFunctionAttributes($attributes as NonTerminalNode),
      body: new FunctionBody($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ReceiveFunctionDefinition);
  }

  public get receiveKeyword(): TerminalNode {
    return this.fetch().receiveKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): ReceiveFunctionAttributes {
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
      modifierKeyword: $modifierKeyword as TerminalNode,
      name: $name as TerminalNode,
      parameters: $parameters === null ? undefined : new ParametersDeclaration($parameters as NonTerminalNode),
      attributes: new ModifierAttributes($attributes as NonTerminalNode),
      body: new FunctionBody($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ModifierDefinition);
  }

  public get modifierKeyword(): TerminalNode {
    return this.fetch().modifierKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get parameters(): ParametersDeclaration | undefined {
    return this.fetch().parameters;
  }

  public get attributes(): ModifierAttributes {
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
      name: new IdentifierPath($name as NonTerminalNode),
      arguments: $arguments === null ? undefined : new ArgumentsDeclaration($arguments as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ModifierInvocation);
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
      eventKeyword: $eventKeyword as TerminalNode,
      name: $name as TerminalNode,
      parameters: new EventParametersDeclaration($parameters as NonTerminalNode),
      anonymousKeyword: $anonymousKeyword === null ? undefined : ($anonymousKeyword as TerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.EventDefinition);
  }

  public get eventKeyword(): TerminalNode {
    return this.fetch().eventKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get parameters(): EventParametersDeclaration {
    return this.fetch().parameters;
  }

  public get anonymousKeyword(): TerminalNode | undefined {
    return this.fetch().anonymousKeyword;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class EventParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      parameters: new EventParameters($parameters as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.EventParametersDeclaration);
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get parameters(): EventParameters {
    return this.fetch().parameters;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class EventParameter {
  private readonly fetch = once(() => {
    const [$typeName, $indexedKeyword, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonTerminalNode),
      indexedKeyword: $indexedKeyword === null ? undefined : ($indexedKeyword as TerminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.EventParameter);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get indexedKeyword(): TerminalNode | undefined {
    return this.fetch().indexedKeyword;
  }

  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

export class UserDefinedValueTypeDefinition {
  private readonly fetch = once(() => {
    const [$typeKeyword, $name, $isKeyword, $valueType, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      typeKeyword: $typeKeyword as TerminalNode,
      name: $name as TerminalNode,
      isKeyword: $isKeyword as TerminalNode,
      valueType: new ElementaryType($valueType as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UserDefinedValueTypeDefinition);
  }

  public get typeKeyword(): TerminalNode {
    return this.fetch().typeKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get isKeyword(): TerminalNode {
    return this.fetch().isKeyword;
  }

  public get valueType(): ElementaryType {
    return this.fetch().valueType;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class ErrorDefinition {
  private readonly fetch = once(() => {
    const [$errorKeyword, $name, $members, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      errorKeyword: $errorKeyword as TerminalNode,
      name: $name as TerminalNode,
      members: new ErrorParametersDeclaration($members as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ErrorDefinition);
  }

  public get errorKeyword(): TerminalNode {
    return this.fetch().errorKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get members(): ErrorParametersDeclaration {
    return this.fetch().members;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class ErrorParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      parameters: new ErrorParameters($parameters as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ErrorParametersDeclaration);
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get parameters(): ErrorParameters {
    return this.fetch().parameters;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class ErrorParameter {
  private readonly fetch = once(() => {
    const [$typeName, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonTerminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ErrorParameter);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

export class ArrayTypeName {
  private readonly fetch = once(() => {
    const [$operand, $openBracket, $index, $closeBracket] = ast_internal.selectSequence(this.cst);

    return {
      operand: new TypeName($operand as NonTerminalNode),
      openBracket: $openBracket as TerminalNode,
      index: $index === null ? undefined : new Expression($index as NonTerminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ArrayTypeName);
  }

  public get operand(): TypeName {
    return this.fetch().operand;
  }

  public get openBracket(): TerminalNode {
    return this.fetch().openBracket;
  }

  public get index(): Expression | undefined {
    return this.fetch().index;
  }

  public get closeBracket(): TerminalNode {
    return this.fetch().closeBracket;
  }
}

export class FunctionType {
  private readonly fetch = once(() => {
    const [$functionKeyword, $parameters, $attributes, $returns] = ast_internal.selectSequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonTerminalNode),
      attributes: new FunctionTypeAttributes($attributes as NonTerminalNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FunctionType);
  }

  public get functionKeyword(): TerminalNode {
    return this.fetch().functionKeyword;
  }

  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  public get attributes(): FunctionTypeAttributes {
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
      mappingKeyword: $mappingKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      keyType: new MappingKey($keyType as NonTerminalNode),
      equalGreaterThan: $equalGreaterThan as TerminalNode,
      valueType: new MappingValue($valueType as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.MappingType);
  }

  public get mappingKeyword(): TerminalNode {
    return this.fetch().mappingKeyword;
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get keyType(): MappingKey {
    return this.fetch().keyType;
  }

  public get equalGreaterThan(): TerminalNode {
    return this.fetch().equalGreaterThan;
  }

  public get valueType(): MappingValue {
    return this.fetch().valueType;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class MappingKey {
  private readonly fetch = once(() => {
    const [$keyType, $name] = ast_internal.selectSequence(this.cst);

    return {
      keyType: new MappingKeyType($keyType as NonTerminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.MappingKey);
  }

  public get keyType(): MappingKeyType {
    return this.fetch().keyType;
  }

  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

export class MappingValue {
  private readonly fetch = once(() => {
    const [$typeName, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonTerminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.MappingValue);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

export class AddressType {
  private readonly fetch = once(() => {
    const [$addressKeyword, $payableKeyword] = ast_internal.selectSequence(this.cst);

    return {
      addressKeyword: $addressKeyword as TerminalNode,
      payableKeyword: $payableKeyword === null ? undefined : ($payableKeyword as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.AddressType);
  }

  public get addressKeyword(): TerminalNode {
    return this.fetch().addressKeyword;
  }

  public get payableKeyword(): TerminalNode | undefined {
    return this.fetch().payableKeyword;
  }
}

export class Block {
  private readonly fetch = once(() => {
    const [$openBrace, $statements, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      openBrace: $openBrace as TerminalNode,
      statements: new Statements($statements as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Block);
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get statements(): Statements {
    return this.fetch().statements;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class UncheckedBlock {
  private readonly fetch = once(() => {
    const [$uncheckedKeyword, $block] = ast_internal.selectSequence(this.cst);

    return {
      uncheckedKeyword: $uncheckedKeyword as TerminalNode,
      block: new Block($block as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UncheckedBlock);
  }

  public get uncheckedKeyword(): TerminalNode {
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
      expression: new Expression($expression as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ExpressionStatement);
  }

  public get expression(): Expression {
    return this.fetch().expression;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class AssemblyStatement {
  private readonly fetch = once(() => {
    const [$assemblyKeyword, $label, $flags, $body] = ast_internal.selectSequence(this.cst);

    return {
      assemblyKeyword: $assemblyKeyword as TerminalNode,
      label: $label === null ? undefined : new StringLiteral($label as NonTerminalNode),
      flags: $flags === null ? undefined : new AssemblyFlagsDeclaration($flags as NonTerminalNode),
      body: new YulBlock($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.AssemblyStatement);
  }

  public get assemblyKeyword(): TerminalNode {
    return this.fetch().assemblyKeyword;
  }

  public get label(): StringLiteral | undefined {
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
      openParen: $openParen as TerminalNode,
      flags: new AssemblyFlags($flags as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.AssemblyFlagsDeclaration);
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get flags(): AssemblyFlags {
    return this.fetch().flags;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class TupleDeconstructionStatement {
  private readonly fetch = once(() => {
    const [$varKeyword, $openParen, $elements, $closeParen, $equal, $expression, $semicolon] =
      ast_internal.selectSequence(this.cst);

    return {
      varKeyword: $varKeyword === null ? undefined : ($varKeyword as TerminalNode),
      openParen: $openParen as TerminalNode,
      elements: new TupleDeconstructionElements($elements as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
      equal: $equal as TerminalNode,
      expression: new Expression($expression as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TupleDeconstructionStatement);
  }

  public get varKeyword(): TerminalNode | undefined {
    return this.fetch().varKeyword;
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get elements(): TupleDeconstructionElements {
    return this.fetch().elements;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }

  public get equal(): TerminalNode {
    return this.fetch().equal;
  }

  public get expression(): Expression {
    return this.fetch().expression;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class TupleDeconstructionElement {
  private readonly fetch = once(() => {
    const [$member] = ast_internal.selectSequence(this.cst);

    return {
      member: $member === null ? undefined : new TupleMember($member as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TupleDeconstructionElement);
  }

  public get member(): TupleMember | undefined {
    return this.fetch().member;
  }
}

export class TypedTupleMember {
  private readonly fetch = once(() => {
    const [$typeName, $storageLocation, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonTerminalNode),
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as NonTerminalNode),
      name: $name as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TypedTupleMember);
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }
}

export class UntypedTupleMember {
  private readonly fetch = once(() => {
    const [$storageLocation, $name] = ast_internal.selectSequence(this.cst);

    return {
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as NonTerminalNode),
      name: $name as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UntypedTupleMember);
  }

  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }
}

export class VariableDeclarationStatement {
  private readonly fetch = once(() => {
    const [$variableType, $storageLocation, $name, $value, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      variableType: new VariableDeclarationType($variableType as NonTerminalNode),
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as NonTerminalNode),
      name: $name as TerminalNode,
      value: $value === null ? undefined : new VariableDeclarationValue($value as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VariableDeclarationStatement);
  }

  public get variableType(): VariableDeclarationType {
    return this.fetch().variableType;
  }

  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get value(): VariableDeclarationValue | undefined {
    return this.fetch().value;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class VariableDeclarationValue {
  private readonly fetch = once(() => {
    const [$equal, $expression] = ast_internal.selectSequence(this.cst);

    return {
      equal: $equal as TerminalNode,
      expression: new Expression($expression as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VariableDeclarationValue);
  }

  public get equal(): TerminalNode {
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
      ifKeyword: $ifKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      condition: new Expression($condition as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
      body: new Statement($body as NonTerminalNode),
      elseBranch: $elseBranch === null ? undefined : new ElseBranch($elseBranch as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.IfStatement);
  }

  public get ifKeyword(): TerminalNode {
    return this.fetch().ifKeyword;
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get condition(): Expression {
    return this.fetch().condition;
  }

  public get closeParen(): TerminalNode {
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
      elseKeyword: $elseKeyword as TerminalNode,
      body: new Statement($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ElseBranch);
  }

  public get elseKeyword(): TerminalNode {
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
      forKeyword: $forKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      initialization: new ForStatementInitialization($initialization as NonTerminalNode),
      condition: new ForStatementCondition($condition as NonTerminalNode),
      iterator: $iterator === null ? undefined : new Expression($iterator as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
      body: new Statement($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ForStatement);
  }

  public get forKeyword(): TerminalNode {
    return this.fetch().forKeyword;
  }

  public get openParen(): TerminalNode {
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

  public get closeParen(): TerminalNode {
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
      whileKeyword: $whileKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      condition: new Expression($condition as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
      body: new Statement($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.WhileStatement);
  }

  public get whileKeyword(): TerminalNode {
    return this.fetch().whileKeyword;
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get condition(): Expression {
    return this.fetch().condition;
  }

  public get closeParen(): TerminalNode {
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
      doKeyword: $doKeyword as TerminalNode,
      body: new Statement($body as NonTerminalNode),
      whileKeyword: $whileKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      condition: new Expression($condition as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.DoWhileStatement);
  }

  public get doKeyword(): TerminalNode {
    return this.fetch().doKeyword;
  }

  public get body(): Statement {
    return this.fetch().body;
  }

  public get whileKeyword(): TerminalNode {
    return this.fetch().whileKeyword;
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get condition(): Expression {
    return this.fetch().condition;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class ContinueStatement {
  private readonly fetch = once(() => {
    const [$continueKeyword, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      continueKeyword: $continueKeyword as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ContinueStatement);
  }

  public get continueKeyword(): TerminalNode {
    return this.fetch().continueKeyword;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class BreakStatement {
  private readonly fetch = once(() => {
    const [$breakKeyword, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      breakKeyword: $breakKeyword as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.BreakStatement);
  }

  public get breakKeyword(): TerminalNode {
    return this.fetch().breakKeyword;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class ReturnStatement {
  private readonly fetch = once(() => {
    const [$returnKeyword, $expression, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      returnKeyword: $returnKeyword as TerminalNode,
      expression: $expression === null ? undefined : new Expression($expression as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ReturnStatement);
  }

  public get returnKeyword(): TerminalNode {
    return this.fetch().returnKeyword;
  }

  public get expression(): Expression | undefined {
    return this.fetch().expression;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class EmitStatement {
  private readonly fetch = once(() => {
    const [$emitKeyword, $event, $arguments, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      emitKeyword: $emitKeyword as TerminalNode,
      event: new IdentifierPath($event as NonTerminalNode),
      arguments: new ArgumentsDeclaration($arguments as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.EmitStatement);
  }

  public get emitKeyword(): TerminalNode {
    return this.fetch().emitKeyword;
  }

  public get event(): IdentifierPath {
    return this.fetch().event;
  }

  public get arguments(): ArgumentsDeclaration {
    return this.fetch().arguments;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class TryStatement {
  private readonly fetch = once(() => {
    const [$tryKeyword, $expression, $returns, $body, $catchClauses] = ast_internal.selectSequence(this.cst);

    return {
      tryKeyword: $tryKeyword as TerminalNode,
      expression: new Expression($expression as NonTerminalNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as NonTerminalNode),
      body: new Block($body as NonTerminalNode),
      catchClauses: new CatchClauses($catchClauses as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TryStatement);
  }

  public get tryKeyword(): TerminalNode {
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
      catchKeyword: $catchKeyword as TerminalNode,
      error: $error === null ? undefined : new CatchClauseError($error as NonTerminalNode),
      body: new Block($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.CatchClause);
  }

  public get catchKeyword(): TerminalNode {
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
      name: $name === null ? undefined : ($name as TerminalNode),
      parameters: new ParametersDeclaration($parameters as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.CatchClauseError);
  }

  public get name(): TerminalNode | undefined {
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
      revertKeyword: $revertKeyword as TerminalNode,
      error: $error === null ? undefined : new IdentifierPath($error as NonTerminalNode),
      arguments: new ArgumentsDeclaration($arguments as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.RevertStatement);
  }

  public get revertKeyword(): TerminalNode {
    return this.fetch().revertKeyword;
  }

  public get error(): IdentifierPath | undefined {
    return this.fetch().error;
  }

  public get arguments(): ArgumentsDeclaration {
    return this.fetch().arguments;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class ThrowStatement {
  private readonly fetch = once(() => {
    const [$throwKeyword, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      throwKeyword: $throwKeyword as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ThrowStatement);
  }

  public get throwKeyword(): TerminalNode {
    return this.fetch().throwKeyword;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class AssignmentExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.AssignmentExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class ConditionalExpression {
  private readonly fetch = once(() => {
    const [$operand, $questionMark, $trueExpression, $colon, $falseExpression] = ast_internal.selectSequence(this.cst);

    return {
      operand: new Expression($operand as NonTerminalNode),
      questionMark: $questionMark as TerminalNode,
      trueExpression: new Expression($trueExpression as NonTerminalNode),
      colon: $colon as TerminalNode,
      falseExpression: new Expression($falseExpression as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ConditionalExpression);
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }

  public get questionMark(): TerminalNode {
    return this.fetch().questionMark;
  }

  public get trueExpression(): Expression {
    return this.fetch().trueExpression;
  }

  public get colon(): TerminalNode {
    return this.fetch().colon;
  }

  public get falseExpression(): Expression {
    return this.fetch().falseExpression;
  }
}

export class OrExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.OrExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.AndExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.EqualityExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ComparisonExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.BitwiseOrExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.BitwiseXorExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.BitwiseAndExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ShiftExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.AdditiveExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.MultiplicativeExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
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
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ExponentiationExpression);
  }

  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

export class PostfixExpression {
  private readonly fetch = once(() => {
    const [$operand, $operator] = ast_internal.selectSequence(this.cst);

    return {
      operand: new Expression($operand as NonTerminalNode),
      operator: $operator as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.PostfixExpression);
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }

  public get operator(): TerminalNode {
    return this.fetch().operator;
  }
}

export class PrefixExpression {
  private readonly fetch = once(() => {
    const [$operator, $operand] = ast_internal.selectSequence(this.cst);

    return {
      operator: $operator as TerminalNode,
      operand: new Expression($operand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.PrefixExpression);
  }

  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }
}

export class FunctionCallExpression {
  private readonly fetch = once(() => {
    const [$operand, $arguments] = ast_internal.selectSequence(this.cst);

    return {
      operand: new Expression($operand as NonTerminalNode),
      arguments: new ArgumentsDeclaration($arguments as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FunctionCallExpression);
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }

  public get arguments(): ArgumentsDeclaration {
    return this.fetch().arguments;
  }
}

export class CallOptionsExpression {
  private readonly fetch = once(() => {
    const [$operand, $openBrace, $options, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      operand: new Expression($operand as NonTerminalNode),
      openBrace: $openBrace as TerminalNode,
      options: new CallOptions($options as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.CallOptionsExpression);
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get options(): CallOptions {
    return this.fetch().options;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class MemberAccessExpression {
  private readonly fetch = once(() => {
    const [$operand, $period, $member] = ast_internal.selectSequence(this.cst);

    return {
      operand: new Expression($operand as NonTerminalNode),
      period: $period as TerminalNode,
      member: new MemberAccess($member as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.MemberAccessExpression);
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }

  public get period(): TerminalNode {
    return this.fetch().period;
  }

  public get member(): MemberAccess {
    return this.fetch().member;
  }
}

export class IndexAccessExpression {
  private readonly fetch = once(() => {
    const [$operand, $openBracket, $start, $end, $closeBracket] = ast_internal.selectSequence(this.cst);

    return {
      operand: new Expression($operand as NonTerminalNode),
      openBracket: $openBracket as TerminalNode,
      start: $start === null ? undefined : new Expression($start as NonTerminalNode),
      end: $end === null ? undefined : new IndexAccessEnd($end as NonTerminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.IndexAccessExpression);
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }

  public get openBracket(): TerminalNode {
    return this.fetch().openBracket;
  }

  public get start(): Expression | undefined {
    return this.fetch().start;
  }

  public get end(): IndexAccessEnd | undefined {
    return this.fetch().end;
  }

  public get closeBracket(): TerminalNode {
    return this.fetch().closeBracket;
  }
}

export class IndexAccessEnd {
  private readonly fetch = once(() => {
    const [$colon, $end] = ast_internal.selectSequence(this.cst);

    return {
      colon: $colon as TerminalNode,
      end: $end === null ? undefined : new Expression($end as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.IndexAccessEnd);
  }

  public get colon(): TerminalNode {
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
      openParen: $openParen as TerminalNode,
      arguments: new PositionalArguments($arguments as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.PositionalArgumentsDeclaration);
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get arguments(): PositionalArguments {
    return this.fetch().arguments;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class NamedArgumentsDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $arguments, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      arguments: $arguments === null ? undefined : new NamedArgumentGroup($arguments as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.NamedArgumentsDeclaration);
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get arguments(): NamedArgumentGroup | undefined {
    return this.fetch().arguments;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class NamedArgumentGroup {
  private readonly fetch = once(() => {
    const [$openBrace, $arguments, $closeBrace] = ast_internal.selectSequence(this.cst);

    return {
      openBrace: $openBrace as TerminalNode,
      arguments: new NamedArguments($arguments as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.NamedArgumentGroup);
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get arguments(): NamedArguments {
    return this.fetch().arguments;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class NamedArgument {
  private readonly fetch = once(() => {
    const [$name, $colon, $value] = ast_internal.selectSequence(this.cst);

    return {
      name: $name as TerminalNode,
      colon: $colon as TerminalNode,
      value: new Expression($value as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.NamedArgument);
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get colon(): TerminalNode {
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
      typeKeyword: $typeKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      typeName: new TypeName($typeName as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TypeExpression);
  }

  public get typeKeyword(): TerminalNode {
    return this.fetch().typeKeyword;
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class NewExpression {
  private readonly fetch = once(() => {
    const [$newKeyword, $typeName] = ast_internal.selectSequence(this.cst);

    return {
      newKeyword: $newKeyword as TerminalNode,
      typeName: new TypeName($typeName as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.NewExpression);
  }

  public get newKeyword(): TerminalNode {
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
      openParen: $openParen as TerminalNode,
      items: new TupleValues($items as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TupleExpression);
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get items(): TupleValues {
    return this.fetch().items;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class TupleValue {
  private readonly fetch = once(() => {
    const [$expression] = ast_internal.selectSequence(this.cst);

    return {
      expression: $expression === null ? undefined : new Expression($expression as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TupleValue);
  }

  public get expression(): Expression | undefined {
    return this.fetch().expression;
  }
}

export class ArrayExpression {
  private readonly fetch = once(() => {
    const [$openBracket, $items, $closeBracket] = ast_internal.selectSequence(this.cst);

    return {
      openBracket: $openBracket as TerminalNode,
      items: new ArrayValues($items as NonTerminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ArrayExpression);
  }

  public get openBracket(): TerminalNode {
    return this.fetch().openBracket;
  }

  public get items(): ArrayValues {
    return this.fetch().items;
  }

  public get closeBracket(): TerminalNode {
    return this.fetch().closeBracket;
  }
}

export class HexNumberExpression {
  private readonly fetch = once(() => {
    const [$literal, $unit] = ast_internal.selectSequence(this.cst);

    return {
      literal: $literal as TerminalNode,
      unit: $unit === null ? undefined : new NumberUnit($unit as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.HexNumberExpression);
  }

  public get literal(): TerminalNode {
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
      literal: $literal as TerminalNode,
      unit: $unit === null ? undefined : new NumberUnit($unit as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.DecimalNumberExpression);
  }

  public get literal(): TerminalNode {
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
      openBrace: $openBrace as TerminalNode,
      statements: new YulStatements($statements as NonTerminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulBlock);
  }

  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  public get statements(): YulStatements {
    return this.fetch().statements;
  }

  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

export class YulFunctionDefinition {
  private readonly fetch = once(() => {
    const [$functionKeyword, $name, $parameters, $returns, $body] = ast_internal.selectSequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TerminalNode,
      name: $name as TerminalNode,
      parameters: new YulParametersDeclaration($parameters as NonTerminalNode),
      returns: $returns === null ? undefined : new YulReturnsDeclaration($returns as NonTerminalNode),
      body: new YulBlock($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulFunctionDefinition);
  }

  public get functionKeyword(): TerminalNode {
    return this.fetch().functionKeyword;
  }

  public get name(): TerminalNode {
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
      openParen: $openParen as TerminalNode,
      parameters: new YulParameters($parameters as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulParametersDeclaration);
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get parameters(): YulParameters {
    return this.fetch().parameters;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

export class YulReturnsDeclaration {
  private readonly fetch = once(() => {
    const [$minusGreaterThan, $variables] = ast_internal.selectSequence(this.cst);

    return {
      minusGreaterThan: $minusGreaterThan as TerminalNode,
      variables: new YulReturnVariables($variables as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulReturnsDeclaration);
  }

  public get minusGreaterThan(): TerminalNode {
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
      letKeyword: $letKeyword as TerminalNode,
      names: $names as TerminalNode,
      value: $value === null ? undefined : new YulVariableDeclarationValue($value as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulVariableDeclarationStatement);
  }

  public get letKeyword(): TerminalNode {
    return this.fetch().letKeyword;
  }

  public get names(): TerminalNode {
    return this.fetch().names;
  }

  public get value(): YulVariableDeclarationValue | undefined {
    return this.fetch().value;
  }
}

export class YulVariableDeclarationValue {
  private readonly fetch = once(() => {
    const [$assignment, $expression] = ast_internal.selectSequence(this.cst);

    return {
      assignment: new YulAssignmentOperator($assignment as NonTerminalNode),
      expression: new YulExpression($expression as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulVariableDeclarationValue);
  }

  public get assignment(): YulAssignmentOperator {
    return this.fetch().assignment;
  }

  public get expression(): YulExpression {
    return this.fetch().expression;
  }
}

export class YulAssignmentStatement {
  private readonly fetch = once(() => {
    const [$names, $assignment, $expression] = ast_internal.selectSequence(this.cst);

    return {
      names: new YulPaths($names as NonTerminalNode),
      assignment: new YulAssignmentOperator($assignment as NonTerminalNode),
      expression: new YulExpression($expression as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulAssignmentStatement);
  }

  public get names(): YulPaths {
    return this.fetch().names;
  }

  public get assignment(): YulAssignmentOperator {
    return this.fetch().assignment;
  }

  public get expression(): YulExpression {
    return this.fetch().expression;
  }
}

export class YulColonAndEqual {
  private readonly fetch = once(() => {
    const [$colon, $equal] = ast_internal.selectSequence(this.cst);

    return {
      colon: $colon as TerminalNode,
      equal: $equal as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulColonAndEqual);
  }

  public get colon(): TerminalNode {
    return this.fetch().colon;
  }

  public get equal(): TerminalNode {
    return this.fetch().equal;
  }
}

export class YulIfStatement {
  private readonly fetch = once(() => {
    const [$ifKeyword, $condition, $body] = ast_internal.selectSequence(this.cst);

    return {
      ifKeyword: $ifKeyword as TerminalNode,
      condition: new YulExpression($condition as NonTerminalNode),
      body: new YulBlock($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulIfStatement);
  }

  public get ifKeyword(): TerminalNode {
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
      forKeyword: $forKeyword as TerminalNode,
      initialization: new YulBlock($initialization as NonTerminalNode),
      condition: new YulExpression($condition as NonTerminalNode),
      iterator: new YulBlock($iterator as NonTerminalNode),
      body: new YulBlock($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulForStatement);
  }

  public get forKeyword(): TerminalNode {
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
      switchKeyword: $switchKeyword as TerminalNode,
      expression: new YulExpression($expression as NonTerminalNode),
      cases: new YulSwitchCases($cases as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulSwitchStatement);
  }

  public get switchKeyword(): TerminalNode {
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
      defaultKeyword: $defaultKeyword as TerminalNode,
      body: new YulBlock($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulDefaultCase);
  }

  public get defaultKeyword(): TerminalNode {
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
      caseKeyword: $caseKeyword as TerminalNode,
      value: new YulLiteral($value as NonTerminalNode),
      body: new YulBlock($body as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulValueCase);
  }

  public get caseKeyword(): TerminalNode {
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
      leaveKeyword: $leaveKeyword as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulLeaveStatement);
  }

  public get leaveKeyword(): TerminalNode {
    return this.fetch().leaveKeyword;
  }
}

export class YulBreakStatement {
  private readonly fetch = once(() => {
    const [$breakKeyword] = ast_internal.selectSequence(this.cst);

    return {
      breakKeyword: $breakKeyword as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulBreakStatement);
  }

  public get breakKeyword(): TerminalNode {
    return this.fetch().breakKeyword;
  }
}

export class YulContinueStatement {
  private readonly fetch = once(() => {
    const [$continueKeyword] = ast_internal.selectSequence(this.cst);

    return {
      continueKeyword: $continueKeyword as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulContinueStatement);
  }

  public get continueKeyword(): TerminalNode {
    return this.fetch().continueKeyword;
  }
}

export class YulLabel {
  private readonly fetch = once(() => {
    const [$label, $colon] = ast_internal.selectSequence(this.cst);

    return {
      label: $label as TerminalNode,
      colon: $colon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulLabel);
  }

  public get label(): TerminalNode {
    return this.fetch().label;
  }

  public get colon(): TerminalNode {
    return this.fetch().colon;
  }
}

export class YulFunctionCallExpression {
  private readonly fetch = once(() => {
    const [$operand, $openParen, $arguments, $closeParen] = ast_internal.selectSequence(this.cst);

    return {
      operand: new YulExpression($operand as NonTerminalNode),
      openParen: $openParen as TerminalNode,
      arguments: new YulArguments($arguments as NonTerminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulFunctionCallExpression);
  }

  public get operand(): YulExpression {
    return this.fetch().operand;
  }

  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  public get arguments(): YulArguments {
    return this.fetch().arguments;
  }

  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
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
      case NonTerminalKind.PragmaDirective:
        return new PragmaDirective(variant as NonTerminalNode);
      case NonTerminalKind.ImportDirective:
        return new ImportDirective(variant as NonTerminalNode);
      case NonTerminalKind.ContractDefinition:
        return new ContractDefinition(variant as NonTerminalNode);
      case NonTerminalKind.InterfaceDefinition:
        return new InterfaceDefinition(variant as NonTerminalNode);
      case NonTerminalKind.LibraryDefinition:
        return new LibraryDefinition(variant as NonTerminalNode);
      case NonTerminalKind.StructDefinition:
        return new StructDefinition(variant as NonTerminalNode);
      case NonTerminalKind.EnumDefinition:
        return new EnumDefinition(variant as NonTerminalNode);
      case NonTerminalKind.FunctionDefinition:
        return new FunctionDefinition(variant as NonTerminalNode);
      case NonTerminalKind.ConstantDefinition:
        return new ConstantDefinition(variant as NonTerminalNode);
      case NonTerminalKind.ErrorDefinition:
        return new ErrorDefinition(variant as NonTerminalNode);
      case NonTerminalKind.UserDefinedValueTypeDefinition:
        return new UserDefinedValueTypeDefinition(variant as NonTerminalNode);
      case NonTerminalKind.UsingDirective:
        return new UsingDirective(variant as NonTerminalNode);
      case NonTerminalKind.EventDefinition:
        return new EventDefinition(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.SourceUnitMember);
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
      case NonTerminalKind.ABICoderPragma:
        return new ABICoderPragma(variant as NonTerminalNode);
      case NonTerminalKind.ExperimentalPragma:
        return new ExperimentalPragma(variant as NonTerminalNode);
      case NonTerminalKind.VersionPragma:
        return new VersionPragma(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Pragma);
  }

  public get variant(): ABICoderPragma | ExperimentalPragma | VersionPragma {
    return this.fetch();
  }
}

export class ExperimentalFeature {
  private readonly fetch: () => StringLiteral | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.StringLiteral:
        return new StringLiteral(variant as NonTerminalNode);

      case TerminalKind.Identifier:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ExperimentalFeature);
  }

  public get variant(): StringLiteral | TerminalNode {
    return this.fetch();
  }
}

export class VersionExpression {
  private readonly fetch: () => VersionRange | VersionComparator | VersionSpecifiers | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.VersionRange:
        return new VersionRange(variant as NonTerminalNode);
      case NonTerminalKind.VersionComparator:
        return new VersionComparator(variant as NonTerminalNode);
      case NonTerminalKind.VersionSpecifiers:
        return new VersionSpecifiers(variant as NonTerminalNode);

      case TerminalKind.SingleQuotedVersionLiteral:
      case TerminalKind.DoubleQuotedVersionLiteral:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VersionExpression);
  }

  public get variant(): VersionRange | VersionComparator | VersionSpecifiers | TerminalNode {
    return this.fetch();
  }
}

export class ImportClause {
  private readonly fetch: () => PathImport | NamedImport | ImportDeconstruction = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.PathImport:
        return new PathImport(variant as NonTerminalNode);
      case NonTerminalKind.NamedImport:
        return new NamedImport(variant as NonTerminalNode);
      case NonTerminalKind.ImportDeconstruction:
        return new ImportDeconstruction(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ImportClause);
  }

  public get variant(): PathImport | NamedImport | ImportDeconstruction {
    return this.fetch();
  }
}

export class UsingClause {
  private readonly fetch: () => IdentifierPath | UsingDeconstruction = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.IdentifierPath:
        return new IdentifierPath(variant as NonTerminalNode);
      case NonTerminalKind.UsingDeconstruction:
        return new UsingDeconstruction(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UsingClause);
  }

  public get variant(): IdentifierPath | UsingDeconstruction {
    return this.fetch();
  }
}

export class UsingOperator {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.Ampersand:
      case TerminalKind.Asterisk:
      case TerminalKind.BangEqual:
      case TerminalKind.Bar:
      case TerminalKind.Caret:
      case TerminalKind.EqualEqual:
      case TerminalKind.GreaterThan:
      case TerminalKind.GreaterThanEqual:
      case TerminalKind.LessThan:
      case TerminalKind.LessThanEqual:
      case TerminalKind.Minus:
      case TerminalKind.Percent:
      case TerminalKind.Plus:
      case TerminalKind.Slash:
      case TerminalKind.Tilde:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UsingOperator);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class UsingTarget {
  private readonly fetch: () => TypeName | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.TypeName:
        return new TypeName(variant as NonTerminalNode);

      case TerminalKind.Asterisk:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UsingTarget);
  }

  public get variant(): TypeName | TerminalNode {
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
      case NonTerminalKind.UsingDirective:
        return new UsingDirective(variant as NonTerminalNode);
      case NonTerminalKind.FunctionDefinition:
        return new FunctionDefinition(variant as NonTerminalNode);
      case NonTerminalKind.ConstructorDefinition:
        return new ConstructorDefinition(variant as NonTerminalNode);
      case NonTerminalKind.ReceiveFunctionDefinition:
        return new ReceiveFunctionDefinition(variant as NonTerminalNode);
      case NonTerminalKind.FallbackFunctionDefinition:
        return new FallbackFunctionDefinition(variant as NonTerminalNode);
      case NonTerminalKind.UnnamedFunctionDefinition:
        return new UnnamedFunctionDefinition(variant as NonTerminalNode);
      case NonTerminalKind.ModifierDefinition:
        return new ModifierDefinition(variant as NonTerminalNode);
      case NonTerminalKind.StructDefinition:
        return new StructDefinition(variant as NonTerminalNode);
      case NonTerminalKind.EnumDefinition:
        return new EnumDefinition(variant as NonTerminalNode);
      case NonTerminalKind.EventDefinition:
        return new EventDefinition(variant as NonTerminalNode);
      case NonTerminalKind.StateVariableDefinition:
        return new StateVariableDefinition(variant as NonTerminalNode);
      case NonTerminalKind.ErrorDefinition:
        return new ErrorDefinition(variant as NonTerminalNode);
      case NonTerminalKind.UserDefinedValueTypeDefinition:
        return new UserDefinedValueTypeDefinition(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ContractMember);
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
  private readonly fetch: () => OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonTerminalNode);

      case TerminalKind.ConstantKeyword:
      case TerminalKind.InternalKeyword:
      case TerminalKind.PrivateKeyword:
      case TerminalKind.PublicKeyword:
      case TerminalKind.ImmutableKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StateVariableAttribute);
  }

  public get variant(): OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

export class FunctionName {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.Identifier:
      case TerminalKind.FallbackKeyword:
      case TerminalKind.ReceiveKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FunctionName);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class FunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonTerminalNode);
      case NonTerminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonTerminalNode);

      case TerminalKind.ConstantKeyword:
      case TerminalKind.ExternalKeyword:
      case TerminalKind.InternalKeyword:
      case TerminalKind.PayableKeyword:
      case TerminalKind.PrivateKeyword:
      case TerminalKind.PublicKeyword:
      case TerminalKind.PureKeyword:
      case TerminalKind.ViewKeyword:
      case TerminalKind.VirtualKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

export class FunctionBody {
  private readonly fetch: () => Block | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.Block:
        return new Block(variant as NonTerminalNode);

      case TerminalKind.Semicolon:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FunctionBody);
  }

  public get variant(): Block | TerminalNode {
    return this.fetch();
  }
}

export class ConstructorAttribute {
  private readonly fetch: () => ModifierInvocation | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonTerminalNode);

      case TerminalKind.InternalKeyword:
      case TerminalKind.OverrideKeyword:
      case TerminalKind.PayableKeyword:
      case TerminalKind.PublicKeyword:
      case TerminalKind.VirtualKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ConstructorAttribute);
  }

  public get variant(): ModifierInvocation | TerminalNode {
    return this.fetch();
  }
}

export class UnnamedFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonTerminalNode);

      case TerminalKind.ConstantKeyword:
      case TerminalKind.ExternalKeyword:
      case TerminalKind.InternalKeyword:
      case TerminalKind.PayableKeyword:
      case TerminalKind.PrivateKeyword:
      case TerminalKind.PublicKeyword:
      case TerminalKind.PureKeyword:
      case TerminalKind.ViewKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UnnamedFunctionAttribute);
  }

  public get variant(): ModifierInvocation | TerminalNode {
    return this.fetch();
  }
}

export class FallbackFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonTerminalNode);
      case NonTerminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonTerminalNode);

      case TerminalKind.ExternalKeyword:
      case TerminalKind.PayableKeyword:
      case TerminalKind.PureKeyword:
      case TerminalKind.ViewKeyword:
      case TerminalKind.VirtualKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FallbackFunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

export class ReceiveFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonTerminalNode);
      case NonTerminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonTerminalNode);

      case TerminalKind.ExternalKeyword:
      case TerminalKind.PayableKeyword:
      case TerminalKind.VirtualKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ReceiveFunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

export class ModifierAttribute {
  private readonly fetch: () => OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonTerminalNode);

      case TerminalKind.VirtualKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ModifierAttribute);
  }

  public get variant(): OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

export class TypeName {
  private readonly fetch: () => ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath = once(
    () => {
      const variant = ast_internal.selectChoice(this.cst);

      switch (variant.kind) {
        case NonTerminalKind.ArrayTypeName:
          return new ArrayTypeName(variant as NonTerminalNode);
        case NonTerminalKind.FunctionType:
          return new FunctionType(variant as NonTerminalNode);
        case NonTerminalKind.MappingType:
          return new MappingType(variant as NonTerminalNode);
        case NonTerminalKind.ElementaryType:
          return new ElementaryType(variant as NonTerminalNode);
        case NonTerminalKind.IdentifierPath:
          return new IdentifierPath(variant as NonTerminalNode);

        default:
          assert.fail(`Unexpected variant: ${variant.kind}`);
      }
    },
  );

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TypeName);
  }

  public get variant(): ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath {
    return this.fetch();
  }
}

export class FunctionTypeAttribute {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.InternalKeyword:
      case TerminalKind.ExternalKeyword:
      case TerminalKind.PrivateKeyword:
      case TerminalKind.PublicKeyword:
      case TerminalKind.ConstantKeyword:
      case TerminalKind.PureKeyword:
      case TerminalKind.ViewKeyword:
      case TerminalKind.PayableKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FunctionTypeAttribute);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class MappingKeyType {
  private readonly fetch: () => ElementaryType | IdentifierPath = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.ElementaryType:
        return new ElementaryType(variant as NonTerminalNode);
      case NonTerminalKind.IdentifierPath:
        return new IdentifierPath(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.MappingKeyType);
  }

  public get variant(): ElementaryType | IdentifierPath {
    return this.fetch();
  }
}

export class ElementaryType {
  private readonly fetch: () => AddressType | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.AddressType:
        return new AddressType(variant as NonTerminalNode);

      case TerminalKind.BoolKeyword:
      case TerminalKind.ByteKeyword:
      case TerminalKind.StringKeyword:
      case TerminalKind.BytesKeyword:
      case TerminalKind.IntKeyword:
      case TerminalKind.UintKeyword:
      case TerminalKind.FixedKeyword:
      case TerminalKind.UfixedKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ElementaryType);
  }

  public get variant(): AddressType | TerminalNode {
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
      case NonTerminalKind.ExpressionStatement:
        return new ExpressionStatement(variant as NonTerminalNode);
      case NonTerminalKind.VariableDeclarationStatement:
        return new VariableDeclarationStatement(variant as NonTerminalNode);
      case NonTerminalKind.TupleDeconstructionStatement:
        return new TupleDeconstructionStatement(variant as NonTerminalNode);
      case NonTerminalKind.IfStatement:
        return new IfStatement(variant as NonTerminalNode);
      case NonTerminalKind.ForStatement:
        return new ForStatement(variant as NonTerminalNode);
      case NonTerminalKind.WhileStatement:
        return new WhileStatement(variant as NonTerminalNode);
      case NonTerminalKind.DoWhileStatement:
        return new DoWhileStatement(variant as NonTerminalNode);
      case NonTerminalKind.ContinueStatement:
        return new ContinueStatement(variant as NonTerminalNode);
      case NonTerminalKind.BreakStatement:
        return new BreakStatement(variant as NonTerminalNode);
      case NonTerminalKind.ReturnStatement:
        return new ReturnStatement(variant as NonTerminalNode);
      case NonTerminalKind.ThrowStatement:
        return new ThrowStatement(variant as NonTerminalNode);
      case NonTerminalKind.EmitStatement:
        return new EmitStatement(variant as NonTerminalNode);
      case NonTerminalKind.TryStatement:
        return new TryStatement(variant as NonTerminalNode);
      case NonTerminalKind.RevertStatement:
        return new RevertStatement(variant as NonTerminalNode);
      case NonTerminalKind.AssemblyStatement:
        return new AssemblyStatement(variant as NonTerminalNode);
      case NonTerminalKind.Block:
        return new Block(variant as NonTerminalNode);
      case NonTerminalKind.UncheckedBlock:
        return new UncheckedBlock(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Statement);
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
      case NonTerminalKind.TypedTupleMember:
        return new TypedTupleMember(variant as NonTerminalNode);
      case NonTerminalKind.UntypedTupleMember:
        return new UntypedTupleMember(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TupleMember);
  }

  public get variant(): TypedTupleMember | UntypedTupleMember {
    return this.fetch();
  }
}

export class VariableDeclarationType {
  private readonly fetch: () => TypeName | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.TypeName:
        return new TypeName(variant as NonTerminalNode);

      case TerminalKind.VarKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VariableDeclarationType);
  }

  public get variant(): TypeName | TerminalNode {
    return this.fetch();
  }
}

export class StorageLocation {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.MemoryKeyword:
      case TerminalKind.StorageKeyword:
      case TerminalKind.CallDataKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StorageLocation);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class ForStatementInitialization {
  private readonly fetch: () =>
    | ExpressionStatement
    | VariableDeclarationStatement
    | TupleDeconstructionStatement
    | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.ExpressionStatement:
        return new ExpressionStatement(variant as NonTerminalNode);
      case NonTerminalKind.VariableDeclarationStatement:
        return new VariableDeclarationStatement(variant as NonTerminalNode);
      case NonTerminalKind.TupleDeconstructionStatement:
        return new TupleDeconstructionStatement(variant as NonTerminalNode);

      case TerminalKind.Semicolon:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ForStatementInitialization);
  }

  public get variant():
    | ExpressionStatement
    | VariableDeclarationStatement
    | TupleDeconstructionStatement
    | TerminalNode {
    return this.fetch();
  }
}

export class ForStatementCondition {
  private readonly fetch: () => ExpressionStatement | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.ExpressionStatement:
        return new ExpressionStatement(variant as NonTerminalNode);

      case TerminalKind.Semicolon:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ForStatementCondition);
  }

  public get variant(): ExpressionStatement | TerminalNode {
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
    | CallOptionsExpression
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
    | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.AssignmentExpression:
        return new AssignmentExpression(variant as NonTerminalNode);
      case NonTerminalKind.ConditionalExpression:
        return new ConditionalExpression(variant as NonTerminalNode);
      case NonTerminalKind.OrExpression:
        return new OrExpression(variant as NonTerminalNode);
      case NonTerminalKind.AndExpression:
        return new AndExpression(variant as NonTerminalNode);
      case NonTerminalKind.EqualityExpression:
        return new EqualityExpression(variant as NonTerminalNode);
      case NonTerminalKind.ComparisonExpression:
        return new ComparisonExpression(variant as NonTerminalNode);
      case NonTerminalKind.BitwiseOrExpression:
        return new BitwiseOrExpression(variant as NonTerminalNode);
      case NonTerminalKind.BitwiseXorExpression:
        return new BitwiseXorExpression(variant as NonTerminalNode);
      case NonTerminalKind.BitwiseAndExpression:
        return new BitwiseAndExpression(variant as NonTerminalNode);
      case NonTerminalKind.ShiftExpression:
        return new ShiftExpression(variant as NonTerminalNode);
      case NonTerminalKind.AdditiveExpression:
        return new AdditiveExpression(variant as NonTerminalNode);
      case NonTerminalKind.MultiplicativeExpression:
        return new MultiplicativeExpression(variant as NonTerminalNode);
      case NonTerminalKind.ExponentiationExpression:
        return new ExponentiationExpression(variant as NonTerminalNode);
      case NonTerminalKind.PostfixExpression:
        return new PostfixExpression(variant as NonTerminalNode);
      case NonTerminalKind.PrefixExpression:
        return new PrefixExpression(variant as NonTerminalNode);
      case NonTerminalKind.FunctionCallExpression:
        return new FunctionCallExpression(variant as NonTerminalNode);
      case NonTerminalKind.CallOptionsExpression:
        return new CallOptionsExpression(variant as NonTerminalNode);
      case NonTerminalKind.MemberAccessExpression:
        return new MemberAccessExpression(variant as NonTerminalNode);
      case NonTerminalKind.IndexAccessExpression:
        return new IndexAccessExpression(variant as NonTerminalNode);
      case NonTerminalKind.NewExpression:
        return new NewExpression(variant as NonTerminalNode);
      case NonTerminalKind.TupleExpression:
        return new TupleExpression(variant as NonTerminalNode);
      case NonTerminalKind.TypeExpression:
        return new TypeExpression(variant as NonTerminalNode);
      case NonTerminalKind.ArrayExpression:
        return new ArrayExpression(variant as NonTerminalNode);
      case NonTerminalKind.HexNumberExpression:
        return new HexNumberExpression(variant as NonTerminalNode);
      case NonTerminalKind.DecimalNumberExpression:
        return new DecimalNumberExpression(variant as NonTerminalNode);
      case NonTerminalKind.StringExpression:
        return new StringExpression(variant as NonTerminalNode);
      case NonTerminalKind.ElementaryType:
        return new ElementaryType(variant as NonTerminalNode);

      case TerminalKind.PayableKeyword:
      case TerminalKind.TrueKeyword:
      case TerminalKind.FalseKeyword:
      case TerminalKind.Identifier:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Expression);
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
    | CallOptionsExpression
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
    | TerminalNode {
    return this.fetch();
  }
}

export class MemberAccess {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.Identifier:
      case TerminalKind.AddressKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.MemberAccess);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class ArgumentsDeclaration {
  private readonly fetch: () => PositionalArgumentsDeclaration | NamedArgumentsDeclaration = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.PositionalArgumentsDeclaration:
        return new PositionalArgumentsDeclaration(variant as NonTerminalNode);
      case NonTerminalKind.NamedArgumentsDeclaration:
        return new NamedArgumentsDeclaration(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ArgumentsDeclaration);
  }

  public get variant(): PositionalArgumentsDeclaration | NamedArgumentsDeclaration {
    return this.fetch();
  }
}

export class NumberUnit {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.WeiKeyword:
      case TerminalKind.GweiKeyword:
      case TerminalKind.SzaboKeyword:
      case TerminalKind.FinneyKeyword:
      case TerminalKind.EtherKeyword:
      case TerminalKind.SecondsKeyword:
      case TerminalKind.MinutesKeyword:
      case TerminalKind.HoursKeyword:
      case TerminalKind.DaysKeyword:
      case TerminalKind.WeeksKeyword:
      case TerminalKind.YearsKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.NumberUnit);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class StringExpression {
  private readonly fetch: () =>
    | StringLiteral
    | StringLiterals
    | HexStringLiteral
    | HexStringLiterals
    | UnicodeStringLiterals = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.StringLiteral:
        return new StringLiteral(variant as NonTerminalNode);
      case NonTerminalKind.StringLiterals:
        return new StringLiterals(variant as NonTerminalNode);
      case NonTerminalKind.HexStringLiteral:
        return new HexStringLiteral(variant as NonTerminalNode);
      case NonTerminalKind.HexStringLiterals:
        return new HexStringLiterals(variant as NonTerminalNode);
      case NonTerminalKind.UnicodeStringLiterals:
        return new UnicodeStringLiterals(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StringExpression);
  }

  public get variant(): StringLiteral | StringLiterals | HexStringLiteral | HexStringLiterals | UnicodeStringLiterals {
    return this.fetch();
  }
}

export class StringLiteral {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.SingleQuotedStringLiteral:
      case TerminalKind.DoubleQuotedStringLiteral:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StringLiteral);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class HexStringLiteral {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.SingleQuotedHexStringLiteral:
      case TerminalKind.DoubleQuotedHexStringLiteral:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.HexStringLiteral);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class UnicodeStringLiteral {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.SingleQuotedUnicodeStringLiteral:
      case TerminalKind.DoubleQuotedUnicodeStringLiteral:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UnicodeStringLiteral);
  }

  public get variant(): TerminalNode {
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
      case NonTerminalKind.YulBlock:
        return new YulBlock(variant as NonTerminalNode);
      case NonTerminalKind.YulFunctionDefinition:
        return new YulFunctionDefinition(variant as NonTerminalNode);
      case NonTerminalKind.YulVariableDeclarationStatement:
        return new YulVariableDeclarationStatement(variant as NonTerminalNode);
      case NonTerminalKind.YulAssignmentStatement:
        return new YulAssignmentStatement(variant as NonTerminalNode);
      case NonTerminalKind.YulIfStatement:
        return new YulIfStatement(variant as NonTerminalNode);
      case NonTerminalKind.YulForStatement:
        return new YulForStatement(variant as NonTerminalNode);
      case NonTerminalKind.YulSwitchStatement:
        return new YulSwitchStatement(variant as NonTerminalNode);
      case NonTerminalKind.YulLeaveStatement:
        return new YulLeaveStatement(variant as NonTerminalNode);
      case NonTerminalKind.YulBreakStatement:
        return new YulBreakStatement(variant as NonTerminalNode);
      case NonTerminalKind.YulContinueStatement:
        return new YulContinueStatement(variant as NonTerminalNode);
      case NonTerminalKind.YulLabel:
        return new YulLabel(variant as NonTerminalNode);
      case NonTerminalKind.YulExpression:
        return new YulExpression(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulStatement);
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

export class YulAssignmentOperator {
  private readonly fetch: () => YulColonAndEqual | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.YulColonAndEqual:
        return new YulColonAndEqual(variant as NonTerminalNode);

      case TerminalKind.ColonEqual:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulAssignmentOperator);
  }

  public get variant(): YulColonAndEqual | TerminalNode {
    return this.fetch();
  }
}

export class YulSwitchCase {
  private readonly fetch: () => YulDefaultCase | YulValueCase = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.YulDefaultCase:
        return new YulDefaultCase(variant as NonTerminalNode);
      case NonTerminalKind.YulValueCase:
        return new YulValueCase(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulSwitchCase);
  }

  public get variant(): YulDefaultCase | YulValueCase {
    return this.fetch();
  }
}

export class YulExpression {
  private readonly fetch: () => YulFunctionCallExpression | YulLiteral | YulBuiltInFunction | YulPath = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.YulFunctionCallExpression:
        return new YulFunctionCallExpression(variant as NonTerminalNode);
      case NonTerminalKind.YulLiteral:
        return new YulLiteral(variant as NonTerminalNode);
      case NonTerminalKind.YulBuiltInFunction:
        return new YulBuiltInFunction(variant as NonTerminalNode);
      case NonTerminalKind.YulPath:
        return new YulPath(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulExpression);
  }

  public get variant(): YulFunctionCallExpression | YulLiteral | YulBuiltInFunction | YulPath {
    return this.fetch();
  }
}

export class YulPathComponent {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.YulIdentifier:
      case TerminalKind.YulAddressKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulPathComponent);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class YulBuiltInFunction {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.YulAddKeyword:
      case TerminalKind.YulAddModKeyword:
      case TerminalKind.YulAddressKeyword:
      case TerminalKind.YulAndKeyword:
      case TerminalKind.YulBalanceKeyword:
      case TerminalKind.YulBlockHashKeyword:
      case TerminalKind.YulByteKeyword:
      case TerminalKind.YulCallCodeKeyword:
      case TerminalKind.YulCallDataCopyKeyword:
      case TerminalKind.YulCallDataLoadKeyword:
      case TerminalKind.YulCallDataSizeKeyword:
      case TerminalKind.YulCallerKeyword:
      case TerminalKind.YulCallKeyword:
      case TerminalKind.YulCallValueKeyword:
      case TerminalKind.YulCoinBaseKeyword:
      case TerminalKind.YulCreateKeyword:
      case TerminalKind.YulDelegateCallKeyword:
      case TerminalKind.YulDivKeyword:
      case TerminalKind.YulEqKeyword:
      case TerminalKind.YulExpKeyword:
      case TerminalKind.YulExtCodeCopyKeyword:
      case TerminalKind.YulExtCodeSizeKeyword:
      case TerminalKind.YulGasKeyword:
      case TerminalKind.YulGasLimitKeyword:
      case TerminalKind.YulGasPriceKeyword:
      case TerminalKind.YulGtKeyword:
      case TerminalKind.YulInvalidKeyword:
      case TerminalKind.YulIsZeroKeyword:
      case TerminalKind.YulLog0Keyword:
      case TerminalKind.YulLog1Keyword:
      case TerminalKind.YulLog2Keyword:
      case TerminalKind.YulLog3Keyword:
      case TerminalKind.YulLog4Keyword:
      case TerminalKind.YulLtKeyword:
      case TerminalKind.YulMLoadKeyword:
      case TerminalKind.YulModKeyword:
      case TerminalKind.YulMSizeKeyword:
      case TerminalKind.YulMStore8Keyword:
      case TerminalKind.YulMStoreKeyword:
      case TerminalKind.YulMulKeyword:
      case TerminalKind.YulMulModKeyword:
      case TerminalKind.YulNotKeyword:
      case TerminalKind.YulNumberKeyword:
      case TerminalKind.YulOriginKeyword:
      case TerminalKind.YulOrKeyword:
      case TerminalKind.YulPopKeyword:
      case TerminalKind.YulReturnKeyword:
      case TerminalKind.YulRevertKeyword:
      case TerminalKind.YulSDivKeyword:
      case TerminalKind.YulSelfDestructKeyword:
      case TerminalKind.YulSgtKeyword:
      case TerminalKind.YulSignExtendKeyword:
      case TerminalKind.YulSLoadKeyword:
      case TerminalKind.YulSltKeyword:
      case TerminalKind.YulSModKeyword:
      case TerminalKind.YulSStoreKeyword:
      case TerminalKind.YulStopKeyword:
      case TerminalKind.YulSubKeyword:
      case TerminalKind.YulTimestampKeyword:
      case TerminalKind.YulXorKeyword:
      case TerminalKind.YulKeccak256Keyword:
      case TerminalKind.YulSha3Keyword:
      case TerminalKind.YulSuicideKeyword:
      case TerminalKind.YulReturnDataCopyKeyword:
      case TerminalKind.YulReturnDataSizeKeyword:
      case TerminalKind.YulStaticCallKeyword:
      case TerminalKind.YulCreate2Keyword:
      case TerminalKind.YulExtCodeHashKeyword:
      case TerminalKind.YulSarKeyword:
      case TerminalKind.YulShlKeyword:
      case TerminalKind.YulShrKeyword:
      case TerminalKind.YulChainIdKeyword:
      case TerminalKind.YulSelfBalanceKeyword:
      case TerminalKind.YulBaseFeeKeyword:
      case TerminalKind.YulDifficultyKeyword:
      case TerminalKind.YulPrevRandaoKeyword:
      case TerminalKind.YulBlobBaseFeeKeyword:
      case TerminalKind.YulBlobHashKeyword:
      case TerminalKind.YulTLoadKeyword:
      case TerminalKind.YulTStoreKeyword:
      case TerminalKind.YulMCopyKeyword:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulBuiltInFunction);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class YulLiteral {
  private readonly fetch: () => HexStringLiteral | StringLiteral | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.HexStringLiteral:
        return new HexStringLiteral(variant as NonTerminalNode);
      case NonTerminalKind.StringLiteral:
        return new StringLiteral(variant as NonTerminalNode);

      case TerminalKind.YulTrueKeyword:
      case TerminalKind.YulFalseKeyword:
      case TerminalKind.YulDecimalLiteral:
      case TerminalKind.YulHexLiteral:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulLiteral);
  }

  public get variant(): HexStringLiteral | StringLiteral | TerminalNode {
    return this.fetch();
  }
}

/*
 * Repeated:
 */

export class SourceUnitMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new SourceUnitMember(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.SourceUnitMembers);
  }

  public get items(): readonly SourceUnitMember[] {
    return this.fetch();
  }
}

export class VersionExpressionSet {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new VersionExpression(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VersionExpressionSet);
  }

  public get items(): readonly VersionExpression[] {
    return this.fetch();
  }
}

export class ContractMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ContractMember(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ContractMembers);
  }

  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

export class InterfaceMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ContractMember(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.InterfaceMembers);
  }

  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

export class LibraryMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ContractMember(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.LibraryMembers);
  }

  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

export class StructMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new StructMember(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StructMembers);
  }

  public get items(): readonly StructMember[] {
    return this.fetch();
  }
}

export class StateVariableAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new StateVariableAttribute(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StateVariableAttributes);
  }

  public get items(): readonly StateVariableAttribute[] {
    return this.fetch();
  }
}

export class FunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new FunctionAttribute(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FunctionAttributes);
  }

  public get items(): readonly FunctionAttribute[] {
    return this.fetch();
  }
}

export class ConstructorAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ConstructorAttribute(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ConstructorAttributes);
  }

  public get items(): readonly ConstructorAttribute[] {
    return this.fetch();
  }
}

export class UnnamedFunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new UnnamedFunctionAttribute(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UnnamedFunctionAttributes);
  }

  public get items(): readonly UnnamedFunctionAttribute[] {
    return this.fetch();
  }
}

export class FallbackFunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new FallbackFunctionAttribute(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FallbackFunctionAttributes);
  }

  public get items(): readonly FallbackFunctionAttribute[] {
    return this.fetch();
  }
}

export class ReceiveFunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ReceiveFunctionAttribute(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ReceiveFunctionAttributes);
  }

  public get items(): readonly ReceiveFunctionAttribute[] {
    return this.fetch();
  }
}

export class ModifierAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ModifierAttribute(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ModifierAttributes);
  }

  public get items(): readonly ModifierAttribute[] {
    return this.fetch();
  }
}

export class FunctionTypeAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new FunctionTypeAttribute(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.FunctionTypeAttributes);
  }

  public get items(): readonly FunctionTypeAttribute[] {
    return this.fetch();
  }
}

export class Statements {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new Statement(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Statements);
  }

  public get items(): readonly Statement[] {
    return this.fetch();
  }
}

export class CatchClauses {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new CatchClause(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.CatchClauses);
  }

  public get items(): readonly CatchClause[] {
    return this.fetch();
  }
}

export class StringLiterals {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new StringLiteral(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.StringLiterals);
  }

  public get items(): readonly StringLiteral[] {
    return this.fetch();
  }
}

export class HexStringLiterals {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new HexStringLiteral(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.HexStringLiterals);
  }

  public get items(): readonly HexStringLiteral[] {
    return this.fetch();
  }
}

export class UnicodeStringLiterals {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new UnicodeStringLiteral(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UnicodeStringLiterals);
  }

  public get items(): readonly UnicodeStringLiteral[] {
    return this.fetch();
  }
}

export class YulStatements {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new YulStatement(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulStatements);
  }

  public get items(): readonly YulStatement[] {
    return this.fetch();
  }
}

export class YulSwitchCases {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new YulSwitchCase(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulSwitchCases);
  }

  public get items(): readonly YulSwitchCase[] {
    return this.fetch();
  }
}

/*
 * Separated:
 */

export class VersionExpressionSets {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new VersionExpressionSet(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VersionExpressionSets);
  }

  public get items(): readonly VersionExpressionSet[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class VersionSpecifiers {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.VersionSpecifiers);
  }

  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class ImportDeconstructionSymbols {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new ImportDeconstructionSymbol(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ImportDeconstructionSymbols);
  }

  public get items(): readonly ImportDeconstructionSymbol[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class UsingDeconstructionSymbols {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new UsingDeconstructionSymbol(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.UsingDeconstructionSymbols);
  }

  public get items(): readonly UsingDeconstructionSymbol[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class InheritanceTypes {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new InheritanceType(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.InheritanceTypes);
  }

  public get items(): readonly InheritanceType[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class EnumMembers {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.EnumMembers);
  }

  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class Parameters {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new Parameter(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Parameters);
  }

  public get items(): readonly Parameter[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class OverridePaths {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new IdentifierPath(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.OverridePaths);
  }

  public get items(): readonly IdentifierPath[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class EventParameters {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new EventParameter(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.EventParameters);
  }

  public get items(): readonly EventParameter[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class ErrorParameters {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new ErrorParameter(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ErrorParameters);
  }

  public get items(): readonly ErrorParameter[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class AssemblyFlags {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new StringLiteral(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.AssemblyFlags);
  }

  public get items(): readonly StringLiteral[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class TupleDeconstructionElements {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new TupleDeconstructionElement(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TupleDeconstructionElements);
  }

  public get items(): readonly TupleDeconstructionElement[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class PositionalArguments {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new Expression(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.PositionalArguments);
  }

  public get items(): readonly Expression[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class NamedArguments {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new NamedArgument(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.NamedArguments);
  }

  public get items(): readonly NamedArgument[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class CallOptions {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new NamedArgument(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.CallOptions);
  }

  public get items(): readonly NamedArgument[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class TupleValues {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new TupleValue(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TupleValues);
  }

  public get items(): readonly TupleValue[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class ArrayValues {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new Expression(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.ArrayValues);
  }

  public get items(): readonly Expression[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class IdentifierPath {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.IdentifierPath);
  }

  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class YulParameters {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulParameters);
  }

  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class YulReturnVariables {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulReturnVariables);
  }

  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class YulArguments {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new YulExpression(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulArguments);
  }

  public get items(): readonly YulExpression[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class YulPaths {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new YulPath(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulPaths);
  }

  public get items(): readonly YulPath[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class YulPath {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return {
      items: items.map((item) => new YulPathComponent(item as NonTerminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.YulPath);
  }

  public get items(): readonly YulPathComponent[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
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

function assertKind(actual: NonTerminalKind, expected: NonTerminalKind): void {
  assert.equal(actual, expected, `${expected} can only be initialized with a CST node of the same kind.`);
}
