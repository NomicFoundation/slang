// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (uint256) {
        return 1;
    }
    // A constant initialized from a function call cannot be folded, so it is
    // not a valid array length.
    uint256 constant LEN = f();
    uint256[LEN] a;
}
