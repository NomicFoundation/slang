import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import {
  assertTerminalNode,
  EdgeLabel,
  NonterminalKind,
  TerminalKind,
  TerminalKindExtensions,
} from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("handling trivia", () => {
  const source = `  true\n`;

  const parser = Parser.create(LanguageFacts.latestVersion());
  const parseOutput = parser.parseNonterminal(NonterminalKind.Expression, source);
  assert(parseOutput.isValid());

  const children = parseOutput.tree.children();
  assert.strictEqual(children.length, 3);

  assert.strictEqual(children[0].label, EdgeLabel.LeadingTrivia);
  assertTerminalNode(children[0].node, TerminalKind.Whitespace, "  ");
  assert(TerminalKindExtensions.isTrivia(children[0].node.kind));

  assert.strictEqual(children[1].label, EdgeLabel.Variant);
  assertTerminalNode(children[1].node, TerminalKind.TrueKeyword, "true");
  assert(!TerminalKindExtensions.isTrivia(children[1].node.kind));

  assert.strictEqual(children[2].label, EdgeLabel.TrailingTrivia);
  assertTerminalNode(children[2].node, TerminalKind.EndOfLine, "\n");
  assert(TerminalKindExtensions.isTrivia(children[2].node.kind));
});
