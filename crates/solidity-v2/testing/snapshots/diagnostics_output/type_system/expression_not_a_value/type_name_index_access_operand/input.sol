// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: the operand of an index access may name a type, since indexing a
// type name names the array type of it — dynamic or fixed-size, elementary or
// user-defined. Here each one is the target of an explicit conversion.

contract Test {
    struct Point {
        uint256 x;
    }

    function f(uint256[] memory a, Point[2] memory b) public pure {
        uint256[] memory c = uint256[](a);
        Point[2] memory d = Point[2](b);
        c;
        d;
    }
}
