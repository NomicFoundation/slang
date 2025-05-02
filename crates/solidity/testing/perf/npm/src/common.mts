export interface Runner {
  name: string;
  test(languageVersion: string, dir: string, file: string): Promise<void>;
}

/// What to test
export enum Options {
  None, // HACK: this is so this option is the false / no entry option
  Parse,
  File, // bindings of main file only
  Project // bindings of the entire project
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
