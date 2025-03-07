import assert from "node:assert";
import { ParseOutput, Parser } from "@nomicfoundation/slang/parser";

export function createTree(): ParseOutput {
  const source = `
    contract Foo {}
    contract Bar {}
    contract Baz {}
  `;

  const parser = Parser.create("0.8.0");

  const parseOutput = parser.parseFileContents(source.trim());
  assert(parseOutput.isValid());

  return parseOutput;
}
