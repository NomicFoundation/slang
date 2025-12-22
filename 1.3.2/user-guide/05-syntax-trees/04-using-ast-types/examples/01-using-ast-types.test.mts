import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { NonterminalKind } from "@nomicfoundation/slang/cst";
import { FunctionDefinition } from "@nomicfoundation/slang/ast";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

test("using the ast", async () => {
  const source = `
    function add(uint256 a, uint256 b) public pure returns (uint256) {
      return a + b;
    }
  `;

  const parser = Parser.create(LanguageFacts.latestVersion());

  const parseOutput = parser.parseNonterminal(NonterminalKind.FunctionDefinition, source);
  assert(parseOutput.isValid());

  const func = new FunctionDefinition(parseOutput.tree);
  assert.strictEqual(func.name.variant.unparse(), "add");

  const parameters = func.parameters.parameters.items.map((parameter) => parameter.cst.unparse().trim());
  assert.deepEqual(parameters, ["uint256 a", "uint256 b"]);

  const attributes = func.attributes.items.map((attribute) => attribute.cst.unparse().trim());
  assert.deepEqual(attributes, ["public", "pure"]);
});
