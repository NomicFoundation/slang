import assert from "assert";
import fs from "node:fs";
import { promisify } from "node:util";
import path from "path";
import * as solc from "solc";
import { Runner } from "./common.mjs";

export class SolcRunner implements Runner {
  public name = "solc";

  async test(languageVersion: string, dir: string, file: string): Promise<number> {
    const loadRemoteVersion: (version: string) => Promise<{ compile: (input: string, options: any) => string }>
      = promisify(solc.default.loadRemoteVersion);

    return await loadRemoteVersion("v" + languageVersion)
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
        const parsing_result = JSON.parse(solcSnapshot.compile(folderMeta, { import: findImports(dir) }));
        if (process.argv.includes("--verbose")) {
          console.log(parsing_result);
        }
        assert(parsing_result["sources"] != undefined);
        if (parsing_result["errors"] && !parsing_result["errors"].every((value: any) => value["type"] == "Warning")) {
          console.log(parsing_result["errors"]);
          assert(false);
        }
        return performance.now() - start;
      })
      .catch((err) => {
        console.error(`Can't process version ${languageVersion}`);
        console.error(err);
        process.exit(-1);
      });
  }
}

function findImports(folder: string): (file: string) => { contents: string } {
  const repoRoot = process.env["REPO_ROOT"];
  assert(repoRoot);
  return (file: string) => {
    // basic sanitization
    while (file.startsWith("/")) {
      file = file.substring(1);
    }
    const absolutePath = path.resolve(folder, file);
    const source = fs.readFileSync(absolutePath, "utf8");
    return { contents: source };
  };
}
