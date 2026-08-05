import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { assertNonterminalNode, NonterminalKind } from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("parsing source files", () => {
  const source = `
    contract Foo {}
  `;

  const parser = Parser.create(LanguageFacts.latestVersion());
  const parseOutput = parser.parseFileContents(source);

  assert(parseOutput.isValid());
  assertNonterminalNode(parseOutput.tree, NonterminalKind.SourceUnit, source);
});
