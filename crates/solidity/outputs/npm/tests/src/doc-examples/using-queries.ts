import { repoPath } from "../utils/files";
import path from "node:path";
import assert from "node:assert";
import fs from "node:fs/promises";

import { Language } from "@nomicfoundation/slang/language";
import { NonterminalKind } from "@nomicfoundation/slang/kinds";
import { Query, QueryMatchIterator } from "@nomicfoundation/slang/query";
import { NonterminalNode, TerminalNode } from "@nomicfoundation/slang/cst";

async function parseDocInputFile(filePath: string) {
  const inputPath = path.join(repoPath("documentation/public/user-guide/inputs"), filePath);
  const source = await fs.readFile(inputPath, "utf8").then((source) => source.trim());
  const language = new Language("0.8.0");
  return language.parse(NonterminalKind.SourceUnit, source);
}

test("using queries", async () => {
  {
    const parse_output = await parseDocInputFile("using-the-cursor.sol");
    // --8<-- [start:creating-a-query]

    // Any `Cursor` can be used to create a query.
    const cursor = parse_output.createTreeCursor();

    const query = Query.parse("[ContractDefinition]");
    const matches: QueryMatchIterator = cursor.query([query]);
    // --8<-- [end:creating-a-query]
    matches; // Silence the unused warning
  }

  {
    const parseOutput = await parseDocInputFile("using-the-cursor.sol");
    const cursor = parseOutput.createTreeCursor();
    // --8<-- [start:visiting-contracts]
    const found = [];

    const query = Query.parse("@contract [ContractDefinition]");
    const matches = cursor.query([query]);

    let match = null;
    while ((match = matches.next())) {
      const captures = match.captures;
      const cursors = captures["contract"];

      const cursor = cursors?.[0]?.node() as NonterminalNode;

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

    const struct_def = Query.parse("[StructDefinition @name [Identifier]]");
    const enum_def = Query.parse("[EnumDefinition @name [Identifier]]");
    const matches = cursor.query([struct_def, enum_def]);

    let match = null;
    while ((match = matches.next())) {
      const index = match.queryNumber;
      const captures = match.captures;
      const cursors = captures["name"];

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

    const query = Query.parse("[TypedTupleMember @type type_name:[_]]");
    const matches = cursor.query([query]);

    let match = null;
    while ((match = matches.next())) {
      const captures = match.captures;
      const cursors = captures["type"];

      const cursor = cursors?.[0];

      names.push((cursor?.node() as NonterminalNode).unparse());
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

    const query = Query.parse(`[ElementaryType @uint_keyword variant:["uint"]]`);
    const matches = cursor.query([query]);

    let match = null;
    while ((match = matches.next())) {
      const captures = match.captures;
      const cursors = captures["uint_keyword"];

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
      [Expression @start ["tx"]]
      [MemberAccess ["origin"]]
    ]`);
    const matches = cursor.query([query]);

    const found = [];

    let match = null;
    while ((match = matches.next())) {
      const captures = match.captures;
      const cursors = captures["txorigin"];

      const cursor = cursors?.[0];

      found.push([cursor?.textOffset.utf8, (cursor?.node() as NonterminalNode).unparse()]);
    }

    assert.deepStrictEqual(found, [[375, "tx.origin"]]);
    // --8<-- [end:tx-origin]
  }
});
