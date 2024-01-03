// --8<-- [start:step-1-imports]
import * as assert from "node:assert";
import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import { ContractDefinition, SourceUnit, SourceUnitMember, SourceUnitMembers } from "@nomicfoundation/slang/ast";
// --8<-- [end:step-1-imports]

test("list contract names", () => {
  // --8<-- [start:step-2-source]
  const source = `
    contract Foo {}
    contract Bar {}
    contract Baz {}
  `;

  const language = new Language("0.8.0");
  const parseTree = language.parse(RuleKind.SourceUnit, source);
  assert.ok(parseTree.isValid);
  // --8<-- [end:step-2-source]

  {
    // --8<-- [start:step-3-cursor]
    const contractNames = [];
    const cursor = parseTree.createTreeCursor();

    while (cursor.goToNextRuleWithKind(RuleKind.ContractDefinition)) {
      // Descend down to the first child of the contract node:
      cursor.goToFirstChild();

      // Then move to the next identifier (the contract name):
      cursor.goToNextTokenWithKind(TokenKind.Identifier);

      const identifier = cursor.node();
      assert.strictEqual(identifier.kind, TokenKind.Identifier);
      contractNames.push(identifier.text);

      // You have to make sure you return the cursor to original position:
      cursor.goToParent();
    }

    assert.deepEqual(contractNames, ["Foo", "Bar", "Baz"]);
    // --8<-- [end:step-3-cursor]
  }

  {
    // --8<-- [start:step-4-ast]
    const rootNode = parseTree.tree();
    assert.strictEqual(rootNode.kind, RuleKind.SourceUnit);

    const sourceUnit = new SourceUnit(rootNode);
    assert.ok(sourceUnit.members instanceof SourceUnitMembers);

    const contractNames = [];

    for (const member of sourceUnit.members.items) {
      assert.ok(member instanceof SourceUnitMember);

      const contract = member.variant;
      assert.ok(contract instanceof ContractDefinition);

      contractNames.push(contract.name.text);
    }

    assert.deepEqual(contractNames, ["Foo", "Bar", "Baz"]);
    // --8<-- [end:step-4-ast]
  }
});
