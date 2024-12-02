// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import * as generated from "../../../wasm/index.mjs";

export const NonterminalKind = generated.cst.NonterminalKind;
export type NonterminalKind = generated.cst.NonterminalKind;

export const TerminalKind = generated.cst.TerminalKind;
export type TerminalKind = generated.cst.TerminalKind;

export const TerminalKindExtensions = generated.cst.TerminalKindExtensions;
export type TerminalKindExtensions = generated.cst.TerminalKindExtensions;

export const EdgeLabel = generated.cst.EdgeLabel;
export type EdgeLabel = generated.cst.EdgeLabel;

export type Node = generated.cst.Node;

export const NodeVariant = generated.cst.NodeKind;
export type NodeVariant = generated.cst.NodeKind;

export const NonterminalNode = generated.cst.NonterminalNode;
export type NonterminalNode = generated.cst.NonterminalNode;

export const TerminalNode = generated.cst.TerminalNode;
export type TerminalNode = generated.cst.TerminalNode;

export type Edge = generated.cst.Edge;

export const Cursor = generated.cst.Cursor;
export type Cursor = generated.cst.Cursor;

export const CursorIterator = generated.cst.CursorIterator;
export type CursorIterator = generated.cst.CursorIterator;

export const AncestorsIterator = generated.cst.AncestorsIterator;
export type AncestorsIterator = generated.cst.AncestorsIterator;

export const Query = generated.cst.Query;
export type Query = generated.cst.Query;

export type QueryError = generated.cst.QueryError;

export type QueryMatch = generated.cst.QueryMatch;

export const QueryMatchIterator = generated.cst.QueryMatchIterator;
export type QueryMatchIterator = generated.cst.QueryMatchIterator;

export type TextIndex = generated.cst.TextIndex;

export type TextRange = generated.cst.TextRange;

/*
 * Helpers:
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
