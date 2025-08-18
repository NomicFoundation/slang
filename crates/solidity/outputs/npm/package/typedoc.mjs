/** @type {Partial<import("typedoc").TypeDocOptions>} */
const config = {
  /*
   * https://typedoc.org/documents/Options.html
   */

  basePath: ".",
  entryPointStrategy: "expand",
  entryPoints: [
    "src/index.mts",
    // __SLANG_NPM_PACKAGE_EXPORTS__ (keep in sync)
    "src/ast/index.mts",
    "src/bindings/index.mts",
    "src/compilation/index.mts",
    "src/cst/index.mts",
    "src/parser/index.mts",
    "src/utils/index.mts",
    // WASM-generated APIs:
    "wasm/index.d.mts",
  ],

  emit: "none",

  readme: "none", // we already have a site-wide README...
  disableSources: true, // links won't be persistant between commits...
  skipErrorChecking: true, // already dne by 'tsc'...
  treatWarningsAsErrors: true,
  excludeInternal: true,

  requiredToBeDocumented: [
    "Enum",
    "EnumMember",
    "Variable",
    "Function",
    "Class",
    "Interface",
    "Constructor",
    "Property",
    "Method",
    "Accessor",
    "TypeAlias",
  ],

  validation: {
    notExported: true,
    invalidLink: true,
    rewrittenLink: true,
    notDocumented: true,
    unusedMergeModuleWith: true,
  },
};

export default config;
