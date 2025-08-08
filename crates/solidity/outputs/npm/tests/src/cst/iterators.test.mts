import { Parser } from "@nomicfoundation/slang/parser";
import {
  Cursor,
  assertNonterminalNode,
  NonterminalKind,
  CursorIterator,
  AncestorsIterator,
  Edge,
} from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

describe("iterators", () => {
  const source = "contract A { function a(){} function b(){} function c(){} }";
  const parser = Parser.create(LanguageFacts.latestVersion());

  const parseOutput = parser.parseFileContents(source);
  const cursor: Cursor = parseOutput.createTreeCursor();

  assertNonterminalNode(cursor.node, NonterminalKind.SourceUnit);

  // Go deep to the second function (`b`):
  expect(cursor.goToNextNonterminalWithKind(NonterminalKind.FunctionDefinition)).toBe(true);
  expect(cursor.goToNextNonterminalWithKind(NonterminalKind.FunctionDefinition)).toBe(true);

  test("cursor.children()", () => {
    verifyChildren(cursor.children());
  });

  test("node.children()", () => {
    verifyChildren(cursor.node.children());
  });

  test("cursor.descendants()", () => {
    verifyDescendants(cursor.descendants());
  });

  test("node.descendants()", () => {
    verifyDescendants(cursor.node.descendants());
  });

  test("cursor.remainingNodes()", () => {
    verifyRemainingNodes(cursor.remainingNodes());
  });

  test("cursor.ancestors()", () => {
    verifyAncestors(cursor.ancestors());
  });
});

function verifyChildren(edges: Edge[]) {
  const values = [];

  for (const edge of edges) {
    values.push(edge.node.unparse());
  }

  expect(values).toEqual([
    // Only direct children:
    " ",
    "function",
    " b",
    "()",
    "",
    "{}",
  ]);
}

function verifyDescendants(iterator: CursorIterator) {
  const values = [];

  for (const edge of iterator) {
    values.push(edge.node.unparse());
  }

  expect(values).toEqual([" ", "function", " b", " ", "b", "()", "(", "", ")", "", "{}", "{}", "{", "", "}"]);
}

function verifyRemainingNodes(iterator: CursorIterator) {
  const values = [];

  for (const edge of iterator) {
    values.push(edge.node.unparse());
  }

  expect(values).toEqual([
    // The current node is the first one:
    " function b(){}",
    // then its descendants:
    " ",
    "function",
    " b",
    " ",
    "b",
    "()",
    "(",
    "",
    ")",
    "",
    "{}",
    "{}",
    "{",
    "",
    "}",
    // then its following siblings:
    " function c(){}",
    " function c(){}",
    " ",
    "function",
    " c",
    " ",
    "c",
    "()",
    "(",
    "",
    ")",
    "",
    "{}",
    "{}",
    "{",
    "",
    "}",
    " ",
    // then the following siblings of the ancestors, all the way to the root:
    "}",
  ]);
}

function verifyAncestors(iterator: AncestorsIterator) {
  const kinds = [];

  for (const node of iterator) {
    kinds.push(node.kind);
  }

  expect(kinds).toEqual([
    // from the direct parent, up all the way to the root:
    NonterminalKind.ContractMember,
    NonterminalKind.ContractMembers,
    NonterminalKind.ContractDefinition,
    NonterminalKind.SourceUnitMember,
    NonterminalKind.SourceUnitMembers,
    NonterminalKind.SourceUnit,
  ]);
}
