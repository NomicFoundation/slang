// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangUtils {
  export { LanguageFacts };
}

/**
 * Provides information about the supported language versions and the grammar.
 */

export class LanguageFacts {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Returns a list of language versions supported by Slang, sorted ascendingly.
   */
  static allVersions(): Array<string>;
  /**
   * Returns the earliest language version supported by Slang.
   */
  static earliestVersion(): string;
  /**
   * Returns the latest language version supported by Slang.
   */
  static latestVersion(): string;
  /**
   * Analyze the version pragmas within a source file, and return a list of supported language versions that match these pragmas.
   * If the source file contains no pragmas, all versions supported by Slang are returned.
   */
  static inferLanguageVersions(input: string): Array<string>;
}
