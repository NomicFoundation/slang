import { parse } from "@solidity-parser/parser";
import { log, readRepoFile, resolveImport, Runner, Timing } from "./common.mjs";

export class SolidityParserRunner implements Runner {
  public name = "solidity parser";

  async test(_languageVersion: string, dir: string, file: string): Promise<Timing[]> {
    const start = performance.now();

    let toProcess = new Set<string>([file]);
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

      const content = readRepoFile(dir, filePath);
      const result = parse(content, { tolerant: true, loc: true });
      if (result.errors) {
        console.error("Errors during parsing with solidity-parser:");
        console.error(result.errors);
        process.exit(-1);
      }

      const imports = result.children.filter((f) => f.type == "ImportDirective");

      imports.forEach((imprt) => {
        toProcess.add(resolveImport(dir, filePath, imprt.path));
      });
    }

    return [new Timing("antlr_build_ast_duration", performance.now() - start)];
  }
}
