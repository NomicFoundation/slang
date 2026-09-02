// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    mapping(uint256 => int128) m1;
    mapping(uint256 => int256) m2;

    // Mappings only reconcile when their key and value types match.
    function f(bool c) public view returns (int256) {
        return (c ? m1 : m2)[0];
    }
}
