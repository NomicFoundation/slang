import assert from "node:assert";
import { Parser } from "@slang-private/testlang-npm-package/parser";
import {
  AdditionExpression,
  Expression,
  MemberAccessExpression,
  NegationExpression,
  SeparatedIdentifiers,
  SourceUnit,
  SourceUnitMembers,
  Tree,
  TreeNode,
  TreeNodeChild,
} from "@slang-private/testlang-npm-package/ast";
import {
  assertNonterminalNode,
  assertTerminalNode,
  NonterminalKind,
  TerminalKind,
} from "@slang-private/testlang-npm-package/cst";

test("create and use sequence types", () => {
  const source = `tree [A B C];`.trim();

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Tree, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.Tree);

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
  assertNonterminalNode(cst, NonterminalKind.TreeNodeChild);

  const treeNodeChild = new TreeNodeChild(cst);
  assertNonterminalNode(treeNodeChild.cst, NonterminalKind.TreeNodeChild);
  expect(treeNodeChild.variant).toBeInstanceOf(TreeNode);

  const treeNode = treeNodeChild.variant as TreeNode;
  assertNonterminalNode(treeNode.cst, NonterminalKind.TreeNode);
  assertNonterminalNode(treeNode.members.cst, NonterminalKind.TreeNodeChildren);
  assertTerminalNode(treeNode.openBracket, TerminalKind.OpenBracket, "[");
  assertTerminalNode(treeNode.closeBracket, TerminalKind.CloseBracket, "]");
});

test("create and use repeated types", () => {
  const source = `tree [A B C];`.trim();

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Tree, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.Tree);

  const tree = new Tree(cst);
  assertNonterminalNode(tree.node.members.cst, NonterminalKind.TreeNodeChildren);

  const names = tree.node.members.items.map((item) => {
    assertTerminalNode(item.variant);
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
  assertNonterminalNode(cst, NonterminalKind.SeparatedIdentifiers);

  const separatedIdentifiers = new SeparatedIdentifiers(cst);
  assertNonterminalNode(separatedIdentifiers.cst, NonterminalKind.SeparatedIdentifiers);

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
  assertNonterminalNode(cst, NonterminalKind.Tree);

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
  assertNonterminalNode(cst, NonterminalKind.Tree);

  const children = cst.children();
  expect(children).toHaveLength(2);

  const [contractKeyword, skippedTerminal] = children;
  assertTerminalNode(contractKeyword!.node, TerminalKind.TreeKeyword, "tree");
  assertTerminalNode(skippedTerminal!.node, TerminalKind.Missing, "");

  // Creating the tree should succeed, as the fields are lazily intialized.
  const tree = new Tree(cst);
  assertNonterminalNode(tree.cst, NonterminalKind.Tree);

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
  assertNonterminalNode(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof NegationExpression);

  const { operator, operand } = expression.variant;
  assertTerminalNode(operator, TerminalKind.Bang, "!");
  assertTerminalNode(operand.variant, TerminalKind.Identifier, "foo");
});

test("create and use postfix expressions", () => {
  const source = `foo.bar`;

  const parser = Parser.create("1.0.0");

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

  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.Expression, source);
  expect(parseOutput.isValid()).toBeTruthy();

  const cst = parseOutput.tree;
  assertNonterminalNode(cst, NonterminalKind.Expression);

  const expression = new Expression(cst);
  assert(expression.variant instanceof AdditionExpression);

  const { leftOperand, operator, rightOperand } = expression.variant;
  assertTerminalNode(leftOperand.variant, TerminalKind.Identifier, "foo");
  assertTerminalNode(operator, TerminalKind.Plus, "+");
  assertTerminalNode(rightOperand.variant, TerminalKind.Identifier, "bar");
});

it("can reuse the same CST nodes after selectors", () => {
  // Bug: https://github.com/NomicFoundation/slang/issues/1128

  const source = `foo + bar`;

  const parser = Parser.create("1.0.0");
  const parseOutput = parser.parseFileContents(source);
  parseOutput.isValid(); // true

  const sourceUnit = new SourceUnit(parseOutput.tree);
  assertNonterminalNode(sourceUnit.cst, NonterminalKind.SourceUnit);

  assert(sourceUnit.members instanceof SourceUnitMembers);
  assertNonterminalNode(sourceUnit.members.cst, NonterminalKind.SourceUnitMembers);
});
