import { Parser } from "@nomicfoundation/slang/parser";

test("invalid semantic version", () => {
  expect(() => Parser.create("foo_bar")).toThrow("Invalid semantic version: 'foo_bar'");
});

test("unsupported language version", () => {
  expect(() => Parser.create("0.0.0")).toThrow("Unsupported language version '0.0.0'.");
});
