import { Definition } from "@nomicfoundation/slang/bindings";
import { BaseRewriter, Node, NonterminalNode } from "@nomicfoundation/slang/cst";

export class Cleaner extends BaseRewriter {
  constructor(private readonly definitions: Definition[]) {
    super();
  }

  removeUnused(node: NonterminalNode): Node | undefined {
    const foundUnused = this.definitions.find((definition) => definition.id == node.id);
    if (foundUnused) {
      // returning `undefined` signals that the node must be deleted
      return undefined;
    } else {
      return node;
    }
  }

  public override rewriteFunctionDefinition(node: NonterminalNode): Node | undefined {
    return this.removeUnused(node);
  }

  public override rewriteStateVariableDefinition(node: NonterminalNode): Node | undefined {
    return this.removeUnused(node);
  }

  public override rewriteModifierDefinition(node: NonterminalNode): Node | undefined {
    return this.removeUnused(node);
  }
}
