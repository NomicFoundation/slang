import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import {
  assertNonterminalNode,
  assertTerminalNode,
  EdgeLabel,
  NonterminalKind,
  TerminalKind,
} from "@nomicfoundation/slang/cst";

test("parsing nonterminals", () => {
  const source = `x+y`;

  const parser = Parser.create("0.8.28");
  const parseOutput = parser.parseNonterminal(NonterminalKind.AdditiveExpression, source);

  assert(parseOutput.isValid());
  assertNonterminalNode(parseOutput.tree, NonterminalKind.AdditiveExpression, source);

  const children = parseOutput.tree.children();
  assert.strictEqual(children.length, 3);

  assert.strictEqual(children[0].label, EdgeLabel.LeftOperand);
  assertNonterminalNode(children[0].node, NonterminalKind.Expression, "x");

  assert.strictEqual(children[1].label, EdgeLabel.Operator);
  assertTerminalNode(children[1].node, TerminalKind.Plus, "+");

  assert.strictEqual(children[2].label, EdgeLabel.RightOperand);
  assertNonterminalNode(children[2].node, NonterminalKind.Expression, "y");
});
