import { TokenNode } from "@slang-private/slang-testlang/cst";
import { RuleKind } from "@slang-private/slang-testlang/kinds";
import { Language } from "@slang-private/slang-testlang/language";
import { Query } from "@slang-private/slang-testlang/query";

test("simple query", () => {
  const language = new Language("1.0.0");
  const tree_source = `tree [A [B C] D];`;
  const parse_output = language.parse(RuleKind.Tree, tree_source);

  const query_source = `[TreeNodeChild ... @id [DelimitedIdentifier]]`;
  const query = Query.parse(query_source);

  const query_result = parse_output.createTreeCursor().query([query]);
  {
    let result = query_result.next();
    let cursors = result!.bindings["id"]!;
    expect(cursors.length).toEqual(1);
    expect((cursors[0]!.node() as TokenNode).text).toEqual("A");
  }
  {
    let result = query_result.next();
    let cursors = result!.bindings["id"]!;
    expect(cursors.length).toEqual(1);
    expect((cursors[0]!.node() as TokenNode).text).toEqual("B");
  }
  {
    let result = query_result.next();
    let cursors = result!.bindings["id"]!;
    expect(cursors.length).toEqual(1);
    expect((cursors[0]!.node() as TokenNode).text).toEqual("C");
  }
  {
    let result = query_result.next();
    let cursors = result!.bindings["id"]!;
    expect(cursors.length).toEqual(1);
    expect((cursors[0]!.node() as TokenNode).text).toEqual("D");
  }
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

test("parser round trip", () => {
  const source = `[TreeNode @b [DelimitedIdentifier]]`;
  const query = Query.parse(source);
  expect(query).toBeDefined();
  expect(query!.text).toEqual(source);
});
