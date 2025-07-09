import { Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("render error reports", () => {
  const source = "contract [AB;";
  const parser = Parser.create(LanguageFacts.latestVersion());

  const parseOutput = parser.parseFileContents(source);
  expect(parseOutput.isValid()).toBeFalsy();

  const errors = parseOutput.errors();
  expect(errors).toHaveLength(1);

  expect(errors[0].message).toBe("Expected Identifier.");

  expect(errors[0].textRange).toEqual({
    start: { utf8: 8, utf16: 8, column: 8, line: 0 },
    end: { utf8: 13, utf16: 13, column: 13, line: 0 },
  });
});
