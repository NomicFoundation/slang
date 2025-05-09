import fs from "node:fs";
import { CompilationBuilder, File } from "@nomicfoundation/slang/compilation";
import { TerminalKind } from "@nomicfoundation/slang/cst";
import assert from "node:assert";
// When debugging, add handleTables at the export list at the end of this imported file
import * as slang_raw from "../../../../outputs/npm/package/wasm/generated/solidity_cargo_wasm.component.js";
import { exit } from "node:process";
import path from "node:path";
import { readRepoFile, Runner, Timing } from "./common.mjs";

function createBuilder(languageVersion: string, directory: string): CompilationBuilder {
  languageVersion = languageVersion.split("+")[0];
  const builder = CompilationBuilder.create({
    languageVersion,

    readFile: async (fileId) => {
      return readRepoFile(directory, fileId);
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
        let realFile = path.join(directory, file);
        try {
          if (fs.statSync(realFile)) {
            return file;
          }
        } catch {
          // continue walking up the path
        }
        i++;
      }
      throw `Can't resolve import ${importPath}`;
    },
  });

  return builder;
}

enum Options {
  BindingsFile,
  BindingsProject,
}

class SlangRunner implements Runner {
  public name = "slang";
  options: Options;

  public constructor(options: Options) {
    this.options = options;
  }

  async test(languageVersion: string, dir: string, file: string): Promise<Timing[]> {
    const startTime = performance.now();
    const builder = createBuilder(languageVersion, dir);

    await builder.addFile(file);

    const unit = builder.build();
    const mainFile = unit.file(file)!;
    const parsedAllFilesTime = performance.now();

    // Validation: there shouldn't be any parsing errors, but if there are, let's print them nicely
    const files_w_errors = unit.files().filter((f) => f.errors().length > 0);
    const errors = files_w_errors.flatMap((f) => f.errors().map((e) => [f.id, e.message, e.textRange]));
    assert.deepStrictEqual(errors, []);

    // first access constructs the graph
    assert(typeof unit.bindingGraph.definitionAt == "function");
    const builtGraphTime = performance.now();

    let files: File[] = [mainFile];

    if (this.options == Options.BindingsProject) {
      files = unit.files();
    }

    files.forEach((file) => {
      let cursor = file.createTreeCursor();
      let emptyDefList = [];
      let neitherDefNorRefSet = new Set<string>();

      while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
        const definition = unit.bindingGraph.definitionAt(cursor);
        const reference = unit.bindingGraph.referenceAt(cursor);

        if (reference && reference.definitions().length == 0) {
          emptyDefList.push(file.id, cursor.node.unparse());
        }

        if (!(definition || reference)) {
          neitherDefNorRefSet.add(cursor.node.unparse());
        }
      }

      // We don't recognize `ABIEncoderV2` in `pragma experimental`, so let's ignore it
      const allowed = ["ABIEncoderV2", "v2"];
      const neitherDefNorRefList = Array.from(neitherDefNorRefSet);
      assert.deepStrictEqual(
        neitherDefNorRefList.filter((e) => !allowed.includes(e)),
        [],
      );
      assert.deepStrictEqual(emptyDefList, []);
    });

    const resolutionTime = performance.now() - builtGraphTime;

    const timings = [
      new Timing("slang_parse_all_files_duration", parsedAllFilesTime - startTime),
      new Timing("slang_init_bindings_graph_duration", builtGraphTime - parsedAllFilesTime),
      new Timing("slang_resolve_all_bindings_duration", resolutionTime),
    ];
    return timings;
  }
}

export class SlangBindingsFileRunner extends SlangRunner {
  public constructor() {
    super(Options.BindingsFile);
  }
}

export class SlangBindingsProjectRunner extends SlangRunner {
  public constructor() {
    super(Options.BindingsProject);
  }
}

// DEBUG: See import above to enable this code
export function printTables() {
  if (process.argv.includes("--print-tables")) {
    if ("handleTables" in slang_raw) {
      const tables = slang_raw["handleTables"] as number[][];
      // The tables contain a list of elements coming in pairs:
      // - At even numbers, a 0 means the slot is not free, and with a number distinct from 0 the slot is free, and
      //   the number is the next free slot (or'ed with a constant).
      // - At odd numbers, the actual handle of the object
      const sums = tables.map((table, index) => [
        index,
        table.reduce((accu, elem, elemix) => {
          if ((elemix & 1) === 0 && elem === 0) {
            return accu + 1;
          } else {
            return accu;
          }
        }, 0),
      ]);
      console.log(sums);
    } else {
      console.error(
        "Asked to print tables, but they are not imported. Add `handleTables` to the list of imports in the solidity_cargo_wasm.component.js",
      );
      exit(-1);
    }
  }
}
