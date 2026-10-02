// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(string memory s) public pure returns (bytes32) {
        return bytes32(s);
    }
}
