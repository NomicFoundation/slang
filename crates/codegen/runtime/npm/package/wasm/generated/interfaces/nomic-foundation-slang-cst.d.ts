// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangCst {
  export { TerminalKindExtensions };
  export { NonterminalNode };
  export { TerminalNode };
  export { Edge };
  export { Cursor };
  export { CursorIterator };
  export { AncestorsIterator };
  export { Query };
  export { QueryMatchIterator };
  export { NonterminalKind };
  export { TerminalKind };
  export { EdgeLabel };
  export { Node };
  export { NodeType };
}
/**
 * Represents different kinds of nonterminal nodes in the syntax tree.
 * These are nodes that can have child nodes and represent higher-level language constructs.
 */
export declare enum NonterminalKind {
  /**
   * Generated stub.
   */
  Stub1 = "Stub1",
  /**
   * Generated stub.
   */
  Stub2 = "Stub2",
  /**
   * Generated stub.
   */
  Stub3 = "Stub3",
}
/**
 * Represents different kinds of terminal nodes in the syntax tree.
 * These are leaf nodes that represent actual tokens in the source code.
 */
export declare enum TerminalKind {
  /**
   * This terminal is created when the parser encounters an unexpected part of the input,
   * and it cannot recognize it as any valid syntax in this position in the grammar.
   */
  Unrecognized = "Unrecognized",
  /**
   * This terminal is created when the parser is expecting a certain terminal, but it cannot find it.
   * Adding the missing input in this position may allow the parser to produce a valid tree there.
   */
  Missing = "Missing",
  /**
   * Generated stub.
   */
  Stub1 = "Stub1",
  /**
   * Generated stub.
   */
  Stub2 = "Stub2",
  /**
   * Generated stub.
   */
  Stub3 = "Stub3",
}
/**
 * Represents the different types of relationships between nodes in the syntax tree.
 */
export declare enum EdgeLabel {
  /**
   * Represents a child node with the label `root`.
   */
  Root = "Root",
  /**
   * Represents a child node with the label `unrecognized`.
   */
  Unrecognized = "Unrecognized",
  /**
   * Represents a child node with the label `missing`.
   */
  Missing = "Missing",
  /**
   * Represents a child node with the label `item`.
   */
  Item = "Item",
  /**
   * Represents a child node with the label `variant`.
   */
  Variant = "Variant",
  /**
   * Represents a child node with the label `separator`.
   */
  Separator = "Separator",
  /**
   * Represents a child node with the label `operand`.
   */
  Operand = "Operand",
  /**
   * Represents a child node with the label `left_operand`.
   */
  LeftOperand = "LeftOperand",
  /**
   * Represents a child node with the label `right_operand`.
   */
  RightOperand = "RightOperand",
  /**
   * Represents a child node with the label `leading_trivia`.
   */
  LeadingTrivia = "LeadingTrivia",
  /**
   * Represents a child node with the label `trailing_trivia`.
   */
  TrailingTrivia = "TrailingTrivia",
  /**
   * Generated stub.
   */
  Stub1 = "Stub1",
  /**
   * Generated stub.
   */
  Stub2 = "Stub2",
  /**
   * Generated stub.
   */
  Stub3 = "Stub3",
}
/**
 * The super type of all nodes in a tree.
 */
export type Node = NonterminalNode | TerminalNode;

/**
 * Enumerates different variants of the `Node` type.
 */
export enum NodeType {
  /**
   * Represents a variant of type `NonterminalNode`.
   */
  NonterminalNode = "NonterminalNode",

  /**
   * Represents a variant of type `TerminalNode`.
   */
  TerminalNode = "TerminalNode",
}
/**
 * Represents a match found by executing queries on a cursor.
 */
export interface QueryMatch {
  /**
   * The index of the query that produced this match.
   */
  queryIndex: number;
  /**
   * A cursor to the matched root node.
   */
  root: Cursor;
  /**
   * List of captured nodes and their names from the query.
   */
  captures: { [key: string]: Array<Cursor> };
}
/**
 * Represents a position in the source text, with indices for different unicode encodings of the source.
 */
export interface TextIndex {
  /**
   * Byte offset in UTF-8 encoding.
   * This is useful when working with languages like Rust that use UTF-8.
   */
  utf8: number;
  /**
   * Byte offset in UTF-8 encoding.
   * This is useful when working with languages like JavaScript that use UTF-16.
   */
  utf16: number;
  /**
   * Line number (0-based).
   * Lines are separated by:
   *
   * - carriage return `\r`.
   * - newline `\n`.
   * - line separator `\u2028`.
   * - paragraph separator `\u2029`.
   */
  line: number;
  /**
   * Column number (0-based).
   * Columns are counted in [unicode scalar values](https://www.unicode.org/glossary/#unicode_scalar_value).
   */
  column: number;
}
/**
 * Represents a range in the source text.
 */
export interface TextRange {
  /**
   * Starting (inclusive) position of the range.
   */
  start: TextIndex;
  /**
   * Ending (exclusive) position of the range.
   */
  end: TextIndex;
}
/**
 * Represents an error that occurred while parsing a query.
 */
export interface QueryError {
  /**
   * A human-readable message describing what went wrong.
   */
  message: string;
  /**
   * The text range where the error occurred in the query code.
   */
  textRange: TextRange;
}

/**
 * Iterator over all ancestors of the current node, starting with the immediate parent, and moving upwards, ending with the root node.
 */

export class AncestorsIterator {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * Returns an iterator over `NonterminalNode` objects. Called by the semantics of the for-of statement.
   */
  [Symbol.iterator](): Iterator<NonterminalNode>;
  /**
   * Returns the next nonterminal node in the iteration, or `undefined` if there are no more nodes.
   */
  next(): NonterminalNode | undefined;
}

/**
 * Provides navigation and traversal capabilities over the syntax tree.
 */

export class Cursor {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Resets the cursor to its initial position.
   */
  reset(): void;
  /**
   * Marks the cursor as completed.
   */
  complete(): void;
  /**
   * Returns whether the cursor has completed its traversal.
   */
  isCompleted(): boolean;
  /**
   * Creates a copy of this cursor at its current position with the same ancestors.
   * The new cursor can be moved independently without affecting the original cursor.
   */
  clone(): Cursor;
  /**
   * Creates a copy of this cursor at its current position, but without any ancestors.
   * This is useful for limiting the scope of a search or query to the sub-tree only, without backtracking to parent nodes.
   * The new cursor can be moved independently without affecting the original cursor.
   */
  spawn(): Cursor;
  /**
   * Returns the current node under the cursor.
   */
  get node(): Node;
  /**
   * Returns a label that describes the relationship between the current node and its parent.
   */
  get label(): EdgeLabel;
  /**
   * Returns the current text offset of the cursor.
   */
  get textOffset(): TextIndex;
  /**
   * Returns the text range covered by the current node.
   */
  get textRange(): TextRange;
  /**
   * Returns the current depth in the tree (i.e. number of ancestors).
   */
  get depth(): number;
  /**
   * Returns the list of child edges directly connected to this node.
   */
  children(): Array<Edge>;
  /**
   * Returns an iterator over all descendants of the current node in pre-order traversal.
   */
  descendants(): CursorIterator;
  /**
   * Returns an iterator over all the remaining nodes in the current tree, moving in pre-order traversal, until the tree is completed.
   */
  remainingNodes(): CursorIterator;
  /**
   * Returns an iterator over all ancestors of the current node, starting with the immediate parent, and moving upwards, ending with the root node.
   */
  ancestors(): AncestorsIterator;
  /**
   * Moves to the next node in pre-order traversal.
   */
  goToNext(): boolean;
  /**
   * Moves to the next node that isn't a descendant of the current node.
   */
  goToNextNonDescendant(): boolean;
  /**
   * Moves to the previous node in pre-order traversal.
   */
  goToPrevious(): boolean;
  /**
   * Moves up to the parent node.
   */
  goToParent(): boolean;
  /**
   * Moves to the first child of the current node.
   */
  goToFirstChild(): boolean;
  /**
   * Moves to the last child of the current node.
   */
  goToLastChild(): boolean;
  /**
   * Moves to the nth child of the current node.
   */
  goToNthChild(childIndex: number): boolean;
  /**
   * Moves to the next sibling node.
   */
  goToNextSibling(): boolean;
  /**
   * Moves to the previous sibling node.
   */
  goToPreviousSibling(): boolean;
  /**
   * Moves to the next terminal node.
   */
  goToNextTerminal(): boolean;
  /**
   * Moves to the next terminal node of a specific kind.
   */
  goToNextTerminalWithKind(kind: TerminalKind): boolean;
  /**
   * Moves to the next terminal node matching any of the given kinds.
   */
  goToNextTerminalWithKinds(kinds: Array<TerminalKind>): boolean;
  /**
   * Nonterminal navigation methods
   * Moves to the next nonterminal node.
   */
  goToNextNonterminal(): boolean;
  /**
   * Moves to the next nonterminal node of a specific kind.
   */
  goToNextNonterminalWithKind(kind: NonterminalKind): boolean;
  /**
   * Moves to the next nonterminal node matching any of the given kinds.
   */
  goToNextNonterminalWithKinds(kinds: Array<NonterminalKind>): boolean;
  /**
   * Executes the given queries and returns an iterator over the matches.
   */
  query(queries: Array<Query>): QueryMatchIterator;
}

/**
 * Iterator over all the remaining nodes in the current tree, moving in pre-order traversal, until the tree is completed.
 */

export class CursorIterator {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * Returns an iterator over `Edge` objects. Called by the semantics of the for-of statement.
   */
  [Symbol.iterator](): Iterator<Edge>;
  /**
   * Returns the next edge in the iteration, or `undefined` if there are no more edges.
   */
  next(): Edge | undefined;
}

/**
 * Represents a connection between nodes in the syntax tree.
 */

export class Edge {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Creates a new edge connecting a terminal node `node` with the label `label`.
   */
  static createWithTerminal(label: EdgeLabel, node: TerminalNode): Edge;
  /**
   * Creates a new edge connecting a nonterminal node `node` with the label `label`.
   */
  static createWithNonterminal(label: EdgeLabel, node: NonterminalNode): Edge;
  /**
   * Describes the relationship between this node and its parent.
   */
  get label(): EdgeLabel;
  /**
   * The target node of this edge.
   */
  get node(): Node;
}

/**
 * Represents a non-terminal node in the syntax tree.
 * These nodes can have child nodes and represent language constructs.
 */

export class NonterminalNode {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * The variant of `NodeType` that corresponds to this class.
   */
  readonly type = NodeType.NonterminalNode;

  /**
   * Coerce this variant to a `NonterminalNode`, or `undefined` if this is not the correct type.
   */
  asNonterminalNode(): this;

  /**
   * Return `true` if this object is an instance of `NonterminalNode`.
   */
  isNonterminalNode(): this is NonterminalNode;

  /**
   * Coerce this variant to a `TerminalNode`, or `undefined` if this is not the correct type.
   */
  asTerminalNode(): undefined;

  /**
   * Return `true` if this object is an instance of `TerminalNode`.
   */
  isTerminalNode(): false;

  /**
   * Creates a new nonterminal node with the specified `kind` and `children`.
   */
  static create(kind: NonterminalKind, children: Array<Edge>): NonterminalNode;
  /**
   * Returns a unique numerical identifier of the node.
   * It is only valid for the lifetime of the enclosing tree.
   * It can change between multiple parses, even for the same source code input.
   */
  get id(): number;
  /**
   * Returns the kind enum of this nonterminal node.
   */
  get kind(): NonterminalKind;
  /**
   * Returns the length of the text span this node covers.
   */
  get textLength(): TextIndex;
  /**
   * Returns the list of child edges directly connected to this node.
   */
  children(): Array<Edge>;
  /**
   * Returns an iterator over all descendants of the current node in pre-order traversal.
   */
  descendants(): CursorIterator;
  /**
   * Converts the node and its children back to source code text.
   */
  unparse(): string;
  /**
   * Converts the node to a JSON representation for debugging.
   */
  toJson(): string;
  /**
   * Creates a cursor positioned at the given text offset within this node.
   */
  createCursor(textOffset: TextIndex): Cursor;
}

/**
 * Represents a tree query for pattern matching in the syntax tree.
 */

export class Query {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Parses a query string into a query object.
   *
   * It will throw a `QueryError` if the query syntax is invalid.
   */
  static create(text: string): Query;
}

/**
 * Iterator over query matches in the syntax tree.
 */

export class QueryMatchIterator {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * Returns an iterator over `QueryMatch` objects. Called by the semantics of the for-of statement.
   */
  [Symbol.iterator](): Iterator<QueryMatch>;
  /**
   * Returns the next match or `undefined` if there are no more matches.
   */
  next(): QueryMatch | undefined;
}

/**
 * Useful extension methods for working with terminals and terminal kinds.
 */

export class TerminalKindExtensions {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Returns `true` if the terminal is an identifier token.
   */
  static isIdentifier(kind: TerminalKind): boolean;
  /**
   * Returns `true` if the terminal is a trivia token. i.e. whitespace, comments, etc...
   */
  static isTrivia(kind: TerminalKind): boolean;
  /**
   * Returns `true` if the terminal is a valid token in the language grammar.
   */
  static isValid(kind: TerminalKind): boolean;
}

/**
 * Represents a terminal node in the syntax tree.
 * These are leaf nodes that represent actual tokens from the source code.
 */

export class TerminalNode {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * The variant of `NodeType` that corresponds to this class.
   */
  readonly type = NodeType.TerminalNode;

  /**
   * Coerce this variant to a `TerminalNode`, or `undefined` if this is not the correct type.
   */
  asTerminalNode(): this;

  /**
   * Return `true` if this object is an instance of `TerminalNode`.
   */
  isTerminalNode(): this is TerminalNode;

  /**
   * Coerce this variant to a `NonterminalNode`, or `undefined` if this is not the correct type.
   */
  asNonterminalNode(): undefined;

  /**
   * Return `true` if this object is an instance of `NonterminalNode`.
   */
  isNonterminalNode(): false;

  /**
   * Creates a new terminal node with the specified `kind` and `text`.
   */
  static create(kind: TerminalKind, text: string): TerminalNode;
  /**
   * Returns a unique numerical identifier of the node.
   * It is only valid for the lifetime of the enclosing tree.
   * It can change between multiple parses, even for the same source code input.
   */
  get id(): number;
  /**
   * Returns the kind enum of this terminal node.
   */
  get kind(): TerminalKind;
  /**
   * Returns the length of the text span this node covers.
   */
  get textLength(): TextIndex;
  /**
   * Returns the list of child edges directly connected to this node.
   */
  children(): Array<Edge>;
  /**
   * Returns an iterator over all descendants of this node in pre-order traversal.
   */
  descendants(): CursorIterator;
  /**
   * Converts the node back to source code text.
   */
  unparse(): string;
  /**
   * Converts the node to a JSON representation for debugging.
   */
  toJson(): string;
}
