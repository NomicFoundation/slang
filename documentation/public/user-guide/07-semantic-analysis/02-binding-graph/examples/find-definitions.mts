import assert from "node:assert";
import { Query } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { Definition } from "@nomicfoundation/slang/bindings";

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
      const nameLocation = definition.nameLocation;
      assert(nameLocation.isUserFileLocation());
      assert.strictEqual(nameLocation.asUserFileLocation()!.fileId, fileId);

      // definiens should too be located in the file we queried
      const definiensLocation = definition.definiensLocation;
      assert(definiensLocation.isUserFileLocation());
      assert.strictEqual(definiensLocation.asUserFileLocation()!.fileId, fileId);

      definitions.push(definition);
    }
  }
  return definitions;
}
