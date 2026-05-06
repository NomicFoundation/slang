import assert from "node:assert";
import { Parser } from "@nomicfoundation/slang/parser";
import { Query, QueryMatchIterator } from "@nomicfoundation/slang/cst";
import { LanguageFacts } from "@nomicfoundation/slang/utils";

export function executeQueries(soliditySource: string, queries: Query[]): QueryMatchIterator {
  const parser = Parser.create(LanguageFacts.latestVersion());

  const parseOutput = parser.parseFileContents(soliditySource);
  assert(parseOutput.isValid());

  return parseOutput.createTreeCursor().query(queries);
}
