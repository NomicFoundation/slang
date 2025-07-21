import assert from "node:assert";
import { ParseOutput, Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

export function createTree(): ParseOutput {
  const source = `
    contract Foo {
      function foo_func() {}
    }
    contract Bar {
      function bar_func() {}
    }
    contract Baz {
      function baz_func() {}
    }
  `;

  const parser = Parser.create(LanguageFacts.latestVersion());

  const parseOutput = parser.parseFileContents(source.trim());
  assert(parseOutput.isValid());

  return parseOutput;
}
