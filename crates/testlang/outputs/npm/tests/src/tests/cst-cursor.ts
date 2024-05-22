import { Language } from "@slang-private/slang-testlang/language";
import { EdgeLabel, NonTerminalKind, TerminalKind } from "@slang-private/slang-testlang/kinds";
import { Cursor } from "@slang-private/slang-testlang/cursor";
import { expectNonTerminal, expectTerminal } from "../utils/cst-helpers";
import { NodeType } from "@slang-private/slang-testlang/cst";

test("use cursor", () => {
  const source = "tree [A [B C] D];";
  const language = new Language("1.0.0");

  const parseOutput = language.parse(NonTerminalKind.SourceUnit, source);
  const cursor: Cursor = parseOutput.createTreeCursor();

  expectNonTerminal(cursor.node(), NonTerminalKind.SourceUnit);
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.SourceUnitMembers);
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.SourceUnitMember);
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.Tree);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.TreeKeyword, "tree");
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.TreeNode);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.OpenBracket, "[");
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.TreeNodeChildren);
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.DelimitedIdentifier, "A");
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.TreeNode);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.OpenBracket, "[");
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.TreeNodeChildren);
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.DelimitedIdentifier, "B");
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.DelimitedIdentifier, "C");
  expect(cursor.goToNext()).toBe(true);

  expectTerminal(cursor.node(), TerminalKind.CloseBracket, "]");
  expect(cursor.goToNext()).toBe(true);

  expectNonTerminal(cursor.node(), NonTerminalKind.TreeNodeChild);
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
  const parseTree = language.parse(NonTerminalKind.SourceUnit, source);

  const cursor = parseTree.createTreeCursor();
  let names: string[] = [];

  while (cursor.goToNextNonterminalWithKind(NonTerminalKind.TreeNode)) {
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
