// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Every `uint8` value fits `ufixed`, so the branches share that type.
    function f(bool c, uint8 x, ufixed y) public pure returns (ufixed) {
        return c ? x : y;
    }
}
