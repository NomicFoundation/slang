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
  assertTerminalNode,
  assertNonterminalNode,
} from "@nomicfoundation/slang/cst";
import { Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("Rewrite TerminalNode", () => {
  class IdRewriter extends BaseRewriter {
    public override rewriteIdentifier(terminalNode: TerminalNode): Node | undefined {
      return TerminalNode.create(terminalNode.kind, terminalNode.unparse() + "New");
    }
  }

  const terminalNode = TerminalNode.create(TerminalKind.Identifier, "test");
  const rewriter = new IdRewriter();
  const result = rewriter.rewriteNode(terminalNode);
  assertTerminalNode(result, TerminalKind.Identifier, "testNew");
});

test("Rewrite NonterminalNode", () => {
  class ContractNameRewriter extends BaseRewriter {
    public override rewriteContractDefinition(node: NonterminalNode): Node | undefined {
      const newChildren = node.children().map((edge) => {
        if (edge.label == EdgeLabel.Name) {
          return Edge.createWithTerminal(EdgeLabel.Name, TerminalNode.create(TerminalKind.Identifier, "NewName"));
        } else {
          return edge;
        }
      });
      return NonterminalNode.create(NonterminalKind.ContractDefinition, newChildren);
    }
  }

  const node = parse(NonterminalKind.ContractDefinition, "contract AContract {}");
  const rewriter = new ContractNameRewriter();
  const result = rewriter.rewriteNode(node);
  assertNonterminalNode(result, NonterminalKind.ContractDefinition, "contract NewName {}");
});

test("Rewrite NonterminalNode Deep", () => {
  class BasicRewriter extends BaseRewriter {
    insideContract = false;

    public override rewriteIdentifier(terminalNode: TerminalNode): Node | undefined {
      if (this.insideContract) {
        const newNode = TerminalNode.create(terminalNode.kind, terminalNode.unparse() + "New");
        this.insideContract = false;
        return newNode;
      } else {
        return terminalNode;
      }
    }

    public override rewriteContractDefinition(node: NonterminalNode): Node | undefined {
      this.insideContract = true;
      return this.rewriteChildren(node);
    }
  }

  const node = parse(NonterminalKind.SourceUnit, "contract AContract {\n  function aFun() public {}\n}");
  const rewriter = new BasicRewriter();
  const result = rewriter.rewriteNode(node);
  assertNonterminalNode(result, NonterminalKind.SourceUnit, "contract AContractNew {\n  function aFun() public {}\n}");
});

test("Remove NonterminalNode", () => {
  class RemovalRewriter extends BaseRewriter {
    public override rewriteFunctionDefinition(_node: NonterminalNode): Node | undefined {
      return undefined;
    }
  }

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
  assertNonterminalNode(result, NonterminalKind.ContractDefinition, expected);
});

function parse(kind: NonterminalKind, input: string): Node {
  const parser = Parser.create(LanguageFacts.latestVersion());
  return parser.parseNonterminal(kind, input).tree;
}
