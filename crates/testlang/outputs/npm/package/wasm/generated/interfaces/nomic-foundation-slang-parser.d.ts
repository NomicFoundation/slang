// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangParser {
  export { Parser };
  export { ParseOutput };
}
import type { Cursor } from "./nomic-foundation-slang-cst.js";
export { Cursor };
import type { NonterminalNode } from "./nomic-foundation-slang-cst.js";
export { NonterminalNode };
import type { NonterminalKind } from "./nomic-foundation-slang-cst.js";
export { NonterminalKind };
import type { TextRange } from "./nomic-foundation-slang-cst.js";
export { TextRange };
/**
 * Represents an error that occurred while parsing source code.
 */
export interface ParseError {
  /**
   * A human-readable message describing what went wrong.
   */
  message: string;
  /**
   * The text range where the error occurred in the source code.
   */
  textRange: TextRange;
}

/**
 * The output of a parsing operation.
 * Contains the resulting syntax tree and any errors encountered during parsing.
 */

export class ParseOutput {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Returns the root node of the parsed syntax tree.
   * Even if there are parsing errors, a partial tree will still be available.
   */
  get tree(): NonterminalNode;
  /**
   * Returns a list of all parsing errors encountered.
   * An empty list indicates a successful parse with no errors.
   */
  errors(): Array<ParseError>;
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

/**
 * A parser instance that can parse source code into syntax trees.
 * Each parser is configured for a specific language version and grammar.
 */

export class Parser {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Creates a new parser instance for the specified language version.
   *
   * It will throw an error if the language version is invalid or not supported.
   */
  static create(languageVersion: string): Parser;
  /**
   * Returns the language version this parser instance is configured for.
   */
  get languageVersion(): string;
  /**
   * Parses the input string into a complete source file.
   */
  parseFileContents(input: string): ParseOutput;
  /**
   * Parses the input string into a nonterminal with the specified kind.
   */
  parseNonterminal(kind: NonterminalKind, input: string): ParseOutput;
}
