// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes32 b, string memory s) public pure returns (bytes memory, bytes memory) {
        return (bytes(b), bytes(s));
    }
}
