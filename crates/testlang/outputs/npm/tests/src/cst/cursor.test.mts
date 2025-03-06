import { Parser } from "@slang-private/testlang-npm-package/parser";
import {
  Cursor,
  EdgeLabel,
  NodeType,
  assertNonterminalNode,
  assertTerminalNode,
  NonterminalKind,
  TerminalKind,
} from "@slang-private/testlang-npm-package/cst";

test("use cursor goToNext()", () => {
  const source = "tree [A [B C] D];";
  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseFileContents(source);
  const cursor: Cursor = parseOutput.createTreeCursor();

  assertNonterminalNode(cursor.node, NonterminalKind.SourceUnit);
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.SourceUnitMembers);
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.SourceUnitMember);
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.Tree);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.TreeKeyword, "tree");
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.TreeNode);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.OpenBracket, "[");
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.TreeNodeChildren);
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.DelimitedIdentifier, "A");
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.TreeNode);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.OpenBracket, "[");
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.TreeNodeChildren);
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.DelimitedIdentifier, "B");
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.DelimitedIdentifier, "C");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.CloseBracket, "]");
  expect(cursor.goToNext()).toBe(true);

  assertNonterminalNode(cursor.node, NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.DelimitedIdentifier, "D");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.CloseBracket, "]");
  expect(cursor.goToNext()).toBe(true);

  assertTerminalNode(cursor.node, TerminalKind.Semicolon, ";");
  expect(cursor.goToNext()).toBe(false);
});

test("access the node using its name", () => {
  const source = "tree [A [B C] D];";
  const parser = Parser.create("1.0.0");
  const parseOutput = parser.parseFileContents(source);

  const cursor = parseOutput.createTreeCursor();
  let names: string[] = [];

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.TreeNode)) {
    const innerCursor = cursor.spawn();
    while (innerCursor.goToNext()) {
      const node = innerCursor.node;
      const label = innerCursor.label;

      if (node.type == NodeType.TerminalNode && (label == EdgeLabel.OpenBracket || label == EdgeLabel.CloseBracket)) {
        names.push(node.unparse());
      }
    }
  }

  expect(names).toEqual(["[", "[", "]", "]", "[", "]"]);
});
