// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint8 x) public pure returns (ufixed16x1) {
        // Every `uint8` value fits the integer range of `ufixed16x1`.
        return ufixed16x1(x);
    }
}
