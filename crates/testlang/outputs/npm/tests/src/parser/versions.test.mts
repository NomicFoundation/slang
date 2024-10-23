import { Parser } from "@slang-private/testlang-npm-package/parser";

test("list supported versions", () => {
  const versions = Parser.supportedVersions();

  expect(versions.length).toBeGreaterThan(0);

  expect(versions.includes("1.0.0")).toBeTruthy();
  expect(versions.includes("0.0.0")).toBeFalsy();
});

test("invalid semantic version", () => {
  expect(() => Parser.create("foo_bar")).toThrow("Invalid semantic version: 'foo_bar'");
});

test("unsupported language version", () => {
  expect(() => Parser.create("0.0.0")).toThrow("Unsupported language version '0.0.0'.");
});
