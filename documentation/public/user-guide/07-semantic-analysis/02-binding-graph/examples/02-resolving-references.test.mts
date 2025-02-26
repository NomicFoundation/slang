import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { assertUserFileLocation, findNextIdentifier } from "./utils.mjs";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";

test("resolving references", async () => {
  const unit = await buildCompilationUnit();

  var cursor = unit.file("contract.sol")!.createTreeCursor();

  // get a cursor pointing to the first `Log` identifier instance
  // ie. the one in the `import` statement
  cursor = findNextIdentifier(cursor, "Log")!;
  assert(cursor.textRange.start.line == 1);

  // get the reference for the reference at the `import` statement
  const importReference = unit.bindingGraph.referenceAt(cursor);
  assert(importReference);
  assertUserFileLocation(importReference.location, "contract.sol", TerminalKind.Identifier, 1);

  // navigate to the definition, which should live in the `events.sol` file
  const importDefinitions = importReference.definitions();
  assert(importDefinitions.length == 1);
  assertUserFileLocation(importDefinitions[0].definiensLocation, "events.sol", NonterminalKind.EventDefinition, 0);

  // same for the second reference in `contract.sol`, which is in the `emit` statement
  cursor = findNextIdentifier(cursor, "Log")!;
  assert(cursor.textRange.start.line == 5);
  const emitReference = unit.bindingGraph.referenceAt(cursor);
  assert(emitReference);
  assertUserFileLocation(emitReference.location, "contract.sol", TerminalKind.Identifier, 5);

  // because the `Log` event is imported by name, in this case we'll find 2 definitions:
  const emitDefinitions = emitReference.definitions();
  assert(emitDefinitions.length == 2);

  // the first one pointing to the `import` statement
  assertUserFileLocation(
    emitDefinitions[0].definiensLocation,
    "contract.sol",
    NonterminalKind.ImportDeconstructionSymbol,
    1,
  );

  // and the second one, the actual event type definition in the `events.sol` file
  assertUserFileLocation(emitDefinitions[1].definiensLocation, "events.sol", NonterminalKind.EventDefinition, 0);
});
