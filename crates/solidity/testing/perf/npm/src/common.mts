import path from "node:path";
import fs from "node:fs/promises";
import assert from "node:assert";
import { CompilationBuilder } from "@nomicfoundation/slang/compilation";

export const INPUT_PATH = "crates/solidity/testing/perf/npm/inputs";

function resolvePath(...relativePaths: string[]): string {
  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);

  return path.join(repoRoot, ...relativePaths);
}

export async function readRepoFile(...relativePaths: string[]): Promise<string> {
  const absolutePath = resolvePath(...relativePaths);
  const source = await fs.readFile(absolutePath, "utf8");

  return source.trim();
}

export async function createBuilder(languageVersion: string): Promise<CompilationBuilder> {
  const builder = CompilationBuilder.create({
    languageVersion,

    readFile: async (fileId) => {
      return readRepoFile(INPUT_PATH, fileId);
    },

    resolveImport: async (sourceFileId, importPath) => {
      const importLiteral = importPath.node.unparse();
      assert(importLiteral.startsWith('"') || importLiteral.startsWith("'"));
      assert(importLiteral.endsWith('"') || importLiteral.endsWith("'"));

      const importString = importLiteral.slice(1, -1);

      // HACK: The source file might be buried in some structure a/b/c/d/file.sol
      // in order to resolve its imports we allow ourselves to walk up the hierarchy
      // until we find the proper root of the import.
      let i = 0;
      while (i < 7) {
        let splat = Array(i + 1).fill("..");
        let file = path.join(sourceFileId, ...splat, importString);
        let realFile = resolvePath(INPUT_PATH, file);
        try {
          if (await fs.stat(realFile)) {
            return file;
          }
        }
        catch { }
        i++;
      }
      throw `Can't resolve import ${importPath}`;
    },
  });

  return builder;
}
