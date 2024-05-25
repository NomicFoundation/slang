import { NodeType, NonTerminalNode, TerminalNode } from "@slang-private/slang-testlang/cst";
import { NonTerminalKind, TerminalKind } from "@slang-private/slang-testlang/kinds";

export function expectNonTerminal(node: unknown, kind: NonTerminalKind): asserts node is NonTerminalNode {
  expect(node).toBeInstanceOf(NonTerminalNode);

  const nonTerminal = node as NonTerminalNode;
  expect(nonTerminal.type).toEqual(NodeType.NonTerminal);
  expect(nonTerminal.kind).toEqual(kind);
}

export function expectTerminal(node: unknown, kind: TerminalKind, text: string): asserts node is TerminalNode {
  expect(node).toBeInstanceOf(TerminalNode);

  const terminal = node as TerminalNode;
  expect(terminal.type).toEqual(NodeType.Terminal);
  expect(terminal.kind).toEqual(kind);
  expect(terminal.text).toEqual(text);
}
