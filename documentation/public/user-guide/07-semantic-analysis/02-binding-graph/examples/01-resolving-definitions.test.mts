import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { findDefinitionsInFile } from "./find-definitions.mjs";
import { NonterminalKind } from "@nomicfoundation/slang/cst";

test("find definitions in file", async () => {
  const unit = await buildCompilationUnit();
  const definitions = findDefinitionsInFile(unit, "contract.sol");

  const found = [];
  for (const definition of definitions) {
    const nameLocation = definition.nameLocation.asUserFileLocation()!;
    const name = nameLocation.cursor.node.unparse();

    const definiensLocation = definition.definiensLocation.asUserFileLocation()!;
    const kind = definiensLocation.cursor.node.kind;

    found.push({ name, kind });
  }

  assert.strictEqual(found.length, 3);
  assert.deepEqual(found, [
    { name: "Log", kind: NonterminalKind.ImportDeconstructionSymbol },
    { name: "MyContract", kind: NonterminalKind.ContractDefinition },
    { name: "test", kind: NonterminalKind.FunctionDefinition },
  ]);
});
