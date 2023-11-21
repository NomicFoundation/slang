import { Language } from "@nomicfoundation/slang/language";
import { ProductionKind, RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import { Cursor } from "@nomicfoundation/slang/cursor";
import { expectRule, expectToken } from "../utils/cst-helpers";

test("use cursor", () => {
  const source = "int256 constant z = 1 + 2;";
  const language = new Language("0.8.1");

  const parseOutput = language.parse(ProductionKind.SourceUnit, source);
  const cursor: Cursor = parseOutput.createTreeCursor();

  expectRule(cursor.node(), RuleKind.SourceUnit);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.SourceUnitMembers);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.ConstantDefinition);
  expect(cursor.goToNext()).toBe(true);

  expectRule(cursor.node(), RuleKind.TypeName);
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

  expectRule(cursor.node(), RuleKind.BinaryExpression);
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
