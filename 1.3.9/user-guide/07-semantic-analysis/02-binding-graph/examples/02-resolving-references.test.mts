import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { findReferencesInFile } from "./find-references.mjs";
import { assertUserFileLocation } from "@nomicfoundation/slang/bindings";

test("find references in file", async () => {
  const unit = await buildCompilationUnit();
  const references = findReferencesInFile(unit, "contract.sol");

  const found = [];
  for (const reference of references) {
    assertUserFileLocation(reference.location);
    const name = reference.location.cursor.node.unparse();
    const line = reference.location.cursor.textRange.start.line;

    found.push({ name, line });
  }

  assert.strictEqual(found.length, 4);
  assert.deepEqual(found, [
    { name: "Log", line: 1 },
    { name: "Log", line: 5 },
    { name: "msg", line: 5 },
    { name: "sender", line: 5 },
  ]);
});
