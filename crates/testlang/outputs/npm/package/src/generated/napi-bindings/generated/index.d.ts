// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// Slang License: https://github.com/NomicFoundation/slang/blob/main/LICENSE
// NAPI-RS License: https://github.com/napi-rs/napi-rs/blob/main/LICENSE

/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export namespace kinds {
  export enum NonTerminalKind {
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
  export enum EdgeLabel {
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
  export enum TerminalKind {
    SKIPPED = "SKIPPED",
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
}
export namespace language {
  export class Language {
    constructor(version: string);
    get version(): string;
    static supportedVersions(): Array<string>;
    parse(kind: kinds.NonTerminalKind, input: string): parse_output.ParseOutput;
  }
}
export namespace cst {
  export enum NodeType {
    NonTerminal = "NonTerminal",
    Terminal = "Terminal",
  }
  export class NonTerminalNode {
    get type(): NodeType.NonTerminal;
    get kind(): kinds.NonTerminalKind;
    get textLength(): text_index.TextIndex;
    children(): Array<cst.Node>;
    createCursor(textOffset: text_index.TextIndex): cursor.Cursor;
    /**
     * Serialize the node to JSON.
     *
     * This method is intended for debugging purposes and may not be stable.
     */
    toJSON(): string;
    unparse(): string;
  }
  export class TerminalNode {
    get type(): NodeType.Terminal;
    get kind(): kinds.TerminalKind;
    get textLength(): text_index.TextIndex;
    get text(): string;
    /**
     * Serialize the node to JSON.
     *
     * This method is intended for debugging purposes and may not be stable.
     */
    toJSON(): string;
    createCursor(textOffset: text_index.TextIndex): cursor.Cursor;
  }
}
export namespace cursor {
  export class Cursor {
    reset(): void;
    complete(): void;
    clone(): Cursor;
    spawn(): Cursor;
    get isCompleted(): boolean;
    node(): cst.Node;
    get label(): kinds.EdgeLabel;
    get textOffset(): text_index.TextIndex;
    get textRange(): text_index.TextRange;
    get depth(): number;
    ancestors(): Array<cst.NonTerminalNode>;
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
    goToNextTerminalWithKind(kind: kinds.TerminalKind): boolean;
    goToNextTerminalWithKinds(kinds: Array<kinds.TerminalKind>): boolean;
    goToNextNonterminal(): boolean;
    goToNextNonterminalWithKind(kind: kinds.NonTerminalKind): boolean;
    goToNextNonterminalWithKinds(kinds: Array<kinds.NonTerminalKind>): boolean;
    query(queries: Array<query.Query>): query.QueryMatchIterator;
  }
}
export namespace parse_error {
  export class ParseError {
    get textRange(): text_index.TextRange;
    toErrorReport(sourceId: string, source: string, withColor: boolean): string;
  }
}
export namespace parse_output {
  export class ParseOutput {
    tree(): cst.Node;
    errors(): Array<parse_error.ParseError>;
    get isValid(): boolean;
    /** Creates a cursor that starts at the root of the parse tree. */
    createTreeCursor(): cursor.Cursor;
  }
}
export namespace query {
  export interface QueryMatch {
    queryNumber: number;
    bindings: { [key: string]: cursor.Cursor[] };
  }
  export class Query {
    static parse(text: string): Query;
  }
  export class QueryMatchIterator {
    next(): QueryMatch | null;
  }
}
export namespace text_index {
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
}
export namespace ast_internal {
  export function selectSequence(node: cst.NonTerminalNode): Array<cst.Node | null>;
  export function selectChoice(node: cst.NonTerminalNode): cst.Node;
  export function selectRepeated(node: cst.NonTerminalNode): Array<cst.Node>;
  export function selectSeparated(node: cst.NonTerminalNode): [Array<cst.Node>, Array<cst.Node>];
}

export namespace cst {
  export type Node = TerminalNode | NonTerminalNode;
}
