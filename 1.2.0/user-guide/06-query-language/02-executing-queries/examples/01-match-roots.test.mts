import assert from "node:assert";
import { executeQueries } from "./common.mjs";
import { Query } from "@nomicfoundation/slang/cst";

test("match roots", () => {
  const matches = executeQueries(
    `
      contract Foo {}
      contract Bar {}
      contract Baz {}
    `,
    [
      Query.create(`
        [ContractDefinition]
      `),
    ],
  );

  const found = [];

  for (const match of matches) {
    found.push(match.root.node.unparse().trim());
  }

  assert.deepStrictEqual(found, ["contract Foo {}", "contract Bar {}", "contract Baz {}"]);
});
