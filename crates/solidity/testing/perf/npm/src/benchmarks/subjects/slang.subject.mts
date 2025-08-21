import { CompilationBuilder } from "@nomicfoundation/slang/compilation";
import { TerminalKind } from "@nomicfoundation/slang/cst";
import assert from "node:assert";
import { Subject, SolidityProject, Timings } from "../common.mjs";

function createBuilder(project: SolidityProject): CompilationBuilder {
  const builder = CompilationBuilder.create({
    languageVersion: project.compilation.plainVersion(),

    readFile: async (fileId) => {
      return project.fileContents(fileId);
    },

    resolveImport: async (sourceFileId, importPath) => {
      const importLiteral = importPath.node.unparse();
      assert(importLiteral.startsWith('"') || importLiteral.startsWith("'"));
      assert(importLiteral.endsWith('"') || importLiteral.endsWith("'"));

      let importString = importLiteral.slice(1, -1);
      return project.resolveImport(sourceFileId, importString);
    },
  });

  return builder;
}

export class SlangSubject implements Subject {
  public name = "slang";

  async test(project: SolidityProject, mainFile: string): Promise<Timings> {
    const startTime = performance.now();
    const builder = createBuilder(project);

    await builder.addFile(mainFile);

    const unit = builder.build();
    const parsedAllFilesTime = performance.now();

    // Validation: there shouldn't be any parsing errors, but if there are, let's print them nicely
    const errors = unit.files().flatMap((f) => f.errors().map((e) => [f.id, e.message, e.textRange]));
    assert.deepStrictEqual(errors, []);

    // first access constructs the graph
    assert(typeof unit.bindingGraph.definitionAt == "function");
    const builtGraphTime = performance.now();

    let mainFileTime = 0;

    unit.files().forEach((file) => {
      const fileResolveTimeStart = performance.now();
      let cursor = file.createTreeCursor();
      let emptyDefList = [];
      let neitherDefNorRefs = new Array<string>();

      while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
        const definition = unit.bindingGraph.definitionAt(cursor);
        const reference = unit.bindingGraph.referenceAt(cursor);

        if (reference && reference.definitions().length == 0) {
          emptyDefList.push(file.id, cursor.node.unparse(), cursor.textRange);
        }

        if (!(definition || reference)) {
          neitherDefNorRefs.push(cursor.node.unparse());
        }
      }

      if (file.id == mainFile) {
        mainFileTime = performance.now() - fileResolveTimeStart;
      }
      assert.deepStrictEqual(neitherDefNorRefs, []);
      assert.deepStrictEqual(emptyDefList, []);
    });

    const resolutionTime = performance.now() - builtGraphTime;

    return new Map([
      ["slang_parse_all_files_duration", parsedAllFilesTime - startTime],
      ["slang_init_bindings_graph_duration", builtGraphTime - parsedAllFilesTime],
      ["slang_resolve_main_file_duration", mainFileTime],
      ["slang_resolve_all_bindings_duration", resolutionTime],
      ["slang_total", performance.now() - startTime],
    ]);
  }
}
