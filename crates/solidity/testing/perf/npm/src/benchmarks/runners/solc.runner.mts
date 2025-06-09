import assert from "assert";
import { promisify } from "node:util";
import * as solc from "solc";
import { log, Runner, SolidityProject, Timing } from "../common.mjs";

export class SolcRunner implements Runner {
  public name = "solc";

  async test(project: SolidityProject, file: string): Promise<Timing[]> {
    const loadRemoteVersion: (version: string) => Promise<{ compile: (input: string, options: any) => string }> =
      promisify(solc.default.loadRemoteVersion);

    return await loadRemoteVersion("v" + project.compilation.compilerVersion)
      .then((solcSnapshot) => {
        const start = performance.now();
        var folderMeta = `{
        "language": "Solidity",
        "sources": {
          "${file}": {
            "urls": ["${file}"]
          }
        },
        "settings": {
          "outputSelection": {
            "*": {
              "": ["ast"]
            }
          }
        }
      }
      `;
        const parsing_result = JSON.parse(solcSnapshot.compile(folderMeta, { import: findImports(project) }));
        log(parsing_result);
        assert(parsing_result["sources"] != undefined);
        if (parsing_result["errors"] && !parsing_result["errors"].every((value: any) => value["type"] == "Warning")) {
          console.log(parsing_result["errors"]);
          assert(false);
        }
        return [new Timing("solc_build_ast_duration", performance.now() - start)];
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
