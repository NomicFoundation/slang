import assert from "node:assert";
import { assertTerminalNode, TerminalKindExtensions } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { assertUserFileLocation, Reference } from "@nomicfoundation/slang/bindings";

export function findReferencesInFile(unit: CompilationUnit, fileId: string): Reference[] {
  const file = unit.file(fileId);
  assert(file);

  const references = [];

  // traverse the file's CST tree looking for identifiers
  const cursor = file.createTreeCursor();
  while (cursor.goToNextTerminal()) {
    assertTerminalNode(cursor.node);
    if (!TerminalKindExtensions.isIdentifier(cursor.node.kind)) {
      continue;
    }

    // attempt to resolve a reference
    const reference = unit.bindingGraph.referenceAt(cursor);

    if (reference) {
      // should be located in the file we queried
      assertUserFileLocation(reference.location);
      assert.strictEqual(reference.location.fileId, fileId);

      references.push(reference);
    }
  }

  return references;
}
