import { NonTerminalKind, TerminalKind } from "@slang-private/slang-testlang/kinds";
import { Language } from "@slang-private/slang-testlang/language";
import { Query } from "@slang-private/slang-testlang/query";
import { expectTerminal } from "../utils/cst-helpers";

test("simple query", () => {
  const language = new Language("1.0.0");
  const tree_source = `tree [A [B C] D];`;
  const parse_output = language.parse(NonTerminalKind.Tree, tree_source);

  const query_source = `[TreeNodeChild ... @id [DelimitedIdentifier]]`;
  const query = Query.parse(query_source);

  const query_result = parse_output.createTreeCursor().query([query]);

  const expectNextResult = (name: string) => {
    let result = query_result.next();
    expect(result).not.toBeNull();
    expect(Object.keys(result!.bindings)).toStrictEqual(["id"]);
    let cursors = result!.bindings["id"]!;
    expect(cursors.length).toEqual(1);
    expectTerminal(cursors[0]!.node(), TerminalKind.DelimitedIdentifier, name);
  };

  expectNextResult("A");
  expectNextResult("B");
  expectNextResult("C");
  expectNextResult("D");

  expect(query_result.next()).toBeNull();
});

test("parser error", () => {
  const source = `[TreeNode @b [DelimitedIdentifier]`;
  // The exact error message is not important, just that it throws.
  expect(() => Query.parse(source)).toThrow();
});
