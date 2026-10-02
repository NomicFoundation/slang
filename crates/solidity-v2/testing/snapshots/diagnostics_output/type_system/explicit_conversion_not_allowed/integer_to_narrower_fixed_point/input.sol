// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(int256 x) public pure returns (fixed) {
        // Not every `int256` value fits the integer range of `fixed`.
        return fixed(x);
    }
}
