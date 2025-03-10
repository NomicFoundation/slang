import { Parser } from "@slang-private/testlang-npm-package/parser";
import {
  NonterminalKind,
  TerminalKind,
  assertNonterminalNode,
  assertTerminalNode,
} from "@slang-private/testlang-npm-package/cst";

test("parse terminal", () => {
  const source = "About_time";
  const parser = Parser.create("1.0.0");

  const tree = parser.parseNonterminal(NonterminalKind.TreeNodeChild, source).tree;
  assertNonterminalNode(tree, NonterminalKind.TreeNodeChild);

  const children = tree.children();
  expect(children).toHaveLength(1);

  assertTerminalNode(children[0].node, TerminalKind.DelimitedIdentifier, "About_time");
});

test("parse nonterminal", () => {
  const source = `tree [A [B C] D];`;
  const parser = Parser.create("1.0.0");

  const tree = parser.parseFileContents(source).tree;
  assertNonterminalNode(tree, NonterminalKind.SourceUnit);

  const children = tree.children();
  expect(children).toHaveLength(1);

  assertNonterminalNode(children[0].node, NonterminalKind.SourceUnitMembers);
});

test("parse unicode characters", () => {
  const source = `"some 😁 emoji"`;
  const parser = Parser.create("1.0.0");

  const nonTerminal = parser.parseNonterminal(NonterminalKind.Literal, source).tree;
  assertNonterminalNode(nonTerminal, NonterminalKind.Literal);

  expect(nonTerminal.textLength).toEqual({
    line: 0,
    column: 14,
    utf16: 15,
    utf8: 17,
  });

  const terminal = nonTerminal.children()[0].node;

  assertTerminalNode(terminal, TerminalKind.StringLiteral, `"some 😁 emoji"`);

  expect(terminal.textLength).toEqual({
    line: 0,
    column: 14,
    utf16: 15,
    utf8: 17,
  });
});
