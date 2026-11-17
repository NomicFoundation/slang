// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint256 key => uint256) public m;

    function g() public view returns (uint256) {
        // Reported as an unknown named argument until #2370 is merged.
        return this.m({key: "x"});
    }
}
