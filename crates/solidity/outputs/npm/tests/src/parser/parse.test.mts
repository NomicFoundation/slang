import { Parser } from "@nomicfoundation/slang/parser";
import { NonterminalKind, TerminalKind, assertNonterminalNode, assertTerminalNode } from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

function new_parser(): Parser {
  return Parser.create(LanguageFacts.latestVersion());
}

test("parse terminal", () => {
  const source = "about_time";
  const parser = new_parser();

  const tree = parser.parseNonterminal(NonterminalKind.ImportDeconstructionSymbol, source).tree;
  assertNonterminalNode(tree, NonterminalKind.ImportDeconstructionSymbol);

  const children = tree.children();
  expect(children).toHaveLength(1);

  assertTerminalNode(children[0].node, TerminalKind.Identifier, "about_time");
});

test("parse nonterminal", () => {
  const source = `contract A {}`;
  const parser = new_parser();

  const tree = parser.parseFileContents(source).tree;
  assertNonterminalNode(tree, NonterminalKind.SourceUnit);

  const children = tree.children();
  expect(children).toHaveLength(1);

  assertNonterminalNode(children[0].node, NonterminalKind.SourceUnitMembers);
});

test("parse unicode characters", () => {
  const source = `unicode"some 😁 emoji"`;
  const parser = new_parser();

  const nonTerminal = parser.parseNonterminal(NonterminalKind.UnicodeStringLiteral, source).tree;
  assertNonterminalNode(nonTerminal, NonterminalKind.UnicodeStringLiteral);

  expect(nonTerminal.textLength).toEqual({
    line: 0,
    column: 21,
    utf16: 22,
    utf8: 24,
  });

  expect(nonTerminal.children().length).toBe(1);
  const terminal = nonTerminal.children()[0].node;

  assertTerminalNode(terminal, TerminalKind.DoubleQuotedUnicodeStringLiteral, `unicode"some 😁 emoji"`);

  expect(terminal.textLength).toEqual({
    line: 0,
    column: 21,
    utf16: 22,
    utf8: 24,
  });
});
