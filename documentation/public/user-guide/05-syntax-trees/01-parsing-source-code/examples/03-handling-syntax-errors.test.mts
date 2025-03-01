import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { assertIsTerminalNode, EdgeLabel, NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";

test("handling syntax errors", () => {
  const source = `contract`;

  const parser = Parser.create("0.8.0");
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

  assert.strictEqual(children[0].label, EdgeLabel.ContractKeyword);
  assertIsTerminalNode(children[0].node, TerminalKind.ContractKeyword, "contract");

  assert.strictEqual(children[1].label, EdgeLabel.Missing);
  assertIsTerminalNode(children[1].node, TerminalKind.Missing, "");
});
