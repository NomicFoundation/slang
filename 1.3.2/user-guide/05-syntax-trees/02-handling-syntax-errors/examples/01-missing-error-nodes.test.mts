import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { assertTerminalNode, NonterminalKind, TerminalKind, TerminalKindExtensions } from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("missing error nodes", () => {
  const source = `contract`;

  const parser = Parser.create(LanguageFacts.latestVersion());
  const parseOutput = parser.parseNonterminal(NonterminalKind.ContractDefinition, source);
  assert(!parseOutput.isValid());

  const errors = parseOutput.errors();
  assert.strictEqual(errors.length, 1);

  assert.strictEqual(errors[0].message, "Expected Identifier.");
  assert.deepStrictEqual(errors[0].textRange, {
    start: { line: 0, column: 8, utf8: 8, utf16: 8 },
    end: { line: 0, column: 8, utf8: 8, utf16: 8 },
  });

  const children = parseOutput.tree.children();
  assert.strictEqual(children.length, 2);

  assertTerminalNode(children[0].node, TerminalKind.ContractKeyword, "contract");

  assertTerminalNode(children[1].node, TerminalKind.Missing, "");
  assert(!TerminalKindExtensions.isValid(children[1].node.kind));
});
