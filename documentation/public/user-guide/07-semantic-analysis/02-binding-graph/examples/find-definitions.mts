import assert from "node:assert";
import { Query } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { assertUserFileLocation, Definition } from "@nomicfoundation/slang/bindings";

export function findDefinitionsInFile(unit: CompilationUnit, fileId: string): Definition[] {
  const file = unit.file(fileId);
  assert(file);

  // query the file's CST tree
  const cursor = file.createTreeCursor();
  const query = Query.create("[Identifier]");
  const matches = cursor.query([query]);

  const definitions = [];
  for (const match of matches) {
    // attempt to resolve a definition
    const definition = unit.bindingGraph.definitionAt(match.root);

    if (definition) {
      // name should be located in the file we queried
      assertUserFileLocation(definition.nameLocation);
      assert.strictEqual(definition.nameLocation.fileId, fileId);

      // definiens should too be located in the file we queried
      assertUserFileLocation(definition.definiensLocation);
      assert.strictEqual(definition.definiensLocation.fileId, fileId);

      definitions.push(definition);
    }
  }

  return definitions;
}
