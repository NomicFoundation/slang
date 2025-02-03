// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangCompilation {
  export { InternalCompilationBuilder };
  export { CompilationUnit };
  export { File };
}
import type { BindingGraph } from "./nomic-foundation-slang-bindings.js";
export { BindingGraph };
import type { NonterminalNode } from "./nomic-foundation-slang-cst.js";
export { NonterminalNode };
import type { Cursor } from "./nomic-foundation-slang-cst.js";
export { Cursor };
import type { ParseError } from "./nomic-foundation-slang-parser.js";
export { ParseError };
/**
 * Contains information about imports found in an added source file.
 */
export interface AddFileResponse {
  /**
   * List of cursors to any import paths found in the file.
   */
  importPaths: Cursor[];
}

/**
 * A complete compilation unit is a complete view over all compilation inputs:
 *
 * - All source files, stored as CSTs.
 * - Name binding graph that exposes relationships between definitions and references in these files.
 * - Any relevant compilation options.
 *
 * It also exposes utilities to traverse the compilation unit and query it.
 */
export class CompilationUnit {
  /**
   * Returns the language version this compilation unit is configured for.
   */
  get languageVersion(): string;
  /**
   * Returns a list of all files in the compilation unit.
   */
  files(): File[];
  /**
   * Returns the file with the specified ID, if it exists.
   */
  file(id: string): File | undefined;
  /**
   * Calculates name binding information for all source files within the compilation unit.
   * Returns a graph that contains all found definitions and their references.
   *
   * Note: building this graph is an expensive operation.
   * It is done lazily on the first access, and cached thereafter.
   */
  get bindingGraph(): BindingGraph;
}

/**
 * A single source file in the compilation unit.
 */
export class File {
  /**
   * Returns the unique identifier of this file.
   */
  get id(): string;
  /**
   * Returns the syntax tree of this file.
   */
  get tree(): NonterminalNode;
  /**
   * Returns a list of all errors encountered during parsing this file.
   */
  errors(): ParseError[];
  /**
   * Creates a cursor for traversing the syntax tree of this file.
   */
  createTreeCursor(): Cursor;
}

/**
 * A builder for creating compilation units.
 * Allows incrementally building a transitive list of all files and their imports.
 *
 * This is an internal API, and exposed via a public `CompilationBuilder` wrapper class written in TypeScript.
 * This allows storing/invoking user supplied callbacks in TypeScript, rather than Rust, which has its limitations.
 *
 * @internal
 */
export class InternalCompilationBuilder {
  /**
   * Creates a new compilation builder for the specified language version.
   */
  static create(languageVersion: string): InternalCompilationBuilder;
  /**
   * Adds a source file to the compilation unit.
   */
  addFile(id: string, contents: string): AddFileResponse;
  /**
   * Resolves an import in the source file to the destination file.
   */
  resolveImport(sourceFileId: string, importPath: Cursor, destinationFileId: string): void;
  /**
   * Builds and returns the final compilation unit.
   */
  build(): CompilationUnit;
}
