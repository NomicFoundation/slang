import { Parser } from "@nomicfoundation/slang/parser";
import {
  EdgeLabel,
  NonterminalKind,
  TerminalKind,
  assertNonterminalNode,
  assertTerminalNode,
} from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

describe("nodes", () => {
  const source = `"foo"`;
  const parseOutput = Parser.create(LanguageFacts.latestVersion()).parseNonterminal(
    NonterminalKind.StringLiteral,
    source,
  );

  const nonTerminal = parseOutput.tree;
  assertNonterminalNode(nonTerminal, NonterminalKind.StringLiteral);

  describe("NonTerminal", () => {
    it("unparse()", () => {
      expect(nonTerminal.unparse()).toEqual(source);
    });

    it("toJson()", () => {
      expect(JSON.parse(nonTerminal.toJson())).toEqual({
        kind: NonterminalKind.StringLiteral,
        children: [
          {
            label: EdgeLabel.Variant,
            node: {
              kind: TerminalKind.DoubleQuotedStringLiteral,
              text: source,
            },
          },
        ],
      });
    });
  });

  const terminal = nonTerminal.children()[0].node;
  assertTerminalNode(terminal, TerminalKind.DoubleQuotedStringLiteral);

  describe("Terminal", () => {
    it("unparse()", () => {
      expect(terminal.unparse()).toEqual(source);
    });

    it("toJson()", () => {
      expect(JSON.parse(terminal.toJson())).toEqual({
        kind: TerminalKind.DoubleQuotedStringLiteral,
        text: source,
      });
    });
  });
});
