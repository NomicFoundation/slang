import assert from "node:assert";
import { CONTRACT_VFS } from "../../04-find-unused-definitions/examples/test-find-unused-definitions.test.mjs";
import { buildCompilationUnit } from "../../common/compilation-builder.mjs";
import { LoggingRewriter } from "./logging-rewriter.mjs";

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
  function checkOwner(address addr) internal returns (bool) {
    log("checkOwner");
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
    log("count");
    return _count;
  }
  function increment(uint delta, uint multiplier) public onlyOwner returns (uint) {
    log("increment");
    require(delta > 0, "Delta must be positive");
    _count += delta;
    return _count;
  }
  function unusedDecrement() private {
    log("unusedDecrement");
    require(checkOwner(msg.sender));
    _count -= 1;
  }
}

import { log } from "__logging.sol";
    `,
  ],
]);

test("inject logging", async () => {
  const unit = await buildCompilationUnit(CONTRACT_VFS, "0.8.0", "contract.sol");

  const loggingRewriter = new LoggingRewriter();
  for (const file of unit.files()) {
    const newNode = loggingRewriter.rewriteNode(file.tree);
    assert.strictEqual(newNode?.unparse(), EXPECTED_VFS.get(file.id));
  }
});
