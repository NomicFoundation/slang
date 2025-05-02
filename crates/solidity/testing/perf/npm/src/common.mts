/// What to test
export enum Options {
  None, // HACK: this is so this option is the false / no entry option
  Parse,
  All // includes binding
}

export interface Test {
  name: string;
  test(languageVersion: string, dir: string, file: string, options: Options): Promise<void>;
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
