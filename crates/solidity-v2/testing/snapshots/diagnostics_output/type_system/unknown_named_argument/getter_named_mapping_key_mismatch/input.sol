// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint256 key => uint256) public m;

    function g() public view returns (uint256) {
        return this.m({other: 1});
    }
}
