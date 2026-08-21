// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (uint256) {
        return true ? 1 : -1;
    }
}
