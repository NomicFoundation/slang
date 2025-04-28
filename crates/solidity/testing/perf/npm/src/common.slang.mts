import fs from "node:fs";
import { CompilationBuilder } from "@nomicfoundation/slang/compilation";
import { TerminalKind } from "@nomicfoundation/slang/cst";
import { MathNumericType, max, mean, round, std } from "mathjs";
import assert from "node:assert";
// When debugging, add handleTables at the export list at the end of this imported file
import * as slang_raw from "../../../../outputs/npm/package/wasm/generated/solidity_cargo_wasm.component.js";
import { exit } from "node:process";
import path from "node:path";
import { Test } from "./common.mjs";

export class Record {
  file: string;
  totalTime: number = 0;
  buildGraphTime: number = 0;
  setupTime: number = 0;
  resolutionTime: number = 0;
  maxGoto: number = 0;
  meanGoto: number = 0;
  stdGoto: MathNumericType = 0;
  numberOfIdentifiers: number = 0;

  public constructor(file: string) {
    this.file = file
  }
}


export function readRepoFile(...relativePaths: string[]): string {
  const absolutePath = path.join(...relativePaths);
  const source = fs.readFileSync(absolutePath, "utf8");

  return source.trim();
}

export function createBuilder(languageVersion: string, directory: string): CompilationBuilder {
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
        }
        catch { }
        i++;
      }
      throw `Can't resolve import ${importPath}`;
    },
  });

  return builder;
}

export class SlangTest implements Test<void> {
  public name = "slang";

  async test(languageVersion: string, dir: string, file: string) {
    await testFile(languageVersion, dir, file);
  }
}

export async function testFile(languageVersion: string, dir: string, file: string, expectedDefs?: number, expectedRefs?: number): Promise<Record> {
  let gotoDefTimes: number[] = Array();
  const startTime = performance.now();
  const builder = createBuilder(languageVersion, dir);

  const record = new Record(file);

  await builder.addFile(file);

  const unit = builder.build();
  const cursor = unit.file(file)!.createTreeCursor();
  record.setupTime = round(performance.now() - startTime);

  // Validation: there shouldn't be any parsing errors, but if there are, let's print them nicely
  const files_w_errors = unit.files().filter((f) => f.errors().length > 0);
  const errors = files_w_errors.flatMap((f) => f.errors().map((e) => [f.id, e.message, e.textRange]));
  assert.deepStrictEqual(errors, []);

  let defs = 0;
  let refs = 0;
  let emptyDefList = [];
  let neitherDefNorRefSet = new Set<string>();

  // first access constructs the graph
  assert(typeof unit.bindingGraph.definitionAt == "function");
  record.buildGraphTime = round(performance.now() - startTime - record.setupTime);

  while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
    record.numberOfIdentifiers++;
    const startDefRef = performance.now();
    const definition = unit.bindingGraph.definitionAt(cursor);
    const reference = unit.bindingGraph.referenceAt(cursor);

    const gotoDefTime = performance.now() - startDefRef;

    if (reference) {
      const defs = reference.definitions().length;
      if (defs > 0) {
        refs++;
      } else {
        emptyDefList.push(cursor.node.unparse());
      }
    }

    if (definition) {
      defs++;
    }

    const value = definition || reference;
    if (!value) {
      neitherDefNorRefSet.add(cursor.node.unparse());
    }

    gotoDefTimes.push(gotoDefTime);
  }

  record.totalTime = round(performance.now() - startTime);
  record.resolutionTime = record.totalTime - record.buildGraphTime - record.setupTime;
  record.maxGoto = round(max(gotoDefTimes));
  record.meanGoto = round(mean(gotoDefTimes));
  record.stdGoto = round(std(gotoDefTimes)[0] || 0);

  // We don't recognize `ABIEncoderV2` in `pragma experimental`, so let's ignore it
  const allowed = ["ABIEncoderV2", "v2"];
  const neitherDefNorRefList = Array.from(neitherDefNorRefSet);
  assert.deepStrictEqual(neitherDefNorRefList.filter((e) => !allowed.includes(e)), []);
  assert.deepStrictEqual(emptyDefList, []);

  if (expectedDefs) {
    assert.equal(refs, expectedRefs);
    assert.equal(defs, expectedDefs);
  }

  return record;
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
      const sums = tables.map((table, index) => [index, table.reduce((accu, elem, elemix) => { if ((elemix & 1) === 0 && (elem === 0)) { return (accu + 1) } else { return accu } }, 0)]);
      console.log(sums);
    }
    else {
      console.error("Asked to print tables, but they are not imported. Add `handleTables` to the list of imports in the solidity_cargo_wasm.component.js");
      exit(-1);
    }
  }
}
