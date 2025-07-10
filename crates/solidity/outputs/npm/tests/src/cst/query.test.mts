import { NonterminalKind, TerminalKind, Query, assertTerminalNode, QueryError } from "@nomicfoundation/slang/cst";
import { Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("simple query", () => {
  const parser = Parser.create(LanguageFacts.latestVersion());
  const treeSource = `function a(){} function b(){} function c(){}`;
  const parseOutput = parser.parseNonterminal(NonterminalKind.ContractMembers, treeSource);

  const querySource = `[FunctionDefinition @id [Identifier]]`;
  const query = Query.create(querySource);

  const matches = parseOutput.createTreeCursor().query([query]);

  const expectNextMatch = (name: string) => {
    const match = matches.next()!;
    expect(Object.keys(match.captures)).toStrictEqual(["id"]);

    const cursors = match.captures["id"];
    expect(cursors).toHaveLength(1);

    const node = cursors[0].node;
    assertTerminalNode(node, TerminalKind.Identifier, name);
  };

  expectNextMatch("a");
  expectNextMatch("b");
  expectNextMatch("c");

  expect(matches.next()).toBeUndefined();
});

test("query syntax error", () => {
  // there is no terminating ']' at the end of this query:
  const source = `[FunctionDefinition @id [Identifier]`;

  try {
    Query.create(source);
    throw new Error("Query.create() should have thrown");
  } catch (error) {
    expect(error).toEqual({
      message: `Parse error:
expected ']' at: 
Alt at: [FunctionDefinition @id [Identifier]
`,
      textRange: {
        start: {
          utf8: 36,
          utf16: 36,
          line: 0,
          column: 36,
        },
        end: {
          utf8: 36,
          utf16: 36,
          line: 0,
          column: 36,
        },
      },
    } satisfies QueryError);
  }
});
