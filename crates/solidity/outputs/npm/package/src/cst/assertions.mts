import { NonterminalKind, NonterminalNode, TerminalKind, TerminalNode } from "./index.mjs";

/**
 * Asserts that `node` is a `NonterminalNode`.
 *
 * If a `kind` value is provided, it will also assert that it matches its `NonterminalKind`.
 *
 * If a `text` value is provided, it will also be asserted against the node contents.
 */
export function assertNonterminalNode(
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
 * Asserts that `node` is a `TerminalNode`.
 *
 * If a `kind` value is provided, it will also assert that it matches its `TerminalKind`.
 *
 * If a `text` value is provided, it will also be asserted against the node contents.
 */
export function assertTerminalNode(node: unknown, kind?: TerminalKind, text?: string): asserts node is TerminalNode {
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
