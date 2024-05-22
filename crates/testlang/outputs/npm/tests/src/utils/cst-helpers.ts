import { NodeType, NonTerminalNode, TerminalNode } from "@slang-private/slang-testlang/cst";
import { NonTerminalKind, TerminalKind } from "@slang-private/slang-testlang/kinds";

export function expectNonTerminal(node: unknown, kind: NonTerminalKind): asserts node is NonTerminalNode {
  expect(node).toBeInstanceOf(NonTerminalNode);

  const rule = node as NonTerminalNode;
  expect(rule.type).toEqual(NodeType.NonTerminal);
  expect(rule.kind).toEqual(kind);
}

export function expectTerminal(node: unknown, kind: TerminalKind, text: string): asserts node is TerminalNode {
  expect(node).toBeInstanceOf(TerminalNode);

  const token = node as TerminalNode;
  expect(token.type).toEqual(NodeType.Terminal);
  expect(token.kind).toEqual(kind);
  expect(token.text).toEqual(text);
}
