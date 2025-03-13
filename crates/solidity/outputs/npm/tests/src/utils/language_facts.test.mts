import assert from "node:assert";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

interface VersionTest {
  pragma: string;
  includes?: string[];
  excludes?: string[];
}

function testVersions(config: VersionTest) {
  const versions = LanguageFacts.inferLanguageVersions(config.pragma);

  if (config.includes) {
    for (const includedVersion of config.includes) {
      assert(versions.includes(includedVersion), `${includedVersion} should be included`);
    }
  }

  if (config.excludes) {
    for (const excludedVersion of config.excludes) {
      assert(!versions.includes(excludedVersion), `${excludedVersion} should not be included`);
    }
  }
}

test("parses concrete version pragma", () => {
  testVersions({
    pragma: "pragma solidity 0.8.1;",
    includes: ["0.8.1"],
  });
});

test("parses caret range", () => {
  testVersions({
    pragma: "pragma solidity ^0.8.0;",
    includes: ["0.8.0", "0.8.5"],
    excludes: ["0.7.0", "1.0.0", "0.9.0"],
  });
});

test("parses hyphen range", () => {
  testVersions({
    pragma: "pragma solidity 0.7.0 - 0.8.0;",
    includes: ["0.7.0", "0.7.2", "0.7.6", "0.8.0"],
    excludes: ["0.6.1", "0.8.1"],
  });
});

test("parses partial version range", () => {
  testVersions({
    pragma: "pragma solidity 0.8",
    includes: ["0.8.0", "0.8.10", "0.8.6"],
    excludes: ["0.6.1", "0.7.0"],
  });
});

test("parses multiple version ranges", () => {
  testVersions({
    pragma: "pragma solidity 0.6.0 || 0.7.1 - 0.8.0",
    includes: ["0.6.0", "0.7.1", "0.7.4", "0.8.0"],
    excludes: ["0.6.5", "0.8.1", "0.7.0"],
  });
});
