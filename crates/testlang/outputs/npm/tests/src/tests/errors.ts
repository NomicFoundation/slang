import { Language } from "@slang-private/slang-testlang/language";
import { RuleKind } from "@slang-private/slang-testlang/kinds";
import { TextIndex } from "@slang-private/slang-testlang/text_index";
import { diagnostic } from "@slang-private/slang-testlang/generated";

test("render error reports", () => {
  const source = "tree [AB;";
  const language = new Language("1.0.0");

  const errors = language.parse(RuleKind.SourceUnit, source).errors();
  expect(errors).toHaveLength(1);

  const diag = errors[0]!.toDiagnostic();
  expect(diag.severity()).toBe(diagnostic.Severity.Error);
  expect(diag.message()).toBe("Expected Identifier or StringLiteral or TreeKeyword.");
  expect(diag.textRange().start).toEqual({ utf8: 5, utf16: 5, char: 5 } satisfies TextIndex);
  expect(diag.textRange().end).toEqual({ utf8: 9, utf16: 9, char: 9 } satisfies TextIndex);
});

test("invalid semantic version", () => {
  expect(() => new Language("foo_bar")).toThrowError("Invalid semantic version 'foo_bar'.");
});

test("unsupported language version", () => {
  expect(() => new Language("0.0.0")).toThrowError("Unsupported language version '0.0.0'.");
});
