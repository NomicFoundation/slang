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
  functionName: string | undefined;
  parser: Parser;

  constructor() {
    super();
    this.parser = Parser.create(LanguageFacts.latestVersion());
  }

  // collect the name of the function being travered
  public override rewriteFunctionDefinition(node: NonterminalNode): Node | undefined {
    const name = node.children().find((edge) => edge.label == EdgeLabel.Name);
    if (!name) {
      return node;
    }

    this.functionName = name.node.unparse().trim();
    // in the recursion is were the injection of code is actually performed
    const recurse = this.rewriteChildren(node);
    this.functionName = undefined;
    return recurse;
  }

  // once in the statements of a function, inject a call to the `log` function.
  public override rewriteStatements(node: NonterminalNode): Node | undefined {
    if (this.functionName) {
      // collect the trivia so we respect the same indentation.
      // NOTE: this will also copy any comment or newlines.
      const trivia = new Array<Edge>();
      this.collectLeadingTrivia(trivia, node);
      const unparsedTrivia = trivia.reduce((soFar, edge) => soFar + edge.node.unparse(), "");

      // the injected code
      const toInject = this.parser.parseNonterminal(
        NonterminalKind.ExpressionStatement,
        `${unparsedTrivia}log("${this.functionName}");\n`,
      ).tree;

      // inject the node and return the new node containing it
      const children = node.children();
      children.unshift(Edge.createWithNonterminal(EdgeLabel.Item, toInject));
      const newNode = NonterminalNode.create(NonterminalKind.Statements, children);
      return newNode;
    }
    return node;
  }

  // at the end of the file, inject the import of the `log` function.
  public override rewriteSourceUnitMembers(node: NonterminalNode): Node | undefined {
    const importMember = this.parser.parseNonterminal(
      NonterminalKind.SourceUnitMember,
      '\nimport { log } from "__logging.sol";\n',
    ).tree;
    const newChildren = node.children();
    newChildren.push(Edge.createWithNonterminal(EdgeLabel.Item, importMember));
    const newNode = NonterminalNode.create(NonterminalKind.SourceUnitMembers, newChildren);

    // NOTE: We should check that the unit actual contained a function where the call to `log` was injected.
    return this.rewriteChildren(newNode);
  }

  // traverse the node's children, collecting the first trivia nodes found along the way and placing them in `edges`.
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
}
