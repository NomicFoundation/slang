import { Language } from "@slang-private/slang-testlang/language";
import { FieldName, RuleKind, TokenKind } from "@slang-private/slang-testlang/kinds";
import { Cursor } from "@slang-private/slang-testlang/cursor";
import { expectRule, expectToken } from "../utils/cst-helpers";
import { NodeType } from "@slang-private/slang-testlang/cst";

test("use cursor", () => {
  const source = "tree [A [B C] D];";
  const language = new Language("1.0.0");

  const parseOutput = language.parse(RuleKind.SourceUnit, source);
  const cursor: Cursor = parseOutput.createTreeCursor();

  expectRule(cursor.node(), RuleKind.SourceUnit);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.SourceUnitMembers);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.SourceUnitMember);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.Tree);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.TreeKeyword, "tree");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TreeNode);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.OpenBracket, "[");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TreeNodeChildren);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.DelimitedIdentifier, "A");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TreeNode);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.OpenBracket, "[");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TreeNodeChildren);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.DelimitedIdentifier, "B");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.DelimitedIdentifier, "C");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.CloseBracket, "]");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TreeNodeChild);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.DelimitedIdentifier, "D");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.CloseBracket, "]");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Semicolon, ";");
  expect(cursor.goToNext()).toBe(false);
});

test("access the node using its name", () => {
  const source = "tree [A [B C] D];";
  const language = new Language("1.0.0");
  const parseTree = language.parse(RuleKind.SourceUnit, source);

  const cursor = parseTree.createTreeCursor();
  let names: string[] = [];

  while (cursor.goToNextRuleWithKind(RuleKind.TreeNode)) {
    const innerCursor = cursor.spawn();
    while (innerCursor.goToNext()) {
      const node = innerCursor.node();
      const nodeName = innerCursor.nodeName;

      if (node.type == NodeType.Token && (nodeName == FieldName.OpenBracket || nodeName == FieldName.CloseBracket)) {
        names.push(node.text);
      }
    }
  }

  expect(names).toEqual(["[", "[", "]", "]", "[", "]"]);
});
