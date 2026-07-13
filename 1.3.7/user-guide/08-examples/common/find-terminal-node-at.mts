import assert from "node:assert";
import { Cursor } from "@nomicfoundation/slang/cst";

export function findTerminalNodeAt(cursor: Cursor, line: number, column: number): Cursor | undefined {
  const range = cursor.textRange;
  if (
    line < range.start.line ||
    (line == range.start.line && column < range.start.column) ||
    line > range.end.line ||
    (line == range.end.line && column > range.end.column)
  ) {
    // initial cursor is outside of the range of the CST tree
    return undefined;
  }

  outer: while (cursor.node.isNonterminalNode()) {
    assert(cursor.goToFirstChild());
    do {
      const childRange = cursor.textRange;
      if (line < childRange.end.line || (line == childRange.end.line && column <= childRange.end.column)) {
        continue outer;
      }
    } while (cursor.goToNextSibling());

    // we should have found a child to recurse into (or the target terminal node)
    throw new Error("should not be reached");
  }
  assert(cursor.node.isTerminalNode());
  return cursor;
}
