import * as wasm from "../../wasm/index.mjs";

export { CompilationBuilder, CompilationBuilderConfig } from "./builder.mjs";

/** {@inheritDoc wasm.compilation.CompilationUnit} */
export const CompilationUnit = wasm.compilation.CompilationUnit;
/** {@inheritDoc wasm.compilation.CompilationUnit} */
export type CompilationUnit = wasm.compilation.CompilationUnit;

/** {@inheritDoc wasm.compilation.File} */
export const File = wasm.compilation.File;
/** {@inheritDoc wasm.compilation.File} */
export type File = wasm.compilation.File;
