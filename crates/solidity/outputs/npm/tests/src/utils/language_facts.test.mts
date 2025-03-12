import assert from "node:assert";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("parses concrete version pragma", () => {
  const versions = LanguageFacts.inferLanguageVersions("pragma solidity 0.8.1;");

  assert.equal(versions.length, 1);
  assert.equal(versions[0], "0.8.1");
});

// test("parses concrete version with inner quotes", () => {
//   const versions = LanguageFacts.inferLanguageVersions("pragma solidity 0.\"8\".1;");

//   assert.equal(versions.length, 1);
//   assert.equal(versions[0], "0.8.1");
// });
