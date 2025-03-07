import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { findUsages } from "./find-usages.mjs";

test("find usages", async () => {
  const CONTRACT_VFS = new Map<string, string>([
    [
      "contract.sol",
      `
contract Counter {
  uint _count;
  constructor(uint initialCount) {
    _count = initialCount;
  }
  function count() public view returns (uint) {
    return _count;
  }
  function increment(uint delta) public returns (uint) {
    require(delta > 0, "Delta must be positive");
    _count += delta;
    return _count;
  }
}
      `,
    ],
  ]);

  const unit = await buildCompilationUnit(CONTRACT_VFS, "0.8.0", "contract.sol");

  const usages = findUsages(unit, "contract.sol", 2, 10); // the _count state variable definition
  assert(typeof usages !== "string");

  assert.deepEqual(usages, [
    { file: "contract.sol", line: 4, column: 4 },
    { file: "contract.sol", line: 7, column: 11 },
    { file: "contract.sol", line: 11, column: 4 },
    { file: "contract.sol", line: 12, column: 11 },
  ]);
});
