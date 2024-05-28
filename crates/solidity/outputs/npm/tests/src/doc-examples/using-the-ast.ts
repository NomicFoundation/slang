import { repoPath } from "../utils/files";
import fs from "node:fs/promises";

// --8<-- [start:imports]
import assert from "node:assert";
import { Language } from "@nomicfoundation/slang/language";
import { NonterminalKind } from "@nomicfoundation/slang/kinds";
import { NonterminalNode } from "@nomicfoundation/slang/cst";
import { FunctionDefinition } from "@nomicfoundation/slang/ast";
// --8<-- [end:imports]

test("using the ast", async () => {
  const inputPath = repoPath("documentation/public/user-guide/inputs/using-the-ast.sol");
  const source = (await fs.readFile(inputPath, "utf8")).trim();

  // --8<-- [start:parse-input]
  const language = new Language("0.8.0");

  const parseOutput = language.parse(NonterminalKind.FunctionDefinition, source);
  // --8<-- [end:parse-input]

  // --8<-- [start:create-node]
  const $function = new FunctionDefinition(parseOutput.tree() as NonterminalNode);

  assert.equal($function.name.variant.text, "add");
  // --8<-- [end:create-node]

  // --8<-- [start:list-parameters]
  const parameters = $function.parameters.parameters.items.map((parameter) => {
    return parameter.name?.text;
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
