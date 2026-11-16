// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint256 => uint256) m;

    function f() public view {
        // Indexing a mapping needs an index expression.
        m[];
    }
}
