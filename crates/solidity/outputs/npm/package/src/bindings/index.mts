import * as wasm from "../../wasm/index.mjs";

export * from "./assertions.mjs";

/** {@inheritDoc wasm.bindings.BindingGraph} */
export const BindingGraph = wasm.bindings.BindingGraph;
/** {@inheritDoc wasm.bindings.BindingGraph} */
export type BindingGraph = wasm.bindings.BindingGraph;

/** {@inheritDoc wasm.bindings.Definition} */
export const Definition = wasm.bindings.Definition;
/** {@inheritDoc wasm.bindings.Definition} */
export type Definition = wasm.bindings.Definition;

/** {@inheritDoc wasm.bindings.Reference} */
export const Reference = wasm.bindings.Reference;
/** {@inheritDoc wasm.bindings.Reference} */
export type Reference = wasm.bindings.Reference;

/** {@inheritDoc wasm.bindings.BindingLocation} */
export type BindingLocation = wasm.bindings.BindingLocation;

/** {@inheritDoc wasm.bindings.BindingLocationType} */
export const BindingLocationType = wasm.bindings.BindingLocationType;
/** {@inheritDoc wasm.bindings.BindingLocationType} */
export type BindingLocationType = wasm.bindings.BindingLocationType;

/** {@inheritDoc wasm.bindings.UserFileLocation} */
export const UserFileLocation = wasm.bindings.UserFileLocation;
/** {@inheritDoc wasm.bindings.UserFileLocation} */
export type UserFileLocation = wasm.bindings.UserFileLocation;

/** {@inheritDoc wasm.bindings.BuiltInLocation} */
export const BuiltInLocation = wasm.bindings.BuiltInLocation;
/** {@inheritDoc wasm.bindings.BuiltInLocation} */
export type BuiltInLocation = wasm.bindings.BuiltInLocation;
