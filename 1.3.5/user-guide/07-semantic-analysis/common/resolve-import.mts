import { Cursor } from "@nomicfoundation/slang/cst";

export async function resolveImport(_sourceFileId: string, importPath: Cursor) {
  // cursor points to the import string literal:
  const importLiteral = importPath.node.unparse();

  // remove surrounding quotes:
  const importString = importLiteral.replace(/^["']/, "").replace(/["']$/, "");

  return importString;
}
