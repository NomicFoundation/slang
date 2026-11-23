// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint256 key => uint256) public m;

    function g() public view returns (uint256) {
        // The named key matches the name of the mapping key.
        return this.m({key: 1});
    }
}
