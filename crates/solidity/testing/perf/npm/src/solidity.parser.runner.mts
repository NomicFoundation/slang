import path from "node:path";
import { parse } from "@solidity-parser/parser";
import { Runner } from "./common.mjs";
import { readFile } from "node:fs/promises";

export class SolidityParserRunner implements Runner {
  public name = "solidity parser";

  async test(_languageVersion: string, dir: string, file: string) {
    const filePath = path.join(dir, file);
    const content = await readFile(filePath, { encoding: "utf8" });
    const result = parse(content, { tolerant: true, loc: true });
    if (result.errors) {
      console.error("Errors during parsing with solidity-parser:");
      console.error(result.errors);
      process.exit(-1);
    }
  }
}
