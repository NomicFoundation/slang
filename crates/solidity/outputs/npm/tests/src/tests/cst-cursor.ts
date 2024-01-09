import { Language } from "@nomicfoundation/slang/language";
import { FieldName, RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import { Cursor } from "@nomicfoundation/slang/cursor";
import { expectRule, expectToken } from "../utils/cst-helpers";
import { NodeType } from "@nomicfoundation/slang/cst";

test("use cursor", () => {
  const source = "int256 constant z = 1 + 2;";
  const language = new Language("0.8.1");

  const parseOutput = language.parse(RuleKind.SourceUnit, source);
  const cursor: Cursor = parseOutput.createTreeCursor();

  expectRule(cursor.node(), RuleKind.SourceUnit);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.SourceUnitMembers);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.SourceUnitMember);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.ConstantDefinition);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TypeName);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.ElementaryType);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.IntKeyword, "int256");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.ConstantKeyword, "constant");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Identifier, "z");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Equal, "=");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.Expression);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.AdditiveExpression);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.Expression);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.DecimalNumberExpression);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.DecimalLiteral, "1");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Plus, "+");
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.Expression);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.DecimalNumberExpression);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.LeadingTrivia);
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Whitespace, " ");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.DecimalLiteral, "2");
  expect(cursor.goToNext()).toBe(true);

  expectToken(cursor.node(), TokenKind.Semicolon, ";");
  expect(cursor.goToNext()).toBe(false);
});

test("access the node using its name", () => {
  const source = "contract Foo {} contract Bar {} contract Baz {}";
  const language = new Language("0.8.0");
  const parseTree = language.parse(RuleKind.SourceUnit, source);

  const cursor = parseTree.createTreeCursor();
  let names: string[] = [];

  while (cursor.goToNextRuleWithKind(RuleKind.ContractDefinition)) {
    const innerCursor = cursor.spawn();
    while (innerCursor.goToNext()) {
      const node = innerCursor.node();
      const nodeName = innerCursor.nodeName;

      if (node.type == NodeType.Token && nodeName == FieldName.Name) {
        names.push(node.text);
      }
    }
  }

  expect(names).toEqual(["Foo", "Bar", "Baz"]);
});
