import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";

test("top-level ESM import", () => {
  const parser = Parser.create("0.8.0");
  assert(parser);
});
