// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(address owner => mapping(uint256 id => uint256[])) public m;

    function g() public view returns (uint256) {
        return this.m({owner: address(0), id: 2});
    }
}
