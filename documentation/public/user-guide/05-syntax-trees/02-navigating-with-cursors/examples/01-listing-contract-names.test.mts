import assert from "node:assert";
import { createTree } from "./common.mjs";
import { assertIsTerminalNode, NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";

test("listing contract names", () => {
  const tree = createTree();
  const cursor = tree.createTreeCursor();

  const contracts = [];

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
    assert(cursor.goToFirstChild());
    assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));

    assertIsTerminalNode(cursor.node);
    contracts.push(cursor.node.unparse());

    assert(cursor.goToParent());
  }

  assert.deepStrictEqual(contracts, ["Foo", "Bar", "Baz"]);
});
