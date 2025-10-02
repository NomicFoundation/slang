import path from "node:path";
import { URL } from "node:url";

// This file includes the logic to solve imports in solidity.
// It is typescript equivalent of `import_resolver.rs`

export class ImportRemap {
  context: string | undefined;
  prefix: string;
  target: string;

  constructor(remapStr: string) {
    const [context, rest] = remapStr.split(":");
    if (rest === undefined) throw new Error(`${remapStr}: Could not separate context from mapping`);
    const [prefix, target] = rest.split("=");
    if (target === undefined) throw new Error(`${remapStr}: Could not separate prefix and target`);
    this.context = context ? context : undefined;
    this.prefix = prefix;
    this.target = target;
  }

  matches(sourcePath: string, importPath: string): boolean {
    const contextMatches = this.context ? sourcePath.startsWith(this.context) : true;
    return contextMatches && importPath.startsWith(this.prefix);
  }

  matchLen(): number {
    return (this.context?.length ?? 0) + this.prefix.length;
  }

  hasKnownBug(): boolean {
    return this.target.includes("remappings.txt");
  }
}

export class SourceMap {
  sourceId: string;
  virtualPath: string;

  constructor(sourceId: string, virtualPath: string) {
    this.sourceId = sourceId;
    this.virtualPath = virtualPath;
  }

  matchesVirtualPath(virtualPath: string): boolean {
    return this.virtualPath === virtualPath || this.virtualPath.replace(/\/\//g, "/") === virtualPath;
  }

  matchesSourceId(sourceId: string): boolean {
    return this.sourceId === sourceId;
  }
}

export class ImportResolver {
  importRemaps: ImportRemap[];
  sourceMaps: SourceMap[];

  constructor(importRemaps: ImportRemap[], sourceMaps: SourceMap[]) {
    this.importRemaps = importRemaps;
    this.sourceMaps = sourceMaps;
  }

  resolveImport(sourceId: string, importPath: string): string | undefined {
    const sourceVirtualPath = this.getVirtualPath(sourceId);
    if (!sourceVirtualPath) return undefined;

    const remappedImport = this.remapImport(sourceVirtualPath, importPath);
    if (remappedImport) {
      return this.getSourceId(remappedImport);
    }

    if (importPath.startsWith("@")) {
      return this.getSourceId(importPath);
    }

    if (pathIsUrl(importPath)) {
      return this.getSourceId(importPath);
    }

    const sourceIsUrl = pathIsUrl(sourceVirtualPath);

    let resolvedPath: string;
    if (sourceIsUrl) {
      resolvedPath = resolveRelativeUrlImport(sourceVirtualPath, importPath);
    } else {
      resolvedPath = resolveRelativeImport(sourceVirtualPath, importPath);
    }

    return (
      this.getSourceId(resolvedPath) ??
      (sourceIsUrl
        ? this.getSourceId(importPath)
        : this.remapImport(sourceVirtualPath, resolvedPath)
          ? this.getSourceId(this.remapImport(sourceVirtualPath, resolvedPath)!)
          : undefined)
    );
  }

  getSourceId(virtualPath: string): string | undefined {
    const found = this.sourceMaps.find((source) => source.matchesVirtualPath(virtualPath));
    return found?.sourceId;
  }

  getVirtualPath(sourceId: string): string | undefined {
    const found = this.sourceMaps.find((source) => source.matchesSourceId(sourceId));
    return found?.virtualPath;
  }

  sourcesCount(): number {
    return this.sourceMaps.length;
  }

  remapImport(sourceVirtualPath: string, importPath: string): string | undefined {
    const candidates = this.importRemaps.filter((remap) => remap.matches(sourceVirtualPath, importPath));
    if (candidates.length === 0) return undefined;
    const best = candidates.reduce((longest, current) => (current.matchLen() > longest.matchLen() ? current : longest));
    return importPath.replace(best.prefix, best.target);
  }
}

function resolveRelativeImport(sourcePath: string, importPath: string): string {
  const sourceDir = path.dirname(sourcePath);
  let resolvedPath = path.normalize(path.join(sourceDir, importPath));
  return resolvedPath;
}

function resolveRelativeUrlImport(sourcePath: string, importPath: string): string {
  const url = new URL(sourcePath);
  const pathPart = url.pathname.replace(/^\/+/, "");
  const resolvedPath = resolveRelativeImport(pathPart, importPath);
  return `${url.protocol}//${url.host}/${resolvedPath}`;
}

function pathIsUrl(p: string): boolean {
  try {
    const url = new URL(p);
    return !!url.protocol && !!url.host;
  } catch {
    return false;
  }
}
