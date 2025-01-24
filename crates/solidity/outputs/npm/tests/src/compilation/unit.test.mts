import assert from "node:assert";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";
import { assertUserFileLocation, createBuilder } from "./common.mjs";

test("retains language version", async () => {
  const builder = await createBuilder();
  const unit = builder.build();

  assert.equal(unit.languageVersion, "0.8.0");
});

test("can handle files with errors", async () => {
  const builder = await createBuilder();
  await builder.addFile("with-errors.sol");

  const unit = builder.build();
  const file = unit.file("with-errors.sol")!;

  const errors = file.errors();
  assert.equal(errors.length, 1);
  assert.equal(errors[0]!.message, "Expected OpenBrace or ReturnsKeyword or Semicolon.");

  assert.equal(file.tree.kind, NonterminalKind.SourceUnit);

  const cursor = file.createTreeCursor();
  assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));
  assert.equal(cursor.node.unparse(), "WithErrors");

  const definition = unit.bindingGraph.definitionAt(cursor)!;
  assertUserFileLocation(definition.nameLocation, "with-errors.sol", TerminalKind.Identifier, 3);
  assertUserFileLocation(definition.definiensLocation, "with-errors.sol", NonterminalKind.ContractDefinition, 2);
});
