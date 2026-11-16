// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint256 => uint256) m;

    function f() public view {
        // A mapping is not an array, so it cannot be sliced.
        m[0:1];
    }
}
