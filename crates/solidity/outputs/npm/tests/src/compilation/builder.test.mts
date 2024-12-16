import assert from "node:assert";
import { CompilationBuilder } from "@nomicfoundation/slang/compilation";
import { NonterminalKind } from "@nomicfoundation/slang/cst";
import { createBuilder } from "./common.mjs";

test("empty builder", async () => {
  const builder = await createBuilder();

  assertFiles(builder, []);
});

test("parent file without imports", async () => {
  const builder = await createBuilder();

  await builder.addFile("parent.sol");

  assertFiles(builder, ["parent.sol"]);
});

test("child file with one import", async () => {
  const builder = await createBuilder();

  await builder.addFile("child.sol");

  assertFiles(builder, ["parent.sol", "child.sol"]);
});

function assertFiles(builder: CompilationBuilder, expected: string[]) {
  const unit = builder.build();

  const actual = unit.files().map((file) => {
    assert.strictEqual(file.tree.kind, NonterminalKind.SourceUnit);
    assert.strictEqual(file.tree.id, file.createTreeCursor().node.id);

    return file.id;
  });

  for (const id of actual) {
    const file = unit.file(id)!;
    assert.strictEqual(file.id, id);
  }

  assert.deepEqual(actual.sort(), expected.sort());
}
