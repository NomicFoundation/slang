// --8<-- [start:step-1-imports]
import { Language } from "@nomicfoundation/slang/language";
import { FieldName, RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import { NodeType } from "@nomicfoundation/slang/cst";

const source = `
contract Foo {}
contract Bar {}
contract Baz {}
`;

test("construct the cursor", () => {
  // --8<-- [start:create-cursor]
  const language = new Language("0.8.0");
  const parseTree = language.parse(RuleKind.SourceUnit, source);

  const cursor = parseTree.createTreeCursor();
  // --8<-- [end:create-cursor]
  cursor; // Suppress unused warning
});

test("list contract names", () => {
  const language = new Language("0.8.0");
  const parseTree = language.parse(RuleKind.SourceUnit, source);

  const cursor = parseTree.createTreeCursor();
  // --8<-- [start:example-list-contract-names]
  const contractNames: string[] = [];

  while (cursor.goToNextRuleWithKinds([RuleKind.ContractDefinition])) {
    // You have to make sure you return the cursor to original position
    cursor.goToFirstChild();
    cursor.goToNextTokenWithKinds([TokenKind.Identifier]);

    const tokenNode = cursor.node();
    if (tokenNode.type == NodeType.Token) {
      contractNames.push(tokenNode.text);
    } else {
      throw new Error("Identifier should be a token node");
    }

    cursor.goToParent();
  }

  expect(contractNames).toEqual(["Foo", "Bar", "Baz"]);
});

test("use cursor spawn", () => {
  const language = new Language("0.8.0");
  const parseTree = language.parse(RuleKind.SourceUnit, source);

  const cursor = parseTree.createTreeCursor();
  // --8<-- [start:example-using-spawn]
  const contractNames: string[] = [];

  while (cursor.goToNextRuleWithKinds([RuleKind.ContractDefinition])) {
    // `.spawn()` creates a new cursor without the path history, which is cheaper
    // than `.clone()`, which copies the path history.
    // Do this when you don't want to worry about restoring the position of the
    // existing cursor.
    const child_cursor = cursor.spawn();
    // You have to make sure you return the cursor to original position
    child_cursor.goToNextTokenWithKinds([TokenKind.Identifier]);

    const tokenNode = child_cursor.node();
    if (tokenNode.type == NodeType.Token) {
      contractNames.push(tokenNode.text);
    } else {
      throw new Error("Identifier should be a token node");
    }
  }

  expect(contractNames).toEqual(["Foo", "Bar", "Baz"]);
  // --8<-- [end:example-using-spawn]
});

test("access the node pointed to the cursor", () => {
  const source = `contract Foo {
	event Transfer(address indexed src, address indexed dst, uint256 value);
	event Approval(address indexed owner, address indexed spender, uint256 value);
}`;

  const language = new Language("0.8.0");
  const parseTree = language.parse(RuleKind.SourceUnit, source);

  const cursor = parseTree.createTreeCursor();
  // --8<-- [start:example-node-accessors]

  cursor.goToNextRuleWithKind(RuleKind.EventParameters);

  let parameterRanges: Array<{ textValue: string; range: [number, number] }> = [];
  // Only visit children of the first event parameters node
  let cursorChild = cursor.spawn();
  while (cursorChild.goToNextRuleWithKind(RuleKind.EventParameter)) {
    // Collect the text value for each parameter rule node
    let textValue = "";
    const innerCursor = cursorChild.spawn();
    while (innerCursor.goToNextToken()) {
      const node = innerCursor.node();
      if (node.type != NodeType.Token) continue;
      textValue += node.text;
    }

    let range = cursorChild.textRange;
    parameterRanges.push({ textValue, range: [range.start.utf8, range.end.utf8] });
  }

  expect(parameterRanges).toEqual([
    { textValue: "address indexed src", range: [31, 50] },
    { textValue: " address indexed dst", range: [51, 71] },
    { textValue: " uint256 value", range: [72, 86] },
  ]);
  // --8<-- [end:example-node-accessors]
});

test("access the node using its name", () => {
  const language = new Language("0.8.0");
  const parseTree = language.parse(RuleKind.SourceUnit, source);

  const cursor = parseTree.createTreeCursor();
  // --8<-- [start:example-using-cursor-with-names]
  let names: string[] = [];

  while (cursor.goToNextRuleWithKind(RuleKind.ContractDefinition)) {
    const innerCursor = cursor.spawn();
    while (innerCursor.goToNext()) {
      const node = innerCursor.node();
      const nodeName = innerCursor.nodeName;

      if (node.type == NodeType.Token && nodeName == FieldName.Name) {
        names.push(node.text);
      }
    }
  }

  expect(names).toEqual(["Foo", "Bar", "Baz"]);
  // --8<-- [end:example-using-cursor-with-names]
});
