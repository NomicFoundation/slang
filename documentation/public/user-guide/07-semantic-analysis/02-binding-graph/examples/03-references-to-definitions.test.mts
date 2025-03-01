import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { findReferencesInFile } from "./find-references.mjs";
import { NonterminalKind } from "@nomicfoundation/slang/cst";

test("navigate from references to definitions", async () => {
  const unit = await buildCompilationUnit();
  const references = findReferencesInFile(unit, "contract.sol");

  const found = [];
  for (const reference of references) {
    const location = reference.location.asUserFileLocation()!;
    const name = location.cursor.node.unparse();
    const line = location.cursor.textRange.start.line;

    // find definitions this reference binds to
    const definitions = [];
    for (const definition of reference.definitions()) {
      if (definition.nameLocation.isUserFileLocation()) {
        // it's a user provided definition
        const nameLocation = definition.nameLocation.asUserFileLocation()!;
        const definiensLocation = definition.definiensLocation.asUserFileLocation()!;
        definitions.push({ file: nameLocation.fileId, kind: definiensLocation.cursor.node.kind });
      } else {
        // it's a built-in
        definitions.push({ file: "BUILT-IN" });
      }
    }

    found.push({ name, line, definitions });
  }

  assert.strictEqual(found.length, 4);
  assert.deepEqual(found, [
    { name: "Log", line: 1, definitions: [{ file: "events.sol", kind: NonterminalKind.EventDefinition }] },
    {
      name: "Log",
      line: 5,
      definitions: [
        { file: "contract.sol", kind: NonterminalKind.ImportDeconstructionSymbol },
        { file: "events.sol", kind: NonterminalKind.EventDefinition },
      ],
    },
    { name: "msg", line: 5, definitions: [{ file: "BUILT-IN" }] },
    { name: "sender", line: 5, definitions: [{ file: "BUILT-IN" }] },
  ]);
});
