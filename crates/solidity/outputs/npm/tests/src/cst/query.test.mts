import { NonterminalKind, Query, QueryError, assertNonterminalNode } from "@nomicfoundation/slang/cst";
import { Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("simple query", () => {
  const parser = Parser.create(LanguageFacts.latestVersion());
  const treeSource = `function a(){} function b(){} function c(){}`;
  const parseOutput = parser.parseNonterminal(NonterminalKind.ContractMembers, treeSource);
  expect(parseOutput.isValid()).toBeTruthy();

  const querySource = `[FunctionDefinition @id name: [FunctionName]]`;
  const query = Query.create(querySource);

  const matches = parseOutput.createTreeCursor().query([query]);

  const expectNextMatch = (name: string) => {
    const match = matches.next();
    expect(match).toBeDefined();
    expect(Object.keys(match!.captures)).toStrictEqual(["id"]);

    const cursors = match!.captures["id"];
    expect(cursors).toHaveLength(1);

    const node = cursors[0].node;
    assertNonterminalNode(node, NonterminalKind.FunctionName);
    expect(node.unparse().trim()).toBe(name);
  };

  expectNextMatch("a");
  expectNextMatch("b");
  expectNextMatch("c");

  expect(matches.next()).toBeUndefined();
});

test("query syntax error", () => {
  // there is no terminating ']' at the end of this query:
  const source = `[FunctionDefinition @id [FunctionName]`;

  try {
    Query.create(source);
    throw new Error("Query.create() should have thrown");
  } catch (error) {
    expect(error).toEqual({
      message: `Parse error:
expected ']' at: 
Alt at: [FunctionDefinition @id [FunctionName]
`,
      textRange: {
        start: {
          utf8: 38,
          utf16: 38,
          line: 0,
          column: 38,
        },
        end: {
          utf8: 38,
          utf16: 38,
          line: 0,
          column: 38,
        },
      },
    } satisfies QueryError);
  }
});
