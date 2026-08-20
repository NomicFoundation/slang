// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint8 a) internal pure returns (uint) {
        return a;
    }

    function f(uint256 a) internal pure returns (uint) {
        return 2 * a;
    }

    function ambiguous() internal pure returns (uint) {
        // `1` is convertible to both `uint8` and `uint256`, so both overloads
        // accept the call.
        return f(1);
    }

    function unambiguous() internal pure returns (uint) {
        // `300` doesn't fit in a `uint8`, so only one overload accepts it.
        return f(300);
    }
}
