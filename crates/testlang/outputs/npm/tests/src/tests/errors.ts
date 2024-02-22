import { Language } from "@slang-private/slang-testlang/language";
import { RuleKind } from "@slang-private/slang-testlang/kinds";

test("render error reports", () => {
  const source = "tree [AB;";
  const language = new Language("1.0.0");

  const errors = language.parse(RuleKind.SourceUnit, source).errors();
  expect(errors).toHaveLength(1);

  const report = errors[0]!.toErrorReport("test.testlang", source, /* withColor */ false);
  expect(report).toEqual(
    `
Error: Expected Identifier or StringLiteral or TreeKeyword.
   ╭─[test.testlang:1:6]
   │
 1 │ tree [AB;
   │      ──┬─  
   │        ╰─── Error occurred here.
───╯
    `.trim(),
  );
});

test("invalid semantic version", () => {
  expect(() => new Language("foo_bar")).toThrowError("Invalid semantic version 'foo_bar'.");
});

test("unsupported language version", () => {
  expect(() => new Language("0.0.0")).toThrowError("Unsupported Testlang language version '0.0.0'.");
});
