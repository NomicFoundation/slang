// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(string => uint256) public m;

    function g() public view returns (uint256) {
        // A getter is only called externally, so its key accepts the literal.
        return this.m("x");
    }
}
