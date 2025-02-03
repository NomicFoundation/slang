import { LanguageFacts } from "@slang-private/testlang-npm-package/utils";

test("allVersions()", () => {
  expect(LanguageFacts.allVersions()).toStrictEqual(["1.0.0", "1.0.1", "1.1.0", "1.1.1"]);
});

test("earliestVersion()", () => {
  expect(LanguageFacts.earliestVersion()).toStrictEqual("1.0.0");
});

test("latestVersion()", () => {
  expect(LanguageFacts.latestVersion()).toStrictEqual("1.1.1");
});
