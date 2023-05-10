import * as generated from "./generated";

export const Language = generated.Language;

export namespace syntax {
  export namespace nodes {
    export const NodeType = generated.NodeType;
    export const RuleKind = generated.RuleKind;
    export const RuleNode = generated.RuleNode;
    export const TokenKind = generated.TokenKind;
    export const TokenNode = generated.TokenNode;
  }

  export namespace parser {
    export const ParseOutput = generated.ParseOutput;
    export const ProductionKind = generated.ProductionKind;
  }
}
