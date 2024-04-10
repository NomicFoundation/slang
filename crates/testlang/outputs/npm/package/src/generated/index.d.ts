// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// Slang License: https://github.com/NomicFoundation/slang/blob/main/LICENSE
// NAPI-RS License: https://github.com/napi-rs/napi-rs/blob/main/LICENSE

/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export namespace kinds {
  export enum RuleKind {
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
  export enum NodeLabel {
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
  export enum TokenKind {
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
    parse(kind: kinds.RuleKind, input: string): parse_output.ParseOutput;
  }
}
export namespace ast_internal {
  export function selectSequence(node: cst.RuleNode): Array<cst.Node | null>;
  export function selectChoice(node: cst.RuleNode): cst.Node;
  export function selectRepeated(node: cst.RuleNode): Array<cst.Node>;
  export function selectSeparated(node: cst.RuleNode): [Array<cst.Node>, Array<cst.Node>];
}
export namespace cst {
  export enum NodeType {
    Rule = 0,
    Token = 1,
  }
  export class RuleNode {
    get type(): NodeType.Rule;
    get kind(): kinds.RuleKind;
    get textLength(): text_index.TextIndex;
    children(): Array<cst.Node>;
    createCursor(textOffset: text_index.TextIndex): cursor.Cursor;
    /** Serialize the token node to JSON. */
    toJSON(): string;
    unparse(): string;
  }
  export class TokenNode {
    get type(): NodeType.Token;
    get kind(): kinds.TokenKind;
    get textLength(): text_index.TextIndex;
    get text(): string;
    /**
     * Serialize the token node to JSON.
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
    get label(): kinds.NodeLabel;
    get textOffset(): text_index.TextIndex;
    get textRange(): text_index.TextRange;
    get depth(): number;
    ancestors(): Array<cst.RuleNode>;
    goToNext(): boolean;
    goToNextNonDescendent(): boolean;
    goToPrevious(): boolean;
    goToParent(): boolean;
    goToFirstChild(): boolean;
    goToLastChild(): boolean;
    goToNthChild(childNumber: number): boolean;
    goToNextSibling(): boolean;
    goToPreviousSibling(): boolean;
    goToNextToken(): boolean;
    goToNextTokenWithKind(kind: kinds.TokenKind): boolean;
    goToNextTokenWithKinds(kinds: Array<kinds.TokenKind>): boolean;
    goToNextRule(): boolean;
    goToNextRuleWithKind(kind: kinds.RuleKind): boolean;
    goToNextRuleWithKinds(kinds: Array<kinds.RuleKind>): boolean;
    query(queries: Array<query.Query>): query.QueryResultIterator;
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
  export interface QueryResult {
    queryNumber: number;
    bindings: { [key: string]: cursor.Cursor[] };
  }
  export class Query {
    static parse(text: string): Query;
  }
  export class QueryResultIterator {
    next(): QueryResult | null;
  }
}
export namespace text_index {
  export interface TextIndex {
    utf8: number;
    utf16: number;
    char: number;
  }
  export interface TextRange {
    start: TextIndex;
    end: TextIndex;
  }
}

export namespace cst {
  export type Node = RuleNode | TokenNode;
}
