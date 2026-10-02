// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    enum Paper { Up, Down }
    enum Ground { North, South }

    function f(Paper p) public pure returns (Ground, Ground) {
        // An enum only converts to another enum through an integer.
        return (Ground(p), Ground(uint8(p)));
    }
}
