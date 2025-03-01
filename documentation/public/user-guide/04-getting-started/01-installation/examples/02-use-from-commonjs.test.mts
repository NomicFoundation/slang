import assert from "node:assert";

test("CommonJS await import", async () => {
  const { Parser } = await import("@nomicfoundation/slang/parser");

  const parser = Parser.create("0.8.0");
  assert(parser);
});
