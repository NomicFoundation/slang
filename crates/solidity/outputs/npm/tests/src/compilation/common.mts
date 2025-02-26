import path from "node:path";
import assert from "node:assert";
import { CompilationBuilder } from "@nomicfoundation/slang/compilation";
import { readRepoFile } from "../utils/files.mjs";
import { BindingLocation } from "@nomicfoundation/slang/bindings";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";

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

export function assertUserFileLocation(
  location: BindingLocation,
  fileId: string,
  kind: TerminalKind | NonterminalKind,
  line: number,
) {
  assert(location.isUserFileLocation());

  assert.strictEqual(location.fileId, fileId);
  assert.strictEqual(location.cursor.node.kind, kind);
  assert.strictEqual(location.cursor.textRange.start.line, line);
}
