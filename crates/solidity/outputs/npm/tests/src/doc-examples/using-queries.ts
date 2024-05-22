import { repoPath } from "../utils/files";
import path from "node:path";
import assert from "node:assert";
import fs from "node:fs/promises";

import { Language } from "@nomicfoundation/slang/language";
import { NonTerminalKind } from "@nomicfoundation/slang/kinds";
import { Query, QueryResultIterator } from "@nomicfoundation/slang/query";
import { NonTerminalNode, TerminalNode } from "@nomicfoundation/slang/cst";

async function parseDocInputFile(filePath: string) {
  const inputPath = path.join(repoPath("documentation/public/user-guide/inputs"), filePath);
  const source = await fs.readFile(inputPath, "utf8").then((source) => source.trim());
  const language = new Language("0.8.0");
  return language.parse(NonTerminalKind.SourceUnit, source);
}

test("using queries", async () => {
  {
    const parse_output = await parseDocInputFile("using-the-cursor.sol");
    // --8<-- [start:creating-a-query]

    // Any `Cursor` can be used to create a query.
    const cursor = parse_output.createTreeCursor();

    const query = Query.parse("[ContractDefinition]");
    const results: QueryResultIterator = cursor.query([query]);
    // --8<-- [end:creating-a-query]
    results; // Silence the unused warning
  }

  {
    const parseOutput = await parseDocInputFile("using-the-cursor.sol");
    const cursor = parseOutput.createTreeCursor();
    // --8<-- [start:visiting-contracts]
    const found = [];

    const query = Query.parse("@contract [ContractDefinition]");
    const results = cursor.query([query]);

    let result = null;
    while ((result = results.next())) {
      const bindings = result.bindings;
      const cursors = bindings["contract"];

      const cursor = cursors?.[0]?.node() as NonTerminalNode;

      found.push(cursor.unparse().trim());
    }

    assert.deepStrictEqual(found, ["contract Foo {}", "contract Bar {}", "contract Baz {}"]);
    // --8<-- [end:visiting-contracts]
  }

  {
    const parse_output = await parseDocInputFile("multiple-data-types.sol");
    const cursor = parse_output.createTreeCursor();
    // --8<-- [start:multiple-patterns]
    const names = [];

    const struct_def = Query.parse("[StructDefinition ... @name [Identifier] ...]");
    const enum_def = Query.parse("[EnumDefinition ... @name [Identifier] ...]");
    const results = cursor.query([struct_def, enum_def]);

    let result = null;
    while ((result = results.next())) {
      const index = result.queryNumber;
      const bindings = result.bindings;
      const cursors = bindings["name"];

      const cursor = cursors?.[0];

      names.push([index, (cursor?.node() as TerminalNode).text]);
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
    const parse_output = await parseDocInputFile("typed-tuple.sol");
    const cursor = parse_output.createTreeCursor();
    // --8<-- [start:matching-on-label]

    const names = [];

    const query = Query.parse("[TypedTupleMember ... @type [type_name: _] ...]");
    const results = cursor.query([query]);

    let result = null;
    while ((result = results.next())) {
      const bindings = result.bindings;
      const cursors = bindings["type"];

      const cursor = cursors?.[0];

      names.push((cursor?.node() as NonTerminalNode).unparse());
    }

    assert.deepStrictEqual(names, ["uint", " uint16", " uint64", " uint256"]);
    // --8<-- [end:matching-on-label]
  }

  {
    // Matching on node's literal value
    const parse_output = await parseDocInputFile("typed-tuple.sol");
    const cursor = parse_output.createTreeCursor();
    // --8<-- [start:matching-on-literal-value]

    const names = [];

    const query = Query.parse(`[ElementaryType @uint_keyword [variant: "uint"]]`);
    const results = cursor.query([query]);

    let result = null;
    while ((result = results.next())) {
      const bindings = result.bindings;
      const cursors = bindings["uint_keyword"];

      const cursor = cursors?.[0];

      names.push((cursor?.node() as TerminalNode).text);
    }

    assert.deepStrictEqual(names, ["uint"]);
    // --8<-- [end:matching-on-literal-value]
  }

  {
    const parse_output = await parseDocInputFile("tx-origin.sol");
    const cursor = parse_output.createTreeCursor();
    // --8<-- [start:tx-origin]
    const query = Query.parse(`
    @txorigin [MemberAccessExpression
      ...
      [Expression
        ...
        @start ["tx"]
        ...
      ]
      ...
      [MemberAccess
        ...
        ["origin"]
        ...
      ]
    ]`);
    const results = cursor.query([query]);

    const found = [];

    let result = null;
    while ((result = results.next())) {
      const bindings = result.bindings;
      const cursors = bindings["txorigin"];

      const cursor = cursors?.[0];

      found.push([cursor?.textOffset.utf8, (cursor?.node() as NonTerminalNode).unparse()]);
    }

    assert.deepStrictEqual(found, [[375, "tx.origin"]]);
    // --8<-- [end:tx-origin]
  }
});
