import { RuleKind, TokenKind } from "@slang-private/slang-testlang/kinds";
import { Language } from "@slang-private/slang-testlang/language";
import { Query } from "@slang-private/slang-testlang/query";
import { expectToken } from "../utils/cst-helpers";

test("simple query", () => {
  const language = new Language("1.0.0");
  const tree_source = `tree [A [B C] D];`;
  const parse_output = language.parse(RuleKind.Tree, tree_source);

  const query_source = `[TreeNodeChild ... @id [DelimitedIdentifier]]`;
  const query = Query.parse(query_source);

  const query_result = parse_output.createTreeCursor().query([query]);

  const expectNextResult = (name: string) => {
    let result = query_result.next();
    expect(result).not.toBeNull();
    expect(Object.keys(result!.bindings)).toStrictEqual(["id"]);
    let cursors = result!.bindings["id"]!;
    expect(cursors.length).toEqual(1);
    expectToken(cursors[0]!.node(), TokenKind.DelimitedIdentifier, name);
  };

  expectNextResult("A");
  expectNextResult("B");
  expectNextResult("C");
  expectNextResult("D");

  expect(query_result.next()).toBeNull();
});

test("parser error", () => {
  const source = `[TreeNode @b [DelimitedIdentifier]`;
  expect(() => Query.parse(source)).toThrowError(
    `
Parse error:
expected '(' at: [TreeNode @b [DelimitedIdentifier]
Alt at: [TreeNode @b [DelimitedIdentifier]
    `.trim(),
  );
});
