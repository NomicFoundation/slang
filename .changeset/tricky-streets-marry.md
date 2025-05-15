---
"@nomicfoundation/slang": minor
---

Add new APIs for creating nodes and edges via the Javascript API:

- `NonterminalNode.create(kind: NonterminalKind, children: Edge[]): NonterminalNode`
- `TerminalNode.create(kind: TerminalKind, text: string): TerminalNode`
- `Edge.createNonterminal(label: EdgeLabel, node: NonterminalNode): Edge`
- `Edge.createTerminal(label: EdgeLabel, node: TerminalNode): Edge`
