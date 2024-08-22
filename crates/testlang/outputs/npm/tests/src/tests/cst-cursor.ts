import { Language } from "@slang-private/slang-testlang/language";
import { expectNonterminal, expectTerminal } from "../utils/cst-helpers";
import { Cursor, EdgeLabel, NodeType, NonterminalKind, TerminalKind } from "@slang-private/slang-testlang/cst";

test("use cursor", () => {
  const source = "tree [A [B C] D];";
  const language = new Language("1.0.0");

  const parseOutput = language.parse(NonterminalKind.SourceUnit, source);
  const cursor: Cursor = parseOutput.createTreeCursor();

  expectNonterminal(cursor.node(), NonterminalKind.SourceUnit);
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.SourceUnitMembers);
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.SourceUnitMember);
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.Tree);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.TreeKeyword, "tree");
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.TreeNode);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.OpenBracket, "[");
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.TreeNodeChildren);
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.DelimitedIdentifier, "A");
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.TreeNode);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.OpenBracket, "[");
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.TreeNodeChildren);
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.DelimitedIdentifier, "B");
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.DelimitedIdentifier, "C");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.CloseBracket, "]");
  expect(cursor.goToNext()).toBe(true);

  expectNonterminal(cursor.node(), NonterminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.DelimitedIdentifier, "D");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.CloseBracket, "]");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.Semicolon, ";");
  expect(cursor.goToNext()).toBe(false);
});

test("access the node using its name", () => {
  const source = "tree [A [B C] D];";
  const language = new Language("1.0.0");
  const parseTree = language.parse(NonterminalKind.SourceUnit, source);

  const cursor = parseTree.createTreeCursor();
  let names: string[] = [];

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.TreeNode)) {
    const innerCursor = cursor.spawn();
    while (innerCursor.goToNext()) {
      const node = innerCursor.node();
      const label = innerCursor.label;

      if (node.type == NodeType.Terminal && (label == EdgeLabel.OpenBracket || label == EdgeLabel.CloseBracket)) {
        names.push(node.text);
      }
    }
  }

  expect(names).toEqual(["[", "[", "]", "]", "[", "]"]);
});
