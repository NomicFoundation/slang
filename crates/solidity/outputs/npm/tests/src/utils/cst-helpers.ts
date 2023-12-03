import { NodeType, RuleNode, TokenNode, Node } from "@nomicfoundation/slang/cst";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";

export function expectRule(node: Node, kind: RuleKind): asserts node is RuleNode {
  expect(node).toBeInstanceOf(RuleNode);
  expect(node.type).toEqual(NodeType.Rule);

  const rule = node as TokenNode;
  expect(rule.kind).toEqual(kind);
}

export function expectToken(node: Node, kind: TokenKind, text: string): asserts node is TokenNode {
  expect(node).toBeInstanceOf(TokenNode);
  expect(node.type).toEqual(NodeType.Token);

  const token = node as TokenNode;
  expect(token.kind).toEqual(kind);
  expect(token.text).toEqual(text);
}
