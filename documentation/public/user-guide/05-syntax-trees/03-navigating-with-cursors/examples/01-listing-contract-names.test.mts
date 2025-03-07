import assert from "node:assert";
import { createTree } from "./common.mjs";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";

test("listing contract names", () => {
  const tree = createTree();
  const cursor = tree.createTreeCursor();

  const contracts = [];

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
    assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));

    contracts.push(cursor.node.unparse());
  }

  assert.deepStrictEqual(contracts, ["Foo", "Bar", "Baz"]);
});
