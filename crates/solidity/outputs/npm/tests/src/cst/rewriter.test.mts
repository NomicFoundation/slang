import assert from "node:assert";
import {
  BaseRewriter,
  NonterminalNode,
  TerminalNode,
  TerminalKind,
  NonterminalKind,
  EdgeLabel,
  Edge,
  Node,
} from "@nomicfoundation/slang/cst";

class BasicRewriter extends BaseRewriter {
  insideContract = false;

  public override rewriteIdentifier(terminalNode: TerminalNode): Node | undefined {
    if (!this.insideContract) {
      return TerminalNode.create(terminalNode.kind, terminalNode.unparse() + "_new");
    } else {
      return terminalNode;
    }
  }

  public override rewriteContractDefinition(node: NonterminalNode): Node | undefined {
    this.insideContract = true;
    const newChildren = node.children().map((edge) => {
      if (edge.label == EdgeLabel.Name) {
        return Edge.createWithTerminal(EdgeLabel.Name, TerminalNode.create(TerminalKind.Identifier, "NewName"));
      } else {
        return edge;
      }
    });
    const newNode = NonterminalNode.create(NonterminalKind.ContractDefinition, newChildren);
    this.insideContract = false;
    return newNode;
  }
}

test("Rewrite TerminalNode", () => {
  console.log("Testing");
  const terminalNode = TerminalNode.create(TerminalKind.Identifier, "test");
  const rewriter = new BasicRewriter();
  const result = rewriter.rewriteNode(terminalNode);
  assert.ok(result);
  if (result instanceof TerminalNode) {
    assert.equal(result.unparse(), "test_new");
  } else {
    assert.fail(`result's type is expected to be TerminalNode, but it's ${typeof result}`);
  }
});

test("Rewrite NonterminalNode", () => {
  const edges = [
    Edge.createWithTerminal(EdgeLabel.ContractKeyword, TerminalNode.create(TerminalKind.ContractKeyword, "contract")),

    Edge.createWithTerminal(EdgeLabel.Name, TerminalNode.create(TerminalKind.Identifier, "AContract")),
  ];
  const nonterminalNode = NonterminalNode.create(NonterminalKind.ContractDefinition, edges);
  const rewriter = new BasicRewriter();
  const result = rewriter.rewriteNode(nonterminalNode);
  assert.ok(result);
  if (result instanceof NonterminalNode) {
    assert.equal(result.children().length, 2);
    const resultEdge0 = result.children()[0];
    const resultEdge1 = result.children()[1];
    assert.equal(resultEdge0.label, EdgeLabel.ContractKeyword);
    assert.equal(resultEdge0.node.unparse(), "contract");
    assert.equal(resultEdge1.label, EdgeLabel.Name);
    assert.equal(resultEdge1.node.unparse(), "NewName");
  } else {
    assert.fail(`result's type is expected to be TerminalNode, but it's ${typeof result}`);
  }
});
