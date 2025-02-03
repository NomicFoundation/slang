import { readRepoFile } from "../utils/files.mjs";
import assert from "node:assert";

import { Parser } from "@nomicfoundation/slang/parser";
import { assertIsNonterminalNode, Query, QueryMatchIterator } from "@nomicfoundation/slang/cst";

async function parseDocInputFile(relativePath: string) {
  const source = await readRepoFile("documentation/public/user-guide/inputs", relativePath);
  const parser = Parser.create("0.8.0");
  return parser.parseFileContents(source);
}

test("using queries", async () => {
  {
    const parseOutput = await parseDocInputFile("using-the-cursor.sol");
    // --8<-- [start:creating-a-query]
    // Any `Cursor` can be used to create a query.
    const cursor = parseOutput.createTreeCursor();

    const query = Query.create("[ContractDefinition]");
    const matches: QueryMatchIterator = cursor.query([query]);
    // --8<-- [end:creating-a-query]

    matches; // Silence the unused warning
  }

  {
    const parseOutput = await parseDocInputFile("using-the-cursor.sol");
    const cursor = parseOutput.createTreeCursor();

    // --8<-- [start:visiting-contracts]
    const found = [];

    const query = Query.create("@contract [ContractDefinition]");
    const matches = cursor.query([query]);

    for (const match of matches) {
      const cursor = match.captures["contract"]![0]!;

      assertIsNonterminalNode(cursor.node);
      found.push(cursor.node.unparse().trim());
    }

    assert.deepStrictEqual(found, ["contract Foo {}", "contract Bar {}", "contract Baz {}"]);
    // --8<-- [end:visiting-contracts]
  }

  {
    const parseOutput = await parseDocInputFile("multiple-data-types.sol");
    const cursor = parseOutput.createTreeCursor();

    // --8<-- [start:multiple-patterns]
    const names = [];

    const structDefinition = Query.create("[StructDefinition @name [Identifier]]");
    const enumDefinition = Query.create("[EnumDefinition @name [Identifier]]");
    const matches = cursor.query([structDefinition, enumDefinition]);

    for (const match of matches) {
      const index = match.queryIndex;
      const cursor = match.captures["name"]![0]!;

      names.push([index, cursor.node.unparse()]);
    }

    assert.deepStrictEqual(names, [
      [0, "Foo"],
      [1, "Bar"],
      [0, "Baz"],
      [1, "Qux"],
    ]);
    // --8<-- [end:multiple-patterns]
  }

  {
    const parseOutput = await parseDocInputFile("typed-tuple.sol");
    const cursor = parseOutput.createTreeCursor();

    // --8<-- [start:matching-on-label]
    const names = [];

    const query = Query.create("[TypedTupleMember @type type_name:[_]]");
    const matches = cursor.query([query]);

    for (const match of matches) {
      const cursor = match.captures["type"]![0]!;

      names.push(cursor.node.unparse());
    }

    assert.deepStrictEqual(names, ["uint", " uint16", " uint64", " uint256"]);
    // --8<-- [end:matching-on-label]
  }

  {
    // Matching on node's literal value
    const parseOutput = await parseDocInputFile("typed-tuple.sol");
    const cursor = parseOutput.createTreeCursor();

    // --8<-- [start:matching-on-literal-value]
    const names = [];

    const query = Query.create(`[ElementaryType @uint_keyword variant:["uint"]]`);
    const matches = cursor.query([query]);

    for (const match of matches) {
      const cursor = match.captures["uint_keyword"]![0]!;

      names.push(cursor.node.unparse());
    }

    assert.deepStrictEqual(names, ["uint"]);
    // --8<-- [end:matching-on-literal-value]
  }

  {
    const parseOutput = await parseDocInputFile("tx-origin.sol");
    const cursor = parseOutput.createTreeCursor();

    // --8<-- [start:tx-origin]
    const query = Query.create(`
    @txorigin [MemberAccessExpression
      [Expression @start ["tx"]]
      ["origin"]
    ]`);

    const matches = cursor.query([query]);
    const found = [];

    for (const match of matches) {
      const cursor = match.captures["txorigin"]![0]!;

      found.push([cursor.textOffset.utf8, cursor.node.unparse()]);
    }

    assert.deepStrictEqual(found, [[375, "tx.origin"]]);
    // --8<-- [end:tx-origin]
  }
});
