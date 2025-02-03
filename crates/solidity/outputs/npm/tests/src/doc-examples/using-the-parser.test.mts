import { readRepoFile } from "../utils/files.mjs";

// --8<-- [start:imports]
import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import {
  assertIsNonterminalNode,
  assertIsTerminalNode,
  NonterminalKind,
  TerminalKind,
} from "@nomicfoundation/slang/cst";
// --8<-- [end:imports]

test("using the parser", async () => {
  const source = await readRepoFile("documentation/public/user-guide/inputs/using-the-parser.sol");

  // --8<-- [start:parse-input]
  const parser = Parser.create("0.8.0");

  const parseOutput = parser.parseNonterminal(NonterminalKind.ContractDefinition, source);
  // --8<-- [end:parse-input]

  // --8<-- [start:print-errors]
  for (const error of parseOutput.errors()) {
    console.error(`Error at byte offset ${error.textRange.start.utf8}: ${error.message}`);
  }
  // --8<-- [end:print-errors]

  // --8<-- [start:assert-is-valid]
  assert(parseOutput.isValid());
  // --8<-- [end:assert-is-valid]

  // --8<-- [start:inspect-tree]
  const contract = parseOutput.tree;
  assertIsNonterminalNode(contract, NonterminalKind.ContractDefinition);

  const contractChildren = contract.children();
  assert.equal(contractChildren.length, 7);

  const [contractKeyword, firstSpace, contractName, secondSpace, openBrace, members, closeBrace] = contractChildren;

  assertIsTerminalNode(contractKeyword!.node, TerminalKind.ContractKeyword, "contract");
  assertIsTerminalNode(firstSpace!.node, TerminalKind.Whitespace, " ");
  assertIsTerminalNode(contractName!.node, TerminalKind.Identifier, "Foo");
  assertIsTerminalNode(secondSpace!.node, TerminalKind.Whitespace, " ");
  assertIsTerminalNode(openBrace!.node, TerminalKind.OpenBrace, "{");
  assertIsNonterminalNode(members!.node, NonterminalKind.ContractMembers);
  assertIsTerminalNode(closeBrace!.node, TerminalKind.CloseBrace, "}");
  // --8<-- [end:inspect-tree]

  // --8<-- [start:unparse-node]
  const contractSource = contract.unparse();
  assert.equal(contractSource, "contract Foo {}");
  // --8<-- [end:unparse-node]
});
