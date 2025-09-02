---
"@nomicfoundation/slang": minor
---

Adds a rewriter API, allowing the transformation of Solidity code via manipulation of its tree.

To rewrite a tree, the user extends the base class `BaseRewriter` and overrides appropriate method(s):

```typescript
class MyRewriter extends BaseRewriter {
  public override rewriteContractDefinition(node: NonterminalNode): Node | undefined {
    // Code to either replace (return a new node), remove (return undefined), 
    // or edit a node (return a modified copy of the given `node`
  }
}

const rewriter = new MyRewriter();
const newNode = rewriter.rewriteNode(someNode);
```
