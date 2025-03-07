import assert from "node:assert";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { jumpToDefinition } from "./jump-to-definition.mjs";

test("jump to definition", async () => {
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

  // the reference to `delta` in the assignment addition
  const definition = jumpToDefinition(unit, "contract.sol", 11, 16);
  assert(typeof definition !== "string");

  assert.deepEqual(definition, { file: "contract.sol", line: 9, column: 26 });
});
