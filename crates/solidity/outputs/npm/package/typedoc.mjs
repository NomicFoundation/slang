/** @type {Partial<import("typedoc").TypeDocOptions>} */
const config = {
  /*
   * https://typedoc.org/documents/Options.html
   */

  basePath: ".",
  entryPointStrategy: "expand",
  entryPoints: [
    "src/generated/index.mts",
    // __SLANG_NPM_PACKAGE_EXPORTS__ (keep in sync)
    "src/generated/ast/index.mts",
    "src/generated/bindings/index.mts",
    "src/generated/compilation/index.mts",
    "src/generated/cst/index.mts",
    "src/generated/parser/index.mts",
    "src/generated/utils/index.mts",
    // WASM-generated APIs:
    "wasm/index.d.mts",
  ],

  emit: "none",

  readme: "none", // we already have a site-wide README...
  disableSources: true, // links won't be persistent between commits...
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
