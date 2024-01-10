import assert from "node:assert";
import path from "node:path";

export function repoPath(relativePath: string): string {
  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);

  return path.join(repoRoot, relativePath);
}
