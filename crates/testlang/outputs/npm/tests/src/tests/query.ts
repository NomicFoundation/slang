import { NonterminalKind, TerminalKind } from "@slang-private/slang-testlang/kinds";
import { Language } from "@slang-private/slang-testlang/language";
import { Query } from "@slang-private/slang-testlang/query";
import { expectTerminal } from "../utils/cst-helpers";

test("simple query", () => {
  const language = new Language("1.0.0");
  const tree_source = `tree [A [B C] D];`;
  const parse_output = language.parse(NonterminalKind.Tree, tree_source);

  const query_source = `[TreeNodeChild @id [DelimitedIdentifier]]`;
  const query = Query.parse(query_source);

  const matches = parse_output.createTreeCursor().query([query]);

  const expectNextMatch = (name: string) => {
    let match = matches.next();
    expect(match).not.toBeNull();
    expect(Object.keys(match!.captures)).toStrictEqual(["id"]);
    let cursors = match!.captures["id"]!;
    expect(cursors.length).toEqual(1);
    expectTerminal(cursors[0]!.node(), TerminalKind.DelimitedIdentifier, name);
  };

  expectNextMatch("A");
  expectNextMatch("B");
  expectNextMatch("C");
  expectNextMatch("D");

  expect(matches.next()).toBeNull();
});

test("parser error", () => {
  const source = `[TreeNode @b [DelimitedIdentifier]`;
  // The exact error message is not important, just that it throws.
  expect(() => Query.parse(source)).toThrow();
});
