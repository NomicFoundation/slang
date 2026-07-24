// SPDX-License-Identifier: MIT
pragma solidity *;

// D's generated getter serves x's selector, so Base.x's body is not part of
// D's deployed bytecode. Only Base embeds D, which is no cycle.

contract Base {
    function x() external virtual returns (uint256) {
        new D();
        return 0;
    }
}

contract D is Base {
    uint256 public override x;
}
