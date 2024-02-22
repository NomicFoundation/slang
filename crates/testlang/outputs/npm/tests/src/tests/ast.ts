import { Language } from "@slang-private/slang-testlang/language";
import { RuleKind, TokenKind } from "@slang-private/slang-testlang/kinds";
import { SeparatedIdentifiers, SourceUnit, Tree, TreeNode, TreeNodeChild } from "@slang-private/slang-testlang/ast";
import { expectRule, expectToken } from "../utils/cst-helpers";
import { TokenNode } from "@slang-private/slang-testlang/cst";

test("create and use sequence types", () => {
  const source = `tree [A B C];`.trim();

  const language = new Language("1.0.0");

  const parseOutput = language.parse(RuleKind.Tree, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.Tree);

  const tree = new Tree(cst);
  expectRule(tree.cst, RuleKind.Tree);
  expect(tree.name).toBeUndefined();
  expect(tree.node.members.items).toHaveLength(3);
});

test("create and use choice types", () => {
  const source = `[B]`.trim();

  const language = new Language("1.0.0");

  const parseOutput = language.parse(RuleKind.TreeNodeChild, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.TreeNodeChild);

  const tree_node_child = new TreeNodeChild(cst);
  expectRule(tree_node_child.cst, RuleKind.TreeNodeChild);
  expect(tree_node_child.variant).toBeInstanceOf(TreeNode);

  const tree_node = tree_node_child.variant as TreeNode;
  expectRule(tree_node.cst, RuleKind.TreeNode);
  expectRule(tree_node.members.cst, RuleKind.TreeNodeChildren);
  expectToken(tree_node.openBracket, TokenKind.OpenBracket, "[");
  expectToken(tree_node.closeBracket, TokenKind.CloseBracket, "]");
});

test("create and use repeated types", () => {
  const source = `tree [A B C];`.trim();

  const language = new Language("1.0.0");

  const parseOutput = language.parse(RuleKind.Tree, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.Tree);

  const tree = new Tree(cst);
  expectRule(tree.node.members.cst, RuleKind.TreeNodeChildren);

  const names = tree.node.members.items.map((item) => {
    expect(item.variant).toBeInstanceOf(TokenNode);

    return (item.variant as TokenNode).text;
  });

  expect(names).toStrictEqual(["A", "B", "C"]);
});

test("create and use separated types", () => {
  const source = `Foo.Bar.Baz`;

  const language = new Language("1.0.0");

  const parseOutput = language.parse(RuleKind.SeparatedIdentifiers, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.SeparatedIdentifiers);

  const separated_identifiers = new SeparatedIdentifiers(cst);
  expectRule(separated_identifiers.cst, RuleKind.SeparatedIdentifiers);

  const identifiers = separated_identifiers.items.map((identifier) => identifier.text);
  expect(identifiers).toStrictEqual(["Foo", "Bar", "Baz"]);

  const periods = separated_identifiers.separators.map((separator) => separator.text);
  expect(periods).toStrictEqual([".", "."]);
});

test("throws an exception on initializing the wrong type", () => {
  const source = `tree [A];`;

  const language = new Language("1.0.0");

  const parseOutput = language.parse(RuleKind.Tree, source);
  expect(parseOutput.errors()).toHaveLength(0);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.Tree);

  expect(() => new SourceUnit(cst)).toThrowError(
    "SourceUnit can only be initialized with a CST node of the same kind.",
  );
});

test("throws an exception on on using an incorrect/incomplete CST node", () => {
  const source = `tree`;

  const language = new Language("1.0.0");

  const parseOutput = language.parse(RuleKind.Tree, source);
  expect(parseOutput.errors()).toHaveLength(1);

  const cst = parseOutput.tree();
  expectRule(cst, RuleKind.Tree);
  expect(cst.children()).toHaveLength(2);

  const [contractKeyword, skippedToken] = cst.children();
  expectToken(contractKeyword, TokenKind.TreeKeyword, "tree");
  expectToken(skippedToken, TokenKind.SKIPPED, "");

  // Creating the tree should succeed, as the fields are lazily intialized.
  const tree = new Tree(cst);
  expectRule(tree.cst, RuleKind.Tree);

  expect(() => tree.node).toThrowError(
    "Unexpected SKIPPED token at index '1'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.",
  );
});
