import { LanguageFacts } from "@slang-private/testlang-npm-package/utils";

test("list supported versions", () => {
  const versions = LanguageFacts.supportedVersions();

  expect(versions.length).toBeGreaterThan(0);

  expect(versions.includes("1.0.0")).toBeTruthy();
  expect(versions.includes("0.0.0")).toBeFalsy();
});
