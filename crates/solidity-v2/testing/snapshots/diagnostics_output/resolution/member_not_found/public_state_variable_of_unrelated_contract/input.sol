// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    uint256 public x;
}

contract C {
    function f() public view returns (uint256) {
        return D.x;
    }
}
