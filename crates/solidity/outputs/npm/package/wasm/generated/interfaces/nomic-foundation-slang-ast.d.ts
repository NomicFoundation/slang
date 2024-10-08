// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangAst {
  export { Selectors };
}
import type { Node } from "./nomic-foundation-slang-cst.js";
export { Node };
import type { NonterminalNode } from "./nomic-foundation-slang-cst.js";
export { NonterminalNode };

export class Selectors {
  static sequence(node: NonterminalNode): (Node | undefined)[];
  static choice(node: NonterminalNode): Node;
  static repeated(node: NonterminalNode): Node[];
  static separated(node: NonterminalNode): Node[][];
}
