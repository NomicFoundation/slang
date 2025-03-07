import assert from "node:assert";
import { TerminalKindExtensions } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { assertUserFileLocation } from "@nomicfoundation/slang/bindings";
import { findTerminalNodeAt } from "../../common/find-terminal-node-at.mjs";

type Usage = {
  file: string;
  line: number;
  column: number;
};

export function findUsages(unit: CompilationUnit, fileId: string, line: number, column: number): Usage[] | string {
  const file = unit.file(fileId);
  if (!file) {
    return `${fileId} not found in compilation unit`;
  }

  const cursor = findTerminalNodeAt(file.createTreeCursor(), line, column);
  if (!cursor) {
    return `${fileId}:${line}:${column} is not a valid text location`;
  }

  assert(cursor.node.isTerminalNode());
  if (!TerminalKindExtensions.isIdentifier(cursor.node.kind)) {
    // location is not a valid identifier
    return `Could not find a valid identifier at ${fileId}:${line}:${column}`;
  }

  const definition = unit.bindingGraph.definitionAt(cursor);
  if (!definition) {
    return `Identifier ${cursor.node.unparse()} is not a definition at ${fileId}:${line}:${column}`;
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
