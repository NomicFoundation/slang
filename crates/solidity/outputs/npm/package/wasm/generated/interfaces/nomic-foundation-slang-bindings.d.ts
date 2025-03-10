// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

export namespace NomicFoundationSlangBindings {
  export { BindingGraph };
  export { Definition };
  export { Reference };
  export { UserFileLocation };
  export { BuiltInLocation };
  export { BindingLocation };
  export { BindingLocationType };
}
import type { Cursor } from "./nomic-foundation-slang-cst.js";
export { Cursor };
/**
 * Represents a location of a symbol (definition or reference) in the binding graph.
 * It can either be in a user file, or a built-in in the language.
 */
export type BindingLocation = UserFileLocation | BuiltInLocation;

/**
 * Enumerates different variants of the `BindingLocation` type.
 */
export enum BindingLocationType {
  /**
   * Represents a variant of type `UserFileLocation`.
   */
  UserFileLocation = "UserFileLocation",

  /**
   * Represents a variant of type `BuiltInLocation`.
   */
  BuiltInLocation = "BuiltInLocation",
}

/**
 * A graph that contains name binding information for all source files within the compilation unit.
 * It stores cursors to all definitions and references, and can resolve the edges between them.
 *
 * Most cursors pointing to identifier terminals will resolve to either a definition or a reference.
 * For example, in `contract A is B {}`, the cursor to identifier `A` will resolve to a definition,
 * and the cursor to identifier `B` will resolve to a reference.
 *
 * However, in some cases, cursors to identifiers can resolve to both at the same time.
 * For example, in `import {X} from "library"`, the cursor to identifier `X` will resolve to a
 * definition (the local import), and also to a reference (to the symbol exported from `"library"`).
 *
 * This graph is error-tolerant, and will return `undefined` for any identifiers that cannot be resolved.
 * For example, when there are syntactic/semantic errors, or missing source files.
 *
 * For more information on identifier terminals, see the `TerminalKindExtensions.isIdentifier()` API.
 */

export class BindingGraph {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Tries to resolve the identifier terminal pointed at by the provided cursor to a definition.
   * If successful, returns the definition. Otherwise, returns `undefined`.
   *
   * For more information on identifier terminals, see the `TerminalKindExtensions.isIdentifier()` API.
   */
  definitionAt(cursor: Cursor): Definition | undefined;
  /**
   * Tries to resolve the identifier terminal pointed at by the provided cursor to a reference.
   * If successful, returns the reference. Otherwise, returns `undefined`.
   *
   * For more information on identifier terminals, see the `TerminalKindExtensions.isIdentifier()` API.
   */
  referenceAt(cursor: Cursor): Reference | undefined;
}

/**
 * Represents a location of a built-in symbol in the language.
 */

export class BuiltInLocation {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * The variant of `BindingLocationType` that corresponds to this class.
   */
  readonly type = BindingLocationType.BuiltInLocation;

  /**
   * Coerce this variant to a `BuiltInLocation`, or `undefined` if this is not the correct type.
   */
  asBuiltInLocation(): this;

  /**
   * Return `true` if this object is an instance of `BuiltInLocation`.
   */
  isBuiltInLocation(): this is BuiltInLocation;

  /**
   * Coerce this variant to a `UserFileLocation`, or `undefined` if this is not the correct type.
   */
  asUserFileLocation(): undefined;

  /**
   * Return `true` if this object is an instance of `UserFileLocation`.
   */
  isUserFileLocation(): false;
}

/**
 * Represents a definition in the binding graph.
 */

export class Definition {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Returns a unique numerical identifier of the definition.
   * It is only valid for the lifetime of the binding graph.
   * It can change between multiple graphs, even for the same source code input.
   */
  get id(): number;
  /**
   * Returns the location of the definition's name.
   * For `contract X {}`, that is the location of the `X` `Identifier` node.
   */
  get nameLocation(): BindingLocation;
  /**
   * Returns the location of the definition's definiens.
   * For `contract X {}`, that is the location of the parent `ContractDefinition` node.
   */
  get definiensLocation(): BindingLocation;
  /**
   * Returns a list of all references that bind to this definition.
   */
  references(): Array<Reference>;
}

/**
 * Represents a reference in the binding graph.
 */

export class Reference {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  /**
   * Returns a unique numerical identifier of the reference.
   * It is only valid for the lifetime of the binding graph.
   * It can change between multiple graphs, even for the same source code input.
   */
  get id(): number;
  /**
   * Returns the location of the reference.
   * For `new X()`, that is the location of the `X` `Identifier` node.
   */
  get location(): BindingLocation;
  /**
   * Returns a list of all definitions related to this reference.
   * Most references have a single definition, but some have multiple, such as when a symbol
   * is imported from another file, and renamed (re-defined) in the current file.
   */
  definitions(): Array<Definition>;
}

/**
 * Represents a location of a user-defined symbol in a user file.
 */

export class UserFileLocation {
  /**
   * This type does not have a public constructor.
   */
  private constructor();

  /**
   * The variant of `BindingLocationType` that corresponds to this class.
   */
  readonly type = BindingLocationType.UserFileLocation;

  /**
   * Coerce this variant to a `UserFileLocation`, or `undefined` if this is not the correct type.
   */
  asUserFileLocation(): this;

  /**
   * Return `true` if this object is an instance of `UserFileLocation`.
   */
  isUserFileLocation(): this is UserFileLocation;

  /**
   * Coerce this variant to a `BuiltInLocation`, or `undefined` if this is not the correct type.
   */
  asBuiltInLocation(): undefined;

  /**
   * Return `true` if this object is an instance of `BuiltInLocation`.
   */
  isBuiltInLocation(): false;

  /**
   * Returns the ID of the file that contains the symbol.
   */
  get fileId(): string;
  /**
   * Returns a cursor to the CST node that contains the symbol.
   */
  get cursor(): Cursor;
}
