import test from "ava";
import { Language } from "@nomicfoundation/slang/language";
import { ProductionKind } from "@nomicfoundation/slang/syntax/parser";

test("render error reports", (t) => {
  const source = "int256 constant";
  const language = new Language("0.8.1");

  const { errors } = language.parse(ProductionKind.SourceUnit, source);
  t.is(errors.length, 1);

  const report = errors[0]?.toErrorReport("test.sol", source, /* withColor */ false);

  t.is(
    report,
    `
Error: Expected Identifier.
   ╭─[test.sol:1:1]
   │
 1 │ int256 constant
   │ ───────┬───────  
   │        ╰───────── Error occurred here.
───╯
`.trim(),
  );
});

test("invalid semantic version", (t) => {
  t.throws(() => new Language("foo_bar"), {
    code: "GenericFailure",
    message: "Invalid semantic version 'foo_bar'.",
  });
});

test("unsupported language version", (t) => {
  t.throws(() => new Language("0.0.0"), {
    code: "GenericFailure",
    message: "Unsupported Solidity language version '0.0.0'.",
  });
});

test("invalid production version", (t) => {
  t.throws(() => new Language("0.4.11").parse(ProductionKind.ConstructorDefinition, ""), {
    code: "GenericFailure",
    message: "Production 'ConstructorDefinition' is not valid in this version of Solidity.",
  });
});
