import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import {
  AdditiveExpression,
  Expression,
  MemberAccessExpression,
  PrefixExpression,
  SourceUnit,
  ContractDefinition,
  FunctionDefinition,
  Block,
  StateVariableDefinition,
  IdentifierPath,
} from "@nomicfoundation/slang/ast";
import { assertNonterminalNode, assertTerminalNode, NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("create and use sequence types", () => {
  const source = `function a(int p1, int p2, int p3){}`;

  const parser = createParser();

  const parseOutput = parser.parseNonterminal(NonterminalKind.FunctionDefinition, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.FunctionDefinition);

  const func = new FunctionDefinition(cst);
  expect(func.functionKeyword.unparse()).toEqual("function");
  expect(func.name.variant.kind).toBe(TerminalKind.Identifier);
  expect(func.name.variant.unparse()).toEqual("a");
  expect(func.attributes.items).toHaveLength(0);
  expect(func.body.variant).toBeInstanceOf(Block);
  expect(func.returns).toBeUndefined();

  // validate a variant type
  const body = func.body.variant as Block;
  expect(body.openBrace.unparse()).toEqual("{");
  expect(body.closeBrace.unparse()).toEqual("}");
  expect(body.statements.items).toHaveLength(0);

  // validate a repeated type
  const parameters = func.parameters;
  expect(body.openBrace.unparse()).toEqual("{");
  expect(body.closeBrace.unparse()).toEqual("}");
  expect(parameters.parameters.items).toHaveLength(3);
  expect(parameters.parameters.separators).toHaveLength(2);
});

test("create and use choice types", () => {
  const source = `contract C { uint a; function f() public {} }`;

  const parser = createParser();

  const parseOutput = parser.parseNonterminal(NonterminalKind.ContractDefinition, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.ContractDefinition);

  const contract = new ContractDefinition(cst);
  expect(contract.name.unparse()).toEqual("C");
  expect(contract.members.items).toHaveLength(2);

  // Check that the first member is a variable declaration
  const varDeclVariant = contract.members.items[0].variant;
  expect(varDeclVariant).toBeInstanceOf(StateVariableDefinition);

  // Check that the second member is a function definition
  const funcDefVariant = contract.members.items[1].variant;
  expect(funcDefVariant).toBeInstanceOf(FunctionDefinition);
});

test("create and use separated types", () => {
  const source = `A.B.C`;

  const parser = createParser();

  const parseOutput = parser.parseNonterminal(NonterminalKind.IdentifierPath, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.IdentifierPath);

  // Assuming you have a QualifiedName AST class
  const qualifiedName = new IdentifierPath(cst);
  const identifiers = qualifiedName.items.map((identifier) => identifier.unparse());
  expect(identifiers).toStrictEqual(["A", "B", "C"]);

  const periods = qualifiedName.separators.map((separator) => separator.unparse());
  expect(periods).toStrictEqual([".", "."]);
});

test("throws an exception on initializing the wrong type", () => {
  const source = `int a`;

  const parser = createParser();

  const parseOutput = parser.parseNonterminal(NonterminalKind.Parameter, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.Parameter);

  expect(() => new SourceUnit(cst)).toThrow(
    "AST node 'SourceUnit' can only be initialized with a CST node of the same kind. Received 'Parameter' instead.",
  );
});

test("throws an exception on using an incorrect/incomplete CST node", () => {
  const source = `contract`;

  const parser = createParser();

  const parseOutput = parser.parseNonterminal(NonterminalKind.ContractDefinition, source);
  expect(parseOutput.isValid()).toBeFalsy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.ContractDefinition);

  const children = cst.children();
  expect(children).toHaveLength(2);

  const [contractKeyword, skippedTerminal] = children;
  assertTerminalNode(contractKeyword!.node, TerminalKind.ContractKeyword, "contract");
  assertTerminalNode(skippedTerminal!.node, TerminalKind.Missing, "");

  // Creating the tree should succeed, as the fields are lazily intialized.
  const tree = new ContractDefinition(cst);
  assertNonterminalNode(tree.cst, NonterminalKind.ContractDefinition);

  expect(() => tree.name).toThrow(
    "Missing child with label 'name'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.",
  );
});

test("create and use prefix expressions", () => {
  const source = `!foo`;

  const parser = createParser();

  const parseOutput = parser.parseNonterminal(NonterminalKind.Expression, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof PrefixExpression);

  const { operator, operand } = expression.variant;
  assertTerminalNode(operator, TerminalKind.Bang, "!");
  assertTerminalNode(operand.variant, TerminalKind.Identifier, "foo");
});

test("create and use postfix expressions", () => {
  const source = `foo.bar`;

  const parser = createParser();

  const parseOutput = parser.parseNonterminal(NonterminalKind.Expression, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof MemberAccessExpression);

  const { operand, period, member } = expression.variant;
  assertTerminalNode(operand.variant, TerminalKind.Identifier, "foo");
  assertTerminalNode(period, TerminalKind.Period, ".");
  assertTerminalNode(member, TerminalKind.Identifier, "bar");
});

test("create and use binary expressions", () => {
  const source = `foo + bar`;

  const parser = createParser();

  const parseOutput = parser.parseNonterminal(NonterminalKind.Expression, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof AdditiveExpression);

  const { leftOperand, operator, rightOperand } = expression.variant;
  assertTerminalNode(leftOperand.variant, TerminalKind.Identifier, "foo");
  assertTerminalNode(operator, TerminalKind.Plus, "+");
  assertTerminalNode(rightOperand.variant, TerminalKind.Identifier, "bar");
});

it("can reuse the same CST nodes after selectors", () => {
  // Bug: https://github.com/NomicFoundation/slang/issues/1128

  const source = `pragma solidity ^0.8.0;`;
  const parser = createParser();
  const parseOutput = parser.parseNonterminal(NonterminalKind.SourceUnit, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const sourceUnit = new SourceUnit(parseOutput.tree.asNonterminalNode()!);
  assertNonterminalNode(sourceUnit.cst, NonterminalKind.SourceUnit);
  assertNonterminalNode(sourceUnit.members.cst, NonterminalKind.SourceUnitMembers);
  assertNonterminalNode(sourceUnit.cst, NonterminalKind.SourceUnit);
});

function createParser(): Parser {
  return Parser.create(LanguageFacts.latestVersion());
}
