import assert from "node:assert";
import { assertUserFileLocation } from "@nomicfoundation/slang/bindings";
import { findUnusedDefinitions } from "./find-unused-definitions.mjs";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";

export const CONTRACT_VFS = new Map<string, string>([
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
  function checkOwner(address addr) internal returns (bool) {
    return _owner == addr;
  }
}

contract Counter is Ownable {
  uint _count;
  uint _unused;
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
  function unusedDecrement() private {
    require(checkOwner(msg.sender));
    _count -= 1;
  }
}
    `,
  ],
]);

test("find unused definitions", async () => {
  const unit = await buildCompilationUnit(CONTRACT_VFS, "0.8.0", "contract.sol");

  const unused = findUnusedDefinitions(unit, "Counter");
  const expected = unused.map((definition) => {
    const location = definition.nameLocation;
    assertUserFileLocation(location);
    const name = location.cursor.node.unparse();
    return [name, location.cursor.textOffset.line];
  });
  assert.deepEqual(expected, [
    ["checkOwner", 10],
    ["_unused", 17],
    ["multiplier", 24],
    ["unusedDecrement", 29],
  ]);
});
