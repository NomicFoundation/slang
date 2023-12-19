import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";

test("list contract names", () => {
  // ...or read a file using `fs.readFileSync` (or `fs.readFile`)
  const data = "contract Foo {} contract Bar {} contract Baz {}";

  const language = new Language("0.8.0");
  const parseTree = language.parse(RuleKind.SourceUnit, data);

  let contractNames = [];
  let cursor = parseTree.createTreeCursor();

  while (cursor.goToNextRuleWithKinds([RuleKind.ContractDefinition])) {
    // You have to make sure you return the cursor to original position
    cursor.goToFirstChild();
    cursor.goToNextTokenWithKinds([TokenKind.Identifier]);

    // The currently pointed-to node is the name of the contract
    let tokenNode = cursor.node();
    if (tokenNode.kind !== TokenKind.Identifier) {
      throw new Error("Expected identifier");
    }
    contractNames.push(tokenNode.text);

    cursor.goToParent();
  }

  expect(contractNames).toEqual(["Foo", "Bar", "Baz"]);
});
