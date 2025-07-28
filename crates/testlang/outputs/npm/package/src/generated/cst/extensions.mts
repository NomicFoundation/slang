// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

import { EdgeLabel, Edge, Node } from "./index.mjs";

/** Create a new `Edge` with the label `label` and node `node`. */
export function createEdge(label: EdgeLabel, node: Node): Edge {
  if (node.isNonterminalNode()) {
    return Edge.createWithNonterminal(label, node.asNonterminalNode());
  } else {
    return Edge.createWithTerminal(label, node.asTerminalNode());
  }
}
