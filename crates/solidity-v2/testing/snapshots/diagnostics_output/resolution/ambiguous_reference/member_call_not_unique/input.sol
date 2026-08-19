// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function a(uint8) public pure returns (uint) {
        return 1;
    }

    function a(uint256) public pure returns (uint) {
        return 2;
    }
}

contract D {
    function ambiguous(C c) public pure returns (uint) {
        // `1` is convertible to both `uint8` and `uint256`, so both overloads
        // accept the call.
        return c.a(1);
    }

    function unambiguous(C c) public pure returns (uint) {
        // `300` doesn't fit in a `uint8`, so only one overload accepts it.
        return c.a(300);
    }
}
