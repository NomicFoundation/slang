import assert from "node:assert";
import { NonterminalKind, TerminalKind } from "@nomicfoundation/slang/cst";
import { assertUserFileLocation, createBuilder } from "./common.mjs";

test("binding graph", async () => {
  const builder = await createBuilder();
  await builder.addFile("child.sol");

  const unit = builder.build();
  const cursor = unit.file("child.sol")!.createTreeCursor();

  // import { Parent } from "./parent.sol";
  //          ^^^^^^
  assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));
  assert.equal(cursor.node.unparse(), "Parent");

  {
    const definition = unit.bindingGraph.definitionAt(cursor)!;
    assertUserFileLocation(definition.nameLocation, "child.sol", TerminalKind.Identifier, 3);
    assertUserFileLocation(definition.definiensLocation, "child.sol", NonterminalKind.ImportDeconstructionSymbol, 3);

    const refs = definition.references();
    assert.equal(refs.length, 1);

    assertUserFileLocation(refs[0]!.location, "child.sol", TerminalKind.Identifier, 5);
  }

  {
    const reference = unit.bindingGraph.referenceAt(cursor)!;
    assertUserFileLocation(reference.location, "child.sol", TerminalKind.Identifier, 3);

    const defs = reference.definitions();
    assert.equal(defs.length, 1);

    assertUserFileLocation(defs[0]!.nameLocation, "parent.sol", TerminalKind.Identifier, 3);
    assertUserFileLocation(defs[0]!.definiensLocation, "parent.sol", NonterminalKind.ContractDefinition, 2);
  }

  // contract Child is Parent {
  //          ^^^^^
  assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));
  assert.equal(cursor.node.unparse(), "Child");

  {
    const definition = unit.bindingGraph.definitionAt(cursor)!;
    assertUserFileLocation(definition.nameLocation, "child.sol", TerminalKind.Identifier, 5);
    assertUserFileLocation(definition.definiensLocation, "child.sol", NonterminalKind.ContractDefinition, 4);
    const refs = definition.references();
    assert.equal(refs.length, 0);
  }

  {
    assert.equal(unit.bindingGraph.referenceAt(cursor), undefined);
  }

  // contract Child is Parent {
  //                   ^^^^^^
  assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));
  assert.equal(cursor.node.unparse(), "Parent");

  {
    assert.equal(unit.bindingGraph.definitionAt(cursor), undefined);
  }

  {
    const reference = unit.bindingGraph.referenceAt(cursor)!;
    assertUserFileLocation(reference.location, "child.sol", TerminalKind.Identifier, 5);

    const defs = reference.definitions();
    assert.equal(defs.length, 2);

    assertUserFileLocation(defs[0]!.nameLocation, "child.sol", TerminalKind.Identifier, 3);
    assertUserFileLocation(defs[0]!.definiensLocation, "child.sol", NonterminalKind.ImportDeconstructionSymbol, 3);

    assertUserFileLocation(defs[1]!.nameLocation, "parent.sol", TerminalKind.Identifier, 3);
    assertUserFileLocation(defs[1]!.definiensLocation, "parent.sol", NonterminalKind.ContractDefinition, 2);
  }

  //   function foo() public pure {
  //            ^^^
  assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));
  assert.equal(cursor.node.unparse(), "foo");

  {
    const definition = unit.bindingGraph.definitionAt(cursor)!;
    assertUserFileLocation(definition.nameLocation, "child.sol", TerminalKind.Identifier, 6);
    assertUserFileLocation(definition.definiensLocation, "child.sol", NonterminalKind.FunctionDefinition, 6);

    const refs = definition.references();
    assert.equal(refs.length, 0);
  }

  {
    assert.equal(unit.bindingGraph.referenceAt(cursor), undefined);
  }

  //     assert(true);
  //     ^^^^^^
  assert(cursor.goToNextTerminalWithKind(TerminalKind.Identifier));
  assert.equal(cursor.node.unparse(), "assert");

  {
    assert.equal(unit.bindingGraph.definitionAt(cursor), undefined);
  }

  {
    const reference = unit.bindingGraph.referenceAt(cursor)!;
    assertUserFileLocation(reference.location, "child.sol", TerminalKind.Identifier, 7);

    const defs = reference.definitions();
    assert.equal(defs.length, 1);

    assert(defs[0]!.nameLocation.isBuiltInLocation());
    assert(defs[0]!.definiensLocation.isBuiltInLocation());
  }

  // Done! No more identifiers in the file.
  assert(!cursor.goToNextTerminalWithKind(TerminalKind.Identifier));
});
