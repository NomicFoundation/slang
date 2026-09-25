// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() external returns (uint256) {}

    function f() public view returns (uint256) {
        return uint256(this.g);
    }
}
