import {
  BaseRewriter,
  Edge,
  NonterminalKind,
  EdgeLabel,
  Node,
  NonterminalNode,
  TerminalKindExtensions,
} from "@nomicfoundation/slang/cst";
import { Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

export class LoggingRewriter extends BaseRewriter {
  toInject: string | undefined;

  public override rewriteFunctionDefinition(node: NonterminalNode): Node | undefined {
    const name = node.children().find((edge) => edge.label == EdgeLabel.Name);
    if (!name) {
      return node;
    }

    this.toInject = name.node.unparse().trim();
    const recurse = this.rewriteChildren(NonterminalKind.FunctionDefinition, node);
    this.toInject = undefined;
    return recurse;
  }

  public override rewriteStatements(node: NonterminalNode): Node | undefined {
    if (this.toInject) {
      // add leading trivia to code to inject
      const trivia = new Array<Edge>();
      this.collectLeadingTrivia(trivia, node);
      const unparsedTrivia = trivia.reduce((soFar, edge) => soFar + edge.node.unparse(), "");
      // the injected code
      const toInject = this.parse(NonterminalKind.ExpressionStatement, `${unparsedTrivia}log("${this.toInject}");\n`);

      const children = node.children();
      children.unshift(Edge.createWithNonterminal(EdgeLabel.Item, toInject));
      const newNode = NonterminalNode.create(NonterminalKind.Statements, children);
      return newNode;
    }
    return node;
  }

  collectLeadingTrivia(edges: Array<Edge>, node: Node) {
    for (const edge of node.children()) {
      if (edge.node.isTerminalNode() && TerminalKindExtensions.isTrivia(edge.node.kind)) {
        edges.push(edge);
      } else {
        this.collectLeadingTrivia(edges, edge.node);
        break; // just from the first non-terminal
      }
    }
  }

  parse(kind: NonterminalKind, input: string): NonterminalNode {
    const parser = Parser.create(LanguageFacts.latestVersion());
    return parser.parseNonterminal(kind, input).tree;
  }
}
