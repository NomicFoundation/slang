import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import { Cursor } from "@nomicfoundation/slang/cursor";
import { expectRule, expectToken } from "../utils/cst-helpers";

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

test("cursor navigation", () => {
  const data = "contract Foo {} contract Bar {} contract Baz {}";

  const language = new Language("0.8.0");
  const parseTree = language.parse(RuleKind.SourceUnit, data);

  let contractNames = [];
  let cursor = parseTree.createTreeCursor();

  while (cursor.goToNextRuleWithKinds([RuleKind.ContractDefinition])) {
    // You have to make sure you return the cursor to original position
    cursor.goToFirstChild();
    cursor.goToNextTokenWithKinds([TokenKind.Identifier]);

    // The currently pointed-to node is the name of the contract
    let tokenNode = cursor.node();
    if (tokenNode.kind !== TokenKind.Identifier) {
      throw new Error("Expected identifier");
    }
    contractNames.push(tokenNode.text);

    cursor.goToParent();
  }

  expect(contractNames).toEqual(["Foo", "Bar", "Baz"]);
});
