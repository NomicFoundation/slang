import assert from "node:assert";
import { Query } from "@nomicfoundation/slang/cst";
import { executeQueries } from "./common.mjs";

test("multiple queries", () => {
  const matches = executeQueries(
    `
      struct Foo { uint a; }
      enum Bar { A, B }
      struct Baz { uint b; uint c; }
      enum Qux { C, D }
    `,
    [
      Query.create(`
        [StructDefinition
          @name [Identifier]
        ]
      `),
      Query.create(`
        [EnumDefinition
          @name [Identifier]
        ]
      `),
    ],
  );

  const found = [];

  for (const match of matches) {
    const names = match.captures["name"];
    assert.strictEqual(names.length, 1);

    found.push({
      queryIndex: match.queryIndex,
      name: names[0].node.unparse(),
    });
  }

  assert.deepStrictEqual(found, [
    { queryIndex: 0, name: "Foo" },
    { queryIndex: 1, name: "Bar" },
    { queryIndex: 0, name: "Baz" },
    { queryIndex: 1, name: "Qux" },
  ]);
});
