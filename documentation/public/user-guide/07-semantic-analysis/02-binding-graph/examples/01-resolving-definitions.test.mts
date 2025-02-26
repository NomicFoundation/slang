import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";

// TODO: add test and re-enable
test.skip("resolving definitions", async () => {
  const unit = await buildCompilationUnit();
  assert(unit);
});
