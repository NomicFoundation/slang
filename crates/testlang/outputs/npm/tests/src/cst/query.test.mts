import {
  NonterminalKind,
  TerminalKind,
  Query,
  assertIsTerminalNode,
  QueryError,
} from "@slang-private/testlang-npm-package/cst";
import { Parser } from "@slang-private/testlang-npm-package/parser";

test("simple query", () => {
  const parser = Parser.create("1.0.0");
  const treeSource = `tree [A [B C] D];`;
  const parseOutput = parser.parse(NonterminalKind.Tree, treeSource);

  const querySource = `[TreeNodeChild @id [DelimitedIdentifier]]`;
  const query = Query.parse(querySource);

  const matches = parseOutput.createTreeCursor().query([query]);

  const expectNextMatch = (name: string) => {
    const match = matches.next()!;
    expect(Object.keys(match.captures)).toStrictEqual(["id"]);

    const cursors = match.captures["id"]!;
    expect(cursors).toHaveLength(1);

    const node = cursors[0]!.node;
    assertIsTerminalNode(node, TerminalKind.DelimitedIdentifier, name);
  };

  expectNextMatch("A");
  expectNextMatch("B");
  expectNextMatch("C");
  expectNextMatch("D");

  expect(matches.next()).toBeUndefined();
});

test("query syntax error", () => {
  // there is no terminating ']' at the end of this query:
  const source = `[TreeNode @b [DelimitedIdentifier]`;

  try {
    Query.parse(source);
    throw new Error("Query.parse() should have thrown");
  } catch (error) {
    expect((error as any).payload).toEqual({
      message: `Parse error:
expected ']' at: 
Alt at: [TreeNode @b [DelimitedIdentifier]
`,
      line: 0,
      column: 34,
    } satisfies QueryError);
  }
});
