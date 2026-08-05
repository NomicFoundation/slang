import { assertTerminalNode, TerminalKindExtensions } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { findTerminalNodeAt } from "../../common/find-terminal-node-at.mjs";

type Target = {
  file: string;
  line: number;
  column: number;
};

export function jumpToDefinition(unit: CompilationUnit, fileId: string, line: number, column: number): Target {
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

  const reference = unit.bindingGraph.referenceAt(cursor);
  if (!reference) {
    throw new Error(`Identifier ${cursor.node.unparse()} is not a reference at ${fileId}:${line}:${column}`);
  }

  const definitions = reference.definitions();
  if (definitions.length == 0) {
    throw new Error(`${cursor.node.unparse()} is not defined`);
  }

  // we take the first definition arbitrarily
  const location = definitions[0].nameLocation;
  if (!location.isUserFileLocation()) {
    throw new Error(`${cursor.node.unparse()} is a built-in`);
  }

  return {
    file: location.fileId,
    line: location.cursor.textOffset.line,
    column: location.cursor.textOffset.column,
  };
}
