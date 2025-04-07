import path from "node:path";
import fs from "node:fs";
import assert from "node:assert";
import { CompilationBuilder } from "@nomicfoundation/slang/compilation";
import { TerminalKind } from "@nomicfoundation/slang/cst";
import { checkCI, sleep } from "./common.mjs";
import { printTables } from "./common.slang.mjs";

export const INPUT_PATH = "crates/solidity/testing/perf/npm/inputs";

export function resolvePath(...relativePaths: string[]): string {
  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);

  return path.join(repoRoot, ...relativePaths);
}

export function readRepoFile(...relativePaths: string[]): Promise<string> {
  const absolutePath = resolvePath(...relativePaths);
  return fs.promises.readFile(absolutePath, "utf8");
}

export function createBuilder(languageVersion: string): CompilationBuilder {
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
          if (fs.statSync(realFile)) {
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

export async function testFile(languageVersion: string, file: string) {
  try {
    const builder = createBuilder(languageVersion);

    await builder.addFile(file);

    const unit = builder.build();
    const cursor = unit.file(file)!.createTreeCursor();

    // Validation: there shouldn't be any parsing errors, but if there are, let's print them nicely
    const files_w_errors = unit.files().filter((f) => f.errors().length > 0);
    const errors = files_w_errors.flatMap((f) => f.errors().map((e) => [f.id, e.message, e.textRange]));
    assert.deepStrictEqual(errors, []);

    // first access constructs the graph
    const bindingGraph = unit.bindingGraph;
    assert(typeof bindingGraph.definitionAt == "function");

    while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
      const definition = unit.bindingGraph.definitionAt(cursor);
      const reference = unit.bindingGraph.referenceAt(cursor);
      assert(definition || reference || cursor.node.unparse().trim() == "ABIEncoderV2");
    }
  }
  catch (e) {
    console.error(e);
  }

  if (typeof global.gc == "function") {
    global.gc();
  }
  else {
    console.warn("Run without --expose-gc");
  }
}

async function run() {
  let i = 1;
  console.log(`iteration, heap_used, external, delta_heap, delta_external`);
  let { heapUsed, external } = process.memoryUsage();
  while (true) {
    await testFile("0.6.12", "0x00e50FAB64eBB37b87df06Aa46b8B35d5f1A4e1A/contracts/misc/UiPoolDataProviderV2V3.sol");
    await sleep(100);
    if (process.argv.includes("--print-tables")) {
      printTables();
    }
    const { heapUsed: roundHU, external: roundE } = process.memoryUsage();
    console.log(`${i}, ${roundHU}, ${roundE}, ${roundHU - heapUsed}, ${roundE - external}`);
    heapUsed = roundHU;
    external = roundE;
    i += 1;
  }
}

checkCI();
await run();

