import assert from "node:assert";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("supported versions", () => {
  assert.strictEqual(LanguageFacts.allVersions().length, 89);

  assert.strictEqual(LanguageFacts.earliestVersion(), "0.4.11");

  assert.strictEqual(LanguageFacts.latestVersion(), "0.8.34");
});
