import * as slang from "@slang-private/slang-testlang";
import { NonterminalKind, TerminalKind } from "@slang-private/slang-testlang/kinds";
import { Language } from "@slang-private/slang-testlang/language";

test("use namespace imports of the API", () => {
  expect(slang.kinds.NonterminalKind.SourceUnit).toEqual("SourceUnit");
  expect(slang.kinds.NonterminalKind.TreeNode).toEqual("TreeNode");
  expect(slang.kinds.TerminalKind.Identifier).toEqual("Identifier");
});

test("use nested imports of the API", () => {
  expect(NonterminalKind.SourceUnit).toEqual("SourceUnit");
  expect(NonterminalKind.TreeNode).toEqual("TreeNode");
  expect(TerminalKind.Identifier).toEqual("Identifier");
});

test("language exposes a root kind", () => {
  expect(Language.rootKind()).toEqual(NonterminalKind.SourceUnit);
});
