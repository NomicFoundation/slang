import fs from "node:fs";
import path from "node:path";
import { parse } from "@solidity-parser/parser";
import { log, Runner, Timing } from "./common.mjs";
import { readFile } from "node:fs/promises";

export class SolidityParserRunner implements Runner {
  public name = "solidity parser";

  async test(_languageVersion: string, dir: string, file: string): Promise<Timing[]> {
    const start = performance.now();

    let toProcess = new Set<string>([path.join(dir, file)]);
    let processed = new Set<string>();

    log("Start");

    while (toProcess.size > 0) {
      const filePath = toProcess.values().next().value!;
      toProcess.delete(filePath);

      if (processed.has(filePath)) {
        continue;
      }

      log(`To process ${filePath}`);
      processed.add(filePath);

      const content = await readFile(filePath, { encoding: "utf8" });
      const result = parse(content, { tolerant: true, loc: true });
      if (result.errors) {
        console.error("Errors during parsing with solidity-parser:");
        console.error(result.errors);
        process.exit(-1);
      }

      const imports = result.children.filter((f) => f.type == "ImportDirective");

      imports.forEach((imprt) => {
        const path = this.resolvePath(filePath, imprt.path);
        toProcess.add(path);
      });
    }

    return [new Timing("antlr_build_ast_duration", performance.now() - start)];
  }

  resolvePath(filePath: string, importPath: string): string {
    // HACK: The source file might be buried in some structure a/b/c/d/file.sol
    // in order to resolve its imports we allow ourselves to walk up the hierarchy
    // until we find the proper root of the import.
    let i = 0;
    while (i < 7) {
      let splat = Array(i + 1).fill("..");
      let file = path.join(filePath, ...splat, importPath);
      try {
        if (fs.statSync(file)) {
          return file;
        }
      } catch {
        // continue walking up the path
      }
      i++;
    }
    throw `Can't resolve ${importPath} in context of ${filePath}`;
  }
}
