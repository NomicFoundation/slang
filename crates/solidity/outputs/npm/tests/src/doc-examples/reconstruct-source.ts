// --8<-- [start:intro]
import { Language } from "@nomicfoundation/slang/language";
import { RuleKind } from "@nomicfoundation/slang/kinds";
import { Cursor } from "@nomicfoundation/slang/cursor";

const source = "int256 constant z = 1 + 2;";
const language = new Language("0.8.11");

let parseOutput = language.parse(RuleKind.SourceUnit, source);
// --8<-- [end:intro]

// --8<-- [start:step-1]
import * as path from "node:path";
import * as fs from "node:fs";

const data = fs.readFileSync(path.join(__dirname, "reconstruct-source.sol"), "utf8");

parseOutput = language.parse(RuleKind.SourceUnit, data);
let cursor: Cursor = parseOutput.createTreeCursor();
// --8<-- [end:step-1]
// --8<-- [start:step-2]
import { TokenNode } from "@nomicfoundation/slang/cst";

let output = "";
while (cursor.goToNext()) {
  let node = cursor.node();
  if (node instanceof TokenNode) {
    output += node.text;
  }
}

// --8<-- [end:step-2]
// We wrap this in a Jest test so that we can verify that the output is correct
test("reconstruct source", () => {
  // --8<-- [start:step-2-assertion]
  // Jest-style assertion for clarity
  expect(output).toEqual("pragma solidity ^0.8.0;\n");
  // --8<-- [end:step-2-assertion]
});
