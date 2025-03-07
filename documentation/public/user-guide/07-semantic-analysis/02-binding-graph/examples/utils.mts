import assert from "node:assert";
import { BindingLocation } from "@nomicfoundation/slang/bindings";
import { Cursor, NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";

export function assertUserFileLocation(
  location: BindingLocation,
  fileId: string,
  kind: TerminalKind | NonterminalKind,
  line: number,
) {
  assert(location.isUserFileLocation());

  assert.strictEqual(location.fileId, fileId);
  assert.strictEqual(location.cursor.node.kind, kind);
  assert.strictEqual(location.cursor.textRange.start.line, line);
}

export function findNextIdentifier(cursor: Cursor, name: string): Cursor | null {
  // use the low-level CST cursor API
  while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
    if (cursor.node.unparse() === name) {
      return cursor;
    }
  }
  return null;
}
