import path from "node:path";
import assert from "node:assert";
import { CompilationBuilder } from "@nomicfoundation/slang/compilation";
import { readRepoFile } from "../utils/files.mjs";

export async function createBuilder(): Promise<CompilationBuilder> {
  const builder = CompilationBuilder.create({
    languageVersion: "0.8.0",

    readFile: async (fileId) => {
      return readRepoFile("crates/solidity/outputs/npm/tests/src/compilation/inputs", fileId);
    },

    resolveImport: async (sourceFileId, importPath) => {
      const importLiteral = importPath.node.unparse();
      assert(importLiteral.startsWith('"'));
      assert(importLiteral.endsWith('"'));

      const importString = importLiteral.slice(1, -1);
      return path.join(sourceFileId, "..", importString);
    },
  });

  return builder;
}
