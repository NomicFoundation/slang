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

export class Tree {
  private readonly fetch = once(() => {
    const [$keyword, $name, $node, $semicolon] = ast_internal.selectSequence(this.cst);

    return {
      keyword: $keyword as TerminalNode,
      name: $name === null ? undefined : ($name as TerminalNode),
      node: new TreeNode($node as NonTerminalNode),
      semicolon: $semicolon as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Tree);
  }

  public get keyword(): TerminalNode {
    return this.fetch().keyword;
  }

  public get name(): TerminalNode | undefined {
    return this.fetch().name;
  }

  public get node(): TreeNode {
    return this.fetch().node;
  }

  public get semicolon(): TerminalNode {
    return this.fetch().semicolon;
  }
}

export class TreeNode {
  private readonly fetch = once(() => {
    const [$openBracket, $members, $closeBracket] = ast_internal.selectSequence(this.cst);

    return {
      openBracket: $openBracket as TerminalNode,
      members: new TreeNodeChildren($members as NonTerminalNode),
      closeBracket: $closeBracket as TerminalNode,
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TreeNode);
  }

  public get openBracket(): TerminalNode {
    return this.fetch().openBracket;
  }

  public get members(): TreeNodeChildren {
    return this.fetch().members;
  }

  public get closeBracket(): TerminalNode {
    return this.fetch().closeBracket;
  }
}

export class AdditionExpression {
  private readonly fetch = once(() => {
    const [$leftOperand, $operator, $rightOperand] = ast_internal.selectSequence(this.cst);

    return {
      leftOperand: new Expression($leftOperand as NonTerminalNode),
      operator: $operator as TerminalNode,
      rightOperand: new Expression($rightOperand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.AdditionExpression);
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

export class NegationExpression {
  private readonly fetch = once(() => {
    const [$operator, $operand] = ast_internal.selectSequence(this.cst);

    return {
      operator: $operator as TerminalNode,
      operand: new Expression($operand as NonTerminalNode),
    };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.NegationExpression);
  }

  public get operator(): TerminalNode {
    return this.fetch().operator;
  }

  public get operand(): Expression {
    return this.fetch().operand;
  }
}

export class MemberAccessExpression {
  private readonly fetch = once(() => {
    const [$operand, $period, $member] = ast_internal.selectSequence(this.cst);

    return {
      operand: new Expression($operand as NonTerminalNode),
      period: $period as TerminalNode,
      member: $member as TerminalNode,
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

  public get member(): TerminalNode {
    return this.fetch().member;
  }
}

/*
 * Choices:
 */

export class SourceUnitMember {
  private readonly fetch: () => Tree | Expression | SeparatedIdentifiers | Literal = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.Tree:
        return new Tree(variant as NonTerminalNode);
      case NonTerminalKind.Expression:
        return new Expression(variant as NonTerminalNode);
      case NonTerminalKind.SeparatedIdentifiers:
        return new SeparatedIdentifiers(variant as NonTerminalNode);
      case NonTerminalKind.Literal:
        return new Literal(variant as NonTerminalNode);

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.SourceUnitMember);
  }

  public get variant(): Tree | Expression | SeparatedIdentifiers | Literal {
    return this.fetch();
  }
}

export class TreeNodeChild {
  private readonly fetch: () => TreeNode | TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case NonTerminalKind.TreeNode:
        return new TreeNode(variant as NonTerminalNode);

      case TerminalKind.DelimitedIdentifier:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TreeNodeChild);
  }

  public get variant(): TreeNode | TerminalNode {
    return this.fetch();
  }
}

export class Expression {
  private readonly fetch: () => AdditionExpression | NegationExpression | MemberAccessExpression | TerminalNode = once(
    () => {
      const variant = ast_internal.selectChoice(this.cst);

      switch (variant.kind) {
        case NonTerminalKind.AdditionExpression:
          return new AdditionExpression(variant as NonTerminalNode);
        case NonTerminalKind.NegationExpression:
          return new NegationExpression(variant as NonTerminalNode);
        case NonTerminalKind.MemberAccessExpression:
          return new MemberAccessExpression(variant as NonTerminalNode);

        case TerminalKind.StringLiteral:
        case TerminalKind.Identifier:
          return variant as TerminalNode;

        default:
          assert.fail(`Unexpected variant: ${variant.kind}`);
      }
    },
  );

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Expression);
  }

  public get variant(): AdditionExpression | NegationExpression | MemberAccessExpression | TerminalNode {
    return this.fetch();
  }
}

export class Literal {
  private readonly fetch: () => TerminalNode = once(() => {
    const variant = ast_internal.selectChoice(this.cst);

    switch (variant.kind) {
      case TerminalKind.StringLiteral:
        return variant as TerminalNode;

      default:
        assert.fail(`Unexpected variant: ${variant.kind}`);
    }
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.Literal);
  }

  public get variant(): TerminalNode {
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

export class TreeNodeChildren {
  private readonly fetch = once(() => {
    const items = ast_internal.selectRepeated(this.cst);
    return items.map((item) => new TreeNodeChild(item as NonTerminalNode));
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.TreeNodeChildren);
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

    return { items: items as TerminalNode[], separators: separators as TerminalNode[] };
  });

  public constructor(public readonly cst: NonTerminalNode) {
    assertKind(this.cst.kind, NonTerminalKind.SeparatedIdentifiers);
  }

  public get items(): readonly TerminalNode[] {
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
