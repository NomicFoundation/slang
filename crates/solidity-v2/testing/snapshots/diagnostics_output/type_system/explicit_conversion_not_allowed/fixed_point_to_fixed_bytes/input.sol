// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(fixed128x18 x) public pure returns (int256, bytes16) {
        // A fixed-point value converts to an integer, but not to `bytesN`.
        return (int256(x), bytes16(x));
    }
}
