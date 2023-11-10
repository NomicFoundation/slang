import { Language } from "@nomicfoundation/slang/language";

test("list supported versions", () => {
  const versions = Language.supportedVersions();

  expect(versions.length).toBeGreaterThan(0);

  expect(versions.includes("0.4.11")).toBeTruthy();
  expect(versions.includes("0.0.0")).toBeFalsy();
});
