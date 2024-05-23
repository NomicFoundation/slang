import { Language } from "@slang-private/slang-testlang/language";
import { NonTerminalKind, TerminalKind } from "@slang-private/slang-testlang/kinds";
import { expectNonTerminal, expectTerminal } from "../utils/cst-helpers";

test("parse token", () => {
  const source = "About_time";
  const language = new Language("1.0.0");

  const parseTree = language.parse(NonTerminalKind.TreeNodeChild, source).tree();
  expectNonTerminal(parseTree, NonTerminalKind.TreeNodeChild);

  const children = parseTree.children();
  expect(children).toHaveLength(1);

  expectTerminal(children[0]!, TerminalKind.DelimitedIdentifier, "About_time");
});

test("parse rule", () => {
  const source = `tree [A [B C] D];`;
  const language = new Language("1.0.0");

  const parseTree = language.parse(NonTerminalKind.SourceUnit, source).tree();
  expectNonTerminal(parseTree, NonTerminalKind.SourceUnit);

  const children = parseTree.children();
  expect(children).toHaveLength(1);

  expectNonTerminal(children[0]!, NonTerminalKind.SourceUnitMembers);
});

test("trivial cursor access", () => {
  const source = `tree [A [B C] D];`;
  const language = new Language("1.0.0");

  const parseOutput = language.parse(NonTerminalKind.SourceUnit, source);
  const node = parseOutput.createTreeCursor().node();
  expectNonTerminal(node, NonTerminalKind.SourceUnit);

  const children = node.children();
  expect(children).toHaveLength(1);
});

test("calculate unicode characters text length", () => {
  const source = `"some 😁 emoji"`;
  const language = new Language("1.0.0");

  const parseTree = language.parse(NonTerminalKind.Literal, source).tree();
  expectNonTerminal(parseTree, NonTerminalKind.Literal);

  expect(parseTree.textLength).toEqual({
    line: 0,
    column: 14,
    utf16: 15,
    utf8: 17,
  });

  const children = parseTree.children();
  expect(children).toHaveLength(1);

  const token = children[0]!;
  expectTerminal(token, TerminalKind.StringLiteral, `"some 😁 emoji"`);
  expect(token.textLength).toEqual({
    line: 0,
    column: 14,
    utf16: 15,
    utf8: 17,
  });
});

test("can unparse rule nodes", () => {
  const source = `tree [A [B C] D];`;
  const language = new Language("1.0.0");

  const parseTree = language.parse(NonTerminalKind.SourceUnit, source).tree();
  expectNonTerminal(parseTree, NonTerminalKind.SourceUnit);

  expect(parseTree.unparse()).toEqual(source);
});
