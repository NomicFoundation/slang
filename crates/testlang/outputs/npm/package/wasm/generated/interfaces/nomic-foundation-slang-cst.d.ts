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
export declare enum TerminalKind {
  /**
   * Built-in:
   */
  Unrecognized = "Unrecognized",
  Missing = "Missing",
  /**
   * Generated:
   */
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
export declare enum EdgeLabel {
  /**
   * Built-in:
   */
  Item = "Item",
  Variant = "Variant",
  Separator = "Separator",
  Operand = "Operand",
  LeftOperand = "LeftOperand",
  RightOperand = "RightOperand",
  LeadingTrivia = "LeadingTrivia",
  TrailingTrivia = "TrailingTrivia",
  /**
   * Generated:
   */
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
export type Node = NonterminalNode | TerminalNode;
export enum NodeVariant {
  NonterminalNode = "NonterminalNode",
  TerminalNode = "TerminalNode",
}
export interface Edge {
  label?: EdgeLabel;
  node: Node;
}
export interface QueryError {
  message: string;
  line: number;
  column: number;
}
export interface QueryMatch {
  queryNumber: number;
  captures: { [key: string]: Cursor[] };
}
export interface TextIndex {
  utf8: number;
  utf16: number;
  line: number;
  column: number;
}
export interface TextRange {
  start: TextIndex;
  end: TextIndex;
}

export class Cursor {
  reset(): void;
  complete(): void;
  isCompleted(): boolean;
  clone(): Cursor;
  spawn(): Cursor;
  get node(): Node;
  get label(): EdgeLabel | undefined;
  get textOffset(): TextIndex;
  get textRange(): TextRange;
  get depth(): number;
  get ancestors(): NonterminalNode[];
  goToNext(): boolean;
  goToNextNonDescendent(): boolean;
  goToPrevious(): boolean;
  goToParent(): boolean;
  goToFirstChild(): boolean;
  goToLastChild(): boolean;
  goToNthChild(childNumber: number): boolean;
  goToNextSibling(): boolean;
  goToPreviousSibling(): boolean;
  goToNextTerminal(): boolean;
  goToNextTerminalWithKind(kind: TerminalKind): boolean;
  goToNextTerminalWithKinds(kinds: TerminalKind[]): boolean;
  goToNextNonterminal(): boolean;
  goToNextNonterminalWithKind(kind: NonterminalKind): boolean;
  goToNextNonterminalWithKinds(kinds: NonterminalKind[]): boolean;
  query(queries: Query[]): QueryMatchIterator;
}

export class NonterminalNode {
  readonly nodeVariant = NodeVariant.NonterminalNode;

  asNonterminalNode(): this;
  isNonterminalNode(): this is NonterminalNode;

  asTerminalNode(): undefined;
  isTerminalNode(): false;

  get id(): number;
  get kind(): NonterminalKind;
  get textLength(): TextIndex;
  get children(): Edge[];
  unparse(): string;
  toJson(): string;
  createCursor(textOffset: TextIndex): Cursor;
}

export class Query {
  static parse(text: string): Query;
}

export class QueryMatchIterator {
  [Symbol.iterator](): Iterator<QueryMatch>;
  next(): QueryMatch | undefined;
}

export class TerminalKindExtensions {
  static isTrivia(kind: TerminalKind): boolean;
  static isValid(kind: TerminalKind): boolean;
}

export class TerminalNode {
  readonly nodeVariant = NodeVariant.TerminalNode;

  asTerminalNode(): this;
  isTerminalNode(): this is TerminalNode;

  asNonterminalNode(): undefined;
  isNonterminalNode(): false;

  get id(): number;
  get kind(): TerminalKind;
  get textLength(): TextIndex;
  get children(): Edge[];
  unparse(): string;
  toJson(): string;
}
