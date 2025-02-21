// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangAst {
  export { Selectors };
}
import type { Node } from "./nomic-foundation-slang-cst.js";
export { Node };
import type { NonterminalNode } from "./nomic-foundation-slang-cst.js";
export { NonterminalNode };

/**
 * @internal
 */

export class Selectors {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  static sequence(node: NonterminalNode): Array<Node | undefined>;
  static choice(node: NonterminalNode): Node;
  static repeated(node: NonterminalNode): Array<Node>;
  static separated(node: NonterminalNode): Array<Array<Node>>;
}
