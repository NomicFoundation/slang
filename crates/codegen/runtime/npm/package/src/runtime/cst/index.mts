import * as wasm from "../../../wasm/index.mjs";

export * from "./assertions.mjs";
export * from "./extensions.mjs";
export * from "./generated/rewriter.mjs";

/** {@inheritDoc wasm.cst.NonterminalKind} */
export const NonterminalKind = wasm.cst.NonterminalKind;
/** {@inheritDoc wasm.cst.NonterminalKind} */
export type NonterminalKind = wasm.cst.NonterminalKind;

/** {@inheritDoc wasm.cst.TerminalKind} */
export const TerminalKind = wasm.cst.TerminalKind;
/** {@inheritDoc wasm.cst.TerminalKind} */
export type TerminalKind = wasm.cst.TerminalKind;

/** {@inheritDoc wasm.cst.TerminalKindExtensions} */
export const TerminalKindExtensions = wasm.cst.TerminalKindExtensions;
/** {@inheritDoc wasm.cst.TerminalKindExtensions} */
export type TerminalKindExtensions = wasm.cst.TerminalKindExtensions;

/** {@inheritDoc wasm.cst.EdgeLabel} */
export const EdgeLabel = wasm.cst.EdgeLabel;
/** {@inheritDoc wasm.cst.EdgeLabel} */
export type EdgeLabel = wasm.cst.EdgeLabel;

/** {@inheritDoc wasm.cst.Node} */
export type Node = wasm.cst.Node;

/** {@inheritDoc wasm.cst.NodeType} */
export const NodeType = wasm.cst.NodeType;
/** {@inheritDoc wasm.cst.NodeType} */
export type NodeType = wasm.cst.NodeType;

/** {@inheritDoc wasm.cst.NonterminalNode} */
export const NonterminalNode = wasm.cst.NonterminalNode;
/** {@inheritDoc wasm.cst.NonterminalNode} */
export type NonterminalNode = wasm.cst.NonterminalNode;

/** {@inheritDoc wasm.cst.TerminalNode} */
export const TerminalNode = wasm.cst.TerminalNode;
/** {@inheritDoc wasm.cst.TerminalNode} */
export type TerminalNode = wasm.cst.TerminalNode;

/** {@inheritDoc wasm.cst.Edge} */
export const Edge = wasm.cst.Edge;
/** {@inheritDoc wasm.cst.Edge} */
export type Edge = wasm.cst.Edge;

/** {@inheritDoc wasm.cst.Cursor} */
export const Cursor = wasm.cst.Cursor;
/** {@inheritDoc wasm.cst.Cursor} */
export type Cursor = wasm.cst.Cursor;

/** {@inheritDoc wasm.cst.CursorIterator} */
export const CursorIterator = wasm.cst.CursorIterator;
/** {@inheritDoc wasm.cst.CursorIterator} */
export type CursorIterator = wasm.cst.CursorIterator;

/** {@inheritDoc wasm.cst.AncestorsIterator} */
export const AncestorsIterator = wasm.cst.AncestorsIterator;
/** {@inheritDoc wasm.cst.AncestorsIterator} */
export type AncestorsIterator = wasm.cst.AncestorsIterator;

/** {@inheritDoc wasm.cst.Query} */
export const Query = wasm.cst.Query;
/** {@inheritDoc wasm.cst.Query} */
export type Query = wasm.cst.Query;

/** {@inheritDoc wasm.cst.QueryError} */
export type QueryError = wasm.cst.QueryError;

/** {@inheritDoc wasm.cst.QueryMatch} */
export type QueryMatch = wasm.cst.QueryMatch;

/** {@inheritDoc wasm.cst.QueryMatchIterator} */
export const QueryMatchIterator = wasm.cst.QueryMatchIterator;
/** {@inheritDoc wasm.cst.QueryMatchIterator} */
export type QueryMatchIterator = wasm.cst.QueryMatchIterator;

/** {@inheritDoc wasm.cst.TextIndex} */
export type TextIndex = wasm.cst.TextIndex;

/** {@inheritDoc wasm.cst.TextRange} */
export type TextRange = wasm.cst.TextRange;
