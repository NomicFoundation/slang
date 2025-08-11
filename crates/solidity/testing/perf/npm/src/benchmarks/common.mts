import fs from "node:fs";
import path from "node:path";

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
  ) {}

  public plainVersion(): string {
    return this.compilerVersion.split("+")[0];
  }

  public entrypoint(): string {
    return this.fullyQualifiedName.substring(0, this.fullyQualifiedName.lastIndexOf(":"));
  }

  public projectName(): string {
    return this.fullyQualifiedName.substring(this.fullyQualifiedName.lastIndexOf(":") + 1);
  }
}

/* Interfaces for JSON deserealization */
interface CompilationMetadata {
  compilerVersion: string;
  fullyQualifiedName: string;
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
  ) {}

  public static build(jsonFile: string): SolidityProject {
    const metadata = JSON.parse(fs.readFileSync(jsonFile, "utf8")) as ContractMetadata;

    // NOTE: we should take other information into account too, in particular, the mappings.
    // This was not necessary for all of the projects we consider, but in the future that might be limiting.
    let compilation = new SolidityCompilation(
      metadata.compilation.compilerVersion,
      metadata.compilation.fullyQualifiedName,
    );

    let sources = new Map<string, string>();
    for (const key in metadata.sources) {
      sources.set(key, metadata.sources[key].content);
    }

    return new SolidityProject(sources, compilation);
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
    const sourceFileDir = path.dirname(sourceFile);

    const file = path.normalize(path.join(sourceFileDir, importString));
    if (this.sources.has(file)) {
      return file;
    } else if (this.sources.has(importString)) {
      return importString;
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
