import { Language } from "@slang-private/slang-testlang/parser";

test("list supported versions", () => {
  const versions = Language.supportedVersions();

  expect(versions.length).toBeGreaterThan(0);

  expect(versions.includes("1.0.0")).toBeTruthy();
  expect(versions.includes("0.0.0")).toBeFalsy();
});
