import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import {
  AdditiveExpression,
  ContractDefinition,
  Expression,
  IdentifierPath,
  SourceUnit,
  SourceUnitMembers,
} from "@nomicfoundation/slang/ast";
import { expectRule, expectToken } from "../utils/cst-helpers";

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

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.ContractDefinition);

  const contract = new ContractDefinition(cst);
  expectRule(contract.cst, RuleKind.ContractDefinition);
  expect(contract.abstractKeyword).toBeNull();
  expectToken(contract.name, TokenKind.Identifier, "Foo");
  expect(contract.members!.items).toHaveLength(2);
});

test("create and use choice types", () => {
  const source = `x + y`;

  const language = new Language("0.8.1");

  const parseOutput = language.parse(RuleKind.Expression, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.Expression);

  const rootExpression = new Expression(cst);
  expectRule(rootExpression.cst, RuleKind.Expression);
  expect(rootExpression.variant).toBeInstanceOf(AdditiveExpression);

  const additiveExpression = rootExpression.variant as AdditiveExpression;
  expectRule(additiveExpression.cst, RuleKind.AdditiveExpression);
  expectToken(additiveExpression.leftOperand.variant, TokenKind.Identifier, "x");
  expectToken(additiveExpression.operator, TokenKind.Plus, "+");
  expectToken(additiveExpression.rightOperand.variant, TokenKind.Identifier, "y");
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

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.SourceUnitMembers);

  const members = new SourceUnitMembers(cst);
  expectRule(members.cst, RuleKind.SourceUnitMembers);

  const names = members.items.map((item) => {
    expect(item.variant).toBeInstanceOf(ContractDefinition);

    return (item.variant as ContractDefinition).name.text;
  });

  expect(names).toStrictEqual(["X", "Y", "Z"]);
});

test("create and use separated types", () => {
  const source = `Foo.Bar.Baz`;

  const language = new Language("0.8.1");

  const parseOutput = language.parse(RuleKind.IdentifierPath, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.IdentifierPath);

  const path = new IdentifierPath(cst);
  expectRule(path.cst, RuleKind.IdentifierPath);

  const identifiers = path.items.map((identifier) => identifier.text);
  expect(identifiers).toStrictEqual(["Foo", "Bar", "Baz"]);

  path.separators.forEach((separator) => {
    expectToken(separator, TokenKind.Period, ".");
  });
});

test("throws an exception on initializing the wrong type", () => {
  const source = `contract Foo {}`;

  const language = new Language("0.8.1");

  const parseOutput = language.parse(RuleKind.ContractDefinition, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.ContractDefinition);

  expect(() => new SourceUnit(cst)).toThrowError(
    "SourceUnit can only be initialized with a CST node of the same kind.",
  );
});

test("throws an exception on on using an incorrect/incomplete CST node", () => {
  const source = `contract`;

  const language = new Language("0.8.1");

  const parseOutput = language.parse(RuleKind.ContractDefinition, source);
  expect(parseOutput.errors()).toHaveLength(1);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.ContractDefinition);
  expect(cst.children()).toHaveLength(2);

  const [contractKeyword, skippedToken] = cst.children();
  expectToken(contractKeyword, TokenKind.ContractKeyword, "contract");
  expectToken(skippedToken, TokenKind.SKIPPED, "");

  // Creating the contract should succeed, as the fields are lazily intialized.
  const contract = new ContractDefinition(cst);
  expectRule(contract.cst, RuleKind.ContractDefinition);

  expect(() => contract.name).toThrowError(
    "Unexpected SKIPPED token at index '1'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.",
  );
});
