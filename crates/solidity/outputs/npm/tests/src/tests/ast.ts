import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import {
  AdditiveExpression,
  ContractDefinition,
  Expression,
  IdentifierPath,
  SourceUnitMembers,
} from "@nomicfoundation/slang/ast";
import { RuleNode } from "@nomicfoundation/slang/cst";
import { expectToken } from "../utils/cst-helpers";

test("create and use sequence types", () => {
  const source = `
    contract Foo {
      uint x;
      uint y;
    }
  `.trim();

  const language = new Language("0.8.1");

  const parseOutput = language.parse(RuleKind.ContractDefinition, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const contract = new ContractDefinition(parseOutput.tree() as RuleNode);
  expect(contract.abstractKeyword()).toBeNull();
  expect(contract.name().text).toEqual("Foo");
  expect(contract.members()?.items()).toHaveLength(2);
});

test("create and use choice types", () => {
  const source = `x + y`;

  const language = new Language("0.8.1");

  const parseOutput = language.parse(RuleKind.Expression, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const rootExpression = new Expression(parseOutput.tree() as RuleNode);
  expect(rootExpression.variant()).toBeInstanceOf(AdditiveExpression);

  const additiveExpression = rootExpression.variant() as AdditiveExpression;
  expectToken(additiveExpression.leftOperand().variant(), TokenKind.Identifier, "x");
  expectToken(additiveExpression.operator(), TokenKind.Plus, "+");
  expectToken(additiveExpression.rightOperand().variant(), TokenKind.Identifier, "y");
});

test("create and use repeated types", () => {
  const source = `
    contract X {}
    contract Y {}
    contract Z {}
  `.trim();

  const language = new Language("0.8.1");

  const parseOutput = language.parse(RuleKind.SourceUnitMembers, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const members = new SourceUnitMembers(parseOutput.tree() as RuleNode);

  const names = members.items().map((item) => {
    expect(item.variant()).toBeInstanceOf(ContractDefinition);

    return (item.variant() as ContractDefinition).name().text;
  });

  expect(names).toStrictEqual(["X", "Y", "Z"]);
});

test("create and use separated types", () => {
  const source = `Foo.Bar.Baz`;

  const language = new Language("0.8.1");

  const parseOutput = language.parse(RuleKind.IdentifierPath, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const path = new IdentifierPath(parseOutput.tree() as RuleNode);

  const identifiers = path?.items().map((identifier) => identifier.text);
  expect(identifiers).toStrictEqual(["Foo", "Bar", "Baz"]);

  const periods = path?.separators().map((period) => period.text);
  expect(periods).toStrictEqual([".", "."]);
});
