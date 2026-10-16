// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256 constant N = 2;

    function f() public pure returns (uint256) {
        uint256[N] memory arr;
        return arr.length;
    }
}
