export interface Test<T> {
  name: string;
  test(languageVersion: string, dir: string, file: string): Promise<T>;
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
