import assert from "node:assert";
import { assertNonterminalNode, NonterminalKind } from "@nomicfoundation/slang/cst";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";

test("compilation unit", async () => {
  const unit = await buildCompilationUnit();

  const files = unit.files();
  assert.equal(files.length, 2);

  assert.equal(files[0].id, "contract.sol");
  assertNonterminalNode(files[0].tree, NonterminalKind.SourceUnit);

  assert.equal(files[1].id, "events.sol");
  assertNonterminalNode(files[1].tree, NonterminalKind.SourceUnit);
});
