import { repoPath } from "../utils/files";
import fs from "node:fs/promises";

// --8<-- [start:imports]
import assert from "node:assert";
import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import { RuleNode, TokenNode } from "@nomicfoundation/slang/cst";
// --8<-- [end:imports]

test("using the cursor", async () => {
  const inputPath = repoPath("documentation/public/user-guide/inputs/using-the-cursor.sol");
  const source = (await fs.readFile(inputPath, "utf8")).trim();

  // --8<-- [start:parse-input]
  const language = new Language("0.8.0");

  const parseOutput = language.parse(RuleKind.SourceUnit, source);
  // --8<-- [end:parse-input]

  {
    // --8<-- [start:listing-contract-names]
    const contracts = [];

    const cursor = parseOutput.createTreeCursor();

    while (cursor.goToNextRuleWithKind(RuleKind.ContractDefinition)) {
      assert(cursor.goToFirstChild());
      assert(cursor.goToNextTokenWithKind(TokenKind.Identifier));

      const tokenNode = cursor.node();
      assert(tokenNode instanceof TokenNode);
      contracts.push(tokenNode.text);

      assert(cursor.goToParent());
    }

    assert.deepStrictEqual(contracts, ["Foo", "Bar", "Baz"]);
    // --8<-- [end:listing-contract-names]
  }

  {
    // --8<-- [start:visiting-sub-tree]
    const contracts = [];

    const cursor = parseOutput.createTreeCursor();

    while (cursor.goToNextRuleWithKind(RuleKind.ContractDefinition)) {
      const childCursor = cursor.spawn();
      assert(childCursor.goToNextTokenWithKind(TokenKind.Identifier));

      const tokenNode = childCursor.node();
      assert(tokenNode instanceof TokenNode);
      contracts.push(tokenNode.text);
    }

    assert.deepStrictEqual(contracts, ["Foo", "Bar", "Baz"]);
    // --8<-- [end:visiting-sub-tree]
  }

  {
    // --8<-- [start:accessing-node-positions]
    const contracts = [];

    const cursor = parseOutput.createTreeCursor();

    while (cursor.goToNextRuleWithKind(RuleKind.ContractDefinition)) {
      const range = cursor.textRange;

      const contractNode = cursor.node();
      assert(contractNode instanceof RuleNode);

      contracts.push([range.start.char, range.end.char, contractNode.unparse().trim()]);
    }

    assert.deepStrictEqual(contracts, [
      [0, 16, "contract Foo {}"],
      [16, 32, "contract Bar {}"],
      [32, 47, "contract Baz {}"],
    ]);
    // --8<-- [end:accessing-node-positions]
  }
});
