---
"@nomicfoundation/slang": minor
---

Add new TypeScript APIs for creating nodes and edges:

- `NonterminalNode.create(kind: NonterminalKind, children: Edge[]): NonterminalNode`
- `TerminalNode.create(kind: TerminalKind, text: string): TerminalNode`
- `createEdge(label: EdgeLabel, node: Node): Edge`
- `Edge.createWithNonterminal(label: EdgeLabel, node: NonterminalNode): Edge`
- `Edge.createWithTerminal(label: EdgeLabel, node: TerminalNode): Edge`
