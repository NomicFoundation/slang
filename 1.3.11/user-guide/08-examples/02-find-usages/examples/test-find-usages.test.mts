import assert from "node:assert";
import { findUsages } from "./find-usages.mjs";
import buildSampleCompilationUnit from "../../common/sample-contract.mjs";

test("find usages", async () => {
  const unit = await buildSampleCompilationUnit();

  // the _count state variable definition
  const usages = findUsages(unit, "contract.sol", 2, 10);

  assert.deepEqual(usages, [
    { file: "contract.sol", line: 4, column: 4 },
    { file: "contract.sol", line: 7, column: 11 },
    { file: "contract.sol", line: 11, column: 4 },
    { file: "contract.sol", line: 12, column: 11 },
  ]);
});
