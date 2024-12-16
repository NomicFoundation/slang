import * as generated from "../../../wasm/index.mjs";
import * as builder from "./builder.mjs";

// This is a wrapper around 'generated.compilation.InternalCompilationBuilder':
export const CompilationBuilder = builder.CompilationBuilder;
export type CompilationBuilder = builder.CompilationBuilder;

export type CompilationBuilderConfig = builder.CompilationBuilderConfig;

export const CompilationUnit = generated.compilation.CompilationUnit;
export type CompilationUnit = generated.compilation.CompilationUnit;

export const File = generated.compilation.File;
export type File = generated.compilation.File;
