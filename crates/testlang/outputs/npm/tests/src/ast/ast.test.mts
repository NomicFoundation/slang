import assert from "node:assert";
import { Parser } from "@slang-private/testlang-npm-package/parser";
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
} from "@slang-private/testlang-npm-package/ast";
import {
  assertIsNonterminalNode,
  assertIsTerminalNode,
  NonterminalKind,
  TerminalKind,
} from "@slang-private/testlang-npm-package/cst";

test("create and use sequence types", () => {
  const source = `tree [A B C];`.trim();

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Tree, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertIsNonterminalNode(cst, NonterminalKind.Tree);

  const ast = new Tree(cst);
  expect(ast.keyword.unparse()).toEqual("tree");
  expect(ast.name).toBeUndefined();
  expect(ast.node.members.items).toHaveLength(3);
  expect(ast.semicolon.unparse()).toEqual(";");
});

test("create and use choice types", () => {
  const source = `[B]`.trim();

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.TreeNodeChild, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertIsNonterminalNode(cst, NonterminalKind.TreeNodeChild);

  const treeNodeChild = new TreeNodeChild(cst);
  assertIsNonterminalNode(treeNodeChild.cst, NonterminalKind.TreeNodeChild);
  expect(treeNodeChild.variant).toBeInstanceOf(TreeNode);

  const treeNode = treeNodeChild.variant as TreeNode;
  assertIsNonterminalNode(treeNode.cst, NonterminalKind.TreeNode);
  assertIsNonterminalNode(treeNode.members.cst, NonterminalKind.TreeNodeChildren);
  assertIsTerminalNode(treeNode.openBracket, TerminalKind.OpenBracket, "[");
  assertIsTerminalNode(treeNode.closeBracket, TerminalKind.CloseBracket, "]");
});

test("create and use repeated types", () => {
  const source = `tree [A B C];`.trim();

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Tree, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertIsNonterminalNode(cst, NonterminalKind.Tree);

  const tree = new Tree(cst);
  assertIsNonterminalNode(tree.node.members.cst, NonterminalKind.TreeNodeChildren);

  const names = tree.node.members.items.map((item) => {
    assertIsTerminalNode(item.variant);
    return item.variant.unparse();
  });

  expect(names).toStrictEqual(["A", "B", "C"]);
});

test("create and use separated types", () => {
  const source = `Foo.Bar.Baz`;

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.SeparatedIdentifiers, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertIsNonterminalNode(cst, NonterminalKind.SeparatedIdentifiers);

  const separatedIdentifiers = new SeparatedIdentifiers(cst);
  assertIsNonterminalNode(separatedIdentifiers.cst, NonterminalKind.SeparatedIdentifiers);

  const identifiers = separatedIdentifiers.items.map((identifier) => identifier.unparse());
  expect(identifiers).toStrictEqual(["Foo", "Bar", "Baz"]);

  const periods = separatedIdentifiers.separators.map((separator) => separator.unparse());
  expect(periods).toStrictEqual([".", "."]);
});

test("throws an exception on initializing the wrong type", () => {
  const source = `tree [A];`;

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Tree, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertIsNonterminalNode(cst, NonterminalKind.Tree);

  expect(() => new SourceUnit(cst)).toThrow(
    "AST node 'SourceUnit' can only be initialized with a CST node of the same kind. Received 'Tree' instead.",
  );
});

test("throws an exception on using an incorrect/incomplete CST node", () => {
  const source = `tree`;

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Tree, source);
  expect(parseOutput.isValid()).toBeFalsy();

  const cst = parseOutput.tree;
  assertIsNonterminalNode(cst, NonterminalKind.Tree);

  const children = cst.children();
  expect(children).toHaveLength(2);

  const [contractKeyword, skippedTerminal] = children;
  assertIsTerminalNode(contractKeyword!.node, TerminalKind.TreeKeyword, "tree");
  assertIsTerminalNode(skippedTerminal!.node, TerminalKind.Missing, "");

  // Creating the tree should succeed, as the fields are lazily intialized.
  const tree = new Tree(cst);
  assertIsNonterminalNode(tree.cst, NonterminalKind.Tree);

  expect(() => tree.node).toThrow(
    "Missing child with label 'node'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.",
  );
});

test("create and use prefix expressions", () => {
  const source = `!foo`;

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Expression, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertIsNonterminalNode(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof NegationExpression);

  const { operator, operand } = expression.variant;
  assertIsTerminalNode(operator, TerminalKind.Bang, "!");
  assertIsTerminalNode(operand.variant, TerminalKind.Identifier, "foo");
});

test("create and use postfix expressions", () => {
  const source = `foo.bar`;

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Expression, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertIsNonterminalNode(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof MemberAccessExpression);

  const { operand, period, member } = expression.variant;
  assertIsTerminalNode(operand.variant, TerminalKind.Identifier, "foo");
  assertIsTerminalNode(period, TerminalKind.Period, ".");
  assertIsTerminalNode(member, TerminalKind.Identifier, "bar");
});

test("create and use binary expressions", () => {
  const source = `foo + bar`;

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Expression, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertIsNonterminalNode(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof AdditionExpression);

  const { leftOperand, operator, rightOperand } = expression.variant;
  assertIsTerminalNode(leftOperand.variant, TerminalKind.Identifier, "foo");
  assertIsTerminalNode(operator, TerminalKind.Plus, "+");
  assertIsTerminalNode(rightOperand.variant, TerminalKind.Identifier, "bar");
});

it("can reuse the same CST nodes after selectors", () => {
  // Bug: https://github.com/NomicFoundation/slang/issues/1128

  const source = `foo + bar`;

  const parser = Parser.create("1.0.0");
  const parseOutput = parser.parseFileContents(source);
  parseOutput.isValid(); // true

  const cst = parseOutput.tree.asNonterminalNode()!;
  const ast = new SourceUnit(cst);

  expect(ast.cst.kind).toBe(NonterminalKind.SourceUnit);

  expect(ast.members.cst.kind).toBe(NonterminalKind.SourceUnitMembers);

  expect(ast.cst.kind).toBe(NonterminalKind.SourceUnit);
});
