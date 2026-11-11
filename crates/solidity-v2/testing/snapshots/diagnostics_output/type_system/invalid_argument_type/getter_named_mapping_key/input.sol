// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint256 key => uint256) public m;

    function g() public view returns (uint256) {
        // The named key resolves to the mapping key parameter, which a string does not convert to.
        return this.m({key: "x"});
    }
}
