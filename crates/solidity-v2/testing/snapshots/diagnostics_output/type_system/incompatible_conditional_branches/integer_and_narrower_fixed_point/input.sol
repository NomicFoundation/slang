// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Not every `int256` value fits `fixed`, and neither converts to the other.
    function f(bool c, int256 x, fixed y) public pure returns (fixed) {
        return c ? x : y;
    }
}
