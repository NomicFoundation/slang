// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import * as wasm from "../../../../wasm/index.mjs";
import { NonterminalKind, NonterminalNode, TerminalNode } from "../../cst/index.mjs";

//
// Sequences:
//

/**
 * This node represents a `SourceUnit` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnit = (* members: *) SourceUnitMembers;
 * ```
 */
export class SourceUnit {
  private readonly fetch = once(() => {
    const [$members] = wasm.ast.Selectors.sequence(this.cst);

    return {
      members: new SourceUnitMembers($members as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `SourceUnit`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `SourceUnit`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.SourceUnit);
  }

  /**
   * Returns the child node that has the label `members`.
   */
  public get members(): SourceUnitMembers {
    return this.fetch().members;
  }
}

/**
 * This node represents a `PragmaDirective` nonterminal, with the following structure:
 *
 * ```ebnf
 * PragmaDirective = (* pragma_keyword: *) PRAGMA_KEYWORD
 *                   (* pragma: *) Pragma
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
export class PragmaDirective {
  private readonly fetch = once(() => {
    const [$pragmaKeyword, $pragma, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      pragmaKeyword: $pragmaKeyword as TerminalNode,
      pragma: new Pragma($pragma as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `PragmaDirective`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `PragmaDirective`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.PragmaDirective);
  }

  /**
   * Returns the child node that has the label `pragma_keyword`.
   */
  public get pragmaKeyword(): TerminalNode {
    return this.fetch().pragmaKeyword;
  }

  /**
   * Returns the child node that has the label `pragma`.
   */
  public get pragma(): Pragma {
    return this.fetch().pragma;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `AbicoderPragma` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.7.5 *)
 * AbicoderPragma = (* abicoder_keyword: *) ABICODER_KEYWORD
 *                  (* version: *) AbicoderVersion;
 * ```
 */
export class AbicoderPragma {
  private readonly fetch = once(() => {
    const [$abicoderKeyword, $version] = wasm.ast.Selectors.sequence(this.cst);

    return {
      abicoderKeyword: $abicoderKeyword as TerminalNode,
      version: new AbicoderVersion($version as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `AbicoderPragma`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AbicoderPragma`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AbicoderPragma);
  }

  /**
   * Returns the child node that has the label `abicoder_keyword`.
   */
  public get abicoderKeyword(): TerminalNode {
    return this.fetch().abicoderKeyword;
  }

  /**
   * Returns the child node that has the label `version`.
   */
  public get version(): AbicoderVersion {
    return this.fetch().version;
  }
}

/**
 * This node represents a `ExperimentalPragma` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.4.16 *)
 * ExperimentalPragma = (* experimental_keyword: *) EXPERIMENTAL_KEYWORD
 *                      (* feature: *) ExperimentalFeature;
 * ```
 */
export class ExperimentalPragma {
  private readonly fetch = once(() => {
    const [$experimentalKeyword, $feature] = wasm.ast.Selectors.sequence(this.cst);

    return {
      experimentalKeyword: $experimentalKeyword as TerminalNode,
      feature: new ExperimentalFeature($feature as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ExperimentalPragma`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ExperimentalPragma`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ExperimentalPragma);
  }

  /**
   * Returns the child node that has the label `experimental_keyword`.
   */
  public get experimentalKeyword(): TerminalNode {
    return this.fetch().experimentalKeyword;
  }

  /**
   * Returns the child node that has the label `feature`.
   */
  public get feature(): ExperimentalFeature {
    return this.fetch().feature;
  }
}

/**
 * This node represents a `VersionPragma` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionPragma = (* solidity_keyword: *) SOLIDITY_KEYWORD
 *                 (* sets: *) VersionExpressionSets;
 * ```
 */
export class VersionPragma {
  private readonly fetch = once(() => {
    const [$solidityKeyword, $sets] = wasm.ast.Selectors.sequence(this.cst);

    return {
      solidityKeyword: $solidityKeyword as TerminalNode,
      sets: new VersionExpressionSets($sets as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `VersionPragma`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VersionPragma`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VersionPragma);
  }

  /**
   * Returns the child node that has the label `solidity_keyword`.
   */
  public get solidityKeyword(): TerminalNode {
    return this.fetch().solidityKeyword;
  }

  /**
   * Returns the child node that has the label `sets`.
   */
  public get sets(): VersionExpressionSets {
    return this.fetch().sets;
  }
}

/**
 * This node represents a `VersionRange` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionRange = (* start: *) VersionLiteral
 *                (* minus: *) MINUS
 *                (* end: *) VersionLiteral;
 * ```
 */
export class VersionRange {
  private readonly fetch = once(() => {
    const [$start, $minus, $end] = wasm.ast.Selectors.sequence(this.cst);

    return {
      start: new VersionLiteral($start as NonterminalNode),
      minus: $minus as TerminalNode,
      end: new VersionLiteral($end as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `VersionRange`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VersionRange`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VersionRange);
  }

  /**
   * Returns the child node that has the label `start`.
   */
  public get start(): VersionLiteral {
    return this.fetch().start;
  }

  /**
   * Returns the child node that has the label `minus`.
   */
  public get minus(): TerminalNode {
    return this.fetch().minus;
  }

  /**
   * Returns the child node that has the label `end`.
   */
  public get end(): VersionLiteral {
    return this.fetch().end;
  }
}

/**
 * This node represents a `VersionTerm` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionTerm = (* operator: *) VersionOperator?
 *               (* literal: *) VersionLiteral;
 * ```
 */
export class VersionTerm {
  private readonly fetch = once(() => {
    const [$operator, $literal] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operator: $operator === undefined ? undefined : new VersionOperator($operator as NonterminalNode),
      literal: new VersionLiteral($literal as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `VersionTerm`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VersionTerm`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VersionTerm);
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): VersionOperator | undefined {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `literal`.
   */
  public get literal(): VersionLiteral {
    return this.fetch().literal;
  }
}

/**
 * This node represents a `ImportDirective` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportDirective = (* import_keyword: *) IMPORT_KEYWORD
 *                   (* clause: *) ImportClause
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
export class ImportDirective {
  private readonly fetch = once(() => {
    const [$importKeyword, $clause, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      importKeyword: $importKeyword as TerminalNode,
      clause: new ImportClause($clause as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ImportDirective`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ImportDirective`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ImportDirective);
  }

  /**
   * Returns the child node that has the label `import_keyword`.
   */
  public get importKeyword(): TerminalNode {
    return this.fetch().importKeyword;
  }

  /**
   * Returns the child node that has the label `clause`.
   */
  public get clause(): ImportClause {
    return this.fetch().clause;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `PathImport` nonterminal, with the following structure:
 *
 * ```ebnf
 * PathImport = (* path: *) StringLiteral
 *              (* alias: *) ImportAlias?;
 * ```
 */
export class PathImport {
  private readonly fetch = once(() => {
    const [$path, $alias] = wasm.ast.Selectors.sequence(this.cst);

    return {
      path: new StringLiteral($path as NonterminalNode),
      alias: $alias === undefined ? undefined : new ImportAlias($alias as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `PathImport`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `PathImport`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.PathImport);
  }

  /**
   * Returns the child node that has the label `path`.
   */
  public get path(): StringLiteral {
    return this.fetch().path;
  }

  /**
   * Returns the child node that has the label `alias`.
   */
  public get alias(): ImportAlias | undefined {
    return this.fetch().alias;
  }
}

/**
 * This node represents a `NamedImport` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedImport = (* asterisk: *) ASTERISK
 *               (* alias: *) ImportAlias
 *               (* from_keyword: *) FROM_KEYWORD
 *               (* path: *) StringLiteral;
 * ```
 */
export class NamedImport {
  private readonly fetch = once(() => {
    const [$asterisk, $alias, $fromKeyword, $path] = wasm.ast.Selectors.sequence(this.cst);

    return {
      asterisk: $asterisk as TerminalNode,
      alias: new ImportAlias($alias as NonterminalNode),
      fromKeyword: $fromKeyword as TerminalNode,
      path: new StringLiteral($path as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `NamedImport`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `NamedImport`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.NamedImport);
  }

  /**
   * Returns the child node that has the label `asterisk`.
   */
  public get asterisk(): TerminalNode {
    return this.fetch().asterisk;
  }

  /**
   * Returns the child node that has the label `alias`.
   */
  public get alias(): ImportAlias {
    return this.fetch().alias;
  }

  /**
   * Returns the child node that has the label `from_keyword`.
   */
  public get fromKeyword(): TerminalNode {
    return this.fetch().fromKeyword;
  }

  /**
   * Returns the child node that has the label `path`.
   */
  public get path(): StringLiteral {
    return this.fetch().path;
  }
}

/**
 * This node represents a `ImportDeconstruction` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportDeconstruction = (* open_brace: *) OPEN_BRACE
 *                        (* symbols: *) ImportDeconstructionSymbols
 *                        (* close_brace: *) CLOSE_BRACE
 *                        (* from_keyword: *) FROM_KEYWORD
 *                        (* path: *) StringLiteral;
 * ```
 */
export class ImportDeconstruction {
  private readonly fetch = once(() => {
    const [$openBrace, $symbols, $closeBrace, $fromKeyword, $path] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openBrace: $openBrace as TerminalNode,
      symbols: new ImportDeconstructionSymbols($symbols as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
      fromKeyword: $fromKeyword as TerminalNode,
      path: new StringLiteral($path as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ImportDeconstruction`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ImportDeconstruction`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ImportDeconstruction);
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `symbols`.
   */
  public get symbols(): ImportDeconstructionSymbols {
    return this.fetch().symbols;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }

  /**
   * Returns the child node that has the label `from_keyword`.
   */
  public get fromKeyword(): TerminalNode {
    return this.fetch().fromKeyword;
  }

  /**
   * Returns the child node that has the label `path`.
   */
  public get path(): StringLiteral {
    return this.fetch().path;
  }
}

/**
 * This node represents a `ImportDeconstructionSymbol` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportDeconstructionSymbol = (* name: *) IDENTIFIER
 *                              (* alias: *) ImportAlias?;
 * ```
 */
export class ImportDeconstructionSymbol {
  private readonly fetch = once(() => {
    const [$name, $alias] = wasm.ast.Selectors.sequence(this.cst);

    return {
      name: $name as TerminalNode,
      alias: $alias === undefined ? undefined : new ImportAlias($alias as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ImportDeconstructionSymbol`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ImportDeconstructionSymbol`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ImportDeconstructionSymbol);
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `alias`.
   */
  public get alias(): ImportAlias | undefined {
    return this.fetch().alias;
  }
}

/**
 * This node represents a `ImportAlias` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportAlias = (* as_keyword: *) AS_KEYWORD
 *               (* identifier: *) IDENTIFIER;
 * ```
 */
export class ImportAlias {
  private readonly fetch = once(() => {
    const [$asKeyword, $identifier] = wasm.ast.Selectors.sequence(this.cst);

    return {
      asKeyword: $asKeyword as TerminalNode,
      identifier: $identifier as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ImportAlias`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ImportAlias`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ImportAlias);
  }

  /**
   * Returns the child node that has the label `as_keyword`.
   */
  public get asKeyword(): TerminalNode {
    return this.fetch().asKeyword;
  }

  /**
   * Returns the child node that has the label `identifier`.
   */
  public get identifier(): TerminalNode {
    return this.fetch().identifier;
  }
}

/**
 * This node represents a `UsingDirective` nonterminal, with the following structure:
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
export class UsingDirective {
  private readonly fetch = once(() => {
    const [$usingKeyword, $clause, $forKeyword, $target, $globalKeyword, $semicolon] = wasm.ast.Selectors.sequence(
      this.cst,
    );

    return {
      usingKeyword: $usingKeyword as TerminalNode,
      clause: new UsingClause($clause as NonterminalNode),
      forKeyword: $forKeyword as TerminalNode,
      target: new UsingTarget($target as NonterminalNode),
      globalKeyword: $globalKeyword === undefined ? undefined : ($globalKeyword as TerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `UsingDirective`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UsingDirective`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UsingDirective);
  }

  /**
   * Returns the child node that has the label `using_keyword`.
   */
  public get usingKeyword(): TerminalNode {
    return this.fetch().usingKeyword;
  }

  /**
   * Returns the child node that has the label `clause`.
   */
  public get clause(): UsingClause {
    return this.fetch().clause;
  }

  /**
   * Returns the child node that has the label `for_keyword`.
   */
  public get forKeyword(): TerminalNode {
    return this.fetch().forKeyword;
  }

  /**
   * Returns the child node that has the label `target`.
   */
  public get target(): UsingTarget {
    return this.fetch().target;
  }

  /**
   * Returns the child node that has the label `global_keyword`.
   */
  public get globalKeyword(): TerminalNode | undefined {
    return this.fetch().globalKeyword;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `UsingDeconstruction` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * UsingDeconstruction = (* open_brace: *) OPEN_BRACE
 *                       (* symbols: *) UsingDeconstructionSymbols
 *                       (* close_brace: *) CLOSE_BRACE;
 * ```
 */
export class UsingDeconstruction {
  private readonly fetch = once(() => {
    const [$openBrace, $symbols, $closeBrace] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openBrace: $openBrace as TerminalNode,
      symbols: new UsingDeconstructionSymbols($symbols as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `UsingDeconstruction`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UsingDeconstruction`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UsingDeconstruction);
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `symbols`.
   */
  public get symbols(): UsingDeconstructionSymbols {
    return this.fetch().symbols;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `UsingDeconstructionSymbol` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * UsingDeconstructionSymbol = (* name: *) IdentifierPath
 *                             (* alias: *) UsingAlias?; (* Introduced in 0.8.19 *)
 * ```
 */
export class UsingDeconstructionSymbol {
  private readonly fetch = once(() => {
    const [$name, $alias] = wasm.ast.Selectors.sequence(this.cst);

    return {
      name: new IdentifierPath($name as NonterminalNode),
      alias: $alias === undefined ? undefined : new UsingAlias($alias as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `UsingDeconstructionSymbol`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UsingDeconstructionSymbol`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UsingDeconstructionSymbol);
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): IdentifierPath {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `alias`.
   */
  public get alias(): UsingAlias | undefined {
    return this.fetch().alias;
  }
}

/**
 * This node represents a `UsingAlias` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.19 *)
 * UsingAlias = (* as_keyword: *) AS_KEYWORD
 *              (* operator: *) UsingOperator;
 * ```
 */
export class UsingAlias {
  private readonly fetch = once(() => {
    const [$asKeyword, $operator] = wasm.ast.Selectors.sequence(this.cst);

    return {
      asKeyword: $asKeyword as TerminalNode,
      operator: new UsingOperator($operator as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `UsingAlias`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UsingAlias`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UsingAlias);
  }

  /**
   * Returns the child node that has the label `as_keyword`.
   */
  public get asKeyword(): TerminalNode {
    return this.fetch().asKeyword;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): UsingOperator {
    return this.fetch().operator;
  }
}

/**
 * This node represents a `ContractDefinition` nonterminal, with the following structure:
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
export class ContractDefinition {
  private readonly fetch = once(() => {
    const [$abstractKeyword, $contractKeyword, $name, $specifiers, $openBrace, $members, $closeBrace] =
      wasm.ast.Selectors.sequence(this.cst);

    return {
      abstractKeyword: $abstractKeyword === undefined ? undefined : ($abstractKeyword as TerminalNode),
      contractKeyword: $contractKeyword as TerminalNode,
      name: $name as TerminalNode,
      specifiers: new ContractSpecifiers($specifiers as NonterminalNode),
      openBrace: $openBrace as TerminalNode,
      members: new ContractMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ContractDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ContractDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ContractDefinition);
  }

  /**
   * Returns the child node that has the label `abstract_keyword`.
   */
  public get abstractKeyword(): TerminalNode | undefined {
    return this.fetch().abstractKeyword;
  }

  /**
   * Returns the child node that has the label `contract_keyword`.
   */
  public get contractKeyword(): TerminalNode {
    return this.fetch().contractKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `specifiers`.
   */
  public get specifiers(): ContractSpecifiers {
    return this.fetch().specifiers;
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `members`.
   */
  public get members(): ContractMembers {
    return this.fetch().members;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `InheritanceSpecifier` nonterminal, with the following structure:
 *
 * ```ebnf
 * InheritanceSpecifier = (* is_keyword: *) IS_KEYWORD
 *                        (* types: *) InheritanceTypes;
 * ```
 */
export class InheritanceSpecifier {
  private readonly fetch = once(() => {
    const [$isKeyword, $types] = wasm.ast.Selectors.sequence(this.cst);

    return {
      isKeyword: $isKeyword as TerminalNode,
      types: new InheritanceTypes($types as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `InheritanceSpecifier`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `InheritanceSpecifier`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.InheritanceSpecifier);
  }

  /**
   * Returns the child node that has the label `is_keyword`.
   */
  public get isKeyword(): TerminalNode {
    return this.fetch().isKeyword;
  }

  /**
   * Returns the child node that has the label `types`.
   */
  public get types(): InheritanceTypes {
    return this.fetch().types;
  }
}

/**
 * This node represents a `InheritanceType` nonterminal, with the following structure:
 *
 * ```ebnf
 * InheritanceType = (* type_name: *) IdentifierPath
 *                   (* arguments: *) ArgumentsDeclaration?;
 * ```
 */
export class InheritanceType {
  private readonly fetch = once(() => {
    const [$typeName, $arguments] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeName: new IdentifierPath($typeName as NonterminalNode),
      arguments: $arguments === undefined ? undefined : new ArgumentsDeclaration($arguments as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `InheritanceType`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `InheritanceType`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.InheritanceType);
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): IdentifierPath {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `arguments`.
   */
  public get arguments(): ArgumentsDeclaration | undefined {
    return this.fetch().arguments;
  }
}

/**
 * This node represents a `StorageLayoutSpecifier` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.29 *)
 * StorageLayoutSpecifier = (* layout_keyword: *) LAYOUT_KEYWORD
 *                          (* at_keyword: *) AT_KEYWORD
 *                          (* expression: *) Expression;
 * ```
 */
export class StorageLayoutSpecifier {
  private readonly fetch = once(() => {
    const [$layoutKeyword, $atKeyword, $expression] = wasm.ast.Selectors.sequence(this.cst);

    return {
      layoutKeyword: $layoutKeyword as TerminalNode,
      atKeyword: $atKeyword as TerminalNode,
      expression: new Expression($expression as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `StorageLayoutSpecifier`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StorageLayoutSpecifier`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StorageLayoutSpecifier);
  }

  /**
   * Returns the child node that has the label `layout_keyword`.
   */
  public get layoutKeyword(): TerminalNode {
    return this.fetch().layoutKeyword;
  }

  /**
   * Returns the child node that has the label `at_keyword`.
   */
  public get atKeyword(): TerminalNode {
    return this.fetch().atKeyword;
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): Expression {
    return this.fetch().expression;
  }
}

/**
 * This node represents a `InterfaceDefinition` nonterminal, with the following structure:
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
export class InterfaceDefinition {
  private readonly fetch = once(() => {
    const [$interfaceKeyword, $name, $inheritance, $openBrace, $members, $closeBrace] = wasm.ast.Selectors.sequence(
      this.cst,
    );

    return {
      interfaceKeyword: $interfaceKeyword as TerminalNode,
      name: $name as TerminalNode,
      inheritance: $inheritance === undefined ? undefined : new InheritanceSpecifier($inheritance as NonterminalNode),
      openBrace: $openBrace as TerminalNode,
      members: new InterfaceMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `InterfaceDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `InterfaceDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.InterfaceDefinition);
  }

  /**
   * Returns the child node that has the label `interface_keyword`.
   */
  public get interfaceKeyword(): TerminalNode {
    return this.fetch().interfaceKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `inheritance`.
   */
  public get inheritance(): InheritanceSpecifier | undefined {
    return this.fetch().inheritance;
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `members`.
   */
  public get members(): InterfaceMembers {
    return this.fetch().members;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `LibraryDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * LibraryDefinition = (* library_keyword: *) LIBRARY_KEYWORD
 *                     (* name: *) IDENTIFIER
 *                     (* open_brace: *) OPEN_BRACE
 *                     (* members: *) LibraryMembers
 *                     (* close_brace: *) CLOSE_BRACE;
 * ```
 */
export class LibraryDefinition {
  private readonly fetch = once(() => {
    const [$libraryKeyword, $name, $openBrace, $members, $closeBrace] = wasm.ast.Selectors.sequence(this.cst);

    return {
      libraryKeyword: $libraryKeyword as TerminalNode,
      name: $name as TerminalNode,
      openBrace: $openBrace as TerminalNode,
      members: new LibraryMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `LibraryDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `LibraryDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.LibraryDefinition);
  }

  /**
   * Returns the child node that has the label `library_keyword`.
   */
  public get libraryKeyword(): TerminalNode {
    return this.fetch().libraryKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `members`.
   */
  public get members(): LibraryMembers {
    return this.fetch().members;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `StructDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * StructDefinition = (* struct_keyword: *) STRUCT_KEYWORD
 *                    (* name: *) IDENTIFIER
 *                    (* open_brace: *) OPEN_BRACE
 *                    (* members: *) StructMembers
 *                    (* close_brace: *) CLOSE_BRACE;
 * ```
 */
export class StructDefinition {
  private readonly fetch = once(() => {
    const [$structKeyword, $name, $openBrace, $members, $closeBrace] = wasm.ast.Selectors.sequence(this.cst);

    return {
      structKeyword: $structKeyword as TerminalNode,
      name: $name as TerminalNode,
      openBrace: $openBrace as TerminalNode,
      members: new StructMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `StructDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StructDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StructDefinition);
  }

  /**
   * Returns the child node that has the label `struct_keyword`.
   */
  public get structKeyword(): TerminalNode {
    return this.fetch().structKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `members`.
   */
  public get members(): StructMembers {
    return this.fetch().members;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `StructMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * StructMember = (* type_name: *) TypeName
 *                (* name: *) IDENTIFIER
 *                (* semicolon: *) SEMICOLON;
 * ```
 */
export class StructMember {
  private readonly fetch = once(() => {
    const [$typeName, $name, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonterminalNode),
      name: $name as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `StructMember`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StructMember`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StructMember);
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `EnumDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * EnumDefinition = (* enum_keyword: *) ENUM_KEYWORD
 *                  (* name: *) IDENTIFIER
 *                  (* open_brace: *) OPEN_BRACE
 *                  (* members: *) EnumMembers
 *                  (* close_brace: *) CLOSE_BRACE;
 * ```
 */
export class EnumDefinition {
  private readonly fetch = once(() => {
    const [$enumKeyword, $name, $openBrace, $members, $closeBrace] = wasm.ast.Selectors.sequence(this.cst);

    return {
      enumKeyword: $enumKeyword as TerminalNode,
      name: $name as TerminalNode,
      openBrace: $openBrace as TerminalNode,
      members: new EnumMembers($members as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `EnumDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `EnumDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.EnumDefinition);
  }

  /**
   * Returns the child node that has the label `enum_keyword`.
   */
  public get enumKeyword(): TerminalNode {
    return this.fetch().enumKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `members`.
   */
  public get members(): EnumMembers {
    return this.fetch().members;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `ConstantDefinition` nonterminal, with the following structure:
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
export class ConstantDefinition {
  private readonly fetch = once(() => {
    const [$typeName, $constantKeyword, $name, $equal, $value, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonterminalNode),
      constantKeyword: $constantKeyword as TerminalNode,
      name: $name as TerminalNode,
      equal: $equal as TerminalNode,
      value: new Expression($value as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ConstantDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ConstantDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ConstantDefinition);
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `constant_keyword`.
   */
  public get constantKeyword(): TerminalNode {
    return this.fetch().constantKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `equal`.
   */
  public get equal(): TerminalNode {
    return this.fetch().equal;
  }

  /**
   * Returns the child node that has the label `value`.
   */
  public get value(): Expression {
    return this.fetch().value;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `StateVariableDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * StateVariableDefinition = (* type_name: *) TypeName
 *                           (* attributes: *) StateVariableAttributes
 *                           (* name: *) IDENTIFIER
 *                           (* value: *) StateVariableDefinitionValue?
 *                           (* semicolon: *) SEMICOLON;
 * ```
 */
export class StateVariableDefinition {
  private readonly fetch = once(() => {
    const [$typeName, $attributes, $name, $value, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonterminalNode),
      attributes: new StateVariableAttributes($attributes as NonterminalNode),
      name: $name as TerminalNode,
      value: $value === undefined ? undefined : new StateVariableDefinitionValue($value as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `StateVariableDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StateVariableDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StateVariableDefinition);
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `attributes`.
   */
  public get attributes(): StateVariableAttributes {
    return this.fetch().attributes;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `value`.
   */
  public get value(): StateVariableDefinitionValue | undefined {
    return this.fetch().value;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `StateVariableDefinitionValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * StateVariableDefinitionValue = (* equal: *) EQUAL
 *                                (* value: *) Expression;
 * ```
 */
export class StateVariableDefinitionValue {
  private readonly fetch = once(() => {
    const [$equal, $value] = wasm.ast.Selectors.sequence(this.cst);

    return {
      equal: $equal as TerminalNode,
      value: new Expression($value as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `StateVariableDefinitionValue`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StateVariableDefinitionValue`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StateVariableDefinitionValue);
  }

  /**
   * Returns the child node that has the label `equal`.
   */
  public get equal(): TerminalNode {
    return this.fetch().equal;
  }

  /**
   * Returns the child node that has the label `value`.
   */
  public get value(): Expression {
    return this.fetch().value;
  }
}

/**
 * This node represents a `FunctionDefinition` nonterminal, with the following structure:
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
export class FunctionDefinition {
  private readonly fetch = once(() => {
    const [$functionKeyword, $name, $parameters, $attributes, $returns, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TerminalNode,
      name: new FunctionName($name as NonterminalNode),
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new FunctionAttributes($attributes as NonterminalNode),
      returns: $returns === undefined ? undefined : new ReturnsDeclaration($returns as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `FunctionDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FunctionDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FunctionDefinition);
  }

  /**
   * Returns the child node that has the label `function_keyword`.
   */
  public get functionKeyword(): TerminalNode {
    return this.fetch().functionKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): FunctionName {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `attributes`.
   */
  public get attributes(): FunctionAttributes {
    return this.fetch().attributes;
  }

  /**
   * Returns the child node that has the label `returns`.
   */
  public get returns(): ReturnsDeclaration | undefined {
    return this.fetch().returns;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

/**
 * This node represents a `ParametersDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * ParametersDeclaration = (* open_paren: *) OPEN_PAREN
 *                         (* parameters: *) Parameters
 *                         (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class ParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      parameters: new Parameters($parameters as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ParametersDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ParametersDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ParametersDeclaration);
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): Parameters {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `Parameter` nonterminal, with the following structure:
 *
 * ```ebnf
 * Parameter = (* type_name: *) TypeName
 *             (* storage_location: *) StorageLocation?
 *             (* name: *) IDENTIFIER?;
 * ```
 */
export class Parameter {
  private readonly fetch = once(() => {
    const [$typeName, $storageLocation, $name] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonterminalNode),
      storageLocation:
        $storageLocation === undefined ? undefined : new StorageLocation($storageLocation as NonterminalNode),
      name: $name === undefined ? undefined : ($name as TerminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `Parameter`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Parameter`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Parameter);
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `storage_location`.
   */
  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

/**
 * This node represents a `OverrideSpecifier` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * OverrideSpecifier = (* override_keyword: *) OVERRIDE_KEYWORD
 *                     (* overridden: *) OverridePathsDeclaration?;
 * ```
 */
export class OverrideSpecifier {
  private readonly fetch = once(() => {
    const [$overrideKeyword, $overridden] = wasm.ast.Selectors.sequence(this.cst);

    return {
      overrideKeyword: $overrideKeyword as TerminalNode,
      overridden: $overridden === undefined ? undefined : new OverridePathsDeclaration($overridden as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `OverrideSpecifier`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `OverrideSpecifier`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.OverrideSpecifier);
  }

  /**
   * Returns the child node that has the label `override_keyword`.
   */
  public get overrideKeyword(): TerminalNode {
    return this.fetch().overrideKeyword;
  }

  /**
   * Returns the child node that has the label `overridden`.
   */
  public get overridden(): OverridePathsDeclaration | undefined {
    return this.fetch().overridden;
  }
}

/**
 * This node represents a `OverridePathsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * OverridePathsDeclaration = (* open_paren: *) OPEN_PAREN
 *                            (* paths: *) OverridePaths
 *                            (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class OverridePathsDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $paths, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      paths: new OverridePaths($paths as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `OverridePathsDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `OverridePathsDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.OverridePathsDeclaration);
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `paths`.
   */
  public get paths(): OverridePaths {
    return this.fetch().paths;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `ReturnsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * ReturnsDeclaration = (* returns_keyword: *) RETURNS_KEYWORD
 *                      (* variables: *) ParametersDeclaration;
 * ```
 */
export class ReturnsDeclaration {
  private readonly fetch = once(() => {
    const [$returnsKeyword, $variables] = wasm.ast.Selectors.sequence(this.cst);

    return {
      returnsKeyword: $returnsKeyword as TerminalNode,
      variables: new ParametersDeclaration($variables as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ReturnsDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ReturnsDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ReturnsDeclaration);
  }

  /**
   * Returns the child node that has the label `returns_keyword`.
   */
  public get returnsKeyword(): TerminalNode {
    return this.fetch().returnsKeyword;
  }

  /**
   * Returns the child node that has the label `variables`.
   */
  public get variables(): ParametersDeclaration {
    return this.fetch().variables;
  }
}

/**
 * This node represents a `ConstructorDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.4.22 *)
 * ConstructorDefinition = (* constructor_keyword: *) CONSTRUCTOR_KEYWORD
 *                         (* parameters: *) ParametersDeclaration
 *                         (* attributes: *) ConstructorAttributes
 *                         (* body: *) Block;
 * ```
 */
export class ConstructorDefinition {
  private readonly fetch = once(() => {
    const [$constructorKeyword, $parameters, $attributes, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      constructorKeyword: $constructorKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new ConstructorAttributes($attributes as NonterminalNode),
      body: new Block($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ConstructorDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ConstructorDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ConstructorDefinition);
  }

  /**
   * Returns the child node that has the label `constructor_keyword`.
   */
  public get constructorKeyword(): TerminalNode {
    return this.fetch().constructorKeyword;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `attributes`.
   */
  public get attributes(): ConstructorAttributes {
    return this.fetch().attributes;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): Block {
    return this.fetch().body;
  }
}

/**
 * This node represents a `UnnamedFunctionDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.6.0 *)
 * UnnamedFunctionDefinition = (* function_keyword: *) FUNCTION_KEYWORD
 *                             (* parameters: *) ParametersDeclaration
 *                             (* attributes: *) UnnamedFunctionAttributes
 *                             (* body: *) FunctionBody;
 * ```
 */
export class UnnamedFunctionDefinition {
  private readonly fetch = once(() => {
    const [$functionKeyword, $parameters, $attributes, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new UnnamedFunctionAttributes($attributes as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `UnnamedFunctionDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UnnamedFunctionDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UnnamedFunctionDefinition);
  }

  /**
   * Returns the child node that has the label `function_keyword`.
   */
  public get functionKeyword(): TerminalNode {
    return this.fetch().functionKeyword;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `attributes`.
   */
  public get attributes(): UnnamedFunctionAttributes {
    return this.fetch().attributes;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

/**
 * This node represents a `FallbackFunctionDefinition` nonterminal, with the following structure:
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
export class FallbackFunctionDefinition {
  private readonly fetch = once(() => {
    const [$fallbackKeyword, $parameters, $attributes, $returns, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      fallbackKeyword: $fallbackKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new FallbackFunctionAttributes($attributes as NonterminalNode),
      returns: $returns === undefined ? undefined : new ReturnsDeclaration($returns as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `FallbackFunctionDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FallbackFunctionDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FallbackFunctionDefinition);
  }

  /**
   * Returns the child node that has the label `fallback_keyword`.
   */
  public get fallbackKeyword(): TerminalNode {
    return this.fetch().fallbackKeyword;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `attributes`.
   */
  public get attributes(): FallbackFunctionAttributes {
    return this.fetch().attributes;
  }

  /**
   * Returns the child node that has the label `returns`.
   */
  public get returns(): ReturnsDeclaration | undefined {
    return this.fetch().returns;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

/**
 * This node represents a `ReceiveFunctionDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * ReceiveFunctionDefinition = (* receive_keyword: *) RECEIVE_KEYWORD
 *                             (* parameters: *) ParametersDeclaration
 *                             (* attributes: *) ReceiveFunctionAttributes
 *                             (* body: *) FunctionBody;
 * ```
 */
export class ReceiveFunctionDefinition {
  private readonly fetch = once(() => {
    const [$receiveKeyword, $parameters, $attributes, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      receiveKeyword: $receiveKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new ReceiveFunctionAttributes($attributes as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ReceiveFunctionDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ReceiveFunctionDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ReceiveFunctionDefinition);
  }

  /**
   * Returns the child node that has the label `receive_keyword`.
   */
  public get receiveKeyword(): TerminalNode {
    return this.fetch().receiveKeyword;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `attributes`.
   */
  public get attributes(): ReceiveFunctionAttributes {
    return this.fetch().attributes;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

/**
 * This node represents a `ModifierDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * ModifierDefinition = (* modifier_keyword: *) MODIFIER_KEYWORD
 *                      (* name: *) IDENTIFIER
 *                      (* parameters: *) ParametersDeclaration?
 *                      (* attributes: *) ModifierAttributes
 *                      (* body: *) FunctionBody;
 * ```
 */
export class ModifierDefinition {
  private readonly fetch = once(() => {
    const [$modifierKeyword, $name, $parameters, $attributes, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      modifierKeyword: $modifierKeyword as TerminalNode,
      name: $name as TerminalNode,
      parameters: $parameters === undefined ? undefined : new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new ModifierAttributes($attributes as NonterminalNode),
      body: new FunctionBody($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ModifierDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ModifierDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ModifierDefinition);
  }

  /**
   * Returns the child node that has the label `modifier_keyword`.
   */
  public get modifierKeyword(): TerminalNode {
    return this.fetch().modifierKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): ParametersDeclaration | undefined {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `attributes`.
   */
  public get attributes(): ModifierAttributes {
    return this.fetch().attributes;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): FunctionBody {
    return this.fetch().body;
  }
}

/**
 * This node represents a `ModifierInvocation` nonterminal, with the following structure:
 *
 * ```ebnf
 * ModifierInvocation = (* name: *) IdentifierPath
 *                      (* arguments: *) ArgumentsDeclaration?;
 * ```
 */
export class ModifierInvocation {
  private readonly fetch = once(() => {
    const [$name, $arguments] = wasm.ast.Selectors.sequence(this.cst);

    return {
      name: new IdentifierPath($name as NonterminalNode),
      arguments: $arguments === undefined ? undefined : new ArgumentsDeclaration($arguments as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ModifierInvocation`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ModifierInvocation`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ModifierInvocation);
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): IdentifierPath {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `arguments`.
   */
  public get arguments(): ArgumentsDeclaration | undefined {
    return this.fetch().arguments;
  }
}

/**
 * This node represents a `EventDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * EventDefinition = (* event_keyword: *) EVENT_KEYWORD
 *                   (* name: *) IDENTIFIER
 *                   (* parameters: *) EventParametersDeclaration
 *                   (* anonymous_keyword: *) ANONYMOUS_KEYWORD?
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
export class EventDefinition {
  private readonly fetch = once(() => {
    const [$eventKeyword, $name, $parameters, $anonymousKeyword, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      eventKeyword: $eventKeyword as TerminalNode,
      name: $name as TerminalNode,
      parameters: new EventParametersDeclaration($parameters as NonterminalNode),
      anonymousKeyword: $anonymousKeyword === undefined ? undefined : ($anonymousKeyword as TerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `EventDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `EventDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.EventDefinition);
  }

  /**
   * Returns the child node that has the label `event_keyword`.
   */
  public get eventKeyword(): TerminalNode {
    return this.fetch().eventKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): EventParametersDeclaration {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `anonymous_keyword`.
   */
  public get anonymousKeyword(): TerminalNode | undefined {
    return this.fetch().anonymousKeyword;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `EventParametersDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * EventParametersDeclaration = (* open_paren: *) OPEN_PAREN
 *                              (* parameters: *) EventParameters
 *                              (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class EventParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      parameters: new EventParameters($parameters as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `EventParametersDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `EventParametersDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.EventParametersDeclaration);
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): EventParameters {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `EventParameter` nonterminal, with the following structure:
 *
 * ```ebnf
 * EventParameter = (* type_name: *) TypeName
 *                  (* indexed_keyword: *) INDEXED_KEYWORD?
 *                  (* name: *) IDENTIFIER?;
 * ```
 */
export class EventParameter {
  private readonly fetch = once(() => {
    const [$typeName, $indexedKeyword, $name] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonterminalNode),
      indexedKeyword: $indexedKeyword === undefined ? undefined : ($indexedKeyword as TerminalNode),
      name: $name === undefined ? undefined : ($name as TerminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `EventParameter`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `EventParameter`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.EventParameter);
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `indexed_keyword`.
   */
  public get indexedKeyword(): TerminalNode | undefined {
    return this.fetch().indexedKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

/**
 * This node represents a `UserDefinedValueTypeDefinition` nonterminal, with the following structure:
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
export class UserDefinedValueTypeDefinition {
  private readonly fetch = once(() => {
    const [$typeKeyword, $name, $isKeyword, $valueType, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeKeyword: $typeKeyword as TerminalNode,
      name: $name as TerminalNode,
      isKeyword: $isKeyword as TerminalNode,
      valueType: new ElementaryType($valueType as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `UserDefinedValueTypeDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UserDefinedValueTypeDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UserDefinedValueTypeDefinition);
  }

  /**
   * Returns the child node that has the label `type_keyword`.
   */
  public get typeKeyword(): TerminalNode {
    return this.fetch().typeKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `is_keyword`.
   */
  public get isKeyword(): TerminalNode {
    return this.fetch().isKeyword;
  }

  /**
   * Returns the child node that has the label `value_type`.
   */
  public get valueType(): ElementaryType {
    return this.fetch().valueType;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `ErrorDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * ErrorDefinition = (* error_keyword: *) ERROR_KEYWORD
 *                   (* name: *) IDENTIFIER
 *                   (* members: *) ErrorParametersDeclaration
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
export class ErrorDefinition {
  private readonly fetch = once(() => {
    const [$errorKeyword, $name, $members, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      errorKeyword: $errorKeyword as TerminalNode,
      name: $name as TerminalNode,
      members: new ErrorParametersDeclaration($members as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ErrorDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ErrorDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ErrorDefinition);
  }

  /**
   * Returns the child node that has the label `error_keyword`.
   */
  public get errorKeyword(): TerminalNode {
    return this.fetch().errorKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `members`.
   */
  public get members(): ErrorParametersDeclaration {
    return this.fetch().members;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `ErrorParametersDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * ErrorParametersDeclaration = (* open_paren: *) OPEN_PAREN
 *                              (* parameters: *) ErrorParameters
 *                              (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class ErrorParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      parameters: new ErrorParameters($parameters as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ErrorParametersDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ErrorParametersDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ErrorParametersDeclaration);
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): ErrorParameters {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `ErrorParameter` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * ErrorParameter = (* type_name: *) TypeName
 *                  (* name: *) IDENTIFIER?;
 * ```
 */
export class ErrorParameter {
  private readonly fetch = once(() => {
    const [$typeName, $name] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonterminalNode),
      name: $name === undefined ? undefined : ($name as TerminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ErrorParameter`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ErrorParameter`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ErrorParameter);
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

/**
 * This node represents a `ArrayTypeName` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * ArrayTypeName = (* operand: *) TypeName
 *                 (* open_bracket: *) OPEN_BRACKET
 *                 (* index: *) Expression?
 *                 (* close_bracket: *) CLOSE_BRACKET;
 * ```
 */
export class ArrayTypeName {
  private readonly fetch = once(() => {
    const [$operand, $openBracket, $index, $closeBracket] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operand: new TypeName($operand as NonterminalNode),
      openBracket: $openBracket as TerminalNode,
      index: $index === undefined ? undefined : new Expression($index as NonterminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ArrayTypeName`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ArrayTypeName`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ArrayTypeName);
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): TypeName {
    return this.fetch().operand;
  }

  /**
   * Returns the child node that has the label `open_bracket`.
   */
  public get openBracket(): TerminalNode {
    return this.fetch().openBracket;
  }

  /**
   * Returns the child node that has the label `index`.
   */
  public get index(): Expression | undefined {
    return this.fetch().index;
  }

  /**
   * Returns the child node that has the label `close_bracket`.
   */
  public get closeBracket(): TerminalNode {
    return this.fetch().closeBracket;
  }
}

/**
 * This node represents a `FunctionType` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionType = (* function_keyword: *) FUNCTION_KEYWORD
 *                (* parameters: *) ParametersDeclaration
 *                (* attributes: *) FunctionTypeAttributes
 *                (* returns: *) ReturnsDeclaration?;
 * ```
 */
export class FunctionType {
  private readonly fetch = once(() => {
    const [$functionKeyword, $parameters, $attributes, $returns] = wasm.ast.Selectors.sequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TerminalNode,
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
      attributes: new FunctionTypeAttributes($attributes as NonterminalNode),
      returns: $returns === undefined ? undefined : new ReturnsDeclaration($returns as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `FunctionType`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FunctionType`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FunctionType);
  }

  /**
   * Returns the child node that has the label `function_keyword`.
   */
  public get functionKeyword(): TerminalNode {
    return this.fetch().functionKeyword;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `attributes`.
   */
  public get attributes(): FunctionTypeAttributes {
    return this.fetch().attributes;
  }

  /**
   * Returns the child node that has the label `returns`.
   */
  public get returns(): ReturnsDeclaration | undefined {
    return this.fetch().returns;
  }
}

/**
 * This node represents a `MappingType` nonterminal, with the following structure:
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
export class MappingType {
  private readonly fetch = once(() => {
    const [$mappingKeyword, $openParen, $keyType, $equalGreaterThan, $valueType, $closeParen] =
      wasm.ast.Selectors.sequence(this.cst);

    return {
      mappingKeyword: $mappingKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      keyType: new MappingKey($keyType as NonterminalNode),
      equalGreaterThan: $equalGreaterThan as TerminalNode,
      valueType: new MappingValue($valueType as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `MappingType`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `MappingType`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.MappingType);
  }

  /**
   * Returns the child node that has the label `mapping_keyword`.
   */
  public get mappingKeyword(): TerminalNode {
    return this.fetch().mappingKeyword;
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `key_type`.
   */
  public get keyType(): MappingKey {
    return this.fetch().keyType;
  }

  /**
   * Returns the child node that has the label `equal_greater_than`.
   */
  public get equalGreaterThan(): TerminalNode {
    return this.fetch().equalGreaterThan;
  }

  /**
   * Returns the child node that has the label `value_type`.
   */
  public get valueType(): MappingValue {
    return this.fetch().valueType;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `MappingKey` nonterminal, with the following structure:
 *
 * ```ebnf
 * MappingKey = (* key_type: *) MappingKeyType
 *              (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
 * ```
 */
export class MappingKey {
  private readonly fetch = once(() => {
    const [$keyType, $name] = wasm.ast.Selectors.sequence(this.cst);

    return {
      keyType: new MappingKeyType($keyType as NonterminalNode),
      name: $name === undefined ? undefined : ($name as TerminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `MappingKey`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `MappingKey`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.MappingKey);
  }

  /**
   * Returns the child node that has the label `key_type`.
   */
  public get keyType(): MappingKeyType {
    return this.fetch().keyType;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

/**
 * This node represents a `MappingValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * MappingValue = (* type_name: *) TypeName
 *                (* name: *) IDENTIFIER?; (* Introduced in 0.8.18 *)
 * ```
 */
export class MappingValue {
  private readonly fetch = once(() => {
    const [$typeName, $name] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonterminalNode),
      name: $name === undefined ? undefined : ($name as TerminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `MappingValue`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `MappingValue`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.MappingValue);
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }
}

/**
 * This node represents a `AddressType` nonterminal, with the following structure:
 *
 * ```ebnf
 * AddressType = (* address_keyword: *) ADDRESS_KEYWORD
 *               (* payable_keyword: *) PAYABLE_KEYWORD?; (* Introduced in 0.5.0 *)
 * ```
 */
export class AddressType {
  private readonly fetch = once(() => {
    const [$addressKeyword, $payableKeyword] = wasm.ast.Selectors.sequence(this.cst);

    return {
      addressKeyword: $addressKeyword as TerminalNode,
      payableKeyword: $payableKeyword === undefined ? undefined : ($payableKeyword as TerminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `AddressType`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AddressType`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AddressType);
  }

  /**
   * Returns the child node that has the label `address_keyword`.
   */
  public get addressKeyword(): TerminalNode {
    return this.fetch().addressKeyword;
  }

  /**
   * Returns the child node that has the label `payable_keyword`.
   */
  public get payableKeyword(): TerminalNode | undefined {
    return this.fetch().payableKeyword;
  }
}

/**
 * This node represents a `Block` nonterminal, with the following structure:
 *
 * ```ebnf
 * Block = (* open_brace: *) OPEN_BRACE
 *         (* statements: *) Statements
 *         (* close_brace: *) CLOSE_BRACE;
 * ```
 */
export class Block {
  private readonly fetch = once(() => {
    const [$openBrace, $statements, $closeBrace] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openBrace: $openBrace as TerminalNode,
      statements: new Statements($statements as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `Block`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Block`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Block);
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `statements`.
   */
  public get statements(): Statements {
    return this.fetch().statements;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `UncheckedBlock` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.0 *)
 * UncheckedBlock = (* unchecked_keyword: *) UNCHECKED_KEYWORD
 *                  (* block: *) Block;
 * ```
 */
export class UncheckedBlock {
  private readonly fetch = once(() => {
    const [$uncheckedKeyword, $block] = wasm.ast.Selectors.sequence(this.cst);

    return {
      uncheckedKeyword: $uncheckedKeyword as TerminalNode,
      block: new Block($block as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `UncheckedBlock`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UncheckedBlock`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UncheckedBlock);
  }

  /**
   * Returns the child node that has the label `unchecked_keyword`.
   */
  public get uncheckedKeyword(): TerminalNode {
    return this.fetch().uncheckedKeyword;
  }

  /**
   * Returns the child node that has the label `block`.
   */
  public get block(): Block {
    return this.fetch().block;
  }
}

/**
 * This node represents a `ExpressionStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * ExpressionStatement = (* expression: *) Expression
 *                       (* semicolon: *) SEMICOLON;
 * ```
 */
export class ExpressionStatement {
  private readonly fetch = once(() => {
    const [$expression, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      expression: new Expression($expression as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ExpressionStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ExpressionStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ExpressionStatement);
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): Expression {
    return this.fetch().expression;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `AssemblyStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * AssemblyStatement = (* assembly_keyword: *) ASSEMBLY_KEYWORD
 *                     (* label: *) StringLiteral?
 *                     (* flags: *) AssemblyFlagsDeclaration? (* Introduced in 0.8.13 *)
 *                     (* body: *) YulBlock;
 * ```
 */
export class AssemblyStatement {
  private readonly fetch = once(() => {
    const [$assemblyKeyword, $label, $flags, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      assemblyKeyword: $assemblyKeyword as TerminalNode,
      label: $label === undefined ? undefined : new StringLiteral($label as NonterminalNode),
      flags: $flags === undefined ? undefined : new AssemblyFlagsDeclaration($flags as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `AssemblyStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AssemblyStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AssemblyStatement);
  }

  /**
   * Returns the child node that has the label `assembly_keyword`.
   */
  public get assemblyKeyword(): TerminalNode {
    return this.fetch().assemblyKeyword;
  }

  /**
   * Returns the child node that has the label `label`.
   */
  public get label(): StringLiteral | undefined {
    return this.fetch().label;
  }

  /**
   * Returns the child node that has the label `flags`.
   */
  public get flags(): AssemblyFlagsDeclaration | undefined {
    return this.fetch().flags;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): YulBlock {
    return this.fetch().body;
  }
}

/**
 * This node represents a `AssemblyFlagsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * AssemblyFlagsDeclaration = (* open_paren: *) OPEN_PAREN
 *                            (* flags: *) AssemblyFlags
 *                            (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class AssemblyFlagsDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $flags, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      flags: new AssemblyFlags($flags as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `AssemblyFlagsDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AssemblyFlagsDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AssemblyFlagsDeclaration);
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `flags`.
   */
  public get flags(): AssemblyFlags {
    return this.fetch().flags;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `TupleDeconstructionStatement` nonterminal, with the following structure:
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
export class TupleDeconstructionStatement {
  private readonly fetch = once(() => {
    const [$varKeyword, $openParen, $elements, $closeParen, $equal, $expression, $semicolon] =
      wasm.ast.Selectors.sequence(this.cst);

    return {
      varKeyword: $varKeyword === undefined ? undefined : ($varKeyword as TerminalNode),
      openParen: $openParen as TerminalNode,
      elements: new TupleDeconstructionElements($elements as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
      equal: $equal as TerminalNode,
      expression: new Expression($expression as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `TupleDeconstructionStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TupleDeconstructionStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TupleDeconstructionStatement);
  }

  /**
   * Returns the child node that has the label `var_keyword`.
   */
  public get varKeyword(): TerminalNode | undefined {
    return this.fetch().varKeyword;
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `elements`.
   */
  public get elements(): TupleDeconstructionElements {
    return this.fetch().elements;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }

  /**
   * Returns the child node that has the label `equal`.
   */
  public get equal(): TerminalNode {
    return this.fetch().equal;
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): Expression {
    return this.fetch().expression;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `TupleDeconstructionElement` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleDeconstructionElement = (* member: *) TupleMember?;
 * ```
 */
export class TupleDeconstructionElement {
  private readonly fetch = once(() => {
    const [$member] = wasm.ast.Selectors.sequence(this.cst);

    return {
      member: $member === undefined ? undefined : new TupleMember($member as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `TupleDeconstructionElement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TupleDeconstructionElement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TupleDeconstructionElement);
  }

  /**
   * Returns the child node that has the label `member`.
   */
  public get member(): TupleMember | undefined {
    return this.fetch().member;
  }
}

/**
 * This node represents a `TypedTupleMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * TypedTupleMember = (* type_name: *) TypeName
 *                    (* storage_location: *) StorageLocation?
 *                    (* name: *) IDENTIFIER;
 * ```
 */
export class TypedTupleMember {
  private readonly fetch = once(() => {
    const [$typeName, $storageLocation, $name] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeName: new TypeName($typeName as NonterminalNode),
      storageLocation:
        $storageLocation === undefined ? undefined : new StorageLocation($storageLocation as NonterminalNode),
      name: $name as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `TypedTupleMember`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TypedTupleMember`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TypedTupleMember);
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `storage_location`.
   */
  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }
}

/**
 * This node represents a `UntypedTupleMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * UntypedTupleMember = (* storage_location: *) StorageLocation?
 *                      (* name: *) IDENTIFIER;
 * ```
 */
export class UntypedTupleMember {
  private readonly fetch = once(() => {
    const [$storageLocation, $name] = wasm.ast.Selectors.sequence(this.cst);

    return {
      storageLocation:
        $storageLocation === undefined ? undefined : new StorageLocation($storageLocation as NonterminalNode),
      name: $name as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `UntypedTupleMember`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UntypedTupleMember`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UntypedTupleMember);
  }

  /**
   * Returns the child node that has the label `storage_location`.
   */
  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }
}

/**
 * This node represents a `VariableDeclarationStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * VariableDeclarationStatement = (* variable_type: *) VariableDeclarationType
 *                                (* storage_location: *) StorageLocation?
 *                                (* name: *) IDENTIFIER
 *                                (* value: *) VariableDeclarationValue?
 *                                (* semicolon: *) SEMICOLON;
 * ```
 */
export class VariableDeclarationStatement {
  private readonly fetch = once(() => {
    const [$variableType, $storageLocation, $name, $value, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      variableType: new VariableDeclarationType($variableType as NonterminalNode),
      storageLocation:
        $storageLocation === undefined ? undefined : new StorageLocation($storageLocation as NonterminalNode),
      name: $name as TerminalNode,
      value: $value === undefined ? undefined : new VariableDeclarationValue($value as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `VariableDeclarationStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VariableDeclarationStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VariableDeclarationStatement);
  }

  /**
   * Returns the child node that has the label `variable_type`.
   */
  public get variableType(): VariableDeclarationType {
    return this.fetch().variableType;
  }

  /**
   * Returns the child node that has the label `storage_location`.
   */
  public get storageLocation(): StorageLocation | undefined {
    return this.fetch().storageLocation;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `value`.
   */
  public get value(): VariableDeclarationValue | undefined {
    return this.fetch().value;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `VariableDeclarationValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * VariableDeclarationValue = (* equal: *) EQUAL
 *                            (* expression: *) Expression;
 * ```
 */
export class VariableDeclarationValue {
  private readonly fetch = once(() => {
    const [$equal, $expression] = wasm.ast.Selectors.sequence(this.cst);

    return {
      equal: $equal as TerminalNode,
      expression: new Expression($expression as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `VariableDeclarationValue`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VariableDeclarationValue`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VariableDeclarationValue);
  }

  /**
   * Returns the child node that has the label `equal`.
   */
  public get equal(): TerminalNode {
    return this.fetch().equal;
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): Expression {
    return this.fetch().expression;
  }
}

/**
 * This node represents a `IfStatement` nonterminal, with the following structure:
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
export class IfStatement {
  private readonly fetch = once(() => {
    const [$ifKeyword, $openParen, $condition, $closeParen, $body, $elseBranch] = wasm.ast.Selectors.sequence(this.cst);

    return {
      ifKeyword: $ifKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      condition: new Expression($condition as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
      body: new Statement($body as NonterminalNode),
      elseBranch: $elseBranch === undefined ? undefined : new ElseBranch($elseBranch as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `IfStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `IfStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.IfStatement);
  }

  /**
   * Returns the child node that has the label `if_keyword`.
   */
  public get ifKeyword(): TerminalNode {
    return this.fetch().ifKeyword;
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `condition`.
   */
  public get condition(): Expression {
    return this.fetch().condition;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): Statement {
    return this.fetch().body;
  }

  /**
   * Returns the child node that has the label `else_branch`.
   */
  public get elseBranch(): ElseBranch | undefined {
    return this.fetch().elseBranch;
  }
}

/**
 * This node represents a `ElseBranch` nonterminal, with the following structure:
 *
 * ```ebnf
 * ElseBranch = (* else_keyword: *) ELSE_KEYWORD
 *              (* body: *) Statement;
 * ```
 */
export class ElseBranch {
  private readonly fetch = once(() => {
    const [$elseKeyword, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      elseKeyword: $elseKeyword as TerminalNode,
      body: new Statement($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ElseBranch`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ElseBranch`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ElseBranch);
  }

  /**
   * Returns the child node that has the label `else_keyword`.
   */
  public get elseKeyword(): TerminalNode {
    return this.fetch().elseKeyword;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): Statement {
    return this.fetch().body;
  }
}

/**
 * This node represents a `ForStatement` nonterminal, with the following structure:
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
export class ForStatement {
  private readonly fetch = once(() => {
    const [$forKeyword, $openParen, $initialization, $condition, $iterator, $closeParen, $body] =
      wasm.ast.Selectors.sequence(this.cst);

    return {
      forKeyword: $forKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      initialization: new ForStatementInitialization($initialization as NonterminalNode),
      condition: new ForStatementCondition($condition as NonterminalNode),
      iterator: $iterator === undefined ? undefined : new Expression($iterator as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
      body: new Statement($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ForStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ForStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ForStatement);
  }

  /**
   * Returns the child node that has the label `for_keyword`.
   */
  public get forKeyword(): TerminalNode {
    return this.fetch().forKeyword;
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `initialization`.
   */
  public get initialization(): ForStatementInitialization {
    return this.fetch().initialization;
  }

  /**
   * Returns the child node that has the label `condition`.
   */
  public get condition(): ForStatementCondition {
    return this.fetch().condition;
  }

  /**
   * Returns the child node that has the label `iterator`.
   */
  public get iterator(): Expression | undefined {
    return this.fetch().iterator;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): Statement {
    return this.fetch().body;
  }
}

/**
 * This node represents a `WhileStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * WhileStatement = (* while_keyword: *) WHILE_KEYWORD
 *                  (* open_paren: *) OPEN_PAREN
 *                  (* condition: *) Expression
 *                  (* close_paren: *) CLOSE_PAREN
 *                  (* body: *) Statement;
 * ```
 */
export class WhileStatement {
  private readonly fetch = once(() => {
    const [$whileKeyword, $openParen, $condition, $closeParen, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      whileKeyword: $whileKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      condition: new Expression($condition as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
      body: new Statement($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `WhileStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `WhileStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.WhileStatement);
  }

  /**
   * Returns the child node that has the label `while_keyword`.
   */
  public get whileKeyword(): TerminalNode {
    return this.fetch().whileKeyword;
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `condition`.
   */
  public get condition(): Expression {
    return this.fetch().condition;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): Statement {
    return this.fetch().body;
  }
}

/**
 * This node represents a `DoWhileStatement` nonterminal, with the following structure:
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
export class DoWhileStatement {
  private readonly fetch = once(() => {
    const [$doKeyword, $body, $whileKeyword, $openParen, $condition, $closeParen, $semicolon] =
      wasm.ast.Selectors.sequence(this.cst);

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

  /**
   * Constructs a new AST node of type `DoWhileStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `DoWhileStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.DoWhileStatement);
  }

  /**
   * Returns the child node that has the label `do_keyword`.
   */
  public get doKeyword(): TerminalNode {
    return this.fetch().doKeyword;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): Statement {
    return this.fetch().body;
  }

  /**
   * Returns the child node that has the label `while_keyword`.
   */
  public get whileKeyword(): TerminalNode {
    return this.fetch().whileKeyword;
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `condition`.
   */
  public get condition(): Expression {
    return this.fetch().condition;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `ContinueStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * ContinueStatement = (* continue_keyword: *) CONTINUE_KEYWORD
 *                     (* semicolon: *) SEMICOLON;
 * ```
 */
export class ContinueStatement {
  private readonly fetch = once(() => {
    const [$continueKeyword, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      continueKeyword: $continueKeyword as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ContinueStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ContinueStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ContinueStatement);
  }

  /**
   * Returns the child node that has the label `continue_keyword`.
   */
  public get continueKeyword(): TerminalNode {
    return this.fetch().continueKeyword;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `BreakStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * BreakStatement = (* break_keyword: *) BREAK_KEYWORD
 *                  (* semicolon: *) SEMICOLON;
 * ```
 */
export class BreakStatement {
  private readonly fetch = once(() => {
    const [$breakKeyword, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      breakKeyword: $breakKeyword as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `BreakStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `BreakStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.BreakStatement);
  }

  /**
   * Returns the child node that has the label `break_keyword`.
   */
  public get breakKeyword(): TerminalNode {
    return this.fetch().breakKeyword;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `ReturnStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * ReturnStatement = (* return_keyword: *) RETURN_KEYWORD
 *                   (* expression: *) Expression?
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
export class ReturnStatement {
  private readonly fetch = once(() => {
    const [$returnKeyword, $expression, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      returnKeyword: $returnKeyword as TerminalNode,
      expression: $expression === undefined ? undefined : new Expression($expression as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ReturnStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ReturnStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ReturnStatement);
  }

  /**
   * Returns the child node that has the label `return_keyword`.
   */
  public get returnKeyword(): TerminalNode {
    return this.fetch().returnKeyword;
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): Expression | undefined {
    return this.fetch().expression;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `EmitStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.4.21 *)
 * EmitStatement = (* emit_keyword: *) EMIT_KEYWORD
 *                 (* event: *) IdentifierPath
 *                 (* arguments: *) ArgumentsDeclaration
 *                 (* semicolon: *) SEMICOLON;
 * ```
 */
export class EmitStatement {
  private readonly fetch = once(() => {
    const [$emitKeyword, $event, $arguments, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      emitKeyword: $emitKeyword as TerminalNode,
      event: new IdentifierPath($event as NonterminalNode),
      arguments: new ArgumentsDeclaration($arguments as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `EmitStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `EmitStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.EmitStatement);
  }

  /**
   * Returns the child node that has the label `emit_keyword`.
   */
  public get emitKeyword(): TerminalNode {
    return this.fetch().emitKeyword;
  }

  /**
   * Returns the child node that has the label `event`.
   */
  public get event(): IdentifierPath {
    return this.fetch().event;
  }

  /**
   * Returns the child node that has the label `arguments`.
   */
  public get arguments(): ArgumentsDeclaration {
    return this.fetch().arguments;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `TryStatement` nonterminal, with the following structure:
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
export class TryStatement {
  private readonly fetch = once(() => {
    const [$tryKeyword, $expression, $returns, $body, $catchClauses] = wasm.ast.Selectors.sequence(this.cst);

    return {
      tryKeyword: $tryKeyword as TerminalNode,
      expression: new Expression($expression as NonterminalNode),
      returns: $returns === undefined ? undefined : new ReturnsDeclaration($returns as NonterminalNode),
      body: new Block($body as NonterminalNode),
      catchClauses: new CatchClauses($catchClauses as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `TryStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TryStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TryStatement);
  }

  /**
   * Returns the child node that has the label `try_keyword`.
   */
  public get tryKeyword(): TerminalNode {
    return this.fetch().tryKeyword;
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): Expression {
    return this.fetch().expression;
  }

  /**
   * Returns the child node that has the label `returns`.
   */
  public get returns(): ReturnsDeclaration | undefined {
    return this.fetch().returns;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): Block {
    return this.fetch().body;
  }

  /**
   * Returns the child node that has the label `catch_clauses`.
   */
  public get catchClauses(): CatchClauses {
    return this.fetch().catchClauses;
  }
}

/**
 * This node represents a `CatchClause` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * CatchClause = (* catch_keyword: *) CATCH_KEYWORD
 *               (* error: *) CatchClauseError?
 *               (* body: *) Block;
 * ```
 */
export class CatchClause {
  private readonly fetch = once(() => {
    const [$catchKeyword, $error, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      catchKeyword: $catchKeyword as TerminalNode,
      error: $error === undefined ? undefined : new CatchClauseError($error as NonterminalNode),
      body: new Block($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `CatchClause`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `CatchClause`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.CatchClause);
  }

  /**
   * Returns the child node that has the label `catch_keyword`.
   */
  public get catchKeyword(): TerminalNode {
    return this.fetch().catchKeyword;
  }

  /**
   * Returns the child node that has the label `error`.
   */
  public get error(): CatchClauseError | undefined {
    return this.fetch().error;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): Block {
    return this.fetch().body;
  }
}

/**
 * This node represents a `CatchClauseError` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * CatchClauseError = (* name: *) IDENTIFIER?
 *                    (* parameters: *) ParametersDeclaration;
 * ```
 */
export class CatchClauseError {
  private readonly fetch = once(() => {
    const [$name, $parameters] = wasm.ast.Selectors.sequence(this.cst);

    return {
      name: $name === undefined ? undefined : ($name as TerminalNode),
      parameters: new ParametersDeclaration($parameters as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `CatchClauseError`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `CatchClauseError`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.CatchClauseError);
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): ParametersDeclaration {
    return this.fetch().parameters;
  }
}

/**
 * This node represents a `RevertStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * RevertStatement = (* revert_keyword: *) REVERT_KEYWORD
 *                   (* error: *) IdentifierPath?
 *                   (* arguments: *) ArgumentsDeclaration
 *                   (* semicolon: *) SEMICOLON;
 * ```
 */
export class RevertStatement {
  private readonly fetch = once(() => {
    const [$revertKeyword, $error, $arguments, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      revertKeyword: $revertKeyword as TerminalNode,
      error: $error === undefined ? undefined : new IdentifierPath($error as NonterminalNode),
      arguments: new ArgumentsDeclaration($arguments as NonterminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `RevertStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `RevertStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.RevertStatement);
  }

  /**
   * Returns the child node that has the label `revert_keyword`.
   */
  public get revertKeyword(): TerminalNode {
    return this.fetch().revertKeyword;
  }

  /**
   * Returns the child node that has the label `error`.
   */
  public get error(): IdentifierPath | undefined {
    return this.fetch().error;
  }

  /**
   * Returns the child node that has the label `arguments`.
   */
  public get arguments(): ArgumentsDeclaration {
    return this.fetch().arguments;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `ThrowStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * ThrowStatement = (* throw_keyword: *) THROW_KEYWORD
 *                  (* semicolon: *) SEMICOLON;
 * ```
 */
export class ThrowStatement {
  private readonly fetch = once(() => {
    const [$throwKeyword, $semicolon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      throwKeyword: $throwKeyword as TerminalNode,
      semicolon: $semicolon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ThrowStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ThrowStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ThrowStatement);
  }

  /**
   * Returns the child node that has the label `throw_keyword`.
   */
  public get throwKeyword(): TerminalNode {
    return this.fetch().throwKeyword;
  }

  /**
   * Returns the child node that has the label `semicolon`.
   */
  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

/**
 * This node represents a `AssignmentExpression` nonterminal, with the following structure:
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
export class AssignmentExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `AssignmentExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AssignmentExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AssignmentExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `ConditionalExpression` nonterminal, with the following structure:
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
export class ConditionalExpression {
  private readonly fetch = once(() => {
    const [$operand, $questionMark, $trueExpression, $colon, $falseExpression] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operand: new Expression($operand as NonterminalNode),
      questionMark: $questionMark as TerminalNode,
      trueExpression: new Expression($trueExpression as NonterminalNode),
      colon: $colon as TerminalNode,
      falseExpression: new Expression($falseExpression as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ConditionalExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ConditionalExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ConditionalExpression);
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): Expression {
    return this.fetch().operand;
  }

  /**
   * Returns the child node that has the label `question_mark`.
   */
  public get questionMark(): TerminalNode {
    return this.fetch().questionMark;
  }

  /**
   * Returns the child node that has the label `true_expression`.
   */
  public get trueExpression(): Expression {
    return this.fetch().trueExpression;
  }

  /**
   * Returns the child node that has the label `colon`.
   */
  public get colon(): TerminalNode {
    return this.fetch().colon;
  }

  /**
   * Returns the child node that has the label `false_expression`.
   */
  public get falseExpression(): Expression {
    return this.fetch().falseExpression;
  }
}

/**
 * This node represents a `OrExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * OrExpression = (* left_operand: *) Expression
 *                (* operator: *) BAR_BAR
 *                (* right_operand: *) Expression;
 * ```
 */
export class OrExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `OrExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `OrExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.OrExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `AndExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * AndExpression = (* left_operand: *) Expression
 *                 (* operator: *) AMPERSAND_AMPERSAND
 *                 (* right_operand: *) Expression;
 * ```
 */
export class AndExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `AndExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AndExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AndExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `EqualityExpression` nonterminal, with the following structure:
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
export class EqualityExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `EqualityExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `EqualityExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.EqualityExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `InequalityExpression` nonterminal, with the following structure:
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
export class InequalityExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `InequalityExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `InequalityExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.InequalityExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `BitwiseOrExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * BitwiseOrExpression = (* left_operand: *) Expression
 *                       (* operator: *) BAR
 *                       (* right_operand: *) Expression;
 * ```
 */
export class BitwiseOrExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `BitwiseOrExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `BitwiseOrExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.BitwiseOrExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `BitwiseXorExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * BitwiseXorExpression = (* left_operand: *) Expression
 *                        (* operator: *) CARET
 *                        (* right_operand: *) Expression;
 * ```
 */
export class BitwiseXorExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `BitwiseXorExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `BitwiseXorExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.BitwiseXorExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `BitwiseAndExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * BitwiseAndExpression = (* left_operand: *) Expression
 *                        (* operator: *) AMPERSAND
 *                        (* right_operand: *) Expression;
 * ```
 */
export class BitwiseAndExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `BitwiseAndExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `BitwiseAndExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.BitwiseAndExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `ShiftExpression` nonterminal, with the following structure:
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
export class ShiftExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ShiftExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ShiftExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ShiftExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `AdditiveExpression` nonterminal, with the following structure:
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
export class AdditiveExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `AdditiveExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AdditiveExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AdditiveExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `MultiplicativeExpression` nonterminal, with the following structure:
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
export class MultiplicativeExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `MultiplicativeExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `MultiplicativeExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.MultiplicativeExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `ExponentiationExpression` nonterminal, with the following structure:
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
export class ExponentiationExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonterminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `ExponentiationExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ExponentiationExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ExponentiationExpression);
  }

  /**
   * Returns the child node that has the label `left_operand`.
   */
  public get leftOperand(): Expression {
    return this.fetch().leftOperand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `right_operand`.
   */
  public get rightOperand(): Expression {
    return this.fetch().rightOperand;
  }
}

/**
 * This node represents a `PostfixExpression` nonterminal, with the following structure:
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
export class PostfixExpression {
  private readonly fetch = once(() => {
    const [$operand, $operator] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operand: new Expression($operand as NonterminalNode),
      operator: $operator as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `PostfixExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `PostfixExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.PostfixExpression);
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): Expression {
    return this.fetch().operand;
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }
}

/**
 * This node represents a `PrefixExpression` nonterminal, with the following structure:
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
export class PrefixExpression {
  private readonly fetch = once(() => {
    const [$operator, $operand] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operator: $operator as TerminalNode,
      operand: new Expression($operand as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `PrefixExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `PrefixExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.PrefixExpression);
  }

  /**
   * Returns the child node that has the label `operator`.
   */
  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): Expression {
    return this.fetch().operand;
  }
}

/**
 * This node represents a `FunctionCallExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * FunctionCallExpression = (* operand: *) Expression
 *                          (* arguments: *) ArgumentsDeclaration;
 * ```
 */
export class FunctionCallExpression {
  private readonly fetch = once(() => {
    const [$operand, $arguments] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operand: new Expression($operand as NonterminalNode),
      arguments: new ArgumentsDeclaration($arguments as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `FunctionCallExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FunctionCallExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FunctionCallExpression);
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): Expression {
    return this.fetch().operand;
  }

  /**
   * Returns the child node that has the label `arguments`.
   */
  public get arguments(): ArgumentsDeclaration {
    return this.fetch().arguments;
  }
}

/**
 * This node represents a `CallOptionsExpression` nonterminal, with the following structure:
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
export class CallOptionsExpression {
  private readonly fetch = once(() => {
    const [$operand, $openBrace, $options, $closeBrace] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operand: new Expression($operand as NonterminalNode),
      openBrace: $openBrace as TerminalNode,
      options: new CallOptions($options as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `CallOptionsExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `CallOptionsExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.CallOptionsExpression);
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): Expression {
    return this.fetch().operand;
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `options`.
   */
  public get options(): CallOptions {
    return this.fetch().options;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `MemberAccessExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * MemberAccessExpression = (* operand: *) Expression
 *                          (* period: *) PERIOD
 *                          (* member: *) IDENTIFIER;
 * ```
 */
export class MemberAccessExpression {
  private readonly fetch = once(() => {
    const [$operand, $period, $member] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operand: new Expression($operand as NonterminalNode),
      period: $period as TerminalNode,
      member: $member as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `MemberAccessExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `MemberAccessExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.MemberAccessExpression);
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): Expression {
    return this.fetch().operand;
  }

  /**
   * Returns the child node that has the label `period`.
   */
  public get period(): TerminalNode {
    return this.fetch().period;
  }

  /**
   * Returns the child node that has the label `member`.
   */
  public get member(): TerminalNode {
    return this.fetch().member;
  }
}

/**
 * This node represents a `IndexAccessExpression` nonterminal, with the following structure:
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
export class IndexAccessExpression {
  private readonly fetch = once(() => {
    const [$operand, $openBracket, $start, $end, $closeBracket] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operand: new Expression($operand as NonterminalNode),
      openBracket: $openBracket as TerminalNode,
      start: $start === undefined ? undefined : new Expression($start as NonterminalNode),
      end: $end === undefined ? undefined : new IndexAccessEnd($end as NonterminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `IndexAccessExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `IndexAccessExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.IndexAccessExpression);
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): Expression {
    return this.fetch().operand;
  }

  /**
   * Returns the child node that has the label `open_bracket`.
   */
  public get openBracket(): TerminalNode {
    return this.fetch().openBracket;
  }

  /**
   * Returns the child node that has the label `start`.
   */
  public get start(): Expression | undefined {
    return this.fetch().start;
  }

  /**
   * Returns the child node that has the label `end`.
   */
  public get end(): IndexAccessEnd | undefined {
    return this.fetch().end;
  }

  /**
   * Returns the child node that has the label `close_bracket`.
   */
  public get closeBracket(): TerminalNode {
    return this.fetch().closeBracket;
  }
}

/**
 * This node represents a `IndexAccessEnd` nonterminal, with the following structure:
 *
 * ```ebnf
 * IndexAccessEnd = (* colon: *) COLON
 *                  (* end: *) Expression?;
 * ```
 */
export class IndexAccessEnd {
  private readonly fetch = once(() => {
    const [$colon, $end] = wasm.ast.Selectors.sequence(this.cst);

    return {
      colon: $colon as TerminalNode,
      end: $end === undefined ? undefined : new Expression($end as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `IndexAccessEnd`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `IndexAccessEnd`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.IndexAccessEnd);
  }

  /**
   * Returns the child node that has the label `colon`.
   */
  public get colon(): TerminalNode {
    return this.fetch().colon;
  }

  /**
   * Returns the child node that has the label `end`.
   */
  public get end(): Expression | undefined {
    return this.fetch().end;
  }
}

/**
 * This node represents a `PositionalArgumentsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * PositionalArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
 *                                  (* arguments: *) PositionalArguments
 *                                  (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class PositionalArgumentsDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $arguments, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      arguments: new PositionalArguments($arguments as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `PositionalArgumentsDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `PositionalArgumentsDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.PositionalArgumentsDeclaration);
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `arguments`.
   */
  public get arguments(): PositionalArguments {
    return this.fetch().arguments;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `NamedArgumentsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedArgumentsDeclaration = (* open_paren: *) OPEN_PAREN
 *                             (* arguments: *) NamedArgumentGroup?
 *                             (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class NamedArgumentsDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $arguments, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      arguments: $arguments === undefined ? undefined : new NamedArgumentGroup($arguments as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `NamedArgumentsDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `NamedArgumentsDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.NamedArgumentsDeclaration);
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `arguments`.
   */
  public get arguments(): NamedArgumentGroup | undefined {
    return this.fetch().arguments;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `NamedArgumentGroup` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedArgumentGroup = (* open_brace: *) OPEN_BRACE
 *                      (* arguments: *) NamedArguments
 *                      (* close_brace: *) CLOSE_BRACE;
 * ```
 */
export class NamedArgumentGroup {
  private readonly fetch = once(() => {
    const [$openBrace, $arguments, $closeBrace] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openBrace: $openBrace as TerminalNode,
      arguments: new NamedArguments($arguments as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `NamedArgumentGroup`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `NamedArgumentGroup`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.NamedArgumentGroup);
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `arguments`.
   */
  public get arguments(): NamedArguments {
    return this.fetch().arguments;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `NamedArgument` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedArgument = (* name: *) IDENTIFIER
 *                 (* colon: *) COLON
 *                 (* value: *) Expression;
 * ```
 */
export class NamedArgument {
  private readonly fetch = once(() => {
    const [$name, $colon, $value] = wasm.ast.Selectors.sequence(this.cst);

    return {
      name: $name as TerminalNode,
      colon: $colon as TerminalNode,
      value: new Expression($value as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `NamedArgument`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `NamedArgument`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.NamedArgument);
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `colon`.
   */
  public get colon(): TerminalNode {
    return this.fetch().colon;
  }

  /**
   * Returns the child node that has the label `value`.
   */
  public get value(): Expression {
    return this.fetch().value;
  }
}

/**
 * This node represents a `TypeExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.5.3 *)
 * TypeExpression = (* type_keyword: *) TYPE_KEYWORD
 *                  (* open_paren: *) OPEN_PAREN
 *                  (* type_name: *) TypeName
 *                  (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class TypeExpression {
  private readonly fetch = once(() => {
    const [$typeKeyword, $openParen, $typeName, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      typeKeyword: $typeKeyword as TerminalNode,
      openParen: $openParen as TerminalNode,
      typeName: new TypeName($typeName as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `TypeExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TypeExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TypeExpression);
  }

  /**
   * Returns the child node that has the label `type_keyword`.
   */
  public get typeKeyword(): TerminalNode {
    return this.fetch().typeKeyword;
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `NewExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * NewExpression = (* new_keyword: *) NEW_KEYWORD
 *                 (* type_name: *) TypeName;
 * ```
 */
export class NewExpression {
  private readonly fetch = once(() => {
    const [$newKeyword, $typeName] = wasm.ast.Selectors.sequence(this.cst);

    return {
      newKeyword: $newKeyword as TerminalNode,
      typeName: new TypeName($typeName as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `NewExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `NewExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.NewExpression);
  }

  /**
   * Returns the child node that has the label `new_keyword`.
   */
  public get newKeyword(): TerminalNode {
    return this.fetch().newKeyword;
  }

  /**
   * Returns the child node that has the label `type_name`.
   */
  public get typeName(): TypeName {
    return this.fetch().typeName;
  }
}

/**
 * This node represents a `TupleExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleExpression = (* open_paren: *) OPEN_PAREN
 *                   (* items: *) TupleValues
 *                   (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class TupleExpression {
  private readonly fetch = once(() => {
    const [$openParen, $items, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      items: new TupleValues($items as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `TupleExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TupleExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TupleExpression);
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `items`.
   */
  public get items(): TupleValues {
    return this.fetch().items;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `TupleValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleValue = (* expression: *) Expression?;
 * ```
 */
export class TupleValue {
  private readonly fetch = once(() => {
    const [$expression] = wasm.ast.Selectors.sequence(this.cst);

    return {
      expression: $expression === undefined ? undefined : new Expression($expression as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `TupleValue`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TupleValue`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TupleValue);
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): Expression | undefined {
    return this.fetch().expression;
  }
}

/**
 * This node represents a `ArrayExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * ArrayExpression = (* open_bracket: *) OPEN_BRACKET
 *                   (* items: *) ArrayValues
 *                   (* close_bracket: *) CLOSE_BRACKET;
 * ```
 */
export class ArrayExpression {
  private readonly fetch = once(() => {
    const [$openBracket, $items, $closeBracket] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openBracket: $openBracket as TerminalNode,
      items: new ArrayValues($items as NonterminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `ArrayExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ArrayExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ArrayExpression);
  }

  /**
   * Returns the child node that has the label `open_bracket`.
   */
  public get openBracket(): TerminalNode {
    return this.fetch().openBracket;
  }

  /**
   * Returns the child node that has the label `items`.
   */
  public get items(): ArrayValues {
    return this.fetch().items;
  }

  /**
   * Returns the child node that has the label `close_bracket`.
   */
  public get closeBracket(): TerminalNode {
    return this.fetch().closeBracket;
  }
}

/**
 * This node represents a `HexNumberExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * HexNumberExpression = (* literal: *) HEX_LITERAL
 *                       (* unit: *) NumberUnit?; (* Deprecated in 0.5.0 *)
 * ```
 */
export class HexNumberExpression {
  private readonly fetch = once(() => {
    const [$literal, $unit] = wasm.ast.Selectors.sequence(this.cst);

    return {
      literal: $literal as TerminalNode,
      unit: $unit === undefined ? undefined : new NumberUnit($unit as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `HexNumberExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `HexNumberExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.HexNumberExpression);
  }

  /**
   * Returns the child node that has the label `literal`.
   */
  public get literal(): TerminalNode {
    return this.fetch().literal;
  }

  /**
   * Returns the child node that has the label `unit`.
   */
  public get unit(): NumberUnit | undefined {
    return this.fetch().unit;
  }
}

/**
 * This node represents a `DecimalNumberExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * DecimalNumberExpression = (* literal: *) DECIMAL_LITERAL
 *                           (* unit: *) NumberUnit?;
 * ```
 */
export class DecimalNumberExpression {
  private readonly fetch = once(() => {
    const [$literal, $unit] = wasm.ast.Selectors.sequence(this.cst);

    return {
      literal: $literal as TerminalNode,
      unit: $unit === undefined ? undefined : new NumberUnit($unit as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `DecimalNumberExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `DecimalNumberExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.DecimalNumberExpression);
  }

  /**
   * Returns the child node that has the label `literal`.
   */
  public get literal(): TerminalNode {
    return this.fetch().literal;
  }

  /**
   * Returns the child node that has the label `unit`.
   */
  public get unit(): NumberUnit | undefined {
    return this.fetch().unit;
  }
}

/**
 * This node represents a `YulBlock` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulBlock = (* open_brace: *) OPEN_BRACE
 *            (* statements: *) YulStatements
 *            (* close_brace: *) CLOSE_BRACE;
 * ```
 */
export class YulBlock {
  private readonly fetch = once(() => {
    const [$openBrace, $statements, $closeBrace] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openBrace: $openBrace as TerminalNode,
      statements: new YulStatements($statements as NonterminalNode),
      closeBrace: $closeBrace as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulBlock`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulBlock`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulBlock);
  }

  /**
   * Returns the child node that has the label `open_brace`.
   */
  public get openBrace(): TerminalNode {
    return this.fetch().openBrace;
  }

  /**
   * Returns the child node that has the label `statements`.
   */
  public get statements(): YulStatements {
    return this.fetch().statements;
  }

  /**
   * Returns the child node that has the label `close_brace`.
   */
  public get closeBrace(): TerminalNode {
    return this.fetch().closeBrace;
  }
}

/**
 * This node represents a `YulFunctionDefinition` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulFunctionDefinition = (* function_keyword: *) YUL_FUNCTION_KEYWORD
 *                         (* name: *) YUL_IDENTIFIER
 *                         (* parameters: *) YulParametersDeclaration
 *                         (* returns: *) YulReturnsDeclaration?
 *                         (* body: *) YulBlock;
 * ```
 */
export class YulFunctionDefinition {
  private readonly fetch = once(() => {
    const [$functionKeyword, $name, $parameters, $returns, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      functionKeyword: $functionKeyword as TerminalNode,
      name: $name as TerminalNode,
      parameters: new YulParametersDeclaration($parameters as NonterminalNode),
      returns: $returns === undefined ? undefined : new YulReturnsDeclaration($returns as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulFunctionDefinition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulFunctionDefinition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulFunctionDefinition);
  }

  /**
   * Returns the child node that has the label `function_keyword`.
   */
  public get functionKeyword(): TerminalNode {
    return this.fetch().functionKeyword;
  }

  /**
   * Returns the child node that has the label `name`.
   */
  public get name(): TerminalNode {
    return this.fetch().name;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): YulParametersDeclaration {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `returns`.
   */
  public get returns(): YulReturnsDeclaration | undefined {
    return this.fetch().returns;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): YulBlock {
    return this.fetch().body;
  }
}

/**
 * This node represents a `YulParametersDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulParametersDeclaration = (* open_paren: *) OPEN_PAREN
 *                            (* parameters: *) YulParameters
 *                            (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class YulParametersDeclaration {
  private readonly fetch = once(() => {
    const [$openParen, $parameters, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      openParen: $openParen as TerminalNode,
      parameters: new YulParameters($parameters as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulParametersDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulParametersDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulParametersDeclaration);
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `parameters`.
   */
  public get parameters(): YulParameters {
    return this.fetch().parameters;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

/**
 * This node represents a `YulReturnsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulReturnsDeclaration = (* minus_greater_than: *) MINUS_GREATER_THAN
 *                         (* variables: *) YulVariableNames;
 * ```
 */
export class YulReturnsDeclaration {
  private readonly fetch = once(() => {
    const [$minusGreaterThan, $variables] = wasm.ast.Selectors.sequence(this.cst);

    return {
      minusGreaterThan: $minusGreaterThan as TerminalNode,
      variables: new YulVariableNames($variables as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulReturnsDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulReturnsDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulReturnsDeclaration);
  }

  /**
   * Returns the child node that has the label `minus_greater_than`.
   */
  public get minusGreaterThan(): TerminalNode {
    return this.fetch().minusGreaterThan;
  }

  /**
   * Returns the child node that has the label `variables`.
   */
  public get variables(): YulVariableNames {
    return this.fetch().variables;
  }
}

/**
 * This node represents a `YulVariableDeclarationStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulVariableDeclarationStatement = (* let_keyword: *) YUL_LET_KEYWORD
 *                                   (* variables: *) YulVariableNames
 *                                   (* value: *) YulVariableDeclarationValue?;
 * ```
 */
export class YulVariableDeclarationStatement {
  private readonly fetch = once(() => {
    const [$letKeyword, $variables, $value] = wasm.ast.Selectors.sequence(this.cst);

    return {
      letKeyword: $letKeyword as TerminalNode,
      variables: new YulVariableNames($variables as NonterminalNode),
      value: $value === undefined ? undefined : new YulVariableDeclarationValue($value as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulVariableDeclarationStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulVariableDeclarationStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulVariableDeclarationStatement);
  }

  /**
   * Returns the child node that has the label `let_keyword`.
   */
  public get letKeyword(): TerminalNode {
    return this.fetch().letKeyword;
  }

  /**
   * Returns the child node that has the label `variables`.
   */
  public get variables(): YulVariableNames {
    return this.fetch().variables;
  }

  /**
   * Returns the child node that has the label `value`.
   */
  public get value(): YulVariableDeclarationValue | undefined {
    return this.fetch().value;
  }
}

/**
 * This node represents a `YulVariableDeclarationValue` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulVariableDeclarationValue = (* assignment: *) YulAssignmentOperator
 *                               (* expression: *) YulExpression;
 * ```
 */
export class YulVariableDeclarationValue {
  private readonly fetch = once(() => {
    const [$assignment, $expression] = wasm.ast.Selectors.sequence(this.cst);

    return {
      assignment: new YulAssignmentOperator($assignment as NonterminalNode),
      expression: new YulExpression($expression as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulVariableDeclarationValue`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulVariableDeclarationValue`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulVariableDeclarationValue);
  }

  /**
   * Returns the child node that has the label `assignment`.
   */
  public get assignment(): YulAssignmentOperator {
    return this.fetch().assignment;
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): YulExpression {
    return this.fetch().expression;
  }
}

/**
 * This node represents a `YulVariableAssignmentStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulVariableAssignmentStatement = (* variables: *) YulPaths
 *                                  (* assignment: *) YulAssignmentOperator
 *                                  (* expression: *) YulExpression;
 * ```
 */
export class YulVariableAssignmentStatement {
  private readonly fetch = once(() => {
    const [$variables, $assignment, $expression] = wasm.ast.Selectors.sequence(this.cst);

    return {
      variables: new YulPaths($variables as NonterminalNode),
      assignment: new YulAssignmentOperator($assignment as NonterminalNode),
      expression: new YulExpression($expression as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulVariableAssignmentStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulVariableAssignmentStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulVariableAssignmentStatement);
  }

  /**
   * Returns the child node that has the label `variables`.
   */
  public get variables(): YulPaths {
    return this.fetch().variables;
  }

  /**
   * Returns the child node that has the label `assignment`.
   */
  public get assignment(): YulAssignmentOperator {
    return this.fetch().assignment;
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): YulExpression {
    return this.fetch().expression;
  }
}

/**
 * This node represents a `YulColonAndEqual` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.5 *)
 * YulColonAndEqual = (* colon: *) COLON
 *                    (* equal: *) EQUAL;
 * ```
 */
export class YulColonAndEqual {
  private readonly fetch = once(() => {
    const [$colon, $equal] = wasm.ast.Selectors.sequence(this.cst);

    return {
      colon: $colon as TerminalNode,
      equal: $equal as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulColonAndEqual`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulColonAndEqual`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulColonAndEqual);
  }

  /**
   * Returns the child node that has the label `colon`.
   */
  public get colon(): TerminalNode {
    return this.fetch().colon;
  }

  /**
   * Returns the child node that has the label `equal`.
   */
  public get equal(): TerminalNode {
    return this.fetch().equal;
  }
}

/**
 * This node represents a `YulStackAssignmentStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * YulStackAssignmentStatement = (* assignment: *) YulStackAssignmentOperator
 *                               (* variable: *) YUL_IDENTIFIER;
 * ```
 */
export class YulStackAssignmentStatement {
  private readonly fetch = once(() => {
    const [$assignment, $variable] = wasm.ast.Selectors.sequence(this.cst);

    return {
      assignment: new YulStackAssignmentOperator($assignment as NonterminalNode),
      variable: $variable as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulStackAssignmentStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulStackAssignmentStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulStackAssignmentStatement);
  }

  /**
   * Returns the child node that has the label `assignment`.
   */
  public get assignment(): YulStackAssignmentOperator {
    return this.fetch().assignment;
  }

  /**
   * Returns the child node that has the label `variable`.
   */
  public get variable(): TerminalNode {
    return this.fetch().variable;
  }
}

/**
 * This node represents a `YulEqualAndColon` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * YulEqualAndColon = (* equal: *) EQUAL
 *                    (* colon: *) COLON;
 * ```
 */
export class YulEqualAndColon {
  private readonly fetch = once(() => {
    const [$equal, $colon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      equal: $equal as TerminalNode,
      colon: $colon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulEqualAndColon`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulEqualAndColon`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulEqualAndColon);
  }

  /**
   * Returns the child node that has the label `equal`.
   */
  public get equal(): TerminalNode {
    return this.fetch().equal;
  }

  /**
   * Returns the child node that has the label `colon`.
   */
  public get colon(): TerminalNode {
    return this.fetch().colon;
  }
}

/**
 * This node represents a `YulIfStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulIfStatement = (* if_keyword: *) YUL_IF_KEYWORD
 *                  (* condition: *) YulExpression
 *                  (* body: *) YulBlock;
 * ```
 */
export class YulIfStatement {
  private readonly fetch = once(() => {
    const [$ifKeyword, $condition, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      ifKeyword: $ifKeyword as TerminalNode,
      condition: new YulExpression($condition as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulIfStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulIfStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulIfStatement);
  }

  /**
   * Returns the child node that has the label `if_keyword`.
   */
  public get ifKeyword(): TerminalNode {
    return this.fetch().ifKeyword;
  }

  /**
   * Returns the child node that has the label `condition`.
   */
  public get condition(): YulExpression {
    return this.fetch().condition;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): YulBlock {
    return this.fetch().body;
  }
}

/**
 * This node represents a `YulForStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulForStatement = (* for_keyword: *) YUL_FOR_KEYWORD
 *                   (* initialization: *) YulBlock
 *                   (* condition: *) YulExpression
 *                   (* iterator: *) YulBlock
 *                   (* body: *) YulBlock;
 * ```
 */
export class YulForStatement {
  private readonly fetch = once(() => {
    const [$forKeyword, $initialization, $condition, $iterator, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      forKeyword: $forKeyword as TerminalNode,
      initialization: new YulBlock($initialization as NonterminalNode),
      condition: new YulExpression($condition as NonterminalNode),
      iterator: new YulBlock($iterator as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulForStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulForStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulForStatement);
  }

  /**
   * Returns the child node that has the label `for_keyword`.
   */
  public get forKeyword(): TerminalNode {
    return this.fetch().forKeyword;
  }

  /**
   * Returns the child node that has the label `initialization`.
   */
  public get initialization(): YulBlock {
    return this.fetch().initialization;
  }

  /**
   * Returns the child node that has the label `condition`.
   */
  public get condition(): YulExpression {
    return this.fetch().condition;
  }

  /**
   * Returns the child node that has the label `iterator`.
   */
  public get iterator(): YulBlock {
    return this.fetch().iterator;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): YulBlock {
    return this.fetch().body;
  }
}

/**
 * This node represents a `YulSwitchStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulSwitchStatement = (* switch_keyword: *) YUL_SWITCH_KEYWORD
 *                      (* expression: *) YulExpression
 *                      (* cases: *) YulSwitchCases;
 * ```
 */
export class YulSwitchStatement {
  private readonly fetch = once(() => {
    const [$switchKeyword, $expression, $cases] = wasm.ast.Selectors.sequence(this.cst);

    return {
      switchKeyword: $switchKeyword as TerminalNode,
      expression: new YulExpression($expression as NonterminalNode),
      cases: new YulSwitchCases($cases as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulSwitchStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulSwitchStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulSwitchStatement);
  }

  /**
   * Returns the child node that has the label `switch_keyword`.
   */
  public get switchKeyword(): TerminalNode {
    return this.fetch().switchKeyword;
  }

  /**
   * Returns the child node that has the label `expression`.
   */
  public get expression(): YulExpression {
    return this.fetch().expression;
  }

  /**
   * Returns the child node that has the label `cases`.
   */
  public get cases(): YulSwitchCases {
    return this.fetch().cases;
  }
}

/**
 * This node represents a `YulDefaultCase` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulDefaultCase = (* default_keyword: *) YUL_DEFAULT_KEYWORD
 *                  (* body: *) YulBlock;
 * ```
 */
export class YulDefaultCase {
  private readonly fetch = once(() => {
    const [$defaultKeyword, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      defaultKeyword: $defaultKeyword as TerminalNode,
      body: new YulBlock($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulDefaultCase`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulDefaultCase`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulDefaultCase);
  }

  /**
   * Returns the child node that has the label `default_keyword`.
   */
  public get defaultKeyword(): TerminalNode {
    return this.fetch().defaultKeyword;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): YulBlock {
    return this.fetch().body;
  }
}

/**
 * This node represents a `YulValueCase` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulValueCase = (* case_keyword: *) YUL_CASE_KEYWORD
 *                (* value: *) YulLiteral
 *                (* body: *) YulBlock;
 * ```
 */
export class YulValueCase {
  private readonly fetch = once(() => {
    const [$caseKeyword, $value, $body] = wasm.ast.Selectors.sequence(this.cst);

    return {
      caseKeyword: $caseKeyword as TerminalNode,
      value: new YulLiteral($value as NonterminalNode),
      body: new YulBlock($body as NonterminalNode),
    };
  });

  /**
   * Constructs a new AST node of type `YulValueCase`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulValueCase`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulValueCase);
  }

  /**
   * Returns the child node that has the label `case_keyword`.
   */
  public get caseKeyword(): TerminalNode {
    return this.fetch().caseKeyword;
  }

  /**
   * Returns the child node that has the label `value`.
   */
  public get value(): YulLiteral {
    return this.fetch().value;
  }

  /**
   * Returns the child node that has the label `body`.
   */
  public get body(): YulBlock {
    return this.fetch().body;
  }
}

/**
 * This node represents a `YulLeaveStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * YulLeaveStatement = (* leave_keyword: *) YUL_LEAVE_KEYWORD;
 * ```
 */
export class YulLeaveStatement {
  private readonly fetch = once(() => {
    const [$leaveKeyword] = wasm.ast.Selectors.sequence(this.cst);

    return {
      leaveKeyword: $leaveKeyword as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulLeaveStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulLeaveStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulLeaveStatement);
  }

  /**
   * Returns the child node that has the label `leave_keyword`.
   */
  public get leaveKeyword(): TerminalNode {
    return this.fetch().leaveKeyword;
  }
}

/**
 * This node represents a `YulBreakStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulBreakStatement = (* break_keyword: *) YUL_BREAK_KEYWORD;
 * ```
 */
export class YulBreakStatement {
  private readonly fetch = once(() => {
    const [$breakKeyword] = wasm.ast.Selectors.sequence(this.cst);

    return {
      breakKeyword: $breakKeyword as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulBreakStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulBreakStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulBreakStatement);
  }

  /**
   * Returns the child node that has the label `break_keyword`.
   */
  public get breakKeyword(): TerminalNode {
    return this.fetch().breakKeyword;
  }
}

/**
 * This node represents a `YulContinueStatement` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulContinueStatement = (* continue_keyword: *) YUL_CONTINUE_KEYWORD;
 * ```
 */
export class YulContinueStatement {
  private readonly fetch = once(() => {
    const [$continueKeyword] = wasm.ast.Selectors.sequence(this.cst);

    return {
      continueKeyword: $continueKeyword as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulContinueStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulContinueStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulContinueStatement);
  }

  /**
   * Returns the child node that has the label `continue_keyword`.
   */
  public get continueKeyword(): TerminalNode {
    return this.fetch().continueKeyword;
  }
}

/**
 * This node represents a `YulLabel` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * YulLabel = (* label: *) YUL_IDENTIFIER
 *            (* colon: *) COLON;
 * ```
 */
export class YulLabel {
  private readonly fetch = once(() => {
    const [$label, $colon] = wasm.ast.Selectors.sequence(this.cst);

    return {
      label: $label as TerminalNode,
      colon: $colon as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulLabel`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulLabel`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulLabel);
  }

  /**
   * Returns the child node that has the label `label`.
   */
  public get label(): TerminalNode {
    return this.fetch().label;
  }

  /**
   * Returns the child node that has the label `colon`.
   */
  public get colon(): TerminalNode {
    return this.fetch().colon;
  }
}

/**
 * This node represents a `YulFunctionCallExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * YulFunctionCallExpression = (* operand: *) YulExpression
 *                             (* open_paren: *) OPEN_PAREN
 *                             (* arguments: *) YulArguments
 *                             (* close_paren: *) CLOSE_PAREN;
 * ```
 */
export class YulFunctionCallExpression {
  private readonly fetch = once(() => {
    const [$operand, $openParen, $arguments, $closeParen] = wasm.ast.Selectors.sequence(this.cst);

    return {
      operand: new YulExpression($operand as NonterminalNode),
      openParen: $openParen as TerminalNode,
      arguments: new YulArguments($arguments as NonterminalNode),
      closeParen: $closeParen as TerminalNode,
    };
  });

  /**
   * Constructs a new AST node of type `YulFunctionCallExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulFunctionCallExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulFunctionCallExpression);
  }

  /**
   * Returns the child node that has the label `operand`.
   */
  public get operand(): YulExpression {
    return this.fetch().operand;
  }

  /**
   * Returns the child node that has the label `open_paren`.
   */
  public get openParen(): TerminalNode {
    return this.fetch().openParen;
  }

  /**
   * Returns the child node that has the label `arguments`.
   */
  public get arguments(): YulArguments {
    return this.fetch().arguments;
  }

  /**
   * Returns the child node that has the label `close_paren`.
   */
  public get closeParen(): TerminalNode {
    return this.fetch().closeParen;
  }
}

//
// Choices:
//

/**
 * This node represents a `SourceUnitMember` nonterminal, with the following structure:
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
    const variant = wasm.ast.Selectors.choice(this.cst);

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
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `SourceUnitMember`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `SourceUnitMember`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.SourceUnitMember);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
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

/**
 * This node represents a `Pragma` nonterminal, with the following structure:
 *
 * ```ebnf
 * Pragma = (* variant: *) VersionPragma
 *        | (* variant: *) AbicoderPragma (* Introduced in 0.7.5 *)
 *        | (* variant: *) ExperimentalPragma; (* Introduced in 0.4.16 *)
 * ```
 */
export class Pragma {
  private readonly fetch: () => VersionPragma | AbicoderPragma | ExperimentalPragma = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.VersionPragma:
        return new VersionPragma(variant as NonterminalNode);
      case NonterminalKind.AbicoderPragma:
        return new AbicoderPragma(variant as NonterminalNode);
      case NonterminalKind.ExperimentalPragma:
        return new ExperimentalPragma(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `Pragma`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Pragma`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Pragma);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): VersionPragma | AbicoderPragma | ExperimentalPragma {
    return this.fetch();
  }
}

/**
 * This node represents a `AbicoderVersion` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.7.5 *)
 * AbicoderVersion = (* variant: *) ABICODER_V1_KEYWORD
 *                 | (* variant: *) ABICODER_V2_KEYWORD;
 * ```
 */
export class AbicoderVersion {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `AbicoderVersion`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AbicoderVersion`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AbicoderVersion);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `ExperimentalFeature` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.4.16 *)
 * ExperimentalFeature = (* variant: *) ABIENCODER_V2_KEYWORD
 *                     | (* variant: *) SMTCHECKER_KEYWORD
 *                     | (* variant: *) StringLiteral;
 * ```
 */
export class ExperimentalFeature {
  private readonly fetch: () => StringLiteral | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.StringLiteral:
        return new StringLiteral(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ExperimentalFeature`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ExperimentalFeature`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ExperimentalFeature);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): StringLiteral | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `VersionExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionExpression = (* variant: *) VersionRange
 *                   | (* variant: *) VersionTerm;
 * ```
 */
export class VersionExpression {
  private readonly fetch: () => VersionRange | VersionTerm = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.VersionRange:
        return new VersionRange(variant as NonterminalNode);
      case NonterminalKind.VersionTerm:
        return new VersionTerm(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `VersionExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VersionExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VersionExpression);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): VersionRange | VersionTerm {
    return this.fetch();
  }
}

/**
 * This node represents a `VersionOperator` nonterminal, with the following structure:
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
export class VersionOperator {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `VersionOperator`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VersionOperator`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VersionOperator);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `VersionLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionLiteral = (* variant: *) SimpleVersionLiteral
 *                | (* variant: *) SINGLE_QUOTED_VERSION_LITERAL
 *                | (* variant: *) DOUBLE_QUOTED_VERSION_LITERAL;
 * ```
 */
export class VersionLiteral {
  private readonly fetch: () => SimpleVersionLiteral | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.SimpleVersionLiteral:
        return new SimpleVersionLiteral(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `VersionLiteral`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VersionLiteral`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VersionLiteral);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): SimpleVersionLiteral | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `ImportClause` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportClause = (* variant: *) PathImport
 *              | (* variant: *) NamedImport
 *              | (* variant: *) ImportDeconstruction;
 * ```
 */
export class ImportClause {
  private readonly fetch: () => PathImport | NamedImport | ImportDeconstruction = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.PathImport:
        return new PathImport(variant as NonterminalNode);
      case NonterminalKind.NamedImport:
        return new NamedImport(variant as NonterminalNode);
      case NonterminalKind.ImportDeconstruction:
        return new ImportDeconstruction(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ImportClause`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ImportClause`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ImportClause);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): PathImport | NamedImport | ImportDeconstruction {
    return this.fetch();
  }
}

/**
 * This node represents a `UsingClause` nonterminal, with the following structure:
 *
 * ```ebnf
 * UsingClause = (* variant: *) IdentifierPath
 *             | (* variant: *) UsingDeconstruction; (* Introduced in 0.8.13 *)
 * ```
 */
export class UsingClause {
  private readonly fetch: () => IdentifierPath | UsingDeconstruction = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.IdentifierPath:
        return new IdentifierPath(variant as NonterminalNode);
      case NonterminalKind.UsingDeconstruction:
        return new UsingDeconstruction(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `UsingClause`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UsingClause`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UsingClause);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): IdentifierPath | UsingDeconstruction {
    return this.fetch();
  }
}

/**
 * This node represents a `UsingOperator` nonterminal, with the following structure:
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
export class UsingOperator {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `UsingOperator`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UsingOperator`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UsingOperator);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `UsingTarget` nonterminal, with the following structure:
 *
 * ```ebnf
 * UsingTarget = (* variant: *) TypeName
 *             | (* variant: *) ASTERISK;
 * ```
 */
export class UsingTarget {
  private readonly fetch: () => TypeName | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.TypeName:
        return new TypeName(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `UsingTarget`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UsingTarget`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UsingTarget);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TypeName | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `ContractSpecifier` nonterminal, with the following structure:
 *
 * ```ebnf
 * ContractSpecifier = (* variant: *) InheritanceSpecifier
 *                   | (* variant: *) StorageLayoutSpecifier; (* Introduced in 0.8.29 *)
 * ```
 */
export class ContractSpecifier {
  private readonly fetch: () => InheritanceSpecifier | StorageLayoutSpecifier = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.InheritanceSpecifier:
        return new InheritanceSpecifier(variant as NonterminalNode);
      case NonterminalKind.StorageLayoutSpecifier:
        return new StorageLayoutSpecifier(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ContractSpecifier`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ContractSpecifier`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ContractSpecifier);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): InheritanceSpecifier | StorageLayoutSpecifier {
    return this.fetch();
  }
}

/**
 * This node represents a `ContractMember` nonterminal, with the following structure:
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
    const variant = wasm.ast.Selectors.choice(this.cst);

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
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ContractMember`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ContractMember`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ContractMember);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
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

/**
 * This node represents a `StateVariableAttribute` nonterminal, with the following structure:
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
export class StateVariableAttribute {
  private readonly fetch: () => OverrideSpecifier | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `StateVariableAttribute`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StateVariableAttribute`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StateVariableAttribute);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `FunctionName` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionName = (* variant: *) IDENTIFIER
 *              | (* variant: *) FALLBACK_KEYWORD
 *              | (* variant: *) RECEIVE_KEYWORD;
 * ```
 */
export class FunctionName {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `FunctionName`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FunctionName`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FunctionName);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `FunctionAttribute` nonterminal, with the following structure:
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
export class FunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `FunctionAttribute`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FunctionAttribute`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FunctionAttribute);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): ModifierInvocation | OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `FunctionBody` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionBody = (* variant: *) Block
 *              | (* variant: *) SEMICOLON;
 * ```
 */
export class FunctionBody {
  private readonly fetch: () => Block | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.Block:
        return new Block(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `FunctionBody`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FunctionBody`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FunctionBody);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): Block | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `ConstructorAttribute` nonterminal, with the following structure:
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
export class ConstructorAttribute {
  private readonly fetch: () => ModifierInvocation | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ConstructorAttribute`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ConstructorAttribute`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ConstructorAttribute);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): ModifierInvocation | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `UnnamedFunctionAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.6.0 *)
 * UnnamedFunctionAttribute = (* variant: *) ModifierInvocation
 *                          | (* variant: *) CONSTANT_KEYWORD (* Deprecated in 0.5.0 *)
 *                          | (* variant: *) EXTERNAL_KEYWORD
 *                          | (* variant: *) INTERNAL_KEYWORD (* Deprecated in 0.5.0 *)
 *                          | (* variant: *) PAYABLE_KEYWORD
 *                          | (* variant: *) PRIVATE_KEYWORD (* Deprecated in 0.5.0 *)
 *                          | (* variant: *) PUBLIC_KEYWORD (* Deprecated in 0.5.0 *)
 *                          | (* variant: *) PURE_KEYWORD (* Introduced in 0.4.16 and deprecated in 0.6.0. *)
 *                          | (* variant: *) VIEW_KEYWORD; (* Introduced in 0.4.16 and deprecated in 0.6.0. *)
 * ```
 */
export class UnnamedFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `UnnamedFunctionAttribute`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UnnamedFunctionAttribute`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UnnamedFunctionAttribute);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): ModifierInvocation | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `FallbackFunctionAttribute` nonterminal, with the following structure:
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
export class FallbackFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `FallbackFunctionAttribute`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FallbackFunctionAttribute`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FallbackFunctionAttribute);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): ModifierInvocation | OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `ReceiveFunctionAttribute` nonterminal, with the following structure:
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
export class ReceiveFunctionAttribute {
  private readonly fetch: () => ModifierInvocation | OverrideSpecifier | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.ModifierInvocation:
        return new ModifierInvocation(variant as NonterminalNode);
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ReceiveFunctionAttribute`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ReceiveFunctionAttribute`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ReceiveFunctionAttribute);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): ModifierInvocation | OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `ModifierAttribute` nonterminal, with the following structure:
 *
 * ```ebnf
 * ModifierAttribute = (* variant: *) OverrideSpecifier (* Introduced in 0.6.0 *)
 *                   | (* variant: *) VIRTUAL_KEYWORD; (* Introduced in 0.6.0 *)
 * ```
 */
export class ModifierAttribute {
  private readonly fetch: () => OverrideSpecifier | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.OverrideSpecifier:
        return new OverrideSpecifier(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ModifierAttribute`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ModifierAttribute`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ModifierAttribute);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): OverrideSpecifier | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `TypeName` nonterminal, with the following structure:
 *
 * ```ebnf
 * TypeName = (* variant: *) ArrayTypeName
 *          | (* variant: *) FunctionType
 *          | (* variant: *) MappingType
 *          | (* variant: *) ElementaryType
 *          | (* variant: *) IdentifierPath;
 * ```
 */
export class TypeName {
  private readonly fetch: () => ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath = once(
    () => {
      const variant = wasm.ast.Selectors.choice(this.cst);

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
          throw new Error(`Unexpected variant: '${variant.kind}'.`);
      }
    },
  );

  /**
   * Constructs a new AST node of type `TypeName`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TypeName`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TypeName);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath {
    return this.fetch();
  }
}

/**
 * This node represents a `FunctionTypeAttribute` nonterminal, with the following structure:
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
export class FunctionTypeAttribute {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `FunctionTypeAttribute`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FunctionTypeAttribute`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FunctionTypeAttribute);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `MappingKeyType` nonterminal, with the following structure:
 *
 * ```ebnf
 * MappingKeyType = (* variant: *) ElementaryType
 *                | (* variant: *) IdentifierPath;
 * ```
 */
export class MappingKeyType {
  private readonly fetch: () => ElementaryType | IdentifierPath = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.ElementaryType:
        return new ElementaryType(variant as NonterminalNode);
      case NonterminalKind.IdentifierPath:
        return new IdentifierPath(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `MappingKeyType`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `MappingKeyType`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.MappingKeyType);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): ElementaryType | IdentifierPath {
    return this.fetch();
  }
}

/**
 * This node represents a `ElementaryType` nonterminal, with the following structure:
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
export class ElementaryType {
  private readonly fetch: () => AddressType | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.AddressType:
        return new AddressType(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ElementaryType`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ElementaryType`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ElementaryType);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): AddressType | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `Statement` nonterminal, with the following structure:
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
    const variant = wasm.ast.Selectors.choice(this.cst);

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
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `Statement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Statement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Statement);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
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

/**
 * This node represents a `TupleMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleMember = (* variant: *) TypedTupleMember
 *             | (* variant: *) UntypedTupleMember;
 * ```
 */
export class TupleMember {
  private readonly fetch: () => TypedTupleMember | UntypedTupleMember = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.TypedTupleMember:
        return new TypedTupleMember(variant as NonterminalNode);
      case NonterminalKind.UntypedTupleMember:
        return new UntypedTupleMember(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `TupleMember`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TupleMember`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TupleMember);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TypedTupleMember | UntypedTupleMember {
    return this.fetch();
  }
}

/**
 * This node represents a `VariableDeclarationType` nonterminal, with the following structure:
 *
 * ```ebnf
 * VariableDeclarationType = (* variant: *) TypeName
 *                         | (* variant: *) VAR_KEYWORD; (* Deprecated in 0.5.0 *)
 * ```
 */
export class VariableDeclarationType {
  private readonly fetch: () => TypeName | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.TypeName:
        return new TypeName(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `VariableDeclarationType`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VariableDeclarationType`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VariableDeclarationType);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TypeName | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `StorageLocation` nonterminal, with the following structure:
 *
 * ```ebnf
 * StorageLocation = (* variant: *) MEMORY_KEYWORD
 *                 | (* variant: *) STORAGE_KEYWORD
 *                 | (* variant: *) CALL_DATA_KEYWORD; (* Introduced in 0.5.0 *)
 * ```
 */
export class StorageLocation {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `StorageLocation`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StorageLocation`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StorageLocation);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `ForStatementInitialization` nonterminal, with the following structure:
 *
 * ```ebnf
 * ForStatementInitialization = (* variant: *) TupleDeconstructionStatement
 *                            | (* variant: *) VariableDeclarationStatement
 *                            | (* variant: *) ExpressionStatement
 *                            | (* variant: *) SEMICOLON;
 * ```
 */
export class ForStatementInitialization {
  private readonly fetch: () =>
    | TupleDeconstructionStatement
    | VariableDeclarationStatement
    | ExpressionStatement
    | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.TupleDeconstructionStatement:
        return new TupleDeconstructionStatement(variant as NonterminalNode);
      case NonterminalKind.VariableDeclarationStatement:
        return new VariableDeclarationStatement(variant as NonterminalNode);
      case NonterminalKind.ExpressionStatement:
        return new ExpressionStatement(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ForStatementInitialization`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ForStatementInitialization`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ForStatementInitialization);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant():
    | TupleDeconstructionStatement
    | VariableDeclarationStatement
    | ExpressionStatement
    | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `ForStatementCondition` nonterminal, with the following structure:
 *
 * ```ebnf
 * ForStatementCondition = (* variant: *) ExpressionStatement
 *                       | (* variant: *) SEMICOLON;
 * ```
 */
export class ForStatementCondition {
  private readonly fetch: () => ExpressionStatement | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.ExpressionStatement:
        return new ExpressionStatement(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ForStatementCondition`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ForStatementCondition`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ForStatementCondition);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): ExpressionStatement | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `Expression` nonterminal, with the following structure:
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
export class Expression {
  private readonly fetch: () =>
    | AssignmentExpression
    | ConditionalExpression
    | OrExpression
    | AndExpression
    | EqualityExpression
    | InequalityExpression
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
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
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
      case NonterminalKind.InequalityExpression:
        return new InequalityExpression(variant as NonterminalNode);
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
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `Expression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Expression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Expression);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant():
    | AssignmentExpression
    | ConditionalExpression
    | OrExpression
    | AndExpression
    | EqualityExpression
    | InequalityExpression
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

/**
 * This node represents a `ArgumentsDeclaration` nonterminal, with the following structure:
 *
 * ```ebnf
 * ArgumentsDeclaration = (* variant: *) PositionalArgumentsDeclaration
 *                      | (* variant: *) NamedArgumentsDeclaration;
 * ```
 */
export class ArgumentsDeclaration {
  private readonly fetch: () => PositionalArgumentsDeclaration | NamedArgumentsDeclaration = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.PositionalArgumentsDeclaration:
        return new PositionalArgumentsDeclaration(variant as NonterminalNode);
      case NonterminalKind.NamedArgumentsDeclaration:
        return new NamedArgumentsDeclaration(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `ArgumentsDeclaration`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ArgumentsDeclaration`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ArgumentsDeclaration);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): PositionalArgumentsDeclaration | NamedArgumentsDeclaration {
    return this.fetch();
  }
}

/**
 * This node represents a `NumberUnit` nonterminal, with the following structure:
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
export class NumberUnit {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `NumberUnit`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `NumberUnit`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.NumberUnit);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `StringExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * StringExpression = (* variant: *) StringLiteral (* Deprecated in 0.5.14 *)
 *                  | (* variant: *) StringLiterals (* Introduced in 0.5.14 *)
 *                  | (* variant: *) HexStringLiteral (* Deprecated in 0.5.14 *)
 *                  | (* variant: *) HexStringLiterals (* Introduced in 0.5.14 *)
 *                  | (* variant: *) UnicodeStringLiterals; (* Introduced in 0.7.0 *)
 * ```
 */
export class StringExpression {
  private readonly fetch: () =>
    | StringLiteral
    | StringLiterals
    | HexStringLiteral
    | HexStringLiterals
    | UnicodeStringLiterals = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

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
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `StringExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StringExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StringExpression);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): StringLiteral | StringLiterals | HexStringLiteral | HexStringLiterals | UnicodeStringLiterals {
    return this.fetch();
  }
}

/**
 * This node represents a `StringLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * StringLiteral = (* variant: *) SINGLE_QUOTED_STRING_LITERAL
 *               | (* variant: *) DOUBLE_QUOTED_STRING_LITERAL;
 * ```
 */
export class StringLiteral {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `StringLiteral`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StringLiteral`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StringLiteral);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `HexStringLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * HexStringLiteral = (* variant: *) SINGLE_QUOTED_HEX_STRING_LITERAL
 *                  | (* variant: *) DOUBLE_QUOTED_HEX_STRING_LITERAL;
 * ```
 */
export class HexStringLiteral {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `HexStringLiteral`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `HexStringLiteral`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.HexStringLiteral);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `UnicodeStringLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.7.0 *)
 * UnicodeStringLiteral = (* variant: *) SINGLE_QUOTED_UNICODE_STRING_LITERAL
 *                      | (* variant: *) DOUBLE_QUOTED_UNICODE_STRING_LITERAL;
 * ```
 */
export class UnicodeStringLiteral {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    return variant as TerminalNode;
  });

  /**
   * Constructs a new AST node of type `UnicodeStringLiteral`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UnicodeStringLiteral`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UnicodeStringLiteral);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `YulStatement` nonterminal, with the following structure:
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
    | YulVariableAssignmentStatement
    | YulLabel
    | YulVariableDeclarationStatement
    | YulExpression = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

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
      case NonterminalKind.YulVariableAssignmentStatement:
        return new YulVariableAssignmentStatement(variant as NonterminalNode);
      case NonterminalKind.YulLabel:
        return new YulLabel(variant as NonterminalNode);
      case NonterminalKind.YulVariableDeclarationStatement:
        return new YulVariableDeclarationStatement(variant as NonterminalNode);
      case NonterminalKind.YulExpression:
        return new YulExpression(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `YulStatement`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulStatement`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulStatement);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
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
    | YulVariableAssignmentStatement
    | YulLabel
    | YulVariableDeclarationStatement
    | YulExpression {
    return this.fetch();
  }
}

/**
 * This node represents a `YulAssignmentOperator` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulAssignmentOperator = (* variant: *) COLON_EQUAL
 *                       | (* variant: *) YulColonAndEqual; (* Deprecated in 0.5.5 *)
 * ```
 */
export class YulAssignmentOperator {
  private readonly fetch: () => YulColonAndEqual | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.YulColonAndEqual:
        return new YulColonAndEqual(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `YulAssignmentOperator`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulAssignmentOperator`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulAssignmentOperator);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): YulColonAndEqual | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `YulStackAssignmentOperator` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.5.0 *)
 * YulStackAssignmentOperator = (* variant: *) EQUAL_COLON
 *                            | (* variant: *) YulEqualAndColon;
 * ```
 */
export class YulStackAssignmentOperator {
  private readonly fetch: () => YulEqualAndColon | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.YulEqualAndColon:
        return new YulEqualAndColon(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `YulStackAssignmentOperator`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulStackAssignmentOperator`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulStackAssignmentOperator);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): YulEqualAndColon | TerminalNode {
    return this.fetch();
  }
}

/**
 * This node represents a `YulSwitchCase` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulSwitchCase = (* variant: *) YulDefaultCase
 *               | (* variant: *) YulValueCase;
 * ```
 */
export class YulSwitchCase {
  private readonly fetch: () => YulDefaultCase | YulValueCase = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.YulDefaultCase:
        return new YulDefaultCase(variant as NonterminalNode);
      case NonterminalKind.YulValueCase:
        return new YulValueCase(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `YulSwitchCase`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulSwitchCase`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulSwitchCase);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): YulDefaultCase | YulValueCase {
    return this.fetch();
  }
}

/**
 * This node represents a `YulExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulExpression = (* variant: *) YulFunctionCallExpression
 *               | (* variant: *) YulLiteral
 *               | (* variant: *) YulPath;
 * ```
 */
export class YulExpression {
  private readonly fetch: () => YulFunctionCallExpression | YulLiteral | YulPath = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    switch (variant.kind) {
      case NonterminalKind.YulFunctionCallExpression:
        return new YulFunctionCallExpression(variant as NonterminalNode);
      case NonterminalKind.YulLiteral:
        return new YulLiteral(variant as NonterminalNode);
      case NonterminalKind.YulPath:
        return new YulPath(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `YulExpression`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulExpression`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulExpression);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): YulFunctionCallExpression | YulLiteral | YulPath {
    return this.fetch();
  }
}

/**
 * This node represents a `YulLiteral` nonterminal, with the following structure:
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
export class YulLiteral {
  private readonly fetch: () => HexStringLiteral | StringLiteral | TerminalNode = once(() => {
    const variant = wasm.ast.Selectors.choice(this.cst);

    if (variant.isTerminalNode()) {
      return variant;
    }

    switch (variant.kind) {
      case NonterminalKind.HexStringLiteral:
        return new HexStringLiteral(variant as NonterminalNode);
      case NonterminalKind.StringLiteral:
        return new StringLiteral(variant as NonterminalNode);

      default:
        throw new Error(`Unexpected variant: '${variant.kind}'.`);
    }
  });

  /**
   * Constructs a new AST node of type `YulLiteral`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulLiteral`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulLiteral);
  }

  /**
   * Returns the child node that has the label `variant`.
   */
  public get variant(): HexStringLiteral | StringLiteral | TerminalNode {
    return this.fetch();
  }
}

//
// Repeated:
//

/**
 * This node represents a `SourceUnitMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnitMembers = (* item: *) SourceUnitMember*;
 * ```
 */
export class SourceUnitMembers {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new SourceUnitMember(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `SourceUnitMembers`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `SourceUnitMembers`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.SourceUnitMembers);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly SourceUnitMember[] {
    return this.fetch();
  }
}

/**
 * This node represents a `VersionExpressionSet` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionExpressionSet = (* item: *) VersionExpression+;
 * ```
 */
export class VersionExpressionSet {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new VersionExpression(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `VersionExpressionSet`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VersionExpressionSet`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VersionExpressionSet);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly VersionExpression[] {
    return this.fetch();
  }
}

/**
 * This node represents a `ContractSpecifiers` nonterminal, with the following structure:
 *
 * ```ebnf
 * ContractSpecifiers = (* item: *) ContractSpecifier*;
 * ```
 */
export class ContractSpecifiers {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new ContractSpecifier(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `ContractSpecifiers`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ContractSpecifiers`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ContractSpecifiers);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly ContractSpecifier[] {
    return this.fetch();
  }
}

/**
 * This node represents a `ContractMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * ContractMembers = (* item: *) ContractMember*;
 * ```
 */
export class ContractMembers {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new ContractMember(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `ContractMembers`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ContractMembers`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ContractMembers);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

/**
 * This node represents a `InterfaceMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * InterfaceMembers = (* item: *) ContractMember*;
 * ```
 */
export class InterfaceMembers {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new ContractMember(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `InterfaceMembers`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `InterfaceMembers`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.InterfaceMembers);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

/**
 * This node represents a `LibraryMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * LibraryMembers = (* item: *) ContractMember*;
 * ```
 */
export class LibraryMembers {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new ContractMember(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `LibraryMembers`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `LibraryMembers`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.LibraryMembers);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly ContractMember[] {
    return this.fetch();
  }
}

/**
 * This node represents a `StructMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * StructMembers = (* item: *) StructMember*;
 * ```
 */
export class StructMembers {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new StructMember(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `StructMembers`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StructMembers`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StructMembers);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly StructMember[] {
    return this.fetch();
  }
}

/**
 * This node represents a `StateVariableAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * StateVariableAttributes = (* item: *) StateVariableAttribute*;
 * ```
 */
export class StateVariableAttributes {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new StateVariableAttribute(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `StateVariableAttributes`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StateVariableAttributes`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StateVariableAttributes);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly StateVariableAttribute[] {
    return this.fetch();
  }
}

/**
 * This node represents a `FunctionAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionAttributes = (* item: *) FunctionAttribute*;
 * ```
 */
export class FunctionAttributes {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new FunctionAttribute(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `FunctionAttributes`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FunctionAttributes`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FunctionAttributes);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly FunctionAttribute[] {
    return this.fetch();
  }
}

/**
 * This node represents a `ConstructorAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.4.22 *)
 * ConstructorAttributes = (* item: *) ConstructorAttribute*;
 * ```
 */
export class ConstructorAttributes {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new ConstructorAttribute(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `ConstructorAttributes`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ConstructorAttributes`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ConstructorAttributes);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly ConstructorAttribute[] {
    return this.fetch();
  }
}

/**
 * This node represents a `UnnamedFunctionAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Deprecated in 0.6.0 *)
 * UnnamedFunctionAttributes = (* item: *) UnnamedFunctionAttribute*;
 * ```
 */
export class UnnamedFunctionAttributes {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new UnnamedFunctionAttribute(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `UnnamedFunctionAttributes`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UnnamedFunctionAttributes`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UnnamedFunctionAttributes);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly UnnamedFunctionAttribute[] {
    return this.fetch();
  }
}

/**
 * This node represents a `FallbackFunctionAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * FallbackFunctionAttributes = (* item: *) FallbackFunctionAttribute*;
 * ```
 */
export class FallbackFunctionAttributes {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new FallbackFunctionAttribute(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `FallbackFunctionAttributes`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FallbackFunctionAttributes`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FallbackFunctionAttributes);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly FallbackFunctionAttribute[] {
    return this.fetch();
  }
}

/**
 * This node represents a `ReceiveFunctionAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * ReceiveFunctionAttributes = (* item: *) ReceiveFunctionAttribute*;
 * ```
 */
export class ReceiveFunctionAttributes {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new ReceiveFunctionAttribute(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `ReceiveFunctionAttributes`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ReceiveFunctionAttributes`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ReceiveFunctionAttributes);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly ReceiveFunctionAttribute[] {
    return this.fetch();
  }
}

/**
 * This node represents a `ModifierAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * ModifierAttributes = (* item: *) ModifierAttribute*;
 * ```
 */
export class ModifierAttributes {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new ModifierAttribute(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `ModifierAttributes`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ModifierAttributes`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ModifierAttributes);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly ModifierAttribute[] {
    return this.fetch();
  }
}

/**
 * This node represents a `FunctionTypeAttributes` nonterminal, with the following structure:
 *
 * ```ebnf
 * FunctionTypeAttributes = (* item: *) FunctionTypeAttribute*;
 * ```
 */
export class FunctionTypeAttributes {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new FunctionTypeAttribute(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `FunctionTypeAttributes`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `FunctionTypeAttributes`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.FunctionTypeAttributes);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly FunctionTypeAttribute[] {
    return this.fetch();
  }
}

/**
 * This node represents a `Statements` nonterminal, with the following structure:
 *
 * ```ebnf
 * Statements = (* item: *) Statement*;
 * ```
 */
export class Statements {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new Statement(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `Statements`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Statements`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Statements);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly Statement[] {
    return this.fetch();
  }
}

/**
 * This node represents a `CatchClauses` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * CatchClauses = (* item: *) CatchClause+;
 * ```
 */
export class CatchClauses {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new CatchClause(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `CatchClauses`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `CatchClauses`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.CatchClauses);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly CatchClause[] {
    return this.fetch();
  }
}

/**
 * This node represents a `StringLiterals` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.5.14 *)
 * StringLiterals = (* item: *) StringLiteral+;
 * ```
 */
export class StringLiterals {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new StringLiteral(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `StringLiterals`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `StringLiterals`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.StringLiterals);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly StringLiteral[] {
    return this.fetch();
  }
}

/**
 * This node represents a `HexStringLiterals` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.5.14 *)
 * HexStringLiterals = (* item: *) HexStringLiteral+;
 * ```
 */
export class HexStringLiterals {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new HexStringLiteral(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `HexStringLiterals`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `HexStringLiterals`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.HexStringLiterals);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly HexStringLiteral[] {
    return this.fetch();
  }
}

/**
 * This node represents a `UnicodeStringLiterals` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.7.0 *)
 * UnicodeStringLiterals = (* item: *) UnicodeStringLiteral+;
 * ```
 */
export class UnicodeStringLiterals {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new UnicodeStringLiteral(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `UnicodeStringLiterals`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UnicodeStringLiterals`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UnicodeStringLiterals);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly UnicodeStringLiteral[] {
    return this.fetch();
  }
}

/**
 * This node represents a `YulStatements` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulStatements = (* item: *) YulStatement*;
 * ```
 */
export class YulStatements {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new YulStatement(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `YulStatements`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulStatements`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulStatements);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly YulStatement[] {
    return this.fetch();
  }
}

/**
 * This node represents a `YulSwitchCases` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulSwitchCases = (* item: *) YulSwitchCase+;
 * ```
 */
export class YulSwitchCases {
  private readonly fetch = once(() => {
    const items = wasm.ast.Selectors.repeated(this.cst);
    return items.map((item) => new YulSwitchCase(item as NonterminalNode));
  });

  /**
   * Constructs a new AST node of type `YulSwitchCases`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulSwitchCases`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulSwitchCases);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly YulSwitchCase[] {
    return this.fetch();
  }
}

//
// Separated:
//

/**
 * This node represents a `VersionExpressionSets` nonterminal, with the following structure:
 *
 * ```ebnf
 * VersionExpressionSets = (* item: *) VersionExpressionSet ((* separator: *) BAR_BAR (* item: *) VersionExpressionSet)*;
 * ```
 */
export class VersionExpressionSets {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new VersionExpressionSet(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `VersionExpressionSets`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `VersionExpressionSets`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.VersionExpressionSets);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly VersionExpressionSet[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `SimpleVersionLiteral` nonterminal, with the following structure:
 *
 * ```ebnf
 * SimpleVersionLiteral = (* item: *) VERSION_SPECIFIER ((* separator: *) PERIOD (* item: *) VERSION_SPECIFIER)*;
 * ```
 */
export class SimpleVersionLiteral {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  /**
   * Constructs a new AST node of type `SimpleVersionLiteral`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `SimpleVersionLiteral`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.SimpleVersionLiteral);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `ImportDeconstructionSymbols` nonterminal, with the following structure:
 *
 * ```ebnf
 * ImportDeconstructionSymbols = (* item: *) ImportDeconstructionSymbol ((* separator: *) COMMA (* item: *) ImportDeconstructionSymbol)*;
 * ```
 */
export class ImportDeconstructionSymbols {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new ImportDeconstructionSymbol(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `ImportDeconstructionSymbols`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ImportDeconstructionSymbols`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ImportDeconstructionSymbols);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly ImportDeconstructionSymbol[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `UsingDeconstructionSymbols` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * UsingDeconstructionSymbols = (* item: *) UsingDeconstructionSymbol ((* separator: *) COMMA (* item: *) UsingDeconstructionSymbol)*;
 * ```
 */
export class UsingDeconstructionSymbols {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new UsingDeconstructionSymbol(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `UsingDeconstructionSymbols`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `UsingDeconstructionSymbols`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.UsingDeconstructionSymbols);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly UsingDeconstructionSymbol[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `InheritanceTypes` nonterminal, with the following structure:
 *
 * ```ebnf
 * InheritanceTypes = (* item: *) InheritanceType ((* separator: *) COMMA (* item: *) InheritanceType)*;
 * ```
 */
export class InheritanceTypes {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new InheritanceType(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `InheritanceTypes`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `InheritanceTypes`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.InheritanceTypes);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly InheritanceType[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `EnumMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * EnumMembers = ((* item: *) IDENTIFIER ((* separator: *) COMMA (* item: *) IDENTIFIER)*)?;
 * ```
 */
export class EnumMembers {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  /**
   * Constructs a new AST node of type `EnumMembers`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `EnumMembers`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.EnumMembers);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `Parameters` nonterminal, with the following structure:
 *
 * ```ebnf
 * Parameters = ((* item: *) Parameter ((* separator: *) COMMA (* item: *) Parameter)*)?;
 * ```
 */
export class Parameters {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new Parameter(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `Parameters`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `Parameters`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.Parameters);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly Parameter[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `OverridePaths` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.0 *)
 * OverridePaths = (* item: *) IdentifierPath ((* separator: *) COMMA (* item: *) IdentifierPath)*;
 * ```
 */
export class OverridePaths {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new IdentifierPath(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `OverridePaths`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `OverridePaths`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.OverridePaths);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly IdentifierPath[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `EventParameters` nonterminal, with the following structure:
 *
 * ```ebnf
 * EventParameters = ((* item: *) EventParameter ((* separator: *) COMMA (* item: *) EventParameter)*)?;
 * ```
 */
export class EventParameters {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new EventParameter(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `EventParameters`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `EventParameters`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.EventParameters);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly EventParameter[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `ErrorParameters` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.4 *)
 * ErrorParameters = ((* item: *) ErrorParameter ((* separator: *) COMMA (* item: *) ErrorParameter)*)?;
 * ```
 */
export class ErrorParameters {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new ErrorParameter(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `ErrorParameters`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ErrorParameters`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ErrorParameters);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly ErrorParameter[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `AssemblyFlags` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.8.13 *)
 * AssemblyFlags = (* item: *) StringLiteral ((* separator: *) COMMA (* item: *) StringLiteral)*;
 * ```
 */
export class AssemblyFlags {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new StringLiteral(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `AssemblyFlags`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `AssemblyFlags`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.AssemblyFlags);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly StringLiteral[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `TupleDeconstructionElements` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleDeconstructionElements = (* item: *) TupleDeconstructionElement ((* separator: *) COMMA (* item: *) TupleDeconstructionElement)*;
 * ```
 */
export class TupleDeconstructionElements {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new TupleDeconstructionElement(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `TupleDeconstructionElements`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TupleDeconstructionElements`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TupleDeconstructionElements);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TupleDeconstructionElement[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `PositionalArguments` nonterminal, with the following structure:
 *
 * ```ebnf
 * PositionalArguments = ((* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*)?;
 * ```
 */
export class PositionalArguments {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new Expression(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `PositionalArguments`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `PositionalArguments`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.PositionalArguments);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly Expression[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `NamedArguments` nonterminal, with the following structure:
 *
 * ```ebnf
 * NamedArguments = ((* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*)?;
 * ```
 */
export class NamedArguments {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new NamedArgument(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `NamedArguments`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `NamedArguments`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.NamedArguments);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly NamedArgument[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `CallOptions` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 0.6.2 *)
 * CallOptions = (* item: *) NamedArgument ((* separator: *) COMMA (* item: *) NamedArgument)*;
 * ```
 */
export class CallOptions {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new NamedArgument(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `CallOptions`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `CallOptions`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.CallOptions);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly NamedArgument[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `TupleValues` nonterminal, with the following structure:
 *
 * ```ebnf
 * TupleValues = (* item: *) TupleValue ((* separator: *) COMMA (* item: *) TupleValue)*;
 * ```
 */
export class TupleValues {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new TupleValue(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `TupleValues`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `TupleValues`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.TupleValues);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TupleValue[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `ArrayValues` nonterminal, with the following structure:
 *
 * ```ebnf
 * ArrayValues = (* item: *) Expression ((* separator: *) COMMA (* item: *) Expression)*;
 * ```
 */
export class ArrayValues {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new Expression(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `ArrayValues`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `ArrayValues`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.ArrayValues);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly Expression[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `IdentifierPath` nonterminal, with the following structure:
 *
 * ```ebnf
 * IdentifierPath = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
 * ```
 */
export class IdentifierPath {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  /**
   * Constructs a new AST node of type `IdentifierPath`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `IdentifierPath`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.IdentifierPath);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `YulParameters` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulParameters = ((* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*)?;
 * ```
 */
export class YulParameters {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  /**
   * Constructs a new AST node of type `YulParameters`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulParameters`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulParameters);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `YulVariableNames` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulVariableNames = (* item: *) YUL_IDENTIFIER ((* separator: *) COMMA (* item: *) YUL_IDENTIFIER)*;
 * ```
 */
export class YulVariableNames {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  /**
   * Constructs a new AST node of type `YulVariableNames`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulVariableNames`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulVariableNames);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `YulArguments` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulArguments = ((* item: *) YulExpression ((* separator: *) COMMA (* item: *) YulExpression)*)?;
 * ```
 */
export class YulArguments {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new YulExpression(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `YulArguments`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulArguments`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulArguments);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly YulExpression[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `YulPaths` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulPaths = (* item: *) YulPath ((* separator: *) COMMA (* item: *) YulPath)*;
 * ```
 */
export class YulPaths {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return {
      items: items!.map((item) => new YulPath(item as NonterminalNode)),
      separators: separators as TerminalNode[],
    };
  });

  /**
   * Constructs a new AST node of type `YulPaths`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulPaths`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulPaths);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly YulPath[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

/**
 * This node represents a `YulPath` nonterminal, with the following structure:
 *
 * ```ebnf
 * YulPath = (* item: *) YUL_IDENTIFIER ((* separator: *) PERIOD (* item: *) YUL_IDENTIFIER)*;
 * ```
 */
export class YulPath {
  private readonly fetch = once(() => {
    const [items, separators] = wasm.ast.Selectors.separated(this.cst);

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  /**
   * Constructs a new AST node of type `YulPath`, given a nonterminal CST node of the same kind.
   */
  public constructor(
    /**
     * The underlying nonterminal CST node of kind `YulPath`.
     */
    public readonly cst: NonterminalNode,
  ) {
    assertKind(this.cst.kind, NonterminalKind.YulPath);
  }

  /**
   * Returns an array of the child nodes that have the label `item`.
   */
  public get items(): readonly TerminalNode[] {
    return this.fetch().items;
  }

  /**
   * Returns an array of the child nodes that have the label `separator`.
   */
  public get separators(): readonly TerminalNode[] {
    return this.fetch().separators;
  }
}

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
  if (actual !== expected) {
    throw new Error(
      `AST node '${expected}' can only be initialized with a CST node of the same kind. Received '${actual}' instead.`,
    );
  }
}
