import { Language } from "@slang-private/slang-testlang/parser";
import { NonterminalKind, TerminalKind } from "@slang-private/slang-testlang/cst";
import { expectNonterminal, expectTerminal } from "../utils/cst-helpers";

test("parse terminal", () => {
  const source = "About_time";
  const language = new Language("1.0.0");

  const parseTree = language.parse(NonterminalKind.TreeNodeChild, source).tree();
  expectNonterminal(parseTree, NonterminalKind.TreeNodeChild);

  const children = parseTree.children();
  expect(children).toHaveLength(1);

  expectTerminal(children[0]!, TerminalKind.DelimitedIdentifier, "About_time");
});

test("parse nonterminal", () => {
  const source = `tree [A [B C] D];`;
  const language = new Language("1.0.0");

  const parseTree = language.parse(NonterminalKind.SourceUnit, source).tree();
  expectNonterminal(parseTree, NonterminalKind.SourceUnit);

  const children = parseTree.children();
  expect(children).toHaveLength(1);

  expectNonterminal(children[0]!, NonterminalKind.SourceUnitMembers);
});

test("trivial cursor access", () => {
  const source = `tree [A [B C] D];`;
  const language = new Language("1.0.0");

  const parseOutput = language.parse(NonterminalKind.SourceUnit, source);
  const node = parseOutput.createTreeCursor().node();
  expectNonterminal(node, NonterminalKind.SourceUnit);

  const children = node.children();
  expect(children).toHaveLength(1);
});

test("calculate unicode characters text length", () => {
  const source = `"some 😁 emoji"`;
  const language = new Language("1.0.0");

  const parseTree = language.parse(NonterminalKind.Literal, source).tree();
  expectNonterminal(parseTree, NonterminalKind.Literal);

  expect(parseTree.textLength).toEqual({
    line: 0,
    column: 14,
    utf16: 15,
    utf8: 17,
  });

  const children = parseTree.children();
  expect(children).toHaveLength(1);

  const terminal = children[0]!;
  expectTerminal(terminal, TerminalKind.StringLiteral, `"some 😁 emoji"`);
  expect(terminal.textLength).toEqual({
    line: 0,
    column: 14,
    utf16: 15,
    utf8: 17,
  });
});

test("can unparse nonterminal nodes", () => {
  const source = `tree [A [B C] D];`;
  const language = new Language("1.0.0");

  const parseTree = language.parse(NonterminalKind.SourceUnit, source).tree();
  expectNonterminal(parseTree, NonterminalKind.SourceUnit);

  expect(parseTree.unparse()).toEqual(source);
});
