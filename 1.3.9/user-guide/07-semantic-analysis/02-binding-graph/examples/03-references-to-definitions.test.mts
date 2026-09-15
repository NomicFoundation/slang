import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { findReferencesInFile } from "./find-references.mjs";
import { NonterminalKind } from "@nomicfoundation/slang/cst";
import { assertUserFileLocation } from "@nomicfoundation/slang/bindings";

test("navigate from references to definitions", async () => {
  const unit = await buildCompilationUnit();
  const references = findReferencesInFile(unit, "contract.sol");

  const found = [];
  for (const reference of references) {
    assertUserFileLocation(reference.location);
    const name = reference.location.cursor.node.unparse();
    const line = reference.location.cursor.textRange.start.line;

    // find definitions this reference binds to
    const definitions = [];
    for (const definition of reference.definitions()) {
      if (definition.nameLocation.isUserFileLocation() && definition.definiensLocation.isUserFileLocation()) {
        // it's a user provided definition
        definitions.push({
          file: definition.nameLocation.fileId,
          kind: definition.definiensLocation.cursor.node.kind,
        });
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
