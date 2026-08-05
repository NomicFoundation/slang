import { assertTerminalNode, Cursor, TerminalKindExtensions } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { Definition } from "@nomicfoundation/slang/bindings";

export function collectAllDefinitions(unit: CompilationUnit): Definition[] {
  const allDefinitions = [];
  for (const file of unit.files()) {
    const cursor = file.createTreeCursor();
    allDefinitions.push(...collectDefinitions(unit, cursor));
  }
  return allDefinitions;
}

export function collectDefinitions(unit: CompilationUnit, root: Cursor): Definition[] {
  const cursor = root.spawn();
  const definitions = [];
  while (cursor.goToNextTerminal()) {
    assertTerminalNode(cursor.node);
    if (!TerminalKindExtensions.isIdentifier(cursor.node.kind)) continue;

    const definition = unit.bindingGraph.definitionAt(cursor);
    if (!definition) continue;

    definitions.push(definition);
  }
  return definitions;
}
