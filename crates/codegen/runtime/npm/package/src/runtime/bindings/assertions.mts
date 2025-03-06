import { Cursor } from "../cst/index.mjs";
import { UserFileLocation, BuiltInLocation } from "./index.mjs";

/**
 * Asserts that `location` is a `UserFileLocation`.
 *
 * If a `fileId` value is provided, it will also assert that it matches its file ID.
 *
 * If a `cursor` value is provided, it will also assert that it points to the same node the cursor is pointing at.
 */
export function assertUserFileLocation(
  location: unknown,
  fileId?: string,
  cursor?: Cursor,
): asserts location is UserFileLocation {
  if (!(location instanceof UserFileLocation)) {
    throw new Error("Location provided is not a UserFileLocation.");
  }

  if (fileId !== undefined && fileId !== location.fileId) {
    throw new Error(`Location's fileId is expected to be '${fileId}', but got '${location.fileId}'.`);
  }

  if (cursor !== undefined && cursor.node.id !== location.cursor.node.id) {
    throw new Error(
      `Location's cursor is expected to point at node ID '${cursor.node.id}', but got '${location.cursor.node.id}'.`,
    );
  }
}

/**
 * Asserts that `location` is a `BuiltInLocation`.
 */
export function assertBuiltInLocation(location: unknown): asserts location is BuiltInLocation {
  if (!(location instanceof BuiltInLocation)) {
    throw new Error("Location provided is not a BuiltInLocation.");
  }
}
