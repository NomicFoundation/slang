import { NodeType, RuleNode, TokenNode, InvalidNode } from "@slang-private/slang-testlang/cst";
import { RuleKind, TokenKind } from "@slang-private/slang-testlang/kinds";

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

export function expectInvalid(node: unknown): asserts node is InvalidNode {
  expect(node).toBeInstanceOf(InvalidNode);
}
