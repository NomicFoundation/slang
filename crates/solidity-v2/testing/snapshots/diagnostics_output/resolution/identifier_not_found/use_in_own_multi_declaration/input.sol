// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (uint256, uint256) {
        (uint256 a, uint256 b) = (b, 1);
        return (a, b);
    }
}
