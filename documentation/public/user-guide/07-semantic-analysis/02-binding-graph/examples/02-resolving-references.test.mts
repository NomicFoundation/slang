import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";

// TODO: add test and re-enable
test.skip("resolving references", async () => {
  const unit = await buildCompilationUnit();
  assert(unit);
});
