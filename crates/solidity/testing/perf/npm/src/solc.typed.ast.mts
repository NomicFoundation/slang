import { CompilationOutput, CompileFailedError, CompilerKind, compileSol } from "solc-typed-ast";
import { Test } from "./common.mjs";

export class SolcTypedAstTest implements Test<void> {
  public name = "solc typed ast";

  async test(languageVersion: string, dir: string, file: string) {
    testFile(languageVersion, dir, file);
  }
}

export async function testFile(languageVersion: string, dir: string, file: string) {
  try {
    await compileSol(file, languageVersion, { basePath: dir }, [CompilationOutput.AST], { kind: CompilerKind.WASM });
  } catch (e) {
    console.error("Errors encountered during compilation:");
    if (e instanceof CompileFailedError) {
      for (const failure of e.failures) {
        for (const error of failure.errors) {
          console.error(error);
        }
      }
    } else {
      console.error(e);
    }
  }

}
