import assert from "node:assert";
import { createTree } from "./common.mjs";
import { NonterminalKind } from "@nomicfoundation/slang/cst";

test("accessing node positions", () => {
  const tree = createTree();
  const cursor = tree.createTreeCursor();

  const contracts = [];

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
    const line = cursor.textRange.start.line;
    const text = cursor.node.unparse().trim();

    contracts.push({ line, text });
  }

  assert.deepStrictEqual(contracts, [
    { line: 0, text: "contract Foo {}" },
    { line: 1, text: "contract Bar {}" },
    { line: 2, text: "contract Baz {}" },
  ]);
});
