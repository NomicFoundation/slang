import assert from "node:assert";
import fs from "node:fs/promises";
import path from "node:path";

export async function readRepoFile(...relativePaths: string[]): Promise<string> {
  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);

  const absolutePath = path.join(repoRoot, ...relativePaths);
  const source = await fs.readFile(absolutePath, "utf8");

  return source.trim();
}
