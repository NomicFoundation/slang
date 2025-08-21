import { Cursor } from "../cst/index.mjs";
import { CompilationUnit } from "./index.mjs";

import * as wasm from "../../wasm/index.mjs";

/**
 * User-provided options and callbacks necessary for the `CompilationBuilder` class to perform its job.
 */
export interface CompilationBuilderConfig {
  /**
   * The language version to parse files with.
   */
  languageVersion: string;

  /**
   * Callback used by this builder to load the contents of a file.
   *
   * The user is responsible for fetching the file from the filesystem.
   * If the file is not found, the callback should return undefined.
   * Any errors thrown by the callback will be propagated to the caller.
   */
  readFile: (fileId: string) => Promise<string | undefined>;

  /**
   * Callback used by this builder to resolve an import path.
   * For example, if a source file contains the following statement:
   *
   * ```solidity
   * import {Foo} from "foo.sol";
   * ```
   *
   * Then the API will invoke the callback with a cursor pointing to the `"foo.sol"` string literal.
   *
   * The user is responsible for resolving it to a file in the compilation, and return its ID.
   * If the callback returns `undefined`, the import will stay unresolved.
   * Any errors thrown by the callback will be propagated to the caller.
   */
  resolveImport: (sourceFileId: string, importPath: Cursor) => Promise<string | undefined>;
}

/**
 * A builder for creating compilation units.
 * Allows incrementally building a list of all files and their imports.
 */
export class CompilationBuilder {
  private readonly seenFiles: Set<string> = new Set();

  private constructor(
    private readonly internalBuilder: wasm.compilation.InternalCompilationBuilder,

    /**
     * The user-supplied configuration.
     */
    public readonly config: CompilationBuilderConfig,
  ) {}

  /**
   * Creates a new compilation builder for the specified language version.
   */
  public static create(config: CompilationBuilderConfig): CompilationBuilder {
    const internalBuilder = wasm.compilation.InternalCompilationBuilder.create(config.languageVersion);
    return new CompilationBuilder(internalBuilder, config);
  }

  /**
   * Adds a source file to the compilation unit.
   * Typically, users only need to add the "root" file, which contains the main contract they are trying to analyze.
   * Any files that are imported by the root file will be discovered and loaded automatically by the config callbacks.
   *
   * Adding multiple files (roots) is supported. For example, an IDE can choose to add all NPM dependencies,
   * regardless of whether they are imported or not, to be able to query the definitions there.
   *
   * Adding a file that has already been added is a no-op.
   */
  public async addFile(id: string): Promise<void> {
    if (this.seenFiles.has(id)) {
      return;
    } else {
      this.seenFiles.add(id);
    }

    const contents = await this.config.readFile(id);
    if (contents === undefined) {
      return;
    }

    const { importPaths } = this.internalBuilder.addFile(id, contents);

    await Promise.all(
      importPaths.map(async (importPath) => {
        const destinationFileId = await this.config.resolveImport(id, importPath);
        if (destinationFileId === undefined) {
          return;
        }

        this.internalBuilder.resolveImport(id, importPath, destinationFileId);

        await this.addFile(destinationFileId);
      }),
    );
  }

  /**
   * Builds and returns the final compilation unit.
   */
  public build(): CompilationUnit {
    return this.internalBuilder.build();
  }
}
