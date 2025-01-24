import { Parser } from "@slang-private/testlang-npm-package/parser";
import {
  Cursor,
  assertIsNonterminalNode,
  NonterminalKind,
  CursorIterator,
  AncestorsIterator,
  Edge,
} from "@slang-private/testlang-npm-package/cst";

describe("iterators", () => {
  const source = "tree [A [B C] D];";
  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseFileContents(source);
  const cursor: Cursor = parseOutput.createTreeCursor();

  assertIsNonterminalNode(cursor.node, NonterminalKind.SourceUnit);

  // Go deep to the second TreeNode (`[B C]`):
  expect(cursor.goToNextNonterminalWithKind(NonterminalKind.TreeNode)).toBe(true);
  expect(cursor.goToNextNonterminalWithKind(NonterminalKind.TreeNode)).toBe(true);

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
    "[",
    "B C",
    "]",
  ]);
}

function verifyDescendants(iterator: CursorIterator) {
  const values = [];

  for (const edge of iterator) {
    values.push(edge.node.unparse());
  }

  expect(values).toEqual([
    // First child (trivia):
    " ",
    // Second child (another node):
    "[",
    "B C",
    "B",
    "B",
    " C",
    " ",
    "C",
    "]",
  ]);
}

function verifyRemainingNodes(iterator: CursorIterator) {
  const values = [];

  for (const edge of iterator) {
    values.push(edge.node.unparse());
  }

  expect(values).toEqual([
    // The current node is the first one:
    " [B C]",
    // then its descendants:
    " ",
    "[",
    "B C",
    "B",
    "B",
    " C",
    " ",
    "C",
    "]",
    // then its following siblings:
    " D",
    " ",
    "D",
    "]",
    // then the following siblings of the ancestors, all the way to the root:
    ";",
  ]);
}

function verifyAncestors(iterator: AncestorsIterator) {
  const kinds = [];

  for (const node of iterator) {
    kinds.push(node.kind);
  }

  expect(kinds).toEqual([
    // from the direct parent, up all the way to the root:
    NonterminalKind.TreeNodeChild,
    NonterminalKind.TreeNodeChildren,
    NonterminalKind.TreeNode,
    NonterminalKind.Tree,
    NonterminalKind.SourceUnitMember,
    NonterminalKind.SourceUnitMembers,
    NonterminalKind.SourceUnit,
  ]);
}
