// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string memory s) public pure returns (uint256) {
        // A string has no members.
        return s.length;
    }

    function g(string memory s) public pure returns (uint256) {
        return bytes(s).length;
    }
}
