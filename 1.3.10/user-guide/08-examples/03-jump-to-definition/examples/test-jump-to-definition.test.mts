import assert from "node:assert";
import { jumpToDefinition } from "./jump-to-definition.mjs";
import buildSampleCompilationUnit from "../../common/sample-contract.mjs";

test("jump to definition", async () => {
  const unit = await buildSampleCompilationUnit();

  // the reference to `delta` in the assignment addition
  const definition = jumpToDefinition(unit, "contract.sol", 11, 16);

  assert.deepEqual(definition, { file: "contract.sol", line: 9, column: 26 });
});
