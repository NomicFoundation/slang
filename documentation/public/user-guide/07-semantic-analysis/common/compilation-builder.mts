import { CompilationBuilder, CompilationUnit } from "@nomicfoundation/slang/compilation";
import { readFile } from "./read-file.mjs";
import { resolveImport } from "./resolve-import.mjs";

export async function buildCompilationUnit(): Promise<CompilationUnit> {
  const builder = CompilationBuilder.create({
    languageVersion: "0.8.22",
    readFile,
    resolveImport,
  });

  await builder.addFile("contract.sol");

  return builder.build();
}
