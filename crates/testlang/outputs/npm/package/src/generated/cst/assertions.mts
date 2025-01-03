// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import { NonterminalKind, NonterminalNode, TerminalKind, TerminalNode } from "./index.mjs";

/**
 * Asserts that this node is a `NonterminalNode` with the provided kind and text.
 */
export function assertIsNonterminalNode(
  node: unknown,
  kind?: NonterminalKind,
  text?: string,
): asserts node is NonterminalNode {
  if (!(node instanceof NonterminalNode)) {
    throw new Error("Node provided is not a NonterminalNode.");
  }

  if (kind !== undefined && kind !== node.kind) {
    throw new Error(`Node's NonterminalKind is expected to be '${kind}', but got '${node.kind}'.`);
  }

  if (text !== undefined && text !== node.unparse()) {
    throw new Error(`Node's text content is expected to be '${text}', but got '${node.unparse()}'.`);
  }
}

/**
 * Asserts that this node is a `TerminalKind` with the provided kind and text.
 */
export function assertIsTerminalNode(node: unknown, kind?: TerminalKind, text?: string): asserts node is TerminalNode {
  if (!(node instanceof TerminalNode)) {
    throw new Error("Node provided is not a TerminalNode.");
  }

  if (kind !== undefined && kind !== node.kind) {
    throw new Error(`Node's TerminalKind is expected to be '${kind}', but got '${node.kind}'.`);
  }

  if (text !== undefined && text !== node.unparse()) {
    throw new Error(`Node's text content is expected to be '${text}', but got '${node.unparse()}'.`);
  }
}
