// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangCst {
  export { TerminalKindExtensions };
  export { NonterminalNode };
  export { TerminalNode };
  export { Cursor };
  export { Query };
  export { QueryMatchIterator };
  export { NonterminalKind };
  export { TerminalKind };
  export { EdgeLabel };
  export { Node };
  export { NodeVariant };
}
/**
 * Represents different kinds of nonterminal nodes in the syntax tree.
 * These are nodes that can have child nodes and represent higher-level language constructs.
 */
export declare enum NonterminalKind {
  AdditionExpression = "AdditionExpression",
  Expression = "Expression",
  Literal = "Literal",
  MemberAccessExpression = "MemberAccessExpression",
  NegationExpression = "NegationExpression",
  SeparatedIdentifiers = "SeparatedIdentifiers",
  SourceUnit = "SourceUnit",
  SourceUnitMember = "SourceUnitMember",
  SourceUnitMembers = "SourceUnitMembers",
  Tree = "Tree",
  TreeNode = "TreeNode",
  TreeNodeChild = "TreeNodeChild",
  TreeNodeChildren = "TreeNodeChildren",
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
  Bang = "Bang",
  CloseBracket = "CloseBracket",
  DelimitedIdentifier = "DelimitedIdentifier",
  EndOfLine = "EndOfLine",
  Identifier = "Identifier",
  MultiLineComment = "MultiLineComment",
  OpenBracket = "OpenBracket",
  Period = "Period",
  Plus = "Plus",
  Semicolon = "Semicolon",
  SingleLineComment = "SingleLineComment",
  StringLiteral = "StringLiteral",
  TreeKeyword = "TreeKeyword",
  Whitespace = "Whitespace",
}
/**
 * Represents the different types of relationships between nodes in the syntax tree.
 */
export declare enum EdgeLabel {
  Item = "Item",
  Variant = "Variant",
  Separator = "Separator",
  Operand = "Operand",
  LeftOperand = "LeftOperand",
  RightOperand = "RightOperand",
  LeadingTrivia = "LeadingTrivia",
  TrailingTrivia = "TrailingTrivia",
  CloseBracket = "CloseBracket",
  Keyword = "Keyword",
  Member = "Member",
  Members = "Members",
  Name = "Name",
  Node = "Node",
  OpenBracket = "OpenBracket",
  Operator = "Operator",
  Period = "Period",
  Semicolon = "Semicolon",
}
/**
 * The super type of all nodes in a tree.
 */
export type Node = NonterminalNode | TerminalNode;
export enum NodeVariant {
  NonterminalNode = "NonterminalNode",
  TerminalNode = "TerminalNode",
}
/**
 * Represents a connection between nodes in the syntax tree.
 */
export interface Edge {
  /**
   * Optional label describing the relationship between nodes.
   */
  label?: EdgeLabel;
  /**
   * The target node of this edge.
   */
  node: Node;
}
/**
 * Represents an error that occurred while parsing a query.
 */
export interface QueryError {
  /**
   * The error message describing what went wrong.
   */
  message: string;
  /**
   * The line number where the error occurred.
   */
  line: number;
  /**
   * The column number where the error occurred.
   */
  column: number;
}
/**
 * Represents a match found by executing a query.
 */
export interface QueryMatch {
  /**
   * The index of the query that produced this match.
   */
  queryNumber: number;
  /**
   * List of captured nodes and their names from the query.
   */
  captures: { [key: string]: Cursor[] };
}
/**
 * Represents a position in the source text, with indices for different unicode encodings of the source.
 */
export interface TextIndex {
  /**
   * Byte offset in UTF-8 encoding.
   */
  utf8: number;
  /**
   * Character offset in UTF-16 encoding.
   */
  utf16: number;
  /**
   * Line number (0-based).
   */
  line: number;
  /**
   * Column number (0-based).
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

export class Cursor {
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
   * Returns the label of the edge from the parent to the current node, if any.
   */
  get label(): EdgeLabel | undefined;
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
   * Returns the list of ancestor nodes up to the root.
   */
  get ancestors(): NonterminalNode[];
  /**
   * Moves to the next node in pre-order traversal.
   */
  goToNext(): boolean;
  /**
   * Moves to the next node that isn't a descendant of the current node.
   */
  goToNextNonDescendent(): boolean;
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
  goToNthChild(childNumber: number): boolean;
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
  goToNextTerminalWithKinds(kinds: TerminalKind[]): boolean;
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
  goToNextNonterminalWithKinds(kinds: NonterminalKind[]): boolean;
  /**
   * Executes the given queries and returns an iterator over the matches.
   */
  query(queries: Query[]): QueryMatchIterator;
}

export class NonterminalNode {
  readonly nodeVariant = NodeVariant.NonterminalNode;

  asNonterminalNode(): this;
  isNonterminalNode(): this is NonterminalNode;

  asTerminalNode(): undefined;
  isTerminalNode(): false;

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
   * Returns the list of child edges connected to this node.
   */
  get children(): Edge[];
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

export class Query {
  /**
   * Parses a query string into a query object.
   * Throws an error if the query syntax is invalid.
   */
  static parse(text: string): Query;
}

export class QueryMatchIterator {
  [Symbol.iterator](): Iterator<QueryMatch>;
  /**
   * Returns the next match or None if there are no more matches.
   */
  next(): QueryMatch | undefined;
}

export class TerminalKindExtensions {
  /**
   * Returns true if the terminal is a trivia token. i.e. whitespace, comments, etc...
   */
  static isTrivia(kind: TerminalKind): boolean;
  /**
   * Returns true if the terminal is a valid token in the language grammar.
   */
  static isValid(kind: TerminalKind): boolean;
}

export class TerminalNode {
  readonly nodeVariant = NodeVariant.TerminalNode;

  asTerminalNode(): this;
  isTerminalNode(): this is TerminalNode;

  asNonterminalNode(): undefined;
  isNonterminalNode(): false;

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
   * Returns the list of child edges connected to this node.
   */
  get children(): Edge[];
  /**
   * Converts the node back to source code text.
   */
  unparse(): string;
  /**
   * Converts the node to a JSON representation for debugging.
   */
  toJson(): string;
}
