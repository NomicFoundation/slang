import { Parser } from "@nomicfoundation/slang/parser";
import { NonterminalKind, TerminalKind, assertNonterminalNode, assertTerminalNode } from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("parse nonterminal", () => {
  const parser = Parser.create(LanguageFacts.latestVersion());
  const source = "about_time";

  const tree = parser.parseNonterminal(NonterminalKind.ImportDeconstructionSymbol, source).tree;
  assertNonterminalNode(tree, NonterminalKind.ImportDeconstructionSymbol);

  const children = tree.children();
  expect(children).toHaveLength(1);

  assertTerminalNode(children[0].node, TerminalKind.Identifier, "about_time");
});

test("parse file contents", () => {
  const parser = Parser.create(LanguageFacts.latestVersion());
  const source = `contract A {}`;

  const tree = parser.parseFileContents(source).tree;
  assertNonterminalNode(tree, NonterminalKind.SourceUnit);

  const children = tree.children();
  expect(children).toHaveLength(1);

  assertNonterminalNode(children[0].node, NonterminalKind.SourceUnitMembers);
});

test("parse unicode characters", () => {
  const parser = Parser.create(LanguageFacts.latestVersion());
  const source = `unicode"some 😁 emoji"`;

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
