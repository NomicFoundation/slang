import assert from "node:assert";
import { ParseOutput, Parser } from "@nomicfoundation/slang/parser";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

// <!-- _PRODUCT_README_ (keep in sync) -->

function createTree(): ParseOutput {
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

test('Get contract names', () => {
  const tree = createTree();
  const cursor = tree.createTreeCursor();

  const contracts = [];

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
    assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));
    contracts.push(cursor.node.unparse());
  }

  assert.deepStrictEqual(contracts, ["Foo", "Bar", "Baz"]);
});
