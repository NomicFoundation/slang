import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import {
  assertIsNonterminalNode,
  assertIsTerminalNode,
  EdgeLabel,
  NonterminalKind,
  TerminalKind,
} from "@nomicfoundation/slang/cst";

test("parsing nonterminals", () => {
  const source = `x+y`;

  const parser = Parser.create("0.8.0");
  const parseOutput = parser.parseNonterminal(NonterminalKind.AdditiveExpression, source);

  assert(parseOutput.isValid());
  assertIsNonterminalNode(parseOutput.tree, NonterminalKind.AdditiveExpression, source);

  const children = parseOutput.tree.children();
  assert.strictEqual(children.length, 3);

  assert.strictEqual(children[0].label, EdgeLabel.LeftOperand);
  assertIsNonterminalNode(children[0].node, NonterminalKind.Expression, "x");

  assert.strictEqual(children[1].label, EdgeLabel.Operator);
  assertIsTerminalNode(children[1].node, TerminalKind.Plus, "+");

  assert.strictEqual(children[2].label, EdgeLabel.RightOperand);
  assertIsNonterminalNode(children[2].node, NonterminalKind.Expression, "y");
});
