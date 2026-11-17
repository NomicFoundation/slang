// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g(string memory s) public pure {
        keccak256(s);
    }
}
