import assert from "node:assert";
import { Query } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { assertUserFileLocation, Reference } from "@nomicfoundation/slang/bindings";

export function findReferencesInFile(unit: CompilationUnit, fileId: string): Reference[] {
  const file = unit.file(fileId);
  assert(file);

  // query the file's CST tree
  const cursor = file.createTreeCursor();
  const query = Query.create("[Identifier]");
  const matches = cursor.query([query]);

  const references = [];
  for (const match of matches) {
    // attempt to resolve a reference
    const reference = unit.bindingGraph.referenceAt(match.root);

    if (reference) {
      // should be located in the file we queried
      assertUserFileLocation(reference.location);
      assert.strictEqual(reference.location.fileId, fileId);

      references.push(reference);
    }
  }

  return references;
}
