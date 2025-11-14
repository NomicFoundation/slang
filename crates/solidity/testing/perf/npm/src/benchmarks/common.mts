import fs from "node:fs";
import path from "node:path";
import { ImportRemap, ImportResolver, SourceMap } from "./import.resolver.mjs";

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export function round2(n: number): number {
  return Math.round(n * 100) / 100;
}

export class SolidityCompilation {
  constructor(
    public compilerVersion: string,
    public fullyQualifiedName: string,
    public compilerSettings: { remappings: string[] },
  ) {}

  public plainVersion(): string {
    return this.compilerVersion.split("+")[0];
  }

  public entrypoint(): string {
    return this.fullyQualifiedName.substring(0, this.fullyQualifiedName.lastIndexOf(":"));
  }
}

/* Interfaces for JSON deserealization */
interface CompilationMetadata {
  compilerVersion: string;
  fullyQualifiedName: string;
  compilerSettings: { remappings: string[] };
}

interface SourceMetadata {
  content: string;
}
interface ContractMetadata {
  sources: Record<string, SourceMetadata>;
  compilation: CompilationMetadata;
}

export class SolidityProject {
  constructor(
    public sources: Map<string, string>,
    public compilation: SolidityCompilation,
    public importResolve: ImportResolver,
  ) {}

  public static build(jsonFile: string): SolidityProject {
    const metadata = JSON.parse(fs.readFileSync(jsonFile, "utf8")) as ContractMetadata;

    // NOTE: we should take other information into account too, in particular, the mappings.
    // This was not necessary for all of the projects we consider, but in the future that might be limiting.
    const compilation = new SolidityCompilation(
      metadata.compilation.compilerVersion,
      metadata.compilation.fullyQualifiedName,
      metadata.compilation.compilerSettings,
    );

    const sources = new Map<string, string>();
    const sourceMap = new Array<SourceMap>();
    for (const key in metadata.sources) {
      sources.set(key, metadata.sources[key].content);
      sourceMap.push(new SourceMap(key, key));
    }

    const importRemaps = metadata.compilation.compilerSettings.remappings.map(
      (remapping) => new ImportRemap(remapping),
    );
    const importResolver = new ImportResolver(importRemaps, sourceMap);
    return new SolidityProject(sources, compilation, importResolver);
  }

  public static mockProject(): SolidityProject {
    return new SolidityProject(
      new Map([["MockContract.sol", "pragma solidity ^0.8.0;\ncontract MockContract { function foo() public {} }"]]),
      new SolidityCompilation("0.8.26+commit.8a97fa7a", "MockContract.sol:MockContract", { remappings: [] }),
      new ImportResolver([], [new SourceMap("MockContract.sol", "MockContract.sol")]),
    );
  }

  public fileContents(file: string): string {
    const content = this.sources.get(file);
    if (content) {
      return content;
    } else {
      throw new Error(`Can't find ${file}`);
    }
  }

  /// Resolves an import of a solidity file. Parameters are:
  /// - `sourceFile`: the relavive path to the file under inspection,
  /// - `importString`: the import string as parsed from the source file.
  /// Returns the relative path of the imported file.
  public resolveImport(sourceFile: string, importString: string): string {
    const resolved = this.importResolve.resolveImport(sourceFile, importString);
    if (!resolved) {
      throw new Error(`Can't resolve import ${importString} in the context of ${sourceFile}`);
    }

    const sourceFileDir = path.dirname(sourceFile);
    const file = path.normalize(path.join(sourceFileDir, resolved));
    if (this.sources.has(file)) {
      return file;
    } else if (this.sources.has(resolved)) {
      return resolved;
    } else {
      throw new Error(`Can't resolve import ${importString} in the context of ${sourceFileDir}`);
    }
  }
}

export type Timings = Map<string, number>;

export interface Subject {
  name: string;

  test(project: SolidityProject, file: string): Promise<Timings>;
}
