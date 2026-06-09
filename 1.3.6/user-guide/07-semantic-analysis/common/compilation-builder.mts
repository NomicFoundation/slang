import { CompilationBuilder, CompilationUnit } from "@nomicfoundation/slang/compilation";
import { LanguageFacts } from "@nomicfoundation/slang/utils";
import { readFile } from "./read-file.mjs";
import { resolveImport } from "./resolve-import.mjs";

export async function buildCompilationUnit(): Promise<CompilationUnit> {
  const builder = CompilationBuilder.create({
    languageVersion: LanguageFacts.latestVersion(),
    readFile,
    resolveImport,
  });

  await builder.addFile("contract.sol");

  return builder.build();
}
