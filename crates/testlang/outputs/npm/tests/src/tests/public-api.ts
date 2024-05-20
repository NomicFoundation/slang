import * as slang from "@slang-private/slang-testlang";
import { NonTerminalKind, TerminalKind } from "@slang-private/slang-testlang/kinds";

test("use namespace imports of the API", () => {
  expect(slang.kinds.NonTerminalKind.SourceUnit).toEqual("SourceUnit");
  expect(slang.kinds.NonTerminalKind.TreeNode).toEqual("TreeNode");
  expect(slang.kinds.TerminalKind.Identifier).toEqual("Identifier");
});

test("use nested imports of the API", () => {
  expect(NonTerminalKind.SourceUnit).toEqual("SourceUnit");
  expect(NonTerminalKind.TreeNode).toEqual("TreeNode");
  expect(TerminalKind.Identifier).toEqual("Identifier");
});
