import { Language } from "@nomicfoundation/slang/language";
import { ProductionKind } from "@nomicfoundation/slang/kinds";

test("render error reports", () => {
  const source = "int256 constant";
  const language = new Language("0.8.1");

  const { errors } = language.parse(ProductionKind.SourceUnit, source);
  expect(errors).toHaveLength(1);

  const report = errors[0]?.toErrorReport("test.sol", source, /* withColor */ false);

  expect(report).toEqual(
    `
Error: Expected Identifier or Semicolon.
   ╭─[test.sol:1:16]
   │
 1 │ int256 constant
   │                │ 
   │                ╰─ Error occurred here.
───╯
`.trim(),
  );
});

test("invalid semantic version", () => {
  expect(() => new Language("foo_bar")).toThrowError("Invalid semantic version 'foo_bar'.");
});

test("unsupported language version", () => {
  expect(() => new Language("0.0.0")).toThrowError("Unsupported Solidity language version '0.0.0'.");
});
