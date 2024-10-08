// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangParser {
  export { Parser };
  export { ParseError };
  export { ParseOutput };
}
import type { Cursor } from "./nomic-foundation-slang-cst.js";
export { Cursor };
import type { Node } from "./nomic-foundation-slang-cst.js";
export { Node };
import type { NonterminalKind } from "./nomic-foundation-slang-cst.js";
export { NonterminalKind };
import type { TextRange } from "./nomic-foundation-slang-cst.js";
export { TextRange };

export class ParseError {
  get textRange(): TextRange;
  get message(): string;
}

export class ParseOutput {
  get tree(): Node;
  get errors(): ParseError[];
  isValid(): boolean;
  createTreeCursor(): Cursor;
}

export class Parser {
  static rootKind(): NonterminalKind;
  static supportedVersions(): string[];
  static create(version: string): Parser;
  get version(): string;
  parse(kind: NonterminalKind, input: string): ParseOutput;
}
