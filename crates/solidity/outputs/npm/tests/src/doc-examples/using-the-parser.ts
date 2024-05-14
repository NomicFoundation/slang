import { repoPath } from "../utils/files";
import fs from "node:fs/promises";

// --8<-- [start:imports]
import assert from "node:assert";
import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import { RuleNode } from "@nomicfoundation/slang/cst";
// --8<-- [end:imports]

test("using the parser", async () => {
  const inputPath = repoPath("documentation/public/user-guide/inputs/using-the-parser.sol");
  const source = (await fs.readFile(inputPath, "utf8")).trim();

  // --8<-- [start:parse-input]
  const language = new Language("0.8.0");

  const parseOutput = language.parse(RuleKind.ContractDefinition, source);
  // --8<-- [end:parse-input]

  // --8<-- [start:print-errors]
  for (const error of parseOutput.errors()) {
    let diagnostic = error.toDiagnostic();
    console.error(`Encountered an error: ${diagnostic.message()}`);
  }
  // --8<-- [end:print-errors]

  // --8<-- [start:assert-is-valid]
  assert(parseOutput.isValid);
  // --8<-- [end:assert-is-valid]

  // --8<-- [start:inspect-tree]
  const contract = parseOutput.tree();
  assert(contract instanceof RuleNode);
  assert.equal(contract.kind, RuleKind.ContractDefinition);

  const contractChildren = contract.children();
  assert.equal(contractChildren.length, 7);

  const [contractKeyword, firstSpace, contractName, secondSpace, openBrace, members, closeBrace] = contractChildren;

  assert.equal(contractKeyword?.kind, TokenKind.ContractKeyword);
  assert.equal(firstSpace?.kind, TokenKind.Whitespace);
  assert.equal(contractName?.kind, TokenKind.Identifier);
  assert.equal(secondSpace?.kind, TokenKind.Whitespace);
  assert.equal(openBrace?.kind, TokenKind.OpenBrace);
  assert.equal(members?.kind, RuleKind.ContractMembers);
  assert.equal(closeBrace?.kind, TokenKind.CloseBrace);
  // --8<-- [end:inspect-tree]

  // --8<-- [start:unparse-node]
  const contractSource = contract.unparse();
  assert.equal(contractSource, "contract Foo {}");
  // --8<-- [end:unparse-node]
});
