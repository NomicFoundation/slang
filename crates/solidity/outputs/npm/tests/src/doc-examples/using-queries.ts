import { repoPath } from "../utils/files";
import path from "node:path";
import assert from "node:assert";
import fs from "node:fs/promises";

import { Language } from "@nomicfoundation/slang/language";
import { RuleKind } from "@nomicfoundation/slang/kinds";
import { Query } from "@nomicfoundation/slang/query";
import { RuleNode, TokenNode } from "@nomicfoundation/slang/cst";
import { query } from "@nomicfoundation/slang/generated";

async function parseDocInputFile(filePath: string) {
  const inputPath = path.join(repoPath("documentation/public/user-guide/inputs"), filePath);
  const source = await fs.readFile(inputPath, "utf8").then((source) => source.trim());
  const language = new Language("0.8.0");
  return language.parse(RuleKind.SourceUnit, source);
}

test("using queries", async () => {
  {
    let parse_output = await parseDocInputFile("using-the-cursor.sol");
    // --8<-- [start:creating-a-query]

    // Any `Cursor` can be used to create a query.
    let cursor = parse_output.createTreeCursor();

    let query = Query.parse("[ContractDefinition]");
    let results: query.QueryResultIterator = cursor.query([query]);
    // --8<-- [end:creating-a-query]
    results; // Silence the unused warning
  }

  {
    let parseOutput = await parseDocInputFile("using-the-cursor.sol");
    let cursor = parseOutput.createTreeCursor();
    // --8<-- [start:listing-contract-names]
    let names = [];

    let query = Query.parse("[ContractDefinition ... @name [Identifier] ...]");
    let results = cursor.query([query]);

    let result = null;
    while ((result = results.next())) {
      let bindings = result.bindings;
      let cursors = bindings["name"];

      let cursor = cursors?.[0];

      names.push((cursor?.node() as TokenNode).text);
    }

    assert.deepStrictEqual(names, ["Foo", "Bar", "Baz"]);
    // --8<-- [end:listing-contract-names]
  }

  {
    let parse_output = await parseDocInputFile("multiple-data-types.sol");
    let cursor = parse_output.createTreeCursor();
    // --8<-- [start:multiple-patterns]
    let names = [];

    let struct_def = Query.parse("[StructDefinition ... @name [Identifier] ...]");
    let enum_def = Query.parse("[EnumDefinition ... @name [Identifier] ...]");
    let results = cursor.query([struct_def, enum_def]);

    let result = null;
    while ((result = results.next())) {
      let index = result.queryNumber;
      let bindings = result.bindings;
      let cursors = bindings["name"];

      let cursor = cursors?.[0];

      names.push([index, (cursor?.node() as TokenNode).text]);
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
    let parse_output = await parseDocInputFile("typed-tuple.sol");
    let cursor = parse_output.createTreeCursor();
    // --8<-- [start:matching-on-label]

    let names = [];

    let query = Query.parse("[TypedTupleMember ... @type [type_name: _] ...]");
    let results = cursor.query([query]);

    let result = null;
    while ((result = results.next())) {
      let bindings = result.bindings;
      let cursors = bindings["type"];

      let cursor = cursors?.[0];

      names.push((cursor?.node() as RuleNode).unparse());
    }

    assert.deepStrictEqual(names, ["uint", " uint16", " uint64", " uint256"]);
    // --8<-- [end:matching-on-label]
  }

  {
    // Matching on node's literal value
    let parse_output = await parseDocInputFile("typed-tuple.sol");
    let cursor = parse_output.createTreeCursor();
    // --8<-- [start:matching-on-literal-value]

    let names = [];

    let query = Query.parse(`[ElementaryType @uint_keyword [variant: "uint"]]`);
    let results = cursor.query([query]);

    let result = null;
    while ((result = results.next())) {
      let bindings = result.bindings;
      let cursors = bindings["uint_keyword"];

      let cursor = cursors?.[0];

      names.push((cursor?.node() as TokenNode).text);
    }

    assert.deepStrictEqual(names, ["uint"]);
    // --8<-- [end:matching-on-literal-value]
  }

  {
    let parse_output = await parseDocInputFile("tx-origin.sol");
    let cursor = parse_output.createTreeCursor();
    // --8<-- [start:tx-origin]
    let query = Query.parse(`
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
    let results = cursor.query([query]);

    let found = [];

    let result = null;
    while ((result = results.next())) {
      let bindings = result.bindings;
      let cursors = bindings["txorigin"];

      let cursor = cursors?.[0];

      found.push([cursor?.textOffset.utf8, (cursor?.node() as RuleNode).unparse()]);
    }

    assert.deepStrictEqual(found, [[375, "tx.origin"]]);
    // --8<-- [end:tx-origin]
  }
});
