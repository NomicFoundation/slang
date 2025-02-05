// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangCst {
  export { TerminalKindExtensions };
  export { NonterminalNode };
  export { TerminalNode };
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
   * Represents a node with kind `AdditionExpression`, having the following structure:
   *
   * ```ebnf
   * (* Left-associative binary operator *)
   * AdditionExpression = (* left_operand: *) Expression
   * (* operator: *) PLUS
   * (* right_operand: *) Expression;
   * ```
   */
  AdditionExpression = "AdditionExpression",
  /**
   * Represents a node with kind `Expression`, having the following structure:
   *
   * ```ebnf
   * Expression = (* variant: *) AdditionExpression
   * | (* variant: *) NegationExpression
   * | (* variant: *) MemberAccessExpression
   * | (* variant: *) STRING_LITERAL
   * | (* variant: *) IDENTIFIER;
   * ```
   */
  Expression = "Expression",
  /**
   * Represents a node with kind `Literal`, having the following structure:
   *
   * ```ebnf
   * Literal = (* variant: *) STRING_LITERAL;
   * ```
   */
  Literal = "Literal",
  /**
   * Represents a node with kind `MemberAccessExpression`, having the following structure:
   *
   * ```ebnf
   * (* Postfix unary operator *)
   * MemberAccessExpression = (* operand: *) Expression
   * (* period: *) PERIOD
   * (* member: *) IDENTIFIER;
   * ```
   */
  MemberAccessExpression = "MemberAccessExpression",
  /**
   * Represents a node with kind `NegationExpression`, having the following structure:
   *
   * ```ebnf
   * (* Prefix unary operator *)
   * NegationExpression = (* operator: *) BANG
   * (* operand: *) Expression;
   * ```
   */
  NegationExpression = "NegationExpression",
  /**
   * Represents a node with kind `SeparatedIdentifiers`, having the following structure:
   *
   * ```ebnf
   * (* Introduced in 1.0.0 *)
   * SeparatedIdentifiers = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
   * ```
   */
  SeparatedIdentifiers = "SeparatedIdentifiers",
  /**
   * Represents a node with kind `SourceUnit`, having the following structure:
   *
   * ```ebnf
   * SourceUnit = (* members: *) SourceUnitMembers;
   * ```
   */
  SourceUnit = "SourceUnit",
  /**
   * Represents a node with kind `SourceUnitMember`, having the following structure:
   *
   * ```ebnf
   * SourceUnitMember = (* variant: *) Tree
   * | (* variant: *) Expression
   * | (* variant: *) SeparatedIdentifiers
   * | (* variant: *) Literal;
   * ```
   */
  SourceUnitMember = "SourceUnitMember",
  /**
   * Represents a node with kind `SourceUnitMembers`, having the following structure:
   *
   * ```ebnf
   * SourceUnitMembers = (* item: *) SourceUnitMember+;
   * ```
   */
  SourceUnitMembers = "SourceUnitMembers",
  /**
   * Represents a node with kind `Tree`, having the following structure:
   *
   * ```ebnf
   * Tree = (* keyword: *) TREE_KEYWORD
   * (* name: *) IDENTIFIER?
   * (* node: *) TreeNode
   * (* semicolon: *) SEMICOLON;
   * ```
   */
  Tree = "Tree",
  /**
   * Represents a node with kind `TreeNode`, having the following structure:
   *
   * ```ebnf
   * TreeNode = (* open_bracket: *) OPEN_BRACKET
   * (* members: *) TreeNodeChildren
   * (* close_bracket: *) CLOSE_BRACKET;
   * ```
   */
  TreeNode = "TreeNode",
  /**
   * Represents a node with kind `TreeNodeChild`, having the following structure:
   *
   * ```ebnf
   * TreeNodeChild = (* variant: *) TreeNode
   * | (* variant: *) DELIMITED_IDENTIFIER;
   * ```
   */
  TreeNodeChild = "TreeNodeChild",
  /**
   * Represents a node with kind `TreeNodeChildren`, having the following structure:
   *
   * ```ebnf
   * TreeNodeChildren = (* item: *) TreeNodeChild+;
   * ```
   */
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
  /**
   * Represents a node with kind `Bang`, having the following structure:
   *
   * ```ebnf
   * BANG = "!";
   * ```
   */
  Bang = "Bang",
  /**
   * Represents a node with kind `CloseBracket`, having the following structure:
   *
   * ```ebnf
   * CLOSE_BRACKET = "]";
   * ```
   */
  CloseBracket = "CloseBracket",
  /**
   * Represents a node with kind `DelimitedIdentifier`, having the following structure:
   *
   * ```ebnf
   * DELIMITED_IDENTIFIER = «DELIMITED_IDENTIFIER_START» «DELIMITED_IDENTIFIER_PART»*;
   * ```
   */
  DelimitedIdentifier = "DelimitedIdentifier",
  /**
   * Represents a node with kind `EndOfLine`, having the following structure:
   *
   * ```ebnf
   * END_OF_LINE = "\r"? "\n";
   * ```
   */
  EndOfLine = "EndOfLine",
  /**
   * Represents a node with kind `Identifier`, having the following structure:
   *
   * ```ebnf
   * IDENTIFIER = «RAW_IDENTIFIER»;
   * ```
   */
  Identifier = "Identifier",
  /**
   * Represents a node with kind `MultiLineComment`, having the following structure:
   *
   * ```ebnf
   * MULTI_LINE_COMMENT = "/*" (?!"*") (!"*" | ("*" (?!"/")))* "*\/";
   * ```
   */
  MultiLineComment = "MultiLineComment",
  /**
   * Represents a node with kind `OpenBracket`, having the following structure:
   *
   * ```ebnf
   * OPEN_BRACKET = "[";
   * ```
   */
  OpenBracket = "OpenBracket",
  /**
   * Represents a node with kind `Period`, having the following structure:
   *
   * ```ebnf
   * PERIOD = ".";
   * ```
   */
  Period = "Period",
  /**
   * Represents a node with kind `Plus`, having the following structure:
   *
   * ```ebnf
   * PLUS = "+";
   * ```
   */
  Plus = "Plus",
  /**
   * Represents a node with kind `Semicolon`, having the following structure:
   *
   * ```ebnf
   * SEMICOLON = ";";
   * ```
   */
  Semicolon = "Semicolon",
  /**
   * Represents a node with kind `SingleLineComment`, having the following structure:
   *
   * ```ebnf
   * SINGLE_LINE_COMMENT = "//" (?!"/") (!("\r" | "\n"))*;
   * ```
   */
  SingleLineComment = "SingleLineComment",
  /**
   * Represents a node with kind `StringLiteral`, having the following structure:
   *
   * ```ebnf
   * STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | !('"' | "\\" | "\r" | "\n"))* '"';
   * ```
   */
  StringLiteral = "StringLiteral",
  /**
   * Represents a node with kind `TreeKeyword`, having the following structure:
   *
   * ```ebnf
   * TREE_KEYWORD = "tree";
   * ```
   */
  TreeKeyword = "TreeKeyword",
  /**
   * Represents a node with kind `Whitespace`, having the following structure:
   *
   * ```ebnf
   * WHITESPACE = (" " | "\t")+;
   * ```
   */
  Whitespace = "Whitespace",
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
   * Represents a child node with the label `close_bracket`.
   */
  CloseBracket = "CloseBracket",
  /**
   * Represents a child node with the label `keyword`.
   */
  Keyword = "Keyword",
  /**
   * Represents a child node with the label `member`.
   */
  Member = "Member",
  /**
   * Represents a child node with the label `members`.
   */
  Members = "Members",
  /**
   * Represents a child node with the label `name`.
   */
  Name = "Name",
  /**
   * Represents a child node with the label `node`.
   */
  Node = "Node",
  /**
   * Represents a child node with the label `open_bracket`.
   */
  OpenBracket = "OpenBracket",
  /**
   * Represents a child node with the label `operator`.
   */
  Operator = "Operator",
  /**
   * Represents a child node with the label `period`.
   */
  Period = "Period",
  /**
   * Represents a child node with the label `semicolon`.
   */
  Semicolon = "Semicolon",
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
 * Represents a connection between nodes in the syntax tree.
 */
export interface Edge {
  /**
   * Describes the relationship between this node and its parent.
   */
  label: EdgeLabel;
  /**
   * The target node of this edge.
   */
  node: Node;
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
  children(): Edge[];
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

/**
 * Iterator over all the remaining nodes in the current tree, moving in pre-order traversal, until the tree is completed.
 */
export class CursorIterator {
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
 * Represents a non-terminal node in the syntax tree.
 * These nodes can have child nodes and represent language constructs.
 */
export class NonterminalNode {
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
  children(): Edge[];
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
   * Parses a query string into a query object.
   * Throws an error if the query syntax is invalid.
   */
  static create(text: string): Query;
}

/**
 * Iterator over query matches in the syntax tree.
 */
export class QueryMatchIterator {
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
  children(): Edge[];
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
