// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint16 x) public pure returns (int8, int16, uint8) {
        // Changing sign and width at once is not allowed, but either alone is.
        return (int8(x), int16(x), uint8(x));
    }
}
