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
import { Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

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

class RemovalRewriter extends BaseRewriter {
  public override rewriteFunctionDefinition(_node: NonterminalNode): Node | undefined {
    return undefined;
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
  const node = parse(NonterminalKind.ContractDefinition, "contract AContract {}");
  const rewriter = new BasicRewriter();
  const result = rewriter.rewriteNode(node);
  assert.ok(result);
  if (result instanceof NonterminalNode) {
    assert.equal(result.unparse(), "contract NewName {}");
  } else {
    assert.fail(`result's type is expected to be TerminalNode, but it's ${typeof result}`);
  }
});

test("Rewrite NonterminalNode Deep", () => {
  const node = parse(NonterminalKind.SourceUnit, "contract AContract {}");
  const rewriter = new BasicRewriter();
  const result = rewriter.rewriteNode(node);
  assert.ok(result);
  if (result instanceof NonterminalNode) {
    assert.equal(result.unparse(), "contract NewName {}");
  } else {
    assert.fail(`result's type is expected to be TerminalNode, but it's ${typeof result}`);
  }
});

test("Remove NonterminalNode", () => {
  const contract = `
    contract AContract {
      function test() {
      }
    }
  `;
  const expected = `
    contract AContract {
    }
  `;
  const node = parse(NonterminalKind.ContractDefinition, contract);
  const rewriter = new RemovalRewriter();
  const result = rewriter.rewriteNode(node);
  assert.ok(result);
  if (result instanceof NonterminalNode) {
    assert.equal(result.unparse(), expected);
  } else {
    assert.fail(`result's type is expected to be TerminalNode, but it's ${typeof result}`);
  }
});

function parse(kind: NonterminalKind, input: string): Node {
  const parser = Parser.create(LanguageFacts.latestVersion());
  return parser.parseNonterminal(kind, input).tree;
}
