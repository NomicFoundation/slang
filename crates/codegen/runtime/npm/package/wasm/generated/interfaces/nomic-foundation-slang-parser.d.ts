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
  /**
   * Returns the text range where the error occurred in the source code.
   */
  get textRange(): TextRange;
  /**
   * Returns a human-readable message describing the parsing error.
   */
  get message(): string;
}

export class ParseOutput {
  /**
   * Returns the root node of the parsed syntax tree.
   * Even if there are parsing errors, a partial tree will still be available.
   */
  get tree(): Node;
  /**
   * Returns a list of all parsing errors encountered.
   * An empty list indicates successful parsing with no errors.
   */
  errors(): ParseError[];
  /**
   * Returns whether the parse was completely successful with no errors.
   * This is equivalent to checking if the errors list is empty.
   */
  isValid(): boolean;
  /**
   * Creates a cursor for traversing the parsed syntax tree.
   * The cursor starts at the root node of the tree.
   */
  createTreeCursor(): Cursor;
}

export class Parser {
  /**
   * Returns the root nonterminal kind for this parser's grammar.
   * This represents the starting point for parsing a complete source file.
   */
  static rootKind(): NonterminalKind;
  /**
   * Returns a list of language versions supported by this parser.
   * Each version string represents a specific grammar configuration.
   */
  static supportedVersions(): string[];
  /**
   * Creates a new parser instance for the specified language version.
   */
  static create(version: string): Parser;
  /**
   * Returns the language version this parser instance is configured for.
   */
  get version(): string;
  /**
   * Parses the input string starting from the specified nonterminal kind.
   */
  parse(kind: NonterminalKind, input: string): ParseOutput;
}
