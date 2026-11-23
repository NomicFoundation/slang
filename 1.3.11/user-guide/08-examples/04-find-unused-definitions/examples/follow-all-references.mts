import { assertTerminalNode, Cursor, TerminalKindExtensions } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { Definition } from "@nomicfoundation/slang/bindings";

export function followAllReferences(unit: CompilationUnit, root: Cursor): Definition[] {
  const referencedDefinitions = [];
  const cursor = root.spawn();
  while (cursor.goToNextTerminal()) {
    assertTerminalNode(cursor.node);
    if (!TerminalKindExtensions.isIdentifier(cursor.node.kind)) continue;

    const reference = unit.bindingGraph.referenceAt(cursor);
    if (!reference) continue;

    for (const definition of reference.definitions()) {
      if (definition.nameLocation.isBuiltInLocation()) continue;
      referencedDefinitions.push(definition);
    }
  }

  return referencedDefinitions;
}
