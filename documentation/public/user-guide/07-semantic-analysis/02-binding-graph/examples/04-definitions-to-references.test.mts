import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { findDefinitionsInFile } from "./find-definitions.mjs";
import { NonterminalKind } from "@nomicfoundation/slang/cst";

test("navigate from definitions to references", async () => {
  const unit = await buildCompilationUnit();
  const definitions = findDefinitionsInFile(unit, "events.sol");

  // there are three definitions in the file: the event and its two parameters
  assert.strictEqual(definitions.length, 3);

  // we only care about the event type definition for this example
  const logEvent = definitions[0];
  assert.strictEqual(
    logEvent.definiensLocation.asUserFileLocation()!.cursor.node.kind,
    NonterminalKind.EventDefinition,
  );

  // find references bound to its definition
  const references = logEvent.references();
  assert.strictEqual(references.length, 2);

  // first should be the import statement
  assert.strictEqual(references[0].location.asUserFileLocation()!.fileId, "contract.sol");
  assert.strictEqual(references[0].location.asUserFileLocation()!.cursor.textRange.start.line, 1);

  // second should be the emit statement
  assert.strictEqual(references[1].location.asUserFileLocation()!.fileId, "contract.sol");
  assert.strictEqual(references[1].location.asUserFileLocation()!.cursor.textRange.start.line, 5);
});
