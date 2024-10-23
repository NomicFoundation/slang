import { NonterminalKind } from "@slang-private/testlang-npm-package/cst";
import { Parser } from "@slang-private/testlang-npm-package/parser";

test("Parser exposes a root kind", () => {
  expect(Parser.rootKind()).toEqual(NonterminalKind.SourceUnit);
});
