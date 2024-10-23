import { readRepoFile } from "../utils/files.mjs";

// --8<-- [start:imports]
import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { NonterminalKind } from "@nomicfoundation/slang/cst";
import { FunctionDefinition } from "@nomicfoundation/slang/ast";
// --8<-- [end:imports]

test("using the ast", async () => {
  const source = await readRepoFile("documentation/public/user-guide/inputs/using-the-ast.sol");

  // --8<-- [start:parse-input]
  const parser = Parser.create("0.8.0");

  const parseOutput = parser.parse(NonterminalKind.FunctionDefinition, source);
  // --8<-- [end:parse-input]

  // --8<-- [start:create-node]
  const $function = new FunctionDefinition(parseOutput.tree.asNonterminalNode()!);

  assert.equal($function.name.variant.unparse(), "add");
  // --8<-- [end:create-node]

  // --8<-- [start:list-parameters]
  const parameters = $function.parameters.parameters.items.map((parameter) => {
    return parameter.name!.unparse();
  });

  assert.deepEqual(parameters, ["a", "b"]);
  // --8<-- [end:list-parameters]

  // --8<-- [start:list-attributes]
  const attributes = $function.attributes.items.map((attribute) => {
    return attribute.cst.unparse().trim();
  });

  assert.deepEqual(attributes, ["public", "pure"]);
  // --8<-- [end:list-attributes]
});
