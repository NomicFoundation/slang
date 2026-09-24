// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes memory b) public pure returns (bytes32) {
        return bytes32(b);
    }
}
