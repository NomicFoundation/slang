import { readRepoFile } from "../utils/files.mjs";

// --8<-- [start:imports]
import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { assertIsTerminalNode, NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";
// --8<-- [end:imports]

test("using the cursor", async () => {
  const source = await readRepoFile("documentation/public/user-guide/inputs/using-the-cursor.sol");

  // --8<-- [start:parse-input]
  const parser = Parser.create("0.8.0");

  const parseOutput = parser.parseFile(source);
  // --8<-- [end:parse-input]

  {
    // --8<-- [start:listing-contract-names]
    const contracts = [];

    const cursor = parseOutput.createTreeCursor();

    while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
      assert(cursor.goToFirstChild());
      assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));

      assertIsTerminalNode(cursor.node);
      contracts.push(cursor.node.unparse());

      assert(cursor.goToParent());
    }

    assert.deepStrictEqual(contracts, ["Foo", "Bar", "Baz"]);
    // --8<-- [end:listing-contract-names]
  }

  {
    // --8<-- [start:visiting-sub-tree]
    const contracts = [];

    const cursor = parseOutput.createTreeCursor();

    while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
      const childCursor = cursor.spawn();
      assert(childCursor.goToNextTerminalWithKind(TerminalKind.Identifier));

      assertIsTerminalNode(childCursor.node);
      contracts.push(childCursor.node.unparse());
    }

    assert.deepStrictEqual(contracts, ["Foo", "Bar", "Baz"]);
    // --8<-- [end:visiting-sub-tree]
  }

  {
    // --8<-- [start:accessing-node-positions]
    const contracts = [];

    const cursor = parseOutput.createTreeCursor();

    while (cursor.goToNextNonterminalWithKind(NonterminalKind.ContractDefinition)) {
      const range = cursor.textRange;

      const contractNode = cursor.node;

      contracts.push([
        range.start.line,
        range.start.column,
        range.end.line,
        range.end.column,
        contractNode.unparse().trim(),
      ]);
    }

    assert.deepStrictEqual(contracts, [
      [0, 0, 1, 0, "contract Foo {}"],
      [1, 0, 2, 0, "contract Bar {}"],
      [2, 0, 2, 15, "contract Baz {}"],
    ]);
    // --8<-- [end:accessing-node-positions]
  }
});
