import * as slang from "@slang-private/testlang-npm-package";
import { NonterminalKind, TerminalKind } from "@slang-private/testlang-npm-package/cst";

test("use namespace imports of the API", () => {
  expect(slang.cst.NonterminalKind.SourceUnit).toEqual("SourceUnit");
  expect(slang.cst.NonterminalKind.TreeNode).toEqual("TreeNode");
  expect(slang.cst.TerminalKind.Identifier).toEqual("Identifier");
});

test("use nested imports of the API", () => {
  expect(NonterminalKind.SourceUnit).toEqual("SourceUnit");
  expect(NonterminalKind.TreeNode).toEqual("TreeNode");
  expect(TerminalKind.Identifier).toEqual("Identifier");
});
