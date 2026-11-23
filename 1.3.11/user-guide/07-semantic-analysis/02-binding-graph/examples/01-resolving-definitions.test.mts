import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { findDefinitionsInFile } from "./find-definitions.mjs";
import { NonterminalKind } from "@nomicfoundation/slang/cst";
import { assertUserFileLocation } from "@nomicfoundation/slang/bindings";

test("find definitions in file", async () => {
  const unit = await buildCompilationUnit();
  const definitions = findDefinitionsInFile(unit, "contract.sol");

  const found = [];
  for (const definition of definitions) {
    assertUserFileLocation(definition.nameLocation);
    const name = definition.nameLocation.cursor.node.unparse();

    assertUserFileLocation(definition.definiensLocation);
    const kind = definition.definiensLocation.cursor.node.kind;

    found.push({ name, kind });
  }

  assert.strictEqual(found.length, 3);
  assert.deepEqual(found, [
    { name: "Log", kind: NonterminalKind.ImportDeconstructionSymbol },
    { name: "MyContract", kind: NonterminalKind.ContractDefinition },
    { name: "test", kind: NonterminalKind.FunctionDefinition },
  ]);
});
