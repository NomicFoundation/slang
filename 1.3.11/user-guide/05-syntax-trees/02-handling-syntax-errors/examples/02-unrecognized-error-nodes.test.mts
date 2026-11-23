import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import {
  assertNonterminalNode,
  assertTerminalNode,
  NonterminalKind,
  TerminalKind,
  TerminalKindExtensions,
} from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("unrecognized error nodes", () => {
  const source = `
    foo();
    %`;

  const parser = Parser.create(LanguageFacts.latestVersion());
  const parseOutput = parser.parseNonterminal(NonterminalKind.Statements, source);
  assert(!parseOutput.isValid());

  const errors = parseOutput.errors();
  assert.strictEqual(errors.length, 1);

  assert.match(errors[0].message, /^Expected AddressKeyword or/);
  assert.deepStrictEqual(errors[0].textRange, {
    start: { line: 2, column: 4, utf8: 16, utf16: 16 },
    end: { line: 2, column: 5, utf8: 17, utf16: 17 },
  });

  const children = parseOutput.tree.children();
  assert.strictEqual(children.length, 3);

  assertNonterminalNode(children[0].node, NonterminalKind.Statement);

  assertTerminalNode(children[1].node, TerminalKind.Whitespace, "    ");
  assert(TerminalKindExtensions.isValid(children[1].node.kind));

  assertTerminalNode(children[2].node, TerminalKind.Unrecognized, "%");
  assert(!TerminalKindExtensions.isValid(children[2].node.kind));
});
