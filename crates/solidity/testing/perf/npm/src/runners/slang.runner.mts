import { CompilationBuilder, File } from "@nomicfoundation/slang/compilation";
import { TerminalKind } from "@nomicfoundation/slang/cst";
import assert from "node:assert";
import { Runner, SolidityProject, Timing } from "../common.mjs";

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

  async test(project: SolidityProject, file: string): Promise<Timing[]> {
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
      let neitherDefNorRefSet = new Set<string>();

      while (cursor.goToNextTerminalWithKind(TerminalKind.Identifier)) {
        const definition = unit.bindingGraph.definitionAt(cursor);
        const reference = unit.bindingGraph.referenceAt(cursor);

        if (reference && reference.definitions().length == 0) {
          emptyDefList.push(file.id, cursor.node.unparse());
        }

        if (!(definition || reference)) {
          neitherDefNorRefSet.add(cursor.node.unparse());
        }
      }

      // We don't recognize `ABIEncoderV2` in `pragma experimental`, so let's ignore it
      const allowed = ["ABIEncoderV2", "v2"];
      const neitherDefNorRefList = Array.from(neitherDefNorRefSet);
      assert.deepStrictEqual(
        neitherDefNorRefList.filter((e) => !allowed.includes(e)),
        [],
      );
      assert.deepStrictEqual(emptyDefList, []);
    });

    const resolutionTime = performance.now() - builtGraphTime;

    const timings = [
      new Timing("slang_parse_all_files_duration", parsedAllFilesTime - startTime),
      new Timing("slang_init_bindings_graph_duration", builtGraphTime - parsedAllFilesTime),
      new Timing("slang_resolve_all_bindings_duration", resolutionTime),
    ];
    return timings;
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
