import fs from "node:fs";
import path from "node:path";

export class Timing {
  public constructor(
    public component: string,
    public time: number,
  ) {}
}

export interface Runner {
  name: string;
  // returns the time it takes to run the test
  test(languageVersion: string, dir: string, file: string): Promise<Timing[]>;
}

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

export function readRepoFile(...relativePaths: string[]): string {
  const absolutePath = path.join(...relativePaths);
  const source = fs.readFileSync(absolutePath, "utf8");

  return source.trim();
}

const verbose = process.argv.includes("--verbose");

export function log(what: string) {
  if (verbose) {
    console.log(what);
  }
}

/// Resolves an import of a solidity file. Parameters are:
/// - `directory`: the directory of the solidity project,
/// - `sourceFile`: the relavive path to the file under inspection,
/// - `importString`: the import string as parsed from the source file.
/// Returns the relative path of the imported file.
export function resolveImport(directory: string, sourceFile: string, importString: string): string {
  const sourceFileDir = path.dirname(sourceFile);

  // first do a little sanitization of the import string: remove the first slashes
  importString = importString.replace(/^\/*/, "");

  const file = path.normalize(path.join(sourceFileDir, importString));
  const realFile = path.join(directory, file);
  if (fs.statSync(realFile, { throwIfNoEntry: false })) {
    return file;
  } else {
    const realFile = path.normalize(path.join(directory, importString));
    if (fs.statSync(realFile, { throwIfNoEntry: false })) {
      // it's already relative to the direcotry, no need to do anything else
      return importString;
    } else {
      throw `Can't resolve import ${importString}`;
    }
  }
}
