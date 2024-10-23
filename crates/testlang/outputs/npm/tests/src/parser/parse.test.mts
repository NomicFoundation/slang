import { Parser } from "@slang-private/testlang-npm-package/parser";
import {
  NonterminalKind,
  TerminalKind,
  assertIsNonterminalNode,
  assertIsTerminalNode,
} from "@slang-private/testlang-npm-package/cst";

test("parse terminal", () => {
  const source = "About_time";
  const parser = Parser.create("1.0.0");

  const parseTree = parser.parse(NonterminalKind.TreeNodeChild, source).tree;
  assertIsNonterminalNode(parseTree, NonterminalKind.TreeNodeChild);

  expect(parseTree.children).toHaveLength(1);

  assertIsTerminalNode(parseTree.children[0]!.node, TerminalKind.DelimitedIdentifier, "About_time");
});

test("parse nonterminal", () => {
  const source = `tree [A [B C] D];`;
  const parser = Parser.create("1.0.0");

  const parseTree = parser.parse(NonterminalKind.SourceUnit, source).tree;
  assertIsNonterminalNode(parseTree, NonterminalKind.SourceUnit);

  expect(parseTree.children).toHaveLength(1);

  assertIsNonterminalNode(parseTree.children[0]!.node, NonterminalKind.SourceUnitMembers);
});

test("parse unicode characters", () => {
  const source = `"some 😁 emoji"`;
  const parser = Parser.create("1.0.0");

  const nonTerminal = parser.parse(NonterminalKind.Literal, source).tree;
  assertIsNonterminalNode(nonTerminal, NonterminalKind.Literal);

  expect(nonTerminal.textLength).toEqual({
    line: 0,
    column: 14,
    utf16: 15,
    utf8: 17,
  });

  const terminal = nonTerminal.children[0]!.node;

  assertIsTerminalNode(terminal, TerminalKind.StringLiteral, `"some 😁 emoji"`);

  expect(terminal.textLength).toEqual({
    line: 0,
    column: 14,
    utf16: 15,
    utf8: 17,
  });
});
