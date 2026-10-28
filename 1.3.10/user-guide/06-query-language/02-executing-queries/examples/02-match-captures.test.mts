import assert from "node:assert";
import { executeQueries } from "./common.mjs";
import { Query } from "@nomicfoundation/slang/cst";

test("match captures", () => {
  const matches = executeQueries(
    `
      contract Foo {}
      contract Bar {}
      contract Baz {}
    `,
    [
      Query.create(`
        [ContractDefinition
          @name name: [Identifier]
        ]
      `),
    ],
  );

  const found = [];

  for (const match of matches) {
    const names = match.captures["name"];
    assert.strictEqual(names.length, 1);

    found.push(names[0].node.unparse());
  }

  assert.deepStrictEqual(found, ["Foo", "Bar", "Baz"]);
});
