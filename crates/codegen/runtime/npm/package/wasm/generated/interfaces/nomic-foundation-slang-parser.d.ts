// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangParser {
  export { Parser };
  export { ParseError };
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
 * Contains information about where the error occurred and what went wrong.
 */
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

/**
 * The output of a parsing operation.
 * Contains the resulting syntax tree and any errors encountered during parsing.
 */
export class ParseOutput {
  /**
   * Returns the root node of the parsed syntax tree.
   * Even if there are parsing errors, a partial tree will still be available.
   */
  get tree(): NonterminalNode;
  /**
   * Returns a list of all parsing errors encountered.
   * An empty list indicates successful parsing with no errors.
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
   * Returns the root nonterminal kind for this parser's grammar.
   * This represents the starting point for parsing a complete source file.
   */
  static rootKind(): NonterminalKind;
  /**
   * Creates a new parser instance for the specified language version.
   */
  static create(languageVersion: string): Parser;
  /**
   * Returns the language version this parser instance is configured for.
   */
  get languageVersion(): string;
  /**
   * Parses the input string starting from the specified nonterminal kind.
   */
  parse(kind: NonterminalKind, input: string): ParseOutput;
}
