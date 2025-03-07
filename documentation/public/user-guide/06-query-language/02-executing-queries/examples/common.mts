import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { Query, QueryMatchIterator } from "@nomicfoundation/slang/cst";

export function executeQueries(soliditySource: string, queries: Query[]): QueryMatchIterator {
  const parser = Parser.create("0.8.28");

  const parseOutput = parser.parseFileContents(soliditySource);
  assert(parseOutput.isValid());

  return parseOutput.createTreeCursor().query(queries);
}
