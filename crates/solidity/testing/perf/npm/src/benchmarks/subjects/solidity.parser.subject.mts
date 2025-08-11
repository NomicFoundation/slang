import { parse } from "@solidity-parser/parser";
import { Subject, SolidityProject, Timings } from "../common.mjs";

export class AntlrSubject implements Subject {
  public name = "solidity parser";

  async test(project: SolidityProject, file: string): Promise<Timings> {
    const start = performance.now();

    let toProcess = new Array<string>(file);
    let processed = new Set<string>();

    while (toProcess.length > 0) {
      const filePath = toProcess.pop()!;

      if (processed.has(filePath)) {
        continue;
      }

      processed.add(filePath);

      const content = project.fileContents(filePath);
      const result = parse(content, { tolerant: true, loc: true });
      if (result.errors) {
        console.error("Errors during parsing with solidity-parser:");
        console.error(result.errors);
        process.exit(-1);
      }

      const imports = result.children.filter((f) => f.type == "ImportDirective");

      imports.forEach((imprt) => {
        toProcess.push(project.resolveImport(filePath, imprt.path));
      });
    }

    return new Map([["antlr_build_ast_duration", performance.now() - start]]);
  }
}
