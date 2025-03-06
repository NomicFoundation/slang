import assert from "node:assert";
import { createTree } from "./common.mjs";
import { assertTerminalNode, NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";

test("visiting subtrees", () => {
  const tree = createTree();
  const cursor = tree.createTreeCursor();

  const contracts = [];

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
    const childCursor = cursor.spawn();
    assert(childCursor.goToNextTerminalWithKind(TerminalKind.Identifier));

    assertTerminalNode(childCursor.node);
    contracts.push(childCursor.node.unparse());
  }

  assert.deepStrictEqual(contracts, ["Foo", "Bar", "Baz"]);
});
