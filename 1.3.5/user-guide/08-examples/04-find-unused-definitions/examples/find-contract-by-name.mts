import assert from "node:assert";
import { Query } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { Definition } from "@nomicfoundation/slang/bindings";

export function findContractByName(unit: CompilationUnit, contractName: string): Definition {
  for (const file of unit.files()) {
    const cursor = file.createTreeCursor();
    const query = Query.create(`
      [ContractDefinition
        @name name: [Identifier]
      ]
    `);
    const matches = cursor.query([query]);

    for (const match of matches) {
      const nameCursor = match.captures["name"][0];
      const name = nameCursor.node.unparse();
      if (name == contractName) {
        const definition = unit.bindingGraph.definitionAt(nameCursor);
        assert(definition);
        return definition;
      }
    }
  }

  throw new Error(`Could not find contract named ${contractName}`);
}
