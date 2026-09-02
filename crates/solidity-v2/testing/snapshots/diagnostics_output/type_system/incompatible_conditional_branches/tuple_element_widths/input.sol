// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Each tuple is wider than the other in a different position, so neither
    // converts to the other.
    function f() public pure returns (uint256, uint256) {
        return true ? (uint256(1), uint128(2)) : (uint128(3), uint256(4));
    }
}
