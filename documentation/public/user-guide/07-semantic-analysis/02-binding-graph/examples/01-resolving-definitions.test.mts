import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";
import { assertUserFileLocation, findNextIdentifier } from "./utils.mjs";

test("resolving definitions", async () => {
  const unit = await buildCompilationUnit();

  // get a cursor pointing to the `Log` identifier of the event definition
  const cursor = findNextIdentifier(unit.file("events.sol")!.createTreeCursor(), "Log");
  assert(cursor);

  // get the definition and the CST node that it names
  // in this case, an `EventDefinition` at line 0 (not 1,
  // because the parser pulls the leading whitespace into it)
  const definition = unit.bindingGraph.definitionAt(cursor);
  assert(definition);
  assertUserFileLocation(definition.definiensLocation, "events.sol", NonterminalKind.EventDefinition, 0);

  // find where this definition is used in the whole compilation unit
  const references = definition.references();
  assert(references.length == 2);

  // we expect two references in `contract.sol`
  // first reference is the `import` statement at line 1
  assertUserFileLocation(references[0].location, "contract.sol", TerminalKind.Identifier, 1);

  // second reference is in the `emit` statement at line 5
  assertUserFileLocation(references[1].location, "contract.sol", TerminalKind.Identifier, 5);
});
