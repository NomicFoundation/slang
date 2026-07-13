import assert from "node:assert";
import { createTree } from "./common.mjs";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";

test("visiting subtrees", () => {
  const tree = createTree();
  const cursor = tree.createTreeCursor();

  const results: { [contractName: string]: string[] } = {};

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
    const childCursor = cursor.spawn();
    assert(childCursor.goToNextTerminalWithKind(TerminalKind.Identifier));

    const contractName = childCursor.node.unparse();
    results[contractName] = [];

    while (childCursor.goToNextNonterminalWithKind(NonterminalKind.FunctionDefinition)) {
      assert(childCursor.goToNextTerminalWithKind(TerminalKind.Identifier));
      results[contractName].push(childCursor.node.unparse());
    }
  }

  assert.deepStrictEqual(results, {
    Foo: ["foo_func"],
    Bar: ["bar_func"],
    Baz: ["baz_func"],
  });
});
