import assert from "node:assert";
import { Query, QueryError } from "@nomicfoundation/slang/cst";

test("query errors", () => {
  try {
    Query.create(`
      [NonExistingNode]
    `);
  } catch (error) {
    const queryError = error as QueryError;

    assert.strictEqual(
      queryError.message.trim(),
      `Parse error:\n'NonExistingNode' is not a valid node kind at: NonExistingNode]`,
    );

    assert.deepStrictEqual(queryError.textRange, {
      start: { line: 1, column: 7, utf8: 8, utf16: 8 },
      end: { line: 2, column: 4, utf8: 29, utf16: 29 },
    });
  }
});
