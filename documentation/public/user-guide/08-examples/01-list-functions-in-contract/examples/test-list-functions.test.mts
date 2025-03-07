import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { listFunctionsInContract } from "./list-functions-in-contract.mjs";

test("list functions by contract name", async () => {
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

  const functions = listFunctionsInContract(unit, "Counter");
  assert(functions);
  assert.strictEqual(functions.length, 2);

  const functionNames = functions.map((fun) => fun.name.cst.unparse().trim());
  assert.deepEqual(functionNames, ["count", "increment"]);
});
