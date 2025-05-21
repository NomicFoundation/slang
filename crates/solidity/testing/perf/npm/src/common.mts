import fs from "node:fs";
import path from "node:path";

export const hasGC = typeof global.gc == "function";

export async function runGC() {
  if (hasGC) {
    global.gc!();
    await sleep(100);
  }
}

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export function checkCI() {
  if (process.env["CI"] == undefined) {
    console.error("Must run with CI=true");
  }
}

export function round2(n: number): number {
  return Math.round(n * 100) / 100;
}

const verbose = process.argv.includes("--verbose");

export function log(what: string) {
  if (verbose) {
    console.log(what);
  }
}

export class SolidityCompilation {
  constructor(public compilerVersion: string) { }

  public plainVersion(): string {
    return this.compilerVersion.split("+")[0];
  }
}
export class SolidityProject {

  constructor(public sources: Map<string, string>, public compilation: SolidityCompilation) {
  }

  public static build(jsonFile: string): SolidityProject {
    const json = JSON.parse(fs.readFileSync(jsonFile, "utf8"));
    let sources = new Map<string, string>();

    if (json.sources && typeof json.sources === "object") {
      for (const [file, data] of Object.entries(json.sources)) {
        if (typeof data === "object" && typeof (data as { content?: string }).content === "string") {
          sources.set(file, (data as { content: string }).content);
        }
        else {
          fail("Invalid source in json");
        }
      }
    }
    else {
      fail("No sources in json");
    }

    let compilation;
    if (json.compilation && typeof json.compilation === "object") {
      if (json.compilation.compilerVersion && typeof json.compilation.compilerVersion === "string") {
        compilation = new SolidityCompilation(json.compilation.compilerVersion);
      }
      else {
        fail("No proper version in json");
      }
    }
    else {
      fail("No compilation data in json");
    }

    return new SolidityProject(sources, compilation);
  }

  public fileContents(file: string): string {
    return this.sources.get(file) || fail(`Can't find ${file}`);
  }

  /// Resolves an import of a solidity file. Parameters are:
  /// - `sourceFile`: the relavive path to the file under inspection,
  /// - `importString`: the import string as parsed from the source file.
  /// Returns the relative path of the imported file.
  public resolveImport(sourceFile: string, importString: string): string {
    const sourceFileDir = path.dirname(sourceFile);

    const file = path.normalize(path.join(sourceFileDir, importString));
    if (this.sources.has(file)) {
      return file;
    }
    else if (this.sources.has(importString)) {
      return importString;
    }
    else {
      fail(`Can't resolve import ${importString} in the context of ${sourceFileDir}`);
    }
  }
}

export class Timing {
  public constructor(
    public component: string,
    public time: number,
  ) {}
}

export interface Runner {
  name: string;

  test(project: SolidityProject, file: string): Promise<Timing[]>;
}
