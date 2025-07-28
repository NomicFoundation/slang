// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import { NonterminalKind, NonterminalNode, Node, TerminalNode, TerminalKind, Edge } from "../index.mjs";

export abstract class BaseRewriter {
  public rewriteNode(node: Node): Node | undefined {
    if (node instanceof NonterminalNode) {
      return this.rewriteNonterminalNode(node);
    } else {
      return this.rewriteTerminalNode(node);
    }
  }

  public rewriteNonterminalNode(node: NonterminalNode): Node | undefined {
    switch (node.kind) {
      case NonterminalKind.AdditionExpression:
        return this.rewriteAdditionExpression(node);

      case NonterminalKind.Expression:
        return this.rewriteExpression(node);

      case NonterminalKind.Literal:
        return this.rewriteLiteral(node);

      case NonterminalKind.MemberAccessExpression:
        return this.rewriteMemberAccessExpression(node);

      case NonterminalKind.NegationExpression:
        return this.rewriteNegationExpression(node);

      case NonterminalKind.SeparatedIdentifiers:
        return this.rewriteSeparatedIdentifiers(node);

      case NonterminalKind.SourceUnit:
        return this.rewriteSourceUnit(node);

      case NonterminalKind.SourceUnitMember:
        return this.rewriteSourceUnitMember(node);

      case NonterminalKind.SourceUnitMembers:
        return this.rewriteSourceUnitMembers(node);

      case NonterminalKind.Tree:
        return this.rewriteTree(node);

      case NonterminalKind.TreeNode:
        return this.rewriteTreeNode(node);

      case NonterminalKind.TreeNodeChild:
        return this.rewriteTreeNodeChild(node);

      case NonterminalKind.TreeNodeChildren:
        return this.rewriteTreeNodeChildren(node);
    }
  }

  public rewriteTerminalNode(node: TerminalNode): Node | undefined {
    switch (node.kind) {
      case TerminalKind.Bang:
        return this.rewriteBang(node);

      case TerminalKind.CloseBracket:
        return this.rewriteCloseBracket(node);

      case TerminalKind.DelimitedIdentifier:
        return this.rewriteDelimitedIdentifier(node);

      case TerminalKind.EndOfLine:
        return this.rewriteEndOfLine(node);

      case TerminalKind.Identifier:
        return this.rewriteIdentifier(node);

      case TerminalKind.MultiLineComment:
        return this.rewriteMultiLineComment(node);

      case TerminalKind.OpenBracket:
        return this.rewriteOpenBracket(node);

      case TerminalKind.Period:
        return this.rewritePeriod(node);

      case TerminalKind.Plus:
        return this.rewritePlus(node);

      case TerminalKind.Semicolon:
        return this.rewriteSemicolon(node);

      case TerminalKind.SingleLineComment:
        return this.rewriteSingleLineComment(node);

      case TerminalKind.StringLiteral:
        return this.rewriteStringLiteral(node);

      case TerminalKind.TreeKeyword:
        return this.rewriteTreeKeyword(node);

      case TerminalKind.Whitespace:
        return this.rewriteWhitespace(node);
    }
    // NOTE: this shouldn't be necessary, beacuse the case above should cover all, however
    // TypeScript for some reason fails to notice the completeness of this switch.
    throw Error("Unreachable");
  }

  /** @virtual */
  public rewriteAdditionExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.AdditionExpression, node);
  }

  /** @virtual */
  public rewriteExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Expression, node);
  }

  /** @virtual */
  public rewriteLiteral(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Literal, node);
  }

  /** @virtual */
  public rewriteMemberAccessExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.MemberAccessExpression, node);
  }

  /** @virtual */
  public rewriteNegationExpression(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.NegationExpression, node);
  }

  /** @virtual */
  public rewriteSeparatedIdentifiers(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.SeparatedIdentifiers, node);
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
  public rewriteTree(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.Tree, node);
  }

  /** @virtual */
  public rewriteTreeNode(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TreeNode, node);
  }

  /** @virtual */
  public rewriteTreeNodeChild(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TreeNodeChild, node);
  }

  /** @virtual */
  public rewriteTreeNodeChildren(node: NonterminalNode): Node | undefined {
    return this.rewriteChildren(NonterminalKind.TreeNodeChildren, node);
  }

  /** @virtual */
  public rewriteBang(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteCloseBracket(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteDelimitedIdentifier(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteEndOfLine(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteIdentifier(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteMultiLineComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteOpenBracket(node: TerminalNode): Node | undefined {
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
  public rewriteSemicolon(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteSingleLineComment(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteStringLiteral(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteTreeKeyword(node: TerminalNode): Node | undefined {
    return node;
  }

  /** @virtual */
  public rewriteWhitespace(node: TerminalNode): Node | undefined {
    return node;
  }

  // generic implementation that only recreates 'node' if children change:
  protected rewriteChildren(kind: NonterminalKind, node: NonterminalNode): NonterminalNode {
    const newChildren: Array<Edge> = new Array<Edge>();
    var anyChildChanged = false;
    for (const child of node.children()) {
      const newChild = this.rewriteNode(child.node);
      if (newChild == undefined) {
        // node was removed
        anyChildChanged = true;
      } else if (newChild.id != child.node.id) {
        // node was changed
        anyChildChanged = true;
        if (newChild instanceof TerminalNode) {
          newChildren.push(Edge.createWithTerminal(child.label, newChild));
        } else {
          newChildren.push(Edge.createWithNonterminal(child.label, newChild));
        }
      }
    }
    if (anyChildChanged) {
      return NonterminalNode.create(kind, newChildren);
    } else {
      return node;
    }
  }
}
