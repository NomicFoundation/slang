import { CompilationBuilder, File } from "@nomicfoundation/slang/compilation";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";
import assert from "node:assert";
import { Runner, SolidityProject, Timings } from "../common.mjs";

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

enum BindingsTarget {
  File,
  Project,
}

class SlangRunner implements Runner {
  public name = "slang";
  target: BindingsTarget;

  public constructor(target: BindingsTarget) {
    this.target = target;
  }

  async test(project: SolidityProject, file: string): Promise<Timings> {
    const startTime = performance.now();
    const builder = createBuilder(project);

    await builder.addFile(file);

    const unit = builder.build();
    const mainFile = unit.file(file)!;
    const parsedAllFilesTime = performance.now();

    // Validation: there shouldn't be any parsing errors, but if there are, let's print them nicely
    const errors = unit.files().flatMap((f) => f.errors().map((e) => [f.id, e.message, e.textRange]));
    assert.deepStrictEqual(errors, []);

    // first access constructs the graph
    assert(typeof unit.bindingGraph.definitionAt == "function");
    const builtGraphTime = performance.now();

    let files: File[] = [mainFile];

    if (this.target == BindingsTarget.Project) {
      files = unit.files();
    }

    files.forEach((file) => {
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
          const ancestor = cursor.ancestors().next();
          // Ignore experimental pragma's identifiers
          if (!ancestor || ancestor.kind != NonterminalKind.ExperimentalFeature) {
            neitherDefNorRefs.push(cursor.node.unparse());
          }
        }
      }

      assert.deepStrictEqual(neitherDefNorRefs, []);
      assert.deepStrictEqual(emptyDefList, []);
    });

    const resolutionTime = performance.now() - builtGraphTime;

    return new Map([
      ["slang_parse_all_files_duration", parsedAllFilesTime - startTime],
      ["slang_init_bindings_graph_duration", builtGraphTime - parsedAllFilesTime],
      ["slang_resolve_all_bindings_duration", resolutionTime],
      ["slang_total", performance.now() - startTime],
    ]);
  }
}

export class SlangBindingsFileRunner extends SlangRunner {
  public constructor() {
    super(BindingsTarget.File);
  }
}

export class SlangBindingsProjectRunner extends SlangRunner {
  public constructor() {
    super(BindingsTarget.Project);
  }
}
