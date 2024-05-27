import { NodeType, NonterminalNode, TerminalNode } from "@slang-private/slang-testlang/cst";
import { NonterminalKind, TerminalKind } from "@slang-private/slang-testlang/kinds";

export function expectNonterminal(node: unknown, kind: NonterminalKind): asserts node is NonterminalNode {
  expect(node).toBeInstanceOf(NonterminalNode);

  const nonTerminal = node as NonterminalNode;
  expect(nonTerminal.type).toEqual(NodeType.Nonterminal);
  expect(nonTerminal.kind).toEqual(kind);
}

export function expectTerminal(node: unknown, kind: TerminalKind, text: string): asserts node is TerminalNode {
  expect(node).toBeInstanceOf(TerminalNode);

  const terminal = node as TerminalNode;
  expect(terminal.type).toEqual(NodeType.Terminal);
  expect(terminal.kind).toEqual(kind);
  expect(terminal.text).toEqual(text);
}
