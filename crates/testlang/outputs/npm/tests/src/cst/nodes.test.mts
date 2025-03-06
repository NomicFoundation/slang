import { Parser } from "@slang-private/testlang-npm-package/parser";
import {
  EdgeLabel,
  NonterminalKind,
  TerminalKind,
  assertNonterminalNode,
  assertTerminalNode,
} from "@slang-private/testlang-npm-package/cst";

describe("nodes", () => {
  const source = `"foo"`;
  const parseOutput = Parser.create("1.0.0").parseNonterminal(NonterminalKind.Literal, source);

  const nonTerminal = parseOutput.tree;
  assertNonterminalNode(nonTerminal, NonterminalKind.Literal);

  describe("NonTerminal", () => {
    it("unparse()", () => {
      expect(nonTerminal.unparse()).toEqual(source);
    });

    it("toJson()", () => {
      expect(JSON.parse(nonTerminal.toJson())).toEqual({
        kind: NonterminalKind.Literal,
        children: [
          {
            label: EdgeLabel.Variant,
            node: {
              kind: TerminalKind.StringLiteral,
              text: source,
            },
          },
        ],
      });
    });
  });

  const terminal = nonTerminal.children()[0].node;
  assertTerminalNode(terminal, TerminalKind.StringLiteral);

  describe("Terminal", () => {
    it("unparse()", () => {
      expect(terminal.unparse()).toEqual(source);
    });

    it("toJson()", () => {
      expect(JSON.parse(terminal.toJson())).toEqual({
        kind: TerminalKind.StringLiteral,
        text: source,
      });
    });
  });
});
