import fs from "node:fs";
import path from "node:path";

export class Timing {
  public component: string;
  public time: number;

  public constructor(component: string, time: number) {
    this.component = component;
    this.time = time;
  }
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

export function log(what: string) {
  if (process.argv.includes("--verbose")) {
    console.log(what);
  }
}
