// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint256 key => uint256) public m;

    function g() public view returns (uint256) {
        // Expected to flip to success once #2370 is merged.
        return this.m({key: 1});
    }
}
