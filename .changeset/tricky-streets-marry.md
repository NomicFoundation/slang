---
"@nomicfoundation/slang": minor
---

Add new APIs for creating nodes and edges via the Javascript API:

- `NonterminalNode.create(kind: NonterminalKind, children: Edge[]): NonterminalNode`
- `TerminalNode.create(kind: TerminalKind, text: string): TerminalNode`
- `createEdge(label: EdgeLabel, node: Node): Edge`
- `Edge.createNonterminal(label: EdgeLabel, node: NonterminalNode): Edge`
- `Edge.createTerminal(label: EdgeLabel, node: TerminalNode): Edge`
