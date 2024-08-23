import assert from "node:assert";
import { Parser } from "@slang-private/slang-testlang/parser";
import {
  AdditionExpression,
  Expression,
  MemberAccessExpression,
  NegationExpression,
  SeparatedIdentifiers,
  SourceUnit,
  Tree,
  TreeNode,
  TreeNodeChild,
} from "@slang-private/slang-testlang/ast";
import { expectNonterminal, expectTerminal } from "../utils/cst-helpers";
import { NonterminalKind, TerminalKind, TerminalNode } from "@slang-private/slang-testlang/cst";

test("create and use sequence types", () => {
  const source = `tree [A B C];`.trim();

  const parser = new Parser("1.0.0");

  const parseOutput = parser.parse(NonterminalKind.Tree, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectNonterminal(cst, NonterminalKind.Tree);

  const tree = new Tree(cst);
  expectNonterminal(tree.cst, NonterminalKind.Tree);
  expect(tree.name).toBeUndefined();
  expect(tree.node.members.items).toHaveLength(3);
});

test("create and use choice types", () => {
  const source = `[B]`.trim();

  const parser = new Parser("1.0.0");

  const parseOutput = parser.parse(NonterminalKind.TreeNodeChild, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectNonterminal(cst, NonterminalKind.TreeNodeChild);

  const tree_node_child = new TreeNodeChild(cst);
  expectNonterminal(tree_node_child.cst, NonterminalKind.TreeNodeChild);
  expect(tree_node_child.variant).toBeInstanceOf(TreeNode);

  const tree_node = tree_node_child.variant as TreeNode;
  expectNonterminal(tree_node.cst, NonterminalKind.TreeNode);
  expectNonterminal(tree_node.members.cst, NonterminalKind.TreeNodeChildren);
  expectTerminal(tree_node.openBracket, TerminalKind.OpenBracket, "[");
  expectTerminal(tree_node.closeBracket, TerminalKind.CloseBracket, "]");
});

test("create and use repeated types", () => {
  const source = `tree [A B C];`.trim();

  const parser = new Parser("1.0.0");

  const parseOutput = parser.parse(NonterminalKind.Tree, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectNonterminal(cst, NonterminalKind.Tree);

  const tree = new Tree(cst);
  expectNonterminal(tree.node.members.cst, NonterminalKind.TreeNodeChildren);

  const names = tree.node.members.items.map((item) => {
    expect(item.variant).toBeInstanceOf(TerminalNode);

    return (item.variant as TerminalNode).text;
  });

  expect(names).toStrictEqual(["A", "B", "C"]);
});

test("create and use separated types", () => {
  const source = `Foo.Bar.Baz`;

  const parser = new Parser("1.0.0");

  const parseOutput = parser.parse(NonterminalKind.SeparatedIdentifiers, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectNonterminal(cst, NonterminalKind.SeparatedIdentifiers);

  const separated_identifiers = new SeparatedIdentifiers(cst);
  expectNonterminal(separated_identifiers.cst, NonterminalKind.SeparatedIdentifiers);

  const identifiers = separated_identifiers.items.map((identifier) => identifier.text);
  expect(identifiers).toStrictEqual(["Foo", "Bar", "Baz"]);

  const periods = separated_identifiers.separators.map((separator) => separator.text);
  expect(periods).toStrictEqual([".", "."]);
});

test("throws an exception on initializing the wrong type", () => {
  const source = `tree [A];`;

  const parser = new Parser("1.0.0");

  const parseOutput = parser.parse(NonterminalKind.Tree, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectNonterminal(cst, NonterminalKind.Tree);

  expect(() => new SourceUnit(cst)).toThrow("SourceUnit can only be initialized with a CST node of the same kind.");
});

test("throws an exception on on using an incorrect/incomplete CST node", () => {
  const source = `tree`;

  const parser = new Parser("1.0.0");

  const parseOutput = parser.parse(NonterminalKind.Tree, source);
  expect(parseOutput.errors()).toHaveLength(1);

  const cst = parseOutput.tree();
  expectNonterminal(cst, NonterminalKind.Tree);
  expect(cst.children()).toHaveLength(2);

  const [contractKeyword, skippedTerminal] = cst.children();
  expectTerminal(contractKeyword, TerminalKind.TreeKeyword, "tree");
  expectTerminal(skippedTerminal, TerminalKind.MISSING, "");

  // Creating the tree should succeed, as the fields are lazily intialized.
  const tree = new Tree(cst);
  expectNonterminal(tree.cst, NonterminalKind.Tree);

  expect(() => tree.node).toThrow(
    "Missing child with label 'node'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.",
  );
});

test("create and use prefix expressions", () => {
  const source = `!foo`;

  const parser = new Parser("1.0.0");

  const parseOutput = parser.parse(NonterminalKind.Expression, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectNonterminal(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof NegationExpression);

  const { operator, operand } = expression.variant;
  expectTerminal(operator, TerminalKind.Bang, "!");
  expectTerminal(operand.variant, TerminalKind.Identifier, "foo");
});

test("create and use postfix expressions", () => {
  const source = `foo.bar`;

  const parser = new Parser("1.0.0");

  const parseOutput = parser.parse(NonterminalKind.Expression, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectNonterminal(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof MemberAccessExpression);

  const { operand, period, member } = expression.variant;
  expectTerminal(operand.variant, TerminalKind.Identifier, "foo");
  expectTerminal(period, TerminalKind.Period, ".");
  expectTerminal(member, TerminalKind.Identifier, "bar");
});

test("create and use binary expressions", () => {
  const source = `foo + bar`;

  const parser = new Parser("1.0.0");

  const parseOutput = parser.parse(NonterminalKind.Expression, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectNonterminal(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof AdditionExpression);

  const { leftOperand, operator, rightOperand } = expression.variant;
  expectTerminal(leftOperand.variant, TerminalKind.Identifier, "foo");
  expectTerminal(operator, TerminalKind.Plus, "+");
  expectTerminal(rightOperand.variant, TerminalKind.Identifier, "bar");
});
