import assert from "node:assert";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

const src = `
  pragma solidity ^0.8.0;

  contract Test {}
`;

test("infer versions", () => {
  const versions = LanguageFacts.inferLanguageVersions(src);

  assert(versions.includes("0.8.0"));
  assert(versions.includes("0.8.3"));
  assert(!versions.includes("0.7.8"));
  assert(!versions.includes("1.0.0"));

  const latest = versions.at(-1);
  assert(latest == "0.8.33");
});
