import { NodeType, RuleNode, TokenNode } from "@nomicfoundation/slang/cst";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";

export function expectRule(node: unknown, kind: RuleKind): asserts node is RuleNode {
  expect(node).toBeInstanceOf(RuleNode);

  const rule = node as RuleNode;
  expect(rule.type).toEqual(NodeType.Rule);
  expect(rule.kind).toEqual(kind);
}

export function expectToken(node: unknown, kind: TokenKind, text: string): asserts node is TokenNode {
  expect(node).toBeInstanceOf(TokenNode);

  const token = node as TokenNode;
  expect(token.type).toEqual(NodeType.Token);
  expect(token.kind).toEqual(kind);
  expect(token.text).toEqual(text);
}
