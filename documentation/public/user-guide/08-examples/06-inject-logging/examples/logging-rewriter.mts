import {
  BaseRewriter,
  Edge,
  NonterminalKind,
  EdgeLabel,
  Node,
  NonterminalNode,
  TerminalKind,
} from "@nomicfoundation/slang/cst";
import { Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

export class LoggingRewriter extends BaseRewriter {
  functionName: string | undefined;
  injected = false;
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
      this.injected = true;

      // collect the trivia so we respect the same indentation.
      const trivia = this.collectLeadingTrivia(node);
      const unparsedTrivia = trivia.reduce((soFar, edge) => soFar + edge.node.unparse(), "");

      // the injected code
      const toInject = this.parser.parseNonterminal(
        NonterminalKind.ExpressionStatement,
        `${unparsedTrivia}log("${this.functionName}");\n`,
      ).tree;

      // inject the node at the beginning of statements, and return the new node containing it
      const children = node.children();
      children.unshift(Edge.createWithNonterminal(EdgeLabel.Item, toInject));
      return NonterminalNode.create(NonterminalKind.Statements, children);
    }
    return node;
  }

  // at the end of the file, inject the import of the `log` function.
  public override rewriteSourceUnitMembers(node: NonterminalNode): Node | undefined {
    const newNode = this.rewriteChildren(node);

    if (!this.injected) {
      // No function was found, return
      return node;
    }

    const importMember = this.parser.parseNonterminal(
      NonterminalKind.SourceUnitMember,
      '\nimport { log } from "__logging.sol";\n',
    ).tree;
    const newChildren = newNode.children();
    newChildren.push(Edge.createWithNonterminal(EdgeLabel.Item, importMember));
    return NonterminalNode.create(NonterminalKind.SourceUnitMembers, newChildren);
  }

  // Traverse the node's children, collecting the first trivia nodes found along the way and placing them in `edges`.
  collectLeadingTrivia(node: Node): Array<Edge> {
    let edges = new Array<Edge>();
    let currentNode: Node | undefined = node;
    let debug = [];
    while (currentNode && currentNode.children().length > 0) {
      const children: Edge[] = currentNode.children();
      currentNode = undefined;
      for (const edge of children) {
        debug.push(`"Label": "${edge.label.toString()}", "Node": ${edge.node.toJson()},`);
        if (edge.node.isTerminalNode()) {
          if (edge.node.kind == TerminalKind.Whitespace) {
            // Just place whitespaces and tabs
            edges.push(edge);
          } else if (
            edge.node.kind == TerminalKind.MultiLineComment ||
            edge.node.kind == TerminalKind.SingleLineComment
          ) {
            // If there are spaces saved and now found some comment, reset them.
            edges.length = 0;
          } else {
            // Reached a keyword, identifier, etc. Done.
            break;
          }
        } else if (edges.length == 0) {
          // Jump into the first non-terminal
          currentNode = edge.node;
          break;
        } else {
          // Trivia was collected, we can finish now
          break;
        }
      }
    }
    return edges;
  }
}
