import { assertTerminalNode, TerminalKindExtensions } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { assertUserFileLocation } from "@nomicfoundation/slang/bindings";
import { findTerminalNodeAt } from "../../common/find-terminal-node-at.mjs";

type Usage = {
  file: string;
  line: number;
  column: number;
};

export function findUsages(unit: CompilationUnit, fileId: string, line: number, column: number): Usage[] {
  const file = unit.file(fileId);
  if (!file) {
    throw new Error(`${fileId} not found in compilation unit`);
  }

  const cursor = findTerminalNodeAt(file.createTreeCursor(), line, column);
  if (!cursor) {
    throw new Error(`${fileId}:${line}:${column} is not a valid text location`);
  }

  assertTerminalNode(cursor.node);
  if (!TerminalKindExtensions.isIdentifier(cursor.node.kind)) {
    // location is not a valid identifier
    throw new Error(`Could not find a valid identifier at ${fileId}:${line}:${column}`);
  }

  const definition = unit.bindingGraph.definitionAt(cursor);
  if (!definition) {
    throw new Error(`Identifier ${cursor.node.unparse()} is not a definition at ${fileId}:${line}:${column}`);
  }

  const references = definition.references();
  const usages = [];
  for (const reference of references) {
    assertUserFileLocation(reference.location);
    usages.push({
      file: reference.location.fileId,
      line: reference.location.cursor.textOffset.line,
      column: reference.location.cursor.textOffset.column,
    });
  }
  return usages;
}
