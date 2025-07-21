import assert from "node:assert";

test("CommonJS await import", async () => {
  const { Parser } = await import("@nomicfoundation/slang/parser");
  const { LanguageFacts } = await import("@nomicfoundation/slang/utils");

  const parser = Parser.create(LanguageFacts.latestVersion());
  assert(parser);
});
