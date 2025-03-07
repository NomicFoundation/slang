import assert from "node:assert";
import { TerminalKindExtensions } from "@nomicfoundation/slang/cst";
import { CompilationUnit } from "@nomicfoundation/slang/compilation";
import { findTerminalNodeAt } from "../../common/find-terminal-node-at.mjs";

type Target = {
  file: string;
  line: number;
  column: number;
};

export function jumpToDefinition(unit: CompilationUnit, fileId: string, line: number, column: number): Target | string {
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

  const reference = unit.bindingGraph.referenceAt(cursor);
  if (!reference) {
    return `Identifier ${cursor.node.unparse()} is not a reference at ${fileId}:${line}:${column}`;
  }

  const definitions = reference.definitions();
  if (definitions.length == 0) {
    return `${cursor.node.unparse()} is not defined`;
  } else {
    // we take the first definition arbitrarily
    const location = definitions[0].nameLocation;
    if (location.isUserFileLocation()) {
      return {
        file: location.fileId,
        line: location.cursor.textOffset.line,
        column: location.cursor.textOffset.column,
      };
    } else {
      return `${cursor.node.unparse()} is a built-in`;
    }
  }
}
