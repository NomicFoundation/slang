import { Parser } from "@slang-private/testlang-npm-package/parser";

test("render error reports", () => {
  const source = "tree [AB;";
  const parser = Parser.create("1.0.0");

  const parseOutput = parser.parseFile(source);
  expect(parseOutput.isValid()).toBeFalsy();

  const errors = parseOutput.errors();
  expect(errors).toHaveLength(1);

  expect(errors[0]!.message).toBe("Expected Identifier or StringLiteral or TreeKeyword.");

  expect(errors[0]!.textRange).toEqual({
    start: { utf8: 5, utf16: 5, column: 5, line: 0 },
    end: { utf8: 9, utf16: 9, column: 9, line: 0 },
  });
});
