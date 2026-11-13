// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint256 => uint256) public m;

    function g() public {
        this.m(1, 2);
    }
}
