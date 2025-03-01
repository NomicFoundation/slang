import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { assertIsNonterminalNode, NonterminalKind } from "@nomicfoundation/slang/cst";

test("parsing source files", () => {
  const source = `
    contract Foo {}
  `;

  const parser = Parser.create("0.8.0");
  const parseOutput = parser.parseFileContents(source);

  assert(parseOutput.isValid());
  assertIsNonterminalNode(parseOutput.tree, NonterminalKind.SourceUnit, source);
});
