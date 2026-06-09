import assert from "node:assert";
import { findUnusedDefinitions } from "../../04-find-unused-definitions/examples/find-unused-definitions.mjs";
import { CONTRACT_VFS } from "../../04-find-unused-definitions/examples/test-find-unused-definitions.test.mjs";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { RemoveUnusedDefs } from "./remove-unused-defs.mjs";

const EXPECTED_VFS = new Map<string, string>([
  [
    "contract.sol",
    `
abstract contract Ownable {
  address _owner;
  constructor() {
    _owner = msg.sender;
  }
  modifier onlyOwner() {
    require(_owner == msg.sender);
    _;
  }
}

contract Counter is Ownable {
  uint _count;
  constructor(uint initialCount) {
    _count = initialCount;
  }
  function count() public view returns (uint) {
    return _count;
  }
  function increment(uint delta, uint multiplier) public onlyOwner returns (uint) {
    require(delta > 0, "Delta must be positive");
    _count += delta;
    return _count;
  }
}
    `,
  ],
]);

test("remove unused definitions", async () => {
  const unit = await buildCompilationUnit(CONTRACT_VFS, "0.8.0", "contract.sol");

  const unused = findUnusedDefinitions(unit, "Counter");
  const removeUnused = new RemoveUnusedDefs(unused);
  for (const file of unit.files()) {
    const newNode = removeUnused.rewriteNode(file.tree);
    assert.strictEqual(newNode?.unparse(), EXPECTED_VFS.get(file.id));
  }
});
