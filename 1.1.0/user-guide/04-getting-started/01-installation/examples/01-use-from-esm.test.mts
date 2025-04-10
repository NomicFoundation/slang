import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("top-level ESM import", () => {
  const parser = Parser.create(LanguageFacts.latestVersion());
  assert(parser);
});
