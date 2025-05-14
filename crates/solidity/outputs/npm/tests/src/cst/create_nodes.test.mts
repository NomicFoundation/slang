import assert from "node:assert";
import {
  NonterminalNode,
  TerminalNode,
  TerminalKind,
  NonterminalKind,
  EdgeLabel,
  Edge,
} from "@nomicfoundation/slang/cst";

test("Create TerminalNode", () => {
  const terminalNode = TerminalNode.create(TerminalKind.ContractKeyword, "contract");
  assert(terminalNode.isTerminalNode());
  assert.equal(terminalNode.kind, TerminalKind.ContractKeyword);
  assert.equal(terminalNode.unparse(), "contract");
});

test("Create NonterminalNode without children", () => {
  const nonterminalNode = NonterminalNode.create(NonterminalKind.ArrayExpression, []);

  assert(nonterminalNode.isNonterminalNode());
  assert.equal(nonterminalNode.asNonterminalNode().kind, NonterminalKind.ArrayExpression);
  assert.equal(nonterminalNode.asNonterminalNode().unparse(), "");
  assert.deepEqual(nonterminalNode.asNonterminalNode().children(), []);
});

test("Create NonterminalNode with children", () => {
  const child = TerminalNode.create(TerminalKind.OpenBracket, "[");

  const childEdge = Edge.createTerminal(EdgeLabel.OpenBracket, child);

  const nonterminalNode = NonterminalNode.create(NonterminalKind.ArrayExpression, [childEdge]);

  assert(nonterminalNode.isNonterminalNode());
  assert.equal(nonterminalNode.asNonterminalNode().kind, NonterminalKind.ArrayExpression);
  assert.equal(nonterminalNode.asNonterminalNode().unparse(), "[");
  assert.deepEqual(nonterminalNode.asNonterminalNode().children(), [childEdge]);
});
