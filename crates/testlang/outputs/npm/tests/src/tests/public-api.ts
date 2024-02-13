import * as slang from "@slang-private/slang-testlang";
import { RuleKind, TokenKind } from "@slang-private/slang-testlang/kinds";

test("use namespace imports of the API", () => {
  expect(slang.kinds.RuleKind.SourceUnit).toEqual("SourceUnit");
  expect(slang.kinds.RuleKind.TreeNode).toEqual("TreeNode");
  expect(slang.kinds.TokenKind.Identifier).toEqual("Identifier");
});

test("use nested imports of the API", () => {
  expect(RuleKind.SourceUnit).toEqual("SourceUnit");
  expect(RuleKind.TreeNode).toEqual("TreeNode");
  expect(TokenKind.Identifier).toEqual("Identifier");
});
