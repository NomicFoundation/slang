// SPDX-License-Identifier: MIT
pragma solidity *;

// The constant's value is compiled into `f`, embedding B's own creation
// code into B.

contract B {
    bytes constant c = type(B).creationCode;

    function f() public pure returns (bytes memory) {
        return c;
    }
}
