import fs from "node:fs";
import path from "node:path";

export interface Runner {
  name: string;
  // returns the time it takes to run the test
  test(languageVersion: string, dir: string, file: string): Promise<number>;
}

/// What to test
export enum Options {
  None, // HACK: this is so this option is the false / no entry option
  Parse,
  File, // resolve bindings of the main file only
  Project, // resolve bindings of the entire project
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
