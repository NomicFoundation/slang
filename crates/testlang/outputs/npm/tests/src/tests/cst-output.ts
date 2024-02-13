import { Language } from "@slang-private/slang-testlang/language";
import { RuleKind, TokenKind } from "@slang-private/slang-testlang/kinds";
import { expectRule, expectToken } from "../utils/cst-helpers";

test("parse token", () => {
  const source = "About_time";
  const language = new Language("1.0.0");

  const parseTree = language.parse(RuleKind.TreeNodeChild, source).tree();
  expectRule(parseTree, RuleKind.TreeNodeChild);

  const children = parseTree.children();
  expect(children).toHaveLength(1);

  expectToken(children[0]!, TokenKind.DelimitedIdentifier, "About_time");
});

test("parse rule", () => {
  const source = `tree [A [B C] D];`;
  const language = new Language("1.0.0");

  const parseTree = language.parse(RuleKind.SourceUnit, source).tree();
  expectRule(parseTree, RuleKind.SourceUnit);

  const children = parseTree.children();
  expect(children).toHaveLength(1);

  expectRule(children[0]!, RuleKind.SourceUnitMembers);
});

test("trivial cursor access", () => {
  const source = `tree [A [B C] D];`;
  const language = new Language("1.0.0");

  const parseOutput = language.parse(RuleKind.SourceUnit, source);
  const node = parseOutput.createTreeCursor().node();
  expectRule(node, RuleKind.SourceUnit);

  const children = node.children();
  expect(children).toHaveLength(1);
});

test("calculate unicode characters text length", () => {
  const source = `"some 😁 emoji"`;
  const language = new Language("1.0.0");

  const parseTree = language.parse(RuleKind.Literal, source).tree();
  expectRule(parseTree, RuleKind.Literal);

  expect(parseTree.textLength).toEqual({
    char: 14,
    utf16: 15,
    utf8: 17,
  });

  const children = parseTree.children();
  expect(children).toHaveLength(1);

  const token = children[0]!;
  expectToken(token, TokenKind.StringLiteral, `"some 😁 emoji"`);
  expect(token.textLength).toEqual({
    char: 14,
    utf16: 15,
    utf8: 17,
  });
});

test("can unparse rule nodes", () => {
  const source = `tree [A [B C] D];`;
  const language = new Language("1.0.0");

  const parseTree = language.parse(RuleKind.SourceUnit, source).tree();
  expectRule(parseTree, RuleKind.SourceUnit);

  expect(parseTree.unparse()).toEqual(source);
});
