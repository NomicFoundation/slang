import { Cursor } from "@nomicfoundation/slang/cst";
import { CompilationBuilder, CompilationUnit } from "@nomicfoundation/slang/compilation";

function buildReadFile(virtualFs: Map<string, string>): (fileId: string) => Promise<string> {
  return async (fileId: string) => {
    const contents = virtualFs.get(fileId);
    if (!contents) {
      throw new Error(`${fileId} not found`);
    }
    return contents;
  };
}

async function resolveImport(_sourceFileId: string, importPath: Cursor) {
  const importLiteral = importPath.node.unparse();
  const importString = importLiteral.replace(/^["']/, "").replace(/["']$/, "");
  return importString;
}

export async function buildCompilationUnit(
  virtualFs: Map<string, string>,
  version: string,
  mainFileId: string,
): Promise<CompilationUnit> {
  const builder = CompilationBuilder.create({
    languageVersion: version,
    readFile: buildReadFile(virtualFs),
    resolveImport,
  });

  await builder.addFile(mainFileId);

  return builder.build();
}
