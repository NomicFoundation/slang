// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import * as assert from "node:assert";
import { ast_internal } from "../../napi-bindings/generated";
import { NodeType, NonterminalNode, TerminalNode } from "../../cst";
import { NonterminalKind } from "../../kinds";

/*
 * Sequences:
 */

export class SourceUnit {
  private readonly fetch = once(() => {
    const [$members] = ast_internal.selectSequence(this.cst);

    return {
      members: new SourceUnitMembers($members as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.SourceUnit);
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
      pragma: new Pragma($pragma as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.PragmaDirective);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ABICoderPragma);
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
      feature: new ExperimentalFeature($feature as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ExperimentalPragma);
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
      sets: new VersionExpressionSets($sets as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VersionPragma);
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
      leftOperand: new VersionExpression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new VersionExpression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VersionRange);
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
      operand: new VersionExpression($operand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VersionComparator);
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
      clause: new ImportClause($clause as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ImportDirective);
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
      path: new StringLiteral($path as NonterminalNode),
      alias: $alias === null ? undefined : new ImportAlias($alias as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.PathImport);
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
      alias: new ImportAlias($alias as NonterminalNode),
      fromKeyword: $fromKeyword as TerminalNode,
      path: new StringLiteral($path as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.NamedImport);
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
      symbols: new ImportDeconstructionSymbols($symbols as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
      fromKeyword: $fromKeyword as TerminalNode,
      path: new StringLiteral($path as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ImportDeconstruction);
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
      alias: $alias === null ? undefined : new ImportAlias($alias as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ImportDeconstructionSymbol);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ImportAlias);
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
      clause: new UsingClause($clause as NonterminalNode),
      forKeyword: $forKeyword as TerminalNode,
      target: new UsingTarget($target as NonterminalNode),
      globalKeyword: $globalKeyword === null ? undefined : ($globalKeyword as TerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UsingDirective);
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
      symbols: new UsingDeconstructionSymbols($symbols as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UsingDeconstruction);
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
      name: new IdentifierPath($name as NonterminalNode),
      alias: $alias === null ? undefined : new UsingAlias($alias as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UsingDeconstructionSymbol);
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
      operator: new UsingOperator($operator as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UsingAlias);
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
    const [$abstractKeyword, $contractKeyword, $name, $inheritance, $openBrace, $members, $closeBrace] =
      ast_internal.selectSequence(this.cst);

    return {
      abstractKeyword: $abstractKeyword === null ? undefined : ($abstractKeyword as TerminalNode),
      contractKeyword: $contractKeyword as TerminalNode,
      name: $name as TerminalNode,
      inheritance: $inheritance === null ? undefined : new InheritanceSpecifier($inheritance as NonterminalNode),
      openBrace: $openBrace as TerminalNode,
      members: new ContractMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ContractDefinition);
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

  public get inheritance(): InheritanceSpecifier | undefined {
    return this.fetch().inheritance;
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
      types: new InheritanceTypes($types as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.InheritanceSpecifier);
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
      typeName: new IdentifierPath($typeName as NonterminalNode),
      arguments: $arguments === null ? undefined : new ArgumentsDeclaration($arguments as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.InheritanceType);
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
    const [$interfaceKeyword, $name, $inheritance, $openBrace, $members, $closeBrace] = ast_internal.selectSequence(
      this.cst,
    );

    return {
      interfaceKeyword: $interfaceKeyword as TerminalNode,
      name: $name as TerminalNode,
      inheritance: $inheritance === null ? undefined : new InheritanceSpecifier($inheritance as NonterminalNode),
      openBrace: $openBrace as TerminalNode,
      members: new InterfaceMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.InterfaceDefinition);
  }

  public get interfaceKeyword(): TerminalNode {
    return this.fetch().interfaceKeyword;
  }

  public get name(): TerminalNode {
    return this.fetch().name;
  }

  public get inheritance(): InheritanceSpecifier | undefined {
    return this.fetch().inheritance;
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
      members: new LibraryMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.LibraryDefinition);
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
      members: new StructMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StructDefinition);
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
      typeName: new TypeName($typeName as NonterminalNode),
      name: $name as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StructMember);
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
      members: new EnumMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.EnumDefinition);
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
      typeName: new TypeName($typeName as NonterminalNode),
      constantKeyword: $constantKeyword as TerminalNode,
      name: $name as TerminalNode,
      equal: $equal as TerminalNode,
      value: new Expression($value as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ConstantDefinition);
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
      typeName: new TypeName($typeName as NonterminalNode),
      attributes: new StateVariableAttributes($attributes as NonterminalNode),
      name: $name as TerminalNode,
      value: $value === null ? undefined : new StateVariableDefinitionValue($value as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StateVariableDefinition);
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
      value: new Expression($value as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StateVariableDefinitionValue);
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
      name: new FunctionName($name as NonterminalNode),
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new FunctionAttributes($attributes as NonterminalNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FunctionDefinition);
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
      parameters: new Parameters($parameters as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ParametersDeclaration);
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
      typeName: new TypeName($typeName as NonterminalNode),
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as NonterminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.Parameter);
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
      overridden: $overridden === null ? undefined : new OverridePathsDeclaration($overridden as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.OverrideSpecifier);
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
      paths: new OverridePaths($paths as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.OverridePathsDeclaration);
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
      variables: new ParametersDeclaration($variables as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ReturnsDeclaration);
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
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new ConstructorAttributes($attributes as NonterminalNode),
      body: new Block($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ConstructorDefinition);
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
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new UnnamedFunctionAttributes($attributes as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UnnamedFunctionDefinition);
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
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new FallbackFunctionAttributes($attributes as NonterminalNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FallbackFunctionDefinition);
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
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new ReceiveFunctionAttributes($attributes as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ReceiveFunctionDefinition);
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
      parameters: $parameters === null ? undefined : new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new ModifierAttributes($attributes as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ModifierDefinition);
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
      name: new IdentifierPath($name as NonterminalNode),
      arguments: $arguments === null ? undefined : new ArgumentsDeclaration($arguments as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ModifierInvocation);
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
      parameters: new EventParametersDeclaration($parameters as NonterminalNode),
      anonymousKeyword: $anonymousKeyword === null ? undefined : ($anonymousKeyword as TerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.EventDefinition);
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
      parameters: new EventParameters($parameters as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.EventParametersDeclaration);
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
      typeName: new TypeName($typeName as NonterminalNode),
      indexedKeyword: $indexedKeyword === null ? undefined : ($indexedKeyword as TerminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.EventParameter);
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
      valueType: new ElementaryType($valueType as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UserDefinedValueTypeDefinition);
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
      members: new ErrorParametersDeclaration($members as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ErrorDefinition);
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
      parameters: new ErrorParameters($parameters as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ErrorParametersDeclaration);
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
      typeName: new TypeName($typeName as NonterminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ErrorParameter);
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
      operand: new TypeName($operand as NonterminalNode),
      openBracket: $openBracket as TerminalNode,
      index: $index === null ? undefined : new Expression($index as NonterminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ArrayTypeName);
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
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new FunctionTypeAttributes($attributes as NonterminalNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FunctionType);
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
      keyType: new MappingKey($keyType as NonterminalNode),
      equalGreaterThan: $equalGreaterThan as TerminalNode,
      valueType: new MappingValue($valueType as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.MappingType);
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
      keyType: new MappingKeyType($keyType as NonterminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.MappingKey);
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
      typeName: new TypeName($typeName as NonterminalNode),
      name: $name === null ? undefined : ($name as TerminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.MappingValue);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.AddressType);
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
      statements: new Statements($statements as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.Block);
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
      block: new Block($block as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UncheckedBlock);
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
      expression: new Expression($expression as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ExpressionStatement);
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
      label: $label === null ? undefined : new StringLiteral($label as NonterminalNode),
      flags: $flags === null ? undefined : new AssemblyFlagsDeclaration($flags as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.AssemblyStatement);
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
      flags: new AssemblyFlags($flags as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.AssemblyFlagsDeclaration);
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
      elements: new TupleDeconstructionElements($elements as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
      equal: $equal as TerminalNode,
      expression: new Expression($expression as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TupleDeconstructionStatement);
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
      member: $member === null ? undefined : new TupleMember($member as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TupleDeconstructionElement);
  }

  public get member(): TupleMember | undefined {
    return this.fetch().member;
  }
}

export class TypedTupleMember {
  private readonly fetch = once(() => {
    const [$typeName, $storageLocation, $name] = ast_internal.selectSequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonterminalNode),
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as NonterminalNode),
      name: $name as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TypedTupleMember);
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
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as NonterminalNode),
      name: $name as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UntypedTupleMember);
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
      variableType: new VariableDeclarationType($variableType as NonterminalNode),
      storageLocation: $storageLocation === null ? undefined : new StorageLocation($storageLocation as NonterminalNode),
      name: $name as TerminalNode,
      value: $value === null ? undefined : new VariableDeclarationValue($value as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VariableDeclarationStatement);
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
      expression: new Expression($expression as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VariableDeclarationValue);
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
      condition: new Expression($condition as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
      body: new Statement($body as NonterminalNode),
      elseBranch: $elseBranch === null ? undefined : new ElseBranch($elseBranch as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.IfStatement);
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
      body: new Statement($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ElseBranch);
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
      initialization: new ForStatementInitialization($initialization as NonterminalNode),
      condition: new ForStatementCondition($condition as NonterminalNode),
      iterator: $iterator === null ? undefined : new Expression($iterator as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
      body: new Statement($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ForStatement);
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
      condition: new Expression($condition as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
      body: new Statement($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.WhileStatement);
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
      body: new Statement($body as NonterminalNode),
      whileKeyword: $whileKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      condition: new Expression($condition as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.DoWhileStatement);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ContinueStatement);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.BreakStatement);
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
      expression: $expression === null ? undefined : new Expression($expression as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ReturnStatement);
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
      event: new IdentifierPath($event as NonterminalNode),
      arguments: new ArgumentsDeclaration($arguments as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.EmitStatement);
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
      expression: new Expression($expression as NonterminalNode),
      returns: $returns === null ? undefined : new ReturnsDeclaration($returns as NonterminalNode),
      body: new Block($body as NonterminalNode),
      catchClauses: new CatchClauses($catchClauses as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TryStatement);
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
      error: $error === null ? undefined : new CatchClauseError($error as NonterminalNode),
      body: new Block($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.CatchClause);
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
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.CatchClauseError);
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
      error: $error === null ? undefined : new IdentifierPath($error as NonterminalNode),
      arguments: new ArgumentsDeclaration($arguments as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.RevertStatement);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ThrowStatement);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.AssignmentExpression);
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
      operand: new Expression($operand as NonterminalNode),
      questionMark: $questionMark as TerminalNode,
      trueExpression: new Expression($trueExpression as NonterminalNode),
      colon: $colon as TerminalNode,
      falseExpression: new Expression($falseExpression as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ConditionalExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.OrExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.AndExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.EqualityExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ComparisonExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.BitwiseOrExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.BitwiseXorExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.BitwiseAndExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ShiftExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.AdditiveExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.MultiplicativeExpression);
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
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ExponentiationExpression);
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
      operand: new Expression($operand as NonterminalNode),
      operator: $operator as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.PostfixExpression);
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
      operand: new Expression($operand as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.PrefixExpression);
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
      operand: new Expression($operand as NonterminalNode),
      arguments: new ArgumentsDeclaration($arguments as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FunctionCallExpression);
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
      operand: new Expression($operand as NonterminalNode),
      openBrace: $openBrace as TerminalNode,
      options: new CallOptions($options as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.CallOptionsExpression);
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
      operand: new Expression($operand as NonterminalNode),
      period: $period as TerminalNode,
      member: $member as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.MemberAccessExpression);
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }

  public get period(): TerminalNode {
    return this.fetch().period;
  }

  public get member(): TerminalNode {
    return this.fetch().member;
  }
}

export class IndexAccessExpression {
  private readonly fetch = once(() => {
    const [$operand, $openBracket, $start, $end, $closeBracket] = ast_internal.selectSequence(this.cst);

    return {
      operand: new Expression($operand as NonterminalNode),
      openBracket: $openBracket as TerminalNode,
      start: $start === null ? undefined : new Expression($start as NonterminalNode),
      end: $end === null ? undefined : new IndexAccessEnd($end as NonterminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.IndexAccessExpression);
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
      end: $end === null ? undefined : new Expression($end as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.IndexAccessEnd);
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
      arguments: new PositionalArguments($arguments as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.PositionalArgumentsDeclaration);
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
      arguments: $arguments === null ? undefined : new NamedArgumentGroup($arguments as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.NamedArgumentsDeclaration);
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
      arguments: new NamedArguments($arguments as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.NamedArgumentGroup);
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
      value: new Expression($value as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.NamedArgument);
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
      typeName: new TypeName($typeName as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TypeExpression);
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
      typeName: new TypeName($typeName as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.NewExpression);
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
      items: new TupleValues($items as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TupleExpression);
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
      expression: $expression === null ? undefined : new Expression($expression as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TupleValue);
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
      items: new ArrayValues($items as NonterminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ArrayExpression);
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
      unit: $unit === null ? undefined : new NumberUnit($unit as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.HexNumberExpression);
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
      unit: $unit === null ? undefined : new NumberUnit($unit as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.DecimalNumberExpression);
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
      statements: new YulStatements($statements as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulBlock);
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
      parameters: new YulParametersDeclaration($parameters as NonterminalNode),
      returns: $returns === null ? undefined : new YulReturnsDeclaration($returns as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulFunctionDefinition);
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
      parameters: new YulParameters($parameters as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulParametersDeclaration);
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
      variables: new YulVariableNames($variables as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulReturnsDeclaration);
  }

  public get minusGreaterThan(): TerminalNode {
    return this.fetch().minusGreaterThan;
  }

  public get variables(): YulVariableNames {
    return this.fetch().variables;
  }
}

export class YulVariableDeclarationStatement {
  private readonly fetch = once(() => {
    const [$letKeyword, $variables, $value] = ast_internal.selectSequence(this.cst);

    return {
      letKeyword: $letKeyword as TerminalNode,
      variables: new YulVariableNames($variables as NonterminalNode),
      value: $value === null ? undefined : new YulVariableDeclarationValue($value as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulVariableDeclarationStatement);
  }

  public get letKeyword(): TerminalNode {
    return this.fetch().letKeyword;
  }

  public get variables(): YulVariableNames {
    return this.fetch().variables;
  }

  public get value(): YulVariableDeclarationValue | undefined {
    return this.fetch().value;
  }
}

export class YulVariableDeclarationValue {
  private readonly fetch = once(() => {
    const [$assignment, $expression] = ast_internal.selectSequence(this.cst);

    return {
      assignment: new YulAssignmentOperator($assignment as NonterminalNode),
      expression: new YulExpression($expression as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulVariableDeclarationValue);
  }

  public get assignment(): YulAssignmentOperator {
    return this.fetch().assignment;
  }

  public get expression(): YulExpression {
    return this.fetch().expression;
  }
}

export class YulVariableAssignmentStatement {
  private readonly fetch = once(() => {
    const [$variables, $assignment, $expression] = ast_internal.selectSequence(this.cst);

    return {
      variables: new YulPaths($variables as NonterminalNode),
      assignment: new YulAssignmentOperator($assignment as NonterminalNode),
      expression: new YulExpression($expression as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulVariableAssignmentStatement);
  }

  public get variables(): YulPaths {
    return this.fetch().variables;
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulColonAndEqual);
  }

  public get colon(): TerminalNode {
    return this.fetch().colon;
  }

  public get equal(): TerminalNode {
    return this.fetch().equal;
  }
}

export class YulStackAssignmentStatement {
  private readonly fetch = once(() => {
    const [$assignment, $variable] = ast_internal.selectSequence(this.cst);

    return {
      assignment: new YulStackAssignmentOperator($assignment as NonterminalNode),
      variable: $variable as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulStackAssignmentStatement);
  }

  public get assignment(): YulStackAssignmentOperator {
    return this.fetch().assignment;
  }

  public get variable(): TerminalNode {
    return this.fetch().variable;
  }
}

export class YulEqualAndColon {
  private readonly fetch = once(() => {
    const [$equal, $colon] = ast_internal.selectSequence(this.cst);

    return {
      equal: $equal as TerminalNode,
      colon: $colon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulEqualAndColon);
  }

  public get equal(): TerminalNode {
    return this.fetch().equal;
  }

  public get colon(): TerminalNode {
    return this.fetch().colon;
  }
}

export class YulIfStatement {
  private readonly fetch = once(() => {
    const [$ifKeyword, $condition, $body] = ast_internal.selectSequence(this.cst);

    return {
      ifKeyword: $ifKeyword as TerminalNode,
      condition: new YulExpression($condition as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulIfStatement);
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
      initialization: new YulBlock($initialization as NonterminalNode),
      condition: new YulExpression($condition as NonterminalNode),
      iterator: new YulBlock($iterator as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulForStatement);
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
      expression: new YulExpression($expression as NonterminalNode),
      cases: new YulSwitchCases($cases as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulSwitchStatement);
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
      body: new YulBlock($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulDefaultCase);
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
      value: new YulLiteral($value as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulValueCase);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulLeaveStatement);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulBreakStatement);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulContinueStatement);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulLabel);
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
      operand: new YulExpression($operand as NonterminalNode),
      openParen: $openParen as TerminalNode,
      arguments: new YulArguments($arguments as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulFunctionCallExpression);
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
    | ErrorDefinition
    | UserDefinedValueTypeDefinition
    | UsingDirective
    | EventDefinition
    | ConstantDefinition = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.PragmaDirective:
        return new PragmaDirective(variant as NonterminalNode);
      case NonterminalKind.ImportDirective:
        return new ImportDirective(variant as NonterminalNode);
      case NonterminalKind.ContractDefinition:
        return new ContractDefinition(variant as NonterminalNode);
      case NonterminalKind.InterfaceDefinition:
        return new InterfaceDefinition(variant as NonterminalNode);
      case NonterminalKind.LibraryDefinition:
        return new LibraryDefinition(variant as NonterminalNode);
      case NonterminalKind.StructDefinition:
        return new StructDefinition(variant as NonterminalNode);
      case NonterminalKind.EnumDefinition:
        return new EnumDefinition(variant as NonterminalNode);
      case NonterminalKind.FunctionDefinition:
        return new FunctionDefinition(variant as NonterminalNode);
      case NonterminalKind.ErrorDefinition:
        return new ErrorDefinition(variant as NonterminalNode);
      case NonterminalKind.UserDefinedValueTypeDefinition:
        return new UserDefinedValueTypeDefinition(variant as NonterminalNode);
      case NonterminalKind.UsingDirective:
        return new UsingDirective(variant as NonterminalNode);
      case NonterminalKind.EventDefinition:
        return new EventDefinition(variant as NonterminalNode);
      case NonterminalKind.ConstantDefinition:
        return new ConstantDefinition(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.SourceUnitMember);
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
    | ErrorDefinition
    | UserDefinedValueTypeDefinition
    | UsingDirective
    | EventDefinition
    | ConstantDefinition {
    return this.fetch();
  }
}

export class Pragma {
  private readonly fetch: () => ABICoderPragma | ExperimentalPragma | VersionPragma = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.ABICoderPragma:
        return new ABICoderPragma(variant as NonterminalNode);
      case NonterminalKind.ExperimentalPragma:
        return new ExperimentalPragma(variant as NonterminalNode);
      case NonterminalKind.VersionPragma:
        return new VersionPragma(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.Pragma);
  }

  public get variant(): ABICoderPragma | ExperimentalPragma | VersionPragma {
    return this.fetch();
  }
}

export class ExperimentalFeature {
  private readonly fetch: () => StringLiteral | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.StringLiteral:
        return new StringLiteral(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ExperimentalFeature);
  }

  public get variant(): StringLiteral | TerminalNode {
    return this.fetch();
  }
}

export class VersionExpression {
  private readonly fetch: () => VersionRange | VersionComparator | VersionSpecifiers | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.VersionRange:
        return new VersionRange(variant as NonterminalNode);
      case NonterminalKind.VersionComparator:
        return new VersionComparator(variant as NonterminalNode);
      case NonterminalKind.VersionSpecifiers:
        return new VersionSpecifiers(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VersionExpression);
  }

  public get variant(): VersionRange | VersionComparator | VersionSpecifiers | TerminalNode {
    return this.fetch();
  }
}

export class ImportClause {
  private readonly fetch: () => PathImport | NamedImport | ImportDeconstruction = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.PathImport:
        return new PathImport(variant as NonterminalNode);
      case NonterminalKind.NamedImport:
        return new NamedImport(variant as NonterminalNode);
      case NonterminalKind.ImportDeconstruction:
        return new ImportDeconstruction(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ImportClause);
  }

  public get variant(): PathImport | NamedImport | ImportDeconstruction {
    return this.fetch();
  }
}

export class UsingClause {
  private readonly fetch: () => IdentifierPath | UsingDeconstruction = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.IdentifierPath:
        return new IdentifierPath(variant as NonterminalNode);
      case NonterminalKind.UsingDeconstruction:
        return new UsingDeconstruction(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UsingClause);
  }

  public get variant(): IdentifierPath | UsingDeconstruction {
    return this.fetch();
  }
}

export class UsingOperator {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UsingOperator);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class UsingTarget {
  private readonly fetch: () => TypeName | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.TypeName:
        return new TypeName(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UsingTarget);
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
    | ErrorDefinition
    | UserDefinedValueTypeDefinition
    | StateVariableDefinition = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.UsingDirective:
        return new UsingDirective(variant as NonterminalNode);
      case NonterminalKind.FunctionDefinition:
        return new FunctionDefinition(variant as NonterminalNode);
      case NonterminalKind.ConstructorDefinition:
        return new ConstructorDefinition(variant as NonterminalNode);
      case NonterminalKind.ReceiveFunctionDefinition:
        return new ReceiveFunctionDefinition(variant as NonterminalNode);
      case NonterminalKind.FallbackFunctionDefinition:
        return new FallbackFunctionDefinition(variant as NonterminalNode);
      case NonterminalKind.UnnamedFunctionDefinition:
        return new UnnamedFunctionDefinition(variant as NonterminalNode);
      case NonterminalKind.ModifierDefinition:
        return new ModifierDefinition(variant as NonterminalNode);
      case NonterminalKind.StructDefinition:
        return new StructDefinition(variant as NonterminalNode);
      case NonterminalKind.EnumDefinition:
        return new EnumDefinition(variant as NonterminalNode);
      case NonterminalKind.EventDefinition:
        return new EventDefinition(variant as NonterminalNode);
      case NonterminalKind.ErrorDefinition:
        return new ErrorDefinition(variant as NonterminalNode);
      case NonterminalKind.UserDefinedValueTypeDefinition:
        return new UserDefinedValueTypeDefinition(variant as NonterminalNode);
      case NonterminalKind.StateVariableDefinition:
        return new StateVariableDefinition(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ContractMember);
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
    | ErrorDefinition
    | UserDefinedValueTypeDefinition
    | StateVariableDefinition {
    return this.fetch();
  }
}

export class StateVariableAttribute {
  private readonly fetch: () => OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StateVariableAttribute);
  }

  public get variant(): OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

export class FunctionName {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FunctionName);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class FunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

export class FunctionBody {
  private readonly fetch: () => Block | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.Block:
        return new Block(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FunctionBody);
  }

  public get variant(): Block | TerminalNode {
    return this.fetch();
  }
}

export class ConstructorAttribute {
  private readonly fetch: () => ModifierInvocation | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ConstructorAttribute);
  }

  public get variant(): ModifierInvocation | TerminalNode {
    return this.fetch();
  }
}

export class UnnamedFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UnnamedFunctionAttribute);
  }

  public get variant(): ModifierInvocation | TerminalNode {
    return this.fetch();
  }
}

export class FallbackFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FallbackFunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

export class ReceiveFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ReceiveFunctionAttribute);
  }

  public get variant(): ModifierInvocation | OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

export class ModifierAttribute {
  private readonly fetch: () => OverrideSpecifier | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ModifierAttribute);
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
        case NonterminalKind.ArrayTypeName:
          return new ArrayTypeName(variant as NonterminalNode);
        case NonterminalKind.FunctionType:
          return new FunctionType(variant as NonterminalNode);
        case NonterminalKind.MappingType:
          return new MappingType(variant as NonterminalNode);
        case NonterminalKind.ElementaryType:
          return new ElementaryType(variant as NonterminalNode);
        case NonterminalKind.IdentifierPath:
          return new IdentifierPath(variant as NonterminalNode);

        default:
          assert.fail(`Unexpected variant: ${variant.kind}`);
      }
    },
  );

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TypeName);
  }

  public get variant(): ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath {
    return this.fetch();
  }
}

export class FunctionTypeAttribute {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FunctionTypeAttribute);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class MappingKeyType {
  private readonly fetch: () => ElementaryType | IdentifierPath = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.ElementaryType:
        return new ElementaryType(variant as NonterminalNode);
      case NonterminalKind.IdentifierPath:
        return new IdentifierPath(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.MappingKeyType);
  }

  public get variant(): ElementaryType | IdentifierPath {
    return this.fetch();
  }
}

export class ElementaryType {
  private readonly fetch: () => AddressType | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.AddressType:
        return new AddressType(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ElementaryType);
  }

  public get variant(): AddressType | TerminalNode {
    return this.fetch();
  }
}

export class Statement {
  private readonly fetch: () =>
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
    | UncheckedBlock
    | TupleDeconstructionStatement
    | VariableDeclarationStatement
    | ExpressionStatement = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.IfStatement:
        return new IfStatement(variant as NonterminalNode);
      case NonterminalKind.ForStatement:
        return new ForStatement(variant as NonterminalNode);
      case NonterminalKind.WhileStatement:
        return new WhileStatement(variant as NonterminalNode);
      case NonterminalKind.DoWhileStatement:
        return new DoWhileStatement(variant as NonterminalNode);
      case NonterminalKind.ContinueStatement:
        return new ContinueStatement(variant as NonterminalNode);
      case NonterminalKind.BreakStatement:
        return new BreakStatement(variant as NonterminalNode);
      case NonterminalKind.ReturnStatement:
        return new ReturnStatement(variant as NonterminalNode);
      case NonterminalKind.ThrowStatement:
        return new ThrowStatement(variant as NonterminalNode);
      case NonterminalKind.EmitStatement:
        return new EmitStatement(variant as NonterminalNode);
      case NonterminalKind.TryStatement:
        return new TryStatement(variant as NonterminalNode);
      case NonterminalKind.RevertStatement:
        return new RevertStatement(variant as NonterminalNode);
      case NonterminalKind.AssemblyStatement:
        return new AssemblyStatement(variant as NonterminalNode);
      case NonterminalKind.Block:
        return new Block(variant as NonterminalNode);
      case NonterminalKind.UncheckedBlock:
        return new UncheckedBlock(variant as NonterminalNode);
      case NonterminalKind.TupleDeconstructionStatement:
        return new TupleDeconstructionStatement(variant as NonterminalNode);
      case NonterminalKind.VariableDeclarationStatement:
        return new VariableDeclarationStatement(variant as NonterminalNode);
      case NonterminalKind.ExpressionStatement:
        return new ExpressionStatement(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.Statement);
  }

  public get variant():
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
    | UncheckedBlock
    | TupleDeconstructionStatement
    | VariableDeclarationStatement
    | ExpressionStatement {
    return this.fetch();
  }
}

export class TupleMember {
  private readonly fetch: () => TypedTupleMember | UntypedTupleMember = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.TypedTupleMember:
        return new TypedTupleMember(variant as NonterminalNode);
      case NonterminalKind.UntypedTupleMember:
        return new UntypedTupleMember(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TupleMember);
  }

  public get variant(): TypedTupleMember | UntypedTupleMember {
    return this.fetch();
  }
}

export class VariableDeclarationType {
  private readonly fetch: () => TypeName | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.TypeName:
        return new TypeName(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VariableDeclarationType);
  }

  public get variant(): TypeName | TerminalNode {
    return this.fetch();
  }
}

export class StorageLocation {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StorageLocation);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class ForStatementInitialization {
  private readonly fetch: () =>
    | TupleDeconstructionStatement
    | VariableDeclarationStatement
    | ExpressionStatement
    | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.TupleDeconstructionStatement:
        return new TupleDeconstructionStatement(variant as NonterminalNode);
      case NonterminalKind.VariableDeclarationStatement:
        return new VariableDeclarationStatement(variant as NonterminalNode);
      case NonterminalKind.ExpressionStatement:
        return new ExpressionStatement(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ForStatementInitialization);
  }

  public get variant():
    | TupleDeconstructionStatement
    | VariableDeclarationStatement
    | ExpressionStatement
    | TerminalNode {
    return this.fetch();
  }
}

export class ForStatementCondition {
  private readonly fetch: () => ExpressionStatement | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.ExpressionStatement:
        return new ExpressionStatement(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ForStatementCondition);
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

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.AssignmentExpression:
        return new AssignmentExpression(variant as NonterminalNode);
      case NonterminalKind.ConditionalExpression:
        return new ConditionalExpression(variant as NonterminalNode);
      case NonterminalKind.OrExpression:
        return new OrExpression(variant as NonterminalNode);
      case NonterminalKind.AndExpression:
        return new AndExpression(variant as NonterminalNode);
      case NonterminalKind.EqualityExpression:
        return new EqualityExpression(variant as NonterminalNode);
      case NonterminalKind.ComparisonExpression:
        return new ComparisonExpression(variant as NonterminalNode);
      case NonterminalKind.BitwiseOrExpression:
        return new BitwiseOrExpression(variant as NonterminalNode);
      case NonterminalKind.BitwiseXorExpression:
        return new BitwiseXorExpression(variant as NonterminalNode);
      case NonterminalKind.BitwiseAndExpression:
        return new BitwiseAndExpression(variant as NonterminalNode);
      case NonterminalKind.ShiftExpression:
        return new ShiftExpression(variant as NonterminalNode);
      case NonterminalKind.AdditiveExpression:
        return new AdditiveExpression(variant as NonterminalNode);
      case NonterminalKind.MultiplicativeExpression:
        return new MultiplicativeExpression(variant as NonterminalNode);
      case NonterminalKind.ExponentiationExpression:
        return new ExponentiationExpression(variant as NonterminalNode);
      case NonterminalKind.PostfixExpression:
        return new PostfixExpression(variant as NonterminalNode);
      case NonterminalKind.PrefixExpression:
        return new PrefixExpression(variant as NonterminalNode);
      case NonterminalKind.FunctionCallExpression:
        return new FunctionCallExpression(variant as NonterminalNode);
      case NonterminalKind.CallOptionsExpression:
        return new CallOptionsExpression(variant as NonterminalNode);
      case NonterminalKind.MemberAccessExpression:
        return new MemberAccessExpression(variant as NonterminalNode);
      case NonterminalKind.IndexAccessExpression:
        return new IndexAccessExpression(variant as NonterminalNode);
      case NonterminalKind.NewExpression:
        return new NewExpression(variant as NonterminalNode);
      case NonterminalKind.TupleExpression:
        return new TupleExpression(variant as NonterminalNode);
      case NonterminalKind.TypeExpression:
        return new TypeExpression(variant as NonterminalNode);
      case NonterminalKind.ArrayExpression:
        return new ArrayExpression(variant as NonterminalNode);
      case NonterminalKind.HexNumberExpression:
        return new HexNumberExpression(variant as NonterminalNode);
      case NonterminalKind.DecimalNumberExpression:
        return new DecimalNumberExpression(variant as NonterminalNode);
      case NonterminalKind.StringExpression:
        return new StringExpression(variant as NonterminalNode);
      case NonterminalKind.ElementaryType:
        return new ElementaryType(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.Expression);
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

export class ArgumentsDeclaration {
  private readonly fetch: () => PositionalArgumentsDeclaration | NamedArgumentsDeclaration = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.PositionalArgumentsDeclaration:
        return new PositionalArgumentsDeclaration(variant as NonterminalNode);
      case NonterminalKind.NamedArgumentsDeclaration:
        return new NamedArgumentsDeclaration(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ArgumentsDeclaration);
  }

  public get variant(): PositionalArgumentsDeclaration | NamedArgumentsDeclaration {
    return this.fetch();
  }
}

export class NumberUnit {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.NumberUnit);
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
      case NonterminalKind.StringLiteral:
        return new StringLiteral(variant as NonterminalNode);
      case NonterminalKind.StringLiterals:
        return new StringLiterals(variant as NonterminalNode);
      case NonterminalKind.HexStringLiteral:
        return new HexStringLiteral(variant as NonterminalNode);
      case NonterminalKind.HexStringLiterals:
        return new HexStringLiterals(variant as NonterminalNode);
      case NonterminalKind.UnicodeStringLiterals:
        return new UnicodeStringLiterals(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StringExpression);
  }

  public get variant(): StringLiteral | StringLiterals | HexStringLiteral | HexStringLiterals | UnicodeStringLiterals {
    return this.fetch();
  }
}

export class StringLiteral {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StringLiteral);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class HexStringLiteral {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.HexStringLiteral);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class UnicodeStringLiteral {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UnicodeStringLiteral);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class YulStatement {
  private readonly fetch: () =>
    | YulBlock
    | YulFunctionDefinition
    | YulStackAssignmentStatement
    | YulIfStatement
    | YulForStatement
    | YulSwitchStatement
    | YulLeaveStatement
    | YulBreakStatement
    | YulContinueStatement
    | YulLabel
    | YulVariableDeclarationStatement
    | YulVariableAssignmentStatement
    | YulExpression = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.YulBlock:
        return new YulBlock(variant as NonterminalNode);
      case NonterminalKind.YulFunctionDefinition:
        return new YulFunctionDefinition(variant as NonterminalNode);
      case NonterminalKind.YulStackAssignmentStatement:
        return new YulStackAssignmentStatement(variant as NonterminalNode);
      case NonterminalKind.YulIfStatement:
        return new YulIfStatement(variant as NonterminalNode);
      case NonterminalKind.YulForStatement:
        return new YulForStatement(variant as NonterminalNode);
      case NonterminalKind.YulSwitchStatement:
        return new YulSwitchStatement(variant as NonterminalNode);
      case NonterminalKind.YulLeaveStatement:
        return new YulLeaveStatement(variant as NonterminalNode);
      case NonterminalKind.YulBreakStatement:
        return new YulBreakStatement(variant as NonterminalNode);
      case NonterminalKind.YulContinueStatement:
        return new YulContinueStatement(variant as NonterminalNode);
      case NonterminalKind.YulLabel:
        return new YulLabel(variant as NonterminalNode);
      case NonterminalKind.YulVariableDeclarationStatement:
        return new YulVariableDeclarationStatement(variant as NonterminalNode);
      case NonterminalKind.YulVariableAssignmentStatement:
        return new YulVariableAssignmentStatement(variant as NonterminalNode);
      case NonterminalKind.YulExpression:
        return new YulExpression(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulStatement);
  }

  public get variant():
    | YulBlock
    | YulFunctionDefinition
    | YulStackAssignmentStatement
    | YulIfStatement
    | YulForStatement
    | YulSwitchStatement
    | YulLeaveStatement
    | YulBreakStatement
    | YulContinueStatement
    | YulLabel
    | YulVariableDeclarationStatement
    | YulVariableAssignmentStatement
    | YulExpression {
    return this.fetch();
  }
}

export class YulAssignmentOperator {
  private readonly fetch: () => YulColonAndEqual | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.YulColonAndEqual:
        return new YulColonAndEqual(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulAssignmentOperator);
  }

  public get variant(): YulColonAndEqual | TerminalNode {
    return this.fetch();
  }
}

export class YulStackAssignmentOperator {
  private readonly fetch: () => YulEqualAndColon | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.YulEqualAndColon:
        return new YulEqualAndColon(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulStackAssignmentOperator);
  }

  public get variant(): YulEqualAndColon | TerminalNode {
    return this.fetch();
  }
}

export class YulSwitchCase {
  private readonly fetch: () => YulDefaultCase | YulValueCase = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.YulDefaultCase:
        return new YulDefaultCase(variant as NonterminalNode);
      case NonterminalKind.YulValueCase:
        return new YulValueCase(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulSwitchCase);
  }

  public get variant(): YulDefaultCase | YulValueCase {
    return this.fetch();
  }
}

export class YulExpression {
  private readonly fetch: () => YulFunctionCallExpression | YulLiteral | YulBuiltInFunction | YulPath = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.YulFunctionCallExpression:
        return new YulFunctionCallExpression(variant as NonterminalNode);
      case NonterminalKind.YulLiteral:
        return new YulLiteral(variant as NonterminalNode);
      case NonterminalKind.YulBuiltInFunction:
        return new YulBuiltInFunction(variant as NonterminalNode);
      case NonterminalKind.YulPath:
        return new YulPath(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulExpression);
  }

  public get variant(): YulFunctionCallExpression | YulLiteral | YulBuiltInFunction | YulPath {
    return this.fetch();
  }
}

export class YulPathComponent {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulPathComponent);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class YulBuiltInFunction {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    return variant as TerminalNode;
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulBuiltInFunction);
  }

  public get variant(): TerminalNode {
    return this.fetch();
  }
}

export class YulLiteral {
  private readonly fetch: () => HexStringLiteral | StringLiteral | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    if (variant.type == NodeType.Terminal) {
      return variant as TerminalNode;
    }

    switch (variant.kind) {
      case NonterminalKind.HexStringLiteral:
        return new HexStringLiteral(variant as NonterminalNode);
      case NonterminalKind.StringLiteral:
        return new StringLiteral(variant as NonterminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulLiteral);
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
    return items.map((item) => new SourceUnitMember(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.SourceUnitMembers);
  }

  public get items(): readonly SourceUnitMember[] {
    return this.fetch();
  }
}

export class VersionExpressionSet {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new VersionExpression(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VersionExpressionSet);
  }

  public get items(): readonly VersionExpression[] {
    return this.fetch();
  }
}

export class ContractMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ContractMember(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ContractMembers);
  }

  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

export class InterfaceMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ContractMember(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.InterfaceMembers);
  }

  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

export class LibraryMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ContractMember(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.LibraryMembers);
  }

  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

export class StructMembers {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new StructMember(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StructMembers);
  }

  public get items(): readonly StructMember[] {
    return this.fetch();
  }
}

export class StateVariableAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new StateVariableAttribute(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StateVariableAttributes);
  }

  public get items(): readonly StateVariableAttribute[] {
    return this.fetch();
  }
}

export class FunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new FunctionAttribute(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FunctionAttributes);
  }

  public get items(): readonly FunctionAttribute[] {
    return this.fetch();
  }
}

export class ConstructorAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ConstructorAttribute(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ConstructorAttributes);
  }

  public get items(): readonly ConstructorAttribute[] {
    return this.fetch();
  }
}

export class UnnamedFunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new UnnamedFunctionAttribute(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UnnamedFunctionAttributes);
  }

  public get items(): readonly UnnamedFunctionAttribute[] {
    return this.fetch();
  }
}

export class FallbackFunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new FallbackFunctionAttribute(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FallbackFunctionAttributes);
  }

  public get items(): readonly FallbackFunctionAttribute[] {
    return this.fetch();
  }
}

export class ReceiveFunctionAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ReceiveFunctionAttribute(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ReceiveFunctionAttributes);
  }

  public get items(): readonly ReceiveFunctionAttribute[] {
    return this.fetch();
  }
}

export class ModifierAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new ModifierAttribute(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ModifierAttributes);
  }

  public get items(): readonly ModifierAttribute[] {
    return this.fetch();
  }
}

export class FunctionTypeAttributes {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new FunctionTypeAttribute(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.FunctionTypeAttributes);
  }

  public get items(): readonly FunctionTypeAttribute[] {
    return this.fetch();
  }
}

export class Statements {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new Statement(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.Statements);
  }

  public get items(): readonly Statement[] {
    return this.fetch();
  }
}

export class CatchClauses {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new CatchClause(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.CatchClauses);
  }

  public get items(): readonly CatchClause[] {
    return this.fetch();
  }
}

export class StringLiterals {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new StringLiteral(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.StringLiterals);
  }

  public get items(): readonly StringLiteral[] {
    return this.fetch();
  }
}

export class HexStringLiterals {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new HexStringLiteral(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.HexStringLiterals);
  }

  public get items(): readonly HexStringLiteral[] {
    return this.fetch();
  }
}

export class UnicodeStringLiterals {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new UnicodeStringLiteral(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UnicodeStringLiterals);
  }

  public get items(): readonly UnicodeStringLiteral[] {
    return this.fetch();
  }
}

export class YulStatements {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new YulStatement(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulStatements);
  }

  public get items(): readonly YulStatement[] {
    return this.fetch();
  }
}

export class YulSwitchCases {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new YulSwitchCase(item as NonterminalNode));
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulSwitchCases);
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
      items: items.map((item) => new VersionExpressionSet(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VersionExpressionSets);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.VersionSpecifiers);
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
      items: items.map((item) => new ImportDeconstructionSymbol(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ImportDeconstructionSymbols);
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
      items: items.map((item) => new UsingDeconstructionSymbol(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.UsingDeconstructionSymbols);
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
      items: items.map((item) => new InheritanceType(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.InheritanceTypes);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.EnumMembers);
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
      items: items.map((item) => new Parameter(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.Parameters);
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
      items: items.map((item) => new IdentifierPath(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.OverridePaths);
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
      items: items.map((item) => new EventParameter(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.EventParameters);
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
      items: items.map((item) => new ErrorParameter(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ErrorParameters);
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
      items: items.map((item) => new StringLiteral(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.AssemblyFlags);
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
      items: items.map((item) => new TupleDeconstructionElement(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TupleDeconstructionElements);
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
      items: items.map((item) => new Expression(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.PositionalArguments);
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
      items: items.map((item) => new NamedArgument(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.NamedArguments);
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
      items: items.map((item) => new NamedArgument(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.CallOptions);
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
      items: items.map((item) => new TupleValue(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.TupleValues);
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
      items: items.map((item) => new Expression(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.ArrayValues);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.IdentifierPath);
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

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulParameters);
  }

  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

export class YulVariableNames {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulVariableNames);
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
      items: items.map((item) => new YulExpression(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulArguments);
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
      items: items.map((item) => new YulPath(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulPaths);
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
      items: items.map((item) => new YulPathComponent(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  public constructor(public readonly cst: NonterminalNode) {
    assertKind(this.cst.kind, NonterminalKind.YulPath);
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

function assertKind(actual: NonterminalKind, expected: NonterminalKind): void {
  assert.equal(actual, expected, `${expected} can only be initialized with a CST node of the same kind.`);
}
