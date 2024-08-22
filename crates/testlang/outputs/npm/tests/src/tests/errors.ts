import { Language } from "@slang-private/slang-testlang/language";
import { NonterminalKind, TextIndex } from "@slang-private/slang-testlang/cst";
import { Severity } from "@slang-private/slang-testlang/diagnostic";

test("render error reports", () => {
  const source = "tree [AB;";
  const language = new Language("1.0.0");

  const errors = language.parse(NonterminalKind.SourceUnit, source).errors();
  expect(errors).toHaveLength(1);

  expect(errors[0]!.severity()).toBe(Severity.Error);
  expect(errors[0]!.message()).toBe("Expected Identifier or StringLiteral or TreeKeyword.");
  expect(errors[0]!.textRange().start).toEqual({ utf8: 5, utf16: 5, column: 5, line: 0 } satisfies TextIndex);
  expect(errors[0]!.textRange().end).toEqual({ utf8: 9, utf16: 9, column: 9, line: 0 } satisfies TextIndex);
});

test("invalid semantic version", () => {
  expect(() => new Language("foo_bar")).toThrow("Invalid semantic version 'foo_bar'.");
});

test("unsupported language version", () => {
  expect(() => new Language("0.0.0")).toThrow("Unsupported language version '0.0.0'.");
});
