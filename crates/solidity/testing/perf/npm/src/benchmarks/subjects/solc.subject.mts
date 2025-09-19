import assert from "assert";
import { promisify } from "node:util";
import * as solc from "solc";
import { Subject, SolidityProject, Timings } from "../common.mjs";

export class SolcSubject implements Subject {
  public name = "solc";

  async test(project: SolidityProject, file: string): Promise<Timings> {
    const loadRemoteVersion: (version: string) => Promise<{ compile: (input: string, options: any) => string }> =
      promisify(solc.default.loadRemoteVersion);

    return await loadRemoteVersion("v" + project.compilation.compilerVersion)
      .then((solcSnapshot) => {
        const start = performance.now();
        const options = JSON.stringify({
          language: "Solidity",
          sources: {
            [file]: {
              urls: [file],
            },
          },
          settings: {
            remappings: project.compilation.compilerSettings.remappings,
            outputSelection: {
              "*": {
                "": ["ast"],
              },
            },
          },
        });

        const parsing_result = JSON.parse(solcSnapshot.compile(options, { import: findImports(project) }));
        assert(parsing_result["sources"] != undefined);
        if (parsing_result["errors"] && !parsing_result["errors"].every((value: any) => value["type"] == "Warning")) {
          console.log(parsing_result["errors"]);
          assert(false);
        }
        return new Map([["solc_build_ast_duration", performance.now() - start]]);
      })
      .catch((err) => {
        console.error(`Can't process version ${project.compilation.compilerVersion}`);
        console.error(err);
        process.exit(-1);
      });
  }
}

function findImports(project: SolidityProject): (file: string) => { contents: string } {
  return (file: string) => {
    return { contents: project.fileContents(file) };
  };
}
