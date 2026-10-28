import assert from "node:assert";
import { assertNonterminalNode, NonterminalKind, Query } from "@nomicfoundation/slang/cst";
import { Parser } from "@nomicfoundation/slang/parser";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("query-syntax-1", () => {
  const query = `
    // --8<-- [start:query-syntax-1]
    [MultiplicativeExpression
      [Expression]
      [Asterisk]
      [Expression]
    ]
    // --8<-- [end:query-syntax-1]
  `;

  const matches = extractMatches(query, NonterminalKind.MultiplicativeExpression, "1*2");

  assert.strictEqual(matches.length, 1);
  assertNonterminalNode(matches[0].root.node, NonterminalKind.MultiplicativeExpression, "1*2");
});

test("query-syntax-2", () => {
  const query = `
    // --8<-- [start:query-syntax-2]
    [MultiplicativeExpression
      left_operand: [Expression]
      [Asterisk]
      right_operand: [Expression]
    ]
    // --8<-- [end:query-syntax-2]
  `;

  const matches = extractMatches(query, NonterminalKind.MultiplicativeExpression, "1*2");

  assert.strictEqual(matches.length, 1);
  assertNonterminalNode(matches[0].root.node, NonterminalKind.MultiplicativeExpression, "1*2");
});

test("query-syntax-3", () => {
  const query = `
    // --8<-- [start:query-syntax-3]
    [MultiplicativeExpression
      left_operand: [_]
      operator: ["*"]
      right_operand: [_]
    ]
    // --8<-- [end:query-syntax-3]
  `;

  const matches = extractMatches(query, NonterminalKind.MultiplicativeExpression, "1*2");

  assert.strictEqual(matches.length, 1);
  assertNonterminalNode(matches[0].root.node, NonterminalKind.MultiplicativeExpression, "1*2");
});

test("query-syntax-4", () => {
  const query = `
    // --8<-- [start:query-syntax-4]
    [MultiplicativeExpression
      left_operand: [_]
      [_]
    ]
    // --8<-- [end:query-syntax-4]
  `;

  const matches = extractMatches(query, NonterminalKind.MultiplicativeExpression, "1*2");

  assert.strictEqual(matches.length, 2);
  assertNonterminalNode(matches[0].root.node, NonterminalKind.MultiplicativeExpression, "1*2");
  assertNonterminalNode(matches[1].root.node, NonterminalKind.MultiplicativeExpression, "1*2");
});

test("query-syntax-5", () => {
  const query = `
    // --8<-- [start:query-syntax-5]
    [MultiplicativeExpression
      [Expression
        [StringExpression]
      ]
    ]
    // --8<-- [end:query-syntax-5]
  `;

  {
    const matches = extractMatches(query, NonterminalKind.MultiplicativeExpression, "1 * 'abc'");

    assert.strictEqual(matches.length, 1);
    assertNonterminalNode(matches[0].root.node, NonterminalKind.MultiplicativeExpression, "1 * 'abc'");
  }

  {
    const matches = extractMatches(query, NonterminalKind.MultiplicativeExpression, "'abc' * 1");

    assert.strictEqual(matches.length, 1);
    assertNonterminalNode(matches[0].root.node, NonterminalKind.MultiplicativeExpression, "'abc' * 1");
  }
});

test("capturing-nodes-1", () => {
  const query = `
    // --8<-- [start:capturing-nodes-1]
    [StructDefinition
      @struct_name name: [Identifier]
    ]
    // --8<-- [end:capturing-nodes-1]
  `;

  const matches = extractMatches(query, NonterminalKind.StructDefinition, "struct Foo {}");

  assert.strictEqual(matches.length, 1);
  assertNonterminalNode(matches[0].root.node, NonterminalKind.StructDefinition, "struct Foo {}");
  assert.strictEqual(matches[0].captures["struct_name"].length, 1);
});

test("capturing-nodes-2", () => {
  const query = `
    // --8<-- [start:capturing-nodes-2]
    [ContractDefinition
      @contract_name name: [Identifier]
      members: [ContractMembers
        [ContractMember
          [EventDefinition
            @event_name name: [Identifier]
          ]
        ]
      ]
    ]
    // --8<-- [end:capturing-nodes-2]
  `;

  const matches = extractMatches(query, NonterminalKind.ContractDefinition, "contract A { event A(); }");

  assert.strictEqual(matches.length, 1);
  assertNonterminalNode(matches[0].root.node, NonterminalKind.ContractDefinition, "contract A { event A(); }");
  assert.strictEqual(matches[0].captures["contract_name"].length, 1);
  assert.strictEqual(matches[0].captures["event_name"].length, 1);
});

test("quantification-1", () => {
  const query = `
    // --8<-- [start:quantification-1]
    [SourceUnit
      members: [
        _
        ([_ @import [ImportDirective]])+
      ]
    ]
    // --8<-- [end:quantification-1]
  `;

  const matches = extractMatches(
    query,
    NonterminalKind.SourceUnit,
    `
      import 'one.sol';
      import 'two.sol';
      
      contract Test {}
    `,
  );

  assert.strictEqual(matches.length, 3);

  assert.strictEqual(matches[0].captures["import"].length, 1);
  assert.strictEqual(matches[0].captures["import"][0].node.unparse().trim(), "import 'one.sol';");

  assert.strictEqual(matches[1].captures["import"].length, 2);
  assert.strictEqual(matches[1].captures["import"][0].node.unparse().trim(), "import 'one.sol';");
  assert.strictEqual(matches[1].captures["import"][1].node.unparse().trim(), "import 'two.sol';");

  assert.strictEqual(matches[2].captures["import"].length, 1);
  assert.strictEqual(matches[2].captures["import"][0].node.unparse().trim(), "import 'two.sol';");
});

test("quantification-2", () => {
  const query = `
    // --8<-- [start:quantification-2]
    [StructDefinition
      @name name: [_]
      members: [
        _
        ([_ @member [Identifier]])+
      ]
    ]
    // --8<-- [end:quantification-2]
  `;

  const matches = extractMatches(
    query,
    NonterminalKind.SourceUnit,
    `
      struct Test {
        int x;
        int y;
      }
    `,
  );

  assert.strictEqual(matches.length, 3);

  assert.strictEqual(matches[0].captures["name"].length, 1);
  assert.strictEqual(matches[0].captures["member"].length, 1);
  assert.strictEqual(matches[0].captures["member"][0].node.unparse(), "x");

  assert.strictEqual(matches[1].captures["name"].length, 1);
  assert.strictEqual(matches[1].captures["member"].length, 2);
  assert.strictEqual(matches[1].captures["member"][0].node.unparse(), "x");
  assert.strictEqual(matches[1].captures["member"][1].node.unparse(), "y");

  assert.strictEqual(matches[2].captures["name"].length, 1);
  assert.strictEqual(matches[2].captures["member"].length, 1);
  assert.strictEqual(matches[2].captures["member"][0].node.unparse(), "y");
});

test("quantification-3", () => {
  const query = `
    // --8<-- [start:quantification-3]
    [FunctionCallExpression
      arguments: [ArgumentsDeclaration
        variant: [PositionalArgumentsDeclaration
          arguments: [PositionalArguments
            (@arg [Expression variant: [StringExpression]])?
          ]
        ]
      ]
    ]
    // --8<-- [end:quantification-3]
  `;

  const matches = extractMatches(query, NonterminalKind.FunctionCallExpression, `call(1, 'abc', 3)`);

  assert.strictEqual(matches.length, 2);
  assert.strictEqual(matches[0].captures["arg"].length, 1);
  assert.strictEqual(matches[1].captures["arg"].length, 0);
});

test("alternations-1", () => {
  const query = `
    // --8<-- [start:alternations-1]
    [FunctionCallExpression
      operand: [Expression
        (
            @function variant: [Identifier]
          | @method variant: [MemberAccessExpression]
        )
      ]
    ]
    // --8<-- [end:alternations-1]
  `;

  {
    const matches = extractMatches(query, NonterminalKind.FunctionCallExpression, "call(1)");

    assert.strictEqual(matches.length, 1);
    assert.strictEqual(matches[0].captures["function"].length, 1);
    assert.strictEqual(matches[0].captures["method"].length, 0);
  }

  {
    const matches = extractMatches(query, NonterminalKind.FunctionCallExpression, "a.call(1)");

    assert.strictEqual(matches.length, 1);
    assert.strictEqual(matches[0].captures["function"].length, 0);
    assert.strictEqual(matches[0].captures["method"].length, 1);
  }
});

test("alternations-2", () => {
  const query = `
    // --8<-- [start:alternations-2]
    @keyword (
        ["break"]
      | ["delete"]
      | ["else"]
      | ["for"]
      | ["function"]
      | ["if"]
      | ["return"]
      | ["try"]
      | ["while"]
    )
    // --8<-- [end:alternations-2]
  `;

  const matches = extractMatches(query, NonterminalKind.IfStatement, "if (true) { break; }");

  assert.strictEqual(matches.length, 2);

  assert.strictEqual(matches[0].captures["keyword"].length, 1);
  assert.strictEqual(matches[0].captures["keyword"][0].node.unparse(), "if");

  assert.strictEqual(matches[1].captures["keyword"].length, 1);
  assert.strictEqual(matches[1].captures["keyword"][0].node.unparse(), "break");
});

test("adjacency-1", () => {
  const query = `
    // --8<-- [start:adjacency-1]
    [FunctionDefinition
      [ParametersDeclaration
        [Parameters
          .
          @first_param [Parameter]
        ]
      ]
    ]
    // --8<-- [end:adjacency-1]
  `;

  const matches = extractMatches(query, NonterminalKind.FunctionDefinition, "function test(int x, int y);");

  assert.strictEqual(matches.length, 1);

  assert.strictEqual(matches[0].captures["first_param"].length, 1);
  assert.strictEqual(matches[0].captures["first_param"][0].node.unparse(), "int x");
});

test("adjacency-2", () => {
  const query = `
    // --8<-- [start:adjacency-2]
    [FunctionDefinition
      [ParametersDeclaration
        [Parameters
          @last_param [Parameter]
          .
        ]
      ]
    ]
    // --8<-- [end:adjacency-2]
  `;

  const matches = extractMatches(query, NonterminalKind.FunctionDefinition, "function test(int x, int y);");

  assert.strictEqual(matches.length, 1);

  assert.strictEqual(matches[0].captures["last_param"].length, 1);
  assert.strictEqual(matches[0].captures["last_param"][0].node.unparse(), " int y");
});

test("adjacency-3", () => {
  const query = `
    // --8<-- [start:adjacency-3]
    [Statements
      @stmt1 [Statement]
      .
      @stmt2 [Statement]
    ]
    // --8<-- [end:adjacency-3]
  `;

  const matches = extractMatches(query, NonterminalKind.Statements, "int x; int y; x + y;");

  assert.strictEqual(matches.length, 2);

  assert.strictEqual(matches[0].captures["stmt1"].length, 1);
  assert.strictEqual(matches[0].captures["stmt1"][0].node.unparse(), "int x;");

  assert.strictEqual(matches[0].captures["stmt2"].length, 1);
  assert.strictEqual(matches[0].captures["stmt2"][0].node.unparse(), " int y;");

  assert.strictEqual(matches[1].captures["stmt1"].length, 1);
  assert.strictEqual(matches[1].captures["stmt1"][0].node.unparse(), " int y;");

  assert.strictEqual(matches[1].captures["stmt2"].length, 1);
  assert.strictEqual(matches[1].captures["stmt2"][0].node.unparse(), " x + y;");
});

function extractMatches(queryString: string, kind: NonterminalKind, source: string) {
  const cleanQueryString = queryString
    .split("\n")
    .filter((line) => !line.includes("--8<--"))
    .join("\n");

  const parser = Parser.create(LanguageFacts.latestVersion());

  const parseOutput = parser.parseNonterminal(kind, source);
  assert(parseOutput.isValid());

  const cursor = parseOutput.createTreeCursor();
  const query = Query.create(cleanQueryString);

  const matches = [];
  for (const match of cursor.query([query])) {
    matches.push(match);
  }

  return matches;
}
