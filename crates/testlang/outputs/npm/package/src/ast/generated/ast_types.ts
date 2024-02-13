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
      members: new SourceUnitMembers($members as RuleNode),
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.SourceUnit);
  }

  public get members(): SourceUnitMembers {
    return this.fetch().members;
  }
}

export class Tree {
  private readonly fetch = once(() => {
    const [$keyword, $name, $node, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      keyword: $keyword as TokenNode,
      name: $name === null ? undefined : ($name as TokenNode),
      node: new TreeNode($node as RuleNode),
      semicolon: $semicolon as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.Tree);
  }

  public get keyword(): TokenNode {
    return this.fetch().keyword;
  }

  public get name(): TokenNode | undefined {
    return this.fetch().name;
  }

  public get node(): TreeNode {
    return this.fetch().node;
  }

  public get semicolon(): TokenNode {
    return this.fetch().semicolon;
  }
}

export class TreeNode {
  private readonly fetch = once(() => {
    const [$openBracket, $members, $closeBracket] = ast_internal.selectSequence(this.cst);

    return {
      openBracket: $openBracket as TokenNode,
      members: new TreeNodeChildren($members as RuleNode),
      closeBracket: $closeBracket as TokenNode,
    };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TreeNode);
  }

  public get openBracket(): TokenNode {
    return this.fetch().openBracket;
  }

  public get members(): TreeNodeChildren {
    return this.fetch().members;
  }

  public get closeBracket(): TokenNode {
    return this.fetch().closeBracket;
  }
}

/*
 * Choices:
 */

export class SourceUnitMember {
  private readonly fetch: () => Tree | SeparatedIdentifiers | Literal = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.Tree:
        return new Tree(variant as RuleNode);
      case RuleKind.SeparatedIdentifiers:
        return new SeparatedIdentifiers(variant as RuleNode);
      case RuleKind.Literal:
        return new Literal(variant as RuleNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.SourceUnitMember);
  }

  public get variant(): Tree | SeparatedIdentifiers | Literal {
    return this.fetch();
  }
}

export class TreeNodeChild {
  private readonly fetch: () => TreeNode | TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case RuleKind.TreeNode:
        return new TreeNode(variant as RuleNode);

      case TokenKind.DelimitedIdentifier:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TreeNodeChild);
  }

  public get variant(): TreeNode | TokenNode {
    return this.fetch();
  }
}

export class Literal {
  private readonly fetch: () => TokenNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TokenKind.StringLiteral:
        return variant as TokenNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.Literal);
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

export class TreeNodeChildren {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new TreeNodeChild(item as RuleNode));
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.TreeNodeChildren);
  }

  public get items(): readonly TreeNodeChild[] {
    return this.fetch();
  }
}

/*
 * Separated:
 */

export class SeparatedIdentifiers {
  private readonly fetch = once(() => {
    const [items, separators] = ast_internal.selectSeparated(this.cst);

    return { items: items as TokenNode[], separators: separators as TokenNode[] };
  });

  public constructor(public readonly cst: RuleNode) {
    assertKind(this.cst.kind, RuleKind.SeparatedIdentifiers);
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
