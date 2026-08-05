import assert from "node:assert";
import { createTree } from "./common.mjs";
import { NonterminalKind } from "@nomicfoundation/slang/cst";

test("accessing node positions", () => {
  const tree = createTree();
  const cursor = tree.createTreeCursor();

  const functions = [];

  while (cursor.goToNextNonterminalWithKind(NonterminalKind.FunctionDefinition)) {
    const line = cursor.textRange.start.line;
    const text = cursor.node.unparse().trim();

    functions.push({ line, text });
  }

  assert.deepStrictEqual(functions, [
    { line: 1, text: "function foo_func() {}" },
    { line: 4, text: "function bar_func() {}" },
    { line: 7, text: "function baz_func() {}" },
  ]);
});
