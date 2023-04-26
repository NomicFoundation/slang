import * as fs from "fs/promises";
import * as github from "@actions/github";
import * as path from "path";
import remarkParse from "remark-parse";
import remarkStringify from "remark-stringify";
import { unified } from "unified";

/*
 * This script is inspired by: https://github.com/changesets/action/blob/main/src/run.ts
 * Main difference is that we create a single release for the entire monorepo, not for every package.
 */
async function createRelease(
  /** @type string */ changelogDir,
  /** @type string */ githubToken,
  /** @type boolean */ dryRun,
) {
  const version = await getPackageVersion(changelogDir);
  const changelogEntry = await getChangelogEntry(changelogDir, version);

  console.log();
  console.log(changelogEntry);
  console.log();

  if (dryRun) {
    console.log("Stopping due to dry run...");
    return;
  }

  const tagName = `v${version}`;

  await github.getOctokit(githubToken).rest.repos.createRelease({
    name: tagName,
    tag_name: tagName,
    body: changelogEntry,
    prerelease: false,
    ...github.context.repo,
  });
}

/**
 * @returns Promise<string>
 */
async function getPackageVersion(/** @type string */ changelogDir) {
  const filePath = path.join(changelogDir, "package.json");
  const fileContents = await fs.readFile(filePath, "utf8");
  const packageJson = JSON.parse(fileContents);

  return packageJson.version;
}

/**
 * @returns Promise<string>
 */
async function getChangelogEntry(/** @type string */ changelogDir, /** @type string */ version) {
  const filePath = path.join(changelogDir, "CHANGELOG.md");
  const fileContents = await fs.readFile(filePath, "utf8");

  const ast = unified().use(remarkParse).parse(fileContents);

  let foundVersion = false;
  const children = [];

  for (const child of ast.children) {
    if (child?.type === "heading" && child.depth === 2) {
      const title = child.children[0];
      if (title?.type === "text" && title.value === version) {
        foundVersion = true;
      } else {
        break; // found a different version. stop here.
      }
    }

    if (foundVersion) {
      children.push(child);
    }
  }

  if (children.length === 0) {
    throw new Error(`Could not find changelog entry for version ${version}\n${fileContents}`);
  }

  ast.children = children;
  return unified().use(remarkStringify).stringify(ast);
}

createRelease(
  /** @type string */ (process.env["CHANGELOG_DIR"]),
  /** @type string */ (process.env["GITHUB_TOKEN"]),
  /** @type string */ (process.env["SLANG_PUBLISH"]) !== "true",
).catch((err) => {
  console.error(err);
  process.exit(1);
});
