// SPDX-License-Identifier: MIT
pragma solidity *;

// A library has no bases, so nothing is ever inherited into it: every clash
// among its members is a same-scope one.

library L {
    struct Shape {
        uint256 x;
    }

    // Redeclares the struct above.
    enum Shape {
        Round,
        Square
    }

    uint256 internal constant LIMIT = 1;

    // Redeclares the constant.
    function LIMIT() internal pure returns (uint256) {
        return 2;
    }

    event Signal(uint256 a);

    // An event and a function are different kinds, so they can't share a name.
    function Signal() internal pure {}

    modifier guarded() {
        _;
    }

    // Neither can a modifier and a function.
    function guarded() internal pure {}

    // Overloading is still fine: these are told apart by their parameters.
    function ok(uint256 a) internal pure returns (uint256) {
        return a;
    }

    function ok(bytes32 a) internal pure returns (bytes32) {
        return a;
    }
}
